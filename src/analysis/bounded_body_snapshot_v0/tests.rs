use super::*;

mod decoded_utf8_byte_len;
mod strict_json_tree;
mod witness;

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
        .field(PathFieldV0::Then)
        .index(1)
        .field(PathFieldV0::Expr)
        .field(PathFieldV0::Args)
        .index(0);
    assert_eq!(path.to_string(), "$.body[2].then[1].expr.args[0]");
}

#[test]
fn snapshot_equality_is_exact_and_node_count_is_derived() {
    let mut builder = SnapshotBuilderV0::new(0);
    let root = builder
        .reserve_node(
            PathV0::root_body().index(0),
            WireNodeKindV0::Stmt(WireStmtKindV0::Break),
            1,
        )
        .unwrap();
    builder.seal_node(root, vec![], vec![]).unwrap();
    builder.add_root(root).unwrap();
    let snapshot = builder.finish().unwrap();
    assert_eq!(snapshot.node_count(), 1);
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

#[test]
fn canonical_atom_schema_is_exhaustive_and_ordered() {
    use AtomKeyV0 as Key;
    use AtomValueKindV0 as Value;
    use TextClassV0 as Text;

    let expected = [
        (
            WireNodeKindV0::Stmt(WireStmtKindV0::Local),
            Some((Key::Name, Value::Text, Some(Text::Atom))),
        ),
        (WireNodeKindV0::Stmt(WireStmtKindV0::Expr), None),
        (WireNodeKindV0::Stmt(WireStmtKindV0::If), None),
        (WireNodeKindV0::Stmt(WireStmtKindV0::Loop), None),
        (
            WireNodeKindV0::Stmt(WireStmtKindV0::LoopRange),
            Some((Key::VarName, Value::Text, Some(Text::Atom))),
        ),
        (WireNodeKindV0::Stmt(WireStmtKindV0::Return), None),
        (WireNodeKindV0::Stmt(WireStmtKindV0::Break), None),
        (WireNodeKindV0::Stmt(WireStmtKindV0::Continue), None),
        (
            WireNodeKindV0::Expr(WireExprKindV0::Int),
            Some((Key::Value, Value::I64, None)),
        ),
        (
            WireNodeKindV0::Expr(WireExprKindV0::Str),
            Some((Key::Value, Value::Text, Some(Text::Literal))),
        ),
        (
            WireNodeKindV0::Expr(WireExprKindV0::Bool),
            Some((Key::Value, Value::Bool, None)),
        ),
        (
            WireNodeKindV0::Expr(WireExprKindV0::Null),
            Some((Key::Value, Value::Null, None)),
        ),
        (
            WireNodeKindV0::Expr(WireExprKindV0::Var),
            Some((Key::Name, Value::Text, Some(Text::Atom))),
        ),
        (
            WireNodeKindV0::Expr(WireExprKindV0::Binary),
            Some((Key::Op, Value::Text, Some(Text::Atom))),
        ),
        (
            WireNodeKindV0::Expr(WireExprKindV0::Compare),
            Some((Key::Op, Value::Text, Some(Text::Atom))),
        ),
        (
            WireNodeKindV0::Expr(WireExprKindV0::Logical),
            Some((Key::Op, Value::Text, Some(Text::Atom))),
        ),
        (
            WireNodeKindV0::Expr(WireExprKindV0::Call),
            Some((Key::Name, Value::Text, Some(Text::Atom))),
        ),
        (
            WireNodeKindV0::Expr(WireExprKindV0::Method),
            Some((Key::Method, Value::Text, Some(Text::Atom))),
        ),
        (
            WireNodeKindV0::Expr(WireExprKindV0::Field),
            Some((Key::Field, Value::Text, Some(Text::Atom))),
        ),
    ];
    assert_eq!(
        expected.len(),
        WireStmtKindV0::ALL.len() + WireExprKindV0::ALL.len()
    );
    for (kind, atom) in expected {
        let schema = kind.atom_schema();
        match atom {
            None => assert!(schema.is_empty(), "unexpected atom schema for {kind:?}"),
            Some((key, value_kind, text_class)) => assert_eq!(
                schema,
                &[AtomSpecV0 {
                    key,
                    value_kind,
                    text_class
                }],
                "atom schema drift for {kind:?}"
            ),
        }
    }
}

#[test]
fn canonical_child_schema_uses_vector_position_as_ordinal() {
    use ChildCardinalityV0 as Card;
    use ChildRoleV0 as Role;

    let specs = |kind: WireNodeKindV0| {
        kind.child_schema()
            .iter()
            .map(|spec| (spec.role, spec.cardinality))
            .collect::<Vec<_>>()
    };
    for kind in [
        WireStmtKindV0::Local,
        WireStmtKindV0::Expr,
        WireStmtKindV0::Return,
    ] {
        assert_eq!(
            specs(WireNodeKindV0::Stmt(kind)),
            vec![(Role::Expr, Card::One)]
        );
    }
    assert_eq!(
        specs(WireNodeKindV0::Stmt(WireStmtKindV0::If)),
        vec![
            (Role::Cond, Card::One),
            (Role::Then, Card::List),
            (Role::Else, Card::OptionalList)
        ]
    );
    assert_eq!(
        specs(WireNodeKindV0::Stmt(WireStmtKindV0::Loop)),
        vec![(Role::Cond, Card::One), (Role::Body, Card::List)]
    );
    assert_eq!(
        specs(WireNodeKindV0::Stmt(WireStmtKindV0::LoopRange)),
        vec![
            (Role::Start, Card::One),
            (Role::End, Card::One),
            (Role::Body, Card::List)
        ]
    );
    for kind in [
        WireExprKindV0::Binary,
        WireExprKindV0::Compare,
        WireExprKindV0::Logical,
    ] {
        assert_eq!(
            specs(WireNodeKindV0::Expr(kind)),
            vec![(Role::Lhs, Card::One), (Role::Rhs, Card::One)]
        );
    }
    assert_eq!(
        specs(WireNodeKindV0::Expr(WireExprKindV0::Call)),
        vec![(Role::Args, Card::List)]
    );
    assert_eq!(
        specs(WireNodeKindV0::Expr(WireExprKindV0::Method)),
        vec![(Role::Recv, Card::One), (Role::Args, Card::List)]
    );
    assert_eq!(
        specs(WireNodeKindV0::Expr(WireExprKindV0::Field)),
        vec![(Role::Recv, Card::One)]
    );
    for kind in [WireStmtKindV0::Break, WireStmtKindV0::Continue] {
        assert!(WireNodeKindV0::Stmt(kind).child_schema().is_empty());
    }
    for kind in [
        WireExprKindV0::Int,
        WireExprKindV0::Str,
        WireExprKindV0::Bool,
        WireExprKindV0::Null,
        WireExprKindV0::Var,
    ] {
        assert!(WireNodeKindV0::Expr(kind).child_schema().is_empty());
    }
}

#[test]
fn operator_wire_encodings_are_closed() {
    assert_eq!(
        BinaryOperatorV0::ALL.map(BinaryOperatorV0::wire_text),
        ["+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>"]
    );
    assert_eq!(
        CompareOperatorV0::ALL.map(CompareOperatorV0::wire_text),
        ["==", "!=", "<", ">", "<=", ">="]
    );
    assert_eq!(
        LogicalOperatorV0::ALL.map(LogicalOperatorV0::wire_text),
        ["&&", "||"]
    );
}

#[test]
fn path_fields_and_depth_convention_are_closed() {
    assert_eq!(DepthConventionV0::ROOT_BODY_CONTAINER, 0);
    assert_eq!(DepthConventionV0::TOP_LEVEL_NODE, 1);
    assert_eq!(
        PathFieldV0::ALL.map(PathFieldV0::wire_text),
        [
            "body", "type", "expr", "cond", "then", "else", "start", "end", "lhs", "rhs", "recv",
            "args", "name", "method", "field", "var_name", "op", "value"
        ]
    );
    for role in [
        ChildRoleV0::Expr,
        ChildRoleV0::Cond,
        ChildRoleV0::Then,
        ChildRoleV0::Else,
        ChildRoleV0::Body,
        ChildRoleV0::Start,
        ChildRoleV0::End,
        ChildRoleV0::Lhs,
        ChildRoleV0::Rhs,
        ChildRoleV0::Recv,
        ChildRoleV0::Args,
    ] {
        assert_eq!(role.path_field().wire_text(), role.wire_text());
    }
}

#[test]
fn validated_view_normalizes_integer_wire_equivalence() {
    let numeric = read_program_v0_body(
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Int","value":-7}}]}"#,
    )
    .expect("numeric Int view");
    let decimal = read_program_v0_body(
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Int","value":"-7"}}]}"#,
    )
    .expect("decimal Int view");

    for view in [&numeric, &decimal] {
        assert_eq!(view.source_program_version(), 0);
        let statement = view.body_node(0).expect("statement");
        let expression = statement.children()[0].1;
        assert_eq!(
            expression.atoms(),
            vec![(AtomKeyV0::Value, ValidatedAtomValueV0::I64(-7))]
        );
    }
}

#[test]
fn validated_text_bundles_value_utf8_bytes_and_class() {
    let view = read_program_v0_body(
        r#"{"version":0,"kind":"Program","body":[{"type":"Local","name":"猫x","expr":{"type":"Str","value":"猫😸"}}]}"#,
    )
    .expect("multibyte text view");
    let local = view.body_node(0).expect("Local");
    assert_eq!(
        local.atoms(),
        vec![(
            AtomKeyV0::Name,
            ValidatedAtomValueV0::Text(ValidatedTextV0::from_decoded("猫x", TextClassV0::Atom,))
        )]
    );
    let string = local.children()[0].1;
    assert_eq!(
        string.atoms(),
        vec![(
            AtomKeyV0::Value,
            ValidatedAtomValueV0::Text(
                ValidatedTextV0::from_decoded("猫😸", TextClassV0::Literal,)
            )
        )]
    );
}

#[test]
fn validated_children_follow_canonical_schema_order() {
    let view = read_program_v0_body(
        r#"{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Bool","value":true},"then":[{"type":"Break"},{"type":"Continue"}],"else":[{"type":"Expr","expr":{"type":"Method","recv":{"type":"Var","name":"x"},"method":"m","args":[{"type":"Int","value":1},{"type":"Null"}]}}]}]}"#,
    )
    .expect("ordered child view");
    let root = view.body_node(0).expect("If");
    let children = root.children();
    assert_eq!(
        children.iter().map(|(role, _)| *role).collect::<Vec<_>>(),
        vec![
            ChildRoleV0::Cond,
            ChildRoleV0::Then,
            ChildRoleV0::Then,
            ChildRoleV0::Else,
        ]
    );
    let expr_stmt = children[3].1;
    let method = expr_stmt.children()[0].1;
    assert_eq!(
        method
            .children()
            .iter()
            .map(|(role, _)| *role)
            .collect::<Vec<_>>(),
        vec![ChildRoleV0::Recv, ChildRoleV0::Args, ChildRoleV0::Args]
    );
}

#[test]
fn validated_node_handles_borrow_the_view() {
    fn first_kind(view: &ValidatedProgramV0BodyView) -> WireNodeKindV0 {
        view.body_node(0).expect("first node").kind()
    }
    let view = read_program_v0_body(r#"{"version":0,"kind":"Program","body":[{"type":"Break"}]}"#)
        .expect("view");
    assert_eq!(
        first_kind(&view),
        WireNodeKindV0::Stmt(WireStmtKindV0::Break)
    );
}

#[test]
fn snapshot_builder_publishes_only_complete_preorder_tables() {
    let mut builder = SnapshotBuilderV0::new(0);
    let root = builder
        .reserve_node(
            PathV0::root_body().index(0),
            WireNodeKindV0::Stmt(WireStmtKindV0::If),
            1,
        )
        .unwrap();
    let cond = builder
        .reserve_node(
            PathV0::root_body().index(0).field(PathFieldV0::Cond),
            WireNodeKindV0::Expr(WireExprKindV0::Bool),
            2,
        )
        .unwrap();
    let then = builder
        .reserve_node(
            PathV0::root_body()
                .index(0)
                .field(PathFieldV0::Then)
                .index(0),
            WireNodeKindV0::Stmt(WireStmtKindV0::Break),
            2,
        )
        .unwrap();
    builder
        .seal_node(
            cond,
            vec![(AtomKeyV0::Value, AtomValueV0::Bool(true))],
            vec![],
        )
        .unwrap();
    builder.seal_node(then, vec![], vec![]).unwrap();
    builder
        .seal_node(
            root,
            vec![],
            vec![(ChildRoleV0::Cond, cond), (ChildRoleV0::Then, then)],
        )
        .unwrap();
    builder.add_root(root).unwrap();
    let snapshot = builder.finish().unwrap();
    assert_eq!(snapshot.node_count(), 3);
    assert_eq!(snapshot.max_depth_observed(), 2);
    assert_eq!(
        snapshot.nodes()[0].kind(),
        WireNodeKindV0::Stmt(WireStmtKindV0::If)
    );
    assert_eq!(
        snapshot.nodes()[0].children(),
        &[(ChildRoleV0::Cond, 1), (ChildRoleV0::Then, 2)]
    );
}

#[test]
fn snapshot_builder_rejects_incomplete_and_double_sealed_drafts() {
    let mut incomplete = SnapshotBuilderV0::new(0);
    let root = incomplete
        .reserve_node(
            PathV0::root_body().index(0),
            WireNodeKindV0::Stmt(WireStmtKindV0::Break),
            1,
        )
        .unwrap();
    incomplete.add_root(root).unwrap();
    assert_eq!(
        incomplete.finish(),
        Err(SnapshotBuildErrorV0::IncompleteDraft)
    );

    let mut duplicate = SnapshotBuilderV0::new(0);
    let root = duplicate
        .reserve_node(
            PathV0::root_body().index(0),
            WireNodeKindV0::Stmt(WireStmtKindV0::Break),
            1,
        )
        .unwrap();
    duplicate.seal_node(root, vec![], vec![]).unwrap();
    assert_eq!(
        duplicate.seal_node(root, vec![], vec![]),
        Err(SnapshotBuildErrorV0::AlreadySealed)
    );
    assert_eq!(duplicate.finish(), Err(SnapshotBuildErrorV0::AlreadySealed));
}

#[test]
fn snapshot_builder_rejects_atom_and_child_schema_drift() {
    let mut atom = SnapshotBuilderV0::new(0);
    let root = atom
        .reserve_node(
            PathV0::root_body().index(0),
            WireNodeKindV0::Expr(WireExprKindV0::Int),
            1,
        )
        .unwrap();
    assert_eq!(
        atom.seal_node(
            root,
            vec![(AtomKeyV0::Value, AtomValueV0::Text("1".into()))],
            vec![]
        ),
        Err(SnapshotBuildErrorV0::AtomSchema)
    );

    let mut child = SnapshotBuilderV0::new(0);
    let root = child
        .reserve_node(
            PathV0::root_body().index(0),
            WireNodeKindV0::Stmt(WireStmtKindV0::Local),
            1,
        )
        .unwrap();
    assert_eq!(
        child.seal_node(
            root,
            vec![(AtomKeyV0::Name, AtomValueV0::Text("x".into()))],
            vec![]
        ),
        Err(SnapshotBuildErrorV0::ChildSchema)
    );
}

#[test]
fn snapshot_builder_rejects_bad_targets_paths_preorder_and_depth() {
    let mut target = SnapshotBuilderV0::new(0);
    let root = target
        .reserve_node(
            PathV0::root_body().index(0),
            WireNodeKindV0::Stmt(WireStmtKindV0::Local),
            1,
        )
        .unwrap();
    target
        .seal_node(
            root,
            vec![(AtomKeyV0::Name, AtomValueV0::Text("x".into()))],
            vec![(ChildRoleV0::Expr, root)],
        )
        .unwrap();
    target.add_root(root).unwrap();
    assert_eq!(target.finish(), Err(SnapshotBuildErrorV0::ChildTarget));

    let mut path = SnapshotBuilderV0::new(0);
    let root = path
        .reserve_node(
            PathV0::root_body().index(0),
            WireNodeKindV0::Stmt(WireStmtKindV0::Local),
            1,
        )
        .unwrap();
    let expr = path
        .reserve_node(
            PathV0::root_body().index(0).field(PathFieldV0::Cond),
            WireNodeKindV0::Expr(WireExprKindV0::Int),
            2,
        )
        .unwrap();
    path.seal_node(expr, vec![(AtomKeyV0::Value, AtomValueV0::I64(1))], vec![])
        .unwrap();
    path.seal_node(
        root,
        vec![(AtomKeyV0::Name, AtomValueV0::Text("x".into()))],
        vec![(ChildRoleV0::Expr, expr)],
    )
    .unwrap();
    path.add_root(root).unwrap();
    assert_eq!(path.finish(), Err(SnapshotBuildErrorV0::ChildPath));

    let mut preorder = SnapshotBuilderV0::new(0);
    let second = preorder
        .reserve_node(
            PathV0::root_body().index(1),
            WireNodeKindV0::Stmt(WireStmtKindV0::Break),
            1,
        )
        .unwrap();
    let first = preorder
        .reserve_node(
            PathV0::root_body().index(0),
            WireNodeKindV0::Stmt(WireStmtKindV0::Continue),
            1,
        )
        .unwrap();
    preorder.seal_node(second, vec![], vec![]).unwrap();
    preorder.seal_node(first, vec![], vec![]).unwrap();
    preorder.add_root(first).unwrap();
    preorder.add_root(second).unwrap();
    assert_eq!(preorder.finish(), Err(SnapshotBuildErrorV0::Preorder));

    let mut depth = SnapshotBuilderV0::new(0);
    let root = depth
        .reserve_node(
            PathV0::root_body().index(0),
            WireNodeKindV0::Stmt(WireStmtKindV0::Break),
            2,
        )
        .unwrap();
    depth.seal_node(root, vec![], vec![]).unwrap();
    depth.add_root(root).unwrap();
    assert_eq!(depth.finish(), Err(SnapshotBuildErrorV0::Depth));
}
