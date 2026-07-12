use super::*;
use serde_json::{json, Value};
use std::collections::BTreeSet;

fn int(value: i64) -> Value {
    json!({"type": "Int", "value": value})
}

fn expr_stmt(expr: Value) -> Value {
    json!({"type": "Expr", "expr": expr})
}

#[test]
fn witness_accepts_empty_body_without_nodes() {
    let view = read_program_v0_body(r#"{"version":0,"kind":"Program","body":[]}"#).unwrap();
    let snapshot = build_snapshot_from_validated_view_v0(&view).unwrap();
    assert_eq!(snapshot.schema_version(), 0);
    assert_eq!(snapshot.source_program_version(), 0);
    assert_eq!(snapshot.node_count(), 0);
    assert_eq!(snapshot.max_depth_observed(), 0);
}

#[test]
fn witness_makes_integer_wire_equivalence_exact() {
    let number = read_program_v0_body(
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Int","value":-7}}]}"#,
    )
    .unwrap();
    let decimal = read_program_v0_body(
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Int","value":"-7"}}]}"#,
    )
    .unwrap();
    assert_eq!(
        build_snapshot_from_validated_view_v0(&number).unwrap(),
        build_snapshot_from_validated_view_v0(&decimal).unwrap()
    );
}

#[test]
fn witness_covers_every_accepted_kind_role_and_operator() {
    let mut body = vec![
        json!({"type":"Local","name":"x","expr":int(1)}),
        json!({"type":"If","cond":{"type":"Bool","value":true},"then":[{"type":"Break"}],"else":[{"type":"Continue"}]}),
        json!({"type":"Loop","cond":{"type":"Var","name":"keep"},"body":[{"type":"Return","expr":{"type":"Null"}}]}),
        json!({"type":"LoopRange","var_name":"i","start":int(0),"end":int(2),"body":[]}),
        expr_stmt(json!({"type":"Str","value":"猫😸"})),
        expr_stmt(json!({"type":"Call","name":"f","args":[int(1)]})),
        expr_stmt(
            json!({"type":"Method","recv":{"type":"Var","name":"x"},"method":"m","args":[{"type":"Bool","value":false}]}),
        ),
        expr_stmt(json!({"type":"Field","recv":{"type":"Var","name":"x"},"field":"value"})),
    ];
    for operator in BinaryOperatorV0::ALL.map(BinaryOperatorV0::wire_text) {
        body.push(expr_stmt(
            json!({"type":"Binary","op":operator,"lhs":int(1),"rhs":int(2)}),
        ));
    }
    for operator in CompareOperatorV0::ALL.map(CompareOperatorV0::wire_text) {
        body.push(expr_stmt(
            json!({"type":"Compare","op":operator,"lhs":int(1),"rhs":int(2)}),
        ));
    }
    for operator in LogicalOperatorV0::ALL.map(LogicalOperatorV0::wire_text) {
        body.push(expr_stmt(json!({"type":"Logical","op":operator,"lhs":{"type":"Bool","value":true},"rhs":{"type":"Bool","value":false}})));
    }
    let input = json!({"version":0,"kind":"Program","body":body}).to_string();
    let view = read_program_v0_body(&input).unwrap();
    let snapshot = build_snapshot_from_validated_view_v0(&view).unwrap();

    let kinds: BTreeSet<_> = snapshot.nodes().iter().map(SnapshotNodeV0::kind).collect();
    for kind in WireStmtKindV0::ALL {
        assert!(
            kinds.contains(&WireNodeKindV0::Stmt(kind)),
            "missing {kind:?}"
        );
    }
    for kind in WireExprKindV0::ALL {
        assert!(
            kinds.contains(&WireNodeKindV0::Expr(kind)),
            "missing {kind:?}"
        );
    }
    let roles: BTreeSet<_> = snapshot
        .nodes()
        .iter()
        .flat_map(|node| node.children().iter().map(|(role, _)| *role))
        .collect();
    assert_eq!(
        roles,
        BTreeSet::from([
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
        ])
    );
    assert!(snapshot.nodes().iter().any(|node| node
        .atoms()
        .contains(&(AtomKeyV0::Value, AtomValueV0::Text("猫😸".into())))));
}

#[test]
fn every_schema_limit_is_inclusive_at_minus_one_limit_and_plus_one() {
    let limits = SnapshotLimitsV0::SCHEMA;

    for depth in [limits.max_depth - 1, limits.max_depth] {
        let mut budget = BoundedBodyBudgetV0::default();
        assert_eq!(budget.observe_node(depth), Ok(()));
    }
    let mut depth = BoundedBodyBudgetV0::default();
    assert_eq!(
        depth.observe_node(limits.max_depth + 1),
        Err(BudgetLimitV0::Depth)
    );

    let mut nodes = BoundedBodyBudgetV0::default();
    for _ in 0..limits.max_node_count - 1 {
        nodes.observe_node(1).unwrap();
    }
    assert_eq!(nodes.node_count(), limits.max_node_count - 1);
    nodes.observe_node(1).unwrap();
    assert_eq!(nodes.node_count(), limits.max_node_count);
    assert_eq!(nodes.observe_node(1), Err(BudgetLimitV0::NodeCount));

    let budget = BoundedBodyBudgetV0::default();
    for count in [
        limits.max_children_per_body - 1,
        limits.max_children_per_body,
    ] {
        assert_eq!(budget.observe_body_children(count), Ok(()));
    }
    assert_eq!(
        budget.observe_body_children(limits.max_children_per_body + 1),
        Err(BudgetLimitV0::ChildrenPerBody)
    );
    for count in [limits.max_arguments - 1, limits.max_arguments] {
        assert_eq!(budget.observe_arguments(count), Ok(()));
    }
    assert_eq!(
        budget.observe_arguments(limits.max_arguments + 1),
        Err(BudgetLimitV0::Arguments)
    );

    for bytes in [limits.max_literal_bytes - 1, limits.max_literal_bytes] {
        let mut budget = BoundedBodyBudgetV0::default();
        assert_eq!(budget.observe_literal(&"x".repeat(bytes)), Ok(()));
    }
    let mut literal = BoundedBodyBudgetV0::default();
    assert_eq!(
        literal.observe_literal(&"x".repeat(limits.max_literal_bytes + 1)),
        Err(BudgetLimitV0::LiteralBytes)
    );
    for bytes in [limits.max_atom_bytes - 1, limits.max_atom_bytes] {
        let mut budget = BoundedBodyBudgetV0::default();
        assert_eq!(budget.observe_atom(&"x".repeat(bytes)), Ok(()));
    }
    let mut atom = BoundedBodyBudgetV0::default();
    assert_eq!(
        atom.observe_atom(&"x".repeat(limits.max_atom_bytes + 1)),
        Err(BudgetLimitV0::AtomBytes)
    );

    for total in [limits.max_total_text_bytes - 1, limits.max_total_text_bytes] {
        let mut budget = BoundedBodyBudgetV0::default();
        let chunk = "x".repeat(limits.max_literal_bytes);
        let full_chunks = total / limits.max_literal_bytes;
        for _ in 0..full_chunks {
            budget.observe_literal(&chunk).unwrap();
        }
        budget
            .observe_literal(&"x".repeat(total % limits.max_literal_bytes))
            .unwrap();
    }
    let mut total = BoundedBodyBudgetV0::default();
    let chunk = "x".repeat(limits.max_literal_bytes);
    for _ in 0..limits.max_total_text_bytes / limits.max_literal_bytes {
        total.observe_literal(&chunk).unwrap();
    }
    assert_eq!(
        total.observe_literal("x"),
        Err(BudgetLimitV0::TotalTextBytes)
    );
}
