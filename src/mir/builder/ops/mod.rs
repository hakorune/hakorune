//! Operator Building Orchestrator Module
//!
//! **Purpose**: Coordinate operator lowering through specialized submodules
//!
//! ## Architecture Overview
//!
//! This module serves as the orchestrator for all operator-level MIR building,
//! delegating to specialized semantic modules and live associated-input
//! descent owners organized by single responsibility:
//!
//! ```text
//! ops/ (587 lines → 1,098 lines with documentation)
//! ├── mod.rs                              - Orchestrator with wrapper methods
//! ├── binary_expression_descent.rs        - Associated-input ordinary Binary descent
//! ├── converters.rs            (105 lines)  - AST → MIR operator conversion
//! ├── arithmetic.rs            (287 lines)  - Arithmetic ops (Add, Sub, Mul, etc.)
//! ├── comparison.rs            (130 lines)  - Comparison ops (Eq, Lt, Ge, etc.)
//! ├── logical_shortcircuit.rs  (169 lines)  - Logical ops (&&, ||)
//! └── unary.rs                 (211 lines)  - Unary ops (-, !, ~)
//! ```
//!
//! ## Design Pattern: Owner + Delegation
//!
//! The raw expression dispatcher partitions source operators before entering
//! this module's associated-input owners. Completion then delegates to the
//! specialized semantic modules:
//!
//! 1. **Raw/default partition**: `ASTNode::BinaryOp` selects ordinary or
//!    short-circuit descent exactly once.
//! 2. **Associated-input owners**: demand children in the family-specific
//!    order.
//! 3. **Semantic completion**: arithmetic, comparison, and short-circuit
//!    owners emit the MIR result.
//!
//! ## Module Responsibilities
//!
//! ### 1. converters.rs - AST → MIR Conversion
//! - **Purpose**: Pure AST → MIR operator enum conversion
//! - **Key Functions**:
//!   - `convert_binary_operator` - BinaryOperator → BinaryOpType
//!   - `convert_unary_operator` - String → UnaryOp
//! - **No side effects**: Pure conversion only
//!
//! ### 2. arithmetic.rs - Arithmetic Operations
//! - **Purpose**: Handle arithmetic binary operations
//! - **Operations**: Add, Sub, Mul, Div, Mod, Shl, Shr, BitAnd, BitOr, BitXor
//! - **Key Features**:
//!   - Operator Box routing (AddOperator, SubOperator, etc.)
//!   - Type facts classification (String vs Integer for Add)
//!   - Core-13 pure expansion (ssot::binop_lower)
//! - **Phase Context**: Phase 2.11 Core-13 pure BinOp
//!
//! ### 3. comparison.rs - Comparison Operations
//! - **Purpose**: Handle comparison operations
//! - **Operations**: Eq, Ne, Lt, Le, Gt, Ge
//! - **Key Features**:
//!   - Operator Box routing (CompareOperator.apply/3)
//!   - IntegerBox cast detection and TypeOp insertion
//!   - LocalSSA finalization (ensure_local_ssa)
//! - **Phase Context**: Phase 2.11 Core-13 pure Compare
//!
//! ### 4. logical_shortcircuit.rs - Logical Operations
//! - **Purpose**: Logical short-circuit evaluation
//! - **Operations**: && (And), || (Or)
//! - **Key Features**:
//!   - 3-predecessor merge (skip/rhs_true/rhs_false)
//!   - Variable map snapshotting and merging
//!   - PHI construction for result value
//!   - Control-flow lowering (not simple BinOp!)
//! - **Phase Context**: Phase 142 JoinIR suffix router integration
//!
//! ### 5. unary.rs - Unary Operations
//! - **Purpose**: Handle unary operations
//! - **Operations**: - (Neg), ! (Not), ~ (BitNot)
//! - **Key Features**:
//!   - Operator Box routing (NegOperator, NotOperator, BitNotOperator)
//!   - Core-13 pure expansion:
//!     - Neg: `Sub 0-x`
//!     - Not: `Compare Eq x-false`
//!     - BitNot: `XOR x-(-1)`
//!   - Guard detection (prevent infinite recursion)
//! - **Phase Context**: Phase 2.11 Core-13 pure UnaryOp
//!
//! ## Benefits of This Architecture
//!
//! 1. **Single Responsibility**: Each module has one operator domain
//! 2. **Improved Testability**: Independent module testing
//! 3. **Better Maintainability**: Changes isolated to responsible module
//! 4. **Enhanced Discoverability**: Clear naming and navigation
//! 5. **Type Safety**: Type facts integration clearly isolated
//!
//! ## Similar Patterns in Codebase
//!
//! This follows the same pattern as:
//! - `lifecycle.rs` - Orchestrator + 4 specialized modules (623 → 4 files)
//! - `stmts.rs` - Orchestrator + 5 specialized modules (681 → 5 files)
//! - All part of Phase 29bq+ cleanliness campaign
//!
//! ## Integration Points
//!
//! - **Called by**: MirBuilder expression building
//! - **Calls**: Specialized operator builders in submodules
//! - **Type Facts**: arithmetic.rs and comparison.rs integrate with type_facts module
//! - **JoinIR**: logical_shortcircuit.rs creates control-flow with PHI nodes
//!
//! ## Phase Context
//!
//! - **Phase 29bq+**: Cleanliness campaign - large file modularization
//! - **Refactoring**: 587-line ops.rs → 5 specialized modules
//! - **Preserved**: All Phase comments, functionality, Core-13 pure expansion

use super::ValueId;
use crate::ast::{ASTNode, BinaryOperator};

pub(super) mod arithmetic;
mod binary_expression_descent;
pub(in crate::mir::builder) use binary_expression_descent::{
    drive_ordinary_binary_expression_v1, BinaryExpressionDescentPortV1, BinarySyntaxViewV1,
    RawLegacyBinaryInputV1,
};
#[cfg(test)]
mod binary_expression_descent_tests;
#[cfg(test)]
mod binary_expression_parity_tests;
#[cfg(test)]
mod binary_expression_raw_tests;
pub(super) mod comparison;
pub(super) mod converters;
pub(super) mod logical_shortcircuit;
mod short_circuit_expression_descent;
pub(in crate::mir::builder) use short_circuit_expression_descent::{
    drive_short_circuit_expression_v1, RawLegacyShortCircuitInputV1,
    ShortCircuitExpressionDescentPortV1, ShortCircuitSyntaxViewV1,
};
#[cfg(test)]
mod short_circuit_expression_descent_tests;
#[cfg(test)]
mod short_circuit_expression_parity_tests;
#[cfg(test)]
mod short_circuit_expression_raw_tests;
pub(super) mod unary;
use converters::BinaryOpType;

impl super::MirBuilder {
    pub(in crate::mir::builder) fn build_binary_op_from_values(
        &mut self,
        operator: BinaryOperator,
        lhs_raw: ValueId,
        rhs_raw: ValueId,
    ) -> Result<ValueId, String> {
        let mir_op = converters::convert_binary_operator(operator)?;

        match mir_op {
            // Arithmetic operations
            BinaryOpType::Arithmetic(op) => {
                let lhs = crate::mir::builder::ssa::local::arg(self, lhs_raw);
                let rhs = crate::mir::builder::ssa::local::arg(self, rhs_raw);
                arithmetic::build_arithmetic_op(self, op, lhs, rhs)
            }
            // Comparison operations
            BinaryOpType::Comparison(op) => self.build_comparison_op(op, lhs_raw, rhs_raw),
        }
    }

    /// Build a unary operation
    ///
    /// **Delegates to**: `unary::build_unary_op`
    ///
    /// This handles all unary operations (-, !, ~) by delegating to the
    /// specialized unary module, which implements Core-13 pure expansion:
    /// - Neg (-): Lowered to `Sub 0-x`
    /// - Not (!): Lowered to `Compare Eq x-false`
    /// - BitNot (~): Lowered to `XOR x-(-1)`
    pub(super) fn build_unary_op(
        &mut self,
        operator: String,
        operand: ASTNode,
    ) -> Result<ValueId, String> {
        unary::build_unary_op(self, operator, operand)
    }
}
