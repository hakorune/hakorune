use super::*;
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};

fn open_postpass_product_for_syntax(source: &str) -> OpenParserPostpassProductV1 {
    let config = ParserBuildConfig::default();
    let pre = super::super::normalize_logical_ops(source);
    let mut tokenizer =
        crate::tokenizer::NyashTokenizer::with_grammar_profile(pre, config.grammar_profile);
    let tokens = tokenizer.tokenize().expect("test source must tokenize");
    let mut parser = NyashParser::new(tokens);
    parser.build_config = config;
    let ast = parser.parse_program().expect("test source must parse");
    parser
        .open_postpass_product(ast)
        .expect("test source must issue BuildGate decision set")
        .prune_build_gates(&parser)
        .expect("test source must produce an open postpass product")
        .lower_delegates()
        .expect("test source must lower delegates")
}

#[test]
fn r6_s3_finalizes_ordinary_box_after_final_parse_postpass() {
    let parsed = NyashParser::parse_from_string_with_source_seal(
        r#"
box Plain {
    run() { return 1 }
}
"#,
        ParserBuildConfig::default(),
    )
    .expect("ordinary Box should issue the final source seal");

    assert_eq!(parsed.source_seals().len(), 1);
    let seal = &parsed.source_seals()[0];
    assert_eq!(seal.inventory().len(), 1);
    assert_eq!(seal.inventory().get("run").unwrap().name(), "run");
    assert_eq!(seal.method_relations().len(), 1);
    assert_eq!(seal.declaration_syntax().name(), "Plain");
    assert_eq!(
        seal.declaration_syntax().kind(),
        crate::parser::source_authority::ParserBoxDeclarationKindV1::Ordinary
    );
    assert!(!seal.declaration_syntax().is_sync());
    assert!(matches!(parsed.ast(), ASTNode::Program { .. }));
}

#[test]
fn r6_s3b_b4_captures_sync_box_syntax_in_the_same_source_seal() {
    let parsed = NyashParser::parse_from_string_with_source_seal(
        r#"
sync box Synchronized {
    run() { return 1 }
}
"#,
        ParserBuildConfig::default(),
    )
    .expect("sync ordinary Box should retain its parser declaration syntax");

    let [seal] = parsed.source_seals() else {
        panic!("expected one source seal")
    };
    assert_eq!(seal.declaration_syntax().name(), "Synchronized");
    assert!(seal.declaration_syntax().is_sync());
}

#[test]
fn r6_s3b_b4_rejects_final_ast_declaration_syntax_drift() {
    let mut renamed = open_postpass_product_for_syntax(
        r#"
box Plain {
    run() { return 1 }
}
"#,
    );
    let ASTNode::Program { statements, .. } = &mut renamed.ast else {
        panic!("source product must contain a Program");
    };
    let ASTNode::BoxDeclaration { name, .. } = &mut statements[0] else {
        panic!("source product must contain a Box");
    };
    *name = "Renamed".to_owned();
    assert!(matches!(
        renamed.finalize(),
        Err(SourceSealFinalizationErrorV1::DeclarationNameMismatch { .. })
    ));

    let mut resynced = open_postpass_product_for_syntax(
        r#"
box Plain {
    run() { return 1 }
}
"#,
    );
    let ASTNode::Program { statements, .. } = &mut resynced.ast else {
        panic!("source product must contain a Program");
    };
    let ASTNode::BoxDeclaration { is_sync, .. } = &mut statements[0] else {
        panic!("source product must contain a Box");
    };
    *is_sync = true;
    assert!(matches!(
        resynced.finalize(),
        Err(SourceSealFinalizationErrorV1::DeclarationSyncMismatch { .. })
    ));
}

#[test]
fn r6_s3b_b2_prunes_selected_top_level_gate_and_preserves_box_path() {
    let parsed = NyashParser::parse_from_string_with_source_seal(
        r#"
gate Build.test {
    box ThenBox { run() { return 1 } }
} else {
    box ElseBox { run() { return 2 } }
}
"#,
        ParserBuildConfig::default(),
    )
    .expect("release config should select the else branch");

    assert_eq!(parsed.source_seals().len(), 1);
    assert!(matches!(
        parsed.ast(),
        ASTNode::Program { statements, .. }
            if matches!(statements.as_slice(), [ASTNode::BoxDeclaration { name, .. }] if name == "ElseBox")
    ));
    assert!(matches!(
        parsed.source_seals()[0].box_site().path().segments(),
        [
            crate::parser::source_path::SourceBoxPathSegmentV1::RootStatement { ordinal: 0 },
            crate::parser::source_path::SourceBoxPathSegmentV1::BuildGate {
                branch: crate::parser::source_authority::SourceBuildGateBranchV1::Else,
                ..
            }
        ]
    ));
}

#[test]
fn r6_s3b_b2_prunes_nested_top_level_gate_once() {
    let config = ParserBuildConfig {
        mode: BuildMode::Test,
        ..ParserBuildConfig::default()
    };
    let parsed = NyashParser::parse_from_string_with_source_seal(
        r#"
gate Build.test {
            gate Build.test {
        box NestedBox { run() { return 1 } }
    }
}
"#,
        config,
    )
    .expect("nested selected gate should issue one rich source product");

    assert_eq!(parsed.source_seals().len(), 1);
    assert!(matches!(
        parsed.source_seals()[0].box_site().path().segments(),
        [
            crate::parser::source_path::SourceBoxPathSegmentV1::RootStatement { ordinal: 0 },
            crate::parser::source_path::SourceBoxPathSegmentV1::BuildGate { .. },
            crate::parser::source_path::SourceBoxPathSegmentV1::BuildGate { .. }
        ]
    ));
}

#[test]
fn r6_s3b_b2_empty_gate_has_no_source_seal_and_still_finalizes() {
    let parsed = NyashParser::parse_from_string_with_source_seal(
        "gate Build.test { } else { }",
        ParserBuildConfig::default(),
    )
    .expect("empty gate should have exact ledger/receipt coverage");
    assert_eq!(parsed.source_seals().len(), 0);
    assert!(matches!(
        parsed.ast(),
        ASTNode::Program { statements, .. } if statements.is_empty()
    ));
}

#[test]
fn r6_s3_does_not_issue_a_partial_seal_for_unsupported_top_level_box() {
    let error = NyashParser::parse_from_string_with_source_seal(
        r#"
static box StaticOnly { run() { return 1 } }
"#,
        ParserBuildConfig::default(),
    )
    .expect_err("static Box must remain outside the bounded rich product");
    assert!(
        error.to_string().contains("ordinary top-level Box only")
            || error
                .to_string()
                .contains("outside the ordinary C-I0 cohort")
    );
}
