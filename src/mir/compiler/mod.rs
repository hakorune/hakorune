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

pub(in crate::mir) mod capability;
pub(in crate::mir) mod function_input;
#[allow(dead_code)]
pub(in crate::mir) mod located;
mod lowering_input;
mod module_session;
#[allow(dead_code)]
mod source_projection;
#[allow(dead_code)]
pub(in crate::mir) mod source_view;

#[cfg(test)]
mod capability_tests;
#[cfg(test)]
mod source_view_tests;

use capability::CanonicalLoweringPreflightV1;
pub use lowering_input::{
    CanonicalLoweringErrorV1, LegacyModuleLoweringInputV1, ResolvedModuleLoweringInputV1,
    VerifiedResolvedSourceUnitV1,
};
use lowering_input::{MirLoweringRequestErrorV1, MirLoweringRequestV1};
use module_session::CanonicalModuleLoweringSessionV1;

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
}

impl MirCompiler {
    /// Create a new MIR compiler
    pub fn new() -> Self {
        Self {
            builder: MirBuilder::new(),
            verifier: MirVerifier::new(),
            optimize: true,
        }
    }

    /// Create with options
    pub fn with_options(optimize: bool) -> Self {
        Self {
            builder: MirBuilder::new(),
            verifier: MirVerifier::new(),
            optimize,
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

        // The candidate Builder is the canonical module transaction. Preflight
        // has already succeeded, and any later error discards this candidate,
        // leaving the compiler's prior Builder state untouched.
        let mut module_session = CanonicalModuleLoweringSessionV1::open(&self.builder);
        let candidate = module_session.builder_mut();
        if let Some(source) = source_file {
            candidate.set_source_file_hint(source.to_string());
        } else {
            candidate.clear_source_file_hint();
        }
        let stage_start = Instant::now();
        let module =
            candidate
                .build_resolved_function_module(plan)
                .map_err(|error| match error {
                    CanonicalResolvedBuildErrorV1::BuilderContract(detail) => {
                        CanonicalLoweringErrorV1::BuilderContract { detail }
                    }
                    CanonicalResolvedBuildErrorV1::DuplicateFunctionPublication {
                        function_name,
                    } => CanonicalLoweringErrorV1::DuplicateFunctionPublication { function_name },
                })?;
        super::compile_timing::trace_stage("build_resolved_module", stage_start.elapsed());
        let result = self.finish_built_canonical_module(module)?;
        module_session.commit(&mut self.builder);
        Ok(result)
    }

    fn finish_built_canonical_module(
        &mut self,
        module: MirModule,
    ) -> Result<MirCompileResult, CanonicalLoweringErrorV1> {
        let result = self
            .finish_built_module(module)
            .map_err(|detail| CanonicalLoweringErrorV1::BuilderContract { detail })?;

        // Legacy reports its historical pre-RC verifier result. Canonical
        // publication has a stronger barrier: verify the fully transformed
        // module after RC insertion and callsite canonicalization, and never
        // let an Err cross the module-session commit.
        let stage_start = Instant::now();
        let verification_result = self.verifier.verify_module(&result.module);
        super::compile_timing::trace_stage("canonical_post_rc_verify", stage_start.elapsed());
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

        self.finish_built_module(module)
    }

    fn finish_built_module(&mut self, mut module: MirModule) -> Result<MirCompileResult, String> {
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

        // Phase 29y.1: RC insertion pass (skeleton - no-op for now)
        // Runs after optimization and verification, before backend codegen
        let stage_start = Instant::now();
        let _rc_stats = insert_rc_instructions(&mut module);
        super::compile_timing::trace_stage("rc_insert", stage_start.elapsed());
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

impl Default for MirCompiler {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests;
