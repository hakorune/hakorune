use std::collections::HashMap;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};

use super::super::{
    NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1, SealedNormalSourcePlanV1,
};
use super::*;

fn helper(name: &str) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: vec!["n".to_owned()],
        param_decls: vec![ParamDecl {
            name: "n".to_owned(),
            declared_type_name: Some("i64".to_owned()),
        }],
        return_type_name: Some("i64".to_owned()),
        body: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Variable {
                name: "n".to_owned(),
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
    }
}

fn call(name: &str) -> ASTNode {
    ASTNode::FunctionCall {
        name: name.to_owned(),
        arguments: vec![ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

fn main_box(body: Vec<ASTNode>) -> ASTNode {
    let mut methods = HashMap::new();
    methods.insert(
        "main".to_owned(),
        ASTNode::FunctionDeclaration {
            name: "main".to_owned(),
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
        },
    );
    ASTNode::BoxDeclaration {
        name: "Main".to_owned(),
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
        is_sync: false,
        is_record: false,
        type_parameters: Vec::new(),
        extends: Vec::new(),
        implements: Vec::new(),
        is_static: true,
        static_init: None,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn resolved_main(
    statements: Vec<ASTNode>,
) -> Result<VerifiedNormalMainDirectCallSourceUnitV1, RejectedNormalMainDirectCallSourceV1> {
    let plan = NormalSourcePlanClassifierV1::seal(PreparedNormalSourcePlanInputV1::new(
        ASTNode::Program {
            statements,
            span: Span::unknown(),
        },
        "normal-main-direct-call-source-test",
    ))
    .unwrap();
    let SealedNormalSourcePlanV1::CallableModule(source) = plan else {
        panic!("expected callable module")
    };
    source
        .prepare_callable_source()
        .unwrap()
        .prepare_helper_catalog(29)
        .unwrap()
        .prepare_main_with_helper_catalog()
}

#[test]
fn call_free_main_and_helper_share_one_retained_catalog() {
    let source = resolved_main(vec![main_box(Vec::new()), helper("helper")]).unwrap();
    let input = source.borrow_function_input().unwrap();

    assert_eq!(source.helper_count(), 1);
    assert_eq!(
        source.source_identity(),
        "normal-main-direct-call-source-test"
    );
    assert_eq!(input.function().direct_call_targets().count(), 0);
    assert!(input.callable_index().is_some());
}

#[test]
fn main_direct_call_uses_helper_owner_from_the_same_compilation_brand() {
    let source = resolved_main(vec![
        main_box(vec![ASTNode::Return {
            value: Some(Box::new(call("helper"))),
            span: Span::unknown(),
        }]),
        helper("helper"),
    ])
    .unwrap();
    let input = source.borrow_function_input().unwrap();
    let targets = input.function().direct_call_targets().collect::<Vec<_>>();
    let [(_, target)] = targets.as_slice() else {
        panic!("expected one direct call")
    };

    assert_eq!(
        input.owner().compilation_brand(),
        target.callable().owner().compilation_brand()
    );
    let header = input
        .callable_index()
        .unwrap()
        .header_for_callable(target.callable())
        .unwrap();
    assert_eq!(header.symbol().as_mir_name(), "helper/1");
}

#[test]
fn unresolved_main_call_rejects_without_call_free_retry() {
    let rejected = resolved_main(vec![
        main_box(vec![ASTNode::Return {
            value: Some(Box::new(call("missing"))),
            span: Span::unknown(),
        }]),
        helper("helper"),
    ])
    .unwrap_err();

    assert_eq!(
        rejected.stage(),
        NormalMainDirectCallSourceStageV1::OwnerForest
    );
    assert!(matches!(
        rejected.error(),
        NormalMainDirectCallSourceErrorV1::OwnerForest(_)
    ));
    rejected.discard();
}
