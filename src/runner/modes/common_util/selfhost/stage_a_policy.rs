/*!
 * Stage-A runtime route policy helpers.
 *
 * RNR-04 (BoxShape):
 * - Keep `route_orchestrator` focused on backend execution routing.
 * - Keep Stage-A payload/compat policy decisions under selfhost runtime helpers.
 */

use super::runtime_route_contract;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StageACompatPolicy {
    Allowed,
    RejectNonStrictWithoutFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StageACompatGuardAction {
    Allow,
    Reject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StageACompatGuardPlan {
    pub(crate) action: StageACompatGuardAction,
    pub(crate) reason: &'static str,
}

pub(crate) fn decide_stage_a_compat_policy(
    strict_enabled: bool,
    fallback_enabled: bool,
) -> StageACompatPolicy {
    if strict_enabled || fallback_enabled {
        StageACompatPolicy::Allowed
    } else {
        StageACompatPolicy::RejectNonStrictWithoutFallback
    }
}

pub(crate) fn decide_stage_a_compat_guard(
    strict_enabled: bool,
    fallback_enabled: bool,
) -> StageACompatGuardPlan {
    if decide_stage_a_compat_policy(strict_enabled, fallback_enabled) == StageACompatPolicy::Allowed
    {
        let reason = if strict_enabled {
            "strict:joinir-dev"
        } else {
            "env:NYASH_VM_USE_FALLBACK=1"
        };
        StageACompatGuardPlan {
            action: StageACompatGuardAction::Allow,
            reason,
        }
    } else {
        StageACompatGuardPlan {
            action: StageACompatGuardAction::Reject,
            reason: "require:NYASH_VM_USE_FALLBACK=1",
        }
    }
}

pub(crate) fn enforce_stage_a_compat_policy_or_exit(source: &str) {
    // Stage-A compat is explicit-only in non-strict mode.
    // Keep this as a narrow gate; new runtime features must not widen it implicitly.
    let strict_enabled = crate::config::env::joinir_dev::strict_enabled();
    let fallback_enabled = crate::config::env::vm_use_fallback();
    let plan = decide_stage_a_compat_guard(strict_enabled, fallback_enabled);
    if plan.action == StageACompatGuardAction::Reject {
        runtime_route_contract::emit_expected_mir_non_strict_compat_disabled(source);
        std::process::exit(1);
    }
}

pub(crate) fn enforce_stage_a_program_payload_policy_or_exit(source: &str) {
    if crate::config::env::joinir_dev::strict_planner_required_enabled() {
        runtime_route_contract::emit_expected_mir_strict_planner_required(source);
        std::process::exit(1);
    }
}

pub(crate) fn is_stage_a_rust_json_bridge_allowed(force_fallback: bool) -> bool {
    force_fallback
}

pub(crate) fn decide_stage_a_rust_json_bridge_guard(force_fallback: bool) -> StageACompatGuardPlan {
    if is_stage_a_rust_json_bridge_allowed(force_fallback) {
        StageACompatGuardPlan {
            action: StageACompatGuardAction::Allow,
            reason: "env:NYASH_VM_USE_FALLBACK=1",
        }
    } else {
        StageACompatGuardPlan {
            action: StageACompatGuardAction::Reject,
            reason: "require:NYASH_VM_USE_FALLBACK=1",
        }
    }
}

pub(crate) fn enforce_stage_a_rust_json_bridge_guard_or_exit(source: &str) {
    // The Rust Program(JSON v0) bridge is compat fallback only.
    // Mainline route additions must not silently make this default.
    let fallback_enabled = crate::config::env::vm_use_fallback();
    let plan = decide_stage_a_rust_json_bridge_guard(fallback_enabled);
    if plan.action == StageACompatGuardAction::Reject {
        runtime_route_contract::emit_freeze_compat_rust_json_v0_bridge(source);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decide_stage_a_compat_policy_non_strict_without_fallback_rejects() {
        let policy = decide_stage_a_compat_policy(false, false);
        assert_eq!(policy, StageACompatPolicy::RejectNonStrictWithoutFallback);
    }

    #[test]
    fn decide_stage_a_compat_policy_non_strict_with_fallback_allows() {
        let policy = decide_stage_a_compat_policy(false, true);
        assert_eq!(policy, StageACompatPolicy::Allowed);
    }

    #[test]
    fn decide_stage_a_compat_policy_strict_allows() {
        let policy = decide_stage_a_compat_policy(true, false);
        assert_eq!(policy, StageACompatPolicy::Allowed);
    }

    #[test]
    fn decide_stage_a_compat_guard_rejects_without_strict_or_fallback() {
        let plan = decide_stage_a_compat_guard(false, false);
        assert_eq!(plan.action, StageACompatGuardAction::Reject);
        assert_eq!(plan.reason, "require:NYASH_VM_USE_FALLBACK=1");
    }

    #[test]
    fn decide_stage_a_compat_guard_allows_in_strict_mode() {
        let plan = decide_stage_a_compat_guard(true, false);
        assert_eq!(plan.action, StageACompatGuardAction::Allow);
        assert_eq!(plan.reason, "strict:joinir-dev");
    }

    #[test]
    fn stage_a_rust_json_bridge_allowed_only_when_explicit() {
        assert!(!is_stage_a_rust_json_bridge_allowed(false));
        assert!(is_stage_a_rust_json_bridge_allowed(true));
    }

    #[test]
    fn decide_stage_a_rust_json_bridge_guard_rejects_without_flag() {
        let plan = decide_stage_a_rust_json_bridge_guard(false);
        assert_eq!(plan.action, StageACompatGuardAction::Reject);
        assert_eq!(plan.reason, "require:NYASH_VM_USE_FALLBACK=1");
    }

    #[test]
    fn decide_stage_a_rust_json_bridge_guard_allows_with_flag() {
        let plan = decide_stage_a_rust_json_bridge_guard(true);
        assert_eq!(plan.action, StageACompatGuardAction::Allow);
        assert_eq!(plan.reason, "env:NYASH_VM_USE_FALLBACK=1");
    }
}
