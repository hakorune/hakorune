// mir_funcscanner_trim_min.rs
// Rust-level test for FuncScannerBox._trim/1 minimal SSA/PHI repro
//
// Goal:
// - Compile lang/src/compiler/entry/func_scanner.hako + minimal _trim test .hako
// - Run MirVerifier to see if Undefined / dominator errors already出るか確認。
// - Optionally execute via VM（将来の回 regressions 用）。

use crate::ast::ASTNode;
use crate::mir::{MirCompiler, MirVerifier};
use crate::parser::NyashParser;

#[test]
fn mir_funcscanner_trim_min_verify_and_vm() {
    // Minimal .hako that calls FuncScannerBox._trim/1 directly。
    let test_file = "lang/src/compiler/tests/funcscanner_trim_min.hako";

    // Stage‑3 + using 系のパーサ設定を揃える（他の FuncScanner 系テストと同じ）。
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    // 必要に応じて MIR / SSA デバッグを有効化（手元での調査用）。
    // std::env::set_var("NYASH_MIR_DEBUG_LOG", "1");
    // std::env::set_var("NYASH_VM_VERIFY_MIR", "1");
    // std::env::set_var("NYASH_IF_HOLE_TRACE", "1");

    // FuncScanner 本体と最小 _trim テストを 1 ソースにまとめる。
    let func_scanner_src = include_str!("../../lang/src/compiler/entry/func_scanner.hako");
    let test_src = std::fs::read_to_string(test_file).expect("Failed to read trim_min .hako");
    let src = format!("{func_scanner_src}\n\n{test_src}");

    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("trim_min: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("trim_min: MIR compile failed");

    eprintln!(
        "[trim/min] module functions = {}",
        compiled.module.functions.len()
    );

    // Optional: dump key functions when NYASH_MIR_TEST_DUMP=1
    if std::env::var("NYASH_MIR_TEST_DUMP").ok().as_deref() == Some("1") {
        use crate::mir::MirPrinter;
        let printer = MirPrinter::new();
        for name in [
            "FuncScannerBox._trim/1",
            "FuncScannerBox.trim/1",
            "FuncScannerBox.skip_whitespace/2",
            "FuncScannerBox.parse_params/1",
            "main",
        ] {
            if let Some(func) = compiled.module.functions.get(name) {
                let dump = printer.print_function(func);
                eprintln!("----- MIR DUMP: {} -----\n{}", name, dump);
            } else {
                eprintln!("[trim/min] WARN: function not found: {}", name);
            }
        }
    }

    // MIR verify: ここで FuncScannerBox._trim/1 / trim/1 の SSA/PHI 崩れを観測する。
    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        eprintln!("[trim/min] MIR verification errors:");
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        // いまは「バグ検出」が目的なので、失敗したらそのまま赤にしておく。
        panic!("trim_min: MIR verification failed");
    }

    // VM 実行はオプション扱い（NYASH_TRIM_MIN_VM=1 のときだけ実行）。
    if std::env::var("NYASH_TRIM_MIN_VM").ok().as_deref() == Some("1") {
        use crate::backend::VM;
        let mut vm = VM::new();
        let vm_out = vm
            .execute_module(&compiled.module)
            .expect("trim_min: VM execution failed");
        let result_str = vm_out.to_string_box().value;
        eprintln!("[trim/min] VM result='{}'", result_str);
        assert_eq!(result_str, "0", "trim_min: expected exit code 0");
    }

    // Cleanup env vars
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_ENABLE_USING");
    std::env::remove_var("HAKO_ENABLE_USING");
    std::env::remove_var("NYASH_PARSER_ALLOW_SEMICOLON");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
    std::env::remove_var("NYASH_MIR_DEBUG_LOG");
    std::env::remove_var("NYASH_VM_VERIFY_MIR");
    std::env::remove_var("NYASH_IF_HOLE_TRACE");
}
