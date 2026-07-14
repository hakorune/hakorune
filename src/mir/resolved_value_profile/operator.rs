//! Closed operator/operand representation table for the first trivial profile.

use crate::ast::{BinaryOperator, LiteralValue};

use super::product::TrivialRepresentationV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TrivialBinaryProfileStopV1 {
    OperatorOutsideProfile,
    OperandsNotExact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TrivialLiteralProfileStopV1 {
    String,
}

pub(super) fn derive_trivial_literal_profile_v1(
    value: &LiteralValue,
) -> Result<TrivialRepresentationV1, TrivialLiteralProfileStopV1> {
    match value {
        LiteralValue::Integer(_) | LiteralValue::TypedInteger { .. } => {
            Ok(TrivialRepresentationV1::InlineI64)
        }
        LiteralValue::Bool(_) => Ok(TrivialRepresentationV1::InlineBool),
        LiteralValue::Float(_) => Ok(TrivialRepresentationV1::InlineF64),
        LiteralValue::String(_) => Err(TrivialLiteralProfileStopV1::String),
        LiteralValue::Void => Ok(TrivialRepresentationV1::ExplicitVoidValue),
        LiteralValue::Null => Ok(TrivialRepresentationV1::NullSentinel),
    }
}

pub(super) fn derive_trivial_binary_profile_v1(
    operator: &BinaryOperator,
    left: TrivialRepresentationV1,
    right: TrivialRepresentationV1,
) -> Result<TrivialRepresentationV1, TrivialBinaryProfileStopV1> {
    use BinaryOperator::*;

    let homogeneous = left == right;
    let numeric = matches!(
        left,
        TrivialRepresentationV1::InlineI64 | TrivialRepresentationV1::InlineF64
    );
    match operator {
        Add | Subtract | Multiply | Divide if homogeneous && numeric => Ok(left),
        Modulo if homogeneous && left == TrivialRepresentationV1::InlineI64 => Ok(left),
        BitAnd | BitOr | BitXor | Shl | Shr
            if homogeneous && left == TrivialRepresentationV1::InlineI64 =>
        {
            Ok(TrivialRepresentationV1::InlineI64)
        }
        Equal | NotEqual if homogeneous => Ok(TrivialRepresentationV1::InlineBool),
        Less | Greater | LessEqual | GreaterEqual if homogeneous && numeric => {
            Ok(TrivialRepresentationV1::InlineBool)
        }
        And | Or => Err(TrivialBinaryProfileStopV1::OperatorOutsideProfile),
        _ => Err(TrivialBinaryProfileStopV1::OperandsNotExact),
    }
}
