use nyash_rust::ast::ASTNode;
use nyash_rust::parser::{BuildMode, NyashParser, ParserBuildConfig};
use std::collections::BTreeSet;

#[test]
fn prunes_top_level_build_test_gate() {
    let ast = NyashParser::parse_from_string_with_build_config(
        r#"
gate Build.test {
    import "HakoTest"
    function testOnly() {
        return 1
    }
} else {
    function releaseOnly() {
        return 2
    }
}
"#,
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect("build cfg gate should parse");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    assert_eq!(statements.len(), 2);
    assert_eq!(statements[0].node_type(), "ImportStatement");
    assert_eq!(statements[1].node_type(), "FunctionDeclaration");
}

#[test]
fn prunes_feature_and_target_predicates() {
    let ast = NyashParser::parse_from_string_with_build_config(
        r#"
gate all(Feature("alloc.fastpath"), Target.os == linux) {
    function enabled() {
        return 1
    }
}
"#,
        ParserBuildConfig {
            mode: BuildMode::Release,
            known_features: BTreeSet::from(["alloc.fastpath".to_string()]),
            enabled_features: BTreeSet::from(["alloc.fastpath".to_string()]),
            target_os: "linux".to_string(),
            ..ParserBuildConfig::default()
        },
    )
    .expect("compound build cfg predicate should parse");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    assert_eq!(statements.len(), 1);
    assert_eq!(statements[0].node_type(), "FunctionDeclaration");
}

#[test]
fn rejects_unknown_feature_during_prune() {
    let err = NyashParser::parse_from_string(
        r#"
gate Feature("typo.feature") {
    function enabled() {
        return 1
    }
}
"#,
    )
    .expect_err("unknown feature should fail fast");

    assert!(
        format!("{err}").contains("unknown feature 'typo.feature'"),
        "unexpected error: {err}"
    );
}

#[test]
fn gate_remains_contextual_identifier_outside_item_head() {
    let ast = NyashParser::parse_from_string(
        r#"
function main() {
    local gate = 1
    return gate
}
"#,
    )
    .expect("ordinary identifier named gate should parse");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    assert_eq!(statements.len(), 1);
    assert_eq!(statements[0].node_type(), "FunctionDeclaration");
}
