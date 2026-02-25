// JoinIR 実験ランナーの A/B 比較テスト（skip_ws / trim_min）
//
// 目的:
// - JoinIR を実際に実行し、既存 VM の結果と一致することを確認する
// - Phase 27.2 のブリッジ実装を env トグル付きで検証する

use crate::ast::ASTNode;
use crate::backend::VM;
use crate::mir::join_ir::{lower_funcscanner_trim_to_joinir, lower_skip_ws_to_joinir, JoinFuncId};
use crate::mir::join_ir_runner::{run_joinir_function, JoinValue};
use crate::mir::MirCompiler;
use crate::parser::NyashParser;
use crate::tests::helpers::joinir_env;

fn require_experiment_toggle() -> bool {
    if !joinir_env::is_experiment_enabled() {
        eprintln!(
            "[joinir/runner] NYASH_JOINIR_EXPERIMENT=1 not set, skipping experimental runner test"
        );
        return false;
    }
    true
}

#[test]
#[ignore] // PHI/LoopForm バグあり - Phase 30 の PHI canary として据え置き
fn joinir_runner_minimal_skip_ws_executes() {
    if !require_experiment_toggle() {
        return;
    }

    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");
    // 無限ループ検出のため、実験テストではステップ上限を小さめに設定しておく。
    // 0 は「上限なし」なので、ここでは明示的な上限を使う。
    std::env::set_var("NYASH_VM_MAX_STEPS", "100000");

    let src = std::fs::read_to_string("apps/tests/minimal_ssa_skip_ws.hako")
        .expect("failed to read minimal_ssa_skip_ws.hako");
    let runner = r#"
static box Runner {
  main(args) {
    return Main.skip("   abc")
  }
}
"#;
    let full_src = format!("{src}\n{runner}");

    let ast: ASTNode = NyashParser::parse_from_string(&full_src).expect("skip_ws: parse failed");
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("skip_ws: MIR compile failed");

    std::env::set_var("NYASH_ENTRY", "Runner.main");
    let mut vm = VM::new();
    let vm_out = vm
        .execute_module(&compiled.module)
        .expect("skip_ws: VM execution failed");
    let vm_result = vm_out.to_string_box().value;
    std::env::remove_var("NYASH_ENTRY");

    let join_module =
        lower_skip_ws_to_joinir(&compiled.module).expect("lower_skip_ws_to_joinir failed");
    // S-5.2-improved: Reuse VM instance for JoinIR Runner
    let join_result = run_joinir_function(
        &mut vm,
        &join_module,
        JoinFuncId::new(0),
        &[JoinValue::Str("   abc".to_string())],
    )
    .expect("JoinIR runner failed for skip_ws");

    assert_eq!(vm_result, "3", "VM expected to skip 3 leading spaces");
    match join_result {
        JoinValue::Int(v) => assert_eq!(v, 3, "JoinIR runner skip_ws result mismatch"),
        other => panic!("JoinIR runner returned non-int value: {:?}", other),
    }

    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
    std::env::remove_var("NYASH_VM_MAX_STEPS");
}

#[test]
#[ignore]
fn joinir_runner_funcscanner_trim_executes() {
    if !require_experiment_toggle() {
        return;
    }

    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");
    // 上と同様、無限ループ検出用にステップ上限を明示しておく。
    std::env::set_var("NYASH_VM_MAX_STEPS", "100000");

    let func_scanner_src = include_str!("../../lang/src/compiler/entry/func_scanner.hako");
    let test_src = std::fs::read_to_string("lang/src/compiler/tests/funcscanner_trim_min.hako")
        .expect("failed to read funcscanner_trim_min.hako");
    let runner = r#"
static box Runner {
  main(args) {
    return FuncScannerBox.trim("   abc  ")
  }
}
"#;
    let full_src = format!("{func_scanner_src}\n{test_src}\n{runner}");

    let ast: ASTNode = NyashParser::parse_from_string(&full_src).expect("trim_min: parse failed");
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("trim_min: MIR compile failed");

    std::env::set_var("NYASH_ENTRY", "Runner.main");
    let mut vm = VM::new();
    let vm_out = vm
        .execute_module(&compiled.module)
        .expect("trim_min: VM execution failed");
    let vm_result = vm_out.to_string_box().value;
    std::env::remove_var("NYASH_ENTRY");

    let join_module = lower_funcscanner_trim_to_joinir(&compiled.module)
        .expect("lower_funcscanner_trim_to_joinir failed");
    // S-5.2-improved: Reuse VM instance for JoinIR Runner
    let join_result = run_joinir_function(
        &mut vm,
        &join_module,
        JoinFuncId::new(0),
        &[JoinValue::Str("   abc  ".to_string())],
    )
    .expect("JoinIR runner failed for trim");

    assert_eq!(vm_result, "abc", "VM trim_min should return stripped text");
    match join_result {
        JoinValue::Str(s) => assert_eq!(s, "abc", "JoinIR runner trim result mismatch"),
        other => panic!("JoinIR runner returned non-string value: {:?}", other),
    }

    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_ENABLE_USING");
    std::env::remove_var("HAKO_ENABLE_USING");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
    std::env::remove_var("NYASH_VM_MAX_STEPS");
}
