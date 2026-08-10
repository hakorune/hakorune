use super::*;
use crate::parser::source_authority::{
    ParserInvocationBrandV1, SourceBoxDeclarationPathV1, SourceBoxPathSegmentV1,
    SourceBuildGateBranchV1, SourceProgramCallablePathV1,
};
use crate::tokenizer::NyashTokenizer;

fn parse_rows(source: &str) -> NyashParser {
    let mut tokenizer = NyashTokenizer::new(source);
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = NyashParser::new(tokens);
    parser.parse_program().unwrap();
    parser
}

#[test]
fn mixed_direct_source_keeps_five_rows_across_four_direct_kinds() {
    let parser = parse_rows(
        "function free() {}\n\
         static function free_static() {}\n\
         static box Main { main() {} }\n\
         static box Utility { ping() {} }\n\
         box Node { value() {} }\n",
    );
    let kinds = parser
        .callable_source_session
        .as_ref()
        .unwrap()
        .rows()
        .iter()
        .map(PreparedDirectCallableSourceV1::kind)
        .collect::<Vec<_>>();
    assert_eq!(
        kinds,
        vec![
            DirectCallableDeclarationKindV1::FreeFunction,
            DirectCallableDeclarationKindV1::FreeStaticFunction,
            DirectCallableDeclarationKindV1::StaticBoxMethod,
            DirectCallableDeclarationKindV1::StaticBoxMethod,
            DirectCallableDeclarationKindV1::InstanceBoxMethod,
        ]
    );
    assert_eq!(
        parser
            .callable_source_session
            .as_ref()
            .unwrap()
            .rows()
            .iter()
            .map(PreparedDirectCallableSourceV1::diagnostic_name)
            .collect::<Vec<_>>(),
        vec!["free", "free_static", "main", "ping", "value"]
    );
    let placements = parser
        .callable_source_session
        .as_ref()
        .unwrap()
        .rows()
        .iter()
        .map(PreparedDirectCallableSourceV1::commit_placement)
        .collect::<Vec<_>>();
    assert!(matches!(
        placements.as_slice(),
        [
            DirectCallableCommitPlacementV1::TopLevel,
            DirectCallableCommitPlacementV1::TopLevel,
            DirectCallableCommitPlacementV1::BoxMethod { .. },
            DirectCallableCommitPlacementV1::BoxMethod { .. },
            DirectCallableCommitPlacementV1::BoxMethod { .. },
        ]
    ));
}

#[test]
fn generated_property_does_not_enter_the_direct_anchor_session() {
    let parser = parse_rows("box Generated { once value: i64 => 1 }\n");
    assert!(parser
        .callable_source_session
        .as_ref()
        .unwrap()
        .rows()
        .is_empty());
}

#[test]
fn generated_delegate_does_not_add_a_direct_anchor_row() {
    let parser = parse_rows(
        "box Target { run() { return 1 } }\n\
         box Host {\n\
           target: Target\n\
           delegate target exposes { run as runAlias }\n\
         }\n",
    );
    let rows = parser.callable_source_session.as_ref().unwrap().rows();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].diagnostic_name(), "run");
    assert_eq!(
        rows[0].kind(),
        DirectCallableDeclarationKindV1::InstanceBoxMethod
    );
}

#[test]
fn top_level_gate_children_keep_both_written_paths_before_selection() {
    let parser =
        parse_rows("gate Build.test { function chosen() {} } else { function chosen() {} }\n");
    let rows = parser.callable_source_session.as_ref().unwrap().rows();
    assert_eq!(rows.len(), 2);
    let branches = rows
        .iter()
        .map(|row| match row.path() {
            SourceProgramCallablePathV1::TopLevel { declaration } => {
                match declaration.compatibility_box_path().segments().last() {
                    Some(SourceBoxPathSegmentV1::BuildGate { branch, .. }) => *branch,
                    other => panic!("expected gated top-level path, got {other:?}"),
                }
            }
            other => panic!("expected top-level path, got {other:?}"),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        branches,
        vec![SourceBuildGateBranchV1::Then, SourceBuildGateBranchV1::Else]
    );
}

#[test]
fn member_gate_children_keep_both_written_paths_before_selection() {
    let parser = parse_rows(
        "box Choice {\n\
           gate Build.test { run() {} } else { run() {} }\n\
         }\n",
    );
    let rows = parser.callable_source_session.as_ref().unwrap().rows();
    assert_eq!(rows.len(), 2);
    let branches = rows
        .iter()
        .map(|row| match row.path() {
            SourceProgramCallablePathV1::BoxMethod { gate_path, .. } => {
                assert_eq!(gate_path.len(), 1);
                gate_path[0].branch()
            }
            other => panic!("expected Box method path, got {other:?}"),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        branches,
        vec![SourceBuildGateBranchV1::Then, SourceBuildGateBranchV1::Else]
    );
}

#[test]
fn nested_member_gate_keeps_the_full_written_branch_path() {
    let parser = parse_rows(
        "box Choice {\n\
           gate Build.test { run() {} } else gate Build.debug { run() {} } else { run() {} }\n\
         }\n",
    );
    let nested_then = parser
        .callable_source_session
        .as_ref()
        .unwrap()
        .rows()
        .iter()
        .find(|row| {
            matches!(
                row.path(),
                SourceProgramCallablePathV1::BoxMethod { gate_path, .. }
                    if gate_path.len() == 2
                        && gate_path[0].branch() == SourceBuildGateBranchV1::Else
                        && gate_path[1].branch() == SourceBuildGateBranchV1::Then
            )
        })
        .expect("nested then declaration must be retained");
    let SourceProgramCallablePathV1::BoxMethod { gate_path, .. } = nested_then.path() else {
        panic!("nested member must retain a Box-method path")
    };
    assert_eq!(gate_path.len(), 2);
    assert_eq!(gate_path[0].branch(), SourceBuildGateBranchV1::Else);
    assert_eq!(gate_path[1].branch(), SourceBuildGateBranchV1::Then);
}

#[test]
fn foreign_parser_path_rejects_before_publication() {
    let owner = ParserInvocationBrandV1::issue();
    let foreign = ParserInvocationBrandV1::issue();
    let session = ParserCallableSourceSessionV1::open(owner);
    let error = session
        .prepare_direct(
            SourceProgramCallablePathV1::top_level(SourceBoxDeclarationPathV1::root(foreign, 0)),
            DirectCallableDeclarationKindV1::FreeFunction,
            DirectCallableCommitPlacementV1::TopLevel,
            "same",
        )
        .unwrap_err();
    assert_eq!(error, DirectCallableSourceIssueV1::ForeignParser);
}

#[test]
fn duplicate_path_rejects_without_cloning_anchor_authority() {
    let brand = ParserInvocationBrandV1::issue();
    let mut session = ParserCallableSourceSessionV1::open(brand.clone());
    let path = || {
        SourceProgramCallablePathV1::top_level(SourceBoxDeclarationPathV1::root(brand.clone(), 0))
    };
    let first = session
        .prepare_direct(
            path(),
            DirectCallableDeclarationKindV1::FreeFunction,
            DirectCallableCommitPlacementV1::TopLevel,
            "same",
        )
        .unwrap();
    session.commit_direct(first).unwrap();

    let second_anchor_same_path = session
        .prepare_direct(
            path(),
            DirectCallableDeclarationKindV1::FreeFunction,
            DirectCallableCommitPlacementV1::TopLevel,
            "same",
        )
        .unwrap();
    assert_eq!(
        session.commit_direct(second_anchor_same_path).unwrap_err(),
        DirectCallableSourceIssueV1::DuplicatePath
    );
    assert_eq!(session.rows().len(), 1);
}

#[test]
fn equal_diagnostics_and_coordinates_in_foreign_sessions_never_recreate_anchor() {
    // Both parses have equal spelling, spans, arity, numeric coordinates, and
    // AST shape. None of those descriptive facts recreate the anchor.
    let left_parser = parse_rows("function identical(value: i64) {}\n");
    let right_parser = parse_rows("function identical(value: i64) {}\n");
    let left = &left_parser.callable_source_session.as_ref().unwrap().rows()[0];
    let right = &right_parser
        .callable_source_session
        .as_ref()
        .unwrap()
        .rows()[0];

    assert_eq!(left.diagnostic_name(), right.diagnostic_name());
    assert!(!left.anchor().same_as(right.anchor()));
}
