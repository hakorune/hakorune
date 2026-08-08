use super::*;
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};

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
    assert!(matches!(parsed.ast(), ASTNode::Program { .. }));
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
