use super::*;
use crate::ast::{ASTNode, DeclarationAttrs, Span};
use std::collections::HashMap;

fn function(name: &str, is_static: bool, arity: usize) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: (0..arity).map(|index| format!("p{index}")).collect(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn program(methods: HashMap<String, ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements: vec![ASTNode::BoxDeclaration {
            name: "Main".to_owned(),
            methods: crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(methods),
            is_static: true,
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_sync: false,
            is_record: false,
            type_parameters: Vec::new(),
            extends: Vec::new(),
            implements: Vec::new(),
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

#[test]
fn expansion_separates_root_children_and_compat_identity() {
    let mut methods = HashMap::new();
    methods.insert("zeta".to_owned(), function("zeta", true, 2));
    methods.insert("main".to_owned(), function("main", true, 1));
    methods.insert("alpha".to_owned(), function("alpha", true, 0));
    let source = program(methods);

    let expansion = VerifiedMainExpansionV1::from_program(&source).unwrap();
    assert_eq!(expansion.root().box_name(), "Main");
    assert_eq!(
        expansion
            .static_children()
            .iter()
            .map(|child| child.symbol())
            .collect::<Vec<_>>(),
        vec!["Main.alpha/0", "Main.zeta/2"]
    );
    let zeta = &expansion.static_children()[1];
    let (symbol, params, param_decls, result, body, uses, attrs) =
        zeta.to_owned_lowering().into_parts();
    assert_eq!(symbol, "Main.zeta/2");
    assert_eq!(params, vec!["p0".to_owned(), "p1".to_owned()]);
    assert!(param_decls.is_empty());
    assert!(result.is_none());
    assert!(body.is_empty());
    assert!(uses.is_empty());
    assert_eq!(attrs, DeclarationAttrs::default());
    let (box_name, callable_symbol, params, param_decls, result, body, uses, attrs) =
        expansion.to_owned_root_lowering().into_parts();
    assert_eq!(box_name, "Main");
    assert_eq!(callable_symbol.as_deref(), Some("Main.main/1"));
    assert_eq!(params, vec!["p0".to_owned()]);
    assert!(param_decls.is_empty());
    assert!(result.is_none());
    assert!(body.is_empty());
    assert!(uses.is_empty());
    assert_eq!(attrs, DeclarationAttrs::default());
    let compat = expansion.callable_main_compat().unwrap();
    assert_eq!(compat.symbol(), "Main.main/1");
    assert_eq!(expansion.root().source(), compat.source());
}

#[test]
fn malformed_or_missing_main_fails_before_builder_effects() {
    let empty = ASTNode::Program {
        statements: Vec::new(),
        span: Span::unknown(),
    };
    assert_eq!(
        VerifiedMainExpansionV1::from_program(&empty).unwrap_err(),
        MainExpansionErrorV1::MainBoxMissing
    );

    let mut methods = HashMap::new();
    methods.insert(
        "main".to_owned(),
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(1),
            span: Span::unknown(),
        },
    );
    assert_eq!(
        VerifiedMainExpansionV1::from_program(&program(methods)).unwrap_err(),
        MainExpansionErrorV1::MainMethodMustBeFunction
    );
}

#[test]
fn app_shape_ignores_non_main_top_level_statements() {
    let mut source = program({
        let mut methods = HashMap::new();
        methods.insert("main".to_owned(), function("main", true, 0));
        methods.insert("helper".to_owned(), function("helper", true, 1));
        methods
    });
    let ASTNode::Program { statements, .. } = &mut source else {
        unreachable!("program helper creates a Program");
    };
    statements.insert(
        0,
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(7),
            span: Span::unknown(),
        },
    );

    let expansion = VerifiedMainExpansionV1::from_program(&source).unwrap();
    assert_eq!(expansion.root().box_name(), "Main");
    assert_eq!(expansion.static_children().len(), 1);
    assert_eq!(expansion.static_children()[0].symbol(), "Main.helper/1");
    assert_eq!(
        expansion.callable_main_compat().unwrap().symbol(),
        "Main.main/0"
    );
}

#[test]
fn script_shape_without_static_main_stays_out_of_this_product() {
    let source = ASTNode::Program {
        statements: vec![ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(1),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };

    assert_eq!(
        VerifiedMainExpansionV1::from_program(&source).unwrap_err(),
        MainExpansionErrorV1::MainBoxMissing
    );
}

#[test]
fn child_and_root_static_contracts_are_checked_before_builder_effects() {
    let mut methods = HashMap::new();
    methods.insert("main".to_owned(), function("main", true, 0));
    methods.insert("instance".to_owned(), function("instance", false, 0));
    assert_eq!(
        VerifiedMainExpansionV1::from_program(&program(methods)).unwrap_err(),
        MainExpansionErrorV1::StaticChildMustBeStatic {
            method: "instance".to_owned(),
        }
    );

    let mut methods = HashMap::new();
    methods.insert("main".to_owned(), function("main", false, 0));
    assert_eq!(
        VerifiedMainExpansionV1::from_program(&program(methods)).unwrap_err(),
        MainExpansionErrorV1::StaticChildMustBeStatic {
            method: "main".to_owned(),
        }
    );
}

#[test]
fn duplicate_main_boxes_are_rejected_without_order_dependence() {
    let mut first_methods = HashMap::new();
    first_methods.insert("main".to_owned(), function("main", true, 0));
    let mut second_methods = HashMap::new();
    second_methods.insert("main".to_owned(), function("main", true, 0));

    let mut source = program(first_methods);
    let ASTNode::Program { statements, .. } = &mut source else {
        unreachable!("program helper creates a Program");
    };
    let ASTNode::BoxDeclaration {
        name,
        methods,
        is_static,
        fields,
        field_decls,
        public_fields,
        private_fields,
        constructors,
        init_fields,
        weak_fields,
        delegates,
        invariants,
        transitions,
        is_interface,
        is_sync,
        is_record,
        type_parameters,
        extends,
        implements,
        static_init,
        attrs,
        span,
    } = &statements[0]
    else {
        unreachable!("program helper creates a Main box");
    };
    statements.push(ASTNode::BoxDeclaration {
        name: name.clone(),
        methods: crate::ast::BoxMethodInventoryV1::from_legacy_ast_map(second_methods),
        is_static: *is_static,
        fields: fields.clone(),
        field_decls: field_decls.clone(),
        public_fields: public_fields.clone(),
        private_fields: private_fields.clone(),
        constructors: constructors.clone(),
        init_fields: init_fields.clone(),
        weak_fields: weak_fields.clone(),
        delegates: delegates.clone(),
        invariants: invariants.clone(),
        transitions: transitions.clone(),
        is_interface: *is_interface,
        is_sync: *is_sync,
        is_record: *is_record,
        type_parameters: type_parameters.clone(),
        extends: extends.clone(),
        implements: implements.clone(),
        static_init: static_init.clone(),
        attrs: attrs.clone(),
        span: *span,
    });

    assert_eq!(
        VerifiedMainExpansionV1::from_program(&source).unwrap_err(),
        MainExpansionErrorV1::DuplicateMainBox
    );
}

#[test]
fn raw_root_selector_accepts_script_without_main_box() {
    let source = ASTNode::Program {
        statements: vec![ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(1),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    assert!(matches!(
        VerifiedRawRootExpansionV1::from_program(&source).unwrap(),
        VerifiedRawRootExpansionV1::Script
    ));
}

#[test]
fn raw_root_selector_rejects_duplicate_main_before_app_expansion() {
    let mut methods = HashMap::new();
    methods.insert("main".to_owned(), function("main", true, 0));
    let mut source = program(methods);
    let ASTNode::Program { statements, .. } = &mut source else {
        unreachable!("program helper creates a Program");
    };
    let duplicate = statements[0].clone();
    statements.push(duplicate);
    assert_eq!(
        VerifiedRawRootExpansionV1::from_program(&source).unwrap_err(),
        MainExpansionErrorV1::DuplicateMainBox
    );
}
