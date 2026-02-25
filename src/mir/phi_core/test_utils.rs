//! Phase 84-5: Test utilities for PHI-related tests
//!
//! This module contains utility functions that are only used by test code.
//! Moved from if_phi.rs during Phase 84-5 cleanup.

use crate::ast::{ASTNode, Span};

/// Phase 40-4.1: JoinIR経由で代入変数を収集
///
/// AST→JSON変換を経由して、then/else ブロック内で代入される変数名を抽出します。
///
/// **使用箇所**:
/// - `src/tests/phase40_array_ext_filter_test.rs` のテストコード
/// - `src/mir/loop_builder/if_lowering.rs` (unused, dead code)
pub fn collect_assigned_vars_via_joinir(
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
) -> std::collections::BTreeSet<String> {
    let mut result = std::collections::BTreeSet::new();

    // Convert then_body to JSON and extract
    let then_prog = ASTNode::Program {
        statements: then_body.to_vec(),
        span: Span::unknown(),
    };
    let then_json = crate::r#macro::ast_json::ast_to_json(&then_prog);
    if let Some(stmts) = then_json.get("statements") {
        extract_vars_from_json_stmts(stmts, &mut result);
    }

    // Process else_body if present
    if let Some(else_statements) = else_body {
        let else_prog = ASTNode::Program {
            statements: else_statements.clone(),
            span: Span::unknown(),
        };
        let else_json = crate::r#macro::ast_json::ast_to_json(&else_prog);
        if let Some(stmts) = else_json.get("statements") {
            extract_vars_from_json_stmts(stmts, &mut result);
        }
    }

    if crate::config::env::joinir_vm_bridge_debug() {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[Phase 40-4.1] collect_assigned_vars_via_joinir: {:?}",
            result
        ));
    }

    result
}

/// Phase 40-4.1: JSON AST から代入変数を抽出（ast_to_json形式対応）
fn extract_vars_from_json_stmts(
    stmts: &serde_json::Value,
    out: &mut std::collections::BTreeSet<String>,
) {
    if let Some(arr) = stmts.as_array() {
        for stmt in arr {
            extract_vars_from_json_stmt(stmt, out);
        }
    }
}

/// Phase 40-4.1: 単一JSON文から変数抽出
fn extract_vars_from_json_stmt(
    stmt: &serde_json::Value,
    out: &mut std::collections::BTreeSet<String>,
) {
    // ast_to_json uses "kind", not "type"
    match stmt.get("kind").and_then(|k| k.as_str()) {
        Some("Local") => {
            // ast_to_json: { "kind": "Local", "variables": ["x", "y"], ... }
            if let Some(vars) = stmt.get("variables").and_then(|v| v.as_array()) {
                for var in vars {
                    if let Some(name) = var.as_str() {
                        out.insert(name.to_string());
                    }
                }
            }
        }
        Some("Assignment") => {
            // ast_to_json: { "kind": "Assignment", "target": { "kind": "Variable", "name": "x" }, ... }
            if let Some(target) = stmt.get("target") {
                if target.get("kind").and_then(|k| k.as_str()) == Some("Variable") {
                    if let Some(name) = target.get("name").and_then(|n| n.as_str()) {
                        out.insert(name.to_string());
                    }
                }
            }
        }
        Some("If") => {
            // ast_to_json: { "kind": "If", "then": [...], "else": [...] }
            if let Some(then_stmts) = stmt.get("then") {
                extract_vars_from_json_stmts(then_stmts, out);
            }
            if let Some(else_stmts) = stmt.get("else") {
                extract_vars_from_json_stmts(else_stmts, out);
            }
        }
        Some("Loop") => {
            // ast_to_json: { "kind": "Loop", "body": [...] }
            if let Some(body) = stmt.get("body") {
                extract_vars_from_json_stmts(body, out);
            }
        }
        Some("Block") => {
            // ast_to_json: { "kind": "Block", "body": [...] } - 通常は "statements" かも
            if let Some(body) = stmt.get("body") {
                extract_vars_from_json_stmts(body, out);
            }
            if let Some(stmts) = stmt.get("statements") {
                extract_vars_from_json_stmts(stmts, out);
            }
        }
        _ => {
            // その他は無視
        }
    }
}
