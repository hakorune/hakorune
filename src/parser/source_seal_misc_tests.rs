use super::*;
use crate::parser::{NyashParser, ParserBuildConfig};

#[test]
fn r6_s3b_a_ast_projection_matches_the_rich_product() {
    let source = r#"
box Plain {
    run() { return 1 }
}
"#;
    let rich =
        NyashParser::parse_from_string_with_source_seal(source, ParserBuildConfig::default())
            .expect("rich direct-Box product should finalize");
    let projected =
        NyashParser::parse_from_string_with_source_seal_ast(source, ParserBuildConfig::default())
            .expect("AST projection should use the rich path");

    assert_eq!(rich.into_ast(), projected);
}

#[test]
fn r6_s3b_a_rich_product_keeps_diagnostic_metadata_outside_source_seal() {
    let parsed = NyashParser::parse_from_string_with_source_seal(
        r#"@rune Public
box Plain {
    run() { return 1 }
}
"#,
        ParserBuildConfig::default(),
    )
    .expect("diagnostic rune metadata must not block the bounded product");

    assert_eq!(parsed.source_seals().len(), 1);
    assert_eq!(parsed.metadata().runes.len(), 1);
    assert_eq!(parsed.metadata().runes[0].name, "Public");
}

#[test]
fn r6_s3b_b3_keeps_delegate_suffix_outside_source_seal() {
    let parsed = NyashParser::parse_from_string_with_source_seal(
        r#"
box Target { run() { return 1 } }
box Host {
    target: Target
    delegate target exposes { run as runAlias }
}
"#,
        ParserBuildConfig::default(),
    )
    .expect("delegate postpass should be included before the final seal");

    assert_eq!(parsed.source_seals().len(), 2);
    let host = match parsed.ast() {
        ASTNode::Program { statements, .. } => statements
            .iter()
            .find_map(|statement| match statement {
                ASTNode::BoxDeclaration { name, methods, .. } if name == "Host" => Some(methods),
                _ => None,
            })
            .expect("delegate host must remain in the final AST"),
        _ => panic!("source-sealed parse must return a Program AST"),
    };
    let generated = host
        .get("runAlias")
        .expect("delegate generated method must remain in descriptive AST inventory");
    assert!(matches!(
        generated.provenance(),
        BoxMethodProvenanceV1::Generated(BoxMethodGeneratedProvenanceV1::Delegate { .. })
    ));
    assert!(parsed.source_seals()[1]
        .inventory()
        .get("runAlias")
        .is_none());
}
