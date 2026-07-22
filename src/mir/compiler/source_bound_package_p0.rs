use super::source_bound_package::{ExactCanonicalPreflightPlanV1, LoweredCanonicalPlanV1};
use super::{MirCompiler, VerifiedResolvedCallableProgramV1, VerifiedResolvedSourceUnitV1};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn function(name: &str) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(literal(1))),
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

fn callable_function(name: &str, value: ASTNode) -> ASTNode {
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

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn call(name: &str, argument: ASTNode) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.into(),
        arguments: vec![argument],
        span: Span::unknown(),
    }
}

#[test]
fn canonical_source_binding_owner0_uses_one_physical_owner() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function("owner0")).unwrap();
    let plan = super::capability::CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    let exact = ExactCanonicalPreflightPlanV1::from_first_family(plan);
    let mut compiler = MirCompiler::new();
    let package = compiler.bind_canonical_source(exact).unwrap();
    let package_brand = package.brand();
    let active = compiler
        .begin_canonical_invocation(package, Some("owner0.hako"), "owner0".to_owned())
        .unwrap();
    assert_eq!(active.brand(), package_brand);
    let lowered = active.lower().unwrap();
    assert_eq!(lowered.brand(), package_brand);
    assert_eq!(lowered.session_brand(), package_brand);
    assert_eq!(lowered.physical_brand(), package_brand);
    assert!(matches!(
        lowered.lowered(),
        LoweredCanonicalPlanV1::Single { .. }
    ));
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn canonical_source_binding_collect0_retains_same_brand_and_receipt() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function("collect0")).unwrap();
    let plan = super::capability::CanonicalLoweringPreflightV1::verify(&unit).unwrap();
    let exact = ExactCanonicalPreflightPlanV1::from_first_family(plan);
    let mut compiler = MirCompiler::new();
    let package = compiler.bind_canonical_source(exact).unwrap();
    let package_brand = package.brand();
    let active = compiler
        .begin_canonical_invocation(package, Some("collect0.hako"), "collect0".to_owned())
        .unwrap();
    let lowered = active.lower().unwrap();
    let collected = lowered.collect().unwrap();
    assert_eq!(collected.brand(), package_brand);
    assert_eq!(collected.session_brand(), package_brand);
    assert_eq!(collected.physical_brand(), package_brand);
    assert_eq!(collected.receipt_brand(), package_brand);
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn canonical_source_binding_collect0_projects_callable_catalog_atomically() {
    let program = VerifiedResolvedCallableProgramV1::resolve(ASTNode::Program {
        statements: vec![
            callable_function("caller", call("callee", variable("x"))),
            callable_function("callee", variable("x")),
        ],
        span: Span::unknown(),
    })
    .unwrap();
    let plan = super::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1::verify(
        program.module(),
    )
    .unwrap();
    let exact = ExactCanonicalPreflightPlanV1::BindingSsaAcyclic(plan);
    let mut compiler = MirCompiler::new();
    let package = compiler.bind_canonical_source(exact).unwrap();
    let package_brand = package.brand();
    let active = compiler
        .begin_canonical_invocation(package, Some("batch0.hako"), "batch0".to_owned())
        .unwrap();
    let lowered = active.lower().unwrap();
    let collected = lowered.collect().unwrap();
    assert_eq!(collected.brand(), package_brand);
    assert_eq!(collected.session_brand(), package_brand);
    assert_eq!(collected.physical_brand(), package_brand);
    assert_eq!(collected.receipt_brand(), package_brand);
    assert!(compiler.builder.current_module.is_none());
}
