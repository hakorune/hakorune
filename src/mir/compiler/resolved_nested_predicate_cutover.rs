//! The bounded Nested Predicate resolved-source consumer.
//!
//! This orchestrator reuses the existing source-bound candidate lifecycle. It
//! owns no selection, Recipe production, SSA, PHI, or fallback authority.

use super::canonical_finalization::CanonicalModuleFinalizerV1;
#[cfg(test)]
use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::external_commit::PreparedModuleExternalCommitV1;
use super::lowering_input::{
    CanonicalLoweringErrorV1, CanonicalResolvedCutoverFailureV1,
    CanonicalResolvedCutoverStageErrorV1, ResolvedModuleLoweringInputV1,
};
use super::module_postprocess::ModulePostprocessOwnerV1;
use super::nested_predicate_profile::CanonicalNestedPredicatePlanV1;
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompileResult, MirCompiler};
use crate::mir::builder::BuilderInvocationConfigV1;

pub(super) fn compile_nested_predicate_source_bound(
    compiler: &mut MirCompiler,
    plan: CanonicalNestedPredicatePlanV1<'_>,
    config: BuilderInvocationConfigV1,
) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
    let prepared = prepare_nested_predicate_source_bound(compiler, plan, config)?;
    Ok(compiler.commit_prepared_module(prepared))
}

/// Test-only late failure after the unpublished candidate has reached the
/// external-commit barrier. Dropping the prepared product must leave the live
/// compiler untouched; production has no fault-injection branch.
#[cfg(test)]
pub(in crate::mir) fn compile_nested_predicate_source_bound_with_prepared_failure_for_test(
    compiler: &mut MirCompiler,
    input: ResolvedModuleLoweringInputV1<'_>,
    source_file: Option<&str>,
) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
    let plan = CanonicalLoweringPreflightV1::verify(input.source_unit())?;
    let CanonicalFirstFamilyPlanV1::Loop(
        super::capability::CanonicalLoopFamilyPlanV1::NestedPredicate(plan),
    ) = plan
    else {
        return Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
            site: "nested_predicate_test_failure".into(),
            actual: input.source_unit().syntax_root().node_type(),
            reason: "nested_predicate_test_requires_nested_plan",
        });
    };
    let config = BuilderInvocationConfigV1::snapshot_for_canonical(&compiler.builder, source_file);
    let prepared = prepare_nested_predicate_source_bound(compiler, plan, config)?;
    drop(prepared);
    Err(bridge_error(
        CanonicalResolvedCutoverStageErrorV1::ExternalCommit(
            super::external_commit::ExternalCommitPreparationErrorV1::EvidenceMismatch,
        ),
    ))
}

fn prepare_nested_predicate_source_bound<'source>(
    compiler: &mut MirCompiler,
    plan: CanonicalNestedPredicatePlanV1<'source>,
    config: BuilderInvocationConfigV1,
) -> Result<PreparedModuleExternalCommitV1<'source>, CanonicalLoweringErrorV1> {
    let header = plan
        .seal_resolved_owner_header_v1()
        .map_err(|error| bridge_error(CanonicalResolvedCutoverStageErrorV1::Header(error)))?;
    let module_name = header.symbol().as_mir_name().to_owned();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::Loop(
            super::capability::CanonicalLoopFamilyPlanV1::NestedPredicate(plan),
        ))
        .map_err(|rejected| {
            bridge_error(CanonicalResolvedCutoverStageErrorV1::SourceBinding(
                rejected.error().clone(),
            ))
        })?;
    let finalized = compiler
        .begin_canonical_invocation_with_config(package, config, module_name)
        .map_err(|rejected| {
            bridge_error(CanonicalResolvedCutoverStageErrorV1::PhysicalOpen(
                rejected.into_error(),
            ))
        })?
        .lower()
        .map_err(|rejected| {
            bridge_error(CanonicalResolvedCutoverStageErrorV1::PhysicalLower(
                rejected.into_error(),
            ))
        })?
        .collect()
        .map_err(|rejected| {
            bridge_error(CanonicalResolvedCutoverStageErrorV1::PhysicalCollection(
                rejected.into_error(),
            ))
        })?
        .complete()
        .map_err(|rejected| {
            bridge_error(CanonicalResolvedCutoverStageErrorV1::PhysicalCompletion(
                rejected.into_error(),
            ))
        })?
        .prepare_drain()
        .map_err(|rejected| {
            bridge_error(CanonicalResolvedCutoverStageErrorV1::PhysicalDrain(
                rejected.into_error(),
            ))
        })?
        .drain()
        .prepare_finalization()
        .map_err(|rejected| {
            bridge_error(CanonicalResolvedCutoverStageErrorV1::FinalizationPrepare(
                rejected.into_error(),
            ))
        })?;
    let finalized = CanonicalModuleFinalizerV1::finalize(finalized).map_err(|rejected| {
        bridge_error(CanonicalResolvedCutoverStageErrorV1::Finalization(
            rejected.into_error(),
        ))
    })?;
    let processed = ModulePostprocessOwnerV1::new(&mut compiler.verifier, compiler.optimize)
        .run(finalized)
        .map_err(|rejected| {
            bridge_error(CanonicalResolvedCutoverStageErrorV1::Postprocess(
                rejected.into_error(),
            ))
        })?;
    compiler
        .prepare_module_external_commit(processed)
        .map_err(|error| bridge_error(CanonicalResolvedCutoverStageErrorV1::ExternalCommit(error)))
}

fn bridge_error(stage: CanonicalResolvedCutoverStageErrorV1) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::ResolvedCutover(CanonicalResolvedCutoverFailureV1::NestedPredicate(
        stage,
    ))
}
