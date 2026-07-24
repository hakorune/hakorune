use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::RegionId;
use crate::mir::{BasicBlockId, MirCompiler, MirInstruction, ValueId};

use super::completion_consumption::{
    emit_canonical_explicit_return, ResolvedFunctionCompletionConsumptionV1,
};
use super::draft_seal::{PreparedFunctionExitV1, ReadyFunctionDraftSealV1};

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
fn canonical_return_never_uses_active_legacy_defer_state() {
    let mut builder = crate::mir::MirBuilder::new();
    builder.function_state.return_defer_active = true;

    let error = emit_canonical_explicit_return(&mut builder, ValueId::new(0)).unwrap_err();
    assert!(error.contains("legacy_return_state_active"));
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

    let completed = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(4))
        .prepare()
        .unwrap()
        .commit();
    assert_eq!(
        completed.exit(),
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
    consumption
        .claim_explicit_return(&site, target, BasicBlockId::new(0), ValueId::new(0))
        .unwrap();
    let ready = consumption
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();

    let completed = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(0))
        .prepare()
        .unwrap()
        .commit();
    assert_eq!(
        completed.exit(),
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

    let completed = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(0))
        .prepare()
        .unwrap()
        .commit();
    assert_eq!(
        completed.exit(),
        PreparedFunctionExitV1::ImplicitUnit {
            block: BasicBlockId::new(0)
        }
    );
}
