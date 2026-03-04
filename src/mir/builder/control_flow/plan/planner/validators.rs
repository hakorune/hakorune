//! Validation functions for planner invariants
//!
//! Extracted from build.rs for better maintainability

#![allow(dead_code)]

use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    CleanupKindFacts, ExitKindFacts, ExitUsageFacts,
};
use crate::mir::builder::control_flow::plan::DomainPlan;
use std::collections::BTreeSet;

/// Check if strict or dev mode is enabled
#[inline]
pub(super) fn is_strict_or_dev_enabled() -> bool {
    crate::config::env::joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled()
}

#[cfg(debug_assertions)]
pub(super) fn debug_assert_exit_usage_matches_plan(
    _plan: &DomainPlan,
    exit_usage: &ExitUsageFacts,
    exit_kinds_present: &BTreeSet<ExitKindFacts>,
) {
    debug_assert_eq!(
        exit_usage.has_break,
        exit_kinds_present.contains(&ExitKindFacts::Break),
        "exit usage break presence mismatch"
    );
    debug_assert_eq!(
        exit_usage.has_continue,
        exit_kinds_present.contains(&ExitKindFacts::Continue),
        "exit usage continue presence mismatch"
    );
    debug_assert_eq!(
        exit_usage.has_return,
        exit_kinds_present.contains(&ExitKindFacts::Return),
        "exit usage return presence mismatch"
    );
    debug_assert_eq!(
        exit_usage.has_unwind,
        exit_kinds_present.contains(&ExitKindFacts::Unwind),
        "exit usage unwind presence mismatch"
    );
}

#[cfg(not(debug_assertions))]
pub(super) fn debug_assert_exit_usage_matches_plan(
    _plan: &DomainPlan,
    _exit_usage: &ExitUsageFacts,
    _exit_kinds_present: &BTreeSet<ExitKindFacts>,
) {
}

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
