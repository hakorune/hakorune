use super::*;
use crate::ast::{FieldDecl, Span};

fn record_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.comp_ctx.register_record_decl(
        "Pair".to_owned(),
        Vec::new(),
        &[FieldDecl {
            name: "value".to_owned(),
            declared_type_name: None,
            is_weak: false,
            default_value: None,
        }],
    );
    builder
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn route_name(prepared: &PreparedRawNewExpressionV1) -> &'static str {
    match &prepared.route {
        PreparedRawNewExpressionRouteV1::Core13Pure { .. } => "core13-pure",
        PreparedRawNewExpressionRouteV1::IntegerLiteral { .. } => "integer-literal",
        PreparedRawNewExpressionRouteV1::Ordinary { .. } => "ordinary",
    }
}

#[test]
fn raw_new_route_preserves_record_error_precedence_before_effects() {
    let builder = record_builder();
    let literal = ASTNode::Literal {
        value: LiteralValue::Integer(1),
        span: Span::unknown(),
    };
    let with_fields = PreparedRawNewExpressionV1::prepare(
        &builder,
        "Pair".to_owned(),
        Vec::new(),
        vec![("value".to_owned(), literal)],
    )
    .err()
    .expect("record field initializer must reject");
    let without_fields =
        PreparedRawNewExpressionV1::prepare(&builder, "Pair".to_owned(), Vec::new(), Vec::new())
            .err()
            .expect("raw record construction must reject");

    assert!(
        with_fields.starts_with("[box-init/record-reject]"),
        "{with_fields}"
    );
    assert!(
        without_fields.starts_with("[record-construction/escape]"),
        "{without_fields}"
    );
    assert!(builder.current_module.is_none());
    assert!(builder.function_state.current_function.is_none());
}

#[test]
fn raw_new_creation_route_preserves_mode_and_integer_priority() {
    let builder = MirBuilder::new();
    crate::test_support::with_env_var("NYASH_MIR_CORE13_PURE", "off", || {
        let integer_route = PreparedRawNewExpressionV1::prepare(
            &builder,
            "IntegerBox".to_owned(),
            vec![integer(7)],
            Vec::new(),
        )
        .unwrap();
        let ordinary = PreparedRawNewExpressionV1::prepare(
            &builder,
            "Page".to_owned(),
            vec![integer(7)],
            Vec::new(),
        )
        .unwrap();
        assert_eq!(route_name(&integer_route), "integer-literal");
        assert_eq!(route_name(&ordinary), "ordinary");
    });
    crate::test_support::with_env_var("NYASH_MIR_CORE13_PURE", "1", || {
        let core13 = PreparedRawNewExpressionV1::prepare(
            &builder,
            "IntegerBox".to_owned(),
            vec![integer(7)],
            Vec::new(),
        )
        .unwrap();
        assert_eq!(route_name(&core13), "core13-pure");
    });
    assert!(builder.current_module.is_none());
    assert!(builder.function_state.current_function.is_none());
}
