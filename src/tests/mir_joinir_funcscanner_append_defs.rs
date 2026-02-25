// mir_joinir_funcscanner_append_defs.rs
// Phase 27.14: FuncScannerBox._append_defs minimal loop JoinIR変換テスト
//
// 目的:
// - FuncScannerBox._append_defs の配列結合ループ（lines 293-300）の JoinIR 変換動作確認
// - LoopForm Case A (`loop(i < n)`) パターンの変換検証
// - Pinned/Carrier/Exit 設計の実装確認
//
// 実行条件:
// - デフォルトでは #[ignore] にしておいて手動実行用にする
// - 環境変数 NYASH_JOINIR_EXPERIMENT=1 で実験モード有効化
//
// Phase 27.14 設計:
// - Pinned: dst (ArrayBox), defs_box (ArrayBox), n (Integer)
// - Carrier: i (Integer)
// - Exit: none (void return, dst は破壊的変更)
//
// LoopForm Case A:
// - 動的条件: `loop(i < n)`
// - break: なし（常に i < n までループ）
// - continue: `i = i + 1` で統一

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::funcscanner_append_defs::lower_funcscanner_append_defs_to_joinir;
use crate::mir::join_ir::*;
use crate::mir::{MirCompiler, ValueId};
use crate::parser::NyashParser;
use crate::tests::helpers::joinir_env;
use std::collections::BTreeMap;

#[test]
#[ignore] // 手動実行用（Phase 27.14 実験段階）
fn mir_joinir_funcscanner_append_defs_auto_lowering() {
    // Phase 27.14: FuncScannerBox._append_defs の MIR → JoinIR 自動変換

    // 環境変数トグルチェック
    if !joinir_env::is_experiment_enabled() {
        eprintln!("[joinir/funcscanner_append_defs] NYASH_JOINIR_EXPERIMENT=1 not set, skipping auto-lowering test");
        return;
    }

    // Step 1: MIR までコンパイル
    // Phase 27.14: Minimal .hako file to avoid complex dependencies
    // Stage-3 parser を有効化（local キーワード対応）
    std::env::set_var("NYASH_FEATURES", "stage3");

    let test_file = "apps/tests/funcscanner_append_defs_minimal.hako";
    let src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));

    let ast: ASTNode =
        NyashParser::parse_from_string(&src).expect("funcscanner_append_defs: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc
        .compile(ast)
        .expect("funcscanner_append_defs: MIR compile failed");

    eprintln!(
        "[joinir/funcscanner_append_defs] MIR module compiled, {} functions",
        compiled.module.functions.len()
    );

    // Step 2: MIR → JoinIR 自動変換
    let join_module = lower_funcscanner_append_defs_to_joinir(&compiled.module)
        .expect("Phase 27.14: JoinIR construction should succeed");

    eprintln!("[joinir/funcscanner_append_defs] JoinIR module generated:");
    eprintln!("{:#?}", join_module);

    // Step 3: 妥当性検証（Phase 27.14）
    assert_eq!(
        join_module.functions.len(),
        2,
        "Expected 2 functions (append_defs_entry + loop_step)"
    );

    let entry_id = JoinFuncId::new(0);
    let loop_step_id = JoinFuncId::new(1);

    // append_defs_entry 関数の検証
    let entry_func = join_module
        .functions
        .get(&entry_id)
        .expect("append_defs_entry function not found");
    assert_eq!(entry_func.name, "append_defs_entry");
    assert_eq!(
        entry_func.params.len(),
        3,
        "append_defs_entry has 3 parameters (dst, defs_box, n)"
    );

    // loop_step 関数の検証
    let loop_step_func = join_module
        .functions
        .get(&loop_step_id)
        .expect("loop_step function not found");
    assert_eq!(loop_step_func.name, "loop_step");
    assert_eq!(
        loop_step_func.params.len(),
        4,
        "loop_step has 4 parameters (dst, defs_box, n, i)"
    );

    // ValueId range 検証 (9000-10999)
    assert_eq!(
        entry_func.params[0].0, 9000,
        "dst parameter should be ValueId(9000)"
    );
    assert_eq!(
        entry_func.params[1].0, 9001,
        "defs_box parameter should be ValueId(9001)"
    );
    assert_eq!(
        entry_func.params[2].0, 9002,
        "n parameter should be ValueId(9002)"
    );

    assert_eq!(
        loop_step_func.params[0].0, 10000,
        "dst_loop parameter should be ValueId(10000)"
    );
    assert_eq!(
        loop_step_func.params[1].0, 10001,
        "defs_box_loop parameter should be ValueId(10001)"
    );
    assert_eq!(
        loop_step_func.params[2].0, 10002,
        "n_loop parameter should be ValueId(10002)"
    );
    assert_eq!(
        loop_step_func.params[3].0, 10003,
        "i_loop parameter should be ValueId(10003)"
    );

    eprintln!("[joinir/funcscanner_append_defs] ✅ 自動変換成功（Phase 27.14）");
}

#[test]
fn mir_joinir_funcscanner_append_defs_type_sanity() {
    // Phase 27.14: 型定義の基本的なサニティチェック（常時実行）
    // funcscanner_append_defs 用の JoinFunction が作成できることを確認

    let entry_id = JoinFuncId::new(30);
    let entry_func = JoinFunction::new(
        entry_id,
        "funcscanner_append_defs_test".to_string(),
        vec![ValueId(9000), ValueId(9001), ValueId(9002)],
    );

    assert_eq!(entry_func.id, entry_id);
    assert_eq!(entry_func.name, "funcscanner_append_defs_test");
    assert_eq!(entry_func.params.len(), 3);
    assert_eq!(entry_func.body.len(), 0);
}

#[test]
fn mir_joinir_funcscanner_append_defs_empty_module_returns_none() {
    // Phase 27.14: 空の MIR モジュールでは None を返すことを確認
    // FuncScannerBox._append_defs/2 関数が存在しない場合のフォールバック動作

    // 最小限の MIR モジュールを作成
    use crate::mir::MirModule;
    let test_module = MirModule::new("test_module".to_string());

    // 対象関数が存在しないので None が返される（正常なフォールバック）
    let result = lower_funcscanner_append_defs_to_joinir(&test_module);

    eprintln!("[joinir/funcscanner_append_defs] empty_module test: result is None (expected)");
    assert!(
        result.is_none(),
        "Empty MirModule should return None (target function not found)"
    );
}

#[test]
#[ignore] // 手動実行: generic_case_a トグル検証（append_defs minimal）
fn mir_joinir_funcscanner_append_defs_generic_matches_handwritten() {
    if !joinir_env::is_experiment_enabled() {
        eprintln!("[joinir/funcscanner_append_defs] NYASH_JOINIR_EXPERIMENT=1 not set, skipping generic test");
        return;
    }

    std::env::set_var("NYASH_FEATURES", "stage3");

    let test_file = "apps/tests/funcscanner_append_defs_minimal.hako";
    let src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));

    let ast: ASTNode =
        NyashParser::parse_from_string(&src).expect("append_defs generic: parse failed");
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc
        .compile(ast)
        .expect("append_defs generic: MIR compile failed");

    fn params_by_name(jm: &JoinModule) -> BTreeMap<String, usize> {
        jm.functions
            .values()
            .map(|f| (f.name.clone(), f.params.len()))
            .collect()
    }

    // Baseline (generic OFF)
    std::env::set_var("NYASH_JOINIR_LOWER_GENERIC", "0");
    let baseline = lower_funcscanner_append_defs_to_joinir(&compiled.module)
        .expect("baseline append_defs lowering failed");

    // Generic ON
    std::env::set_var("NYASH_JOINIR_LOWER_GENERIC", "1");
    let generic = lower_funcscanner_append_defs_to_joinir(&compiled.module)
        .expect("generic append_defs lowering failed");

    let baseline_params = params_by_name(&baseline);
    let generic_params = params_by_name(&generic);

    assert_eq!(baseline_params.len(), 2, "baseline should have 2 functions");
    assert_eq!(generic_params.len(), 2, "generic should have 2 functions");
    assert_eq!(
        baseline_params.len(),
        generic_params.len(),
        "function count mismatch"
    );

    let expected_funcs = ["append_defs_entry", "loop_step"];
    for name in expected_funcs {
        let b_params = baseline_params
            .get(name)
            .copied()
            .unwrap_or_else(|| panic!("baseline missing function {}", name));
        let g_params = generic_params
            .get(name)
            .copied()
            .unwrap_or_else(|| panic!("generic missing function {}", name));
        assert_eq!(
            b_params, g_params,
            "param count differs for function {}",
            name
        );
    }

    eprintln!("[joinir/funcscanner_append_defs] ✅ generic_case_a JoinIR matches baseline");
}
