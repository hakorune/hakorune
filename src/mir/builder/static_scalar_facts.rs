//! Static scalar method facts.
//!
//! This is a narrow fact surface, not generic CSE and not a whole-box purity
//! marker. A fact is produced only when the method body itself proves a
//! zero-arg static method returns a literal scalar.

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::ValueId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StaticScalarValue {
    I64(i64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StaticScalarMethodFact {
    pub method_symbol: String,
    pub value: StaticScalarValue,
    pub proof: &'static str,
}

pub(crate) fn infer_static_scalar_method_fact(
    method_symbol: &str,
    params: &[String],
    body: &[ASTNode],
) -> Option<StaticScalarMethodFact> {
    if !params.is_empty() || body.len() != 1 {
        return None;
    }

    let ASTNode::Return {
        value: Some(value), ..
    } = &body[0]
    else {
        return None;
    };

    let ASTNode::Literal { value, .. } = value.as_ref() else {
        return None;
    };

    let value = match value {
        LiteralValue::Integer(value) => StaticScalarValue::I64(*value),
        LiteralValue::TypedInteger { value, .. } => StaticScalarValue::I64(*value),
        LiteralValue::Bool(value) => StaticScalarValue::Bool(*value),
        _ => return None,
    };

    Some(StaticScalarMethodFact {
        method_symbol: method_symbol.to_string(),
        value,
        proof: "zero_arg_return_literal_only",
    })
}

pub(crate) fn emit_static_scalar_fact_const(
    builder: &mut super::MirBuilder,
    fact: &StaticScalarMethodFact,
) -> Result<ValueId, String> {
    match fact.value {
        StaticScalarValue::I64(value) => {
            crate::mir::builder::emission::constant::emit_integer(builder, value)
        }
        StaticScalarValue::Bool(value) => {
            crate::mir::builder::emission::constant::emit_bool(builder, value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn return_integer(value: i64) -> ASTNode {
        ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(value),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }
    }

    #[test]
    fn infers_zero_arg_integer_return_literal() {
        let fact =
            infer_static_scalar_method_fact("Reason.small_no_page/0", &[], &[return_integer(1)])
                .expect("return literal should infer a fact");
        assert_eq!(fact.method_symbol, "Reason.small_no_page/0");
        assert_eq!(fact.value, StaticScalarValue::I64(1));
        assert_eq!(fact.proof, "zero_arg_return_literal_only");
    }

    #[test]
    fn rejects_methods_with_params() {
        let params = vec!["x".to_string()];
        assert!(infer_static_scalar_method_fact("Reason.with_arg/1", &params, &[return_integer(1)])
            .is_none());
    }

    #[test]
    fn rejects_non_literal_return_expression() {
        let body = [ASTNode::Return {
            value: Some(Box::new(ASTNode::FunctionCall {
                name: "other".to_string(),
                arguments: Vec::new(),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }];
        assert!(infer_static_scalar_method_fact("Reason.other/0", &[], &body).is_none());
    }
}
