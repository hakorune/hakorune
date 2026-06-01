use nyash_rust::ast::ASTNode;
use nyash_rust::parser::{BuildMode, BuildGateExplainReport, NyashParser, ParserBuildConfig};
use std::collections::BTreeSet;

#[test]
fn explain_report_counts_active_and_inactive_branches() {
    let (ast, report) = NyashParser::parse_from_string_with_build_config_and_explain_report(
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
    .expect("build cfg gate explain should parse");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    assert_eq!(statements.len(), 2);
    assert_eq!(statements[0].node_type(), "ImportStatement");
    assert_eq!(statements[1].node_type(), "FunctionDeclaration");

    assert_eq!(report.output_contract, BuildGateExplainReport::OUTPUT_CONTRACT);
    assert_eq!(report.conditional_group_count, 1);
    assert_eq!(report.active_branch_count, 1);
    assert_eq!(report.inactive_branch_count, 1);
    assert_eq!(report.inactive_branch_mir_count, 0);
    assert_eq!(
        report.to_kv_lines(),
        vec![
            "output_contract=hakorune-build-cfg-explain-v0".to_string(),
            "conditional_group_count=1".to_string(),
            "active_branch_count=1".to_string(),
            "inactive_branch_count=1".to_string(),
            "inactive_branch_mir_count=0".to_string(),
            "summary=ok".to_string(),
        ]
    );
}

#[test]
fn explain_report_smoke_differs_by_build_mode() {
    let source = r#"
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
"#;

    let (test_ast, test_report) = NyashParser::parse_from_string_with_build_config_and_explain_report(
        source,
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect("test build cfg explain should parse");

    let (release_ast, release_report) = NyashParser::parse_from_string_with_build_config_and_explain_report(
        source,
        ParserBuildConfig {
            mode: BuildMode::Release,
            ..ParserBuildConfig::default()
        },
    )
    .expect("release build cfg explain should parse");

    let ASTNode::Program { statements: test_statements, .. } = test_ast else {
        panic!("expected Program");
    };
    let ASTNode::Program {
        statements: release_statements,
        ..
    } = release_ast else {
        panic!("expected Program");
    };

    assert_eq!(test_statements.len(), 2);
    assert_eq!(release_statements.len(), 1);
    assert_eq!(test_statements[0].node_type(), "ImportStatement");
    assert_eq!(release_statements[0].node_type(), "FunctionDeclaration");

    assert_eq!(test_report.conditional_group_count, 1);
    assert_eq!(release_report.conditional_group_count, 1);
    assert_eq!(test_report.active_branch_count, 1);
    assert_eq!(release_report.active_branch_count, 1);
    assert_eq!(test_report.inactive_branch_count, 1);
    assert_eq!(release_report.inactive_branch_count, 1);
    assert_eq!(test_report.inactive_branch_mir_count, 0);
    assert_eq!(release_report.inactive_branch_mir_count, 0);
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
