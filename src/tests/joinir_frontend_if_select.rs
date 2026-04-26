//! JoinIR Frontend — IfSelect A/B Test (Phase 34-2)
//!
//! Route A: 既存経路（AST→MIR→PHI→VM）
//! Route B: 新経路（AST→JoinIR→MIR'→VM）

use crate::mir::join_ir_ops::JoinValue;
use crate::tests::helpers::joinir_frontend::JoinIrFrontendTestRunner;

/// Phase 34-2: IfSelect simple pattern の A/B テスト
///
/// 入力: `fixtures/joinir_if_select_simple.program.json`
/// パターン: `if cond { return 10 } else { return 20 }`
/// 期待: Route A と Route B の結果が一致
#[test]
fn joinir_frontend_if_select_simple_ab_test() {
    JoinIrFrontendTestRunner::from_fixture(
        "tests/fixtures/joinir_frontend/joinir_if_select_simple.program.json",
    )
    .lower()
    .expect("Failed to lower fixture")
    .run_cases(&[
        (vec![JoinValue::Int(1)], JoinValue::Int(10)),
        (vec![JoinValue::Int(0)], JoinValue::Int(20)),
    ])
    .expect("Test cases failed");
}

/// Phase 34-3: IfSelect local pattern の A/B テスト
///
/// 入力: `fixtures/joinir_if_select_local.program.json`
/// パターン: `if cond { x=10 } else { x=20 }; return x` (意味論的)
/// 期待: simple と同じ JoinIR 出力（Select ベース）
#[test]
fn joinir_frontend_if_select_local_ab_test() {
    JoinIrFrontendTestRunner::from_fixture(
        "tests/fixtures/joinir_frontend/joinir_if_select_local.program.json",
    )
    .lower()
    .expect("Failed to lower fixture")
    .run_cases(&[
        (vec![JoinValue::Int(1)], JoinValue::Int(10)),
        (vec![JoinValue::Int(0)], JoinValue::Int(20)),
    ])
    .expect("Test cases failed");
}

/// Phase 34-6: JsonShapeToMap._read_value_from_pair/1 の完全実装テスト
///
/// 入力: `fixtures/json_shape_read_value.program.json`
/// パターン: `if at { return v.substring(0, at) } else { return v }`
/// 期待: 本物の substring 呼び出しが JoinIR MethodCall → MIR BoxCall で実行される
#[test]
fn joinir_frontend_json_shape_read_value_ab_test() {
    JoinIrFrontendTestRunner::from_fixture(
        "tests/fixtures/joinir_frontend/json_shape_read_value.program.json",
    )
    .lower()
    .expect("Failed to lower fixture")
    .run_cases(&[
        (
            vec![JoinValue::Str("hello".to_string()), JoinValue::Int(3)],
            JoinValue::Str("hel".to_string()),
        ),
        (
            vec![JoinValue::Str("world".to_string()), JoinValue::Int(0)],
            JoinValue::Str("world".to_string()),
        ),
    ])
    .expect("Test cases failed");
}

/// Phase 34-7: tiny while loop の A/B テスト
///
/// 入力: `fixtures/loop_frontend_simple.program.json`
/// パターン: `local i = 0; local acc = 0; loop(i < n) { acc = acc + 1; i = i + 1; } return acc`
/// 期待: Case-A な JoinIR (entry → loop_step → return) が生成され、正しく実行される
#[test]
fn joinir_frontend_loop_simple_ab_test() {
    JoinIrFrontendTestRunner::from_fixture(
        "tests/fixtures/joinir_frontend/loop_frontend_simple.program.json",
    )
    .lower()
    .expect("Failed to lower fixture")
    .run_cases(&[
        (vec![JoinValue::Int(0)], JoinValue::Int(0)),
        (vec![JoinValue::Int(3)], JoinValue::Int(3)),
        (vec![JoinValue::Int(5)], JoinValue::Int(5)),
    ])
    .expect("Test cases failed");
}

/// Phase 34-8: Break pattern の A/B テスト
///
/// 入力: `fixtures/loop_frontend_break.program.json`
/// パターン: `loop { if i >= n { break }; acc = acc + i; i = i + 1 }`
/// 期待: n=5 → acc=10 (0+1+2+3+4)
#[test]
fn joinir_frontend_loop_break_ab_test() {
    JoinIrFrontendTestRunner::from_fixture(
        "tests/fixtures/joinir_frontend/loop_frontend_break.program.json",
    )
    .lower()
    .expect("Failed to lower fixture")
    .run_cases(&[
        (vec![JoinValue::Int(0)], JoinValue::Int(0)),  // n=0 → 0
        (vec![JoinValue::Int(5)], JoinValue::Int(10)), // n=5 → 10 (0+1+2+3+4)
        (vec![JoinValue::Int(3)], JoinValue::Int(3)),  // n=3 → 3 (0+1+2)
    ])
    .expect("Test cases failed");
}

/// Phase 34-8: Continue pattern の A/B テスト
///
/// 入力: `fixtures/loop_frontend_continue.program.json`
/// パターン: `loop { i = i + 1; if i == 3 { continue }; acc = acc + i }`
/// 期待: n=5 → acc=12 (1+2+4+5, i==3 スキップ)
#[test]
fn joinir_frontend_loop_continue_ab_test() {
    JoinIrFrontendTestRunner::from_fixture(
        "tests/fixtures/joinir_frontend/loop_frontend_continue.program.json",
    )
    .lower()
    .expect("Failed to lower fixture")
    .run_cases(&[
        (vec![JoinValue::Int(0)], JoinValue::Int(0)),  // n=0 → 0
        (vec![JoinValue::Int(5)], JoinValue::Int(12)), // n=5 → 12 (1+2+4+5)
        (vec![JoinValue::Int(2)], JoinValue::Int(3)),  // n=2 → 3 (1+2)
    ])
    .expect("Test cases failed");
}
