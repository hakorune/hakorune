use super::*;
use crate::mir::join_ir::JoinInst;
use crate::runtime::get_global_ring0;

/// Phase 41-4.4: NestedIfMerge パターン検出のテスト
///
/// 2レベル以上のネスト if が検出されることを確認する。
#[test]
fn test_nested_if_pattern_detection_two_levels() {
    // 2レベルのネスト if: if a { if b { x = 1 } }
    let program_json = serde_json::json!({
        "defs": [{
            "name": "parse_loop",
            "params": ["src", "i"],
            "body": {
                "body": [
                    {
                        "type": "Local",
                        "name": "x",
                        "expr": {"type": "Int", "value": 0}
                    },
                    {
                        "type": "If",
                        "cond": {"type": "Var", "name": "a_cond"},
                        "then": [
                            {
                                "type": "If",
                                "cond": {"type": "Var", "name": "b_cond"},
                                "then": [
                                    {
                                        "type": "Local",
                                        "name": "x",
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
                        "expr": {"type": "Var", "name": "x"}
                    }
                ]
            }
        }]
    });

    // ExtractCtx を用意
    let mut ctx = ExtractCtx::new(2);
    ctx.register_param("src".to_string(), crate::mir::ValueId(0));
    ctx.register_param("i".to_string(), crate::mir::ValueId(1));
    ctx.register_param("a_cond".to_string(), crate::mir::ValueId(2));
    ctx.register_param("b_cond".to_string(), crate::mir::ValueId(3));

    // AST から body を取得
    let stmts = program_json["defs"][0]["body"]["body"]
        .as_array()
        .expect("body must be array");

    // パターン検出
    let lowerer = AstToJoinIrLowerer::new();
    let pattern = lowerer.try_match_nested_if_pattern(stmts, &mut ctx);

    assert!(pattern.is_some(), "2-level nested if should be detected");
    let pattern = pattern.unwrap();

    // 2つの条件が検出される
    assert_eq!(pattern.conds.len(), 2, "Should have 2 conditions");

    // 1つの変数代入が検出される
    assert_eq!(pattern.merges.len(), 1, "Should have 1 merge");
    assert_eq!(pattern.merges[0].0, "x", "Merged variable should be 'x'");
}

/// Phase 41-4.4: NestedIfMerge lowering のテスト（dev flag 必要）
///
/// HAKO_JOINIR_NESTED_IF=1 が設定されている場合のみ実行。
/// 設定されていない場合はスキップ。
#[test]
fn test_nested_if_merge_lowering() {
    // Dev flag がない場合はスキップ
    if !crate::config::env::joinir_dev::nested_if_enabled() {
        get_global_ring0().log.debug(
            "[Phase 41-4] Skipping test_nested_if_merge_lowering: \
             Set HAKO_JOINIR_NESTED_IF=1 to enable",
        );
        return;
    }

    // 2レベルのネスト if
    let program_json = serde_json::json!({
        "defs": [{
            "name": "parse_loop",
            "params": ["src", "i"],
            "body": {
                "body": [
                    {
                        "type": "Local",
                        "name": "x",
                        "expr": {"type": "Int", "value": 0}
                    },
                    {
                        "type": "If",
                        "cond": {"type": "Compare", "op": ">", "lhs": {"type": "Var", "name": "i"}, "rhs": {"type": "Int", "value": 0}},
                        "then": [
                            {
                                "type": "If",
                                "cond": {"type": "Compare", "op": "<", "lhs": {"type": "Var", "name": "i"}, "rhs": {"type": "Int", "value": 100}},
                                "then": [
                                    {
                                        "type": "Local",
                                        "name": "x",
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
                        "expr": {"type": "Var", "name": "x"}
                    }
                ]
            }
        }]
    });

    let mut lowerer = AstToJoinIrLowerer::new();
    let join_module = lowerer.lower_nested_if_pattern(&program_json);

    // JoinModule に 1 つの関数がある
    assert_eq!(join_module.functions.len(), 1, "Should have 1 function");

    // entry が設定されている
    assert!(join_module.entry.is_some(), "Entry should be set");

    // NestedIfMerge 命令が含まれている
    let entry_id = join_module.entry.unwrap();
    let entry_func = join_module
        .functions
        .get(&entry_id)
        .expect("Entry function");

    let has_nested_if_merge = entry_func
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::NestedIfMerge { .. }));

    assert!(
        has_nested_if_merge,
        "JoinFunction should contain NestedIfMerge instruction"
    );

    get_global_ring0().log.debug("[Phase 41-4] test_nested_if_merge_lowering PASSED");
}

/// Phase 41-4.4: 単一レベル if はマッチしないことを確認
#[test]
fn test_nested_if_pattern_single_level_does_not_match() {
    // 1レベルのif: if a { x = 1 }
    let program_json = serde_json::json!({
        "defs": [{
            "name": "test",
            "params": [],
            "body": {
                "body": [
                    {
                        "type": "If",
                        "cond": {"type": "Var", "name": "a"},
                        "then": [
                            {
                                "type": "Local",
                                "name": "x",
                                "expr": {"type": "Int", "value": 1}
                            }
                        ],
                        "else": []
                    }
                ]
            }
        }]
    });

    let mut ctx = ExtractCtx::new(0);
    let stmts = program_json["defs"][0]["body"]["body"]
        .as_array()
        .expect("body");

    let lowerer = AstToJoinIrLowerer::new();
    let pattern = lowerer.try_match_nested_if_pattern(stmts, &mut ctx);

    // 1レベルはマッチしない
    assert!(
        pattern.is_none(),
        "Single-level if should NOT match NestedIfMerge pattern"
    );
}

// ========================================
// Phase 45: read_quoted_from Pattern Tests
// ========================================

/// Phase 45: read_quoted_from lowering のテスト（dev flag 必要）
///
/// HAKO_JOINIR_READ_QUOTED=1 が設定されている場合のみ実行。
/// 設定されていない場合はスキップ。
#[test]
fn test_read_quoted_from_lowering() {
    // Dev flag がない場合はスキップ
    if !crate::config::env::joinir_dev::read_quoted_enabled() {
        get_global_ring0().log.debug(
            "[Phase 45] Skipping test_read_quoted_from_lowering: \
             Set HAKO_JOINIR_READ_QUOTED=1 to enable",
        );
        return;
    }

    // read_quoted_from パターンの JSON 表現
    let program_json = serde_json::json!({
        "defs": [{
            "name": "read_quoted_from",
            "params": ["s", "pos"],
            "body": {
                "body": [
                    // local i = pos
                    {
                        "type": "Local",
                        "name": "i",
                        "expr": {"type": "Var", "name": "pos"}
                    },
                    // if s.substring(i, i+1) != '"' { return "" }
                    {
                        "type": "If",
                        "cond": {
                            "type": "Compare",
                            "op": "!=",
                            "lhs": {
                                "type": "Method",
                                "receiver": {"type": "Var", "name": "s"},
                                "method": "substring",
                                "args": [
                                    {"type": "Var", "name": "i"},
                                    {"type": "Binary", "op": "+", "lhs": {"type": "Var", "name": "i"}, "rhs": {"type": "Int", "value": 1}}
                                ]
                            },
                            "rhs": {"type": "String", "value": "\""}
                        },
                        "then": [
                            {"type": "Return", "expr": {"type": "String", "value": ""}}
                        ],
                        "else": []
                    },
                    // i = i + 1
                    {
                        "type": "Local",
                        "name": "i",
                        "expr": {"type": "Binary", "op": "+", "lhs": {"type": "Var", "name": "i"}, "rhs": {"type": "Int", "value": 1}}
                    },
                    // local out = ""
                    {
                        "type": "Local",
                        "name": "out",
                        "expr": {"type": "String", "value": ""}
                    },
                    // local n = s.length()
                    {
                        "type": "Local",
                        "name": "n",
                        "expr": {
                            "type": "Method",
                            "receiver": {"type": "Var", "name": "s"},
                            "method": "length",
                            "args": []
                        }
                    },
                    // Loop (simplified, loop body handled by lower_read_quoted_pattern)
                    {
                        "type": "Loop",
                        "cond": {
                            "type": "Compare",
                            "op": "<",
                            "lhs": {"type": "Var", "name": "i"},
                            "rhs": {"type": "Var", "name": "n"}
                        },
                        "body": []
                    },
                    // return out
                    {
                        "type": "Return",
                        "expr": {"type": "Var", "name": "out"}
                    }
                ]
            }
        }]
    });

    let mut lowerer = AstToJoinIrLowerer::new();
    let join_module = lowerer.lower_read_quoted_pattern(&program_json);

    // JoinModule に 4 つの関数がある（entry, k_guard_fail, loop_step, k_exit）
    assert_eq!(join_module.functions.len(), 4, "Should have 4 functions");

    // entry が設定されている
    assert!(join_module.entry.is_some(), "Entry should be set");

    // 関数名を確認
    let func_names: Vec<&str> = join_module
        .functions
        .values()
        .map(|f| f.name.as_str())
        .collect();

    assert!(
        func_names.iter().any(|n| *n == "read_quoted_from"),
        "Should have entry function"
    );
    assert!(
        func_names.iter().any(|n| n.contains("loop_step")),
        "Should have loop_step function"
    );
    assert!(
        func_names.iter().any(|n| n.contains("k_exit")),
        "Should have k_exit function"
    );
    assert!(
        func_names.iter().any(|n| n.contains("k_guard_fail")),
        "Should have k_guard_fail function"
    );

    get_global_ring0().log.debug("[Phase 45] test_read_quoted_from_lowering PASSED");
    get_global_ring0().log.debug(&format!(
        "[Phase 45] Functions: {:?}",
        func_names
    ));
}

/// Phase 45: lowering で生成される JoinInst の種類確認
#[test]
fn test_read_quoted_from_lowering_instructions() {
    // Dev flag がない場合はスキップ
    if !crate::config::env::joinir_dev::read_quoted_enabled() {
        return;
    }

    // 簡易的な program_json（実際の AST 構造は不要、パターンが認識されればOK）
    let program_json = serde_json::json!({
        "defs": [{
            "name": "read_quoted_from",
            "params": ["s", "pos"],
            "body": { "body": [] }
        }]
    });

    let mut lowerer = AstToJoinIrLowerer::new();
    let join_module = lowerer.lower_read_quoted_pattern(&program_json);

    // entry 関数の命令を確認
    let entry_id = join_module.entry.unwrap();
    let entry_func = join_module
        .functions
        .get(&entry_id)
        .expect("Entry function");

    // entry には以下が含まれる:
    // - Compute (Const, BinOp, Compare)
    // - MethodCall (substring, length)
    // - Jump (guard check)
    // - Call (loop_step)
    // - Ret

    let has_method_call = entry_func
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::MethodCall { .. }));
    let has_jump = entry_func
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Jump { .. }));
    let has_call = entry_func
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Call { .. }));
    let has_ret = entry_func
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Ret { .. }));

    assert!(
        has_method_call,
        "Entry should have MethodCall for substring/length"
    );
    assert!(has_jump, "Entry should have Jump for guard check");
    assert!(has_call, "Entry should have Call for loop_step");
    assert!(has_ret, "Entry should have Ret");

    // loop_step 関数の命令を確認
    let loop_step_func = join_module
        .functions
        .values()
        .find(|f| f.name.contains("loop_step"))
        .expect("loop_step function");

    let loop_has_jump = loop_step_func
        .body
        .iter()
        .filter(|inst| matches!(inst, JoinInst::Jump { .. }))
        .count();

    // loop_step には 2 つの Jump がある: exit check と break check
    assert_eq!(
        loop_has_jump, 2,
        "loop_step should have 2 Jumps (exit check, break check)"
    );

    get_global_ring0().log.debug(
        "[Phase 45] test_read_quoted_from_lowering_instructions PASSED",
    );
}
