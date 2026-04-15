//! Compatibility wrapper for top-level facts statement-walk helpers.

#[allow(unused_imports)]
pub(crate) use crate::mir::builder::control_flow::facts::stmt_walk::{
    flatten_stmt_list, strip_trailing_continue_view, walk_stmt_list,
};
