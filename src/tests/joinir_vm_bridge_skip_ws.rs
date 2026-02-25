// Phase 27-shortterm S-4.4: JoinIR → Rust VM Bridge A/B Test
//
// 目的:
// - JoinIR を VM ブリッジ経由で実行し、直接 VM 実行の結果と一致することを確認する
// - Route A (AST→MIR→VM) と Route C (AST→MIR→JoinIR→MIR'→VM) の比較
//
// Test Pattern:
// - skip_ws("   abc") → 3 (leading spaces count)
//
// Implementation Status:
// - S-4.3: Basic bridge structure ✅
// - S-4.4-A: Call/Jump instructions for skip_ws pattern ✅
// - S-4.4-B: A/B test (this file) ✅

use crate::ast::ASTNode;
use crate::backend::VM;
use crate::mir::join_ir::{lower_skip_ws_to_joinir, JoinFuncId};
use crate::mir::join_ir_ops::JoinValue;
use crate::mir::join_ir_vm_bridge::run_joinir_via_vm;
use crate::mir::MirCompiler;
use crate::parser::NyashParser;

fn require_experiment_toggle() -> bool {
    if !crate::config::env::joinir_dev::vm_bridge_enabled() {
        eprintln!("[joinir/vm_bridge] NYASH_JOINIR_VM_BRIDGE=1 not set, skipping VM bridge test");
        return false;
    }
    true
}

#[test]
#[ignore]
fn joinir_vm_bridge_skip_ws_matches_direct_vm() {
    if !require_experiment_toggle() {
        return;
    }

    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");
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

    // Route A: AST → MIR → VM (direct)
    eprintln!("[joinir_vm_bridge_test] Route A: Direct VM execution");
    std::env::set_var("NYASH_ENTRY", "Runner.main");
    let mut vm = VM::new();
    let vm_out = vm
        .execute_module(&compiled.module)
        .expect("skip_ws: VM execution failed");
    let vm_result = vm_out.to_string_box().value;
    std::env::remove_var("NYASH_ENTRY");

    eprintln!("[joinir_vm_bridge_test] Route A result: {}", vm_result);

    // Route C: AST → MIR → JoinIR → MIR' → VM (via bridge)
    eprintln!("[joinir_vm_bridge_test] Route C: JoinIR → VM bridge execution");
    let join_module =
        lower_skip_ws_to_joinir(&compiled.module).expect("lower_skip_ws_to_joinir failed");

    let bridge_result = run_joinir_via_vm(
        &join_module,
        JoinFuncId::new(0),
        &[JoinValue::Str("   abc".to_string())],
    )
    .expect("JoinIR VM bridge failed for skip_ws");

    eprintln!(
        "[joinir_vm_bridge_test] Route C result: {:?}",
        bridge_result
    );

    // Assertions: Both routes should produce the same result
    assert_eq!(
        vm_result, "3",
        "Route A (VM) expected to skip 3 leading spaces"
    );
    match bridge_result {
        JoinValue::Int(v) => {
            assert_eq!(v, 3, "Route C (JoinIR→VM bridge) skip_ws result mismatch");
            eprintln!("[joinir_vm_bridge_test] ✅ A/B test passed: both routes returned 3");
        }
        other => panic!("JoinIR VM bridge returned non-int value: {:?}", other),
    }

    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
    std::env::remove_var("NYASH_VM_MAX_STEPS");
}
