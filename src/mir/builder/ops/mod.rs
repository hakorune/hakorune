//! Operator Building Orchestrator Module
//!
//! **Purpose**: Coordinate operator lowering through specialized submodules
//!
//! ## Architecture Overview
//!
//! This module serves as the orchestrator for all operator-level MIR building,
//! delegating to 5 specialized submodules organized by single responsibility:
//!
//! ```text
//! ops/ (587 lines → 1,098 lines with documentation)
//! ├── mod.rs                   (196 lines)  - Orchestrator with wrapper methods
//! ├── converters.rs            (105 lines)  - AST → MIR operator conversion
//! ├── arithmetic.rs            (287 lines)  - Arithmetic ops (Add, Sub, Mul, etc.)
//! ├── comparison.rs            (130 lines)  - Comparison ops (Eq, Lt, Ge, etc.)
//! ├── logical_shortcircuit.rs  (169 lines)  - Logical ops (&&, ||)
//! └── unary.rs                 (211 lines)  - Unary ops (-, !, ~)
//! ```
//!
//! ## Design Pattern: Orchestrator + Delegation
//!
//! This module follows the **wrapper pattern** seen in `stmts.rs` and `lifecycle.rs`:
//!
//! 1. **Orchestrator (mod.rs)**: Thin wrapper methods on MirBuilder
//! 2. **Specialized Modules**: Free functions that take `&mut MirBuilder`
//! 3. **No Circular Dependencies**: All modules import from parent builder
//!
//! ### Wrapper Methods
//!
//! Each public method in `impl MirBuilder` is a thin wrapper that delegates
//! to the appropriate specialized module:
//!
//! ```rust
//! // Orchestrator wrapper in mod.rs
//! pub(super) fn build_binary_op(...) -> Result<ValueId, String> {
//!     // 1. Convert AST operator to MIR operator enum
//!     let mir_op = converters::convert_binary_operator(operator)?;
//!
//!     // 2. Delegate to specialist module based on operator type
//!     match mir_op {
//!         BinaryOpType::Arithmetic(op) => arithmetic::build_arithmetic_op(self, op, lhs, rhs),
//!         BinaryOpType::Comparison(op) => self.build_comparison_op(op, lhs, rhs),
//!     }
//! }
//! ```
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
pub(super) mod comparison;
pub(super) mod converters;
pub(super) mod logical_shortcircuit;
pub(super) mod unary;
use converters::BinaryOpType;

impl super::MirBuilder {
    /// Build a binary operation
    ///
    /// **Delegates to**: Specialized operator modules based on operator type
    ///
    /// This is the main entry point for binary operator lowering, which routes
    /// to different specialist modules:
    /// - Logical operators (&&, ||) → `logical_shortcircuit::build_logical_shortcircuit`
    /// - Arithmetic operators → `arithmetic::build_arithmetic_op`
    /// - Comparison operators → `comparison::build_comparison_op`
    ///
    /// **Note**: Logical operators use control-flow lowering with PHI nodes,
    /// not simple BinOp instructions, to implement short-circuit evaluation.
    pub(super) fn build_binary_op(
        &mut self,
        left: ASTNode,
        operator: BinaryOperator,
        right: ASTNode,
    ) -> Result<ValueId, String> {
        // Short-circuit logical ops: lower to control-flow so RHS is evaluated conditionally
        if matches!(operator, BinaryOperator::And | BinaryOperator::Or) {
            return logical_shortcircuit::build_logical_shortcircuit(self, left, operator, right);
        }

        let lhs_raw = self.build_expression(left)?;
        let rhs_raw = self.build_expression(right)?;

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
