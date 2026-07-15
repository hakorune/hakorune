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

fn plan_counts(plan: &VerifiedRecursiveCallableModulePlanV1<'_>) -> Vec<(String, usize)> {
    plan.plans_by_key()
        .iter()
        .map(|(key, plan)| (key.name().to_string(), plan.direct_call_count()))
        .collect()
}

#[test]
fn seals_mutual_self_and_mixed_recursive_components_into_typed_plans() {
    let source = program(vec![
        function("root", call("a", variable())),
        function("a", add(call("a", variable()), call("b", variable()))),
        function("b", call("a", variable())),
        function("c", call("d", variable())),
        function("d", call("c", variable())),
        function("leaf", variable()),
    ]);
    let plan = VerifiedRecursiveCallableModulePlanV1::verify(source.module()).unwrap();
    assert!(std::ptr::eq(plan.module(), source.module()));
    assert_eq!(plan.partition().recursive_component_count(), 2);
    assert_eq!(plan.partition().inventory().call_sites().len(), 6);
    assert_eq!(plan.plans_by_key().len(), 6);
    assert_eq!(
        plan_counts(&plan),
        [
            ("a".to_string(), 2),
            ("b".to_string(), 1),
            ("c".to_string(), 1),
            ("d".to_string(), 1),
            ("leaf".to_string(), 0),
            ("root".to_string(), 1),
        ]
    );
}

#[test]
fn declaration_reorder_preserves_partition_and_typed_plan_keys() {
    let mut observed = Vec::new();
    for functions in [
        vec![
            function("root", call("a", variable())),
            function("a", call("b", variable())),
            function("b", add(call("a", variable()), call("leaf", variable()))),
            function("leaf", variable()),
        ],
        vec![
            function("leaf", variable()),
            function("b", add(call("a", variable()), call("leaf", variable()))),
            function("root", call("a", variable())),
            function("a", call("b", variable())),
        ],
    ] {
        let source = program(functions);
        let plan = VerifiedRecursiveCallableModulePlanV1::verify(source.module()).unwrap();
        observed.push((
            plan.partition()
                .components()
                .iter()
                .map(|component| {
                    (
                        component.id().anchor().clone(),
                        component.members().to_vec(),
                        component.recursion_kind(),
                    )
                })
                .collect::<Vec<_>>(),
            plan.partition().condensation_edges().to_vec(),
            plan.partition().condensation_order().to_vec(),
            plan.plans_by_key().keys().cloned().collect::<Vec<_>>(),
        ));
    }
    assert_eq!(observed[0], observed[1]);
}

#[test]
fn seals_singleton_finite_self_calls_through_the_canonical_plan() {
    for (value, expected_calls) in [
        (call("only", variable()), 1),
        (add(call("only", variable()), call("only", variable())), 2),
        (call("only", call("only", variable())), 2),
    ] {
        let source = program(vec![function("only", value)]);
        let plan = VerifiedRecursiveCallableModulePlanV1::verify(source.module()).unwrap();

        assert_eq!(plan.partition().inventory().nodes().len(), 1);
        assert_eq!(plan.partition().components().len(), 1);
        assert_eq!(plan.partition().recursive_component_count(), 1);
        assert_eq!(
            plan.partition().inventory().call_sites().len(),
            expected_calls
        );
        assert_eq!(plan.plans_by_key().len(), 1);
        assert_eq!(plan_counts(&plan), [("only".to_string(), expected_calls)]);
    }
}

#[test]
fn singleton_admission_rejects_zero_call_and_acyclic_modules() {
    let zero = program(vec![function("only", variable())]);
    assert!(matches!(
        VerifiedRecursiveCallableModulePlanV1::verify(zero.module()),
        Err(RecursiveCallableModulePlanErrorV1::DirectCallCardinality { actual: 0 })
    ));

    let acyclic = program(vec![
        function("a", call("b", variable())),
        function("b", variable()),
    ]);
    assert!(matches!(
        VerifiedRecursiveCallableModulePlanV1::verify(acyclic.module()),
        Err(RecursiveCallableModulePlanErrorV1::NoRecursiveComponent)
    ));
}

#[test]
fn rejects_zero_call_acyclic_and_nontrivial_profiles() {
    let zero = program(vec![function("a", variable()), function("b", variable())]);
    assert!(matches!(
        VerifiedRecursiveCallableModulePlanV1::verify(zero.module()),
        Err(RecursiveCallableModulePlanErrorV1::DirectCallCardinality { actual: 0 })
    ));

    let acyclic = program(vec![
        function("a", call("b", variable())),
        function("b", variable()),
    ]);
    assert!(matches!(
        VerifiedRecursiveCallableModulePlanV1::verify(acyclic.module()),
        Err(RecursiveCallableModulePlanErrorV1::NoRecursiveComponent)
    ));

    let invalid = program(vec![
        function("root", call("bad", variable())),
        function("bad", call("root", variable())),
        function(
            "unused",
            ASTNode::Literal {
                value: LiteralValue::String("not-i64".to_string()),
                span: Span::unknown(),
            },
        ),
    ]);
    assert!(matches!(
        VerifiedRecursiveCallableModulePlanV1::verify(invalid.module()),
        Err(RecursiveCallableModulePlanErrorV1::FunctionPreflight { .. })
    ));
}
