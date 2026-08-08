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
    let relations = parsed.generated_delegate_source_relations();
    assert_eq!(relations.len(), 1);
    assert_eq!(relations[0].exposed_method_name(), "runAlias");
    assert_eq!(relations[0].source_method_name(), "run");
    assert_eq!(relations[0].delegate_field_name(), "target");
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

#[test]
fn r6_s3b_c_i0_zero_delegate_program_is_an_exact_noop() {
    let parsed = NyashParser::parse_from_string_with_source_seal(
        r#"
box Plain {
    run() { return 1 }
}
"#,
        ParserBuildConfig::default(),
    )
    .expect("ordinary Box without delegates should remain a valid source product");

    assert!(parsed.generated_delegate_source_relations().is_empty());
    let ASTNode::Program { statements, .. } = parsed.ast() else {
        panic!("source-sealed parse must return a Program AST");
    };
    let ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
        panic!("fixture must contain one Box");
    };
    assert_eq!(methods.len(), 1);
    assert!(methods.get("runAlias").is_none());
}
