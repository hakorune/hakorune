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
mod callable_catalog_cutover_tests;
#[cfg(test)]
mod legacy_candidate_session_tests;
#[cfg(test)]
mod operator_call_retirement_tests;
#[cfg(test)]
mod source_view_tests;
#[cfg(test)]
mod static_result_target_only_emission_tests;
pub(in crate::mir) use lowering_input::LegacyModuleLoweringInputV1;
pub use lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use module_session::InvocationIdentityIssuerV1;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirFinishScheduleV1 {
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



fn validate_builder_operator_call_ingress_once_v1(
) -> Result<(), crate::config::env::BuilderOperatorCallIngressErrorV1> {
    crate::config::env::validate_builder_operator_call_ingress_v1()
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
impl Default for MirCompiler {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests;
