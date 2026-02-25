// mir_joinir_stageb_body.rs
// Phase 28: StageBBodyExtractorBox.build_body_src JoinIR 変換テスト
//
// 目的:
// - StageBBodyExtractorBox.build_body_src/2 の Case A ループを JoinIR に落とせることを確認
// - Pinned/Carrier/Exit の構造を固定化（Pinned: src/args/n, Carrier: acc/i, Exit: acc）
//
// 実行条件:
// - #[ignore] で手動実行。環境変数 NYASH_JOINIR_EXPERIMENT=1 が必要。

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::stageb_body::lower_stageb_body_to_joinir;
use crate::mir::join_ir::*;
use crate::mir::{MirCompiler, ValueId};
use crate::parser::NyashParser;
use crate::tests::helpers::joinir_env;

#[test]
#[ignore] // 手動実行用（実験モードのみ）
fn mir_joinir_stageb_body_auto_lowering() {
    if !joinir_env::is_experiment_enabled() {
        eprintln!(
            "[joinir/stageb_body] NYASH_JOINIR_EXPERIMENT=1 not set, skipping auto-lowering test"
        );
        return;
    }

    // Stage-3 パーサを有効化（local/loop を安全に扱うため）
    std::env::set_var("NYASH_FEATURES", "stage3");

    let test_file = "apps/tests/stageb_body_extract_minimal.hako";
    let src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("stageb_body: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("stageb_body: MIR compile failed");

    eprintln!(
        "[joinir/stageb_body] MIR module compiled, {} functions",
        compiled.module.functions.len()
    );

    let join_module = lower_stageb_body_to_joinir(&compiled.module)
        .expect("StageBBodyExtractorBox.build_body_src should lower to JoinIR");

    eprintln!("[joinir/stageb_body] JoinIR module generated:");
    eprintln!("{:#?}", join_module);

    assert_eq!(join_module.functions.len(), 2, "Expected entry + loop_step");

    let entry_id = JoinFuncId::new(0);
    let loop_id = JoinFuncId::new(1);

    let entry_func = join_module
        .functions
        .get(&entry_id)
        .expect("build_body_src function not found");
    assert_eq!(entry_func.name, "build_body_src");
    assert_eq!(
        entry_func.params.len(),
        2,
        "build_body_src should take (src, args)"
    );

    let loop_func = join_module
        .functions
        .get(&loop_id)
        .expect("loop_step function not found");
    assert_eq!(loop_func.name, "loop_step");
    assert_eq!(
        loop_func.params.len(),
        5,
        "loop_step should take (src, args, n, acc, i)"
    );

    eprintln!("[joinir/stageb_body] ✅ 自動変換成功（Phase 28）");
}

#[test]
fn mir_joinir_stageb_body_type_sanity() {
    let entry_id = JoinFuncId::new(42);
    let f = JoinFunction::new(
        entry_id,
        "stageb_body_test".to_string(),
        vec![ValueId(1), ValueId(2)],
    );
    assert_eq!(f.id, entry_id);
    assert_eq!(f.name, "stageb_body_test");
    assert_eq!(f.params.len(), 2);
}

#[test]
fn mir_joinir_stageb_body_empty_module_returns_none() {
    use crate::mir::MirModule;
    let test_module = MirModule::new("empty".to_string());
    let result = lower_stageb_body_to_joinir(&test_module);
    eprintln!("[joinir/stageb_body] empty_module test => {:?}", result);
    assert!(result.is_none());
}
