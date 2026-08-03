//! The first production DirectAccum consumer.
//!
//! This module only orchestrates the existing source-bound candidate
//! lifecycle.  It does not own selection, Recipe production, SSA, PHI, or a
//! second module ingress.  A DirectAccum plan is already sealed before this
//! function is called; every failure therefore drops the unpublished
//! candidate and leaves the live compiler Builder untouched.

use super::canonical_finalization::CanonicalModuleFinalizerV1;
use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::direct_accum_profile::CanonicalDirectAccumPlanV1;
use super::external_commit::PreparedModuleExternalCommitV1;
use super::lowering_input::{CanonicalLoweringErrorV1, ResolvedModuleLoweringInputV1};
use super::module_postprocess::ModulePostprocessOwnerV1;
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompileResult, MirCompiler};
use crate::mir::verification::MirVerifier;

pub(super) fn compile_direct_accum_source_bound(
    compiler: &mut MirCompiler,
    plan: CanonicalDirectAccumPlanV1<'_>,
    source_file: Option<&str>,
) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
    let prepared = prepare_direct_accum_source_bound(compiler, plan, source_file)?;
    Ok(compiler.commit_prepared_module(prepared))
}

fn prepare_direct_accum_source_bound<'source>(
    compiler: &mut MirCompiler,
    plan: CanonicalDirectAccumPlanV1<'source>,
    source_file: Option<&str>,
) -> Result<PreparedModuleExternalCommitV1<'source>, CanonicalLoweringErrorV1> {
    let header = plan
        .seal_resolved_owner_header_v1()
        .map_err(|error| bridge_error("header", error))?;
    let module_name = header.symbol().as_mir_name().to_owned();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::Loop(
            super::capability::CanonicalLoopFamilyPlanV1::DirectAccum(plan),
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
    let processed = run_postprocess(&mut compiler.verifier, compiler.optimize, finalized)?;
    compiler
        .prepare_module_external_commit(processed)
        .map_err(|error| bridge_error("external_prepare", error))
}

/// Test-only late failure after every candidate stage has completed. Dropping
/// the prepared product is the only operation exercised here; production
/// callers still have exactly one commit edge.
#[cfg(test)]
pub(in crate::mir) fn compile_direct_accum_source_bound_with_prepared_failure_for_test(
    compiler: &mut MirCompiler,
    input: ResolvedModuleLoweringInputV1<'_>,
    source_file: Option<&str>,
) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
    let plan = CanonicalLoweringPreflightV1::verify(input.source_unit())?;
    let CanonicalFirstFamilyPlanV1::Loop(
        super::capability::CanonicalLoopFamilyPlanV1::DirectAccum(plan),
    ) = plan
    else {
        return Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
            site: "direct_accum_test_failure".into(),
            actual: input.source_unit().syntax_root().node_type(),
            reason: "direct_accum_test_requires_direct_plan",
        });
    };
    let prepared = prepare_direct_accum_source_bound(compiler, plan, source_file)?;
    drop(prepared);
    Err(CanonicalLoweringErrorV1::BuilderContract {
        detail: "direct_accum/test_injected_prepared_commit_failure".into(),
    })
}

fn run_postprocess<'source>(
    verifier: &mut MirVerifier,
    optimize: bool,
    finalized: super::canonical_finalization::FinalizedModuleInvocationV1<'source>,
) -> Result<
    super::module_postprocess::PostprocessedModuleInvocationV1<'source>,
    CanonicalLoweringErrorV1,
> {
    ModulePostprocessOwnerV1::new(verifier, optimize)
        .run(finalized)
        .map_err(|rejected| bridge_error("postprocess", rejected.error()))
}

/// Keep bridge-stage failures inside the existing canonical typed terminal.
/// No stage can authorize a retry or a legacy route.
fn bridge_error(stage: &'static str, error: impl std::fmt::Debug) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::BuilderContract {
        detail: format!("direct_accum/{stage}: {error:?}"),
    }
}
