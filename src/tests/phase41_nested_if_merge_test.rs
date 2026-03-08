//! Phase 41-4: NestedIfMerge A/B Test
//!
//! Route A: 既存経路（AST→MIR→if_phi/conservative→VM）
//! Route B: 新経路（AST→JoinIR(NestedIfMerge)→MIR→VM）
//!
//! このテストは HAKO_JOINIR_NESTED_IF=1 フラグでのみ Route B を有効化する。

use crate::mir::join_ir::frontend::AstToJoinIrLowerer;
use crate::mir::join_ir::JoinInst;
use crate::mir::join_ir_ops::JoinValue;
use crate::mir::join_ir_vm_bridge::run_joinir_via_vm;

/// Phase 41-4.5: NestedIfMerge パスが有効化されることを確認
///
/// dev flag HAKO_JOINIR_NESTED_IF=1 がある場合のみ、
/// `nested_if_merge` route key が NestedIfMerge 命令を生成することを確認する。
#[test]
fn phase41_nested_if_merge_path_activation() {
    // Dev flag がない場合はスキップ
    if !crate::config::env::joinir_dev::nested_if_enabled() {
        eprintln!(
            "[Phase 41-4] Skipping phase41_nested_if_merge_path_activation: \
             Set HAKO_JOINIR_NESTED_IF=1 to enable"
        );
        return;
    }

    // 2レベルのネスト if: if a > 0 { if b > 0 { result = 42 } }; return result
    let program_json = serde_json::json!({
        "defs": [{
            "name": "nested_if_merge",
            "params": ["a", "b"],
            "body": {
                "body": [
                    {
                        "type": "Local",
                        "name": "result",
                        "expr": {"type": "Int", "value": 0}
                    },
                    {
                        "type": "If",
                        "cond": {"type": "Compare", "op": ">", "lhs": {"type": "Var", "name": "a"}, "rhs": {"type": "Int", "value": 0}},
                        "then": [
                            {
                                "type": "If",
                                "cond": {"type": "Compare", "op": ">", "lhs": {"type": "Var", "name": "b"}, "rhs": {"type": "Int", "value": 0}},
                                "then": [
                                    {
                                        "type": "Local",
                                        "name": "result",
                                        "expr": {"type": "Int", "value": 42}
                                    }
                                ],
                                "else": []
                            }
                        ],
                        "else": []
                    },
                    {
                        "type": "Return",
                        "expr": {"type": "Var", "name": "result"}
                    }
                ]
            }
        }]
    });

    let mut lowerer = AstToJoinIrLowerer::new();
    let join_module = lowerer.lower_program_json(&program_json);

    // NestedIfMerge 命令が含まれていることを確認
    let entry_id = join_module.entry.expect("Entry should be set");
    let entry_func = join_module
        .functions
        .get(&entry_id)
        .expect("Entry function");

    let nested_if_merge_count = entry_func
        .body
        .iter()
        .filter(|inst| matches!(inst, JoinInst::NestedIfMerge { .. }))
        .count();

    assert!(
        nested_if_merge_count > 0,
        "[Phase 41-4.5] NestedIfMerge instruction not found in nested_if_merge. \
         Expected at least 1, found {}",
        nested_if_merge_count
    );

    eprintln!(
        "[Phase 41-4.5] NestedIfMerge path activated: {} NestedIfMerge instructions generated",
        nested_if_merge_count
    );
}

/// Phase 41-4.6: Route B (JoinIR NestedIfMerge) の実行テスト
///
/// 2レベルのネスト if パターンを JoinIR で lowering し、
/// VM Bridge 経由で実行して正しい結果を得ることを確認する。
#[test]
fn phase41_nested_if_merge_route_b_execution() {
    // Dev flag がない場合はスキップ
    if !crate::config::env::joinir_dev::nested_if_enabled() {
        eprintln!(
            "[Phase 41-4] Skipping phase41_nested_if_merge_route_b_execution: \
             Set HAKO_JOINIR_NESTED_IF=1 to enable"
        );
        return;
    }

    // 2レベルのネスト if
    let program_json = serde_json::json!({
        "defs": [{
            "name": "nested_if_merge",
            "params": ["a", "b"],
            "body": {
                "body": [
                    {
                        "type": "Local",
                        "name": "result",
                        "expr": {"type": "Int", "value": 0}
                    },
                    {
                        "type": "If",
                        "cond": {"type": "Compare", "op": ">", "lhs": {"type": "Var", "name": "a"}, "rhs": {"type": "Int", "value": 0}},
                        "then": [
                            {
                                "type": "If",
                                "cond": {"type": "Compare", "op": ">", "lhs": {"type": "Var", "name": "b"}, "rhs": {"type": "Int", "value": 0}},
                                "then": [
                                    {
                                        "type": "Local",
                                        "name": "result",
                                        "expr": {"type": "Int", "value": 42}
                                    }
                                ],
                                "else": []
                            }
                        ],
                        "else": []
                    },
                    {
                        "type": "Return",
                        "expr": {"type": "Var", "name": "result"}
                    }
                ]
            }
        }]
    });

    let mut lowerer = AstToJoinIrLowerer::new();
    let join_module = lowerer.lower_program_json(&program_json);
    let entry_id = join_module.entry.expect("Entry should be set");

    // テストケース: (a, b) -> expected result
    let test_cases = [
        // 両方 true -> 42
        ((5, 5), 42),
        // a > 0 だが b <= 0 -> 0 (else path)
        ((5, 0), 0),
        ((5, -1), 0),
        // a <= 0 -> 0 (outer else path)
        ((0, 5), 0),
        ((-1, 5), 0),
        // 両方 false -> 0
        ((0, 0), 0),
        ((-1, -1), 0),
    ];

    for ((a, b), expected) in test_cases {
        let inputs = vec![JoinValue::Int(a), JoinValue::Int(b)];

        let result = run_joinir_via_vm(&join_module, entry_id, &inputs)
            .expect(&format!("Failed to execute with inputs ({}, {})", a, b));

        assert_eq!(
            result,
            JoinValue::Int(expected),
            "[Phase 41-4.6] Route B execution failed for ({}, {}): expected {}, got {:?}",
            a,
            b,
            expected,
            result
        );
    }

    eprintln!(
        "[Phase 41-4.6] Route B execution test PASSED: {} test cases",
        test_cases.len()
    );
}

/// Phase 41-4.6: Route A/B 結果比較テスト
///
/// Route A（既存 if_phi/conservative）と Route B（JoinIR NestedIfMerge）の
/// 実行結果が一致することを確認する。
///
/// 注意: このテストは現時点では Route B の実行結果のみを確認する。
/// Route A との完全な比較は、nested_if_merge route が本番パイプラインに統合された後に行う。
#[test]
fn phase41_nested_if_merge_route_ab_comparison() {
    // Dev flag がない場合はスキップ
    if !crate::config::env::joinir_dev::nested_if_enabled() {
        eprintln!(
            "[Phase 41-4] Skipping phase41_nested_if_merge_route_ab_comparison: \
             Set HAKO_JOINIR_NESTED_IF=1 to enable"
        );
        return;
    }

    // Route B の実行結果を確認（Route A は nested_if_merge 統合後に比較予定）
    let program_json = serde_json::json!({
        "defs": [{
            "name": "nested_if_merge",
            "params": ["a", "b"],
            "body": {
                "body": [
                    {
                        "type": "Local",
                        "name": "result",
                        "expr": {"type": "Int", "value": 0}
                    },
                    {
                        "type": "If",
                        "cond": {"type": "Compare", "op": ">", "lhs": {"type": "Var", "name": "a"}, "rhs": {"type": "Int", "value": 0}},
                        "then": [
                            {
                                "type": "If",
                                "cond": {"type": "Compare", "op": ">", "lhs": {"type": "Var", "name": "b"}, "rhs": {"type": "Int", "value": 0}},
                                "then": [
                                    {
                                        "type": "Local",
                                        "name": "result",
                                        "expr": {"type": "Int", "value": 42}
                                    }
                                ],
                                "else": []
                            }
                        ],
                        "else": []
                    },
                    {
                        "type": "Return",
                        "expr": {"type": "Var", "name": "result"}
                    }
                ]
            }
        }]
    });

    let mut lowerer = AstToJoinIrLowerer::new();
    let join_module = lowerer.lower_program_json(&program_json);
    let entry_id = join_module.entry.expect("Entry should be set");

    // Route B: JoinIR経由で実行
    let route_b_result = run_joinir_via_vm(
        &join_module,
        entry_id,
        &[JoinValue::Int(5), JoinValue::Int(5)],
    )
    .expect("Route B execution failed");

    // 期待結果: a=5, b=5 -> 両方 > 0 -> result = 42
    let expected = JoinValue::Int(42);

    assert_eq!(
        route_b_result, expected,
        "[Phase 41-4.6] Route B result mismatch: expected {:?}, got {:?}",
        expected, route_b_result
    );

    eprintln!(
        "[Phase 41-4.6] Route A/B comparison: Route B = {:?} (Route A comparison deferred to pipeline integration)",
        route_b_result
    );
}
