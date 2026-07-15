use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};

use super::*;
use crate::mir::compiler::VerifiedResolvedCallableProgramV1;

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

#[test]
fn inventory_accepts_self_and_cycle_edges_without_owning_graph_policy() {
    let self_call = program(vec![function("a", call("a")), function("b", variable())]);
    let self_inventory = VerifiedCallableGraphInventoryV1::verify(self_call.module()).unwrap();
    assert_eq!(self_inventory.call_sites().len(), 1);
    assert_eq!(self_inventory.unique_edges().len(), 1);

    let cycle = program(vec![function("a", call("b")), function("b", call("a"))]);
    let cycle_inventory = VerifiedCallableGraphInventoryV1::verify(cycle.module()).unwrap();
    assert_eq!(cycle_inventory.call_sites().len(), 2);
    assert_eq!(cycle_inventory.unique_edges().len(), 2);
}

#[test]
fn synthetic_inventory_rejects_foreign_target_and_duplicate_site() {
    let source = program(vec![function("a", call("b")), function("b", variable())]);
    let inventory = VerifiedCallableGraphInventoryV1::verify(source.module()).unwrap();
    let foreign = program(vec![function("foreign", variable())]);
    let foreign_key = foreign
        .module()
        .functions_by_key()
        .keys()
        .next()
        .unwrap()
        .clone();

    let mut foreign_sites = inventory.call_sites().to_vec();
    foreign_sites[0].target = foreign_key;
    assert!(matches!(
        seal_inventory(inventory.nodes().to_vec(), foreign_sites),
        Err(CallableGraphInventoryErrorV1::TargetOutsideNodeSet { .. })
    ));

    let mut duplicate_sites = inventory.call_sites().to_vec();
    duplicate_sites.push(duplicate_sites[0].clone());
    assert!(matches!(
        seal_inventory(inventory.nodes().to_vec(), duplicate_sites),
        Err(CallableGraphInventoryErrorV1::DuplicateGraphSite { .. })
    ));
}
