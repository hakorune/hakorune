//! MIR compilation for LLVM mode
//!
//! Handles AST → MIR compilation.

use nyash_rust::mir::{MirCompileResult, MirCompiler, NormalCompileRequestV1};
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
        let result =
            Self::compile_with(outcome, filename, imports, options, |compiler, request| {
                compiler.compile_normal(request)
            })?;
        crate::console_println!("📊 MIR Module compiled successfully!");
        crate::console_println!("📊 Functions: {}", result.module.functions.len());
        Ok(result)
    }

    pub(crate) fn compile_normal_callable_with_published<R>(
        outcome: crate::runner::modes::common_util::normal_callable::NormalCallableMaterializationOutcomeV1,
        filename: Option<&str>,
        imports: HashMap<String, String>,
        options: LlvmCompileOptions,
        consume: impl for<'m> FnOnce(
            &crate::mir::function::PublishedMirBackendView<'m>,
            &Result<(), Vec<crate::mir::VerificationError>>,
        ) -> Result<R, String>,
    ) -> Result<crate::mir::NormalPublishedCompileOutcome<R>, String> {
        Self::compile_with(outcome, filename, imports, options, |compiler, request| {
            compiler.compile_normal_with_published(request, consume)
        })
    }

    fn compile_with<R>(
        outcome: crate::runner::modes::common_util::normal_callable::NormalCallableMaterializationOutcomeV1,
        filename: Option<&str>,
        imports: HashMap<String, String>,
        options: LlvmCompileOptions,
        compile: impl FnOnce(&mut MirCompiler, NormalCompileRequestV1) -> Result<R, String>,
    ) -> Result<R, String> {
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
        let _rw_future = match options.future_rewrite_route {
            FutureRewriteRoute::EnvFutureExterns => {
                Some(EnvVarRestore::set("NYASH_REWRITE_FUTURE", "1"))
            }
        };
        let mut mir_compiler = MirCompiler::new();

        compile(
            &mut mir_compiler,
            request.with_compile_target_capability(target_capability),
        )
        .map_err(|e| format!("MIR compilation error: {}", e))
    }
}
