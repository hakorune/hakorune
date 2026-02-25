// mir_joinir_funcscanner_trim.rs
// Phase 27.1: FuncScannerBox.trim JoinIR変換テスト
//
// 目的:
// - FuncScannerBox.trim/1 の MIR → JoinIR 自動変換の動作確認
// - trailing whitespace 除去ループの JoinIR 表現検証
// - Phase 27.0 skip_ws に続く実用ループ変換（より簡単なケース）
//
// 実行条件:
// - デフォルトでは #[ignore] にしておいて手動実行用にする
// - 環境変数 NYASH_JOINIR_EXPERIMENT=1 で実験モード有効化
//
// Phase 27.4-C 対応（現状メモのみ）:
// - このテストは JoinIR 変換のみを検証（VM 実行なし）
// - かつては Header φ bypass を有効化する JoinIR 実験フラグがあったが、
//   Phase 73 時点では削除済み（常時 OFF）となっている。
//
// Phase 27.5 対応:
// - このテストは Header φ だけでなく、Exit φ（e の合流＋substring(b, e) 呼び出し）も JoinIR で k_exit として表現できることを検証
// - trim のループには2箇所の break パスがあり、どちらも substring(b, e) の結果を返す
// - ExitShape Option A として設計: exit_args = [e] で、ループ内で substring(b, e) を計算済み

use crate::ast::ASTNode;
use crate::mir::join_ir::*;
use crate::mir::{MirCompiler, ValueId};
use crate::parser::NyashParser;
use crate::tests::helpers::joinir_env;
use std::collections::BTreeMap;

#[test]
#[ignore] // 手動実行用（Phase 27.1 実験段階）
fn mir_joinir_funcscanner_trim_auto_lowering() {
    // Phase 27.1: FuncScannerBox.trim の MIR → JoinIR 自動変換

    // 環境変数トグルチェック
    if !joinir_env::is_experiment_enabled() {
        eprintln!("[joinir/trim] NYASH_JOINIR_EXPERIMENT=1 not set, skipping auto-lowering test");
        return;
    }

    // Stage-3 parser を有効化（local キーワード対応）
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");

    // Step 1: MIR までコンパイル
    // FuncScanner 本体と最小テストを結合
    let test_file = "lang/src/compiler/tests/funcscanner_trim_min.hako";
    let func_scanner_src = include_str!("../../lang/src/compiler/entry/func_scanner.hako");
    let test_src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));
    let src = format!("{func_scanner_src}\n\n{test_src}");

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("trim: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("trim: MIR compile failed");

    eprintln!(
        "[joinir/trim] MIR module compiled, {} functions",
        compiled.module.functions.len()
    );

    // Step 2: MIR → JoinIR 自動変換
    let join_module = lower_funcscanner_trim_to_joinir(&compiled.module)
        .expect("lower_funcscanner_trim_to_joinir failed");

    eprintln!("[joinir/trim] JoinIR module generated:");
    eprintln!("{:#?}", join_module);

    // Step 3: 妥当性検証
    assert_eq!(
        join_module.functions.len(),
        3,
        "Expected 3 functions (trim_main + loop_step + skip_leading)"
    );

    let trim_main_id = JoinFuncId::new(0);
    let loop_step_id = JoinFuncId::new(1);

    // trim_main 関数の検証
    let trim_main_func = join_module
        .functions
        .get(&trim_main_id)
        .expect("trim_main function not found");
    assert_eq!(trim_main_func.name, "trim_main");
    assert_eq!(
        trim_main_func.params.len(),
        1,
        "trim_main has 1 parameter (s)"
    );
    assert!(
        trim_main_func.body.len() >= 5,
        "trim_main should have at least 5 instructions"
    );

    // loop_step 関数の検証
    let loop_step_func = join_module
        .functions
        .get(&loop_step_id)
        .expect("loop_step function not found");
    assert_eq!(loop_step_func.name, "loop_step");
    assert_eq!(
        loop_step_func.params.len(),
        3,
        "loop_step has 3 parameters (str, b, e)"
    );
    assert!(
        loop_step_func.body.len() >= 10,
        "loop_step should have multiple instructions"
    );

    eprintln!("[joinir/trim] ✅ 自動変換成功（Phase 27.1）");
}

#[test]
fn mir_joinir_funcscanner_trim_type_sanity() {
    // Phase 27.1: 型定義の基本的なサニティチェック（常時実行）
    // trim 用の JoinFunction が作成できることを確認

    let trim_main_id = JoinFuncId::new(10);
    let trim_main_func =
        JoinFunction::new(trim_main_id, "trim_main_test".to_string(), vec![ValueId(1)]);

    assert_eq!(trim_main_func.id, trim_main_id);
    assert_eq!(trim_main_func.name, "trim_main_test");
    assert_eq!(trim_main_func.params.len(), 1);
    assert_eq!(trim_main_func.body.len(), 0);
}

#[test]
#[ignore] // 手動実行: generic_case_a トグル検証（trim minimal）
fn mir_joinir_funcscanner_trim_generic_matches_handwritten() {
    if !joinir_env::is_experiment_enabled() {
        eprintln!("[joinir/trim] NYASH_JOINIR_EXPERIMENT=1 not set, skipping generic test");
        return;
    }

    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");

    let func_scanner_src = include_str!("../../lang/src/compiler/entry/func_scanner.hako");
    let test_file = "lang/src/compiler/tests/funcscanner_trim_min.hako";
    let test_src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));
    let src = format!("{func_scanner_src}\n\n{test_src}");

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("trim generic: parse failed");
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("trim generic: MIR compile failed");

    fn params_by_name(jm: &JoinModule) -> BTreeMap<String, usize> {
        jm.functions
            .values()
            .map(|f| (f.name.clone(), f.params.len()))
            .collect()
    }

    // Baseline (generic OFF)
    std::env::set_var("NYASH_JOINIR_LOWER_GENERIC", "0");
    let baseline =
        lower_funcscanner_trim_to_joinir(&compiled.module).expect("baseline trim lowering failed");

    // Generic ON
    std::env::set_var("NYASH_JOINIR_LOWER_GENERIC", "1");
    let generic =
        lower_funcscanner_trim_to_joinir(&compiled.module).expect("generic trim lowering failed");

    let baseline_params = params_by_name(&baseline);
    let generic_params = params_by_name(&generic);

    assert_eq!(baseline_params.len(), 3, "baseline should have 3 functions");
    assert_eq!(generic_params.len(), 3, "generic should have 3 functions");
    assert_eq!(
        baseline_params.len(),
        generic_params.len(),
        "function count mismatch"
    );

    let expected_funcs = ["trim_main", "loop_step", "skip_leading"];
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

    eprintln!("[joinir/trim] ✅ generic_case_a JoinIR matches baseline");
}
