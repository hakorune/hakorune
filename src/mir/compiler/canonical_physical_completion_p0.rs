use super::canonical_physical_completion::{
    CanonicalDrainPrepareErrorV1, CanonicalDrainedInvocationV1, CanonicalPhysicalCompletionErrorV1,
};
use super::source_bound_package::{
    CollectedCanonicalPhysicalInvocationV1, ExactCanonicalPreflightPlanV1,
};
use super::{MirCompiler, VerifiedResolvedCallableProgramV1};
use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};

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
    let source =
        super::VerifiedResolvedSourceUnitV1::resolve_function(ASTNode::FunctionDeclaration {
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
        })
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
            assert_eq!(product.physical.receipt_brand(), product.token.brand());
            assert_eq!(product.physical.family, product.token.family());
        }
        CanonicalDrainedInvocationV1::Callable(_) => panic!("single route drained as callable"),
    }
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn physical_drain_accepts_canonical_condition_fn_spelling() {
    let source =
        super::VerifiedResolvedSourceUnitV1::resolve_function(ASTNode::FunctionDeclaration {
            name: "condition_fn".into(),
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
        })
        .unwrap();
    let plan = super::capability::CanonicalLoweringPreflightV1::verify(&source).unwrap();
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::from_first_family(plan))
        .unwrap();
    let drained = compiler
        .begin_canonical_invocation(package, Some("condition_fn.hako"), "condition_fn".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap()
        .prepare_drain()
        .unwrap()
        .drain();
    let CanonicalDrainedInvocationV1::Single(product) = drained else {
        panic!("canonical condition_fn route drained as callable")
    };
    assert!(product
        .physical
        .module
        .functions
        .contains_key("condition_fn/0"));
}

#[test]
fn physical_prepare_failure_leaves_live_builder_unchanged() {
    let source =
        super::VerifiedResolvedSourceUnitV1::resolve_function(ASTNode::FunctionDeclaration {
            name: "prepare_failure".into(),
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
        })
        .unwrap();
    let plan = super::capability::CanonicalLoweringPreflightV1::verify(&source).unwrap();
    let mut compiler = MirCompiler::new();
    let package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::from_first_family(plan))
        .unwrap();
    let mut complete = compiler
        .begin_canonical_invocation(
            package,
            Some("prepare_failure.hako"),
            "prepare_failure".into(),
        )
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap()
        .complete()
        .unwrap();
    complete.publish_single_shell_for_test();
    let live_before = compiler.builder.current_module.is_none();
    let rejected = complete.prepare_drain().unwrap_err();
    assert!(matches!(
        rejected.error,
        CanonicalDrainPrepareErrorV1::Physical(
            crate::mir::builder::CanonicalPhysicalDrainPrepareErrorV1::PublishedShell { .. }
        )
    ));
    assert_eq!(compiler.builder.current_module.is_none(), live_before);
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
            assert_eq!(product.physical.receipt_brand(), product.token.brand());
            assert_eq!(product.capability.brand(), product.token.brand());
            assert_eq!(product.capability.family(), product.token.family());
        }
        CanonicalDrainedInvocationV1::Single(_) => panic!("recursive route drained as single"),
    }
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn capability_brand_drift_is_not_misreported_as_foreign_physical_brand() {
    let mut compiler = MirCompiler::new();
    let first = program(vec![
        function("caller", call("callee")),
        function("callee", call("callee")),
    ]);
    let first_plan =
        super::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1::verify(
            first.module(),
        )
        .unwrap();
    let first_package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::BindingSsaRecursive(first_plan))
        .unwrap();
    let first_collected = compiler
        .begin_canonical_invocation(first_package, Some("first.hako"), "first".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap();

    let second = program(vec![
        function("caller", call("callee")),
        function("callee", call("callee")),
    ]);
    let second_plan =
        super::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1::verify(
            second.module(),
        )
        .unwrap();
    let second_package = compiler
        .bind_canonical_source(ExactCanonicalPreflightPlanV1::BindingSsaRecursive(
            second_plan,
        ))
        .unwrap();
    let second_collected = compiler
        .begin_canonical_invocation(second_package, Some("second.hako"), "second".into())
        .unwrap()
        .lower()
        .unwrap()
        .collect()
        .unwrap();

    let CollectedCanonicalPhysicalInvocationV1::Callable {
        token: first_token,
        continuation: first_continuation,
        session: first_session,
        capability: _,
        physical: first_physical,
    } = first_collected
    else {
        panic!("first route changed family")
    };
    let CollectedCanonicalPhysicalInvocationV1::Callable {
        capability: second_capability,
        ..
    } = second_collected
    else {
        panic!("second route changed family")
    };
    let mixed = CollectedCanonicalPhysicalInvocationV1::Callable {
        token: first_token,
        continuation: first_continuation,
        session: first_session,
        capability: second_capability,
        physical: first_physical,
    };
    let rejected = mixed.complete().unwrap_err();
    assert!(matches!(
        rejected.error,
        CanonicalPhysicalCompletionErrorV1::CapabilityMismatch
    ));
}
