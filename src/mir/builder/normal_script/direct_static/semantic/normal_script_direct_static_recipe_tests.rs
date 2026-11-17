use super::*;
use crate::mir::resolved_semantics::{BodyShapeRelationV1, SourcePathSegmentV1, SourcePathV1};

fn terminal_sites() -> (
    crate::mir::resolved_semantics::SourceStmtSiteV1,
    crate::mir::resolved_semantics::SourceExprSiteV1,
) {
    let statement = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(0))
        .stmt();
    let call = SourcePathV1::from_node(statement.node())
        .child(SourcePathSegmentV1::Value)
        .expr();
    (statement, call)
}

#[test]
fn terminal_shape_accepts_bare_final_sequence_call() {
    let statement = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(0))
        .stmt();
    let call = SourcePathV1::from_node(statement.node()).expr();
    assert!(validate_terminal_relation(
        &ScriptSourceContinuationTerminalV1::Sequence(statement.clone()),
        &statement,
        &call,
        &[],
    )
    .is_ok());
}

#[test]
fn terminal_shape_rejects_final_local_value_as_sequence_result() {
    let (statement, call) = terminal_sites();
    let relation = BodyShapeRelationV1 {
        parent: statement.node().clone(),
        role: SourcePathSegmentV1::Value,
        child: call.clone(),
    };
    assert!(matches!(
        validate_terminal_relation(
            &ScriptSourceContinuationTerminalV1::Sequence(statement.clone()),
            &statement,
            &call,
            &[relation],
        ),
        Err(ScriptDirectStaticRecipeIssueV1::TerminalRelationMismatch(_))
    ));
}

#[test]
fn terminal_shape_accepts_direct_root_return_value() {
    let (statement, call) = terminal_sites();
    let relation = BodyShapeRelationV1 {
        parent: statement.node().clone(),
        role: SourcePathSegmentV1::Value,
        child: call.clone(),
    };
    assert!(validate_terminal_relation(
        &ScriptSourceContinuationTerminalV1::Return(statement.clone()),
        &statement,
        &call,
        &[relation],
    )
    .is_ok());
}

#[test]
fn terminal_shape_rejects_nested_return_call() {
    let (statement, direct_value) = terminal_sites();
    let call = SourcePathV1::from_node(direct_value.node())
        .child(SourcePathSegmentV1::Argument(0))
        .expr();
    let relation_to_wrapper = BodyShapeRelationV1 {
        parent: direct_value.node().clone(),
        role: SourcePathSegmentV1::Argument(0),
        child: call.clone(),
    };
    let relation_to_return = BodyShapeRelationV1 {
        parent: statement.node().clone(),
        role: SourcePathSegmentV1::Value,
        child: direct_value,
    };
    assert!(matches!(
        validate_terminal_relation(
            &ScriptSourceContinuationTerminalV1::Return(statement),
            &SourcePathV1::program_body()
                .child(SourcePathSegmentV1::ProgramBody(0))
                .stmt(),
            &call,
            &[relation_to_wrapper, relation_to_return],
        ),
        Err(ScriptDirectStaticRecipeIssueV1::DuplicateFinalValueRelation(_))
    ));
}
