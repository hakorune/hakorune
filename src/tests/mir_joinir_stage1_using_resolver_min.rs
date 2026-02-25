// mir_joinir_stage1_using_resolver_min.rs
// Phase 27.12: Stage1UsingResolverBox.resolve_for_source minimal loop JoinIR変換テスト
//
// 目的:
// - Stage1UsingResolverBox.resolve_for_source の entries ループ（lines 44-91）の JoinIR 変換動作確認
// - LoopForm Case A (`loop(i < n)`) パターンの変換検証
// - Pinned/Carrier/Exit 設計の実装確認
//
// 実行条件:
// - デフォルトでは #[ignore] にしておいて手動実行用にする
// - 環境変数 NYASH_JOINIR_EXPERIMENT=1 で実験モード有効化
//
// Phase 27.12 設計:
// - Pinned: entries (ArrayBox), n (Integer), modules (MapBox), seen (MapBox)
// - Carrier: i (Integer), prefix (String)
// - Exit: prefix (String - 最終的な連結文字列)
//
// LoopForm Case A:
// - 動的条件: `loop(i < n)`
// - break: なし（常に i < n までループ）
// - continue: `i = next_i` で統一

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::stage1_using_resolver::lower_stage1_usingresolver_to_joinir;
use crate::mir::join_ir::*;
use crate::mir::{MirCompiler, ValueId};
use crate::parser::NyashParser;
use crate::tests::helpers::joinir_env;
use std::collections::BTreeMap;

fn ensure_joinir_strict_env() {
    std::env::set_var("NYASH_JOINIR_CORE", "1");
    std::env::set_var("NYASH_JOINIR_STRICT", "1");
}

#[test]
#[ignore] // 手動実行用（Stage-1 minimal）
fn mir_joinir_stage1_using_resolver_auto_lowering() {
    // Phase 27.12: Stage1UsingResolverBox.resolve_for_source の MIR → JoinIR 自動変換

    // 環境変数トグルチェック
    if !joinir_env::is_experiment_enabled() {
        eprintln!("[joinir/stage1_using_resolver] NYASH_JOINIR_EXPERIMENT=1 not set, skipping auto-lowering test");
        return;
    }

    // Step 1: MIR までコンパイル
    // Phase 27.13: Minimal .hako file to avoid `using` statement parser issues
    // Stage-3 parser を有効化（local キーワード対応）
    std::env::set_var("NYASH_FEATURES", "stage3");

    let test_file = "apps/tests/stage1_usingresolver_minimal.hako";
    let src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));

    let ast: ASTNode =
        NyashParser::parse_from_string(&src).expect("stage1_using_resolver: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc
        .compile(ast)
        .expect("stage1_using_resolver: MIR compile failed");

    eprintln!(
        "[joinir/stage1_using_resolver] MIR module compiled, {} functions",
        compiled.module.functions.len()
    );

    // Step 2: MIR → JoinIR 自動変換
    let join_module = lower_stage1_usingresolver_to_joinir(&compiled.module)
        .expect("Phase 27.13: JoinIR construction should succeed");

    eprintln!("[joinir/stage1_using_resolver] JoinIR module generated:");
    eprintln!("{:#?}", join_module);

    // Step 3: 妥当性検証（Phase 27.13 以降で実装）
    assert_eq!(
        join_module.functions.len(),
        2,
        "Expected 2 functions (resolve_entries + loop_step)"
    );

    let resolve_id = JoinFuncId::new(0);
    let loop_step_id = JoinFuncId::new(1);

    // resolve_entries 関数の検証
    let resolve_func = join_module
        .functions
        .get(&resolve_id)
        .expect("resolve_entries function not found");
    assert_eq!(resolve_func.name, "resolve_entries");
    assert_eq!(
        resolve_func.params.len(),
        5,
        "resolve_entries has 5 parameters (entries, n, modules, seen, prefix_init)"
    );

    // loop_step 関数の検証
    let loop_step_func = join_module
        .functions
        .get(&loop_step_id)
        .expect("loop_step function not found");
    assert_eq!(loop_step_func.name, "loop_step");
    assert_eq!(
        loop_step_func.params.len(),
        6,
        "loop_step has 6 parameters (entries, n, modules, seen, prefix, i)"
    );

    eprintln!("[joinir/stage1_using_resolver] ✅ 自動変換成功（Phase 27.12/27.13）");
}

#[test]
fn mir_joinir_stage1_using_resolver_type_sanity() {
    // Phase 27.12: 型定義の基本的なサニティチェック（常時実行）
    // stage1_using_resolver 用の JoinFunction が作成できることを確認
    ensure_joinir_strict_env();

    let resolve_id = JoinFuncId::new(20);
    let resolve_func = JoinFunction::new(
        resolve_id,
        "stage1_using_resolver_test".to_string(),
        vec![ValueId(1), ValueId(2), ValueId(3), ValueId(4), ValueId(5)],
    );

    assert_eq!(resolve_func.id, resolve_id);
    assert_eq!(resolve_func.name, "stage1_using_resolver_test");
    assert_eq!(resolve_func.params.len(), 5);
    assert_eq!(resolve_func.body.len(), 0);
}

#[test]
fn mir_joinir_stage1_using_resolver_empty_module_returns_none() {
    // Phase 27.13: 空の MIR モジュールでは None を返すことを確認
    // Stage1UsingResolverBox.resolve_for_source/1 関数が存在しない場合のフォールバック動作
    ensure_joinir_strict_env();

    // 最小限の MIR モジュールを作成
    use crate::mir::MirModule;
    let test_module = MirModule::new("test_module".to_string());

    // 対象関数が存在しないので None が返される（正常なフォールバック）
    let result = lower_stage1_usingresolver_to_joinir(&test_module);

    eprintln!("[joinir/stage1_using_resolver] empty_module test: result is None (expected)");
    assert!(
        result.is_none(),
        "Empty MirModule should return None (target function not found)"
    );
}

#[test]
#[ignore] // 手動実行: generic_case_a トグル検証（stage1_using_resolver minimal）
fn mir_joinir_stage1_using_resolver_generic_matches_handwritten() {
    if !joinir_env::is_experiment_enabled() {
        eprintln!(
            "[joinir/stage1_using_resolver] NYASH_JOINIR_EXPERIMENT=1 not set, skipping generic test"
        );
        return;
    }

    std::env::set_var("NYASH_FEATURES", "stage3");

    let test_file = "apps/tests/stage1_usingresolver_minimal.hako";
    let src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));

    let ast: ASTNode =
        NyashParser::parse_from_string(&src).expect("stage1_using_resolver generic: parse failed");
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc
        .compile(ast)
        .expect("stage1_using_resolver generic: MIR compile failed");

    fn params_by_name(jm: &JoinModule) -> BTreeMap<String, usize> {
        jm.functions
            .values()
            .map(|f| (f.name.clone(), f.params.len()))
            .collect()
    }

    // Baseline (generic OFF)
    std::env::set_var("NYASH_JOINIR_LOWER_GENERIC", "0");
    let baseline = lower_stage1_usingresolver_to_joinir(&compiled.module)
        .expect("baseline stage1_using_resolver lowering failed");

    // Generic ON
    std::env::set_var("NYASH_JOINIR_LOWER_GENERIC", "1");
    let generic = lower_stage1_usingresolver_to_joinir(&compiled.module)
        .expect("generic stage1_using_resolver lowering failed");

    let baseline_params = params_by_name(&baseline);
    let generic_params = params_by_name(&generic);

    assert_eq!(baseline_params.len(), 2, "baseline should have 2 functions");
    assert_eq!(generic_params.len(), 2, "generic should have 2 functions");
    assert_eq!(
        baseline_params.len(),
        generic_params.len(),
        "function count mismatch"
    );

    let expected_funcs = ["resolve_entries", "loop_step"];
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

    eprintln!("[joinir/stage1_using_resolver] ✅ generic_case_a JoinIR matches baseline");
}
