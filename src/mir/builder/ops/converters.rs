//! AST → MIR Operator Converters
//!
//! This module provides pure functions for converting AST operator representations
//! to their corresponding MIR enum types.
//!
//! ## Purpose
//!
//! - Pure AST → MIR operator enum conversion (no MirBuilder state required)
//! - Convert AST BinaryOperator to MIR BinaryOpType (arithmetic/comparison)
//! - Convert AST unary operator strings to MIR UnaryOp enum
//! - Error handling for unknown/unsupported operators
//!
//! ## Responsibilities
//!
//! - **Binary operators**: Maps AST BinaryOperator enum to MIR BinaryOpType
//!   - Arithmetic: Add, Sub, Mul, Div, Mod, Shl, Shr, BitAnd, BitOr, BitXor
//!   - Comparison: Eq, Ne, Lt, Le, Gt, Ge
//!   - Logical: And, Or (classified as Arithmetic for MIR purposes)
//!
//! - **Unary operators**: Maps string operators to MIR UnaryOp enum
//!   - Neg: "-"
//!   - Not: "!" or "not"
//!   - BitNot: "~"
//!
//! - **Error handling**: Returns descriptive errors for unsupported operators
//!
//! ## Design Notes
//!
//! These are **pure functions** with no side effects or dependencies on MirBuilder state.
//! They perform only enum-to-enum conversions and string matching.

use crate::ast::BinaryOperator;
use crate::mir::{BinaryOp, CompareOp, UnaryOp};

/// Internal classification for binary operations
#[derive(Debug)]
pub(in crate::mir::builder) enum BinaryOpType {
    Arithmetic(BinaryOp),
    Comparison(CompareOp),
}

/// Convert AST binary operator to MIR BinaryOpType
///
/// Maps AST BinaryOperator enum to either an arithmetic operation (BinaryOp)
/// or a comparison operation (CompareOp).
///
/// ## Arguments
///
/// - `op`: The AST binary operator to convert
///
/// ## Returns
///
/// - `Ok(BinaryOpType)`: The corresponding MIR operation type
/// - `Err(String)`: Error message (currently all operators are supported)
pub(in crate::mir::builder) fn convert_binary_operator(
    op: BinaryOperator,
) -> Result<BinaryOpType, String> {
    match op {
        BinaryOperator::Add => Ok(BinaryOpType::Arithmetic(BinaryOp::Add)),
        BinaryOperator::Subtract => Ok(BinaryOpType::Arithmetic(BinaryOp::Sub)),
        BinaryOperator::Multiply => Ok(BinaryOpType::Arithmetic(BinaryOp::Mul)),
        BinaryOperator::Divide => Ok(BinaryOpType::Arithmetic(BinaryOp::Div)),
        BinaryOperator::Modulo => Ok(BinaryOpType::Arithmetic(BinaryOp::Mod)),
        BinaryOperator::Shl => Ok(BinaryOpType::Arithmetic(BinaryOp::Shl)),
        BinaryOperator::Shr => Ok(BinaryOpType::Arithmetic(BinaryOp::Shr)),
        BinaryOperator::BitAnd => Ok(BinaryOpType::Arithmetic(BinaryOp::BitAnd)),
        BinaryOperator::BitOr => Ok(BinaryOpType::Arithmetic(BinaryOp::BitOr)),
        BinaryOperator::BitXor => Ok(BinaryOpType::Arithmetic(BinaryOp::BitXor)),
        BinaryOperator::Equal => Ok(BinaryOpType::Comparison(CompareOp::Eq)),
        BinaryOperator::NotEqual => Ok(BinaryOpType::Comparison(CompareOp::Ne)),
        BinaryOperator::Less => Ok(BinaryOpType::Comparison(CompareOp::Lt)),
        BinaryOperator::LessEqual => Ok(BinaryOpType::Comparison(CompareOp::Le)),
        BinaryOperator::Greater => Ok(BinaryOpType::Comparison(CompareOp::Gt)),
        BinaryOperator::GreaterEqual => Ok(BinaryOpType::Comparison(CompareOp::Ge)),
        BinaryOperator::And => Ok(BinaryOpType::Arithmetic(BinaryOp::And)),
        BinaryOperator::Or => Ok(BinaryOpType::Arithmetic(BinaryOp::Or)),
    }
}

/// Convert AST unary operator string to MIR UnaryOp enum
///
/// Maps string representations of unary operators to the corresponding MIR UnaryOp enum.
///
/// ## Supported Operators
///
/// - `"-"`: Negation (Neg)
/// - `"!"` or `"not"`: Logical NOT (Not)
/// - `"~"`: Bitwise NOT (BitNot)
///
/// ## Arguments
///
/// - `op`: The operator string to convert
///
/// ## Returns
///
/// - `Ok(UnaryOp)`: The corresponding MIR unary operation
/// - `Err(String)`: Error message for unsupported operators
pub(in crate::mir::builder) fn convert_unary_operator(op: String) -> Result<UnaryOp, String> {
    match op.as_str() {
        "-" => Ok(UnaryOp::Neg),
        "!" | "not" => Ok(UnaryOp::Not),
        "~" => Ok(UnaryOp::BitNot),
        _ => Err(format!("Unsupported unary operator: {}", op)),
    }
}
