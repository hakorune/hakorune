use crate::ast::ASTNode;
use crate::mir::{printer::MirPrinter, MirCompiler, MirVerifier};
use crate::parser::NyashParser;

#[test]
fn mir_stage1_cli_stage1_main_compiles_and_verifies() {
    // Stage‑3 + using を有効化して stage1_cli.hako をそのままパースする。
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");

    let src = include_str!("../../lang/src/runner/stage1_cli.hako");
    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");

    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    // オプション: 環境変数で Stage1Cli.stage1_main の MIR をダンプできるようにする。
    if std::env::var("NYASH_STAGE1_MAIN_DUMP").ok().as_deref() == Some("1") {
        let printer = MirPrinter::verbose();
        let txt = printer.print_module(&cr.module);
        eprintln!("=== MIR stage1_cli.hako ===\n{}", txt);
    }

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for stage1_cli.hako (stage1_main and related paths)");
    }
}
