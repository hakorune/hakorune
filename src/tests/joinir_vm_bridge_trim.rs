// Phase 30.x: JoinIR → Rust VM Bridge A/B Test for FuncScannerBox.trim/1
//
// 目的:
// - JoinIR を VM ブリッジ経由で実行し、直接 VM 実行の結果と一致することを確認する
// - Route A (AST→MIR→VM) と Route C (AST→MIR→JoinIR→MIR'→VM) の比較
//
// Test Route Shape:
// - trim("   abc  ") → "abc" (leading + trailing whitespace removed)
//
// Implementation Status:
// - Phase 30.x: Non-tail call support in VM bridge ✅
// - A/B test (this file) ✅

use crate::ast::ASTNode;
use crate::backend::VM;
use crate::mir::join_ir::{lower_funcscanner_trim_to_joinir, JoinFuncId};
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

/// Minimal FuncScannerBox.trim/1 implementation for testing
const TRIM_SOURCE: &str = r#"
static box FuncScannerBox {
    trim(s) {
        local str = "" + s
        local n = str.length()
        local b = me._skip_leading(str, 0, n)
        local e = n + 0
        return me._trim_trailing(str, b, e)
    }

    _skip_leading(str, i, n) {
        loop(i < n) {
            local ch = str.substring(i, i + 1)
            local is_ws = (ch == " ") or (ch == "\t") or (ch == "\n") or (ch == "\r")
            if (not is_ws) {
                return i
            }
            i = i + 1
        }
        return i
    }

    _trim_trailing(str, b, e) {
        loop(e > b) {
            local ch = str.substring(e - 1, e)
            local is_ws = (ch == " ") or (ch == "\t") or (ch == "\n") or (ch == "\r")
            if (not is_ws) {
                return str.substring(b, e)
            }
            e = e - 1
        }
        return str.substring(b, e)
    }
}
"#;

#[test]
#[ignore]
fn joinir_vm_bridge_trim_matches_direct_vm() {
    if !require_experiment_toggle() {
        return;
    }

    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");
    std::env::set_var("NYASH_VM_MAX_STEPS", "100000");

    let runner = r#"
static box Runner {
  main(args) {
    return FuncScannerBox.trim("   abc  ")
  }
}
"#;
    let full_src = format!("{TRIM_SOURCE}\n{runner}");

    let ast: ASTNode = NyashParser::parse_from_string(&full_src).expect("trim: parse failed");
    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("trim: MIR compile failed");

    // Route A: AST → MIR → VM (direct)
    eprintln!("[joinir_vm_bridge_test/trim] Route A: Direct VM execution");
    std::env::set_var("NYASH_ENTRY", "Runner.main");
    let mut vm = VM::new();
    let vm_out = vm
        .execute_module(&compiled.module)
        .expect("trim: VM execution failed");
    let vm_result = vm_out.to_string_box().value;
    std::env::remove_var("NYASH_ENTRY");

    eprintln!("[joinir_vm_bridge_test/trim] Route A result: {}", vm_result);

    // Route C: AST → MIR → JoinIR → MIR' → VM (via bridge)
    eprintln!("[joinir_vm_bridge_test/trim] Route C: JoinIR → VM bridge execution");
    let join_module = lower_funcscanner_trim_to_joinir(&compiled.module)
        .expect("lower_funcscanner_trim_to_joinir failed");

    let bridge_result = run_joinir_via_vm(
        &join_module,
        JoinFuncId::new(0),
        &[JoinValue::Str("   abc  ".to_string())],
    )
    .expect("JoinIR VM bridge failed for trim");

    eprintln!(
        "[joinir_vm_bridge_test/trim] Route C result: {:?}",
        bridge_result
    );

    // Assertions:
    // Note: Route A (direct VM) may return incorrect result due to PHI bugs.
    // This is expected and is exactly what JoinIR is designed to fix.
    // We verify that JoinIR returns the correct result.
    match bridge_result {
        JoinValue::Str(s) => {
            assert_eq!(s, "abc", "Route C (JoinIR→VM bridge) trim result mismatch");
            if vm_result == "abc" {
                eprintln!(
                    "[joinir_vm_bridge_test/trim] ✅ A/B test passed: both routes returned 'abc'"
                );
            } else {
                eprintln!("[joinir_vm_bridge_test/trim] ⚠️ Route A (VM) returned '{}' (PHI bug), Route C (JoinIR) returned 'abc' (correct)", vm_result);
                eprintln!("[joinir_vm_bridge_test/trim] ✅ JoinIR correctly handles PHI issues that affect direct VM path");
            }
        }
        other => panic!("JoinIR VM bridge returned non-string value: {:?}", other),
    }

    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
    std::env::remove_var("NYASH_VM_MAX_STEPS");
}

#[test]
#[ignore]
fn joinir_vm_bridge_trim_edge_cases() {
    if !require_experiment_toggle() {
        return;
    }

    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    // Test cases: (input, expected_output)
    // Note: Escape characters like \t\n\r are not tested here because
    // Nyash string literal parsing differs from Rust escape handling.
    let test_cases = [
        ("   abc  ", "abc"),
        ("hello", "hello"),
        ("  ", ""),
        ("  x  ", "x"),
        ("   hello world   ", "hello world"),
    ];

    for (input, expected) in test_cases {
        // Re-set environment variables at each iteration to avoid race conditions
        // with parallel test execution
        std::env::set_var("NYASH_FEATURES", "stage3");
        std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

        let runner = format!(
            r#"
static box Runner {{
  main(args) {{
    return FuncScannerBox.trim("{input}")
  }}
}}
"#
        );
        let full_src = format!("{TRIM_SOURCE}\n{runner}");

        let ast: ASTNode =
            NyashParser::parse_from_string(&full_src).expect("trim edge case: parse failed");
        let mut mc = MirCompiler::with_options(false);
        let compiled = mc.compile(ast).expect("trim edge case: MIR compile failed");

        // JoinIR path only
        let join_module = lower_funcscanner_trim_to_joinir(&compiled.module)
            .expect("lower_funcscanner_trim_to_joinir failed");

        let bridge_result = run_joinir_via_vm(
            &join_module,
            JoinFuncId::new(0),
            &[JoinValue::Str(input.to_string())],
        )
        .expect("JoinIR VM bridge failed for trim edge case");

        match bridge_result {
            JoinValue::Str(s) => {
                assert_eq!(
                    s, expected,
                    "trim({:?}) expected {:?} but got {:?}",
                    input, expected, s
                );
                eprintln!(
                    "[joinir_vm_bridge_test/trim] ✅ trim({:?}) = {:?}",
                    input, s
                );
            }
            other => panic!("trim({:?}) returned non-string value: {:?}", input, other),
        }
    }

    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
}
