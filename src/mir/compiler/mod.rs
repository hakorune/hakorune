use super::builder::{CanonicalResolvedBuildErrorV1, MirBuilder};
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

#[allow(dead_code)]
pub(in crate::mir) mod acyclic_callable_graph;
#[allow(dead_code)]
pub(in crate::mir) mod acyclic_callable_module_plan;
#[allow(dead_code)]
pub(in crate::mir) mod callable_graph_inventory;
#[allow(dead_code)]
pub(in crate::mir) mod callable_scc_partition;
#[allow(dead_code)]
mod canonical_drain_manifest;
#[allow(dead_code)]
pub(in crate::mir) mod canonical_finalization;
#[allow(dead_code)]
pub(in crate::mir) mod canonical_physical_completion;
pub(in crate::mir) mod capability;
#[allow(dead_code)]
pub(in crate::mir) mod external_commit;
pub(in crate::mir) mod function_input;
#[allow(dead_code)]
pub(in crate::mir) mod located;
mod lowering_input;
#[allow(dead_code)]
pub(in crate::mir) mod module_postprocess;
mod module_session;
#[allow(dead_code)]
pub(in crate::mir) mod raw_finalization;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_eligibility;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_children;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_eligibility_classifier;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_package;
#[allow(dead_code)]
pub(in crate::mir) mod raw_runtime_inputs;
#[allow(dead_code)]
pub(in crate::mir) mod raw_source_binding;
#[allow(dead_code)]
pub(in crate::mir) mod recursive_callable_module_plan;
#[allow(dead_code)]
pub(in crate::mir) mod resolved_callable_module;
mod resolved_callable_module_input;
#[allow(dead_code)]
pub(in crate::mir) mod resolved_callable_module_preflight;
#[allow(dead_code)]
pub(in crate::mir) mod source_bound_package;
#[allow(dead_code)]
mod source_projection;
#[allow(dead_code)]
pub(in crate::mir) mod source_view;

#[cfg(test)]
mod acyclic_callable_module_activation_tests;
#[cfg(test)]
mod callable_batch_correspondence_test_support;
#[cfg(test)]
mod callable_catalog_cutover_tests;
#[cfg(test)]
mod canonical_bridge_fixture0_p0;
#[cfg(test)]
mod canonical_drain_manifest_p0;
#[cfg(test)]
mod canonical_finalization_p0;
#[cfg(test)]
mod canonical_physical_completion_p0;
#[cfg(test)]
mod capability_tests;
#[cfg(test)]
mod drain_policy_p0;
#[cfg(test)]
mod external_commit_p0;
#[cfg(test)]
mod finite_direct_call_tests;
#[cfg(test)]
mod module_postprocess_failure_p0;
#[cfg(test)]
mod module_postprocess_p0;
#[cfg(test)]
mod module_session_borrow_p0_tests;
#[cfg(test)]
mod prod_activation_p0_r1;
#[cfg(test)]
mod raw_root_eligibility_p0;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_plan0;
#[cfg(test)]
mod raw_source_binding_p0;
#[cfg(test)]
mod recursive_callable_module_activation_tests;
#[cfg(test)]
mod resolved_callable_module_preflight_tests;
#[cfg(test)]
mod resolved_callable_module_tests;
#[cfg(test)]
mod sibling_call_tests;
#[cfg(test)]
mod source_bound_package_p0;
#[cfg(test)]
mod source_view_tests;

use crate::mir::builder::BuilderInvocationConfigV1;
use capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
pub use lowering_input::{
    CanonicalLoweringErrorV1, LegacyModuleLoweringInputV1, ResolvedModuleLoweringInputV1,
    VerifiedResolvedSourceUnitV1,
};
use lowering_input::{MirLoweringRequestErrorV1, MirLoweringRequestV1};
use module_session::CanonicalModuleLoweringSessionV1;
use raw_source_binding::{
    RawCallableMainSelectionV1, RawIngressRequestV1, RejectedRawSourceBindingV1,
    SourceBoundRawPackageV1,
};
pub use resolved_callable_module_input::{
    ResolvedCallableModuleLoweringInputV1, VerifiedResolvedCallableProgramV1,
};
use source_bound_package::{
    CanonicalPhysicalInvocationV1, ExactCanonicalPreflightPlanV1, InvocationIdentityIssuerV1,
    LoweredCanonicalPlanV1, RejectedCanonicalLoweringV1, RejectedCanonicalPhysicalOpenV1,
    RejectedCanonicalSourceBindingV1, SourceBoundCanonicalPackageV1,
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
    CurrentCanonicalAPlus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirFinishScheduleV1 {
    Canonical(CanonicalFinishScheduleV1),
    Legacy,
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
            Self::Canonical(CanonicalFinishScheduleV1::CurrentCanonicalAPlus) | Self::Legacy => {
                LegacyRcInsertionScheduleV1::Run
            }
        }
    }
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

fn map_canonical_build_error(error: CanonicalResolvedBuildErrorV1) -> CanonicalLoweringErrorV1 {
    match error {
        CanonicalResolvedBuildErrorV1::BuilderContract(detail) => {
            CanonicalLoweringErrorV1::BuilderContract { detail }
        }
        CanonicalResolvedBuildErrorV1::DuplicateFunctionPublication { function_name } => {
            CanonicalLoweringErrorV1::DuplicateFunctionPublication { function_name }
        }
    }
}

fn set_candidate_source_hint(candidate: &mut MirBuilder, source_file: Option<&str>) {
    match source_file {
        Some(source) => candidate.set_source_file_hint(source.to_string()),
        None => candidate.clear_source_file_hint(),
    }
}

/// MIR compilation result
#[derive(Debug, Clone)]
pub struct MirCompileResult {
    pub module: MirModule,
    pub verification_result: Result<(), Vec<VerificationError>>,
}

/// MIR compiler - converts AST to MIR/SSA form
pub struct MirCompiler {
    builder: MirBuilder,
    verifier: MirVerifier,
    optimize: bool,
    invocation_identity: InvocationIdentityIssuerV1,
}

/// Unpublished LOWER0 result.  The candidate session stays owned until a
/// later collector/publication row consumes this product.
pub(in crate::mir) struct CanonicalLoweringCandidateV1<'a> {
    session: CanonicalModuleLoweringSessionV1,
    lowered: LoweredCanonicalPlanV1<'a>,
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

    /// SOURCE-BIND0 sole constructor.  The exact canonical plan is the only
    /// source authority; identity is minted only after its continuation has
    /// been sealed.  LOWER0 is the only future package consumer.
    pub(in crate::mir) fn bind_canonical_source<'a>(
        &mut self,
        plan: ExactCanonicalPreflightPlanV1<'a>,
    ) -> Result<SourceBoundCanonicalPackageV1<'a>, RejectedCanonicalSourceBindingV1<'a>> {
        SourceBoundCanonicalPackageV1::bind(&mut self.invocation_identity, plan)
    }

    /// RAW-SOURCE0-BIND0 disconnected source-bound Raw ingress.  The config
    /// snapshot is taken before binding effects, and no Builder/collector/
    /// ledger/session consumer is opened by this terminal.
    pub(in crate::mir) fn bind_raw_source(
        &mut self,
        input: LegacyModuleLoweringInputV1,
        source_file: Option<&str>,
        module_name: impl Into<Box<str>>,
        callable_main: RawCallableMainSelectionV1,
    ) -> Result<SourceBoundRawPackageV1, RejectedRawSourceBindingV1> {
        let config = BuilderInvocationConfigV1::snapshot_for_raw(&self.builder, source_file);
        let request = RawIngressRequestV1::new(input, config, module_name, callable_main);
        SourceBoundRawPackageV1::bind(&mut self.invocation_identity, request)
    }

    /// RAW-SOURCE0-LOWER0-S0 disconnected package-to-draft owner handoff.
    /// Public ingress and production execution remain intentionally absent.
    pub(in crate::mir) fn begin_raw_draft(
        &self,
        package: SourceBoundRawPackageV1,
    ) -> crate::mir::builder::RawDraftInvocationV1 {
        crate::mir::builder::RawDraftInvocationV1::open(package, &self.builder)
    }

    /// OWNER0's compiler-owned physical bridge.  The package is consumed only
    /// after one real session, shell, and collector have opened from its token.
    pub(in crate::mir) fn begin_canonical_invocation<'a>(
        &mut self,
        package: SourceBoundCanonicalPackageV1<'a>,
        source_file: Option<&str>,
        module_name: String,
    ) -> Result<CanonicalPhysicalInvocationV1<'a>, RejectedCanonicalPhysicalOpenV1<'a>> {
        let config = crate::mir::builder::BuilderInvocationConfigV1::snapshot_for_canonical(
            &self.builder,
            source_file,
        );
        package.open_physical(&self.builder, config, module_name)
    }

    /// LOWER0's disconnected plan-consuming terminal.  It opens only the
    /// candidate session, moves the package into a draft lowerer, and keeps
    /// the live Builder untouched on every failure.
    pub(in crate::mir) fn lower_canonical_source<'a>(
        &mut self,
        package: SourceBoundCanonicalPackageV1<'a>,
        source_file: Option<&str>,
    ) -> Result<CanonicalLoweringCandidateV1<'a>, RejectedCanonicalLoweringV1<'a>> {
        let mut session = CanonicalModuleLoweringSessionV1::open(&self.builder);
        set_candidate_source_hint(session.builder_mut(), source_file);
        let lowered = package.consume(session.builder_mut())?;
        Ok(CanonicalLoweringCandidateV1 { session, lowered })
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
        self.compile_legacy(LegacyModuleLoweringInputV1::bare_ast(ast), source_file)
    }

    /// Compile AST to MIR with an explicit imported static-box alias table.
    pub fn compile_with_source_and_imports(
        &mut self,
        ast: crate::ast::ASTNode,
        source_file: Option<&str>,
        imports: std::collections::HashMap<String, String>,
    ) -> Result<MirCompileResult, String> {
        self.builder.comp_ctx.set_using_import_boxes(imports);
        self.compile_legacy_request(LegacyModuleLoweringInputV1::bare_ast(ast), source_file)
    }

    /// Compile syntax that carries a verified canonical source-unit seal.
    ///
    /// SA3-B activates one closed non-main static/free function family. The
    /// whole source unit is rejected before Builder effects if it is outside
    /// that capability.
    pub fn compile_resolved(
        &mut self,
        input: ResolvedModuleLoweringInputV1<'_>,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
        self.compile_request(MirLoweringRequestV1::Resolved(input), source_file)
            .map_err(MirLoweringRequestErrorV1::into_canonical)
    }

    /// Compile one exact P0c-F acyclic callable Program.
    ///
    /// This is an explicit canonical ingress. It never retries the legacy
    /// route, and every header/body/plan/draft is sealed before the caller's
    /// Builder is mutated.
    pub fn compile_resolved_callable_module(
        &mut self,
        input: ResolvedCallableModuleLoweringInputV1<'_>,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
        if self.builder.repl_mode {
            return Err(CanonicalLoweringErrorV1::UnsupportedCanonicalOwnerKind);
        }
        let plan = acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1::verify(
            input.program().module(),
        )
        .map_err(|error| callable_program_stage_error("acyclic_activation", error))?;

        let stage_start = Instant::now();
        let mut module_session = CanonicalModuleLoweringSessionV1::open(&self.builder);
        set_candidate_source_hint(module_session.builder_mut(), source_file);
        let candidate = module_session
            .builder_mut()
            .build_acyclic_callable_module_candidate(plan)
            .map_err(|error| callable_program_stage_error("module_transaction", error))?;
        super::compile_timing::trace_stage("build_resolved_callable_module", stage_start.elapsed());
        let result = self.finish_built_canonical_module(
            candidate,
            CanonicalFinishScheduleV1::TrivialBindingSsa,
        )?;
        module_session.commit(&mut self.builder);
        Ok(result)
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

    /// Compile an explicitly non-canonical AST input.
    pub fn compile_legacy(
        &mut self,
        input: LegacyModuleLoweringInputV1,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, String> {
        self.builder.comp_ctx.clear_using_import_boxes();
        self.compile_legacy_request(input, source_file)
    }

    fn compile_legacy_request(
        &mut self,
        input: LegacyModuleLoweringInputV1,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, String> {
        self.compile_request(MirLoweringRequestV1::Legacy(input), source_file)
            .map_err(MirLoweringRequestErrorV1::into_legacy)
    }

    /// The sole route-selection site. The request enum ends at this boundary.
    fn compile_request(
        &mut self,
        request: MirLoweringRequestV1<'_>,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, MirLoweringRequestErrorV1> {
        match request {
            MirLoweringRequestV1::Resolved(input) => self
                .compile_resolved_first_family(input, source_file)
                .map_err(MirLoweringRequestErrorV1::Canonical),
            MirLoweringRequestV1::Legacy(input) => {
                let (ast, _legacy_origin) = input.into_parts();
                self.compile_with_source_internal(ast, source_file)
                    .map_err(MirLoweringRequestErrorV1::Legacy)
            }
        }
    }

    fn compile_resolved_first_family(
        &mut self,
        input: ResolvedModuleLoweringInputV1<'_>,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
        if self.builder.repl_mode {
            return Err(CanonicalLoweringErrorV1::UnsupportedCanonicalOwnerKind);
        }
        let plan = CanonicalLoweringPreflightV1::verify(input.source_unit())?;

        let stage_start = Instant::now();
        // The sealed whole-unit plan is matched exactly once after preflight
        // and before the candidate session is opened. Each owner carries its
        // matching finish schedule through publication; a lowering error
        // returns directly and is never retried through the other owner.
        let (module_session, module, finish_schedule) = match plan {
            CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => {
                let mut session = CanonicalModuleLoweringSessionV1::open(&self.builder);
                set_candidate_source_hint(session.builder_mut(), source_file);
                let module = session
                    .builder_mut()
                    .build_resolved_trivial_function_module(plan)
                    .map_err(map_canonical_build_error)?;
                (
                    session,
                    module,
                    CanonicalFinishScheduleV1::TrivialBindingSsa,
                )
            }
            CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(plan) => {
                let mut session = CanonicalModuleLoweringSessionV1::open(&self.builder);
                set_candidate_source_hint(session.builder_mut(), source_file);
                let module = session
                    .builder_mut()
                    .build_resolved_function_module(plan)
                    .map_err(map_canonical_build_error)?;
                (
                    session,
                    module,
                    CanonicalFinishScheduleV1::CurrentCanonicalAPlus,
                )
            }
        };
        super::compile_timing::trace_stage("build_resolved_module", stage_start.elapsed());
        let result = self.finish_built_canonical_module(module, finish_schedule)?;
        module_session.commit(&mut self.builder);
        Ok(result)
    }

    fn finish_built_canonical_module(
        &mut self,
        module: MirModule,
        schedule: CanonicalFinishScheduleV1,
    ) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
        let result = self
            .finish_built_module(module, MirFinishScheduleV1::Canonical(schedule))
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

    fn compile_with_source_internal(
        &mut self,
        ast: crate::ast::ASTNode,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, String> {
        if let Some(src) = source_file {
            self.builder.set_source_file_hint(src.to_string());
        } else {
            self.builder.clear_source_file_hint();
        }

        // Convert AST to MIR using builder
        let stage_start = Instant::now();
        let module = self.builder.build_module(ast)?;
        super::compile_timing::trace_stage("build_module", stage_start.elapsed());

        self.finish_built_module(module, MirFinishScheduleV1::Legacy)
    }

    fn finish_built_module(
        &mut self,
        mut module: MirModule,
        schedule: MirFinishScheduleV1,
    ) -> Result<MirCompileResult, String> {
        // Builder attaches declaration runes before each function body is fully
        // lowered. Refresh once after module build so optimizer consumers see
        // body-dependent rune facts such as verified required InlinePlan.
        let stage_start = Instant::now();
        super::rune_plan_refresh::refresh_module_rune_plans(&mut module);
        super::compile_timing::trace_stage("rune_refresh", stage_start.elapsed());

        if self.optimize {
            let stage_start = Instant::now();
            let mut optimizer = MirOptimizer::new();
            let stats = optimizer.optimize_module(&mut module);
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
        refresh_module_semantic_metadata(&mut module);
        super::compile_timing::trace_stage("semantic_refresh", stage_start.elapsed());
        let stage_start = Instant::now();
        let canonicalized = super::passes::callsite_canonicalize::canonicalize_for_site(
            &mut module,
            super::passes::callsite_canonicalize::CallsiteCanonicalizeScheduleSite::MirCompilerPostRc,
        );
        super::compile_timing::trace_stage("canonicalize", stage_start.elapsed());
        if canonicalized > 0 {
            let stage_start = Instant::now();
            refresh_module_semantic_metadata(&mut module);
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
