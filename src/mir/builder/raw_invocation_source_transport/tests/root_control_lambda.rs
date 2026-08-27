use super::*;
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
};
use crate::mir::builder::RawSourceLocatorV1;

#[test]
fn located_root_derives_exact_body_item_without_reissuing_lineage() {
    let box_statement = ASTNode::BoxDeclaration {
        name: "Page".to_owned(),
        fields: Vec::new(),
        field_decls: Vec::new(),
        public_fields: Vec::new(),
        private_fields: Vec::new(),
        methods: crate::ast::BoxMethodInventoryV1::empty(),
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
        attrs: crate::ast::DeclarationAttrs::default(),
        span: crate::ast::Span::unknown(),
    };
    let root = RawInvocationRootLineageV1::Main(RawSourceLocatorV1::for_test(
        0,
        "Main",
        "main",
        "Main.main/0",
        0,
    ));
    let (_, context) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::root(Vec::<ASTNode>::new(), root.clone()),
    );
    let (_, child) =
        RawInvocationSourceContextV1::from_transport(context.body_statement(box_statement, 3));

    assert!(matches!(
        child,
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::Main(_),
            ..
        }
    ));
    assert_eq!(
        child.site().expect("located child").segments(),
        &[SourcePathSegmentV1::Body(3)]
    );
}

#[test]
fn located_controls_and_diagnostic_terminals_keep_exact_parent_sites() {
    let controls = [
        ASTNode::If {
            condition: Box::new(integer(1)),
            then_body: vec![integer(2)],
            else_body: Some(vec![integer(3)]),
            span: Span::unknown(),
        },
        ASTNode::Loop {
            condition: Box::new(integer(1)),
            body: vec![integer(2)],
            span: Span::unknown(),
        },
        ASTNode::TaskScope {
            body: vec![integer(1)],
            source_keyword: "co".to_owned(),
            span: Span::unknown(),
        },
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_owned(),
            body: vec![integer(1)],
            span: Span::unknown(),
        },
        ASTNode::ScopeBox {
            body: vec![integer(1)],
            span: Span::unknown(),
        },
        ASTNode::BlockExpr {
            prelude_stmts: vec![integer(1)],
            tail_expr: Box::new(integer(2)),
            span: Span::unknown(),
        },
        ASTNode::LoopRange {
            var_name: "i".to_owned(),
            start: Box::new(integer(0)),
            end: Box::new(integer(1)),
            body: Vec::new(),
            span: Span::unknown(),
        },
        ASTNode::ContextScope {
            name: "ctx".to_owned(),
            declared_type_name: None,
            value: Box::new(integer(1)),
            body: Vec::new(),
            source_keyword: "context".to_owned(),
            span: Span::unknown(),
        },
    ];
    let (_, root) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::root(
            Vec::<ASTNode>::new(),
            RawInvocationRootLineageV1::ScriptRoot,
        ));

    for (index, control) in controls.into_iter().enumerate() {
        let (_, child) =
            RawInvocationSourceContextV1::from_transport(root.body_statement(control, index));
        assert!(matches!(
            child,
            RawInvocationSourceContextV1::Located { .. }
        ));
        assert_eq!(
            child.site().expect("structured control site").segments(),
            &[SourcePathSegmentV1::Body(index as u32)]
        );
    }
}

#[test]
fn lambda_statement_keeps_an_exact_parent_site() {
    let lambda = ASTNode::Lambda {
        params: Vec::new(),
        body: vec![integer(1)],
        span: Span::unknown(),
    };
    let (_, root) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::root(
            Vec::<ASTNode>::new(),
            RawInvocationRootLineageV1::ScriptRoot,
        ));
    let (_, child) = RawInvocationSourceContextV1::from_transport(root.body_statement(lambda, 4));

    assert!(matches!(
        child,
        RawInvocationSourceContextV1::Located { .. }
    ));
    assert_eq!(
        child.site().expect("located Lambda statement").segments(),
        &[SourcePathSegmentV1::Body(4)]
    );
}
