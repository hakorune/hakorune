use super::completion_test_support::*;
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::RegionId;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourceStmtSiteV1,
};
use crate::mir::{BasicBlockId, MirBuilder, MirCompiler, MirInstruction, MirType, ValueId};
use super::completion_consumption::ResolvedFunctionCompletionConsumptionV1;
use super::draft_seal::{
    DetachedFunctionExitClaimSetV1, FunctionDraftSealPreparationErrorV1,
    MultiSiteExitPreparationErrorV1, PreparedFunctionExitV1, ReadyFunctionDraftSealV1,
};


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
fn multi_site_completion_claims_are_keyed_by_source_site_and_returned_in_source_order() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "completion_multi_site",
        vec![if_return(1), return_stmt(Some(literal(2)))],
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let target = input.function().lowering_roots().function_pair().region();
    let completion = verify_function_completion_v1(input).unwrap();
    assert_eq!(completion.explicit_sites().len(), 2);
    let sites = completion.explicit_sites().to_vec();
    let mut consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();

    // Claim in reverse order to prove the source site, not Vec insertion
    // order, is the matching key.
    consumption
        .claim_explicit_return(&sites[1], target, BasicBlockId::new(11), ValueId::new(21))
        .unwrap();
    consumption
        .claim_explicit_return(&sites[0], target, BasicBlockId::new(10), ValueId::new(20))
        .unwrap();
    let ready = consumption
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();

    let ready = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(11));
    let detached = DetachedFunctionExitClaimSetV1::prepare(&ready).unwrap();
    assert_eq!(detached.claims().len(), 2);
    assert_eq!(detached.claims()[0].site(), &sites[0]);
    assert_eq!(detached.claims()[1].site(), &sites[1]);
    assert_eq!(
        detached.claims()[0].exit(),
        PreparedFunctionExitV1::ExplicitValue {
            block: BasicBlockId::new(10),
            value: ValueId::new(20),
        }
    );
    let pair = detached.into_exact_two().unwrap();
    assert_eq!(pair[0].site(), &sites[0]);
    assert_eq!(pair[1].site(), &sites[1]);

    let error = ready.prepare_exit_borrowed().unwrap_err();
    assert_eq!(
        error,
        FunctionDraftSealPreparationErrorV1::MultipleExplicitReturnClaimsUnsupported
    );
}

#[test]
fn detached_multi_site_exit_rejects_single_site_and_unit_claims() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "completion_single_site_detached",
        vec![return_stmt(Some(literal(1)))],
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let target = input.function().lowering_roots().function_pair().region();
    let completion = verify_function_completion_v1(input).unwrap();
    let site = completion.explicit_site().unwrap().clone();
    let mut consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();
    consumption
        .claim_explicit_return(&site, target, BasicBlockId::new(1), ValueId::new(2))
        .unwrap();
    let ready = consumption
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();
    let ready = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(1));
    assert_eq!(
        DetachedFunctionExitClaimSetV1::prepare(&ready),
        Err(MultiSiteExitPreparationErrorV1::ExplicitReturnClaimCountNotTwo { actual: 1 })
    );

    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "completion_multi_unit_detached",
        vec![if_return_unit(), return_stmt(None)],
    ))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let target = input.function().lowering_roots().function_pair().region();
    let completion = verify_function_completion_v1(input).unwrap();
    let sites = completion.explicit_sites().to_vec();
    let mut consumption =
        ResolvedFunctionCompletionConsumptionV1::new(input.owner(), completion).unwrap();
    consumption.claim_explicit_unit(&sites[0], target).unwrap();
    consumption.claim_explicit_unit(&sites[1], target).unwrap();
    let ready = consumption
        .finish(body.site(), body.statements().len() as u32, target)
        .unwrap();
    let ready = ReadyFunctionDraftSealV1::new(ready, BasicBlockId::new(1));
    assert_eq!(
        DetachedFunctionExitClaimSetV1::prepare(&ready),
        Err(MultiSiteExitPreparationErrorV1::ExplicitReturnUnitClaim)
    );
}
