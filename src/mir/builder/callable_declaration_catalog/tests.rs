use std::collections::HashMap;

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};

use super::*;

fn function(name: &str, params: &[(&str, Option<&str>)], result: Option<&str>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_string(),
        params: params.iter().map(|(name, _)| (*name).to_string()).collect(),
        param_decls: params
            .iter()
            .map(|(name, ty)| ParamDecl {
                name: (*name).to_string(),
                declared_type_name: ty.map(str::to_string),
            })
            .collect(),
        return_type_name: result.map(str::to_string),
        body: vec![ASTNode::Return {
            value: None,
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

fn static_box(owner: &str, functions: Vec<ASTNode>) -> ASTNode {
    let methods = functions
        .into_iter()
        .map(|function| {
            let ASTNode::FunctionDeclaration { name, .. } = &function else {
                unreachable!()
            };
            (name.clone(), function)
        })
        .collect::<HashMap<_, _>>();
    ASTNode::BoxDeclaration {
        name: owner.to_string(),
        fields: Vec::new(),
        field_decls: Vec::new(),
        public_fields: Vec::new(),
        private_fields: Vec::new(),
        methods,
        constructors: HashMap::new(),
        init_fields: Vec::new(),
        weak_fields: Vec::new(),
        delegates: Vec::new(),
        invariants: Vec::new(),
        transitions: Vec::new(),
        is_interface: false,
        is_record: false,
        extends: Vec::new(),
        implements: Vec::new(),
        type_parameters: Vec::new(),
        is_sync: false,
        is_static: true,
        static_init: None,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn program(statements: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements,
        span: Span::unknown(),
    }
}

#[test]
fn seals_complete_structured_rows_and_declared_return_spelling() {
    let source = program(vec![static_box(
        "Helpers",
        vec![
            function("seed", &[], Some("i64")),
            function("project", &[("value", None)], None),
        ],
    )]);
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&source).unwrap();

    assert_eq!(catalog.len(), 2);
    let seed = catalog.candidates("seed", 0).first().expect("seed key");
    assert_eq!(
        seed.namespace(),
        SameModuleCallableNamespaceV1::StaticBoxMethod
    );
    assert_eq!(seed.owner(), "Helpers");
    assert_eq!(seed.name(), "seed");
    assert_eq!(seed.arity(), 0);
    assert_eq!(seed.mir_symbol_projection(), "Helpers.seed/0");
    let seed = catalog.declaration(seed).unwrap();
    assert_eq!(seed.return_type_name(), Some("i64"));
    assert_eq!(seed.body().len(), 1);

    let project = catalog.candidates("project", 1).first().unwrap();
    let project = catalog.declaration(project).unwrap();
    assert_eq!(project.params(), &["value"]);
    assert_eq!(project.param_decls()[0].name, "value");
    assert_eq!(project.return_type_name(), None);
}

#[test]
fn declaration_reorder_preserves_normalized_key_inventory() {
    let left = program(vec![
        static_box("Beta", vec![function("run", &[("x", None)], None)]),
        static_box("Alpha", vec![function("run", &[("x", None)], None)]),
    ]);
    let right = program(vec![
        static_box("Alpha", vec![function("run", &[("x", None)], None)]),
        static_box("Beta", vec![function("run", &[("x", None)], None)]),
    ]);
    let normalized = |source: &ASTNode| {
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(source)
            .unwrap()
            .keys()
            .map(CanonicalSameModuleCallableKeyV1::mir_symbol_projection)
            .collect::<Vec<_>>()
    };

    assert_eq!(normalized(&left), normalized(&right));
    assert_eq!(normalized(&left), ["Alpha.run/1", "Beta.run/1"]);
}

#[test]
fn rejects_duplicate_owner_and_malformed_method_pairing() {
    let duplicate = program(vec![
        static_box("Helpers", vec![function("a", &[], None)]),
        static_box("Helpers", vec![function("b", &[], None)]),
    ]);
    assert_eq!(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&duplicate).unwrap_err(),
        SameModuleCallableDeclarationCatalogErrorV1::DuplicateStaticBoxOwner {
            owner: "Helpers".to_string(),
        }
    );

    let mut malformed = static_box("Helpers", vec![function("actual", &[], None)]);
    let ASTNode::BoxDeclaration { methods, .. } = &mut malformed else {
        unreachable!()
    };
    let function = methods.remove("actual").unwrap();
    methods.insert("map_name".to_string(), function);
    assert_eq!(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program(vec![malformed]))
            .unwrap_err(),
        SameModuleCallableDeclarationCatalogErrorV1::MethodNameMismatch {
            owner: "Helpers".to_string(),
            map_name: "map_name".to_string(),
            declaration_name: "actual".to_string(),
        }
    );
}

#[test]
fn rejects_parameter_declaration_drift_and_ignores_nonstatic_boxes() {
    let mut drift = function("run", &[("value", None)], None);
    let ASTNode::FunctionDeclaration { param_decls, .. } = &mut drift else {
        unreachable!()
    };
    param_decls[0].name = "other".to_string();
    let error =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program(vec![static_box(
            "Helpers",
            vec![drift],
        )]))
        .unwrap_err();
    assert!(matches!(
        error,
        SameModuleCallableDeclarationCatalogErrorV1::ParameterNameMismatch { index: 0, .. }
    ));

    let mut ordinary = static_box("Ordinary", vec![function("run", &[], None)]);
    let ASTNode::BoxDeclaration { is_static, .. } = &mut ordinary else {
        unreachable!()
    };
    *is_static = false;
    let catalog =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program(vec![ordinary]))
            .unwrap();
    assert!(catalog.is_empty());
}
