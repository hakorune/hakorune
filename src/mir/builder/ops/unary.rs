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
//! - Uses: `converters::convert_unary_operator` for operator conversion

use super::super::{MirInstruction, MirType, ValueId};
use crate::ast::ASTNode;

/// Build a unary operation.
///
/// This function handles unary operators (-, !, ~) with three possible execution paths:
///
/// 1. **Operator Box Routing** (when `NYASH_BUILDER_OPERATOR_BOX_ALL_CALL=1`):
///    - Routes to NegOperator/NotOperator/BitNotOperator
///    - Includes guard detection to prevent infinite recursion
///    - Sets appropriate return type (Integer or Bool)
///
/// 2. **Core-13 Pure Expansion** (when `mir_core13_pure()` is enabled):
///    - `-x` → `Sub(0, x)` (zero subtraction)
///    - `!x` → `Compare(Eq, x, false)` (equality with false)
///    - `~x` → `BitXor(x, -1)` (XOR with all-ones)
///
/// 3. **Direct UnaryOp Emission** (default path):
///    - Emits MIR UnaryOp instruction directly
///    - No type tracking in this path
///
/// # Arguments
///
/// * `builder` - The MIR builder instance
/// * `operator` - The unary operator string ("-", "!", "not", "~")
/// * `operand` - The operand AST node
///
/// # Returns
///
/// Returns the ValueId of the result, or an error message if the operator is invalid
/// or the operand expression fails to build.
///
/// # Example Usage
///
/// ```ignore
/// // From exprs.rs handling UnaryOp pattern:
/// let result = unary::build_unary_op(self, "-".to_string(), operand_ast)?;
/// ```
///
/// # Guard Detection
///
/// To prevent infinite recursion when operator boxes call themselves internally,
/// we check if the current function name starts with the operator's guard prefix:
/// - `NegOperator.apply/` for negation
/// - `NotOperator.apply/` for logical NOT
/// - `BitNotOperator.apply/` for bitwise NOT
///
/// If inside a guard, we fall through to direct UnaryOp emission.
pub(super) fn build_unary_op(
    builder: &mut super::super::MirBuilder,
    operator: String,
    operand: ASTNode,
) -> Result<ValueId, String> {
    let return_type = match operator.as_str() {
        "-" | "~" => MirType::Integer,
        "!" | "not" => MirType::Bool,
        _ => return Err(format!("Unsupported unary operator: {}", operator)),
    };
    if operator == "-" {
        if let ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(n),
            ..
        } = &operand
        {
            if let Some(negated) = n.checked_neg() {
                let dst = crate::mir::builder::emission::constant::emit_integer(builder, negated)?;
                builder.type_ctx.value_types.insert(dst, MirType::Integer);
                return Ok(dst);
            }
        }
    }
    let operand_val = builder.build_expression(operand)?;
    let all_call = crate::config::env::builder_operator_box_all_call();
    if all_call {
        let (name, guard_prefix) = match operator.as_str() {
            "-" => ("NegOperator.apply/1", "NegOperator.apply/"),
            "!" | "not" => ("NotOperator.apply/1", "NotOperator.apply/"),
            "~" => ("BitNotOperator.apply/1", "BitNotOperator.apply/"),
            _ => unreachable!("validated by return_type"),
        };
        let in_guard = builder
            .scope_ctx
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
                .type_ctx
                .value_types
                .insert(dst, return_type.clone());
            return Ok(dst);
        }
    }
    // Core-13 純化: UnaryOp を直接 展開（Neg/Not/BitNot）
    if crate::config::env::mir_core13_pure() {
        match operator.as_str() {
            "-" => {
                let zero = crate::mir::builder::emission::constant::emit_integer(builder, 0)?;
                let dst = builder.next_value_id();
                builder.emit_instruction(MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::Sub,
                    lhs: zero,
                    rhs: operand_val,
                })?;
                builder
                    .type_ctx
                    .value_types
                    .insert(dst, return_type.clone());
                return Ok(dst);
            }
            "!" | "not" => {
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
                    .type_ctx
                    .value_types
                    .insert(dst, return_type.clone());
                return Ok(dst);
            }
            "~" => {
                let all1 = crate::mir::builder::emission::constant::emit_integer(builder, -1)?;
                let dst = builder.next_value_id();
                builder.emit_instruction(MirInstruction::BinOp {
                    dst,
                    op: crate::mir::BinaryOp::BitXor,
                    lhs: operand_val,
                    rhs: all1,
                })?;
                builder
                    .type_ctx
                    .value_types
                    .insert(dst, return_type.clone());
                return Ok(dst);
            }
            _ => {}
        }
    }
    let dst = builder.next_value_id();
    let mir_op = super::converters::convert_unary_operator(operator)?;
    builder.emit_instruction(MirInstruction::UnaryOp {
        dst,
        op: mir_op,
        operand: operand_val,
    })?;
    builder.type_ctx.value_types.insert(dst, return_type);
    Ok(dst)
}
