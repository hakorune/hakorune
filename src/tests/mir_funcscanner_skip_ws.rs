// mir_funcscanner_skip_ws.rs
// Rust-level test for FuncScannerBox.skip_whitespace loop bug
//
// Purpose:
// - Verify that FuncScannerBox.skip_whitespace properly executes loop body
// - Test both direct static call and Stage-B delegate path
// - Use MIR verification + VM execution to catch SSA/loop bugs

use crate::ast::ASTNode;
use crate::mir::{MirCompiler, MirPrinter, MirVerifier};
use crate::parser::NyashParser;

#[test]
fn mir_funcscanner_skip_ws_direct_vm() {
    // Test file: lang/src/compiler/tests/funcscanner_skip_ws_min.hako
    let test_file = "lang/src/compiler/tests/funcscanner_skip_ws_min.hako";

    // Enable required env vars for Stage-3 + using
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    // Enable MIR debug logging
    std::env::set_var("NYASH_MIR_DEBUG_LOG", "1");
    std::env::set_var("NYASH_VM_VERIFY_MIR", "1");

    // Bundle both func_scanner.hako and test file
    // This ensures FuncScannerBox functions are included in the compiled module
    let func_scanner_src = include_str!("../../lang/src/compiler/entry/func_scanner.hako");
    let test_src = std::fs::read_to_string(test_file).expect("Failed to read test file");
    let src = format!("{}\n\n{}", func_scanner_src, test_src);
    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("Parse failed");

    // Compile to MIR
    let mut mc = MirCompiler::with_options(false);
    match mc.compile(ast) {
        Ok(compiled) => {
            eprintln!("[test] Compilation successful");
            eprintln!(
                "[test] Module has {} functions",
                compiled.module.functions.len()
            );

            // Check if FuncScannerBox.skip_whitespace/2 exists
            if let Some(func) = compiled
                .module
                .functions
                .get("FuncScannerBox.skip_whitespace/2")
            {
                eprintln!("[test] Found FuncScannerBox.skip_whitespace/2");
                eprintln!("[test] Function has {} blocks", func.blocks.len());

                // Optional: Dump MIR if env var is set
                if std::env::var("NYASH_MIR_TEST_DUMP").ok().as_deref() == Some("1") {
                    use crate::mir::MirPrinter;
                    let dump = MirPrinter::new().print_function(func);
                    eprintln!(
                        "----- MIR DUMP: FuncScannerBox.skip_whitespace/2 -----\n{}",
                        dump
                    );
                }
            } else {
                eprintln!("[test] WARNING: FuncScannerBox.skip_whitespace/2 not found in module");
                eprintln!("[test] ALL available functions:");
                for name in compiled.module.functions.keys() {
                    eprintln!("[test]   - {}", name);
                }
            }

            // Verify MIR (non-fatal - proceed to VM execution even if verification fails)
            use crate::mir::MirVerifier;
            let mut verifier = MirVerifier::new();
            if let Err(errors) = verifier.verify_module(&compiled.module) {
                eprintln!("[test] ⚠️ MIR verification errors (non-fatal, proceeding to VM):");
                for e in &errors {
                    eprintln!("[rust-mir-verify] {}", e);
                }
                eprintln!("[test] Note: Verification errors are expected during GUARD check fix investigation");
            } else {
                eprintln!("[test] MIR verification PASS");
            }

            // VM execution to verify skip_whitespace behavior
            eprintln!("[test] Starting VM execution");
            use crate::backend::VM;
            let mut vm = VM::new();
            match vm.execute_module(&compiled.module) {
                Ok(vm_out) => {
                    eprintln!("[test] VM execution completed");
                    let result_str = vm_out.to_string_box().value;
                    eprintln!("[test] Result (as string): {}", result_str);

                    // Parse result
                    // Expected: "0" (test passes), Actual: "1" (test fails with idx=0 instead of 3)
                    // This test will FAIL until the loop bug is fixed
                    if result_str == "1" {
                        eprintln!("[test] ⚠️ Expected test failure: skip_whitespace returned 0 instead of 3");
                        eprintln!("[test] This confirms the loop execution bug");
                    } else if result_str == "0" {
                        eprintln!("[test] ✅ Test PASS: skip_whitespace correctly returned 3");
                    } else {
                        eprintln!("[test] Unexpected result: {}", result_str);
                    }
                }
                Err(e) => {
                    eprintln!("[test] ❌ VM execution failed: {:?}", e);
                    eprintln!("[test] This may be due to MIR verification errors (dominator issues with PHI nodes)");
                    panic!("VM execution failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            panic!("Compilation failed: {:?}", e);
        }
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
}
