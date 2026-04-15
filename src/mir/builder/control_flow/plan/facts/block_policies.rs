//! Shared policies for Facts recipe builders (SSOT helper).
//!
//! Purpose:
//! - Avoid policy drift across `*_block.rs` recipe builders.
//! - Keep "allow_extended" gates centralized for small vocabulary decisions.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::cleanup::policies::cond_prelude_vocab::{
    classify_cond_prelude_stmt, CondPreludeStmtKind,
};

/// Shared effect-statement vocabulary policy for Facts recipe builders.
///
/// - Uses `cond_prelude_vocab` as the base allowlist.
/// - `Print` is allowed only when `allow_extended` is enabled.
pub(in crate::mir::builder) fn is_allowed_effect_stmt(
    stmt: &ASTNode,
    allow_extended: bool,
) -> bool {
    match classify_cond_prelude_stmt(stmt) {
        Some(CondPreludeStmtKind::Print) => allow_extended,
        Some(_) => true,
        None => false,
    }
}
