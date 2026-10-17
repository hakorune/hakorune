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

#[test]
fn source_result_finalizer_checks_every_return_operand_without_first_type_default() {
    use crate::mir::{BasicBlock, ConstValue, EffectMask, ValueId};
    use crate::mir::function::{FunctionSignature, MirFunction};
    for is_bool in [false, true] {
        let body = if is_bool { "return true" } else { "return 7" };
        let package = crate::mir::normal_callable_semantic_package::issue_source_package_fixture_v1(
            &format!("box Item {{ value: i64 birth() {{ me.value = 1 }} }} box Probe {{ flag: i64 birth() {{ me.flag = 0 }} leaf(p) {{ {body} }} bridge(h: Item, p) {{ if p <= 0 {{ return 0 }} local kept = new Probe() me.leaf(p) return 0 }} }} static box Main {{ main() {{ return 0 }} }}")
        ).unwrap();
        let ledger = package.ordinary_new_claim_ledger_for_test();
        let kind = if is_bool {
            crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool
        } else { crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer };
        let loan = package.batch().declarations()
            .filter(|declaration| declaration.parameter_count() == 1)
            .filter_map(|declaration| ledger.callable_scalar_result_source_v1(declaration.owner()).unwrap())
            .find(|loan| loan.kind() == kind).unwrap_or_else(|| panic!("original source scalar participant {kind:?}"));
        let flow = loan.completion().cleanup().root_flow().expect("original full callee Home walk");
        assert!(flow.all_exits_ready());
        for exit in loan.completion().explicit_sites() {
            assert!(flow.exit_row(exit).unwrap().unwrap().homes().is_empty(), "pure literal callee Homes");
        }
        let expected = if is_bool { MirType::Bool } else { MirType::Integer };
        let source = crate::mir::normal_callable_semantic_package::CallableResultSourceV1::Scalar(loan);
        for change in 0..4 {
            // Independent unpublished physical type fixture, not source-exit or
            // executable-input acceptance. The result loan comes from real source.
            let mut function = MirFunction::new(FunctionSignature {
                name: "scalar-finalization-fixture".into(), params: Vec::new(),
                return_type: MirType::Unknown, effects: EffectMask::PURE,
            }, BasicBlockId(0));
            let mut builder = MirBuilder::new();
            for index in 0..2 {
                let value = function.next_value_id();
                let mut block = BasicBlock::new(BasicBlockId(index));
                let correct = !(change == 1 && index == 1);
                let constant = if is_bool == correct { ConstValue::Bool(true) } else { ConstValue::Integer(7) };
                builder.function_state.type_ctx.value_types.insert(value,
                    if matches!(constant, ConstValue::Bool(_)) { MirType::Bool } else { MirType::Integer });
                block.add_instruction(MirInstruction::Const { dst: value, value: constant });
                block.add_instruction(MirInstruction::Return { value: if index == 1 && change == 2 {
                    None
                } else if index == 1 && change == 3 { Some(ValueId(99)) } else { Some(value) } });
                function.blocks.insert(block.id, block);
            }
            builder.function_state.current_function = Some(function);
            builder.function_state.current_block = Some(BasicBlockId(0));
            let finalized = builder.finalize_function_draft_with_body_sources_v1(true, None, Some(&source), None);
            if change == 0 {
                assert_eq!(finalized.unwrap().signature.return_type, expected);
            } else {
                assert!(finalized.unwrap_err().contains("callable-scalar/return-type-mismatch"));
            }
        }
    }
}
