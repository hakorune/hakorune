use std::sync::Arc;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::RegionId;
use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1};
use crate::mir::{BasicBlockId, MirBuilder, MirCompiler, MirInstruction, MirType, ValueId};

use super::completion_consumption::ResolvedFunctionCompletionConsumptionV1;
use super::draft_seal::{PreparedFunctionExitV1, ReadyFunctionDraftSealV1};

fn resolved_product(name: &str) -> Arc<crate::mir::resolved_semantics::VerifiedResolvedFunctionV1> {
    let function = function(name, Vec::new());
    let view = FunctionSyntaxViewV1::from_ast(&function).unwrap();
    FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(view)
        .unwrap()
}

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn return_stmt(value: Option<ASTNode>) -> ASTNode {
    ASTNode::Return {
        value: value.map(Box::new),
        span: Span::unknown(),
    }
}

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn compile(name: &str, body: Vec<ASTNode>) -> crate::mir::MirFunction {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(name, body)).unwrap();
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("completion_fixture.hako"))
        .unwrap();
    assert!(result.verification_result.is_ok());
    result.module.functions[&format!("{name}/0")].clone()
}

fn return_count(function: &crate::mir::MirFunction) -> usize {
    function
        .blocks
        .values()
        .filter(|block| {
            matches!(
                block.terminator.as_ref(),
                Some(MirInstruction::Return { .. })
            )
        })
        .count()
}

#[test]
fn explicit_value_return_is_emitted_exactly_once() {
    let function = compile("completion_value", vec![return_stmt(Some(literal(7)))]);
    assert_eq!(return_count(&function), 1);
}

#[test]
fn explicit_void_return_is_emitted_exactly_once() {
    let function = compile("completion_explicit_void", vec![return_stmt(None)]);
    assert_eq!(return_count(&function), 1);
}

#[test]
fn empty_and_nonempty_implicit_fallthrough_emit_one_return_each() {
    let empty = compile("completion_empty", Vec::new());
    let nonempty = compile("completion_nonempty", vec![literal(1)]);
    assert_eq!(return_count(&empty), 1);
    assert_eq!(return_count(&nonempty), 1);
}

#[test]
fn implicit_completion_consumes_the_exact_body_end_and_target() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "completion_implicit_claim",
        vec![literal(1)],
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let target = input.function().lowering_roots().function_pair().region();
    let completion = verify_function_completion_v1(input).unwrap();
    let consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();

    let error = consumption.finish(body.site(), 2, target).unwrap_err();
    assert!(error.contains("implicit_body_mismatch"));
}

#[test]
fn explicit_completion_rejects_wrong_target_before_emission() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "completion_wrong_target",
        vec![return_stmt(Some(literal(7)))],
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let site = completion.explicit_site().unwrap().clone();
    let target = completion.target_function();
    let wrong_target = RegionId::new(target.owner(), target.slot() + 1);
    let mut consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();

    let error = consumption
        .claim_explicit_return(&site, wrong_target, BasicBlockId::new(0), ValueId::new(0))
        .unwrap_err();
    assert!(error.contains("target_mismatch"));
}

#[test]
fn explicit_completion_retains_exact_lowered_operand_witness() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "completion_operand_witness",
        vec![return_stmt(Some(literal(7)))],
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let site = completion.explicit_site().unwrap().clone();
    let target = input.function().lowering_roots().function_pair().region();
    let mut consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();

    consumption
        .claim_explicit_return(&site, target, BasicBlockId::new(3), ValueId::new(17))
        .unwrap();
    let ready = consumption
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();
    let witness = ready.explicit_operand().unwrap();
    assert_eq!(witness.block(), BasicBlockId::new(3));
    assert_eq!(witness.value(), ValueId::new(17));
}

#[test]
fn draft_seal_prepares_the_exact_explicit_operand_without_reclassifying_it() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "draft_seal_explicit_operand",
        vec![return_stmt(Some(literal(7)))],
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let site = completion.explicit_site().unwrap().clone();
    let target = input.function().lowering_roots().function_pair().region();
    let mut consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();
    consumption
        .claim_explicit_return(&site, target, BasicBlockId::new(4), ValueId::new(23))
        .unwrap();
    let ready = consumption
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();

    let ready = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(4));
    assert_eq!(
        ready.prepare_exit_borrowed().unwrap(),
        PreparedFunctionExitV1::ExplicitValue {
            block: BasicBlockId::new(4),
            value: ValueId::new(23),
        }
    );
    let prepared = ready.prepare().unwrap();
    assert_eq!(
        prepared.exit(),
        PreparedFunctionExitV1::ExplicitValue {
            block: BasicBlockId::new(4),
            value: ValueId::new(23),
        }
    );
}

#[test]
fn draft_seal_keeps_explicit_unit_distinct_from_implicit_unit() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "draft_seal_explicit_unit",
        vec![return_stmt(None)],
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let site = completion.explicit_site().unwrap().clone();
    let target = input.function().lowering_roots().function_pair().region();
    let mut consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();
    consumption.claim_explicit_unit(&site, target).unwrap();
    let ready = consumption
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();

    let prepared = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(0))
        .prepare()
        .unwrap();
    assert_eq!(
        prepared.exit(),
        PreparedFunctionExitV1::ExplicitUnit {
            block: BasicBlockId::new(0)
        }
    );
}

#[test]
fn draft_seal_marks_empty_completion_as_implicit_unit() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "draft_seal_implicit_unit",
        Vec::new(),
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let target = input.function().lowering_roots().function_pair().region();
    let completion = verify_function_completion_v1(input).unwrap();
    let consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();
    let ready = consumption
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();

    let prepared = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(0))
        .prepare()
        .unwrap();
    assert_eq!(
        prepared.exit(),
        PreparedFunctionExitV1::ImplicitUnit {
            block: BasicBlockId::new(0)
        }
    );
}

#[test]
fn draft_seal_projection_materializes_without_mutating_live_function() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "draft_seal_projection",
        vec![return_stmt(Some(literal(7)))],
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let site = completion.explicit_site().unwrap().clone();
    let target = input.function().lowering_roots().function_pair().region();
    let mut consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();
    consumption
        .claim_explicit_return(&site, target, BasicBlockId::new(0), ValueId::new(1))
        .unwrap();
    let ready = consumption
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("draft_seal_projection/0".to_string());
    let value = builder.alloc_value_for_test();
    builder
        .emit_for_test(MirInstruction::Const {
            dst: value,
            value: crate::mir::ConstValue::Integer(7),
        })
        .unwrap();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, MirType::Integer);
    let before = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .get(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .len();
    let before_next_value_id = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .next_value_id;

    let projected = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(0))
        .prepare()
        .unwrap()
        .project(&builder)
        .unwrap()
        .prepare_phi_closure()
        .unwrap()
        .prepare_type_facts()
        .unwrap()
        .prepare_metadata()
        .unwrap();
    assert_eq!(
        projected.projection().function().signature.return_type,
        MirType::Integer
    );
    assert!(matches!(
        projected
            .projection()
            .function()
            .get_block(BasicBlockId::new(0))
            .unwrap()
            .terminator,
        Some(MirInstruction::Return { value: Some(id) }) if id == value
    ));
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .get(&BasicBlockId::new(0))
            .unwrap()
            .instructions
            .len(),
        before
    );
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .next_value_id,
        before_next_value_id
    );
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .get_block(BasicBlockId::new(0))
        .unwrap()
        .terminator
        .is_none());
}

#[test]
fn draft_seal_projection_skips_reserved_void_value_ids() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "draft_seal_projection_void",
        Vec::new(),
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let target = input.function().lowering_roots().function_pair().region();
    let completion = verify_function_completion_v1(input).unwrap();
    let ready = ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion)
        .unwrap()
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("draft_seal_projection_void/0".to_string());
    builder
        .function_state
        .compilation
        .reserved_value_ids
        .insert(ValueId::new(0));
    let projected = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(0))
        .prepare()
        .unwrap()
        .project(&builder)
        .unwrap();
    let returned = match projected
        .function()
        .get_block(BasicBlockId::new(0))
        .unwrap()
        .terminator
        .as_ref()
    {
        Some(MirInstruction::Return { value: Some(value) }) => *value,
        other => panic!("expected projected return, got {other:?}"),
    };
    assert_eq!(returned, ValueId::new(1));
    assert_eq!(
        projected.type_ctx().get_type(returned),
        Some(&MirType::Void)
    );
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .get_block(BasicBlockId::new(0))
        .unwrap()
        .terminator
        .is_none());
}

#[test]
fn draft_seal_projection_prepares_stale_facts_without_live_map_mutation() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "draft_seal_projection_stale",
        Vec::new(),
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let target = input.function().lowering_roots().function_pair().region();
    let completion = verify_function_completion_v1(input).unwrap();
    let ready = ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion)
        .unwrap()
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("draft_seal_projection_stale/0".to_string());
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(ValueId::new(77), MirType::Integer);
    let before = builder.function_state.type_ctx.value_types.clone();
    let metadata_plan = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(0))
        .prepare()
        .unwrap()
        .project(&builder)
        .unwrap()
        .prepare_phi_closure()
        .unwrap()
        .prepare_type_facts()
        .unwrap()
        .prepare_metadata()
        .unwrap();
    assert!(metadata_plan.metadata().return_exit_contract.is_none());
    assert_eq!(
        metadata_plan.signature().result(),
        super::draft_seal::PreparedFunctionResultV1::Unit
    );
    let prepared = metadata_plan.prepare_stale_facts(&builder).unwrap();

    assert_eq!(prepared.stale_count(), 1);
    assert_eq!(
        prepared.projection().type_ctx().get_type(ValueId::new(77)),
        Some(&MirType::Integer)
    );
    let verified = prepared.verify().unwrap();
    assert!(verified.metadata().return_exit_contract.is_none());
    assert_eq!(
        verified.projection().type_ctx().get_type(ValueId::new(77)),
        None
    );
    assert_eq!(builder.function_state.type_ctx.value_types, before);
}

#[test]
fn open_draft_seal_prepares_and_commits_one_projected_function() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "draft_seal_open_commit",
        vec![return_stmt(Some(literal(7)))],
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    let site = completion.explicit_site().unwrap().clone();
    let target = input.function().lowering_roots().function_pair().region();
    let mut consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();
    consumption
        .claim_explicit_return(&site, target, BasicBlockId::new(0), ValueId::new(0))
        .unwrap();
    let ready = consumption
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();

    let mut builder = MirBuilder::new();
    let product = resolved_product("draft_seal_open_commit/0");
    let owner = product.owner();
    let session = builder.open_resolved_function_draft_seal_session_v1("draft_seal_open_commit/0");
    let mut open = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(0)).open(session);
    open.builder_mut()
        .function_state
        .resolved_binding_state
        .install(&product)
        .unwrap();
    open.builder_mut()
        .function_state
        .resolved_binding_state
        .finish(owner)
        .unwrap();
    open.builder_mut()
        .enter_function_for_test("draft_seal_open_commit/0".into());
    open.builder_mut()
        .emit_for_test(MirInstruction::Const {
            dst: ValueId::new(0),
            value: crate::mir::ConstValue::Integer(7),
        })
        .unwrap();
    open.builder_mut()
        .function_state
        .type_ctx
        .value_types
        .insert(ValueId::new(0), MirType::Integer);

    let prepared = match open.prepare() {
        Ok(prepared) => prepared,
        Err(rejected) => {
            rejected.discard();
            panic!("open draft-seal prepare unexpectedly rejected")
        }
    };
    let completed = prepared.commit();
    assert_eq!(completed.draft().signature.name, "draft_seal_open_commit/0");
    assert_eq!(completed.draft().signature.return_type, MirType::Integer);
    assert!(matches!(
        completed.draft().get_block(BasicBlockId::new(0)).unwrap().terminator,
        Some(MirInstruction::Return { value: Some(value) }) if value == ValueId::new(0)
    ));
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
}

#[test]
fn open_draft_seal_rejection_discards_the_unpublished_session() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "draft_seal_open_reject",
        Vec::new(),
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let target = input.function().lowering_roots().function_pair().region();
    let completion = verify_function_completion_v1(input).unwrap();
    let ready = ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion)
        .unwrap()
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();

    let mut builder = MirBuilder::new();
    let product = resolved_product("draft_seal_open_reject/0");
    let owner = product.owner();
    let session = builder.open_resolved_function_draft_seal_session_v1("draft_seal_open_reject/0");
    let mut open = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(0)).open(session);
    open.builder_mut()
        .function_state
        .resolved_binding_state
        .install(&product)
        .unwrap();
    open.builder_mut()
        .function_state
        .resolved_binding_state
        .finish(owner)
        .unwrap();
    open.builder_mut()
        .enter_function_for_test("draft_seal_open_reject/0".into());
    open.builder_mut()
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });

    let rejected = match open.prepare() {
        Ok(_) => panic!("preterminated function unexpectedly prepared"),
        Err(rejected) => rejected,
    };
    assert_eq!(
        rejected.stage(),
        super::draft_seal_owner::FunctionDraftSealStageV1::Exit
    );
    rejected.discard();
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
}
