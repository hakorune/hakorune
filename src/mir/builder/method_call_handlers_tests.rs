use super::*;
use crate::ast::{LiteralValue, Span};
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::builder::calls::{AssociatedMethodCallArgumentsV1, RawLegacyMethodCallInputV1};
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::parser::NyashParser;

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn instruction_count(builder: &MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .map(|block| block.instructions.len())
        .sum()
}

#[test]
fn prepared_me_standard_unified_is_effect_free_until_execute() {
    let arguments = vec![integer(1)];
    let input = RawLegacyMethodCallInputV1::new(
        ASTNode::Me {
            span: Span::unknown(),
        },
        "routeMethod".to_string(),
        arguments.clone(),
    );
    let mut builder = MirBuilder::new();
    let root =
        NyashParser::parse_from_string("static box RouteCatalogSentinel { noop() { return 0 } }")
            .unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root).unwrap();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .unwrap();
    builder.enter_function_for_test("RouteOwner.caller/0".to_string());
    let me = crate::mir::builder::emission::constant::emit_integer(&mut builder, 9).unwrap();
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert("me".to_string(), me);
    let before = instruction_count(&builder);
    let mut port = RawLegacyChildLoweringPortV1;
    let prepared =
        prepare_me_call_execution_v1(&builder, "routeMethod", &arguments, &mut port).unwrap();
    assert!(prepared.is_standard_unified());
    assert_eq!(
        instruction_count(&builder),
        before,
        "prepare must not emit MIR"
    );

    let mut descent = AssociatedMethodCallArgumentsV1::new(&mut port, &input);
    let result = MeCallPolicyBox::execute(
        &mut builder,
        "routeMethod",
        &arguments,
        &mut descent,
        prepared,
    )
    .unwrap()
    .expect("bound me standard route must execute");
    assert!(instruction_count(&builder) > before);
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .any(|instruction| matches!(
            instruction,
            crate::mir::MirInstruction::Call { dst, .. } if *dst == Some(result)
        )));
}

fn lowered_global(
    expected_params: usize,
    receiver: PreparedMeReceiverV1,
) -> PreparedMeCallExecutionV1 {
    PreparedMeCallExecutionV1::LoweredGlobal {
        owner: "RouteOwner".to_owned(),
        prepared: crate::mir::builder::me_call_header_observation::PreparedMeLoweredCallV1::from_test_parts(
            expected_params,
            receiver,
        ),
    }
}

#[test]
fn strict_me_arity_rejects_before_argument_descent() {
    let prepared = lowered_global(2, PreparedMeReceiverV1::Static);
    let error =
        MeCallPolicyBox::validate_prepared_me_arity_before_descent(&prepared, "route", 1, true)
            .expect_err("strict mismatch must stop before descent");
    assert!(error.starts_with("[freeze:contract][me-call/arity]"));
}

#[test]
fn strict_me_arity_counts_the_explicit_instance_receiver() {
    let matching = lowered_global(
        2,
        PreparedMeReceiverV1::Instance {
            me: Some(ValueId(7)),
        },
    );
    MeCallPolicyBox::validate_prepared_me_arity_before_descent(&matching, "route", 1, true)
        .expect("one source argument plus me matches two parameters");

    let mismatch = lowered_global(
        3,
        PreparedMeReceiverV1::Instance {
            me: Some(ValueId(7)),
        },
    );
    assert!(MeCallPolicyBox::validate_prepared_me_arity_before_descent(
        &mismatch, "route", 1, true
    )
    .is_err());
}

#[test]
fn explicit_compatibility_override_keeps_mismatch_nonfatal() {
    let prepared = lowered_global(2, PreparedMeReceiverV1::Static);
    MeCallPolicyBox::validate_prepared_me_arity_before_descent(&prepared, "route", 1, false)
        .expect("explicit compatibility mode is the only permissive state");
}
