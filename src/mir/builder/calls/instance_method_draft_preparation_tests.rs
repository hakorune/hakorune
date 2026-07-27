use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction, MirType};

use super::super::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use super::instance_method_draft_preparation::{
    prepare_instance_method_draft_body_v1, InstanceMethodDraftPreparationRequestV1,
};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn scalar_return(value: i64) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(integer(value))),
        span: Span::unknown(),
    }
}

fn block_snapshot(
    function: &crate::mir::MirFunction,
) -> Vec<(BasicBlockId, Vec<MirInstruction>, Option<MirInstruction>)> {
    let mut rows = function
        .blocks
        .iter()
        .map(|(block, body)| (*block, body.instructions.clone(), body.terminator.clone()))
        .collect::<Vec<_>>();
    rows.sort_by_key(|(block, _, _)| *block);
    rows
}

fn assert_function_parity(legacy: &crate::mir::MirFunction, port_aware: &crate::mir::MirFunction) {
    assert_eq!(legacy.signature.name, port_aware.signature.name);
    assert_eq!(legacy.signature.params, port_aware.signature.params);
    assert_eq!(
        legacy.signature.return_type,
        port_aware.signature.return_type
    );
    assert_eq!(legacy.params, port_aware.params);
    assert_eq!(legacy.locals, port_aware.locals);
    assert_eq!(legacy.next_value_id, port_aware.next_value_id);
    assert_eq!(block_snapshot(legacy), block_snapshot(port_aware));
    assert_eq!(
        legacy.metadata.declared_param_decls,
        port_aware.metadata.declared_param_decls
    );
    assert_eq!(
        legacy.metadata.declared_return_type_name,
        port_aware.metadata.declared_return_type_name
    );
    assert_eq!(
        legacy.metadata.declared_capability_uses,
        port_aware.metadata.declared_capability_uses
    );
    assert_eq!(legacy.metadata.value_types, port_aware.metadata.value_types);
}

#[test]
fn preparation_installs_exact_method_header_and_preserves_owned_body() {
    let body = vec![integer(42)];
    let mut builder = MirBuilder::new();

    let prepared = prepare_instance_method_draft_body_v1(
        &mut builder,
        InstanceMethodDraftPreparationRequestV1::new(
            "Fixture.read/1".to_owned(),
            "Fixture".to_owned(),
            vec!["value".to_owned()],
            vec![ParamDecl {
                name: "value".to_owned(),
                declared_type_name: Some("i64".to_owned()),
            }],
            Some("i64".to_owned()),
            body.clone(),
            vec!["ReadHeap".to_owned()],
            DeclarationAttrs::default(),
        ),
    )
    .unwrap();

    assert_eq!(prepared.body(), body.as_slice());
    let function = builder.function_state.current_function.as_ref().unwrap();
    assert_eq!(function.signature.name, "Fixture.read/1");
    assert_eq!(
        function.signature.params,
        vec![MirType::Box("Fixture".to_owned()), MirType::Integer]
    );
    assert_eq!(function.signature.return_type, MirType::Integer);
    assert_eq!(function.params.len(), 2);
    assert_eq!(
        function.metadata.declared_capability_uses,
        vec!["ReadHeap".to_owned()]
    );
    assert_eq!(function.metadata.declared_param_decls.len(), 2);
    assert!(function
        .blocks
        .values()
        .all(|block| block.instructions.is_empty() && block.terminator.is_none()));
    assert!(builder
        .function_state
        .variable_ctx
        .variable_map
        .contains_key("me"));
    assert!(builder
        .function_state
        .variable_ctx
        .variable_map
        .contains_key("value"));
}

#[test]
fn prepared_body_is_consumed_once_without_clone_authority() {
    let body = vec![integer(1), integer(2)];
    let mut builder = MirBuilder::new();
    let prepared = prepare_instance_method_draft_body_v1(
        &mut builder,
        InstanceMethodDraftPreparationRequestV1::new(
            "Fixture.consume/0".to_owned(),
            "Fixture".to_owned(),
            Vec::new(),
            Vec::new(),
            None,
            body.clone(),
            Vec::new(),
            DeclarationAttrs::default(),
        ),
    )
    .unwrap();

    assert_eq!(prepared.into_body(), body);
}

#[test]
fn legacy_completion_preserves_port_aware_instance_draft_contract() {
    for body in [Vec::new(), vec![scalar_return(7)]] {
        let mut through_legacy_completion = MirBuilder::new();
        let mut legacy_port = RawLegacyChildLoweringPortV1;
        let legacy_prepared = through_legacy_completion
            .build_instance_method_draft_with_port_v1(
                &mut legacy_port,
                "Fixture.parity/0".to_owned(),
                "Fixture".to_owned(),
                Vec::new(),
                Vec::new(),
                Some("i64".to_owned()),
                body.clone(),
                vec!["ReadHeap".to_owned()],
                DeclarationAttrs::default(),
            )
            .unwrap();
        let legacy_draft = through_legacy_completion
            .finalize_port_aware_draft_for_legacy_v1(legacy_prepared)
            .unwrap();

        let mut explicit_lookup = MirBuilder::new();
        let mut lookup_port = RawLegacyChildLoweringPortV1;
        let lookup_prepared = explicit_lookup
            .build_instance_method_draft_with_port_v1(
                &mut lookup_port,
                "Fixture.parity/0".to_owned(),
                "Fixture".to_owned(),
                Vec::new(),
                Vec::new(),
                Some("i64".to_owned()),
                body,
                vec!["ReadHeap".to_owned()],
                DeclarationAttrs::default(),
            )
            .unwrap();
        let returns_value = lookup_prepared.returns_value_for_test();
        let lookup_draft = explicit_lookup
            .finalize_function_draft_with_lookup(returns_value, None)
            .unwrap();

        assert_function_parity(&legacy_draft, &lookup_draft);
    }
}
