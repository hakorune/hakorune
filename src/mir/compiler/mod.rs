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
pub(crate) mod canonical_core_dispatch;
#[allow(dead_code)]
mod canonical_drain_manifest;
#[allow(dead_code)]
pub(in crate::mir) mod canonical_finalization;
#[allow(dead_code)]
pub(in crate::mir) mod canonical_physical_completion;
pub(in crate::mir) mod capability;
#[allow(dead_code)]
pub(in crate::mir) mod direct_accum_capability;
#[cfg(test)]
pub(crate) mod direct_accum_observation;
#[allow(dead_code)]
pub(in crate::mir) mod direct_accum_prefix;
#[allow(dead_code)]
pub(in crate::mir) mod direct_accum_profile;
#[allow(dead_code)]
pub(in crate::mir) mod direct_accum_projection;
#[allow(dead_code)]
pub(in crate::mir) mod external_commit;
pub(in crate::mir) mod function_input;
#[allow(dead_code)]
pub(in crate::mir) mod generic_g0_projection;
#[allow(dead_code)]
pub(in crate::mir) mod located;
#[cfg(test)] pub(crate) mod loop_cond_break_continue_observation;
#[allow(dead_code)] pub(in crate::mir) mod loop_cond_break_continue_projection;
#[cfg(test)] pub(crate) mod loop_true_break_continue_observation;
#[allow(dead_code)] pub(in crate::mir) mod loop_true_break_continue_projection;
mod lowering_input;
#[allow(dead_code)]
pub(in crate::mir) mod module_postprocess;
pub(in crate::mir) mod module_postprocess_stages;
mod module_session;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_effect_plan;
#[cfg(test)]
pub(crate) mod nested_predicate_observation;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_physical_input;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_producer;
#[cfg(test)]
mod nested_predicate_producer_tests;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_projection;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_source_handoff;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_topology;
#[cfg(test)]
pub(crate) use nested_predicate_producer_tests::nested_function as nested_function_for_p3_test;
#[cfg(test)] mod loop_true_break_continue_projection_tests;
#[cfg(test)] pub(crate) use loop_true_break_continue_projection_tests::positive_function as loop_true_function_for_test;
#[cfg(test)] mod loop_cond_break_continue_projection_tests;
#[cfg(test)] pub(crate) use loop_cond_break_continue_projection_tests::positive_function as loop_cond_function_for_test;
#[cfg(test)] mod nested_predicate_effect_plan_tests;
#[cfg(test)]
mod nested_predicate_physical_input_tests;
#[allow(dead_code)]
pub(in crate::mir) mod nested_predicate_profile;
#[cfg(test)]
mod nested_predicate_profile_tests;
#[cfg(test)]
mod nested_predicate_projection_tests;
#[cfg(test)]
mod nested_predicate_topology_tests;
mod normal_default_pipeline;
#[allow(dead_code)]
pub(crate) mod normal_source_plan;
pub(in crate::mir) mod publication_kernel;
pub(in crate::mir) mod raw_public_ingress;
pub(in crate::mir) mod raw_published_compile;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_callable_main;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_children;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_decl_access;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_drain;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_eligibility;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_eligibility_classifier;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_eligibility_prepare;
pub(in crate::mir) mod raw_root_external_commit;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_finalization;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_helper_coverage;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_package;
pub(in crate::mir) mod raw_root_postprocess;
pub(in crate::mir) mod raw_root_publication;
pub(in crate::mir) mod raw_root_publication_adapter;
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
mod resolved_direct_accum_cutover;
#[allow(dead_code)]
mod resolved_nested_predicate_cutover;
#[allow(dead_code)]
pub(in crate::mir) mod source_bound_package;
#[allow(dead_code)]
pub(in crate::mir) mod source_bound_plan;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_ny_main;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_physical;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_projection;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_published_invocation;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_reference;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_result;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_selection;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_thunk;
pub(in crate::mir) mod source_entry_vm_diagnostic;
#[cfg(feature = "vm-reference")]
pub(in crate::mir) mod source_entry_vm_execution;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_vm_invocation;
#[cfg(feature = "vm-reference")]
pub(in crate::mir) mod source_entry_vm_raw_adapter;
#[allow(dead_code)]
pub(in crate::mir) mod source_entry_vm_reference;
#[cfg(test)]
mod source_entry_vm_reference_p0;
#[cfg(feature = "vm-reference")]
pub(in crate::mir) mod source_entry_vm_runner_adapter;
#[allow(dead_code)]
pub(in crate::mir) mod source_projection;
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
mod external_commit_p0;
#[cfg(test)]
mod finite_direct_call_tests;
#[cfg(test)]
mod generic_g0_numeric_projection_tests;
#[cfg(test)]
mod generic_g0_projection_tests;
#[cfg(test)]
mod if_recipe_candidate_abort_d2_tests;
#[cfg(test)]
mod legacy_candidate_session_tests;
#[cfg(test)]
mod loop_candidate_abort_p0;
#[cfg(test)]
mod module_postprocess_failure_p0;
#[cfg(test)]
mod module_postprocess_p0;
#[cfg(test)]
mod module_session_borrow_p0_tests;
#[cfg(test)]
mod nested_if_recipe_d2_tests;
#[cfg(test)]
mod prod_activation_p0_r1;
#[cfg(test)]
mod raw_public_cutover_parity_snapshot;
#[cfg(test)]
mod raw_public_cutover_parity_success_p0;
#[cfg(test)]
mod raw_public_ingress_p0;
#[cfg(test)]
mod raw_root_body_p0;
#[cfg(test)]
mod raw_root_callable_main_p0;
#[cfg(test)]
mod raw_root_decl_access_p0;
#[cfg(test)]
mod raw_root_drain_p0;
#[cfg(test)]
mod raw_root_eligibility_p0;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_environment_manifest;
#[cfg(test)]
mod raw_root_external_commit_p0;
#[cfg(test)]
mod raw_root_finalization_p0;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_manifest_package;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_plan0;
#[cfg(test)]
mod raw_root_postprocess_p0;
#[cfg(test)]
mod raw_root_publication_adapter_p0;
#[cfg(test)]
mod raw_root_publication_p0;
#[allow(dead_code)]
pub(in crate::mir) mod raw_root_source_facts;
#[cfg(test)]
mod raw_source_binding_p0;
#[cfg(test)]
mod recursive_callable_module_activation_tests;
#[cfg(test)]
mod resolved_callable_module_preflight_tests;
#[cfg(test)]
mod resolved_callable_module_tests;
#[cfg(test)]
mod resolved_direct_accum_hardening_p0;
#[cfg(test)]
mod sibling_call_tests;
#[cfg(test)]
mod source_bound_package_p0;
#[cfg(test)]
mod source_view_tests;

use crate::mir::builder::BuilderInvocationConfigV1;
use capability::{
    CanonicalFirstFamilyPlanV1, CanonicalLoopFamilyPlanV1, CanonicalLoweringPreflightV1,
};
pub(in crate::mir) use lowering_input::LegacyModuleLoweringInputV1;
pub use lowering_input::{
    CanonicalLoweringErrorV1, ResolvedModuleLoweringInputV1, VerifiedResolvedSourceUnitV1,
};
use module_session::CanonicalModuleLoweringSessionV1;
pub use normal_default_pipeline::{
    NormalCompileRequestV1, NormalProgramCompileRequestErrorV1,
    RejectedNormalProgramCompileRequestV1,
};
pub(crate) use normal_default_pipeline::{
    RejectedPostMacroWholeFileProgramV1, VerifiedPostMacroWholeFileProgramV1,
};
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

    /// PUBLIC-INGRESS-CONFIG0: the NarrowV1 public request owns exact empty
    /// imports while disconnected Raw fixtures retain their legacy snapshot.
    pub(in crate::mir) fn bind_raw_source_for_public(
        &mut self,
        input: LegacyModuleLoweringInputV1,
        source_file: Option<&str>,
        module_name: impl Into<Box<str>>,
        callable_main: RawCallableMainSelectionV1,
        disposition: crate::mir::compiler::raw_public_ingress::RawPublicImportDispositionV1,
    ) -> Result<SourceBoundRawPackageV1, RejectedRawSourceBindingV1> {
        let config = match disposition {
            crate::mir::compiler::raw_public_ingress::RawPublicImportDispositionV1::None => {
                BuilderInvocationConfigV1::snapshot_for_raw_with_imports(
                    &self.builder,
                    source_file,
                    std::collections::HashMap::new(),
                )
            }
        };
        let request = RawIngressRequestV1::new(input, config, module_name, callable_main);
        SourceBoundRawPackageV1::bind(&mut self.invocation_identity, request)
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
        self.compile_public_program(ast, source_file, std::collections::HashMap::new())
    }

    /// Compile AST to MIR with an explicit imported static-box alias table.
    pub fn compile_with_source_and_imports(
        &mut self,
        ast: crate::ast::ASTNode,
        source_file: Option<&str>,
        imports: std::collections::HashMap<String, String>,
    ) -> Result<MirCompileResult, String> {
        self.compile_public_program(ast, source_file, imports)
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
        self.compile_resolved_first_family(input, source_file)
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
            CanonicalFirstFamilyPlanV1::Loop(CanonicalLoopFamilyPlanV1::DirectAccum(plan)) => {
                return resolved_direct_accum_cutover::compile_direct_accum_source_bound(
                    self,
                    plan,
                    source_file,
                );
            }
            CanonicalFirstFamilyPlanV1::Loop(CanonicalLoopFamilyPlanV1::NestedPredicate(plan)) => {
                return resolved_nested_predicate_cutover::compile_nested_predicate_source_bound(
                    self,
                    plan,
                    source_file,
                );
            }
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
