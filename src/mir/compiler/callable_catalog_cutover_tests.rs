use crate::parser::NyashParser;

use super::MirCompiler;

fn parse(source: &str) -> crate::ast::ASTNode {
    NyashParser::parse_from_string(source).expect("CUT0 fixture must parse")
}

#[test]
fn ambiguous_rejection_does_not_poison_following_catalog_session() {
    let ambiguous = include_str!(concat!(
        "../../../apps/bare-static-recovery-proof/",
        "ambiguous.hako"
    ));
    let valid = include_str!(concat!(
        "../../../apps/bare-static-recovery-proof/",
        "provider_first_script.hako"
    ));

    let mut compiler = MirCompiler::new();
    let error = compiler.compile(parse(ambiguous)).unwrap_err().to_string();
    assert!(error.contains("Unresolved function: 'm_seed'"));

    let result = compiler
        .compile(parse(valid))
        .expect("next root must install a fresh complete catalog");
    assert!(result.module.functions.contains_key("Helpers.m_seed/1"));
    assert!(result.module.functions.contains_key("Helpers.z_use/1"));
}
