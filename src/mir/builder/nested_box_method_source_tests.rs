use super::*;

use crate::ast::{ASTNode, Span};
use crate::mir::builder::raw_invocation_source_transport::RawUnlocatedPortalV1;
use crate::mir::builder::RawSourceLocatorV1;
use crate::mir::resolved_semantics::{SourceBodyKindV1, SourcePathSegmentV1};

#[test]
fn located_parent_seals_function_relative_nested_method_root() {
    let root = RawInvocationRootLineageV1::Main(RawSourceLocatorV1::for_test(
        0,
        "Main",
        "main",
        "Main.main/0",
        0,
    ));
    let (_, parent) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::root(Vec::<ASTNode>::new(), root),
    );
    let (_, parent) = RawInvocationSourceContextV1::from_transport(parent.body_statement(
        ASTNode::BoxDeclaration {
            name: "Page".to_owned(),
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods: std::collections::HashMap::new(),
            constructors: std::collections::HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_record: false,
            is_static: false,
            extends: Vec::new(),
            implements: Vec::new(),
            type_parameters: Vec::new(),
            is_sync: false,
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        },
        2,
    ));
    let source = PreparedNestedBoxMethodSourceV1::from_located_parent(Some(parent), "run".into())
        .expect("located Box parent");
    let (_, method) = RawInvocationSourceContextV1::from_transport(source.transport());
    assert!(matches!(
        method,
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::NestedBoxMethod { .. },
            body_kind: Some(SourceBodyKindV1::Function),
            ..
        }
    ));
    let (_, body) = RawInvocationSourceContextV1::from_transport(method.body_statement(
        ASTNode::Return {
            value: None,
            span: Span::unknown(),
        },
        0,
    ));
    assert_eq!(
        body.site().expect("nested body site").segments(),
        &[
            SourcePathSegmentV1::FunctionBody,
            SourcePathSegmentV1::Body(0)
        ]
    );
}

#[test]
fn unlocated_parent_rejects_before_method_capture() {
    let (_, unlocated) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::unlocated((), RawUnlocatedPortalV1::CallObject),
    );
    assert!(
        PreparedNestedBoxMethodSourceV1::from_located_parent(Some(unlocated), "run".into())
            .is_err()
    );
}
