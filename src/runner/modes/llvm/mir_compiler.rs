//! MIR compilation for LLVM mode
//!
//! Handles AST → MIR compilation.

use nyash_rust::{ast::ASTNode, mir::MirCompiler, mir::MirModule};

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
    pub fn compile(ast: ASTNode, filename: Option<&str>) -> Result<MirModule, String> {
        let _rw_future = EnvVarRestore::set("NYASH_REWRITE_FUTURE", "1");
        let mut mir_compiler = MirCompiler::new();

        let compile_result =
            crate::runner::modes::common_util::source_hint::compile_with_source_hint(
                &mut mir_compiler,
                ast,
                filename,
            ).map_err(|e| format!("MIR compilation error: {}", e))?;

        crate::console_println!("📊 MIR Module compiled successfully!");
        crate::console_println!("📊 Functions: {}", compile_result.module.functions.len());

        Ok(compile_result.module)
    }
}
