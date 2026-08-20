//! LLVM MIR compile options.
//!
//! This names LLVM compile-time policy without letting the runner mutate env
//! variables ad hoc.

use nyash_rust::{
    ast::ASTNode,
    mir::{compile_target_capability::*, MirCompileResult, MirModule},
};
use std::collections::HashMap;

use super::mir_compiler::MirCompilerBox;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FutureRewriteRoute {
    EnvFutureExterns,
}

impl FutureRewriteRoute {
    pub fn report_value(self) -> &'static str {
        match self {
            FutureRewriteRoute::EnvFutureExterns => "env_forced_llvm_future_externs",
        }
    }

    pub fn option_value(self) -> &'static str {
        match self {
            FutureRewriteRoute::EnvFutureExterns => "env_future_externs",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LlvmCompileOptions {
    pub future_rewrite_route: FutureRewriteRoute,
    pub(crate) pinned_text_target_profile: PinnedTextCompileTargetProfileV1,
}

impl LlvmCompileOptions {
    pub fn current_default() -> Self {
        Self {
            future_rewrite_route: FutureRewriteRoute::EnvFutureExterns,
            pinned_text_target_profile:
                PinnedTextCompileTargetProfileV1::NyRtTextResidencePtr64As0V1,
        }
    }

    pub(crate) fn issue_pinned_text_target_capability(
        &self,
    ) -> Result<PinnedTextCompileTargetCapabilityV1, String> {
        PinnedTextCompileTargetCapabilityIssuerV1::issue(self.pinned_text_target_profile)
            .map_err(|error| error.to_string())
    }
}

pub struct CompileOptionsBox;

impl CompileOptionsBox {
    pub(crate) fn compile_normal_callable(
        outcome: crate::runner::modes::common_util::normal_callable::
            NormalCallableMaterializationOutcomeV1,
        filename: Option<&str>,
        imports: HashMap<String, String>,
        options: LlvmCompileOptions,
    ) -> Result<MirCompileResult, String> {
        MirCompilerBox::compile_normal_callable(outcome, filename, imports, options)
    }

    pub fn compile(
        ast: ASTNode,
        filename: Option<&str>,
        imports: HashMap<String, String>,
        options: LlvmCompileOptions,
    ) -> Result<MirModule, String> {
        MirCompilerBox::compile(ast, filename, imports, options)
    }
}
