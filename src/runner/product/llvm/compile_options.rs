//! LLVM MIR compile options.
//!
//! This names LLVM compile-time policy without letting the runner mutate env
//! variables ad hoc.

use nyash_rust::{ast::ASTNode, mir::MirModule};

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
}

impl LlvmCompileOptions {
    pub fn current_default() -> Self {
        Self {
            future_rewrite_route: FutureRewriteRoute::EnvFutureExterns,
        }
    }
}

pub struct CompileOptionsBox;

impl CompileOptionsBox {
    pub fn compile(
        ast: ASTNode,
        filename: Option<&str>,
        options: LlvmCompileOptions,
    ) -> Result<MirModule, String> {
        MirCompilerBox::compile(ast, filename, options)
    }
}
