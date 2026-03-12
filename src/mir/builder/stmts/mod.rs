//! Statement Processing Orchestrator Module
//!
//! **Purpose**: Coordinate statement execution through specialized submodules
//!
//! ## Architecture Overview
//!
//! This module serves as the orchestrator for all statement-level MIR building,
//! delegating to 5 specialized submodules organized by single responsibility:
//!
//! ```text
//! stmts/ (681 lines → 5 modules)
//! ├── mod.rs              (63 lines)  - Orchestrator with wrapper methods
//! ├── async_stmt.rs      (132 lines)  - Async operations (nowait/await)
//! ├── block_stmt.rs      (185 lines)  - Block execution + JoinIR suffix router
//! ├── print_stmt.rs      (195 lines)  - Print I/O with TypeOp support
//! ├── return_stmt.rs     (218 lines)  - Return + match-return optimization
//! └── variable_stmt.rs   (224 lines)  - Variable lifecycle (local/me)
//! ```
//!
//! ## Design Pattern: Orchestrator + Delegation
//!
//! This module follows the **wrapper pattern** seen in `lifecycle.rs`:
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
//! pub(super) fn build_block(&mut self, statements: Vec<ASTNode>) -> Result<ValueId, String> {
//!     block_stmt::build_block(self, statements)  // Delegate to specialist
//! }
//! ```
//!
//! ## Module Responsibilities
//!
//! ### 1. async_stmt.rs - Async Operations
//! - **Purpose**: Handle async operations (nowait/await)
//! - **Key Functions**:
//!   - `build_nowait_statement` - Spawn async tasks
//!   - `build_await_expression` - Wait for Future completion
//! - **Phase Context**: Phase 84 Future type registration
//!
//! ### 2. block_stmt.rs - Block Execution
//! - **Purpose**: Sequential statement execution with JoinIR integration
//! - **Key Functions**:
//!   - `build_block` - Process statement sequences
//!   - `build_statement` - Dispatch single statements
//! - **Critical**: Phase 142 JoinIR suffix router integration point
//!
//! ### 3. print_stmt.rs - Print I/O
//! - **Purpose**: Print statement handling with early TypeOp detection
//! - **Key Functions**:
//!   - `build_print_statement` - Print with TypeOp optimization
//! - **Features**: isType/asType pattern detection, unified call support
//!
//! ### 4. return_stmt.rs - Return Handling
//! - **Purpose**: Return statement with match-return optimization
//! - **Key Functions**:
//!   - `build_return_statement` - Return with CorePlan optimization
//! - **Features**: Match-return optimization, defer mechanism
//!
//! ### 5. variable_stmt.rs - Variable Lifecycle
//! - **Purpose**: Variable declaration and receiver resolution
//! - **Key Functions**:
//!   - `build_local_statement` - Local variable declaration
//!   - `build_me_expression` - Receiver resolution (me/this)
//! - **Phase Context**: Phase 135 ValueId allocation, Phase 269 Fail-Fast
//!
//! ## Benefits of This Architecture
//!
//! 1. **Single Responsibility**: Each module has one clear purpose
//! 2. **Modularity**: Easy to locate and modify specific statement handling
//! 3. **Testability**: Modules can be tested independently
//! 4. **Maintainability**: Changes isolated to affected modules
//! 5. **Discoverability**: Clear naming makes intent obvious
//!
//! ## Similar Patterns in Codebase
//!
//! This follows the same pattern as:
//! - `lifecycle.rs` - Orchestrator + 4 specialized modules (623 → 4 files)
//! - Both are part of Phase 29bq+ cleanliness campaign
//!
//! ## Integration Points
//!
//! - **Called by**: MirBuilder expression/statement building
//! - **Calls**: Specialized statement builders in submodules
//! - **JoinIR Integration**: block_stmt.rs contains Phase 142 suffix router
//!
//! ## Phase Context
//!
//! - **Phase 29bq+**: Cleanliness campaign - large file modularization
//! - **Refactoring**: 681-line stmts.rs → 5 specialized modules
//! - **Preserved**: All Phase comments, functionality, JoinIR integration

pub(super) mod async_stmt;
pub(super) mod block_stmt;
pub(super) mod print_stmt;
pub(super) mod return_stmt;
pub(super) mod variable_stmt;

use super::ValueId;
use crate::ast::ASTNode;

impl super::MirBuilder {
    // ========== Block Execution ==========

    /// Build a block by sequentially processing statements
    ///
    /// **Delegates to**: `block_stmt::build_block`
    ///
    /// This is the main entry point for block execution, which integrates
    /// with the Phase 142 JoinIR suffix router for pattern detection.
    pub(super) fn build_block(&mut self, statements: Vec<ASTNode>) -> Result<ValueId, String> {
        block_stmt::build_block(self, statements)
    }

    /// Build a single statement node
    ///
    /// **Delegates to**: `block_stmt::build_statement`
    ///
    /// Handles statement-level If, While, ForRange, and delegates other
    /// expressions to build_expression.
    pub(super) fn build_statement(&mut self, node: ASTNode) -> Result<ValueId, String> {
        block_stmt::build_statement(self, node)
    }

    /// Phase 212.5: Statement としての If 処理（副作用のみ）
    ///
    /// ループ内 if や top-level statement if はここを通る。
    /// Expression としての if（値を使う場合）は build_expression 経由。
    ///
    /// # Arguments
    /// * `condition` - If の条件式
    /// * `then_body` - then ブロックの statements
    /// * `else_body` - else ブロックの statements (optional)
    ///
    /// # Example
    /// ```hako
    /// if i > 0 {
    ///     sum = sum + 1  // ← Statement としての If
    /// }
    /// ```
    pub(super) fn build_if_statement(
        &mut self,
        condition: ASTNode,
        then_body: Vec<ASTNode>,
        else_body: Option<Vec<ASTNode>>,
    ) -> Result<(), String> {
        use crate::ast::Span;

        // then_body と else_body を ASTNode::Program に変換
        let then_node = ASTNode::Program {
            statements: then_body,
            span: Span::unknown(),
        };
        let else_node = else_body.map(|b| ASTNode::Program {
            statements: b,
            span: Span::unknown(),
        });

        // 既存の If lowering を呼ぶ（cf_if は lower_if_form を呼ぶ）
        // 戻り値は無視（Statement なので値は使わない）
        let _result = self.cf_if(condition, then_node, else_node)?;

        Ok(())
    }
}
