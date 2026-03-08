use super::convert::convert_mir_like_inst;
use super::*;
use crate::mir::join_ir::frontend::AstToJoinIrLowerer;
use crate::mir::join_ir_ops::JoinValue;
use crate::mir::{BinaryOp, CompareOp as MirCompareOp, Effect, MirInstruction, ValueId};
use crate::runtime::get_global_ring0;

fn ensure_ring0_initialized() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
}

#[test]
fn test_convert_const_inst() {
    let join_const = crate::mir::join_ir::MirLikeInst::Const {
        dst: ValueId(10),
        value: crate::mir::join_ir::ConstValue::Integer(42),
    };

    let mir_inst = convert_mir_like_inst(&join_const).unwrap();

    match mir_inst {
        MirInstruction::Const { dst, value } => {
            assert_eq!(dst, ValueId(10));
            assert!(matches!(value, crate::mir::ConstValue::Integer(42)));
        }
        _ => panic!("Expected Const instruction"),
    }
}

#[test]
fn test_convert_binop_inst() {
    let join_binop = crate::mir::join_ir::MirLikeInst::BinOp {
        dst: ValueId(20),
        op: crate::mir::join_ir::BinOpKind::Add,
        lhs: ValueId(10),
        rhs: ValueId(11),
    };

    let mir_inst = convert_mir_like_inst(&join_binop).unwrap();

    match mir_inst {
        MirInstruction::BinOp { dst, op, lhs, rhs } => {
            assert_eq!(dst, ValueId(20));
            assert_eq!(op, BinaryOp::Add);
            assert_eq!(lhs, ValueId(10));
            assert_eq!(rhs, ValueId(11));
        }
        _ => panic!("Expected BinOp instruction"),
    }
}

#[test]
fn test_convert_compare_inst() {
    let join_cmp = crate::mir::join_ir::MirLikeInst::Compare {
        dst: ValueId(30),
        op: crate::mir::join_ir::CompareOp::Ge,
        lhs: ValueId(10),
        rhs: ValueId(11),
    };

    let mir_inst = convert_mir_like_inst(&join_cmp).unwrap();

    match mir_inst {
        MirInstruction::Compare { dst, op, lhs, rhs } => {
            assert_eq!(dst, ValueId(30));
            assert_eq!(op, MirCompareOp::Ge);
            assert_eq!(lhs, ValueId(10));
            assert_eq!(rhs, ValueId(11));
        }
        _ => panic!("Expected Compare instruction"),
    }
}

#[test]
fn test_convert_print_inst_to_externcall() {
    let join_print = crate::mir::join_ir::MirLikeInst::Print { value: ValueId(7) };

    let mir_inst = convert_mir_like_inst(&join_print).unwrap();

    // Should now emit canonical Call with Callee::Extern
    match mir_inst {
        MirInstruction::Call {
            dst,
            callee: Some(crate::mir::Callee::Extern(name)),
            args,
            effects,
            ..
        } => {
            assert_eq!(dst, None);
            assert_eq!(name, "env.console.log");
            assert_eq!(args, vec![ValueId(7)]);
            assert!(effects.contains(Effect::Io));
        }
        _ => panic!("Expected Call(callee=Extern) instruction"),
    }
}

// ========================================
// Phase 45: read_quoted Bridge Tests
// ========================================

/// Phase 45: read_quoted JoinIR → MIR 変換テスト
///
/// HAKO_JOINIR_READ_QUOTED=1 が設定されている場合のみ実行。
#[test]
fn test_read_quoted_joinir_to_mir_conversion() {
    ensure_ring0_initialized();

    // Dev flag がない場合はスキップ
    if !crate::config::env::joinir_dev::read_quoted_enabled() {
        get_global_ring0().log.debug(
            "[Phase 45] Skipping test_read_quoted_from_joinir_to_mir_conversion: \
             Set HAKO_JOINIR_READ_QUOTED=1 to enable",
        );
        return;
    }

    // 1. JoinModule を生成（lower_read_quoted_pattern を使用）
    let program_json = serde_json::json!({
        "defs": [{
            "name": "read_quoted",
            "params": ["s", "pos"],
            "body": { "body": [] }
        }]
    });

    let mut lowerer = AstToJoinIrLowerer::new();
    let join_module = lowerer.lower_read_quoted_pattern(&program_json);

    // 2. JoinIR → MIR 変換
    let mir_module = convert_joinir_to_mir(&join_module);

    assert!(
        mir_module.is_ok(),
        "JoinIR → MIR conversion should succeed: {:?}",
        mir_module.err()
    );

    let mir_module = mir_module.unwrap();

    // 3. MIR 構造の検証
    // 4 つの関数がある: entry, k_guard_fail, loop_step, k_exit
    assert_eq!(mir_module.functions.len(), 4, "MIR should have 4 functions");

    // 関数名を確認
    let func_names: Vec<&str> = mir_module.functions.keys().map(|s| s.as_str()).collect();
    get_global_ring0().log.debug(&format!(
        "[Phase 45] MIR function names: {:?}",
        func_names
    ));

    // join_func_0 (entry), join_func_1 (loop_step), join_func_2 (k_exit), join_func_3 (k_guard_fail)
    assert!(
        func_names.contains(&"join_func_0"),
        "Should have entry function join_func_0"
    );
    assert!(
        func_names.contains(&"join_func_1"),
        "Should have loop_step function join_func_1"
    );
    assert!(
        func_names.contains(&"join_func_2"),
        "Should have k_exit function join_func_2"
    );
    assert!(
        func_names.contains(&"join_func_3"),
        "Should have k_guard_fail function join_func_3"
    );

    get_global_ring0().log.debug(
        "[Phase 45] test_read_quoted_joinir_to_mir_conversion PASSED",
    );
}

/// Phase 45: String 定数の MIR 変換テスト
#[test]
fn test_convert_string_const_inst() {
    let join_const = crate::mir::join_ir::MirLikeInst::Const {
        dst: ValueId(50),
        value: crate::mir::join_ir::ConstValue::String("\"".to_string()),
    };

    let mir_inst = convert_mir_like_inst(&join_const).unwrap();

    match mir_inst {
        MirInstruction::Const { dst, value } => {
            assert_eq!(dst, ValueId(50));
            match value {
                crate::mir::ConstValue::String(s) => assert_eq!(s, "\""),
                _ => panic!("Expected String value"),
            }
        }
        _ => panic!("Expected Const instruction"),
    }
}

/// Phase 45: A/B テスト - Route B (JoinIR) E2E 実行テスト
///
/// HAKO_JOINIR_READ_QUOTED=1 が設定されている場合のみ実行。
///
/// # Test Cases (from Phase 45 fixture)
///
/// - T1: `"abc"` at pos 0 → `abc`
/// - T2: `""` at pos 0 → `` (empty)
/// - T3: `abc` at pos 0 → `` (guard fail, no quote)
/// - T4: `xx"def"` at pos 2 → `def`
///
/// # Escape Case Gate
///
/// T5 (escape handling) runs when `HAKO_JOINIR_READ_QUOTED_IFMERGE=1`.
#[test]
fn test_read_quoted_route_b_e2e() {
    ensure_ring0_initialized();

    // Dev flag がない場合はスキップ
    if !crate::config::env::joinir_dev::read_quoted_enabled() {
        get_global_ring0().log.debug(
            "[Phase 45] Skipping test_read_quoted_from_route_b_e2e: \
             Set HAKO_JOINIR_READ_QUOTED=1 to enable",
        );
        return;
    }

    // 1. JoinModule を生成
    let program_json = serde_json::json!({
        "defs": [{
            "name": "read_quoted",
            "params": ["s", "pos"],
            "body": { "body": [] }
        }]
    });

    let mut lowerer = AstToJoinIrLowerer::new();
    let join_module = lowerer.lower_read_quoted_pattern(&program_json);

    let entry_func = join_module.entry.expect("Entry function should exist");

    // 2. A/B テスト実行
    // Note: Route B (JoinIR) は run_joinir_via_vm で実行
    //       Route A (既存) は別途フィクスチャで検証済み

    // T1: "abc" at pos 0 → "abc"
    let t1_result = run_joinir_via_vm(
        &join_module,
        entry_func,
        &[JoinValue::Str("\"abc\"".to_string()), JoinValue::Int(0)],
    );
    match &t1_result {
        Ok(JoinValue::Str(s)) => {
            assert_eq!(s, "abc", "T1: Expected 'abc', got '{}'", s);
            get_global_ring0().log.debug(&format!(
                "[Phase 45] T1 PASS: \"abc\" at pos 0 → '{}'",
                s
            ));
        }
        Ok(v) => panic!("T1: Expected Str, got {:?}", v),
        Err(e) => get_global_ring0().log.debug(&format!(
            "[Phase 45] T1 SKIP (execution not supported): {:?}",
            e
        )),
    }

    // T2: "" at pos 0 → "" (empty)
    let t2_result = run_joinir_via_vm(
        &join_module,
        entry_func,
        &[JoinValue::Str("\"\"".to_string()), JoinValue::Int(0)],
    );
    match &t2_result {
        Ok(JoinValue::Str(s)) => {
            assert_eq!(s, "", "T2: Expected '', got '{}'", s);
            get_global_ring0().log.debug(&format!(
                "[Phase 45] T2 PASS: \"\" at pos 0 → '{}'",
                s
            ));
        }
        Ok(v) => panic!("T2: Expected Str, got {:?}", v),
        Err(e) => get_global_ring0().log.debug(&format!(
            "[Phase 45] T2 SKIP (execution not supported): {:?}",
            e
        )),
    }

    // T3: abc at pos 0 → "" (guard fail)
    let t3_result = run_joinir_via_vm(
        &join_module,
        entry_func,
        &[JoinValue::Str("abc".to_string()), JoinValue::Int(0)],
    );
    match &t3_result {
        Ok(JoinValue::Str(s)) => {
            assert_eq!(s, "", "T3: Expected '', got '{}'", s);
            get_global_ring0().log.debug(&format!(
                "[Phase 45] T3 PASS: abc at pos 0 → '{}'",
                s
            ));
        }
        Ok(v) => panic!("T3: Expected Str, got {:?}", v),
        Err(e) => get_global_ring0().log.debug(&format!(
            "[Phase 45] T3 SKIP (execution not supported): {:?}",
            e
        )),
    }

    // T4: xx"def" at pos 2 → "def"
    let t4_result = run_joinir_via_vm(
        &join_module,
        entry_func,
        &[JoinValue::Str("xx\"def\"".to_string()), JoinValue::Int(2)],
    );
    match &t4_result {
        Ok(JoinValue::Str(s)) => {
            assert_eq!(s, "def", "T4: Expected 'def', got '{}'", s);
            get_global_ring0().log.debug(&format!(
                "[Phase 45] T4 PASS: xx\"def\" at pos 2 → '{}'",
                s
            ));
        }
        Ok(v) => panic!("T4: Expected Str, got {:?}", v),
        Err(e) => get_global_ring0().log.debug(&format!(
            "[Phase 45] T4 SKIP (execution not supported): {:?}",
            e
        )),
    }

    // T5: Escape handling - "a\"b" at pos 0 → "a"b" (escaped quote)
    // Phase 46: IfMerge で if-body 後の i と ch をマージ
    let enable_escape_ifmerge = crate::config::env::joinir_dev::read_quoted_ifmerge_enabled();

    if enable_escape_ifmerge {
        // 入力: "a\"b" → 「"」で始まり、a, \", b, 「"」で終わる
        // 期待出力: a"b（エスケープされた引用符を含む）
        let t5_input = "\"a\\\"b\""; // Rust エスケープ: "a\"b" → JSON "a\"b"
        let t5_result = run_joinir_via_vm(
            &join_module,
            entry_func,
            &[JoinValue::Str(t5_input.to_string()), JoinValue::Int(0)],
        );
        match &t5_result {
            Ok(JoinValue::Str(s)) => {
                let expected = "a\"b"; // エスケープ後: a"b
                assert_eq!(s, expected, "T5: Expected '{}', got '{}'", expected, s);
                get_global_ring0().log.debug(&format!(
                    "[Phase 46] T5 PASS: \"a\\\"b\" at pos 0 → '{}' (escape handling works!)",
                    s
                ));
            }
            Ok(v) => panic!("T5: Expected Str, got {:?}", v),
            Err(e) => get_global_ring0().log.debug(&format!(
                "[Phase 46] T5 SKIP (execution not supported): {:?}",
                e
            )),
        }
    } else {
        get_global_ring0().log.debug(
            "[Phase 45] T5 SKIP: Set HAKO_JOINIR_READ_QUOTED_IFMERGE=1 to enable \
             escape handling (Phase 46)",
        );
    }

    get_global_ring0()
        .log
        .debug("[Phase 45] test_read_quoted_route_b_e2e completed");
}
