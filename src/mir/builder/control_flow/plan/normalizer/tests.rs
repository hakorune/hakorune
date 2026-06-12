use super::PlanNormalizer;
use crate::ast::{ASTNode, FieldDecl, Span};
use crate::mir::builder::MirBuilder;
use crate::mir::{Effect, EffectMask, MirType, ValueId};
use std::collections::BTreeMap;

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn method_call(object: ASTNode, method: &str, arguments: Vec<ASTNode>) -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(object),
        method: method.to_string(),
        arguments,
        span: Span::unknown(),
    }
}

#[test]
fn lower_value_ast_accepts_me_field_access() {
    let mut builder = MirBuilder::new();
    builder.comp_ctx.register_user_box_with_field_decls(
        "Counter".to_string(),
        vec![FieldDecl {
            name: "limit".to_string(),
            declared_type_name: Some("IntegerBox".to_string()),
            is_weak: false,
            default_value: None,
        }],
    );
    let me_id = builder.alloc_typed(MirType::Box("Counter".to_string()));
    builder
        .variable_ctx
        .variable_map
        .insert("me".to_string(), me_id);
    builder
        .type_ctx
        .value_origin_newbox
        .insert(me_id, "Counter".to_string());

    let expr = ASTNode::FieldAccess {
        object: Box::new(ASTNode::Me {
            span: Span::unknown(),
        }),
        field: "limit".to_string(),
        span: Span::unknown(),
    };

    let (value_id, effects) =
        PlanNormalizer::lower_value_ast(&expr, &mut builder, &BTreeMap::new())
            .expect("me.field should lower in value context");

    assert_eq!(
        builder.type_ctx.get_type(value_id),
        Some(&MirType::Box("IntegerBox".to_string()))
    );
    assert!(matches!(
        effects.as_slice(),
        [super::CoreEffectPlan::FieldGet {
            dst,
            base,
            field,
            declared_type: Some(MirType::Box(type_name)),
        }] if *dst == value_id
            && *base == me_id
            && field == "limit"
            && type_name == "IntegerBox"
    ));
}

#[test]
fn lower_value_ast_keeps_nested_method_call_receiver_chain() {
    let mut builder = MirBuilder::new();
    let array_id = ValueId(1);
    let index_id = ValueId(2);
    builder
        .variable_ctx
        .variable_map
        .insert("arr".to_string(), array_id);
    builder
        .variable_ctx
        .variable_map
        .insert("idx".to_string(), index_id);
    builder
        .type_ctx
        .set_type(array_id, MirType::Box("RuntimeDataBox".to_string()));
    builder.type_ctx.set_type(index_id, MirType::Integer);

    let expr = method_call(
        method_call(var("arr"), "get", vec![var("idx")]),
        "length",
        vec![],
    );

    let (outer_result, effects) =
        PlanNormalizer::lower_value_ast(&expr, &mut builder, &BTreeMap::new())
            .expect("nested method call should lower");

    assert_eq!(
        effects.len(),
        2,
        "expected get + length effects, got {effects:?}"
    );

    let inner_result = match &effects[0] {
        super::CoreEffectPlan::MethodCall {
            dst: Some(dst),
            object,
            method,
            args,
            effects,
        } => {
            assert_eq!(*object, array_id, "get should stay on the array receiver");
            assert_eq!(method, "get");
            assert_eq!(args.as_slice(), &[index_id]);
            assert_eq!(*effects, EffectMask::PURE.add(Effect::Io));
            *dst
        }
        other => panic!("first effect must be inner get, got {:?}", other),
    };

    match &effects[1] {
        super::CoreEffectPlan::MethodCall {
            dst: Some(dst),
            object,
            method,
            args,
            effects,
        } => {
            assert_eq!(*dst, outer_result);
            assert_eq!(*object, inner_result, "length must receive the get result");
            assert_eq!(method, "length");
            assert!(args.is_empty());
            assert_eq!(*effects, EffectMask::PURE.add(Effect::Io));
        }
        other => panic!("second effect must be outer length, got {:?}", other),
    }
}
