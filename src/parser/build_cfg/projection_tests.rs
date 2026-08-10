use crate::ast::ASTNode;
use crate::parser::build_gate_selection::BuildGateSelectionOutcomeV1;
use crate::parser::postpass_envelope::{ExplainDemandV1, PostpassDemandV1};
use crate::parser::source_authority::{SourceBoxPathSegmentV1, SourceBuildGateBranchV1};
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

fn projected_gate_slots(mode: BuildMode) -> super::BuildGateProjectionOutputV1 {
    let tokens = NyashTokenizer::new(
        "function before() {}\n\
         gate Build.test { function chosen() {} } else { function chosen() {} }\n\
         function after() {}\n",
    )
    .tokenize()
    .unwrap();
    let mut parser = NyashParser::new(tokens);
    parser.build_config = ParserBuildConfig {
        mode,
        ..ParserBuildConfig::default()
    };
    let ast = parser.parse_program().unwrap();
    let decisions = parser.issue_build_gate_decision_set(&ast).unwrap();
    let records = parser.take_source_build_gate_records();
    super::project_build_gates(&parser, ast, &decisions, &records, false).unwrap()
}

#[test]
fn source_projection_records_exact_final_slots_for_then_and_else() {
    for (mode, expected_branch) in [
        (BuildMode::Test, SourceBuildGateBranchV1::Then),
        (BuildMode::Release, SourceBuildGateBranchV1::Else),
    ] {
        let output = projected_gate_slots(mode);
        let rows = output.item_slots.rows();
        assert_eq!(rows.len(), 3);
        assert_eq!(
            rows.iter()
                .map(|row| row.final_statement_slot())
                .collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        assert!(matches!(
            rows[0].source_path().compatibility_box_path().segments(),
            [SourceBoxPathSegmentV1::RootStatement { ordinal: 0 }]
        ));
        assert!(matches!(
            rows[1]
                .source_path()
                .compatibility_box_path()
                .segments(),
            [
                SourceBoxPathSegmentV1::RootStatement { ordinal: 1 },
                SourceBoxPathSegmentV1::BuildGate { branch, child_ordinal: 0, .. }
            ] if *branch == expected_branch
        ));
        assert!(matches!(
            rows[2].source_path().compatibility_box_path().segments(),
            [SourceBoxPathSegmentV1::RootStatement { ordinal: 2 }]
        ));
    }
}
