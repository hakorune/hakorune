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
use crate::mir::join_ir::lowering::loop_form_intake::LoopFormIntake;
use crate::mir::{BasicBlockId, MirQuery, ValueId};
use std::collections::{BTreeMap, BTreeSet};

// ============================================================================
// Test Fixtures (shared across all tests)
// ============================================================================

fn make_dummy_loop_form() -> crate::mir::loop_form::LoopForm {
    crate::mir::loop_form::LoopForm {
        preheader: BasicBlockId::new(1),
        header: BasicBlockId::new(2),
        body: BasicBlockId::new(3),
        latch: BasicBlockId::new(4),
        exit: BasicBlockId::new(100),
        continue_targets: vec![],
        break_targets: vec![],
    }
}

fn make_dummy_intake() -> LoopFormIntake {
    let mut header_snapshot = BTreeMap::new();
    header_snapshot.insert("s".to_string(), ValueId(10));
    header_snapshot.insert("n".to_string(), ValueId(20));
    header_snapshot.insert("i".to_string(), ValueId(30));

    LoopFormIntake {
        pinned_ordered: vec!["s".to_string(), "n".to_string()],
        carrier_ordered: vec!["i".to_string()],
        header_snapshot,
        exit_snapshots: vec![],
        exit_preds: vec![],
    }
}

struct EmptyQuery;
impl MirQuery for EmptyQuery {
    fn insts_in_block(&self, _bb: BasicBlockId) -> &[crate::mir::MirInstruction] {
        &[]
    }
    fn succs(&self, _bb: BasicBlockId) -> Vec<BasicBlockId> {
        Vec::new()
    }
    fn reads_of(&self, _inst: &crate::mir::MirInstruction) -> Vec<ValueId> {
        Vec::new()
    }
    fn writes_of(&self, _inst: &crate::mir::MirInstruction) -> Vec<ValueId> {
        Vec::new()
    }
}

// ============================================================================
// LoopScopeShape Tests (shape.rs)
// ============================================================================

#[test]
fn test_from_loop_form_basic() {
    let loop_form = make_dummy_loop_form();
    let intake = make_dummy_intake();
    let query = EmptyQuery;

    let scope = LoopScopeShape::from_loop_form(&loop_form, &intake, &query, None);

    assert!(scope.is_some());
    let scope = scope.unwrap();

    assert_eq!(scope.header, BasicBlockId::new(2));
    assert_eq!(scope.body, BasicBlockId::new(3));
    assert_eq!(scope.latch, BasicBlockId::new(4));
    assert_eq!(scope.exit, BasicBlockId::new(100));

    assert!(scope.pinned.contains("s"));
    assert!(scope.pinned.contains("n"));
    assert_eq!(scope.pinned.len(), 2);

    assert!(scope.carriers.contains("i"));
    assert_eq!(scope.carriers.len(), 1);

    assert_eq!(scope.progress_carrier, Some("i".to_string()));
}

#[test]
fn test_needs_header_phi() {
    let loop_form = make_dummy_loop_form();
    let intake = make_dummy_intake();
    let query = EmptyQuery;

    let scope = LoopScopeShape::from_loop_form(&loop_form, &intake, &query, None).unwrap();

    assert!(scope.needs_header_phi("s"));
    assert!(scope.needs_header_phi("n"));
    assert!(scope.needs_header_phi("i"));
    assert!(!scope.needs_header_phi("unknown"));
}

#[test]
fn test_needs_exit_phi() {
    let loop_form = make_dummy_loop_form();
    let intake = make_dummy_intake();
    let query = EmptyQuery;

    let scope = LoopScopeShape::from_loop_form(&loop_form, &intake, &query, None).unwrap();

    assert!(scope.needs_exit_phi("s"));
    assert!(scope.needs_exit_phi("n"));
    assert!(scope.needs_exit_phi("i"));
}

#[test]
fn test_ordered_accessors() {
    let loop_form = make_dummy_loop_form();
    let intake = make_dummy_intake();
    let query = EmptyQuery;

    let scope = LoopScopeShape::from_loop_form(&loop_form, &intake, &query, None).unwrap();

    let pinned = scope.pinned_ordered();
    assert_eq!(pinned.len(), 2);
    assert!(pinned.contains(&"s".to_string()));
    assert!(pinned.contains(&"n".to_string()));

    let carriers = scope.carriers_ordered();
    assert_eq!(carriers.len(), 1);
    assert!(carriers.contains(&"i".to_string()));
}

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
#[test]
fn test_block_ids_preserved() {
    let loop_form = crate::mir::loop_form::LoopForm {
        preheader: BasicBlockId::new(100),
        header: BasicBlockId::new(200),
        body: BasicBlockId::new(300),
        latch: BasicBlockId::new(400),
        exit: BasicBlockId::new(500),
        continue_targets: vec![],
        break_targets: vec![],
    };

    let intake = make_dummy_intake();
    let query = EmptyQuery;

    let scope = LoopScopeShape::from_loop_form(&loop_form, &intake, &query, None).unwrap();

    assert_eq!(scope.header, BasicBlockId::new(200));
    assert_eq!(scope.body, BasicBlockId::new(300));
    assert_eq!(scope.latch, BasicBlockId::new(400));
    assert_eq!(scope.exit, BasicBlockId::new(500));
}

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
#[test]
fn test_needs_phi_consistency() {
    let loop_form = make_dummy_loop_form();
    let intake = make_dummy_intake();
    let query = EmptyQuery;

    let scope = LoopScopeShape::from_loop_form(&loop_form, &intake, &query, None).unwrap();

    for var in &scope.pinned {
        assert!(
            scope.needs_header_phi(var),
            "pinned var {} should need header phi",
            var
        );
        assert!(
            scope.needs_exit_phi(var),
            "pinned var {} should need exit phi",
            var
        );
    }

    for var in &scope.carriers {
        assert!(
            scope.needs_header_phi(var),
            "carrier var {} should need header phi",
            var
        );
        assert!(
            scope.needs_exit_phi(var),
            "carrier var {} should need exit phi",
            var
        );
    }

    for var in &scope.body_locals {
        assert!(
            !scope.needs_header_phi(var),
            "body_local var {} should NOT need header phi",
            var
        );
    }
}

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

/// Phase 48-4: from_loop_form で variable_definitions が埋まることを確認
#[test]
fn test_variable_definitions_from_inspector() {
    let loop_form = make_dummy_loop_form();
    let mut intake = make_dummy_intake();

    // exit_snapshots を追加（複数の exit predecessor をシミュレート）
    let mut exit1_snap = BTreeMap::new();
    exit1_snap.insert("s".to_string(), ValueId(11));
    exit1_snap.insert("n".to_string(), ValueId(21));
    exit1_snap.insert("i".to_string(), ValueId(31));
    exit1_snap.insert("x".to_string(), ValueId(41)); // x は exit1 でのみ利用可能

    let mut exit2_snap = BTreeMap::new();
    exit2_snap.insert("s".to_string(), ValueId(12));
    exit2_snap.insert("n".to_string(), ValueId(22));
    exit2_snap.insert("i".to_string(), ValueId(32));

    intake.exit_snapshots = vec![
        (BasicBlockId::new(10), exit1_snap),
        (BasicBlockId::new(11), exit2_snap),
    ];

    let query = EmptyQuery;

    let scope = LoopScopeShape::from_loop_form(&loop_form, &intake, &query, None).unwrap();

    // s, n, i は両方の exit で利用可能 → is_available_in_all should be true
    assert!(scope.is_available_in_all("s", &[BasicBlockId::new(10), BasicBlockId::new(11)]));
    assert!(scope.is_available_in_all("n", &[BasicBlockId::new(10), BasicBlockId::new(11)]));
    assert!(scope.is_available_in_all("i", &[BasicBlockId::new(10), BasicBlockId::new(11)]));

    // x は exit1 (block 10) でのみ利用可能 → both blocks requires should be false
    assert!(!scope.is_available_in_all("x", &[BasicBlockId::new(10), BasicBlockId::new(11)]));

    // x は exit1 (block 10) のみなら true
    assert!(scope.is_available_in_all("x", &[BasicBlockId::new(10)]));
}

/// Phase 48-4: 一部のブロックでのみ利用可能な変数の判定
#[test]
fn test_variable_definitions_partial_availability() {
    let loop_form = make_dummy_loop_form();
    let mut intake = make_dummy_intake();

    // 3つの exit predecessor を用意
    let mut exit1_snap = BTreeMap::new();
    exit1_snap.insert("s".to_string(), ValueId(10));
    exit1_snap.insert("y".to_string(), ValueId(50)); // y は exit1 でのみ

    let mut exit2_snap = BTreeMap::new();
    exit2_snap.insert("s".to_string(), ValueId(11));
    exit2_snap.insert("z".to_string(), ValueId(60)); // z は exit2 でのみ

    let mut exit3_snap = BTreeMap::new();
    exit3_snap.insert("s".to_string(), ValueId(12));
    // s のみ

    intake.exit_snapshots = vec![
        (BasicBlockId::new(20), exit1_snap),
        (BasicBlockId::new(21), exit2_snap),
        (BasicBlockId::new(22), exit3_snap),
    ];

    let query = EmptyQuery;

    let scope = LoopScopeShape::from_loop_form(&loop_form, &intake, &query, None).unwrap();

    // s は全 exit で利用可能
    assert!(scope.is_available_in_all(
        "s",
        &[
            BasicBlockId::new(20),
            BasicBlockId::new(21),
            BasicBlockId::new(22)
        ]
    ));

    // y は exit1 でのみ利用可能
    assert!(scope.is_available_in_all("y", &[BasicBlockId::new(20)]));
    assert!(!scope.is_available_in_all("y", &[BasicBlockId::new(20), BasicBlockId::new(21)]));

    // z は exit2 でのみ利用可能
    assert!(scope.is_available_in_all("z", &[BasicBlockId::new(21)]));
    assert!(!scope.is_available_in_all("z", &[BasicBlockId::new(21), BasicBlockId::new(22)]));
}
