use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};

use super::*;
use crate::mir::compiler::VerifiedResolvedCallableProgramV1;

fn variable() -> ASTNode {
    ASTNode::Variable {
        name: "x".to_string(),
        span: Span::unknown(),
    }
}

fn call(name: &str, argument: ASTNode) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.to_string(),
        arguments: vec![argument],
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

fn plan_counts(plan: &VerifiedAcyclicCallableModulePlanV1<'_>) -> Vec<(String, usize)> {
    plan.plans_by_key()
        .iter()
        .map(|(key, plan)| (key.name().to_string(), plan.direct_call_count()))
        .collect()
}

#[test]
fn seals_nested_finite_calls_into_a_typed_canonical_plan_map() {
    let source = program(vec![
        function("root", call("step", call("step", variable()))),
        function("step", variable()),
    ]);
    let plan = VerifiedAcyclicCallableModulePlanV1::verify(source.module()).unwrap();
    assert!(std::ptr::eq(plan.module(), source.module()));
    assert_eq!(plan.graph().call_sites().len(), 2);
    assert_eq!(plan.graph().unique_edges().len(), 1);
    assert_eq!(
        plan_counts(&plan),
        [("root".to_string(), 2), ("step".to_string(), 0)]
    );
}

#[test]
fn declaration_reorder_preserves_graph_and_typed_plan_keys() {
    let mut observed = Vec::new();
    for functions in [
        vec![
            function(
                "root",
                add(call("left", variable()), call("right", variable())),
            ),
            function("left", variable()),
            function("right", variable()),
        ],
        vec![
            function("right", variable()),
            function("left", variable()),
            function(
                "root",
                add(call("left", variable()), call("right", variable())),
            ),
        ],
    ] {
        let source = program(functions);
        let plan = VerifiedAcyclicCallableModulePlanV1::verify(source.module()).unwrap();
        observed.push((
            plan.graph().clone(),
            plan.plans_by_key().keys().cloned().collect::<Vec<_>>(),
        ));
    }
    assert_eq!(observed[0], observed[1]);
}

#[test]
fn rejects_one_function_zero_call_cycles_and_nontrivial_function_profiles() {
    let one = program(vec![function("only", variable())]);
    assert!(matches!(
        VerifiedAcyclicCallableModulePlanV1::verify(one.module()),
        Err(AcyclicCallableModulePlanErrorV1::FunctionCardinality { actual: 1 })
    ));

    let zero = program(vec![function("a", variable()), function("b", variable())]);
    assert!(matches!(
        VerifiedAcyclicCallableModulePlanV1::verify(zero.module()),
        Err(AcyclicCallableModulePlanErrorV1::DirectCallCardinality { actual: 0 })
    ));

    let cycle = program(vec![
        function("a", call("b", variable())),
        function("b", call("a", variable())),
    ]);
    assert!(matches!(
        VerifiedAcyclicCallableModulePlanV1::verify(cycle.module()),
        Err(AcyclicCallableModulePlanErrorV1::Graph(
            AcyclicCallableGraphErrorV1::Cycle { .. }
        ))
    ));

    let invalid = program(vec![
        function("root", call("bad", variable())),
        function(
            "bad",
            ASTNode::Literal {
                value: LiteralValue::String("not-i64".to_string()),
                span: Span::unknown(),
            },
        ),
    ]);
    assert!(matches!(
        VerifiedAcyclicCallableModulePlanV1::verify(invalid.module()),
        Err(AcyclicCallableModulePlanErrorV1::FunctionPreflight { .. })
    ));
}
