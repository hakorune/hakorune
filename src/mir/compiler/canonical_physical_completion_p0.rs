use super::canonical_physical_completion::CanonicalDrainedInvocationV1;
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompiler, VerifiedResolvedCallableProgramV1};
use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, ParamDecl, Span};

fn variable() -> ASTNode {
    ASTNode::Variable {
        name: "x".into(),
        span: Span::unknown(),
    }
}

fn call(name: &str) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.into(),
        arguments: vec![variable()],
        span: Span::unknown(),
    }
}

fn function(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: vec!["x".into()],
        param_decls: vec![ParamDecl {
            name: "x".into(),
            declared_type_name: Some("i64".into()),
        }],
        return_type_name: Some("i64".into()),
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

fn a_plus_source() -> super::VerifiedResolvedSourceUnitV1 {
    let variable = |name: &str| ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    };
    let literal = |value: i64| ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    };
    let add = |left: ASTNode, right: ASTNode| ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    };
    super::VerifiedResolvedSourceUnitV1::resolve_function(ASTNode::FunctionDeclaration {
        name: "a_plus".into(),
        params: vec!["arg".into()],
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["x".into()],
                initial_values: vec![Some(Box::new(add(variable("arg"), literal(1))))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(variable("x")),
                value: Box::new(add(variable("x"), literal(1))),
                span: Span::unknown(),
            },
            ASTNode::Outbox {
                variables: vec!["result".into()],
                initial_values: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(variable("x"))),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    })
    .unwrap()
}

#[test]
fn compiler_bridge_drains_a_plus_single_route() {
    let source = a_plus_source();
    let plan = match super::capability::CanonicalLoweringPreflightV1::verify(&source).unwrap() {
        super::capability::CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(plan) => plan,
        _ => panic!("A+ fixture changed preflight family"),
    };
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::APlus(plan))
        .unwrap();
    let complete = compiler
        .begin_canonical_invocation(package, Some("a_plus.hako"), "a_plus".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    let drained = complete.prepare_drain().unwrap().drain();
    let CanonicalDrainedInvocationV1::Single(product) = drained else {
        panic!("A+ route drained as callable")
    };
    assert_eq!(product.physical.module.functions.len(), 1);
    assert!(product.physical.module.functions.contains_key("a_plus/1"));
}

#[test]
fn compiler_bridge_completion_retains_single_physical_receipt() {
    let source = super::VerifiedResolvedSourceUnitV1::resolve_function(
        ASTNode::FunctionDeclaration {
            name: "single".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::Literal {
                    value: crate::ast::LiteralValue::Integer(1),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            }],
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        },
    )
    .unwrap();
    let plan = super::capability::CanonicalLoweringPreflightV1::verify(&source).unwrap();
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::from_first_family(plan))
        .unwrap();
    let complete = compiler
        .begin_canonical_invocation(package, Some("complete.hako"), "complete".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    let drained = complete.prepare_drain().unwrap().drain();
    match drained {
        CanonicalDrainedInvocationV1::Single(product) => {
            assert!(product.physical.module.functions.contains_key("single/0"));
            assert_eq!(product.physical.module.functions.len(), 1);
        }
        CanonicalDrainedInvocationV1::Callable(_) => panic!("single route drained as callable"),
    }
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn compiler_bridge_completion_retains_acyclic_capability_and_receipt() {
    let source = program(vec![
        function("caller", call("callee")),
        function("callee", variable()),
    ]);
    let plan = super::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1::verify(
        source.module(),
    )
    .unwrap();
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(plan))
        .unwrap();
    let complete = compiler
        .begin_canonical_invocation(package, Some("acyclic.hako"), "acyclic".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    let drained = complete.prepare_drain().unwrap().drain();
    match drained {
        CanonicalDrainedInvocationV1::Callable(product) => {
            assert_eq!(product.physical.module.functions.len(), 2);
            assert!(product.physical.module.functions.contains_key("caller/1"));
            assert!(product.physical.module.functions.contains_key("callee/1"));
        }
        CanonicalDrainedInvocationV1::Single(_) => panic!("acyclic route drained as single"),
    }
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn compiler_bridge_completion_retains_recursive_capability_and_receipt() {
    let source = program(vec![function("loop", call("loop"))]);
    let plan =
        super::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1::verify(
            source.module(),
        )
        .unwrap();
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::BindingSsaRecursive(plan))
        .unwrap();
    let complete = compiler
        .begin_canonical_invocation(package, Some("recursive.hako"), "recursive".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    let drained = complete.prepare_drain().unwrap().drain();
    match drained {
        CanonicalDrainedInvocationV1::Callable(product) => {
            assert_eq!(product.physical.module.functions.len(), 1);
            assert!(product.physical.module.functions.contains_key("loop/1"));
        }
        CanonicalDrainedInvocationV1::Single(_) => panic!("recursive route drained as single"),
    }
    assert!(compiler.builder.current_module.is_none());
}
