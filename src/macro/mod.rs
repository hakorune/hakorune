//! Macro System scaffolding (Phase 16 – MVP)
//!
//! Goal: Provide minimal, typed interfaces for AST pattern matching and
//! HIR patch based expansion. Backends (MIR/JIT/LLVM) remain unchanged.

pub mod ast_json;
pub mod ctx;
pub mod engine;
pub mod log;
pub mod macro_box;
pub mod macro_box_ny;
pub mod pattern;
pub mod test_harness;

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
    let out2 = test_harness::maybe_inject_test_harness(&out);
    if crate::config::env::macro_trace() {
        crate::macro_log!("[macro] output AST: {:?}", out2);
    }
    out2
}
