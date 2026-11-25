//! Macro System scaffolding (Phase 16 – MVP)
//!
//! Goal: Provide minimal, typed interfaces for AST pattern matching and
//! HIR patch based expansion. Backends (MIR/JIT/LLVM) remain unchanged.

pub mod ast_json;
pub mod ctx;
pub(crate) mod default_derive;
pub mod engine;
pub mod log;
pub mod macro_box;
pub mod macro_box_ny;
mod normal_callable_transform;
mod normal_policy;
pub(crate) use normal_policy::NormalMacroPolicyV1;
#[cfg(test)]
#[path = "normal_callable_transform_tests.rs"]
mod normal_callable_transform_tests;
pub mod pattern;
pub mod test_harness;

#[cfg(test)]
pub(crate) use normal_callable_transform::transform_normal_callable_program_v1;
pub(crate) use normal_callable_transform::{
    transform_normal_callable_program_with_policy_v1, NormalCallableTransformCompatibilityV1,
    NormalCallableTransformOutcomeV1, NormalCallableTransformRejectV1,
};

use nyash_rust::ASTNode;

/// Enable/disable macro system via env gate.
pub fn enabled() -> bool {
    // Default ON. Disable with NYASH_MACRO_DISABLE=1 or NYASH_MACRO_ENABLE=0/false/off.
    if let Some(v) = crate::config::env::macro_disable() {
        if v {
            return false;
        }
    }
    if let Some(v) = crate::config::env::macro_enable() {
        if !v {
            return false;
        }
        return true;
    }
    true
}

/// A hook to dump AST for `--expand` (pre/post). Expansion is no-op for now.
pub fn maybe_expand_and_dump(ast: &ASTNode, _dump_only: bool) -> ASTNode {
    if !enabled() {
        return ast.clone();
    }
    // Initialize user macro boxes (if any, behind env gates)
    self::macro_box::init_builtin();
    self::macro_box_ny::init_from_env();
    if crate::config::env::macro_trace() {
        crate::macro_log!("[macro] input AST: {:?}", ast);
    }
    let mut eng = self::engine::MacroEngine::new();
    let (out, _patches) = eng.expand(ast);
    let out2 = match test_harness::issue_test_harness_transform_v1(&out) {
        test_harness::TestHarnessTransformDispositionV1::Unchanged => out,
        test_harness::TestHarnessTransformDispositionV1::GeneratedTail(transformed) => transformed,
    };
    if crate::config::env::macro_trace() {
        crate::macro_log!("[macro] output AST: {:?}", out2);
    }
    require_file_functions(&out2);
    out2
}

/// Gate the AST at a file-level macro expansion boundary. Methods and
/// constructors are declarations owned by the box member inventory.
pub(crate) fn validate_file_functions(ast: &ASTNode) -> Result<(), crate::ParseError> {
    if let ASTNode::FunctionDeclaration { span, .. } = ast {
        return Err(crate::ParseError::function_outside_box(span.line));
    }
    let mut result = Ok(());
    ast.for_each_child(&mut |child| {
        if result.is_ok() {
            result = match (ast, child) {
                (ASTNode::BoxDeclaration { .. }, ASTNode::FunctionDeclaration { .. }) => {
                    let mut body_result = Ok(());
                    child.for_each_child(&mut |node| {
                        if body_result.is_ok() {
                            body_result = validate_file_functions(node);
                        }
                    });
                    body_result
                }
                _ => validate_file_functions(child),
            };
        }
    });
    result
}

pub(crate) fn require_file_functions(ast: &ASTNode) {
    if let Err(error) = validate_file_functions(ast) {
        crate::runtime::get_global_ring0().log.error(&error.to_string());
        std::process::exit(1);
    }
}
