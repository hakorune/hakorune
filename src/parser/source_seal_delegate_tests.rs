use super::*;
use crate::parser::ParserBuildConfig;

#[test]
fn r6_s3b_c_s0_transports_delegate_rows_but_keeps_them_out_of_final_seal() {
    let parsed = NyashParser::parse_from_string_with_source_seal(
        r#"
box Target {
    run() { return 1 }
}
box Host {
    target: Target
    delegate target exposes { run as runAlias }
}
"#,
        ParserBuildConfig::default(),
    )
    .expect("delegate source transport should not change parsing");

    let host_seal = &parsed.source_seals()[1];
    assert!(host_seal.prepared.delegate_source_declarations.is_empty());
    assert!(host_seal.inventory().get("runAlias").is_none());
    let generated = match parsed.ast() {
        ASTNode::Program { statements, .. } => statements
            .iter()
            .find_map(|statement| match statement {
                ASTNode::BoxDeclaration { name, methods, .. } if name == "Host" => {
                    methods.get("runAlias")
                }
                _ => None,
            })
            .expect("delegate generated method must remain in descriptive AST"),
        _ => panic!("source-sealed parse must return a Program AST"),
    };
    assert!(matches!(
        generated.provenance(),
        crate::ast::BoxMethodProvenanceV1::Generated(
            crate::ast::BoxMethodGeneratedProvenanceV1::Delegate { .. }
        )
    ));
}
