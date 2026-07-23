//! Focused S0 acceptance matrix for the source-only Raw eligibility boundary.

use super::raw_root_eligibility::{
    RawEligibleCatalogV1, RawRootEligibilityErrorV1, RawRootEligibilityStageV1,
};
use super::raw_source_binding::RawCallableMainSelectionV1;
use super::{LegacyModuleLoweringInputV1, MirCompiler};
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::module_invocation_identity::ModuleInvocationFamilyV1;
use std::collections::HashMap;

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn app(body: Vec<ASTNode>, with_sibling: bool) -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert("main".into(), function("main", body));
    let mut statements = vec![ASTNode::BoxDeclaration {
        name: "Main".into(),
        methods,
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
    }];
    if with_sibling {
        statements.push(ASTNode::BoxDeclaration {
            name: "Other".into(),
            methods: HashMap::new(),
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
        });
    }
    ASTNode::Program {
        statements,
        span: Span::unknown(),
    }
}

fn bind(
    source: ASTNode,
    selection: RawCallableMainSelectionV1,
) -> super::raw_root_package::SourceBoundRawRootPackageV1 {
    let mut compiler = MirCompiler::new();
    compiler
        .bind_raw_source(
            LegacyModuleLoweringInputV1::bare_ast(source),
            None,
            "eligibility-p0",
            selection,
        )
        .unwrap()
        .into_root_package()
        .unwrap()
}

#[test]
fn selected_callable_main_is_only_a_continuation_disposition() {
    let eligible = bind(app(Vec::new(), false), RawCallableMainSelectionV1::Required)
        .prepare_eligibility()
        .unwrap();
    assert_eq!(
        eligible.proof().catalog(),
        RawEligibleCatalogV1::PlainStaticMain { helper_count: 0 }
    );
}

#[test]
fn unsupported_preprocessed_and_process_slot_shapes_reject() {
    let using = bind(
        ASTNode::Program {
            statements: vec![ASTNode::UsingStatement {
                namespace_name: "x".into(),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    )
    .prepare_eligibility()
    .unwrap_err();
    assert_eq!(using.stage(), RawRootEligibilityStageV1::Work);
    assert!(matches!(
        using.error(),
        RawRootEligibilityErrorV1::UnsupportedWork { .. }
    ));

    let new_expr = bind(
        ASTNode::Program {
            statements: vec![ASTNode::New {
                class: "IntegerBox".into(),
                arguments: Vec::new(),
                field_initializers: Vec::new(),
                type_arguments: Vec::new(),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    )
    .prepare_eligibility()
    .unwrap_err();
    assert_eq!(new_expr.stage(), RawRootEligibilityStageV1::Slots);
    assert!(matches!(
        new_expr.error(),
        RawRootEligibilityErrorV1::UnsupportedProcessGlobalSlot { .. }
    ));
}

#[test]
fn partial_catalog_and_main_arity_reject_before_physical_open() {
    let partial = bind(app(Vec::new(), true), RawCallableMainSelectionV1::Omitted)
        .prepare_eligibility()
        .unwrap_err();
    assert_eq!(partial.stage(), RawRootEligibilityStageV1::Catalog);
    assert_eq!(partial.owner_family(), ModuleInvocationFamilyV1::Raw);
    assert_eq!(partial.owner_module_name(), "eligibility-p0");

    let mut methods = HashMap::new();
    methods.insert(
        "main".into(),
        ASTNode::FunctionDeclaration {
            name: "main".into(),
            params: vec!["args".into()],
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        },
    );
    let arity = bind(
        ASTNode::Program {
            statements: vec![ASTNode::BoxDeclaration {
                name: "Main".into(),
                methods,
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
        },
        RawCallableMainSelectionV1::Omitted,
    )
    .prepare_eligibility()
    .unwrap_err();
    assert!(matches!(
        arity.error(),
        RawRootEligibilityErrorV1::MainMustBeArityZero
    ));
}

#[test]
fn rejection_keeps_identity_and_no_physical_owner_is_opened() {
    let package = bind(
        ASTNode::Program {
            statements: vec![ASTNode::StaticConstTable {
                name: "T".into(),
                element_type: "u16".into(),
                values: vec![1, 2],
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        },
        RawCallableMainSelectionV1::Omitted,
    );
    let brand = package.brand();
    let rejected = package.prepare_eligibility().unwrap_err();
    assert_eq!(rejected.owner_brand(), brand);
    assert_eq!(rejected.owner_family(), ModuleInvocationFamilyV1::Raw);
    rejected.discard();
}

#[allow(dead_code)]
fn _literal_fixture() -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(1),
        span: Span::unknown(),
    }
}
