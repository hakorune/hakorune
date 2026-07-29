//! Unary Operations Module
//!
//! This module handles building unary operations (-, !, ~) with support for:
//! - Operator Box routing (NegOperator, NotOperator, BitNotOperator)
//! - Core-13 pure expansion (when mir_core13_pure() gate enabled)
//! - Guard detection to prevent infinite recursion
//! - Return type tracking (Integer vs Bool)
//!
//! ## Operator Box Routing
//!
//! When `NYASH_BUILDER_OPERATOR_BOX_ALL_CALL=1` is set:
//! - `-x` → `NegOperator.apply/1(x)` (returns Integer)
//! - `!x` → `NotOperator.apply/1(x)` (returns Bool)
//! - `~x` → `BitNotOperator.apply/1(x)` (returns Integer)
//!
//! Guard detection prevents infinite recursion by checking if we're already inside
//! the operator method being called.
//!
//! ## Core-13 Pure Expansion
//!
//! When `mir_core13_pure()` gate is enabled, unary operations are expanded to
//! Core-13 pure instructions:
//! - `-x` → `Sub(0, x)` (negation via zero subtraction)
//! - `!x` → `Compare(Eq, x, false)` (logical NOT via equality comparison)
//! - `~x` → `BitXor(x, -1)` (bitwise NOT via XOR with all-ones)
//!
//! ## Type Tracking
//!
//! Each operator has a well-defined return type:
//! - Negation (`-`): Integer
//! - Logical NOT (`!`, `not`): Bool
//! - Bitwise NOT (`~`): Integer
//!
//! ## Example Transformations
//!
//! ### Operator Box Call (ALL_CALL mode)
//! ```ignore
//! -x  →  %result = Call("NegOperator.apply/1", [x]) : Integer
//! !x  →  %result = Call("NotOperator.apply/1", [x]) : Bool
//! ~x  →  %result = Call("BitNotOperator.apply/1", [x]) : Integer
//! ```
//!
//! ### Core-13 Pure Expansion
//! ```ignore
//! -x  →  %zero = Const(0)
//!        %result = BinOp(Sub, %zero, x)
//!
//! !x  →  %false = Const(false)
//!        %result = Compare(Eq, x, %false)
//!
//! ~x  →  %all_ones = Const(-1)
//!        %result = BinOp(BitXor, x, %all_ones)
//! ```
//!
//! ## Responsibilities
//!
//! - Evaluate operand expression
//! - Check for operator box routing flags
//! - Detect guard conditions to prevent recursion
//! - Apply Core-13 pure expansion when enabled
//! - Emit appropriate MIR instruction (UnaryOp or expanded form)
//! - Track result type in type context
//!
//! ## Integration Points
//!
//! - Called from: `exprs.rs` when handling UnaryOp AST pattern
//! - Uses: `emission::constant` for Core-13 expansion constants
//! - Uses: `emission::compare` for logical NOT expansion
//! - Owns the typed source-operator projection used by every raw Unary route

use super::super::{MirInstruction, MirType, ValueId};
use crate::ast::{ASTNode, UnaryOperator};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};

pub(in crate::mir::builder) struct PreparedRawUnaryV1 {
    route: PreparedRawUnaryRouteV1,
}

enum PreparedRawUnaryRouteV1 {
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

    fn operator_box(self) -> (&'static str, &'static str) {
        match self {
            Self::Minus => ("NegOperator.apply/1", "NegOperator.apply/"),
            Self::Not => ("NotOperator.apply/1", "NotOperator.apply/"),
            Self::BitNot => ("BitNotOperator.apply/1", "BitNotOperator.apply/"),
        }
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
    let all_call = crate::config::env::builder_operator_box_all_call();
    if all_call {
        let (name, guard_prefix) = operator.operator_box();
        let in_guard = builder
            .function_state
            .current_function
            .as_ref()
            .map(|f| f.signature.name.starts_with(guard_prefix))
            .unwrap_or(false);
        let dst = builder.next_value_id();
        if !in_guard {
            builder.emit_legacy_call(
                Some(dst),
                super::super::CallTarget::Global(name.to_string()),
                vec![operand_val],
            )?;
            builder
                .function_state
                .type_ctx
                .value_types
                .insert(dst, return_type.clone());
            return Ok(dst);
        }
    }
    // Core-13 純化: UnaryOp を直接 展開（Neg/Not/BitNot）
    if crate::config::env::mir_core13_pure() {
        match operator {
            PreparedRawOrdinaryUnaryOperatorV1::Minus => {
                let zero = crate::mir::builder::emission::constant::emit_integer(builder, 0)?;
                let dst = builder.next_value_id();
                builder.emit_instruction(MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::Sub,
                    lhs: zero,
                    rhs: operand_val,
                })?;
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(dst, return_type.clone());
                return Ok(dst);
            }
            PreparedRawOrdinaryUnaryOperatorV1::Not => {
                let f = crate::mir::builder::emission::constant::emit_bool(builder, false)?;
                let dst = builder.next_value_id();
                crate::mir::builder::emission::compare::emit_to(
                    builder,
                    dst,
                    crate::mir::CompareOp::Eq,
                    operand_val,
                    f,
                )?;
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(dst, return_type.clone());
                return Ok(dst);
            }
            PreparedRawOrdinaryUnaryOperatorV1::BitNot => {
                let all1 = crate::mir::builder::emission::constant::emit_integer(builder, -1)?;
                let dst = builder.next_value_id();
                builder.emit_instruction(MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::BitXor,
                    lhs: operand_val,
                    rhs: all1,
                })?;
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(dst, return_type.clone());
                return Ok(dst);
            }
        }
    }
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
            route(PreparedRawUnaryV1::prepare(UnaryOperator::Minus, operand())),
            route(PreparedRawUnaryV1::prepare(UnaryOperator::Not, operand())),
            route(PreparedRawUnaryV1::prepare(
                UnaryOperator::BitNot,
                operand(),
            )),
        ];
        assert_eq!(routes, ["weak", "minus", "not", "bit-not"]);
    }
}
