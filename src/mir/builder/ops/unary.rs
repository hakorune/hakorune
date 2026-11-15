//! Raw unary source-operator projection.
//!
//! Integer literal negation is folded only when checked negation succeeds.
//! Otherwise evaluate the operand once, emit its declared UnaryOp, and retain
//! the result type. Borrowed/owned expression responsibilities stay with the
//! existing recursive child port.

use super::super::{MirInstruction, MirType, ValueId};
use crate::ast::{ASTNode, UnaryOperator};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};

pub(in crate::mir::builder) struct PreparedRawUnaryV1 {
    route: PreparedRawUnaryRouteV1,
}

enum PreparedRawUnaryRouteV1 {
    UnsupportedCopy,
    UnsupportedMove,
    UnsupportedShared,
    Weak {
        operand: ASTNode,
    },
    Ordinary {
        operator: PreparedRawOrdinaryUnaryOperatorV1,
        operand: ASTNode,
    },
}

#[derive(Clone, Copy)]
enum PreparedRawOrdinaryUnaryOperatorV1 {
    Minus,
    Not,
    BitNot,
}

impl PreparedRawUnaryV1 {
    pub(in crate::mir::builder) fn prepare(operator: UnaryOperator, operand: ASTNode) -> Self {
        let route = match operator {
            UnaryOperator::Copy => PreparedRawUnaryRouteV1::UnsupportedCopy,
            UnaryOperator::Move => PreparedRawUnaryRouteV1::UnsupportedMove,
            UnaryOperator::Shared => PreparedRawUnaryRouteV1::UnsupportedShared,
            UnaryOperator::Weak => PreparedRawUnaryRouteV1::Weak { operand },
            UnaryOperator::Minus => PreparedRawUnaryRouteV1::Ordinary {
                operator: PreparedRawOrdinaryUnaryOperatorV1::Minus,
                operand,
            },
            UnaryOperator::Not => PreparedRawUnaryRouteV1::Ordinary {
                operator: PreparedRawOrdinaryUnaryOperatorV1::Not,
                operand,
            },
            UnaryOperator::BitNot => PreparedRawUnaryRouteV1::Ordinary {
                operator: PreparedRawOrdinaryUnaryOperatorV1::BitNot,
                operand,
            },
        };
        Self { route }
    }
}

impl PreparedRawOrdinaryUnaryOperatorV1 {
    fn return_type(self) -> MirType {
        match self {
            Self::Minus | Self::BitNot => MirType::Integer,
            Self::Not => MirType::Bool,
        }
    }

    fn is_minus(self) -> bool {
        matches!(self, Self::Minus)
    }

    fn mir_operator(self) -> crate::mir::UnaryOp {
        match self {
            Self::Minus => crate::mir::UnaryOp::Neg,
            Self::Not => crate::mir::UnaryOp::Not,
            Self::BitNot => crate::mir::UnaryOp::BitNot,
        }
    }
}

pub(in crate::mir::builder) fn lower_prepared_raw_unary_with_port_v1<Port>(
    builder: &mut super::super::MirBuilder,
    port: &mut Port,
    prepared: PreparedRawUnaryV1,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
{
    match prepared.route {
        PreparedRawUnaryRouteV1::UnsupportedCopy => Err("[type/copy_contract_unsupported] copy requires the checked source expression owner".into()),
        PreparedRawUnaryRouteV1::UnsupportedMove => Err(
            "[freeze:contract][ownership/move-not-supported] move has not reached the Home owner"
                .to_string(),
        ),
        PreparedRawUnaryRouteV1::UnsupportedShared => Err(
            "[freeze:contract][ownership/shared-not-supported] share is supported only by a checked local initializer; use share new T(), share move x or share h".to_string(),
        ),
        PreparedRawUnaryRouteV1::Weak { operand } => {
            let box_value = drive_legacy_expression_v1(builder, port, operand)?;
            builder.emit_weak_new(box_value)
        }
        PreparedRawUnaryRouteV1::Ordinary { operator, operand } => {
            lower_prepared_raw_ordinary_unary_with_port_v1(builder, port, operator, operand)
        }
    }
}

fn lower_prepared_raw_ordinary_unary_with_port_v1<Port>(
    builder: &mut super::super::MirBuilder,
    port: &mut Port,
    operator: PreparedRawOrdinaryUnaryOperatorV1,
    operand: ASTNode,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
{
    let return_type = operator.return_type();
    if operator.is_minus() {
        if let ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(n),
            ..
        } = &operand
        {
            if let Some(negated) = n.checked_neg() {
                return crate::mir::builder::emission::constant::emit_integer(builder, negated);
            }
        }
    }
    let operand_val = drive_legacy_expression_v1(builder, port, operand)?;
    let dst = builder.next_value_id();
    let mir_op = operator.mir_operator();
    builder.emit_instruction(MirInstruction::UnaryOp {
        dst,
        op: mir_op,
        operand: operand_val,
    })?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(dst, return_type);
    Ok(dst)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};

    fn operand() -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }
    }

    fn route(prepared: PreparedRawUnaryV1) -> &'static str {
        match prepared.route {
            PreparedRawUnaryRouteV1::UnsupportedCopy => "copy-unsupported",
            PreparedRawUnaryRouteV1::UnsupportedMove => "move-unsupported",
            PreparedRawUnaryRouteV1::UnsupportedShared => "shared-unsupported",
            PreparedRawUnaryRouteV1::Weak { .. } => "weak",
            PreparedRawUnaryRouteV1::Ordinary {
                operator: PreparedRawOrdinaryUnaryOperatorV1::Minus,
                ..
            } => "minus",
            PreparedRawUnaryRouteV1::Ordinary {
                operator: PreparedRawOrdinaryUnaryOperatorV1::Not,
                ..
            } => "not",
            PreparedRawUnaryRouteV1::Ordinary {
                operator: PreparedRawOrdinaryUnaryOperatorV1::BitNot,
                ..
            } => "bit-not",
        }
    }

    #[test]
    fn source_operator_partition_is_total_and_disjoint() {
        let routes = [
            route(PreparedRawUnaryV1::prepare(UnaryOperator::Weak, operand())),
            route(PreparedRawUnaryV1::prepare(UnaryOperator::Move, operand())),
            route(PreparedRawUnaryV1::prepare(
                UnaryOperator::Shared,
                operand(),
            )),
            route(PreparedRawUnaryV1::prepare(UnaryOperator::Minus, operand())),
            route(PreparedRawUnaryV1::prepare(UnaryOperator::Not, operand())),
            route(PreparedRawUnaryV1::prepare(
                UnaryOperator::BitNot,
                operand(),
            )),
        ];
        assert_eq!(
            routes,
            [
                "weak",
                "move-unsupported",
                "shared-unsupported",
                "minus",
                "not",
                "bit-not"
            ]
        );
    }
}
