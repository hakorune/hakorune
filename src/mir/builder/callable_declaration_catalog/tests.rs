use std::collections::HashMap;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, RuneAttr, Span};

use super::*;

fn function(
    name: &str,
    params: &[(&str, Option<&str>)],
    result: Option<&str>,
    is_static: bool,
) -> ASTNode {
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
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn static_function(name: &str, params: &[(&str, Option<&str>)], result: Option<&str>) -> ASTNode {
    function(name, params, result, true)
}

fn instance_function(name: &str, params: &[(&str, Option<&str>)], result: Option<&str>) -> ASTNode {
    function(name, params, result, false)
}

fn box_declaration(owner: &str, functions: Vec<ASTNode>, is_static: bool) -> ASTNode {
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
        is_static,
        static_init: None,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn static_box(owner: &str, functions: Vec<ASTNode>) -> ASTNode {
    box_declaration(owner, functions, true)
}

fn instance_box(owner: &str, functions: Vec<ASTNode>) -> ASTNode {
    box_declaration(owner, functions, false)
}

fn program(statements: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements,
        span: Span::unknown(),
    }
}

fn scalar() -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(1),
        span: Span::unknown(),
    }
}

#[test]
fn seals_static_rows_with_complete_header_and_body_pairing() {
    let source = program(vec![static_box(
        "Helpers",
        vec![
            static_function("seed", &[], Some("i64")),
            static_function("project", &[("value", Some("i64"))], None),
        ],
    )]);
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&source).unwrap();

    assert_eq!(catalog.len(), 2);
    let seed = catalog
        .static_candidates("seed", 0)
        .first()
        .expect("seed key");
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

    let project = catalog
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "Helpers",
            "project",
            1,
        )
        .unwrap();
    assert_eq!(project.params(), &["value"]);
    assert_eq!(project.param_decls()[0].name, "value");
    assert_eq!(
        project.param_decls()[0].declared_type_name.as_deref(),
        Some("i64")
    );
    assert_eq!(project.return_type_name(), None);
}

#[test]
fn separates_instance_rows_from_static_candidate_lookup() {
    let source = program(vec![
        instance_box(
            "Ordinary",
            vec![instance_function("run", &[("x", None)], Some("i64"))],
        ),
        static_box("Static", vec![static_function("run", &[("x", None)], None)]),
    ]);
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&source).unwrap();

    assert_eq!(catalog.len(), 2);
    assert_eq!(catalog.static_candidates("run", 1).len(), 1);
    assert_eq!(catalog.static_candidates("run", 1)[0].owner(), "Static");
    let instance = catalog
        .declaration_for(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            "Ordinary",
            "run",
            1,
        )
        .expect("instance method declaration");
    assert_eq!(
        instance.key().namespace(),
        SameModuleCallableNamespaceV1::InstanceBoxMethod
    );
    assert_eq!(instance.return_type_name(), Some("i64"));
}

#[test]
fn retains_exact_uses_and_declaration_attrs() {
    let mut method = instance_function("run", &[("x", Some("i64"))], Some("i64"));
    let ASTNode::FunctionDeclaration { uses, attrs, .. } = &mut method else {
        unreachable!()
    };
    *uses = vec!["rawbuf".to_string(), "atomic".to_string()];
    attrs.runes.push(RuneAttr {
        name: "Inline".to_string(),
        args: vec!["prefer".to_string()],
    });

    let source = program(vec![instance_box("Worker", vec![method])]);
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&source).unwrap();
    let declaration = catalog
        .declaration_for(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            "Worker",
            "run",
            1,
        )
        .expect("exact instance declaration");

    assert_eq!(declaration.uses(), &["rawbuf", "atomic"]);
    assert_eq!(
        declaration.attrs().runes,
        [RuneAttr {
            name: "Inline".to_string(),
            args: vec!["prefer".to_string()],
        }]
    );
}

#[test]
fn root_seal_preserves_program_single_box_and_expression_surfaces() {
    let static_root = static_box("Static", vec![static_function("run", &[], None)]);
    let static_catalog =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&static_root).unwrap();
    assert_eq!(static_catalog.len(), 1);
    assert_eq!(static_catalog.static_candidates("run", 0).len(), 1);

    let instance_root = instance_box(
        "Ordinary",
        vec![instance_function("read", &[], Some("i64"))],
    );
    let instance_catalog =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&instance_root).unwrap();
    assert_eq!(instance_catalog.len(), 1);
    assert!(instance_catalog.static_candidates("read", 0).is_empty());

    assert!(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&scalar())
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&scalar()).unwrap_err(),
        SameModuleCallableDeclarationCatalogErrorV1::ProgramRequired
    );
}

#[test]
fn declaration_reorder_preserves_normalized_inventory() {
    let attributed = |mut function: ASTNode, capability: &str, hint: &str| {
        let ASTNode::FunctionDeclaration { uses, attrs, .. } = &mut function else {
            unreachable!()
        };
        uses.push(capability.to_string());
        attrs.runes.push(RuneAttr {
            name: "Hint".to_string(),
            args: vec![hint.to_string()],
        });
        function
    };
    let beta = attributed(
        instance_function("read", &[("x", None)], None),
        "rawbuf",
        "hot",
    );
    let alpha = attributed(
        static_function("run", &[("x", None)], None),
        "atomic",
        "cold",
    );
    let left = program(vec![
        instance_box("Beta", vec![beta.clone()]),
        static_box("Alpha", vec![alpha.clone()]),
    ]);
    let right = program(vec![
        static_box("Alpha", vec![alpha]),
        instance_box("Beta", vec![beta]),
    ]);
    let normalized = |source: &ASTNode| {
        let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(source).unwrap();
        catalog
            .declarations()
            .map(|(key, declaration)| {
                (
                    key.namespace(),
                    key.owner().to_string(),
                    key.name().to_string(),
                    key.arity(),
                    declaration.uses().to_vec(),
                    declaration.attrs().clone(),
                )
            })
            .collect::<Vec<_>>()
    };

    assert_eq!(normalized(&left), normalized(&right));
}

#[test]
fn excludes_non_catalog_callable_surfaces() {
    let mut ordinary = instance_box(
        "Ordinary",
        vec![static_function("static_member", &[], None)],
    );
    let ASTNode::BoxDeclaration { constructors, .. } = &mut ordinary else {
        unreachable!()
    };
    constructors.insert("birth".to_string(), instance_function("birth", &[], None));

    let mut record = static_box("Record", vec![static_function("run", &[], None)]);
    let ASTNode::BoxDeclaration { is_record, .. } = &mut record else {
        unreachable!()
    };
    *is_record = true;

    let mut sync = instance_box("Sync", vec![instance_function("run", &[], None)]);
    let ASTNode::BoxDeclaration { is_sync, .. } = &mut sync else {
        unreachable!()
    };
    *is_sync = true;

    let source = program(vec![
        static_function("top_level", &[], None),
        ordinary,
        record,
        sync,
    ]);
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&source).unwrap();
    assert!(catalog.is_empty());
}

#[test]
fn rejects_duplicate_owner_and_malformed_method_pairing() {
    let duplicate = program(vec![
        static_box("Helpers", vec![static_function("a", &[], None)]),
        instance_box("Helpers", vec![instance_function("b", &[], None)]),
    ]);
    assert_eq!(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&duplicate).unwrap_err(),
        SameModuleCallableDeclarationCatalogErrorV1::DuplicateBoxOwner {
            owner: "Helpers".to_string(),
        }
    );

    let mut malformed = static_box("Helpers", vec![static_function("actual", &[], None)]);
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

    let mut non_function = static_box("Broken", Vec::new());
    let ASTNode::BoxDeclaration { methods, .. } = &mut non_function else {
        unreachable!()
    };
    methods.insert("run".to_string(), scalar());
    assert_eq!(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program(vec![non_function]))
            .unwrap_err(),
        SameModuleCallableDeclarationCatalogErrorV1::MethodMustBeFunction {
            owner: "Broken".to_string(),
            method: "run".to_string(),
        }
    );
}

#[test]
fn rejects_parameter_declaration_name_and_cardinality_drift() {
    let mut name_drift = static_function("run", &[("value", None)], None);
    let ASTNode::FunctionDeclaration { param_decls, .. } = &mut name_drift else {
        unreachable!()
    };
    param_decls[0].name = "other".to_string();
    let error =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program(vec![static_box(
            "Helpers",
            vec![name_drift],
        )]))
        .unwrap_err();
    assert!(matches!(
        error,
        SameModuleCallableDeclarationCatalogErrorV1::ParameterNameMismatch { index: 0, .. }
    ));

    let mut cardinality_drift = instance_function("read", &[("value", None)], None);
    let ASTNode::FunctionDeclaration { param_decls, .. } = &mut cardinality_drift else {
        unreachable!()
    };
    param_decls.clear();
    let error =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program(vec![instance_box(
            "Ordinary",
            vec![cardinality_drift],
        )]))
        .unwrap_err();
    assert!(matches!(
        error,
        SameModuleCallableDeclarationCatalogErrorV1::ParameterDeclarationCardinality {
            params: 1,
            declarations: 0,
            ..
        }
    ));
}

#[test]
fn catalog_session_rejects_missing_and_duplicate_install_without_replacing_first() {
    use crate::mir::builder::compilation_context::CompilationContext;

    let mut context = CompilationContext::new();
    assert!(matches!(
        context.callable_declaration_catalog(),
        Err(SameModuleCallableDeclarationCatalogSessionErrorV1::QueryBeforeInstall)
    ));

    let first =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program(vec![static_box(
            "Helpers",
            vec![static_function("run", &[], None)],
        )]))
        .unwrap();
    context.install_callable_declaration_catalog(first).unwrap();
    assert_eq!(context.callable_declaration_catalog().unwrap().len(), 1);

    let duplicate = VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&scalar()).unwrap();
    assert_eq!(
        context
            .install_callable_declaration_catalog(duplicate)
            .unwrap_err(),
        SameModuleCallableDeclarationCatalogSessionErrorV1::DuplicateInstall
    );
    assert_eq!(context.callable_declaration_catalog().unwrap().len(), 1);

    context.clear_callable_declaration_catalog();
    assert!(matches!(
        context.callable_declaration_catalog(),
        Err(SameModuleCallableDeclarationCatalogSessionErrorV1::QueryBeforeInstall)
    ));
    let empty = VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&scalar()).unwrap();
    context.install_callable_declaration_catalog(empty).unwrap();
    assert!(context.callable_declaration_catalog().unwrap().is_empty());
}
