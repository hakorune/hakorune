// mir_joinir_stageb_funcscanner.rs
// Phase 28: StageBFuncScannerBox.scan_all_boxes JoinIR 変換テスト
//
// 目的:
// - StageBFuncScannerBox.scan_all_boxes/1 の Case A ループを JoinIR に落とせることを確認
// - Pinned/Carrier/Exit の構造を固定化（Pinned: src/n, Carrier: defs/i, Exit: defs）
//
// 実行条件:
// - #[ignore] で手動実行。環境変数 NYASH_JOINIR_EXPERIMENT=1 が必要。

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::stageb_funcscanner::lower_stageb_funcscanner_to_joinir;
use crate::mir::join_ir::*;
use crate::mir::{MirCompiler, ValueId};
use crate::parser::NyashParser;
use crate::tests::helpers::joinir_env;

#[test]
#[ignore] // 手動実行用（実験モードのみ）
fn mir_joinir_stageb_funcscanner_auto_lowering() {
    if !joinir_env::is_experiment_enabled() {
        eprintln!("[joinir/stageb_funcscanner] NYASH_JOINIR_EXPERIMENT=1 not set, skipping auto-lowering test");
        return;
    }

    // Stage-3 パーサを有効化
    std::env::set_var("NYASH_FEATURES", "stage3");

    let test_file = "apps/tests/stageb_funcscanner_scan_boxes_minimal.hako";
    let src = std::fs::read_to_string(test_file)
        .unwrap_or_else(|_| panic!("Failed to read {}", test_file));

    let ast: ASTNode =
        NyashParser::parse_from_string(&src).expect("stageb_funcscanner: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc
        .compile(ast)
        .expect("stageb_funcscanner: MIR compile failed");

    eprintln!(
        "[joinir/stageb_funcscanner] MIR module compiled, {} functions",
        compiled.module.functions.len()
    );

    let join_module = lower_stageb_funcscanner_to_joinir(&compiled.module)
        .expect("StageBFuncScannerBox.scan_all_boxes should lower to JoinIR");

    eprintln!("[joinir/stageb_funcscanner] JoinIR module generated:");
    eprintln!("{:#?}", join_module);

    assert_eq!(join_module.functions.len(), 2, "Expected entry + loop_step");

    let entry_id = JoinFuncId::new(0);
    let loop_id = JoinFuncId::new(1);

    let entry_func = join_module
        .functions
        .get(&entry_id)
        .expect("scan_all_boxes function not found");
    assert_eq!(entry_func.name, "scan_all_boxes");
    assert_eq!(
        entry_func.params.len(),
        1,
        "scan_all_boxes should take (src)"
    );

    let loop_func = join_module
        .functions
        .get(&loop_id)
        .expect("loop_step function not found");
    assert_eq!(loop_func.name, "loop_step");
    assert_eq!(
        loop_func.params.len(),
        4,
        "loop_step should take (src, n, defs, i)"
    );

    eprintln!("[joinir/stageb_funcscanner] ✅ 自動変換成功（Phase 28）");
}

#[test]
fn mir_joinir_stageb_funcscanner_type_sanity() {
    let entry_id = JoinFuncId::new(24);
    let f = JoinFunction::new(
        entry_id,
        "stageb_funcscanner_test".to_string(),
        vec![ValueId(1)],
    );
    assert_eq!(f.id, entry_id);
    assert_eq!(f.name, "stageb_funcscanner_test");
    assert_eq!(f.params.len(), 1);
}

#[test]
fn mir_joinir_stageb_funcscanner_empty_module_returns_none() {
    use crate::mir::MirModule;
    let test_module = MirModule::new("empty".to_string());
    let result = lower_stageb_funcscanner_to_joinir(&test_module);
    eprintln!(
        "[joinir/stageb_funcscanner] empty_module test => {:?}",
        result
    );
    assert!(result.is_none());
}
