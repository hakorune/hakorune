use super::*;

#[test]
fn schema_limits_are_fixed_and_inclusive() {
    let limits = SnapshotLimitsV0::SCHEMA;
    assert_eq!(limits.max_depth, 64);
    assert_eq!(limits.max_node_count, 32_768);
    assert_eq!(limits.max_total_text_bytes, 4_194_304);

    let mut budget = BoundedBodyBudgetV0::default();
    assert_eq!(budget.observe_node(64), Ok(()));
    assert_eq!(budget.observe_node(65), Err(BudgetLimitV0::Depth));
    assert_eq!(budget.observe_body_children(2_048), Ok(()));
    assert_eq!(
        budget.observe_body_children(2_049),
        Err(BudgetLimitV0::ChildrenPerBody)
    );
    assert_eq!(budget.observe_arguments(128), Ok(()));
    assert_eq!(budget.observe_arguments(129), Err(BudgetLimitV0::Arguments));
}

#[test]
fn path_is_structural_and_zero_based() {
    let path = PathV0::root_body()
        .index(2)
        .field("then")
        .index(1)
        .field("expr")
        .field("args")
        .index(0);
    assert_eq!(path.to_string(), "$.body[2].then[1].expr.args[0]");
}

#[test]
fn snapshot_equality_is_exact_and_node_count_is_derived() {
    let node = SnapshotNodeV0 {
        path: PathV0::root_body().index(0),
        kind: WireNodeKindV0::Stmt(WireStmtKindV0::Break),
        atoms: vec![],
        children: vec![],
    };
    let snapshot = BoundedBodyAnalysisSnapshotV0::new(0, vec![node], 1);
    assert_eq!(snapshot.node_count, 1);
    assert_eq!(snapshot.clone(), snapshot);
}

#[test]
fn outcomes_preserve_unsupported_and_invalid_input() {
    let issue = AnalysisIssueV0 {
        path: PathV0::root_body().index(0),
        node_kind: Some("Float".to_string()),
        reason: "unsupported.wire_kind",
    };
    assert!(matches!(
        BoundedBodyAnalysisOutcomeV0::Unsupported(issue.clone()),
        BoundedBodyAnalysisOutcomeV0::Unsupported(observed) if observed == issue
    ));
    assert!(matches!(
        BoundedBodyAnalysisOutcomeV0::InvalidInput(issue),
        BoundedBodyAnalysisOutcomeV0::InvalidInput(_)
    ));
}
