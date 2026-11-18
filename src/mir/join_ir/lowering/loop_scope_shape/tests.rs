//! Tests for loop_scope_shape module
//!
//! ## Test Organization
//!
//! This file contains tests for multiple components:
//! - **LoopScopeShape tests** (shape.rs): from_loop_form, needs_phi, block_ids, ordering
//! - **CaseAContext tests** (case_a*.rs): validation, lowering shape
//! - **Variable classification tests** (structural.rs): classify, phi consistency
//! - **Variable definitions tests** (builder.rs): availability, inspector
//!
//! Future work: Consider splitting into per-module test files for better organization.

use super::shape::LoopVarClass;
use super::*;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::{BTreeMap, BTreeSet};

// ============================================================================
// Test Fixtures (shared across all tests)
// ============================================================================



// ============================================================================
// LoopScopeShape Tests (shape.rs)
// ============================================================================





// ============================================================================
// CaseAContext Tests (case_a*.rs)
// ============================================================================

/// CaseAContext::from_scope で header == exit のとき None を返すテスト
#[test]
fn test_from_scope_validation_header_eq_exit() {
    use crate::mir::join_ir::lowering::value_id_ranges::skip_ws as vid;

    let scope = LoopScopeShape {
        header: BasicBlockId::new(10),
        body: BasicBlockId::new(11),
        latch: BasicBlockId::new(12),
        exit: BasicBlockId::new(10),
        pinned: vec!["s".to_string()].into_iter().collect(),
        carriers: vec!["i".to_string()].into_iter().collect(),
        body_locals: BTreeSet::new(),
        exit_live: vec!["i".to_string()].into_iter().collect(),
        progress_carrier: Some("i".to_string()),
        variable_definitions: BTreeMap::new(),
    };

    let ctx = CaseAContext::from_scope(scope, "test", |offset| vid::loop_step(offset));
    assert!(
        ctx.is_none(),
        "from_scope should return None when header == exit"
    );
}

/// block IDs が LoopForm から正しく伝播されるテスト

/// BTreeSet による順序決定性の確認テスト
#[test]
fn test_deterministic_order() {
    let mut set1: BTreeSet<String> = BTreeSet::new();
    set1.insert("z".to_string());
    set1.insert("a".to_string());
    set1.insert("m".to_string());

    let mut set2: BTreeSet<String> = BTreeSet::new();
    set2.insert("m".to_string());
    set2.insert("z".to_string());
    set2.insert("a".to_string());

    let vec1: Vec<_> = set1.iter().cloned().collect();
    let vec2: Vec<_> = set2.iter().cloned().collect();

    assert_eq!(vec1, vec2);
    assert_eq!(
        vec1,
        vec!["a".to_string(), "m".to_string(), "z".to_string()]
    );
}

/// needs_header_phi と needs_exit_phi の一貫性テスト

// ============================================================================
// Variable Classification Tests (structural.rs)
// ============================================================================

/// classify() メソッドのテスト
#[test]
fn test_classify_method() {
    let scope = LoopScopeShape {
        header: BasicBlockId::new(2),
        body: BasicBlockId::new(3),
        latch: BasicBlockId::new(4),
        exit: BasicBlockId::new(100),
        pinned: vec!["s".to_string(), "n".to_string()].into_iter().collect(),
        carriers: vec!["i".to_string()].into_iter().collect(),
        body_locals: vec!["x".to_string(), "ch".to_string()]
            .into_iter()
            .collect(),
        exit_live: vec![
            "s".to_string(),
            "n".to_string(),
            "i".to_string(),
            "x".to_string(),
        ]
        .into_iter()
        .collect(),
        progress_carrier: Some("i".to_string()),
        variable_definitions: BTreeMap::new(),
    };

    assert_eq!(scope.classify("s"), LoopVarClass::Pinned);
    assert_eq!(scope.classify("n"), LoopVarClass::Pinned);
    assert_eq!(scope.classify("i"), LoopVarClass::Carrier);
    assert_eq!(scope.classify("x"), LoopVarClass::BodyLocalExit);
    assert_eq!(scope.classify("ch"), LoopVarClass::BodyLocalInternal);
    assert_eq!(scope.classify("unknown"), LoopVarClass::BodyLocalInternal);
}

/// classify() と needs_*_phi() の一貫性
#[test]
fn test_classify_phi_consistency() {
    let scope = LoopScopeShape {
        header: BasicBlockId::new(2),
        body: BasicBlockId::new(3),
        latch: BasicBlockId::new(4),
        exit: BasicBlockId::new(100),
        pinned: vec!["s".to_string()].into_iter().collect(),
        carriers: vec!["i".to_string()].into_iter().collect(),
        body_locals: vec!["x".to_string(), "ch".to_string()]
            .into_iter()
            .collect(),
        exit_live: vec!["s".to_string(), "i".to_string(), "x".to_string()]
            .into_iter()
            .collect(),
        progress_carrier: Some("i".to_string()),
        variable_definitions: BTreeMap::new(),
    };

    for var in ["s", "i", "x", "ch", "unknown"] {
        let class = scope.classify(var);
        assert_eq!(
            class.needs_header_phi(),
            scope.needs_header_phi(var),
            "classify and needs_header_phi mismatch for {}",
            var
        );
        assert_eq!(
            class.needs_exit_phi(),
            scope.needs_exit_phi(var),
            "classify and needs_exit_phi mismatch for {}",
            var
        );
    }
}

/// get_exit_live() API テスト
#[test]
fn test_get_exit_live() {
    let scope = LoopScopeShape {
        header: BasicBlockId::new(2),
        body: BasicBlockId::new(3),
        latch: BasicBlockId::new(4),
        exit: BasicBlockId::new(100),
        pinned: vec!["s".to_string()].into_iter().collect(),
        carriers: vec!["i".to_string()].into_iter().collect(),
        body_locals: BTreeSet::new(),
        exit_live: vec!["s".to_string(), "i".to_string()].into_iter().collect(),
        progress_carrier: Some("i".to_string()),
        variable_definitions: BTreeMap::new(),
    };

    let exit_live = scope.get_exit_live();
    assert_eq!(exit_live.len(), 2);
    assert!(exit_live.contains("s"));
    assert!(exit_live.contains("i"));
}

// ============================================================================
// Variable Definitions Tests (builder.rs)
// ============================================================================

/// Phase 48-4: is_available_in_all() API テスト（空の variable_definitions）
#[test]
fn test_is_available_in_all_phase48_4() {
    // Phase 48-4: variable_definitions が空のため常に false
    let scope = LoopScopeShape {
        header: BasicBlockId::new(2),
        body: BasicBlockId::new(3),
        latch: BasicBlockId::new(4),
        exit: BasicBlockId::new(100),
        pinned: vec!["s".to_string()].into_iter().collect(),
        carriers: vec!["i".to_string()].into_iter().collect(),
        body_locals: BTreeSet::new(),
        exit_live: vec!["s".to_string(), "i".to_string()].into_iter().collect(),
        progress_carrier: Some("i".to_string()),
        variable_definitions: BTreeMap::new(), // Phase 48-4: 空で初期化
    };

    // variable_definitions が空のため、すべて false を返す
    assert!(!scope.is_available_in_all("x", &[BasicBlockId::new(3)]));
    assert!(!scope.is_available_in_all("i", &[BasicBlockId::new(3)]));
    assert!(!scope.is_available_in_all("unknown", &[BasicBlockId::new(3)]));
}

/// Phase 48-5+ 想定: is_available_in_all() with variable_definitions
#[test]
fn test_is_available_in_all_phase48_5_future() {
    // Phase 48-5+ で variable_definitions が統合された状態をシミュレート
    let mut variable_definitions = BTreeMap::new();
    variable_definitions.insert(
        "x".to_string(),
        vec![BasicBlockId::new(3), BasicBlockId::new(4)]
            .into_iter()
            .collect(),
    );
    variable_definitions.insert(
        "i".to_string(),
        vec![
            BasicBlockId::new(2),
            BasicBlockId::new(3),
            BasicBlockId::new(4),
        ]
        .into_iter()
        .collect(),
    );

    let scope = LoopScopeShape {
        header: BasicBlockId::new(2),
        body: BasicBlockId::new(3),
        latch: BasicBlockId::new(4),
        exit: BasicBlockId::new(100),
        pinned: vec!["s".to_string()].into_iter().collect(),
        carriers: vec!["i".to_string()].into_iter().collect(),
        body_locals: vec!["x".to_string()].into_iter().collect(),
        exit_live: vec!["s".to_string(), "i".to_string(), "x".to_string()]
            .into_iter()
            .collect(),
        progress_carrier: Some("i".to_string()),
        variable_definitions, // Phase 48-5+ で統合された状態
    };

    // x は block 3, 4 で定義 → block 3, 4 を要求すれば true
    assert!(scope.is_available_in_all("x", &[BasicBlockId::new(3), BasicBlockId::new(4)]));

    // x は block 2 で未定義 → block 2, 3 を要求すれば false
    assert!(!scope.is_available_in_all("x", &[BasicBlockId::new(2), BasicBlockId::new(3)]));

    // i は block 2, 3, 4 で定義 → すべて要求しても true
    assert!(scope.is_available_in_all(
        "i",
        &[
            BasicBlockId::new(2),
            BasicBlockId::new(3),
            BasicBlockId::new(4)
        ]
    ));

    // unknown は variable_definitions にない → false
    assert!(!scope.is_available_in_all("unknown", &[BasicBlockId::new(3)]));
}
