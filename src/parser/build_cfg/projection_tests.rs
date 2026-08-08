use crate::ast::ASTNode;
use crate::parser::build_gate_selection::BuildGateSelectionOutcomeV1;
use crate::parser::postpass_envelope::{ExplainDemandV1, PostpassDemandV1};
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};
use crate::tokenizer::NyashTokenizer;

#[test]
fn shared_projection_consumes_one_selected_source_gate() {
    let tokens =
        NyashTokenizer::new("gate Build.test { box Enabled {} } else { box Disabled {} }\n")
            .tokenize()
            .unwrap();
    let mut parser = NyashParser::new(tokens);
    parser.build_config = ParserBuildConfig {
        mode: BuildMode::Test,
        ..ParserBuildConfig::default()
    };
    let ast = parser.parse_program().unwrap();
    let envelope = parser
        .open_postpass_product(ast)
        .unwrap()
        .finish_total_s0(
            &parser,
            PostpassDemandV1 {
                explain: ExplainDemandV1::Capture,
            },
        )
        .unwrap();
    let report = envelope.explain().expect("shared projection report");
    assert_eq!(report.conditional_group_count, 1);
    assert_eq!(report.active_branch_count, 1);
    assert_eq!(report.inactive_branch_count, 1);
    let ASTNode::Program { statements, .. } = envelope.into_ast() else {
        panic!("expected projected program");
    };
    assert!(
        matches!(statements.as_slice(), [ASTNode::BoxDeclaration { name, .. }] if name == "Enabled")
    );
}

#[test]
fn shared_projection_emits_no_else_receipt_without_a_child_path() {
    let tokens = NyashTokenizer::new("gate Build.test { box Hidden {} }\n")
        .tokenize()
        .unwrap();
    let mut parser = NyashParser::new(tokens);
    let ast = parser.parse_program().unwrap();
    let decisions = parser.issue_build_gate_decision_set(&ast).unwrap();
    let records = parser.take_source_build_gate_records();

    let output = super::project_build_gates(&parser, ast, &decisions, &records, false).unwrap();
    assert_eq!(output.receipts.len(), 1);
    assert_eq!(
        output.receipts[0].selected_branch,
        BuildGateSelectionOutcomeV1::NoElse
    );
    assert!(matches!(
        output.ast,
        ASTNode::Program { statements, .. } if statements.is_empty()
    ));
}
