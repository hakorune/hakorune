//! Phase 255 P2: Post-loop early return plan (policy-level SSOT)
//!
//! # Responsibility
//!
//! - Describe a post-loop guard that emulates an in-loop `return` without making
//!   loop-break lowering itself return-in-loop aware.
//! - Keep the plan policy-agnostic so multiple loop-break families can reuse it.
//!
//! # Architecture
//!
//! Ensures exit PHI values are used (prevents DCE elimination). The post-loop
//! guard creates a conditional that references the exit PHI value, forcing it
//! to be live and preventing dead code elimination.
//!
//! # Usage Patterns
//!
//! ## Loop-break route: Less Than (balanced_depth_scan)
//!
//! Used in `json_cur.find_balanced_*` family functions.
//!
//! ```ignore
//! PostLoopEarlyReturnPlan {
//!     cond: BinaryOp { Less, var("i"), var("n") },
//!     ret_expr: var("i"),
//! }
//! ```
//!
//! Generated post-loop guard:
//! ```nyash
//! if i < n {
//!     return i
//! }
//! ```
//!
//! ## Pattern 6: Not Equal (index_of)
//!
//! Used in `StringUtils.index_of` and similar search functions.
//!
//! ```ignore
//! PostLoopEarlyReturnPlan {
//!     cond: BinaryOp { NotEqual, var("i"), Literal(-1) },
//!     ret_expr: var("i"),
//! }
//! ```
//!
//! Generated post-loop guard:
//! ```nyash
//! if i != -1 {
//!     return i
//! }
//! ```
//!
//! # Builder Decision
//!
//! Deferred to Phase 256+. Currently 2 patterns use this (Pattern 2 and Pattern 6).
//! Direct construction is acceptable. Will create builder when 4+ patterns use it.

use crate::ast::ASTNode;

#[derive(Debug, Clone)]
pub struct PostLoopEarlyReturnPlan {
    /// Condition for the post-loop guard (e.g., `i < n` or `i != -1`)
    pub cond: ASTNode,
    /// Expression to return if condition is true (e.g., `var("i")`)
    pub ret_expr: ASTNode,
}
