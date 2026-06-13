use super::*;

/// Phase 186: Unit tests for LoopBodyLocalInitLowerer
///
/// These tests verify the core lowering logic without needing complex AST construction.
/// Full integration tests are in apps/tests/phase186_*.hako files.

#[test]
fn test_condition_env_basic() {
    // Smoke test: ConditionEnv creation
    let mut env = ConditionEnv::new();
    env.insert("pos".to_string(), ValueId(10));
    assert_eq!(env.get("pos"), Some(ValueId(10)));
}

#[test]
fn test_loop_body_local_env_integration() {
    // Verify LoopBodyLocalEnv works with init lowerer
    let mut env = LoopBodyLocalEnv::new();
    env.insert("temp".to_string(), ValueId(100));
    assert_eq!(env.get("temp"), Some(ValueId(100)));
    assert_eq!(env.len(), 1);
}

#[test]
fn test_skip_duplicate_check() {
    // Test that env.get() correctly identifies existing variables
    let mut env = LoopBodyLocalEnv::new();
    env.insert("temp".to_string(), ValueId(999));

    // Simulates the skip logic in lower_single_init
    if env.get("temp").is_some() {
        // Should enter this branch
        assert_eq!(env.get("temp"), Some(ValueId(999)));
    } else {
        panic!("Should have found existing variable");
    }
}

// Note: Full lowering tests (with actual AST nodes) are in integration tests:
// - apps/tests/phase186_p2_body_local_digit_pos_min.hako
// - apps/tests/phase184_body_local_update.hako (regression)
// - apps/tests/phase185_p2_body_local_int_min.hako (regression)
//
// Building AST manually in Rust is verbose and error-prone.
// Integration tests provide better coverage with real .hako code.
