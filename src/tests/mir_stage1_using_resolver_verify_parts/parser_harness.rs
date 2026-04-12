use crate::ast::ASTNode;
use crate::mir::{MirCompiler, MirVerifier};
use crate::parser::NyashParser;

use super::shared::ensure_stage3_env;

/// Verify MIR/SSA for ParserBox.parse_program2 in isolation by compiling a small wrapper.
#[test]
fn mir_parserbox_parse_program2_harness_parses_minimal_source() {
    ensure_stage3_env();
    let src = r#"
using lang.compiler.parser.parser_box as ParserBox

static box ParserBoxHarness {
  method main(src) {
    local p = new ParserBox()
    p.stage3_enable(1)
    return p.parse_program2(src)
  }
}
"#;

    let harness_ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(harness_ast).expect("compile");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for ParserBoxHarness");
    }
}
