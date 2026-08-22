use crate::ast::{ASTNode, LiteralValue, Span};

use super::source_method_typeop_route::{
    classify_source_method_typeop_route_v1, SourceMethodTypeOpDispositionV1 as Disposition,
    SourceMethodTypeOpKindV1 as Kind,
};

fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.into()),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn string_box(value: &str) -> ASTNode {
    ASTNode::New {
        class: "StringBox".into(),
        arguments: vec![string(value)],
        field_initializers: vec![],
        type_arguments: vec![],
        span: Span::unknown(),
    }
}

#[test]
fn direct_string_typeops_are_typed_non_candidates() {
    for (method, kind) in [("is", Kind::Is), ("as", Kind::As)] {
        assert_eq!(
            classify_source_method_typeop_route_v1(method, &[string("Integer")]),
            Disposition::TypeOp {
                kind,
                type_name: "Integer".into(),
            }
        );
    }
}

#[test]
fn string_box_type_name_matches_builder_shape() {
    assert_eq!(
        classify_source_method_typeop_route_v1("is", &[string_box("I64")]),
        Disposition::TypeOp {
            kind: Kind::Is,
            type_name: "I64".into(),
        }
    );
}

#[test]
fn ordinary_is_as_shapes_remain_ordinary() {
    for (method, arguments) in [
        ("is", Vec::new()),
        ("as", vec![integer(1), integer(2)]),
        ("is", vec![integer(1)]),
        (
            "as",
            vec![ASTNode::Variable {
                name: "value".into(),
                span: Span::unknown(),
            }],
        ),
        ("run", vec![string("Integer")]),
    ] {
        assert_eq!(
            classify_source_method_typeop_route_v1(method, &arguments),
            Disposition::Ordinary
        );
    }
}
