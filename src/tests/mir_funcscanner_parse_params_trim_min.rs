// mir_funcscanner_parse_params_trim_min.rs
// Rust-level test for FuncScannerBox.parse_params + trim minimal SSA/PHI repro
//
// Goal:
// - Compile lang/src/compiler/entry/func_scanner.hako + minimal test .hako
// - Run MirVerifier to see if the undefined-value / dominator errors already
//   appear in this smaller case。
// - Optionally execute via VM to surface runtime InvalidValue errors。

use crate::ast::ASTNode;
use crate::mir::{MirCompiler, MirVerifier};
use crate::parser::NyashParser;

#[test]
fn mir_funcscanner_parse_params_trim_min_verify_and_vm() {
    // Minimal .hako that calls FuncScannerBox.parse_params + trim。
    let test_file = "lang/src/compiler/tests/funcscanner_parse_params_trim_min.hako";

    // Align parser env to Stage‑3 + using 経路（既存の skip_ws 系と揃えておく）。
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    // Optional: enable MIR debug / SSA debug when running this test manually。
    // std::env::set_var("NYASH_MIR_DEBUG_LOG", "1");
    // std::env::set_var("NYASH_VM_VERIFY_MIR", "1");
    // std::env::set_var("NYASH_IF_HOLE_TRACE", "1");

    // Bundle FuncScanner 本体と最小テスト。
    let func_scanner_src = include_str!("../../lang/src/compiler/entry/func_scanner.hako");
    let test_src = std::fs::read_to_string(test_file).expect("Failed to read minimal test .hako");
    let src = format!("{func_scanner_src}\n\n{test_src}");

    let ast: ASTNode =
        NyashParser::parse_from_string(&src).expect("parse_params_trim_min: parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc
        .compile(ast)
        .expect("parse_params_trim_min: MIR compile failed");

    eprintln!(
        "[parse-params-trim/min] module functions = {}",
        compiled.module.functions.len()
    );

    // Optional MIR dump for targeted functions when NYASH_MIR_TEST_DUMP=1
    if std::env::var("NYASH_MIR_TEST_DUMP").ok().as_deref() == Some("1") {
        use crate::mir::MirPrinter;
        let printer = MirPrinter::new();
        for name in [
            "FuncScannerBox.parse_params/1",
            "FuncScannerBox.trim/1",
            "main",
        ] {
            if let Some(func) = compiled.module.functions.get(name) {
                let dump = printer.print_function(func);
                eprintln!("----- MIR DUMP: {} -----\n{}", name, dump);
            } else {
                eprintln!("[parse-params-trim/min] WARN: function not found: {}", name);
            }
        }
    }

    // MIR verify: ここで Undefined / dominator エラーが出るかを見る。
    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        eprintln!("[parse-params-trim/min] MIR verification errors:");
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        // いまは「バグ検出」が目的なので、失敗しても panic しておく。
        panic!("parse_params_trim_min: MIR verification failed");
    }

    // VM 実行も一度試しておく（将来の回 regressions 用）。
    use crate::backend::VM;
    let mut vm = VM::new();
    let vm_out = vm
        .execute_module(&compiled.module)
        .expect("parse_params_trim_min: VM execution failed");
    let result_str = vm_out.to_string_box().value;
    eprintln!("[parse-params-trim/min] VM result='{}'", result_str);

    // Main.main は成功時 0 を返す想定。
    assert_eq!(
        result_str, "0",
        "parse_params_trim_min: expected exit code 0"
    );

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
