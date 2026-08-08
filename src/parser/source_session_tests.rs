use super::source_authority::SourceBoxPathSegmentV1;
use super::source_gate_ledger::SourceBuildGateScopeV1;
use super::NyashParser;
use crate::tokenizer::NyashTokenizer;

#[test]
fn parser_session_owns_fresh_brand_and_top_level_cursor() {
    let left = NyashParser::new(Vec::new());
    let right = NyashParser::new(Vec::new());

    assert_ne!(
        left.source_invocation_brand(),
        right.source_invocation_brand()
    );
    assert_eq!(left.next_source_statement_ordinal, 0);
    assert_eq!(left.active_source_statement_ordinal(), None);
}

#[test]
fn parser_session_advances_top_level_cursor_once_per_statement() {
    let mut tokenizer = NyashTokenizer::new("box First {}\nbox Second {}\n");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = NyashParser::new(tokens);

    parser.parse().unwrap();

    assert_eq!(parser.next_source_statement_ordinal, 2);
    assert_eq!(parser.active_source_statement_ordinal(), None);
}

#[test]
fn build_gate_path_distinguishes_multiple_box_children() {
    let mut tokenizer =
        NyashTokenizer::new("gate Build.test {\n  box First {}\n  box Second {}\n}\n");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = NyashParser::new(tokens);

    let ast = parser.parse_program().unwrap();
    assert!(matches!(ast, crate::ast::ASTNode::Program { .. }));
    assert_eq!(parser.prepared_source_seals.len(), 2);

    let paths: Vec<_> = parser
        .prepared_source_seals
        .iter()
        .map(|prepared| prepared.box_site().path().segments().to_vec())
        .collect();
    assert!(matches!(
        paths[0].as_slice(),
        [
            SourceBoxPathSegmentV1::RootStatement { ordinal: 0 },
            SourceBoxPathSegmentV1::BuildGate {
                gate_id,
                branch: super::source_authority::SourceBuildGateBranchV1::Then,
                child_ordinal: 0,
            }
        ] if gate_id.raw() == 0
    ));
    assert!(matches!(
        paths[1].as_slice(),
        [
            SourceBoxPathSegmentV1::RootStatement { ordinal: 0 },
            SourceBoxPathSegmentV1::BuildGate {
                gate_id,
                branch: super::source_authority::SourceBuildGateBranchV1::Then,
                child_ordinal: 1,
            }
        ] if gate_id.raw() == 0
    ));
}

#[test]
fn nested_build_gate_path_keeps_parent_child_coordinate() {
    let mut tokenizer =
        NyashTokenizer::new("gate Build.test {\n  gate Build.debug {\n    box Nested {}\n  }\n}\n");
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = NyashParser::new(tokens);

    parser.parse_program().unwrap();
    let path = parser.prepared_source_seals[0].box_site().path();
    assert!(matches!(
        path.segments(),
        [
            SourceBoxPathSegmentV1::RootStatement { ordinal: 0 },
            SourceBoxPathSegmentV1::BuildGate {
                gate_id: outer_gate,
                branch: super::source_authority::SourceBuildGateBranchV1::Then,
                child_ordinal: 0,
            },
            SourceBoxPathSegmentV1::BuildGate {
                gate_id: inner_gate,
                branch: super::source_authority::SourceBuildGateBranchV1::Then,
                child_ordinal: 0,
            }
        ] if outer_gate.raw() == 0 && inner_gate.raw() == 1
    ));
}

#[test]
fn b2_top_level_gate_ledger_is_source_preordered_and_scoped() {
    let mut tokenizer = NyashTokenizer::new(
        "gate Build.test {\n  gate Build.debug { box Nested {} }\n  box Direct {}\n}\n",
    );
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = NyashParser::new(tokens);

    parser.parse_program().unwrap();
    assert_eq!(parser.prepared_source_build_gate_records.len(), 2);
    assert!(parser
        .prepared_source_build_gate_records
        .iter()
        .all(|record| record.scope == SourceBuildGateScopeV1::TopLevelItem));
    assert_eq!(
        parser.prepared_source_build_gate_records[0].gate_id.raw(),
        0
    );
    assert_eq!(
        parser.prepared_source_build_gate_records[1].gate_id.raw(),
        1
    );
}

#[test]
fn b2_method_body_gate_does_not_enter_top_level_ledger() {
    let mut tokenizer = NyashTokenizer::new(
        "box Plain {\n  run() { gate Build.test { return 1 } else { return 2 } }\n}\n",
    );
    let tokens = tokenizer.tokenize().unwrap();
    let mut parser = NyashParser::new(tokens);

    parser.parse_program().unwrap();
    assert!(parser.prepared_source_build_gate_records.is_empty());
}
