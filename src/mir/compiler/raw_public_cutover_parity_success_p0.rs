//! PARITY0-S0a: the first bounded Legacy-vs-Raw success witness.

use super::raw_public_cutover_parity_snapshot::snapshot_module;
use super::MirCompiler;
use crate::ast::{ASTNode, Span};

fn empty_script() -> ASTNode {
    ASTNode::Program { statements: Vec::new(), span: Span::unknown() }
}

#[test]
fn empty_script_legacy_and_raw_have_the_same_normalized_snapshot() {
    let ast = empty_script();
    let mut legacy = MirCompiler::new();
    let legacy_result = legacy
        .compile_with_source(ast.clone(), Some("parity-empty.hako"))
        .expect("legacy empty Script should compile");
    let mut raw = MirCompiler::new();
    let raw_result = raw
        .compile_raw_with_source(ast, Some("parity-empty.hako"))
        .expect("Raw empty Script should compile");

    let legacy_snapshot = snapshot_module(&legacy_result.module)
        .expect("legacy empty Script must use only the PARITY0 snapshot dialect");
    let raw_snapshot = snapshot_module(&raw_result.module)
        .expect("Raw empty Script must use only the PARITY0 snapshot dialect");
    assert_eq!(legacy_snapshot, raw_snapshot);
}
