use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, ParamDecl, Span};

use super::*;
use crate::mir::compiler::VerifiedResolvedCallableProgramV1;

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn call(name: &str) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.to_string(),
        arguments: vec![variable("x")],
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

fn leaf(name: &str) -> ASTNode {
    function(name, variable("x"))
}

fn program(functions: Vec<ASTNode>) -> VerifiedResolvedCallableProgramV1 {
    VerifiedResolvedCallableProgramV1::resolve(ASTNode::Program {
        statements: functions,
        span: Span::unknown(),
    })
    .unwrap()
}

fn key_names(keys: &[CanonicalCallableKeyV1]) -> Vec<&str> {
    keys.iter().map(|key| key.name()).collect()
}

#[test]
fn repeated_sites_and_multiple_targets_keep_multiplicity_but_unique_edges() {
    let first = program(vec![
        function("root", add(add(call("step"), call("step")), call("leaf"))),
        leaf("step"),
        leaf("leaf"),
    ]);
    let reordered = program(vec![
        leaf("leaf"),
        function("root", add(add(call("step"), call("step")), call("leaf"))),
        leaf("step"),
    ]);
    let graph = VerifiedAcyclicCallableGraphV1::verify(first.module()).unwrap();
    let reordered_graph = VerifiedAcyclicCallableGraphV1::verify(reordered.module()).unwrap();

    assert_eq!(graph, reordered_graph);
    assert_eq!(graph.nodes().len(), 3);
    assert_eq!(graph.call_sites().len(), 3);
    assert_eq!(graph.unique_edges().len(), 2);
    assert_eq!(
        key_names(graph.topological_order()),
        ["root", "leaf", "step"]
    );
    assert_eq!(
        graph
            .call_sites()
            .iter()
            .filter(|site| site.target().name() == "step")
            .count(),
        2
    );
}

#[test]
fn multi_hop_chain_and_isolated_node_have_deterministic_topology() {
    let source = program(vec![
        function("a", call("b")),
        function("b", call("c")),
        leaf("c"),
        leaf("d"),
    ]);
    let graph = VerifiedAcyclicCallableGraphV1::verify(source.module()).unwrap();
    assert_eq!(graph.call_sites().len(), 2);
    assert_eq!(graph.unique_edges().len(), 2);
    assert_eq!(key_names(graph.topological_order()), ["a", "b", "c", "d"]);
}

#[test]
fn self_edge_and_recursive_cycles_reject_with_source_witnesses() {
    let self_call = program(vec![function("a", call("a")), leaf("b")]);
    assert!(matches!(
        VerifiedAcyclicCallableGraphV1::verify(self_call.module()),
        Err(AcyclicCallableGraphErrorV1::SelfEdge { .. })
    ));

    let two_cycle = program(vec![function("a", call("b")), function("b", call("a"))]);
    let Err(AcyclicCallableGraphErrorV1::Cycle {
        residual_nodes,
        witness_sites,
    }) = VerifiedAcyclicCallableGraphV1::verify(two_cycle.module())
    else {
        panic!("expected two-node cycle")
    };
    assert_eq!(key_names(&residual_nodes), ["a", "b"]);
    assert_eq!(witness_sites.len(), 2);

    let three_cycle = program(vec![
        function("a", call("b")),
        function("b", call("c")),
        function("c", call("a")),
    ]);
    let Err(AcyclicCallableGraphErrorV1::Cycle {
        residual_nodes,
        witness_sites,
    }) = VerifiedAcyclicCallableGraphV1::verify(three_cycle.module())
    else {
        panic!("expected three-node cycle")
    };
    assert_eq!(key_names(&residual_nodes), ["a", "b", "c"]);
    assert_eq!(witness_sites.len(), 3);
}
