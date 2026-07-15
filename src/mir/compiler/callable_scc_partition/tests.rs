use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, ParamDecl, Span};

use super::*;
use crate::mir::compiler::{
    callable_graph_inventory::VerifiedCallableGraphInventoryV1, VerifiedResolvedCallableProgramV1,
};

fn variable() -> ASTNode {
    ASTNode::Variable {
        name: "x".to_string(),
        span: Span::unknown(),
    }
}

fn call(name: &str) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.to_string(),
        arguments: vec![variable()],
        span: Span::unknown(),
    }
}

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn function(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_string(),
        params: vec!["x".to_string()],
        param_decls: vec![ParamDecl {
            name: "x".to_string(),
            declared_type_name: Some("i64".to_string()),
        }],
        return_type_name: Some("i64".to_string()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(value)),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn program(functions: Vec<ASTNode>) -> VerifiedResolvedCallableProgramV1 {
    VerifiedResolvedCallableProgramV1::resolve(ASTNode::Program {
        statements: functions,
        span: Span::unknown(),
    })
    .unwrap()
}

fn partition(functions: Vec<ASTNode>) -> VerifiedCallableSccPartitionV1 {
    let source = program(functions);
    let inventory = VerifiedCallableGraphInventoryV1::verify(source.module()).unwrap();
    VerifiedCallableSccPartitionV1::verify(inventory).unwrap()
}

fn component_shape(
    partition: &VerifiedCallableSccPartitionV1,
) -> Vec<(String, Vec<String>, CallableSccRecursionKindV1)> {
    partition
        .components()
        .iter()
        .map(|component| {
            (
                component.id().anchor().name().to_string(),
                component
                    .members()
                    .iter()
                    .map(|member| member.name().to_string())
                    .collect(),
                component.recursion_kind(),
            )
        })
        .collect()
}

fn key(inventory: &VerifiedCallableGraphInventoryV1, name: &str) -> CanonicalCallableKeyV1 {
    inventory
        .nodes()
        .iter()
        .find(|key| key.name() == name)
        .unwrap()
        .clone()
}

fn draft(
    inventory: &VerifiedCallableGraphInventoryV1,
    id: &str,
    members: &[&str],
) -> CallableSccDraftV1 {
    CallableSccDraftV1 {
        id: CallableSccIdV1 {
            anchor: key(inventory, id),
        },
        members: members.iter().map(|name| key(inventory, name)).collect(),
    }
}

#[test]
fn classifies_nonrecursive_self_and_finite_mutual_components() {
    let acyclic = partition(vec![function("a", call("b")), function("b", variable())]);
    assert_eq!(acyclic.recursive_component_count(), 0);
    assert_eq!(acyclic.condensation_edges().len(), 1);

    let self_recursive = partition(vec![function("a", call("a")), function("b", variable())]);
    assert_eq!(self_recursive.recursive_component_count(), 1);
    assert_eq!(
        component_shape(&self_recursive),
        [
            (
                "a".to_string(),
                vec!["a".to_string()],
                CallableSccRecursionKindV1::SelfRecursive,
            ),
            (
                "b".to_string(),
                vec!["b".to_string()],
                CallableSccRecursionKindV1::NonRecursive,
            ),
        ]
    );

    let mutual = partition(vec![function("a", call("b")), function("b", call("a"))]);
    assert_eq!(mutual.recursive_component_count(), 1);
    assert_eq!(
        component_shape(&mutual),
        [(
            "a".to_string(),
            vec!["a".to_string(), "b".to_string()],
            CallableSccRecursionKindV1::MutualRecursive {
                contains_self_edge: false,
            },
        )]
    );

    let three = partition(vec![
        function("a", call("b")),
        function("b", call("c")),
        function("c", call("a")),
    ]);
    assert_eq!(three.components().len(), 1);
    assert_eq!(three.components()[0].id().anchor().name(), "a");
    assert_eq!(three.components()[0].members().len(), 3);
}

#[test]
fn mixed_graph_keeps_multiple_sccs_and_deduplicates_condensation_edges() {
    let partition = partition(vec![
        function("outer", add(call("a"), call("a"))),
        function("a", add(call("a"), call("b"))),
        function("b", add(call("a"), call("leaf"))),
        function("c", call("d")),
        function("d", call("c")),
        function("leaf", variable()),
    ]);

    assert_eq!(partition.inventory().call_sites().len(), 8);
    assert_eq!(partition.recursive_component_count(), 2);
    assert_eq!(partition.components().len(), 4);
    assert_eq!(partition.condensation_edges().len(), 2);
    assert_eq!(partition.condensation_order().len(), 4);
    let a = partition
        .components()
        .iter()
        .find(|component| component.id().anchor().name() == "a")
        .unwrap();
    assert_eq!(
        a.recursion_kind(),
        CallableSccRecursionKindV1::MutualRecursive {
            contains_self_edge: true,
        }
    );
}

#[test]
fn declaration_reorder_preserves_component_identity_and_condensation() {
    let mut observed = Vec::new();
    for functions in [
        vec![
            function("outer", call("a")),
            function("a", call("b")),
            function("b", add(call("a"), call("leaf"))),
            function("leaf", variable()),
        ],
        vec![
            function("leaf", variable()),
            function("b", add(call("a"), call("leaf"))),
            function("outer", call("a")),
            function("a", call("b")),
        ],
    ] {
        let partition = partition(functions);
        observed.push((
            component_shape(&partition),
            partition.condensation_edges().to_vec(),
            partition.condensation_order().to_vec(),
        ));
    }
    assert_eq!(observed[0], observed[1]);
}

#[test]
fn malformed_private_drafts_reject_identity_membership_and_scc_drift() {
    let source = program(vec![function("a", call("b")), function("b", variable())]);
    let inventory = VerifiedCallableGraphInventoryV1::verify(source.module()).unwrap();
    let empty = CallableSccDraftV1 {
        id: CallableSccIdV1 {
            anchor: key(&inventory, "a"),
        },
        members: Vec::new(),
    };
    assert!(matches!(
        seal_partition(inventory, vec![empty]),
        Err(CallableSccPartitionErrorV1::EmptyComponent)
    ));

    let source = program(vec![function("a", call("b")), function("b", variable())]);
    let inventory = VerifiedCallableGraphInventoryV1::verify(source.module()).unwrap();
    let drafts = vec![draft(&inventory, "b", &["a"])];
    assert!(matches!(
        seal_partition(inventory, drafts),
        Err(CallableSccPartitionErrorV1::IdMismatch { .. })
    ));

    let source = program(vec![function("a", call("b")), function("b", variable())]);
    let inventory = VerifiedCallableGraphInventoryV1::verify(source.module()).unwrap();
    let drafts = vec![draft(&inventory, "a", &["a"])];
    assert!(matches!(
        seal_partition(inventory, drafts),
        Err(CallableSccPartitionErrorV1::MissingMember(_))
    ));

    let source = program(vec![function("a", call("b")), function("b", variable())]);
    let inventory = VerifiedCallableGraphInventoryV1::verify(source.module()).unwrap();
    let drafts = vec![draft(&inventory, "a", &["a", "b"])];
    assert!(matches!(
        seal_partition(inventory, drafts),
        Err(CallableSccPartitionErrorV1::ComponentNotStronglyConnected(
            _
        ))
    ));

    let source = program(vec![function("a", call("b")), function("b", call("a"))]);
    let inventory = VerifiedCallableGraphInventoryV1::verify(source.module()).unwrap();
    let drafts = vec![
        draft(&inventory, "a", &["a"]),
        draft(&inventory, "b", &["b"]),
    ];
    assert!(matches!(
        seal_partition(inventory, drafts),
        Err(CallableSccPartitionErrorV1::CondensationCycle { .. })
    ));
}
