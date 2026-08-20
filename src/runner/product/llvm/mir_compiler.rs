//! MIR compilation for LLVM mode
//!
//! Handles AST → MIR compilation.

use nyash_rust::{
    ast::ASTNode,
    mir::{MirCompileResult, MirCompiler, MirModule, NormalCompileRequestV1},
};
use std::collections::HashMap;

use super::compile_options::{FutureRewriteRoute, LlvmCompileOptions};

/// MIR compiler Box
///
/// **Responsibility**: Compile AST to MIR
/// **Input**: ast, filename
/// **Output**: Result<MirModule, String>
pub struct MirCompilerBox;

struct EnvVarRestore {
    key: &'static str,
    prev: Option<String>,
}

impl EnvVarRestore {
    fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, prev }
    }
}

impl Drop for EnvVarRestore {
    fn drop(&mut self) {
        match self.prev.as_deref() {
            Some(v) => std::env::set_var(self.key, v),
            None => std::env::remove_var(self.key),
        }
    }
}

impl MirCompilerBox {
    pub(crate) fn compile_normal_callable(
        outcome: crate::runner::modes::common_util::normal_callable::
            NormalCallableMaterializationOutcomeV1,
        filename: Option<&str>,
        imports: HashMap<String, String>,
        options: LlvmCompileOptions,
    ) -> Result<MirCompileResult, String> {
        let target_capability = options.issue_pinned_text_target_capability()?;
        let request = match outcome {
            crate::runner::modes::common_util::normal_callable::
                NormalCallableMaterializationOutcomeV1::SourceBacked(source) => {
                NormalCompileRequestV1::for_llvm_callable_source(source, filename, imports)
            }
            crate::runner::modes::common_util::normal_callable::
                NormalCallableMaterializationOutcomeV1::Compatibility(origin) =>
                NormalCompileRequestV1::for_llvm_compatibility(origin, filename, imports),
        };
        Self::compile_request(
            request.with_compile_target_capability(target_capability),
            options,
        )
    }

    /// Compile AST to MIR
    ///
    /// This function compiles the AST to MIR using source hint for better error messages.
    pub fn compile(
        ast: ASTNode,
        filename: Option<&str>,
        imports: HashMap<String, String>,
        options: LlvmCompileOptions,
    ) -> Result<MirModule, String> {
        let target_capability = options.issue_pinned_text_target_capability()?;
        let request = NormalCompileRequestV1::for_llvm_source(ast, filename, imports)
            .map_err(|error| format!("MIR compilation error: {error}"))?;
        Self::compile_request(
            request.with_compile_target_capability(target_capability),
            options,
        )
        .map(|result| result.module)
    }

    fn compile_request(
        request: NormalCompileRequestV1,
        options: LlvmCompileOptions,
    ) -> Result<MirCompileResult, String> {
        let _rw_future = match options.future_rewrite_route {
            FutureRewriteRoute::EnvFutureExterns => {
                Some(EnvVarRestore::set("NYASH_REWRITE_FUTURE", "1"))
            }
        };
        let mut mir_compiler = MirCompiler::new();

        let compile_result = mir_compiler
            .compile_normal(request)
            .map_err(|e| format!("MIR compilation error: {}", e))?;

        crate::console_println!("📊 MIR Module compiled successfully!");
        crate::console_println!("📊 Functions: {}", compile_result.module.functions.len());

        Ok(compile_result)
    }
}
