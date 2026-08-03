//! The bounded Nested Predicate resolved-source consumer.
//!
//! This orchestrator reuses the existing source-bound candidate lifecycle. It
//! owns no selection, Recipe production, SSA, PHI, or fallback authority.

use super::canonical_finalization::CanonicalModuleFinalizerV1;
#[cfg(test)]
use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::external_commit::PreparedModuleExternalCommitV1;
use super::lowering_input::{CanonicalLoweringErrorV1, ResolvedModuleLoweringInputV1};
use super::module_postprocess::ModulePostprocessOwnerV1;
use super::nested_predicate_profile::CanonicalNestedPredicatePlanV1;
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompileResult, MirCompiler};

pub(super) fn compile_nested_predicate_source_bound(
    compiler: &mut MirCompiler,
    plan: CanonicalNestedPredicatePlanV1<'_>,
    source_file: Option<&str>,
) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
    let prepared = prepare_nested_predicate_source_bound(compiler, plan, source_file)?;
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
    let prepared = prepare_nested_predicate_source_bound(compiler, plan, source_file)?;
    drop(prepared);
    Err(CanonicalLoweringErrorV1::BuilderContract {
        detail: "nested_predicate/test_injected_prepared_commit_failure".into(),
    })
}

fn prepare_nested_predicate_source_bound<'source>(
    compiler: &mut MirCompiler,
    plan: CanonicalNestedPredicatePlanV1<'source>,
    source_file: Option<&str>,
) -> Result<PreparedModuleExternalCommitV1<'source>, CanonicalLoweringErrorV1> {
    let header = plan
        .seal_resolved_owner_header_v1()
        .map_err(|error| bridge_error("header", error))?;
    let module_name = header.symbol().as_mir_name().to_owned();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::Loop(
            super::capability::CanonicalLoopFamilyPlanV1::NestedPredicate(plan),
        ))
        .map_err(|rejected| bridge_error("source_binding", rejected.error()))?;
    let finalized = compiler
        .begin_canonical_invocation(package, source_file, module_name)
        .map_err(|rejected| bridge_error("physical_open", rejected.error()))?
        .lower()
        .map_err(|rejected| bridge_error("physical_lower", rejected.error()))?
        .collect()
        .map_err(|rejected| bridge_error("physical_collect", rejected.error()))?
        .complete()
        .map_err(|rejected| bridge_error("physical_complete", &rejected.error))?
        .prepare_drain()
        .map_err(|rejected| bridge_error("physical_drain", &rejected.error))?
        .drain()
        .prepare_finalization()
        .map_err(|rejected| bridge_error("finalization_prepare", &rejected.error))?;
    let finalized = CanonicalModuleFinalizerV1::finalize(finalized)
        .map_err(|rejected| bridge_error("finalization", &rejected.error))?;
    let processed = ModulePostprocessOwnerV1::new(&mut compiler.verifier, compiler.optimize)
        .run(finalized)
        .map_err(|rejected| bridge_error("postprocess", rejected.error()))?;
    compiler
        .prepare_module_external_commit(processed)
        .map_err(|error| bridge_error("external_prepare", error))
}

fn bridge_error(stage: &'static str, error: impl std::fmt::Debug) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::BuilderContract {
        detail: format!("nested_predicate/{stage}: {error:?}"),
    }
}
