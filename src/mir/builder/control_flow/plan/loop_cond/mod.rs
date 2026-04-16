//! Loop-condition pattern variants (SSOT)
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! shallowing: moved from loop_cond_unified/variants/

// Re-export unified helpers (moved to parent directory)
pub(in crate::mir::builder) use super::loop_cond_unified_helpers;

// break_continue variant
pub(in crate::mir::builder) mod break_continue_accept;
pub(in crate::mir::builder) mod break_continue_classify;
pub(in crate::mir::builder) mod break_continue_entry;
pub(in crate::mir::builder) mod break_continue_facts;
pub(in crate::mir::builder) mod break_continue_helpers;
pub(in crate::mir::builder) mod break_continue_item;
pub(in crate::mir::builder) mod break_continue_tree;
pub(in crate::mir::builder) mod break_continue_validator_cond;
pub(in crate::mir::builder) mod break_continue_validator_else;
pub(in crate::mir::builder) mod break_continue_validator_exit;
pub(in crate::mir::builder) mod break_continue_validator_prelude;

pub(super) const MAX_NESTED_LOOPS: usize = 8;

// true_break_continue variant (flat file)
pub(in crate::mir::builder) mod true_break_continue;
pub(in crate::mir::builder) mod true_break_continue_helpers;
