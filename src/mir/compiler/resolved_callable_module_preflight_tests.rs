use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};
use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, CanonicalCallableKeyV1, VerifiedCallableHeaderSourceUnitV1,
    VerifiedOwnerFreeCallableCatalogSourceUnitV1,
};

use super::capability::CanonicalFirstFamilyPlanV1;
use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use super::resolved_callable_module_preflight::{
    CallableModulePreflightErrorV1, VerifiedCallableModulePreflightV1,
};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.into()),
        span: Span::unknown(),
    }
}

fn function(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: vec!["n".into()],
        param_decls: vec![ParamDecl {
            name: "n".into(),
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

fn resolve(functions: Vec<ASTNode>) -> VerifiedResolvedCallableModuleV1 {
    let source = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(ASTNode::Program {
        statements: functions,
        span: Span::unknown(),
    })
    .unwrap();
    let owner_free = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source).unwrap();
    let catalog = CallableCatalogSealOutcomeV1::seal(owner_free, 29).unwrap();
    VerifiedResolvedCallableModuleV1::resolve(catalog).unwrap()
}

fn key(module: &VerifiedResolvedCallableModuleV1, name: &str) -> CanonicalCallableKeyV1 {
    module
        .functions_by_key()
        .keys()
        .find(|key| key.name() == name)
        .unwrap()
        .clone()
}

#[test]
fn seals_every_function_plan_before_publishing_the_module_preflight() {
    let module = resolve(vec![
        function("first", variable("n")),
        function("second", variable("n")),
    ]);
    let preflight = VerifiedCallableModulePreflightV1::verify(&module).unwrap();

    assert!(std::ptr::eq(preflight.module(), &module));
    assert_eq!(preflight.plans_by_key().len(), 2);
    for name in ["first", "second"] {
        assert!(matches!(
            preflight.plans_by_key().get(&key(&module, name)).unwrap(),
            CanonicalFirstFamilyPlanV1::TrivialBindingSsa(_)
        ));
    }
}

#[test]
fn declaration_order_does_not_change_the_canonical_preflight_key_set() {
    let mut observed = Vec::new();
    for functions in [
        vec![
            function("first", variable("n")),
            function("second", variable("n")),
        ],
        vec![
            function("second", variable("n")),
            function("first", variable("n")),
        ],
    ] {
        let module = resolve(functions);
        let preflight = VerifiedCallableModulePreflightV1::verify(&module).unwrap();
        observed.push(preflight.plans_by_key().keys().cloned().collect::<Vec<_>>());
    }
    assert_eq!(observed[0], observed[1]);
}

#[test]
fn one_late_function_failure_publishes_no_partial_preflight_product() {
    let module = resolve(vec![
        function("first", variable("n")),
        function("second", string("not i64")),
    ]);
    let expected = key(&module, "second");
    let error = VerifiedCallableModulePreflightV1::verify(&module).unwrap_err();

    assert!(matches!(
        error,
        CallableModulePreflightErrorV1::Function { key: failed, .. }
            if failed == expected
    ));
}
