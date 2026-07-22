use super::canonical_physical_completion::CanonicalPhysicalCompleteInvocationV1;
use super::source_bound_package::ExactCanonicalPreflightPlanV1;
use super::{MirCompiler, VerifiedResolvedCallableProgramV1};
use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use crate::mir::module_invocation_identity::ModuleInvocationFamilyV1;

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
    let brand = package.brand();
    let complete = compiler
        .begin_canonical_invocation(package, Some("complete.hako"), "complete".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    match complete {
        CanonicalPhysicalCompleteInvocationV1::Single(product) => {
            assert_eq!(product.token.brand(), brand);
            assert_eq!(product.session.brand(), brand);
            assert_eq!(product.physical.brand(), brand);
            assert_eq!(product.physical.receipt_brand(), brand);
        }
        CanonicalPhysicalCompleteInvocationV1::Callable(_) => panic!("single route changed family"),
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
    let brand = package.brand();
    let complete = compiler
        .begin_canonical_invocation(package, Some("acyclic.hako"), "acyclic".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    match complete {
        CanonicalPhysicalCompleteInvocationV1::Callable(product) => {
            assert_eq!(product.token.brand(), brand);
            assert_eq!(product.session.brand(), brand);
            assert_eq!(product.physical.brand(), brand);
            assert_eq!(product.physical.receipt_brand(), brand);
            assert_eq!(product.capability.brand(), brand);
            assert_eq!(product.capability.family(), ModuleInvocationFamilyV1::BindingSsaAcyclic);
        }
        CanonicalPhysicalCompleteInvocationV1::Single(_) => panic!("acyclic route became single"),
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
    let brand = package.brand();
    let complete = compiler
        .begin_canonical_invocation(package, Some("recursive.hako"), "recursive".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    match complete {
        CanonicalPhysicalCompleteInvocationV1::Callable(product) => {
            assert_eq!(product.token.brand(), brand);
            assert_eq!(product.session.brand(), brand);
            assert_eq!(product.physical.brand(), brand);
            assert_eq!(product.physical.receipt_brand(), brand);
            assert_eq!(product.capability.brand(), brand);
            assert_eq!(product.capability.family(), ModuleInvocationFamilyV1::BindingSsaRecursive);
        }
        CanonicalPhysicalCompleteInvocationV1::Single(_) => panic!("recursive route became single"),
    }
    assert!(compiler.builder.current_module.is_none());
}
