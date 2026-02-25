// Phase 40-1: JoinIR Frontend if-in-loop A/B Test
//
// Purpose:
// - Verify that JoinIR Frontend if-in-loop variable tracking helper methods work correctly
// - Unit tests for extract_assigned_vars_from_body() and extract_if_in_loop_modified_vars()
//
// Design:
// - Unit tests for JSON AST parsing and variable tracking logic
// - Integration A/B tests marked as #[ignore] until JoinIR integration is complete
//
// See: docs/private/roadmap2/phases/phase-39-if-phi-level2/joinir_extension_design.md

// NOTE: Integration A/B tests will be added after JoinIR integration is complete
// For Phase 40-1 MVP, we focus on unit tests for the helper methods

// ============================================================================
// Metadata Extraction Tests (Unit tests for helper methods)
// ============================================================================

#[test]
fn test_extract_assigned_vars_from_body() {
    // Unit test for extract_assigned_vars_from_body()
    // This tests the JSON AST recursive walk logic

    use serde_json::json;

    let body = json!([
        {"type": "Local", "name": "x", "expr": {"type": "Const", "value": 1}},
        {"type": "If", "cond": {}, "then": [
            {"type": "Local", "name": "y", "expr": {"type": "Const", "value": 2}}
        ], "else": null},
        {"type": "Loop", "cond": {}, "body": [
            {"type": "Local", "name": "z", "expr": {"type": "Const", "value": 3}}
        ]}
    ]);

    let mut lowerer = crate::mir::join_ir::frontend::ast_lowerer::AstToJoinIrLowerer::new();
    let result = lowerer.extract_assigned_vars_from_body(&body);

    assert!(result.contains("x"), "Should detect x assignment");
    assert!(
        result.contains("y"),
        "Should detect y assignment in if-branch"
    );
    assert!(
        result.contains("z"),
        "Should detect z assignment in loop-body"
    );
    assert_eq!(result.len(), 3, "Should detect exactly 3 assignments");
}

#[test]
fn test_extract_if_assigned_vars() {
    // Unit test for extract_if_assigned_vars()
    // This tests if-statement assignment filtering

    use serde_json::json;

    let body = json!([
        {"type": "Local", "name": "x", "expr": {"type": "Const", "value": 1}},
        {"type": "If", "cond": {}, "then": [
            {"type": "Local", "name": "y", "expr": {"type": "Const", "value": 2}}
        ], "else": [
            {"type": "Local", "name": "z", "expr": {"type": "Const", "value": 3}}
        ]}
    ]);

    let mut lowerer = crate::mir::join_ir::frontend::ast_lowerer::AstToJoinIrLowerer::new();
    let result = lowerer.extract_if_assigned_vars(&body);

    assert!(
        !result.contains("x"),
        "Should NOT include top-level assignment"
    );
    assert!(result.contains("y"), "Should include if-then assignment");
    assert!(result.contains("z"), "Should include if-else assignment");
    assert_eq!(result.len(), 2, "Should detect exactly 2 if-assignments");
}

#[test]
fn test_extract_if_in_loop_modified_vars() {
    // Unit test for extract_if_in_loop_modified_vars()
    // This tests the full pipeline: all_assigned ∩ if_assigned ∩ loop_vars

    use serde_json::json;
    use std::collections::HashSet;

    let loop_body = json!([
        {"type": "Local", "name": "i", "expr": {}},  // Loop counter (not in if)
        {"type": "If", "cond": {}, "then": [
            {"type": "Local", "name": "out", "expr": {}}  // Loop-carried, in if
        ], "else": null}
    ]);

    let mut loop_vars = HashSet::new();
    loop_vars.insert("i".to_string());
    loop_vars.insert("out".to_string());

    let mut lowerer = crate::mir::join_ir::frontend::ast_lowerer::AstToJoinIrLowerer::new();
    let result = lowerer.extract_if_in_loop_modified_vars(&loop_body, &loop_vars);

    assert!(
        !result.contains("i"),
        "Should NOT include non-if assignment"
    );
    assert!(
        result.contains("out"),
        "Should include if-in-loop modification"
    );
    assert_eq!(
        result.len(),
        1,
        "Should detect exactly 1 if-in-loop variable"
    );
}
