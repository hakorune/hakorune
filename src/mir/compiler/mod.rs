use super::builder::MirBuilder;
use super::function::DynamicV2MetadataPairObservation;
use super::function::MirModule;
use super::optimizer::MirOptimizer;
use super::passes::rc_insertion::insert_rc_instructions;
use super::printer::MirPrinter;
use super::semantic_refresh::{
    refresh_and_validate_for_boundary, refresh_module_semantic_metadata, ContractRefreshBoundary,
};
use super::verification::MirVerifier;
use super::verification_types::VerificationError;
use std::time::Instant;

// Keep the large module topology out of the compiler entrypoint. The included
// declarations are expanded in this module, so paths, visibility, cfgs, and
// sibling `super::` relationships remain unchanged.
include!("module_registry.in.rs");

#[cfg(test)]
mod callable_batch_correspondence_test_support;
#[cfg(test)]
mod callable_catalog_cutover_tests;
#[cfg(test)]
mod capability_tests;
#[cfg(test)]
mod finite_direct_call_tests;
#[cfg(test)]
mod generic_g0_numeric_projection_tests;
#[cfg(test)]
mod generic_g0_observation_tests;
#[cfg(test)]
mod generic_g0_projection_tests;
#[cfg(test)]
mod legacy_candidate_session_tests;
#[cfg(test)]
mod module_session_borrow_p0_tests;
#[cfg(test)]
mod operator_call_retirement_tests;
#[cfg(test)]
mod recursive_callable_module_activation_tests;
#[cfg(test)]
mod resolved_callable_module_tests;
#[cfg(test)]
mod source_view_tests;
#[cfg(test)]
mod static_result_target_only_emission_tests;
pub(in crate::mir) use lowering_input::LegacyModuleLoweringInputV1;
pub use lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use module_session::{CanonicalModuleLoweringSessionV1, InvocationIdentityIssuerV1};
pub(in crate::mir) use normal_default_pipeline::published_backend_view;
pub(crate) use normal_default_pipeline::{
    emit_lifecycle_physical_abi_json, PublishedLifecyclePhysicalAbiInputV1,
    PublishedLifecycleRuntimeRequirementsV1,
};
pub use normal_default_pipeline::{
    NormalCompileRequestV1, NormalProgramCompileRequestErrorV1,
    RejectedNormalProgramCompileRequestV1,
};
pub(crate) use normal_default_pipeline::{
    NormalPublishedCompileOutcome, RejectedPostMacroWholeFileProgramV1,
    VerifiedPostMacroWholeFileProgramV1,
};
pub use resolved_callable_module_input::{
    ResolvedCallableModuleLoweringInputV1, VerifiedResolvedCallableProgramV1,
};

/// Closed post-build schedule selected with the canonical lowering owner.
///
/// The schedule is carried separately from the MIR module so a selected
/// Binding-SSA owner cannot accidentally enter legacy RC insertion.  Route
/// selection remains pre-Builder; this value only materializes that sealed
/// decision after the candidate module has been built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CanonicalFinishScheduleV1 {
    TrivialBindingSsa,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirFinishScheduleV1 {
    Canonical(CanonicalFinishScheduleV1),
    Legacy,
    SelectedDynamic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LegacyRcInsertionScheduleV1 {
    Skip,
    Run,
}

impl MirFinishScheduleV1 {
    fn legacy_rc_insertion(self) -> LegacyRcInsertionScheduleV1 {
        match self {
            Self::Canonical(CanonicalFinishScheduleV1::TrivialBindingSsa) => {
                LegacyRcInsertionScheduleV1::Skip
            }
            Self::Legacy => LegacyRcInsertionScheduleV1::Run,
            Self::SelectedDynamic => LegacyRcInsertionScheduleV1::Skip,
        }
    }
}

/// Select the post-build schedule from the already-sealed module metadata.
/// The pair observation is the sole selected-route authority; this helper does
/// not inspect names, AST, or MIR instructions to reconstruct the decision.
fn finish_schedule_for_normal_module(module: &MirModule) -> Result<MirFinishScheduleV1, String> {
    let mut selected_function: Option<&str> = None;
    for (name, function) in &module.functions {
        match function.metadata.selected_dynamic_metadata_observation() {
            DynamicV2MetadataPairObservation::Ordinary => {}
            DynamicV2MetadataPairObservation::Selected { .. } => {
                if selected_function.replace(name.as_str()).is_some() {
                    return Err("selected Dynamic metadata pair count exceeds one".to_owned());
                }
            }
            DynamicV2MetadataPairObservation::Scrubbed => {
                return Err(format!(
                    "selected Dynamic metadata pair is scrubbed for function {name}"
                ));
            }
            DynamicV2MetadataPairObservation::Partial => {
                return Err(format!(
                    "selected Dynamic metadata pair is partial for function {name}"
                ));
            }
        }
    }
    Ok(if selected_function.is_some() {
        MirFinishScheduleV1::SelectedDynamic
    } else {
        MirFinishScheduleV1::Legacy
    })
}

fn require_canonical_verification(
    verification_result: Result<(), Vec<VerificationError>>,
) -> Result<(), CanonicalLoweringErrorV1> {
    verification_result.map_err(|errors| CanonicalLoweringErrorV1::MirVerificationFailed {
        errors: errors
            .into_iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    })
}

fn set_candidate_source_hint(candidate: &mut MirBuilder, source_file: Option<&str>) {
    match source_file {
        Some(source) => candidate.set_source_file_hint(source.to_string()),
        None => candidate.clear_source_file_hint(),
    }
}

fn validate_builder_operator_call_ingress_once_v1(
) -> Result<(), crate::config::env::BuilderOperatorCallIngressErrorV1> {
    crate::config::env::validate_builder_operator_call_ingress_v1()
}

fn map_operator_call_ingress_error(
    error: crate::config::env::BuilderOperatorCallIngressErrorV1,
) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::OperatorCallIngress {
        detail: error.to_string(),
    }
}

/// MIR compilation result
#[derive(Debug, Clone)]
pub struct MirCompileResult {
    pub module: MirModule,
    pub verification_result: Result<(), Vec<VerificationError>>,
}

impl MirCompileResult {
    /// Consume a compile result only after its final verification succeeded.
    /// This is the selected physical boundary; ordinary compatibility callers
    /// may continue to inspect the legacy result fields explicitly.
    pub fn into_verified_module(self) -> Result<MirModule, String> {
        match self.verification_result {
            Ok(()) => Ok(self.module),
            Err(errors) => Err(errors
                .into_iter()
                .map(|error| error.to_string())
                .collect::<Vec<_>>()
                .join("; ")),
        }
    }
}

/// MIR compiler - converts AST to MIR/SSA form
pub struct MirCompiler {
    builder: MirBuilder,
    verifier: MirVerifier,
    optimize: bool,
    invocation_identity: InvocationIdentityIssuerV1,
}

impl MirCompiler {
    /// Create a new MIR compiler
    pub fn new() -> Self {
        Self {
            builder: MirBuilder::new(),
            verifier: MirVerifier::new(),
            optimize: true,
            invocation_identity: InvocationIdentityIssuerV1::new(),
        }
    }

    /// Create with options
    pub fn with_options(optimize: bool) -> Self {
        Self {
            builder: MirBuilder::new(),
            verifier: MirVerifier::new(),
            optimize,
            invocation_identity: InvocationIdentityIssuerV1::new(),
        }
    }

    /// Phase 288 P2: Set REPL mode flag
    pub fn set_repl_mode(&mut self, repl_mode: bool) {
        self.builder.repl_mode = repl_mode;
    }

    /// Phase 288: REPL mode での内部ログ抑制フラグを設定
    pub fn set_quiet_internal_logs(&mut self, quiet: bool) {
        self.builder.comp_ctx.quiet_internal_logs = quiet;
    }

    /// Compile AST to MIR module with verification
    pub fn compile_with_source(
        &mut self,
        ast: crate::ast::ASTNode,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, String> {
        self.compile_public_program(ast, source_file, std::collections::HashMap::new())
    }

    fn compile_public_program(
        &mut self,
        ast: crate::ast::ASTNode,
        source_file: Option<&str>,
        imports: std::collections::HashMap<String, String>,
    ) -> Result<MirCompileResult, String> {
        let request = NormalCompileRequestV1::for_mir_mode(ast, source_file, imports).map_err(
            |rejected| {
                let message = rejected.error().to_string();
                rejected.discard();
                message
            },
        )?;
        self.compile_normal(request)
    }

    /// Compile one exact P0c-MR recursive callable Program.
    ///
    /// This explicit ingress never probes the acyclic, self-call, or legacy
    /// routes. Every SCC, function plan, and draft is sealed before the
    /// caller's Builder is mutated.
    pub fn compile_resolved_recursive_callable_module(
        &mut self,
        input: ResolvedCallableModuleLoweringInputV1<'_>,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
        validate_builder_operator_call_ingress_once_v1()
            .map_err(map_operator_call_ingress_error)?;
        if self.builder.repl_mode {
            return Err(CanonicalLoweringErrorV1::UnsupportedCanonicalOwnerKind);
        }
        let plan = recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1::verify(
            input.program().module(),
        )
        .map_err(|error| callable_program_stage_error("recursive_activation", error))?;

        let stage_start = Instant::now();
        let mut module_session = CanonicalModuleLoweringSessionV1::open(&self.builder);
        set_candidate_source_hint(module_session.builder_mut(), source_file);
        let candidate = module_session
            .builder_mut()
            .build_recursive_callable_module_candidate(plan)
            .map_err(|error| callable_program_stage_error("recursive_transaction", error))?;
        super::compile_timing::trace_stage(
            "build_resolved_recursive_callable_module",
            stage_start.elapsed(),
        );
        let result = self.finish_built_canonical_module(
            candidate,
            CanonicalFinishScheduleV1::TrivialBindingSsa,
        )?;
        module_session.commit(&mut self.builder);
        Ok(result)
    }

    fn finish_built_canonical_module(
        &mut self,
        module: MirModule,
        schedule: CanonicalFinishScheduleV1,
    ) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
        let result = self
            .finish_built_module(
                module,
                MirFinishScheduleV1::Canonical(schedule),
                &Default::default(),
            )
            .map_err(|detail| CanonicalLoweringErrorV1::BuilderContract { detail })?;

        // Legacy reports its historical pre-transform verifier result.
        // Canonical publication has a stronger barrier: verify the fully
        // transformed module after its selected finish schedule and callsite
        // canonicalization, and never let an Err cross the module-session
        // commit.
        let stage_start = Instant::now();
        let verification_result = self.verifier.verify_module(&result.module);
        super::compile_timing::trace_stage(
            "canonical_post_transform_verify",
            stage_start.elapsed(),
        );
        require_canonical_verification(verification_result)?;

        Ok(MirCompileResult {
            module: result.module,
            verification_result: Ok(()),
        })
    }

    fn finish_built_module(
        &mut self,
        mut module: MirModule,
        schedule: MirFinishScheduleV1,
        scalar_roots: &crate::mir::passes::dce::RequiredLivenessRoots,
    ) -> Result<MirCompileResult, String> {
        if schedule == MirFinishScheduleV1::SelectedDynamic {
            // DraftSeal and the selected metadata pair are already closed at
            // this boundary.  No generic optimizer, RC insertion, metadata
            // refresh, or callsite canonicalizer may borrow the sealed module.
            let verification_result = MirVerifier::new_strict().verify_module(&module);
            return Ok(MirCompileResult {
                module,
                verification_result,
            });
        }

        // Builder attaches declaration runes before each function body is fully
        // lowered. Refresh once after module build so optimizer consumers see
        // body-dependent rune facts such as verified required InlinePlan.
        let stage_start = Instant::now();
        super::rune_plan_refresh::refresh_module_rune_plans(&mut module);
        super::compile_timing::trace_stage("rune_refresh", stage_start.elapsed());

        if self.optimize {
            let stage_start = Instant::now();
            let mut optimizer = MirOptimizer::new();
            let stats = optimizer.optimize_module_with_required_roots(&mut module, scalar_roots);
            super::compile_timing::trace_stage("optimize", stage_start.elapsed());
            if (crate::config::env::opt_diag_fail() || crate::config::env::opt_diag_forbid_legacy())
                && stats.diagnostics_reported > 0
            {
                return Err(format!(
                    "Diagnostic failure: {} issues detected (unlowered/legacy)",
                    stats.diagnostics_reported
                ));
            }
        }

        let stage_start = Instant::now();
        {
            let _contracts =
                refresh_and_validate_for_boundary(&mut module, ContractRefreshBoundary::Verifier)?;
        }
        super::compile_timing::trace_stage("pre_verify_refresh", stage_start.elapsed());

        // Verify the generated MIR
        let stage_start = Instant::now();
        let verification_result = self.verifier.verify_module(&module);
        super::compile_timing::trace_stage("verify", stage_start.elapsed());

        match schedule.legacy_rc_insertion() {
            LegacyRcInsertionScheduleV1::Skip => {}
            LegacyRcInsertionScheduleV1::Run => {
                // Phase 29y.1 legacy RC insertion. The current A+ and legacy
                // routes retain their historical behavior. A selected
                // Binding-SSA route is forbidden from entering this pass.
                let stage_start = Instant::now();
                let _rc_stats = insert_rc_instructions(&mut module);
                super::compile_timing::trace_stage("rc_insert", stage_start.elapsed());
            }
        }
        let stage_start = Instant::now();
        refresh_module_semantic_metadata(&mut module)?;
        super::compile_timing::trace_stage("semantic_refresh", stage_start.elapsed());
        let stage_start = Instant::now();
        let canonicalized = super::passes::callsite_canonicalize::canonicalize_for_site(
            &mut module,
            super::passes::callsite_canonicalize::CallsiteCanonicalizeScheduleSite::MirCompilerPostRc,
        );
        super::compile_timing::trace_stage("canonicalize", stage_start.elapsed());
        if canonicalized > 0 {
            let stage_start = Instant::now();
            refresh_module_semantic_metadata(&mut module)?;
            super::compile_timing::trace_stage(
                "semantic_refresh_after_canonicalize",
                stage_start.elapsed(),
            );
        }

        Ok(MirCompileResult {
            module,
            verification_result,
        })
    }

    /// Compile AST to MIR module with verification (no source hint).
    pub fn compile(&mut self, ast: crate::ast::ASTNode) -> Result<MirCompileResult, String> {
        self.compile_with_source(ast, None)
    }

    /// Dump MIR to string for debugging
    pub fn dump_mir(&self, module: &MirModule) -> String {
        MirPrinter::new().print_module(module)
    }
}
fn callable_program_stage_error(
    stage: &'static str,
    error: impl std::fmt::Debug,
) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::SourceUnitResolution {
        detail: format!("[freeze:contract][canonical_callable_module/{stage}] {error:?}"),
    }
}
impl Default for MirCompiler {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests;
