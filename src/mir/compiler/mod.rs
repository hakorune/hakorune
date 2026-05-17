use super::builder::MirBuilder;
use super::function::MirModule;
use super::optimizer::MirOptimizer;
use super::passes::rc_insertion::insert_rc_instructions;
use super::printer::MirPrinter;
use super::semantic_refresh::refresh_module_semantic_metadata;
use super::verification::MirVerifier;
use super::verification_types::VerificationError;

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
        self.builder.comp_ctx.clear_using_import_boxes();
        self.compile_with_source_internal(ast, source_file)
    }

    /// Compile AST to MIR with an explicit imported static-box alias table.
    pub fn compile_with_source_and_imports(
        &mut self,
        ast: crate::ast::ASTNode,
        source_file: Option<&str>,
        imports: std::collections::HashMap<String, String>,
    ) -> Result<MirCompileResult, String> {
        self.builder.comp_ctx.set_using_import_boxes(imports);
        self.compile_with_source_internal(ast, source_file)
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
        let mut module = self.builder.build_module(ast)?;

        // Builder attaches declaration runes before each function body is fully
        // lowered. Refresh once after module build so optimizer consumers see
        // body-dependent rune facts such as verified required InlinePlan.
        super::rune_plan_refresh::refresh_module_rune_plans(&mut module);

        if self.optimize {
            let mut optimizer = MirOptimizer::new();
            let stats = optimizer.optimize_module(&mut module);
            if (crate::config::env::opt_diag_fail() || crate::config::env::opt_diag_forbid_legacy())
                && stats.diagnostics_reported > 0
            {
                return Err(format!(
                    "Diagnostic failure: {} issues detected (unlowered/legacy)",
                    stats.diagnostics_reported
                ));
            }
        }

        super::exact_numeric_field_contracts::refresh_module_exact_numeric_runtime_check_contracts(
            &mut module,
        );

        // Verify the generated MIR
        let verification_result = self.verifier.verify_module(&module);

        // Phase 29y.1: RC insertion pass (skeleton - no-op for now)
        // Runs after optimization and verification, before backend codegen
        let _rc_stats = insert_rc_instructions(&mut module);
        refresh_module_semantic_metadata(&mut module);
        let canonicalized =
            super::passes::callsite_canonicalize::canonicalize_callsites(&mut module);
        if canonicalized > 0 {
            refresh_module_semantic_metadata(&mut module);
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
