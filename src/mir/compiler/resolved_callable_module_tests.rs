use std::collections::BTreeMap;

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, CallableLookupErrorV1, CanonicalCallableKeyV1, FunctionOwnerIdV1,
    ResolveFunctionErrorV1, ResolveOwnerForestErrorV1, SourceCallableDeclarationSiteV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, VerifiedCallableCatalogSourceUnitV1,
    VerifiedCallableHeaderSourceUnitV1, VerifiedOwnerFreeCallableCatalogSourceUnitV1,
    VerifiedSemanticOwnerForestV1,
};

use super::resolved_callable_module::{
    ResolveCallableModuleErrorV1, VerifiedResolvedCallableModuleV1, VerifiedResolvedFunctionUnitV1,
};
use super::source_projection::VerifiedSourceProjectionV1;

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn call(name: &str) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.into(),
        arguments: vec![variable("n")],
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

fn resolve(
    functions: Vec<ASTNode>,
) -> Result<VerifiedResolvedCallableModuleV1, ResolveCallableModuleErrorV1> {
    let source = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(ASTNode::Program {
        statements: functions,
        span: Span::unknown(),
    })
    .unwrap();
    let owner_free = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source).unwrap();
    let catalog = CallableCatalogSealOutcomeV1::seal(owner_free, 17).unwrap();
    VerifiedResolvedCallableModuleV1::resolve(catalog)
}

fn return_call_site() -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]))
}

fn owner_for(module: &VerifiedResolvedCallableModuleV1, name: &str) -> FunctionOwnerIdV1 {
    module
        .source()
        .catalog()
        .index()
        .resolve_free_static_source_call(name, 1)
        .unwrap()
        .callable()
        .owner()
}

#[test]
fn passive_module_carrier_exposes_only_the_canonical_keyed_primary_map() {
    fn schema(
        module: &VerifiedResolvedCallableModuleV1,
    ) -> (
        &VerifiedCallableCatalogSourceUnitV1,
        &BTreeMap<CanonicalCallableKeyV1, VerifiedResolvedFunctionUnitV1>,
    ) {
        (module.source(), module.functions_by_key())
    }

    let _typed_schema = schema;
}

#[test]
fn passive_function_unit_keeps_site_forest_and_projection_together() {
    fn schema(
        unit: &VerifiedResolvedFunctionUnitV1,
    ) -> (
        SourceCallableDeclarationSiteV1,
        &VerifiedSemanticOwnerForestV1,
        &VerifiedSourceProjectionV1,
    ) {
        (unit.declaration_site(), unit.forest(), unit.projection())
    }

    let _typed_schema = schema;
}

#[test]
fn resolves_forward_and_self_calls_against_the_complete_catalog() {
    let module = resolve(vec![
        function("first", call("second")),
        function("second", call("second")),
    ])
    .unwrap();
    let first_owner = owner_for(&module, "first");
    let second_owner = owner_for(&module, "second");

    for (name, expected_target) in [("first", second_owner), ("second", second_owner)] {
        let header = module
            .source()
            .catalog()
            .index()
            .resolve_free_static_source_call(name, 1)
            .unwrap();
        let function = module.function(header.source_key()).unwrap();
        assert_eq!(function.forest().roots(), &[header.callable().owner()]);
        assert_eq!(
            function
                .forest()
                .owner(header.callable().owner())
                .unwrap()
                .direct_call_target(&return_call_site())
                .unwrap()
                .callable()
                .owner(),
            expected_target
        );
    }
    assert_ne!(first_owner, second_owner);
}

#[test]
fn declaration_order_does_not_change_backward_call_resolution() {
    for functions in [
        vec![
            function("first", call("second")),
            function("second", variable("n")),
        ],
        vec![
            function("second", variable("n")),
            function("first", call("second")),
        ],
    ] {
        let module = resolve(functions).unwrap();
        let second_owner = owner_for(&module, "second");
        let first_header = module
            .source()
            .catalog()
            .index()
            .resolve_free_static_source_call("first", 1)
            .unwrap();
        assert_eq!(
            module
                .function(first_header.source_key())
                .unwrap()
                .forest()
                .owner(first_header.callable().owner())
                .unwrap()
                .direct_call_target(&return_call_site())
                .unwrap()
                .callable()
                .owner(),
            second_owner
        );
    }
}

#[test]
fn lambda_owners_keep_the_catalog_compilation_brand() {
    let lambda = ASTNode::Lambda {
        params: Vec::new(),
        body: vec![variable("n")],
        span: Span::unknown(),
    };
    let module = resolve(vec![
        function("first", lambda),
        function("second", variable("n")),
    ])
    .unwrap();
    let first_header = module
        .source()
        .catalog()
        .index()
        .resolve_free_static_source_call("first", 1)
        .unwrap();
    let function = module.function(first_header.source_key()).unwrap();
    let root = function.forest().roots()[0];

    assert_eq!(function.forest().owner_count(), 2);
    for owner in function.forest().owners().map(|(owner, _)| owner) {
        assert_eq!(owner.compilation_brand(), root.compilation_brand());
    }
}

#[test]
fn unknown_target_rejects_before_a_resolved_module_is_published() {
    let error = resolve(vec![
        function("first", call("missing")),
        function("second", variable("n")),
    ])
    .unwrap_err();

    assert!(matches!(
        error,
        ResolveCallableModuleErrorV1::OwnerForest(
            _,
            ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::CallableLookup(
                CallableLookupErrorV1::MissingExactSourceKey
            ))
        )
    ));
}
