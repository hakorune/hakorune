// mir_joinir_skip_ws.rs
// Phase 27.1: minimal_ssa_skip_ws JoinIR変換テスト
//
// 目的:
// - minimal_ssa_skip_ws.hako の MIR → JoinIR 自動変換の動作確認
// - ネストしたif + loop(1 == 1) + break パターンの変換検証
// - Phase 26-H の拡張版（複雑度: 中程度）
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
// - このテストは Header φ だけでなく、Exit φ（i の合流）も JoinIR で k_exit(i) として表現できていることを検証
// - skip_ws は2箇所の break パスがあり、どちらも i を返す → LoopExitShape::exit_args = [i]

use crate::ast::ASTNode;
use crate::mir::join_ir::*;
use crate::mir::{MirCompiler, ValueId};
use crate::parser::NyashParser;
use crate::tests::helpers::joinir_env;
use std::collections::BTreeMap;

#[test]
#[ignore] // 手動実行用（Phase 27.1 実験段階）
fn mir_joinir_skip_ws_auto_lowering() {
    // Phase 27.1: minimal_ssa_skip_ws の MIR → JoinIR 自動変換

    // 環境変数トグルチェック
    if !joinir_env::is_experiment_enabled() {
        eprintln!(
            "[joinir/skip_ws] NYASH_JOINIR_EXPERIMENT=1 not set, skipping auto-lowering test"
        );
        return;
    }

    // Step 1: MIR までコンパイル
    // Stage-3 parser を有効化（local キーワード対応）
    std::env::set_var("NYASH_FEATURES", "stage3");

    let test_file = "apps/tests/minimal_ssa_skip_ws.hako";
    let src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("skip_ws: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("skip_ws: MIR compile failed");

    eprintln!(
        "[joinir/skip_ws] MIR module compiled, {} functions",
        compiled.module.functions.len()
    );

    // Step 2: MIR → JoinIR 自動変換
    let join_module =
        lower_skip_ws_to_joinir(&compiled.module).expect("lower_skip_ws_to_joinir failed");

    eprintln!("[joinir/skip_ws] JoinIR module generated:");
    eprintln!("{:#?}", join_module);

    // Step 3: 妥当性検証
    assert_eq!(
        join_module.functions.len(),
        2,
        "Expected 2 functions (skip + loop_step)"
    );

    let skip_id = JoinFuncId::new(0);
    let loop_step_id = JoinFuncId::new(1);

    // skip 関数の検証
    let skip_func = join_module
        .functions
        .get(&skip_id)
        .expect("skip function not found");
    assert_eq!(skip_func.name, "skip");
    assert_eq!(skip_func.params.len(), 1, "skip has 1 parameter (s)");
    assert!(
        skip_func.body.len() >= 3,
        "skip should have at least 3 instructions (const 0, length call, loop_step call)"
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
        "loop_step has 3 parameters (s, i, n)"
    );
    assert!(loop_step_func.body.len() >= 8, "loop_step should have multiple instructions (comparisons, substring, recursive call, etc.)");

    eprintln!("[joinir/skip_ws] ✅ 自動変換成功（Phase 27.1）");
}

#[test]
fn mir_joinir_skip_ws_type_sanity() {
    // Phase 27.1: 型定義の基本的なサニティチェック（常時実行）
    // skip_ws 用の JoinFunction が作成できることを確認

    let skip_id = JoinFuncId::new(10);
    let skip_func = JoinFunction::new(
        skip_id,
        "skip_ws_test".to_string(),
        vec![ValueId(1), ValueId(2), ValueId(3)],
    );

    assert_eq!(skip_func.id, skip_id);
    assert_eq!(skip_func.name, "skip_ws_test");
    assert_eq!(skip_func.params.len(), 3);
    assert_eq!(skip_func.body.len(), 0);
}

#[test]
fn mir_joinir_skip_ws_generic_matches_handwritten() {
    if !joinir_env::is_experiment_enabled() {
        eprintln!("[joinir/skip_ws] NYASH_JOINIR_EXPERIMENT=1 not set, skipping generic test");
        return;
    }

    // Stage-3 parserを有効化
    std::env::set_var("NYASH_FEATURES", "stage3");

    let test_file = "apps/tests/minimal_ssa_skip_ws.hako";
    let src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("skip_ws generic: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc
        .compile(ast)
        .expect("skip_ws generic: MIR compile failed");

    fn params_by_name(jm: &JoinModule) -> BTreeMap<String, usize> {
        jm.functions
            .values()
            .map(|f| (f.name.clone(), f.params.len()))
            .collect()
    }

    fn has_jump_to_cont(jm: &JoinModule, func_name: &str, cont: JoinContId) -> bool {
        jm.functions
            .values()
            .find(|f| f.name == func_name)
            .map(|f| {
                f.body.iter().any(|inst| match inst {
                    JoinInst::Jump { cont: c, .. } => *c == cont,
                    _ => false,
                })
            })
            .unwrap_or(false)
    }

    // Baseline (generic OFF)
    std::env::set_var("NYASH_JOINIR_LOWER_GENERIC", "0");
    let baseline =
        lower_skip_ws_to_joinir(&compiled.module).expect("baseline skip_ws lowering failed");

    // Generic ON
    std::env::set_var("NYASH_JOINIR_LOWER_GENERIC", "1");
    let generic =
        lower_skip_ws_to_joinir(&compiled.module).expect("generic skip_ws lowering failed");

    // Compare shape (function count + params + loop_step/k_exit 相当の存在確認)
    let baseline_params = params_by_name(&baseline);
    let generic_params = params_by_name(&generic);

    assert_eq!(baseline_params.len(), 2, "baseline should have 2 functions");
    assert_eq!(generic_params.len(), 2, "generic should have 2 functions");
    assert_eq!(
        baseline_params.len(),
        generic_params.len(),
        "function count mismatch"
    );

    let expected_funcs = ["skip", "loop_step"];
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

    let exit_cont = JoinContId::new(0);
    assert!(
        has_jump_to_cont(&baseline, "loop_step", exit_cont),
        "baseline loop_step should jump to k_exit(cont0)"
    );
    assert!(
        has_jump_to_cont(&generic, "loop_step", exit_cont),
        "generic loop_step should jump to k_exit(cont0)"
    );

    eprintln!("[joinir/skip_ws] ✅ generic_case_a JoinIR matches baseline");
}
