//! MIR compilation for LLVM mode
//!
//! Handles AST → MIR compilation.

use nyash_rust::{
    ast::ASTNode,
    mir::{MirCompiler, MirModule, NormalCompileRequestV1},
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
    /// Compile AST to MIR
    ///
    /// This function compiles the AST to MIR using source hint for better error messages.
    pub fn compile(
        ast: ASTNode,
        filename: Option<&str>,
        imports: HashMap<String, String>,
        options: LlvmCompileOptions,
    ) -> Result<MirModule, String> {
        let _rw_future = match options.future_rewrite_route {
            FutureRewriteRoute::EnvFutureExterns => {
                Some(EnvVarRestore::set("NYASH_REWRITE_FUTURE", "1"))
            }
        };
        let mut mir_compiler = MirCompiler::new();

        let request = NormalCompileRequestV1::for_llvm_source(ast, filename, imports);
        let compile_result = mir_compiler
            .compile_normal(request)
            .map_err(|e| format!("MIR compilation error: {}", e))?;

        crate::console_println!("📊 MIR Module compiled successfully!");
        crate::console_println!("📊 Functions: {}", compile_result.module.functions.len());

        Ok(compile_result.module)
    }
}
