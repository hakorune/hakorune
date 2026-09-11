//! The bounded Generic G0 source-bound production terminal.
//!
//! This is the Generic sibling of the existing Loop cutovers.  It only
//! orchestrates the shared Single lifecycle; source facts, physical lowering,
//! completion, and publication remain owned by their existing modules.

use super::canonical_finalization::CanonicalModuleFinalizerV1;
use super::capability::{CanonicalGenericG0PlanV1, ResolvedOwnerHeaderSealErrorV1};
use super::canonical_finalization::CanonicalFinalizationErrorV1;
use super::canonical_physical_completion::{
    CanonicalDrainPrepareErrorV1, CanonicalPhysicalCompletionErrorV1,
};
use super::external_commit::ExternalCommitPreparationErrorV1;
use super::module_postprocess::ModulePostprocessErrorV1;
use super::external_commit::PreparedModuleExternalCommitV1;
use super::generic_g0_physical_operation_cohort::{
    GenericG0PhysicalEmitterAdmissionRejectV1, GenericG0PhysicalOperationCohortRejectV1,
};
use super::lowering_input::{
    CanonicalGenericG0BoundaryErrorV1, CanonicalLoweringErrorV1,
};
use super::module_postprocess::ModulePostprocessOwnerV1;
use super::source_bound_package::{
    CanonicalPhysicalOpenErrorV1, CanonicalPlanLoweringErrorV1, ExactCanonicalPreflightPlanV1,
    SourceBindingErrorV1,
};
use crate::mir::builder::BuilderInvocationConfigV1;
use crate::mir::builder::CanonicalPhysicalCollectionErrorV1;
use crate::mir::builder::resolved_lowering::CanonicalResolvedBuildErrorV1;
use super::{MirCompileResult, MirCompiler};

pub(super) fn compile_generic_g0_source_bound(
    compiler: &mut MirCompiler,
    plan: CanonicalGenericG0PlanV1<'_>,
    config: BuilderInvocationConfigV1,
) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
    let prepared = prepare_generic_g0_source_bound(compiler, plan, config)?;
    Ok(compiler.commit_prepared_module(prepared))
}

/// Test-only late failure after the candidate has crossed the external-commit
/// preparation barrier. Dropping the prepared product is the sole operation;
/// production retains exactly one commit edge.
#[cfg(test)]
pub(in crate::mir) fn compile_generic_g0_source_bound_with_prepared_failure_for_test(
    compiler: &mut MirCompiler,
    plan: CanonicalGenericG0PlanV1<'_>,
    source_file: Option<&str>,
) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
    let config = BuilderInvocationConfigV1::snapshot_for_canonical(&compiler.builder, source_file);
    let prepared = prepare_generic_g0_source_bound(compiler, plan, config)?;
    drop(prepared);
    Err(CanonicalLoweringErrorV1::BuilderContract {
        detail: "generic_g0/test_injected_prepared_commit_failure".to_owned(),
    })
}

fn prepare_generic_g0_source_bound<'source>(
    compiler: &mut MirCompiler,
    plan: CanonicalGenericG0PlanV1<'source>,
    config: BuilderInvocationConfigV1,
) -> Result<PreparedModuleExternalCommitV1<'source>, CanonicalLoweringErrorV1> {
    let header = plan
        .seal_resolved_owner_header_v1()
        .map_err(map_header_error)?;
    let module_name = header.symbol().as_mir_name().to_owned();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::Loop(
            super::capability::CanonicalLoopFamilyPlanV1::GenericG0(plan),
        ))
        .map_err(|rejected| map_source_binding_error(rejected.error().clone()))?;
    let finalized = compiler
        .begin_canonical_invocation_with_config(package, config, module_name)
        .map_err(|rejected| map_physical_open_error(rejected.into_error()))?
        .lower()
        .map_err(|rejected| map_physical_lower_error(rejected.into_error()))?
        .collect()
        .map_err(|rejected| map_physical_collection_error(rejected.into_error()))?
        .complete()
        .map_err(|rejected| map_physical_completion_error(rejected.into_error()))?
        .prepare_drain()
        .map_err(|rejected| map_physical_drain_error(rejected.into_error()))?
        .drain()
        .prepare_finalization()
        .map_err(|rejected| map_finalization_prepare_error(rejected.into_error()))?;
    let finalized = CanonicalModuleFinalizerV1::finalize(finalized)
        .map_err(|rejected| map_finalization_error(rejected.into_error()))?;
    let processed = ModulePostprocessOwnerV1::new(&mut compiler.verifier, compiler.optimize)
        .run(finalized)
        .map_err(|rejected| map_postprocess_error(rejected.into_error()))?;
    compiler
        .prepare_module_external_commit(processed)
        .map_err(map_external_commit_error)
}

fn generic_g0_error(error: CanonicalGenericG0BoundaryErrorV1) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::GenericG0(error)
}

fn map_header_error(error: ResolvedOwnerHeaderSealErrorV1) -> CanonicalLoweringErrorV1 {
    generic_g0_error(CanonicalGenericG0BoundaryErrorV1::Header(error))
}

fn map_source_binding_error(error: SourceBindingErrorV1) -> CanonicalLoweringErrorV1 {
    generic_g0_error(CanonicalGenericG0BoundaryErrorV1::SourceBinding(error))
}

fn map_physical_open_error(error: CanonicalPhysicalOpenErrorV1) -> CanonicalLoweringErrorV1 {
    generic_g0_error(CanonicalGenericG0BoundaryErrorV1::PhysicalOpen(error))
}

fn map_physical_lower_error(error: CanonicalPlanLoweringErrorV1) -> CanonicalLoweringErrorV1 {
    let error = match error {
        CanonicalPlanLoweringErrorV1::Single(
            CanonicalResolvedBuildErrorV1::GenericG0Admission(error),
        ) => map_admission_error(error),
        CanonicalPlanLoweringErrorV1::Single(
            CanonicalResolvedBuildErrorV1::GenericG0Lowerer(detail),
        ) => CanonicalGenericG0BoundaryErrorV1::Lowerer(detail),
        other => CanonicalGenericG0BoundaryErrorV1::PhysicalLower(other),
    };
    generic_g0_error(error)
}

fn map_admission_error(
    error: GenericG0PhysicalEmitterAdmissionRejectV1,
) -> CanonicalGenericG0BoundaryErrorV1 {
    match error {
        GenericG0PhysicalEmitterAdmissionRejectV1::SourceParent(error) => {
            CanonicalGenericG0BoundaryErrorV1::SourceParent(error)
        }
        GenericG0PhysicalEmitterAdmissionRejectV1::Cohort(
            GenericG0PhysicalOperationCohortRejectV1::SourceParent(error),
        ) => CanonicalGenericG0BoundaryErrorV1::SourceParent(error),
        GenericG0PhysicalEmitterAdmissionRejectV1::Cohort(error) => {
            CanonicalGenericG0BoundaryErrorV1::Cohort(error)
        }
        error => CanonicalGenericG0BoundaryErrorV1::Admission(error),
    }
}

fn map_physical_collection_error(
    error: CanonicalPhysicalCollectionErrorV1,
) -> CanonicalLoweringErrorV1 {
    generic_g0_error(CanonicalGenericG0BoundaryErrorV1::PhysicalCollection(error))
}

fn map_physical_completion_error(
    error: CanonicalPhysicalCompletionErrorV1,
) -> CanonicalLoweringErrorV1 {
    generic_g0_error(CanonicalGenericG0BoundaryErrorV1::PhysicalCompletion(error))
}

fn map_physical_drain_error(error: CanonicalDrainPrepareErrorV1) -> CanonicalLoweringErrorV1 {
    generic_g0_error(CanonicalGenericG0BoundaryErrorV1::PhysicalDrain(error))
}

fn map_finalization_prepare_error(error: CanonicalFinalizationErrorV1) -> CanonicalLoweringErrorV1 {
    generic_g0_error(CanonicalGenericG0BoundaryErrorV1::FinalizationPrepare(error))
}

fn map_finalization_error(error: CanonicalFinalizationErrorV1) -> CanonicalLoweringErrorV1 {
    generic_g0_error(CanonicalGenericG0BoundaryErrorV1::Finalization(error))
}

fn map_postprocess_error(error: ModulePostprocessErrorV1) -> CanonicalLoweringErrorV1 {
    generic_g0_error(CanonicalGenericG0BoundaryErrorV1::Postprocess(error))
}

fn map_external_commit_error(error: ExternalCommitPreparationErrorV1) -> CanonicalLoweringErrorV1 {
    generic_g0_error(CanonicalGenericG0BoundaryErrorV1::ExternalCommit(error))
}
