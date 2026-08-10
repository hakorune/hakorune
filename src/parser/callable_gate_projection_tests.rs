use super::postpass_envelope::PostpassDemandV1;
use super::source_authority::{SourceBuildGateBranchV1, SourceProgramCallablePathV1};
use super::{BuildMode, NyashParser, ParserBuildConfig};
use crate::tokenizer::NyashTokenizer;

fn open_pruned(
    source: &str,
    config: ParserBuildConfig,
) -> super::source_seal::OpenParserPostpassProductV1 {
    let tokens = NyashTokenizer::new(source).tokenize().unwrap();
    let mut parser = NyashParser::new(tokens).with_build_config(config);
    let ast = parser.parse_program().unwrap();
    parser
        .open_postpass_product(ast)
        .unwrap()
        .prune_build_gates(&parser)
        .unwrap()
}

#[test]
fn selected_top_level_and_member_rows_are_pruned_in_one_transaction() {
    let product = open_pruned(
        "gate Build.test {\n\
           function selected_free() {}\n\
           box Choice { gate Build.test { run() {} } else { run() {} } }\n\
         } else { function unselected_free() {} }\n",
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    );
    let rows = product.source_session.direct_callable_rows();
    assert_eq!(rows.len(), 2);
    assert!(rows
        .iter()
        .any(|row| row.diagnostic_name() == "selected_free"));
    assert!(!rows
        .iter()
        .any(|row| row.diagnostic_name() == "unselected_free"));
    let member = rows
        .iter()
        .find(|row| row.diagnostic_name() == "run")
        .expect("selected member row");
    assert!(matches!(
        member.path(),
        SourceProgramCallablePathV1::BoxMethod { gate_path, .. }
            if gate_path.len() == 1
                && gate_path[0].branch() == SourceBuildGateBranchV1::Then
    ));
}

#[test]
fn nested_member_selection_keeps_the_full_selected_path_only() {
    let product = open_pruned(
        "box Choice {\n\
           gate Build.test { run() {} }\n\
           else gate Build.debug { run() {} } else { run() {} }\n\
         }\n",
        ParserBuildConfig::default(),
    );
    let rows = product.source_session.direct_callable_rows();
    assert_eq!(rows.len(), 1);
    assert!(matches!(
        rows[0].path(),
        SourceProgramCallablePathV1::BoxMethod { gate_path, .. }
            if gate_path.len() == 2
                && gate_path[0].branch() == SourceBuildGateBranchV1::Else
                && gate_path[1].branch() == SourceBuildGateBranchV1::Else
    ));
}

#[test]
fn top_level_else_and_nested_top_level_leaf_are_selected_exactly() {
    let product = open_pruned(
        "gate Build.test { function outer_then() {} } else {\n\
           gate Build.debug { function inner_then() {} }\n\
           else { function inner_else() {} }\n\
         }\n",
        ParserBuildConfig::default(),
    );
    let rows = product.source_session.direct_callable_rows();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].diagnostic_name(), "inner_else");
    let segments = rows[0]
        .path()
        .declaration()
        .compatibility_box_path()
        .segments();
    assert!(matches!(
        segments,
        [
            _,
            super::source_authority::SourceBoxPathSegmentV1::BuildGate {
                branch: SourceBuildGateBranchV1::Else,
                ..
            },
            super::source_authority::SourceBoxPathSegmentV1::BuildGate {
                branch: SourceBuildGateBranchV1::Else,
                ..
            }
        ]
    ));
}

#[test]
fn inactive_outer_member_branch_does_not_demand_its_nested_receipt() {
    let product = open_pruned(
        "box Choice {\n\
           gate Build.test {\n\
             gate Build.debug { run() {} } else { run() {} }\n\
           } else { run() {} }\n\
         }\n",
        ParserBuildConfig::default(),
    );
    let rows = product.source_session.direct_callable_rows();
    assert_eq!(rows.len(), 1);
    assert!(matches!(
        rows[0].path(),
        SourceProgramCallablePathV1::BoxMethod { gate_path, .. }
            if gate_path.len() == 1
                && gate_path[0].branch() == SourceBuildGateBranchV1::Else
    ));
}

#[test]
fn opening_postpass_twice_rejects_the_moved_callable_session() {
    let tokens = NyashTokenizer::new("function once() {}\n")
        .tokenize()
        .unwrap();
    let mut parser = NyashParser::new(tokens);
    let ast = parser.parse_program().unwrap();
    let second_ast = ast.clone();
    let _first = parser.open_postpass_product(ast).unwrap();
    let error = parser.open_postpass_product(second_ast).unwrap_err();
    assert!(format!("{error}").contains("already moved into postpass"));
}

#[test]
fn compatibility_finish_retains_selected_callable_rows_privately() {
    let tokens = NyashTokenizer::new(
        "static box Main { main() {} }\n\
         gate Build.test { function chosen() {} } else { function hidden() {} }\n",
    )
    .tokenize()
    .unwrap();
    let mut parser = NyashParser::new(tokens).with_build_config(ParserBuildConfig {
        mode: BuildMode::Test,
        ..ParserBuildConfig::default()
    });
    let ast = parser.parse_program().unwrap();
    let completed = parser
        .open_postpass_product(ast)
        .unwrap()
        .finish_total_s0(&parser, PostpassDemandV1::default())
        .unwrap();
    let names = completed
        .direct_callable_rows()
        .iter()
        .map(|row| row.diagnostic_name())
        .collect::<Vec<_>>();
    assert_eq!(names, vec!["main", "chosen"]);
}
