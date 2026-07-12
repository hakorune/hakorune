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

#[test]
fn strict_program_v0_body_accepts_nested_closed_subset() {
    let input = r#"{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Bool","value":true},"then":[{"type":"Return","expr":{"type":"Int","value":"-1"}}],"else":null}]}"#;
    let view = read_program_v0_body(input).expect("accepted strict body");
    assert_eq!(view.body_len(), 1);
}

#[test]
fn strict_program_v0_body_rejects_duplicate_keys_and_trailing_input() {
    for input in [
        r#"{"version":0,"version":0,"kind":"Program","body":[]}"#,
        r#"{"version":0,"kind":"Program","body":[]} trailing"#,
    ] {
        assert!(matches!(
            read_program_v0_body(input),
            Err(ProgramV0BodyViewError::InvalidInput { .. })
        ));
    }
}

#[test]
fn strict_program_v0_body_rejects_unknown_fields_and_tags() {
    for input in [
        r#"{"version":0,"kind":"Program","body":[],"mystery":1}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"FutureStmt"}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"FutureExpr"}}]}"#,
    ] {
        assert!(matches!(
            read_program_v0_body(input),
            Err(ProgramV0BodyViewError::InvalidInput { .. })
        ));
    }
}

#[test]
fn strict_program_v0_body_preserves_known_unsupported_boundary() {
    let unsupported =
        read_program_v0_body(r#"{"version":0,"kind":"Program","body":[{"type":"Try"}]}"#);
    assert!(matches!(
        unsupported,
        Err(ProgramV0BodyViewError::Unsupported { node_kind, reason, .. })
            if node_kind == "Try" && reason == "unsupported.wire_kind"
    ));

    let seam = read_program_v0_body(
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Float","value":1.0}}]}"#,
    );
    assert!(matches!(
        seam,
        Err(ProgramV0BodyViewError::Unsupported { node_kind, reason, .. })
            if node_kind == "Float" && reason == "transport.schema_mismatch_stop"
    ));
}

#[test]
fn strict_program_v0_body_validates_required_children_and_scalars() {
    for input in [
        r#"{"version":1,"kind":"Program","body":[]}"#,
        r#"{"version":"0","kind":"Program","body":[]}"#,
        r#"{"version":0,"kind":"Other","body":[]}"#,
        r#"{"version":0,"kind":"Program","body":null}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Return","expr":null}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Int","value":"01"}}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Int","value":"9223372036854775808"}}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Binary","op":"==","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":2}}}]}"#,
    ] {
        assert!(
            matches!(
                read_program_v0_body(input),
                Err(ProgramV0BodyViewError::InvalidInput { .. })
            ),
            "input unexpectedly accepted: {input}"
        );
    }
}

#[test]
fn strict_json_duplicate_detection_uses_decoded_unicode_keys() {
    let input = r#"{"version":0,"kind":"Program","body":[],"\u0062ody":[]}"#;
    assert!(matches!(
        read_program_v0_body(input),
        Err(ProgramV0BodyViewError::InvalidInput { reason, .. }) if reason.contains("duplicate key")
    ));
}
