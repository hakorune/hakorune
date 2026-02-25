// Smallest repro for skip_whitespace SSA/PHI issues
// Inputs: lang/src/compiler/tests/funcscanner_skip_ws_min.hako
// Goal: verify + VM execute without undefined ValueId

use crate::ast::ASTNode;
use crate::mir::{MirCompiler, MirVerifier};
use crate::parser::NyashParser;

#[test]
fn mir_funcscanner_skip_ws_min_verify_and_vm() {
    let test_file = "lang/src/compiler/tests/funcscanner_skip_ws_min.hako";

    // Stage-3 + using
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");
    std::env::set_var("NYASH_DISABLE_PLUGINS", "1");

    let src = std::fs::read_to_string(test_file).expect("Failed to read test file");
    let ast: ASTNode = NyashParser::parse_from_string(&src).expect("parse failed");

    let mut mc = MirCompiler::with_options(false);
    let compiled = mc.compile(ast).expect("compile failed");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&compiled.module) {
        panic!("MIR verify failed: {:?}", errors);
    }

    use crate::backend::VM;
    let mut vm = VM::new();
    vm.execute_module(&compiled.module).expect("VM exec failed");

    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_FEATURES");
    std::env::remove_var("NYASH_ENABLE_USING");
    std::env::remove_var("HAKO_ENABLE_USING");
    std::env::remove_var("NYASH_DISABLE_PLUGINS");
}
