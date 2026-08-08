use super::{BuildGateReachabilityV1, PreparedBuildGateDecisionSetV1};
use crate::ast::BuildPredicate;
use crate::parser::build_gate_selection::BuildGateSelectionOutcomeV1;
use crate::parser::{NyashParser, ParseError, ParserBuildConfig};
use crate::tokenizer::NyashTokenizer;

fn parse_with_config(
    source: &str,
    config: ParserBuildConfig,
) -> Result<(NyashParser, crate::ast::ASTNode), ParseError> {
    let tokens = NyashTokenizer::new(source).tokenize()?;
    let mut parser = NyashParser::new(tokens).with_build_config(config);
    let ast = parser.parse_program()?;
    Ok((parser, ast))
}

#[test]
fn i0_c_issues_one_row_per_nested_ast_gate_and_preserves_reachability() {
    let (mut parser, ast) = parse_with_config(
        "gate Build.debug { gate Build.debug { box Inner {} } }\n",
        ParserBuildConfig::default(),
    )
    .expect("fixture must parse");

    let decisions = parser
        .issue_build_gate_decision_set(&ast)
        .expect("decision set must seal");
    assert_eq!(decisions.rows().len(), 2);
    assert_eq!(
        decisions.rows()[0].selected_branch,
        BuildGateSelectionOutcomeV1::NoElse
    );
    assert_eq!(
        decisions.rows()[0].reachability,
        BuildGateReachabilityV1::Reachable
    );
    assert_eq!(
        decisions.rows()[1].selected_branch,
        BuildGateSelectionOutcomeV1::NoElse
    );
    assert_eq!(
        decisions.rows()[1].reachability,
        BuildGateReachabilityV1::InactiveSubtree
    );
}

#[test]
fn i0_c_validates_unknown_feature_even_inside_inactive_subtree() {
    let (mut parser, ast) = parse_with_config(
        "gate Build.debug { gate Feature(\"missing\") { box Hidden {} } }\n",
        ParserBuildConfig::default(),
    )
    .expect("fixture must parse before BuildCfg evaluation");

    let error = parser
        .issue_build_gate_decision_set(&ast)
        .expect_err("unknown feature must fail-fast");
    assert!(
        matches!(error, ParseError::BuildCfg { message, .. } if message.contains("unknown feature"))
    );
}

#[test]
fn i0_c_rejects_observation_predicate_mismatch() {
    let (mut parser, ast) = parse_with_config(
        "gate Build.release { box Visible {} }\n",
        ParserBuildConfig::default(),
    )
    .expect("fixture must parse");
    parser.build_gate_observations[0].predicate = BuildPredicate::BuildFlag("debug".to_owned());

    let error = parser
        .issue_build_gate_decision_set(&ast)
        .expect_err("AST and parser observation must co-seal exactly");
    assert!(
        matches!(error, ParseError::BuildCfg { message, .. } if message.contains("observation/AST mismatch"))
    );
}

#[test]
fn i0_c_empty_program_has_a_zero_row_decision_set() {
    let (mut parser, ast) =
        parse_with_config("\n", ParserBuildConfig::default()).expect("empty fixture must parse");
    let decisions = parser
        .issue_build_gate_decision_set(&ast)
        .expect("empty decision set must seal");
    assert!(decisions.rows().is_empty());
}

#[test]
fn i0_c_product_is_non_clone_and_keeps_parser_brand() {
    let (mut parser, ast) = parse_with_config(
        "gate Build.release { box Visible {} }\n",
        ParserBuildConfig::default(),
    )
    .expect("fixture must parse");
    let expected = parser.source_invocation_brand();
    let decisions: PreparedBuildGateDecisionSetV1 = parser
        .issue_build_gate_decision_set(&ast)
        .expect("decision set must seal");
    assert_eq!(decisions.brand(), &expected);
}
