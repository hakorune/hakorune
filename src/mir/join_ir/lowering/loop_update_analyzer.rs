//! Loop Update Expression Analyzer
//!
//! Phase 197: Extracts update expressions from loop body to generate
//! correct carrier update semantics.
//!
//! # Purpose
//!
//! The `loop_continue_only` lowerer needs to know how each carrier variable is updated
//! in the loop body. Instead of hardcoding "count uses +1, sum uses +i",
//! we extract the actual update expressions from the AST.
//!
//! # Example
//!
//! ```nyash
//! loop(i < 10) {
//!   i = i + 1         // UpdateExpr::BinOp { lhs: "i", op: Add, rhs: Const(1) }
//!   sum = sum + i     // UpdateExpr::BinOp { lhs: "sum", op: Add, rhs: "i" }
//!   count = count + 1 // UpdateExpr::BinOp { lhs: "count", op: Add, rhs: Const(1) }
//! }
//! ```

use crate::mir::join_ir::BinOpKind;

/// Update expression for a carrier variable
#[derive(Debug, Clone)]
pub enum UpdateExpr {
    /// Constant increment: carrier = carrier + N
    Const(i64),
    /// Binary operation: carrier = carrier op rhs
    BinOp {
        lhs: String,
        op: BinOpKind,
        rhs: UpdateRhs,
    },
}

/// Right-hand side of update expression
///
/// Phase 178: Extended to detect string updates for multi-carrier loops.
/// Phase 190: Extended to detect number accumulation pattern (result = result * base + digit).
/// The goal is "carrier detection", not full semantic understanding.
#[derive(Debug, Clone)]
pub enum UpdateRhs {
    /// Numeric constant: count + 1
    Const(i64),
    /// Variable reference: sum + i
    Variable(String),
    /// Phase 178: String literal: result + "x"
    StringLiteral(String),
    /// Phase 190: Number accumulation pattern: result = result * base + digit
    /// Represents expressions like: result * 10 + digit
    NumberAccumulation {
        /// Multiplication base (e.g., 10 for decimal, 2 for binary)
        base: i64,
        /// Variable name containing the digit to add
        digit_var: String,
    },
    /// Phase 178: Other expression (method call, complex expr)
    /// Used to detect "carrier has an update" without understanding semantics
    Other,
}
