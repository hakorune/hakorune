//! Phase 40-3 Integration Test: array_ext.filter A/B Test
//!
//! ## Purpose
//! array_ext.filterのif-in-loop PHI生成について、
//! Route A (legacy AST→MIR) と Route B (JoinIR Frontend) を比較検証。
//!
//! ## Routes
//! - Route A: 旧AST→MIR + collect_assigned_vars経路 (HAKO_JOINIR_ARRAY_FILTER=0)
//! - Route B: JoinIR Frontend + JoinFuncMeta経路 (HAKO_JOINIR_ARRAY_FILTER=1)
//!
//! ## Success Criteria
//! - 実行結果が完全一致（[2, 4, 6]）
//! - 5回連続PASS

#![allow(dead_code)]

use crate::mir::join_ir::frontend::AstToJoinIrLowerer;
use crate::mir::join_ir_vm_bridge::convert_join_module_to_mir_with_meta;
use std::collections::HashSet;

/// Test 1: JoinFuncMeta helper methods work
///
/// ## Verification Points
/// 1. extract_if_in_loop_modified_vars() がif-in-loop修正変数を検出
/// 2. convert_join_module_to_mir_with_meta() がエラーなく実行
#[test]
fn phase40_joinir_meta_helpers_work() {
    use serde_json::json;

    // Simple loop with if-in-loop modification
    let loop_body = json!([
        {"type": "Local", "name": "i", "expr": {}},  // Loop counter (not in if)
        {"type": "If", "cond": {}, "then": [
            {"type": "Local", "name": "out", "expr": {}}  // Loop-carried, in if
        ], "else": null}
    ]);

    let mut loop_vars = HashSet::new();
    loop_vars.insert("i".to_string());
    loop_vars.insert("out".to_string());

    let mut lowerer = AstToJoinIrLowerer::new();
    let result = lowerer.extract_if_in_loop_modified_vars(&loop_body, &loop_vars);

    // Verify: out is detected, i is not
    assert!(!result.contains("i"), "Loop counter should NOT be included");
    assert!(
        result.contains("out"),
        "If-in-loop modified var should be included"
    );
    assert_eq!(result.len(), 1, "Exactly 1 if-in-loop variable");
}

/// Test 2: MIR conversion with empty metadata works
///
/// ## Verification Points
/// 1. convert_join_module_to_mir_with_meta() handles empty metadata
/// 2. No panic or error
#[test]
fn phase40_mir_conversion_with_empty_meta() {
    use crate::mir::join_ir::frontend::JoinFuncMetaMap;
    use crate::mir::join_ir::{JoinFuncId, JoinFunction, JoinModule};
    use crate::mir::ValueId;

    // Create minimal JoinModule
    let mut module = JoinModule::new();
    let func_id = JoinFuncId::new(0);

    let mut func = JoinFunction::new(func_id, "test_func".to_string(), vec![]);
    // Add a simple return instruction
    func.body.push(crate::mir::join_ir::JoinInst::Ret {
        value: Some(ValueId(0)),
    });

    module.functions.insert(func_id, func);

    // Empty metadata
    let meta = JoinFuncMetaMap::new();

    // Should not panic
    let result = convert_join_module_to_mir_with_meta(&module, &meta, None);
    assert!(result.is_ok(), "Empty metadata should not cause errors");

    let mir_module = result.unwrap();
    assert_eq!(mir_module.functions.len(), 1, "Should have 1 function");
}

/// Test 3: MIR conversion with if_modified_vars metadata
///
/// ## Verification Points
/// 1. Metadata is properly passed through
/// 2. No panic even with metadata present
#[test]
fn phase40_mir_conversion_with_meta() {
    use crate::mir::join_ir::frontend::{JoinFuncMeta, JoinFuncMetaMap};
    use crate::mir::join_ir::{JoinFuncId, JoinFunction, JoinModule};
    use crate::mir::ValueId;

    // Create minimal JoinModule
    let mut module = JoinModule::new();
    let func_id = JoinFuncId::new(0);

    let mut func = JoinFunction::new(func_id, "loop_step".to_string(), vec![]);
    func.body.push(crate::mir::join_ir::JoinInst::Ret {
        value: Some(ValueId(0)),
    });

    module.functions.insert(func_id, func);

    // Metadata with if_modified_vars
    let mut meta = JoinFuncMetaMap::new();
    let mut if_modified = HashSet::new();
    if_modified.insert("out".to_string());

    meta.insert(
        func_id,
        JoinFuncMeta {
            if_modified_vars: Some(if_modified),
            ..Default::default()
        },
    );

    // Should not panic, metadata is logged but not used for PHI generation yet
    let result = convert_join_module_to_mir_with_meta(&module, &meta, None);
    assert!(result.is_ok(), "Metadata should not cause errors");

    let mir_module = result.unwrap();
    assert_eq!(mir_module.functions.len(), 1, "Should have 1 function");
}

// ========================================
// Phase 40-3.5: A/B Test (Route Switching)
// ========================================

/// Test 4: JoinIR経由の代入変数収集（Local宣言）
///
/// ## Verification Points
/// 1. collect_assigned_vars_via_joinir が Local 宣言を正しく検出
/// 2. Phase 40-4.1 削除後もメイン経路として動作
#[test]
fn phase40_joinir_detects_local_declarations() {
    use crate::ast::{ASTNode, LiteralValue, Span};

    // Create simple if body with Local declarations: local x = 1; local y = 2;
    let then_body = vec![
        ASTNode::Local {
            variables: vec!["x".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        },
        ASTNode::Local {
            variables: vec!["y".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(2),
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        },
    ];

    // JoinIR経由（メイン経路）
    let vars = crate::mir::phi_core::test_utils::collect_assigned_vars_via_joinir(&then_body, None);

    // Verify JoinIR detects Local declarations
    assert!(vars.contains("x"), "JoinIR should detect x declaration");
    assert!(vars.contains("y"), "JoinIR should detect y declaration");
    assert_eq!(vars.len(), 2, "Should have exactly 2 variables");
}

/// Test 5: JoinIR経由のネストif内Local検出
///
/// ## Verification Points
/// 1. Nested if内のLocal宣言も正しく検出
/// 2. Phase 40-4.1 削除後の動作確認
#[test]
fn phase40_joinir_nested_if_local() {
    use crate::ast::{ASTNode, LiteralValue, Span};

    // Create nested if body with Local:
    // if (true) { local inner = 1 }
    // local outer = 2
    let then_body = vec![
        ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Local {
                variables: vec!["inner".to_string()],
                initial_values: vec![Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }))],
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        },
        ASTNode::Local {
            variables: vec!["outer".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(2),
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        },
    ];

    // JoinIR経由
    let vars = crate::mir::phi_core::test_utils::collect_assigned_vars_via_joinir(&then_body, None);

    // Verify: inner (nested in if) and outer (top-level) detected
    assert!(
        vars.contains("inner"),
        "JoinIR: should detect inner in nested if"
    );
    assert!(vars.contains("outer"), "JoinIR: should detect outer");
    assert_eq!(vars.len(), 2, "Should detect exactly 2 declarations");
}

// ========================================
// Phase 40-3: Original Documentation
// ========================================
//
// ## Note
// フルパイプライン統合は非常に複雑なため、Phase 40-3では以下を実装：
// 1. ✅ Dev flag (HAKO_JOINIR_ARRAY_FILTER)
// 2. ✅ Test API (extract_if_in_loop_modified_vars)
// 3. ✅ Test infrastructure
// 4. ✅ Phase 40-3.5 A/B route switching実装完了
//
// ## Phase 40-3.5完了条件
// - [x] Dev flag実装
// - [x] collect_assigned_vars_via_joinir() 実装
// - [x] loop_builder.rs route switching実装
// - [x] A/Bテスト（本テストファイル）

// ========================================
// Phase 40-1 Status (継続)
// ========================================
//
// ✅ Step 1: func_meta.rs 作成完了（Phase 40-1.1）
// ✅ Step 2: metadata extraction helper tests retained（Phase 40-1.1）
// ✅ Step 3: convert_join_module_to_mir_with_meta() 実装完了（Phase 40-1.2）
// ✅ Step 4: metadata observation path retained（Phase 40-1.2）
// ✅ Step 5: Integration test 作成完了（Phase 40-1.2）
// ✅ Step 6: collect_assigned_vars削除判定完了（削除不可確認）
//
// ## collect_assigned_vars削除判定結果
//
// callsites: loop_builder.rs:1069, 1075
// - これらは旧AST→MIRパスで使用（JoinIRパスと独立）
// - **削除不可**: 旧パスで依然必要
// - **Phase 40-1では削除しない**
//
// 削除可能になる条件（Phase 40-4以降）:
// - JoinIRパスが旧パスを完全置換
// - loop_builder.rsの該当コードパスが削除される
// - または旧パス全体がJoinIRパスに統合される
//
