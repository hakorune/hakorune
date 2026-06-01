use nyash_rust::ast::ASTNode;
use nyash_rust::ast::LiteralValue;
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

#[test]
fn member_level_gate_selects_box_declarations_without_layout_drift() {
    let source = r#"
box ChoiceBox {
    gate Build.test {
        value: i64
        choose() {
            return 1
        }
    } else {
        value: i64
        choose() {
            return 2
        }
    }
}
"#;

    let ast = NyashParser::parse_from_string_with_build_config(
        source,
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect("member-level gate should parse in test build");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    assert_eq!(statements.len(), 1);

    let ASTNode::BoxDeclaration {
        fields,
        field_decls,
        methods,
        ..
    } = &statements[0]
    else {
        panic!("expected BoxDeclaration");
    };
    assert_eq!(fields, &vec!["value".to_string()]);
    assert_eq!(field_decls.len(), 1);
    assert_eq!(field_decls[0].name, "value");
    assert_eq!(field_decls[0].declared_type_name.as_deref(), Some("i64"));

    let choose = methods.get("choose").expect("expected choose() method");
    let ASTNode::FunctionDeclaration { body, .. } = choose else {
        panic!("expected FunctionDeclaration");
    };
    assert_eq!(body.len(), 1);
    let ASTNode::Return { value: Some(expr), .. } = &body[0] else {
        panic!("expected return statement");
    };
    let ASTNode::Literal {
        value: LiteralValue::Integer(n),
        ..
    } = expr.as_ref()
    else {
        panic!("expected integer literal");
    };
    assert_eq!(*n, 1);
}

#[test]
fn member_level_gate_rejects_layout_drift_by_default() {
    let err = NyashParser::parse_from_string_with_build_config(
        r#"
box DriftBox {
    gate Build.test {
        value: i64
        choose() {
            return 1
        }
    } else {
        other: i64
        choose() {
            return 2
        }
    }
}
"#,
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect_err("layout drift should be rejected");

    assert!(
        format!("{err}").contains("same public signature"),
        "unexpected error: {err}"
    );
}

#[test]
fn statement_level_gate_prunes_inactive_branch_inside_method_body() {
    let source = r#"
function main() {
    gate Build.test {
        local chosen = 1
        return chosen
    } else {
        local chosen = 2
        return chosen
    }
}
"#;

    let test_ast = NyashParser::parse_from_string_with_build_config(
        source,
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect("statement-level gate should parse in test build");

    let release_ast = NyashParser::parse_from_string_with_build_config(
        source,
        ParserBuildConfig {
            mode: BuildMode::Release,
            ..ParserBuildConfig::default()
        },
    )
    .expect("statement-level gate should parse in release build");

    let ASTNode::Program { statements: test_statements, .. } = test_ast else {
        panic!("expected Program");
    };
    let ASTNode::Program {
        statements: release_statements,
        ..
    } = release_ast else {
        panic!("expected Program");
    };

    assert_eq!(test_statements.len(), 1);
    assert_eq!(release_statements.len(), 1);

    let ASTNode::FunctionDeclaration { body: test_body, .. } = &test_statements[0] else {
        panic!("expected FunctionDeclaration");
    };
    let ASTNode::FunctionDeclaration {
        body: release_body,
        ..
    } = &release_statements[0] else {
        panic!("expected FunctionDeclaration");
    };

    assert_eq!(test_body.len(), 2);
    assert_eq!(release_body.len(), 2);
    assert_eq!(test_body[0].node_type(), "Local");
    assert_eq!(release_body[0].node_type(), "Local");
    assert_eq!(test_body[1].node_type(), "Return");
    assert_eq!(release_body[1].node_type(), "Return");

    let ASTNode::Local {
        initial_values: test_initial_values,
        ..
    } = &test_body[0] else {
        panic!("expected Local");
    };
    let ASTNode::Local {
        initial_values: release_initial_values,
        ..
    } = &release_body[0] else {
        panic!("expected Local");
    };

    let Some(Some(test_expr)) = test_initial_values.first() else {
        panic!("expected test init value");
    };
    let Some(Some(release_expr)) = release_initial_values.first() else {
        panic!("expected release init value");
    };

    let ASTNode::Literal {
        value: LiteralValue::Integer(test_n),
        ..
    } = test_expr.as_ref()
    else {
        panic!("expected integer literal");
    };
    let ASTNode::Literal {
        value: LiteralValue::Integer(release_n),
        ..
    } = release_expr.as_ref()
    else {
        panic!("expected integer literal");
    };

    assert_eq!(*test_n, 1);
    assert_eq!(*release_n, 2);
}

#[test]
fn statement_level_gate_is_counted_in_build_cfg_explain_report() {
    let source = r#"
function main() {
    gate Build.test {
        local chosen = 1
        return chosen
    } else {
        local chosen = 2
        return chosen
    }
}
"#;

    let (_ast, report) = NyashParser::parse_from_string_with_build_config_and_explain_report(
        source,
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect("statement-level gate should parse in explain mode");

    assert_eq!(report.output_contract, BuildGateExplainReport::OUTPUT_CONTRACT);
    assert_eq!(report.conditional_group_count, 1);
    assert_eq!(report.active_branch_count, 1);
    assert_eq!(report.inactive_branch_count, 1);
    assert_eq!(report.inactive_branch_mir_count, 0);
}
