//! Validation functions for planner invariants
//!
//! Extracted from build.rs for better maintainability

use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    CleanupKindFacts, ExitKindFacts,
};
use std::collections::BTreeSet;

#[cfg(debug_assertions)]
pub(super) fn debug_assert_cleanup_kinds_match_exit_kinds(
    cleanup_kinds_present: &BTreeSet<CleanupKindFacts>,
    exit_kinds_present: &BTreeSet<ExitKindFacts>,
) {
    for cleanup_kind in cleanup_kinds_present {
        let exit_kind = match cleanup_kind {
            CleanupKindFacts::Return => ExitKindFacts::Return,
            CleanupKindFacts::Break => ExitKindFacts::Break,
            CleanupKindFacts::Continue => ExitKindFacts::Continue,
        };
        debug_assert!(
            exit_kinds_present.contains(&exit_kind),
            "cleanup kind requires matching exit kind presence"
        );
    }
}

#[cfg(not(debug_assertions))]
pub(super) fn debug_assert_cleanup_kinds_match_exit_kinds(
    _cleanup_kinds_present: &BTreeSet<CleanupKindFacts>,
    _exit_kinds_present: &BTreeSet<ExitKindFacts>,
) {
}
