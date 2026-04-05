/*!
 * Route orchestrator SSOT for runner-side lane selection.
 *
 * Phase 59x C1:
 * - Centralize explicit keep/reference selection for the `vm` backend family.
 * - Canonical internal lane names are `rust-vm-keep`, `vm-hako-reference`,
 *   and `vm-compat-fallback`.
 * - `vm-compat-fallback` stays compatibility-only.
 * - `vm` backend family is explicit legacy keep/debug only; it is not a
 *   day-to-day route.
 * - Day-to-day mainline stays on direct/core routes; this orchestrator owns only
 *   explicit keep/reference requests and must not silently widen back into a
 *   default owner path.
 */

use super::NyashRunner;

pub(crate) const VM_ROUTE_TAG_PRE_DISPATCH: &str = "vm-route/pre-dispatch";
pub(crate) const VM_ROUTE_TAG_SELECT: &str = "vm-route/select";
pub(crate) const VM_ROUTE_FREEZE_COMPAT_BYPASS: &str = "vm-route/compat-bypass";
pub(crate) const DERUST_ROUTE_TAG_SELECT: &str = "derust-route/select";
pub(crate) const VM_LANE_RUST_KEEP: &str = "rust-vm-keep";
pub(crate) const VM_LANE_HAKO_REFERENCE: &str = "vm-hako-reference";
pub(crate) const VM_LANE_COMPAT_FALLBACK: &str = "vm-compat-fallback";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VmRouteAction {
    Vm,
    VmHako,
    CompatFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VmRoutePlan {
    pub(crate) backend: &'static str,
    pub(crate) lane: &'static str,
    pub(crate) reason: &'static str,
    pub(crate) action: VmRouteAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VmCompatGuardAction {
    Allow,
    Reject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VmCompatGuardPlan {
    pub(crate) action: VmCompatGuardAction,
    pub(crate) reason: &'static str,
}

pub(crate) fn format_vm_route_pre_dispatch(backend: &str, file: &str) -> String {
    format!(
        "[{VM_ROUTE_TAG_PRE_DISPATCH}] backend={} file={}",
        backend, file
    )
}

pub(crate) fn emit_vm_route_pre_dispatch(backend: &str, file: &str) {
    if crate::config::env::vm_route_trace() {
        eprintln!("{}", format_vm_route_pre_dispatch(backend, file));
    }
}

pub(crate) fn format_vm_route_select(plan: &VmRoutePlan) -> String {
    format!(
        "[{VM_ROUTE_TAG_SELECT}] backend={} lane={} reason={}",
        plan.backend, plan.lane, plan.reason
    )
}

fn emit_vm_route_select(plan: &VmRoutePlan) {
    if crate::config::env::vm_route_trace() {
        eprintln!("{}", format_vm_route_select(plan));
    }
}

pub(crate) fn format_derust_route_select(
    backend: &str,
    lane: &str,
    source: &str,
    reason: &str,
) -> String {
    format!(
        "[{DERUST_ROUTE_TAG_SELECT}] backend={} lane={} source={} reason={}",
        backend, lane, source, reason
    )
}

fn emit_derust_route_select(backend: &str, lane: &str, source: &str, reason: &str) {
    if crate::config::env::vm_route_trace() {
        eprintln!(
            "{}",
            format_derust_route_select(backend, lane, source, reason)
        );
    }
}

pub(crate) fn decide_derust_route_source(
    backend: &str,
    strict_or_dev: bool,
    force_fallback: bool,
    prefer_vm_hako: bool,
) -> &'static str {
    if backend != "vm" {
        return "rust-thin";
    }
    if !strict_or_dev {
        return "rust-thin";
    }
    if !force_fallback && !prefer_vm_hako {
        return "rust-thin-explicit";
    }
    "hako-skeleton"
}

pub(crate) fn decide_vm_route_plan(
    backend: &str,
    force_fallback: bool,
    prefer_vm_hako: bool,
) -> Option<VmRoutePlan> {
    match backend {
        "vm" => {
            if force_fallback {
                Some(VmRoutePlan {
                    backend: "vm",
                    lane: VM_LANE_COMPAT_FALLBACK,
                    reason: "env:NYASH_VM_USE_FALLBACK=1",
                    action: VmRouteAction::CompatFallback,
                })
            } else if prefer_vm_hako {
                Some(VmRoutePlan {
                    backend: "vm",
                    lane: VM_LANE_HAKO_REFERENCE,
                    reason: "strict-dev-prefer",
                    action: VmRouteAction::VmHako,
                })
            } else {
                Some(VmRoutePlan {
                    backend: "vm",
                    lane: VM_LANE_RUST_KEEP,
                    reason: "explicit-keep-override",
                    action: VmRouteAction::Vm,
                })
            }
        }
        "vm-hako" => Some(VmRoutePlan {
            backend: "vm-hako",
            lane: VM_LANE_HAKO_REFERENCE,
            reason: "explicit-reference-override",
            action: VmRouteAction::VmHako,
        }),
        _ => None,
    }
}

fn force_vm_lane_for_emit(groups: &crate::cli::CliGroups) -> bool {
    groups.emit.emit_mir_json.is_some() || groups.emit.emit_exe.is_some()
}

pub(crate) fn execute_vm_family_route(
    runner: &NyashRunner,
    backend: &str,
    filename: &str,
) -> bool {
    let groups = runner.config.as_groups();
    let force_fallback = crate::config::env::vm_use_fallback();
    let force_vm_for_emit = force_vm_lane_for_emit(&groups);
    let prefer_vm_hako = crate::config::env::vm_hako_prefer_strict_dev() && !force_vm_for_emit;
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let plan = if backend == "vm" && force_vm_for_emit && !force_fallback {
        // Emit-mode keeps the legacy rust-vm lane explicit for compat/debug
        // contracts; this is not a new mainline owner path.
        VmRoutePlan {
            backend: "vm",
            lane: VM_LANE_RUST_KEEP,
            reason: "emit-mode-force-rust-vm-keep",
            action: VmRouteAction::Vm,
        }
    } else {
        let Some(plan) = decide_vm_route_plan(backend, force_fallback, prefer_vm_hako) else {
            return false;
        };
        plan
    };
    let source = decide_derust_route_source(backend, strict_or_dev, force_fallback, prefer_vm_hako);
    emit_vm_route_select(&plan);
    emit_derust_route_select(plan.backend, plan.lane, source, plan.reason);
    match plan.action {
        VmRouteAction::Vm => runner.execute_vm_mode(filename),
        VmRouteAction::VmHako => runner.execute_vm_hako_mode(filename),
        VmRouteAction::CompatFallback => runner.execute_vm_fallback_interpreter(filename),
    }
    true
}

pub(crate) fn is_vm_compat_fallback_allowed(force_fallback: bool) -> bool {
    force_fallback
}

pub(crate) fn decide_vm_compat_fallback_guard(force_fallback: bool) -> VmCompatGuardPlan {
    if is_vm_compat_fallback_allowed(force_fallback) {
        VmCompatGuardPlan {
            action: VmCompatGuardAction::Allow,
            reason: "env:NYASH_VM_USE_FALLBACK=1",
        }
    } else {
        VmCompatGuardPlan {
            action: VmCompatGuardAction::Reject,
            reason: "require:NYASH_VM_USE_FALLBACK=1",
        }
    }
}

pub(crate) fn format_vm_compat_bypass_freeze(route: &str) -> String {
    format!(
        "[freeze:contract][{}] route={} require=NYASH_VM_USE_FALLBACK=1",
        VM_ROUTE_FREEZE_COMPAT_BYPASS, route
    )
}

pub(crate) fn enforce_vm_compat_fallback_guard_or_exit(route: &str) {
    let force_fallback = crate::config::env::vm_use_fallback();
    let plan = decide_vm_compat_fallback_guard(force_fallback);
    if plan.action == VmCompatGuardAction::Reject {
        eprintln!("{}", format_vm_compat_bypass_freeze(route));
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decide_vm_route_plan_vm_default() {
        let plan = decide_vm_route_plan("vm", false, false).expect("plan");
        assert_eq!(plan.backend, "vm");
        assert_eq!(plan.lane, VM_LANE_RUST_KEEP);
        assert_eq!(plan.reason, "explicit-keep-override");
        assert_eq!(plan.action, VmRouteAction::Vm);
    }

    #[test]
    fn decide_vm_route_plan_vm_hako_preferred() {
        let plan = decide_vm_route_plan("vm", false, true).expect("plan");
        assert_eq!(plan.backend, "vm");
        assert_eq!(plan.lane, VM_LANE_HAKO_REFERENCE);
        assert_eq!(plan.reason, "strict-dev-prefer");
        assert_eq!(plan.action, VmRouteAction::VmHako);
    }

    #[test]
    fn decide_vm_route_plan_vm_fallback_forced() {
        let plan = decide_vm_route_plan("vm", true, true).expect("plan");
        assert_eq!(plan.backend, "vm");
        assert_eq!(plan.lane, VM_LANE_COMPAT_FALLBACK);
        assert_eq!(plan.reason, "env:NYASH_VM_USE_FALLBACK=1");
        assert_eq!(plan.action, VmRouteAction::CompatFallback);
    }

    #[test]
    fn decide_vm_route_plan_vm_hako_explicit_backend() {
        let plan = decide_vm_route_plan("vm-hako", false, false).expect("plan");
        assert_eq!(plan.backend, "vm-hako");
        assert_eq!(plan.lane, VM_LANE_HAKO_REFERENCE);
        assert_eq!(plan.reason, "explicit-reference-override");
        assert_eq!(plan.action, VmRouteAction::VmHako);
    }

    #[test]
    fn decide_vm_route_plan_unknown_backend_is_none() {
        assert!(decide_vm_route_plan("llvm", false, false).is_none());
    }

    #[test]
    fn force_vm_lane_for_emit_mir_json() {
        let mut cfg = crate::cli::CliConfig::default();
        cfg.emit_mir_json = Some("/tmp/out.json".to_string());
        let mut groups = cfg.as_groups();
        assert!(force_vm_lane_for_emit(&groups));
        groups.emit.emit_mir_json = None;
        assert!(!force_vm_lane_for_emit(&groups));
    }

    #[test]
    fn format_vm_route_pre_dispatch_is_stable() {
        assert_eq!(
            format_vm_route_pre_dispatch("vm", "apps/tests/min.hako"),
            "[vm-route/pre-dispatch] backend=vm file=apps/tests/min.hako"
        );
    }

    #[test]
    fn format_vm_route_select_is_stable() {
        let plan = VmRoutePlan {
            backend: "vm",
            lane: VM_LANE_COMPAT_FALLBACK,
            reason: "env:NYASH_VM_USE_FALLBACK=1",
            action: VmRouteAction::CompatFallback,
        };
        assert_eq!(
            format_vm_route_select(&plan),
            "[vm-route/select] backend=vm lane=vm-compat-fallback reason=env:NYASH_VM_USE_FALLBACK=1"
        );
    }

    #[test]
    fn format_derust_route_select_is_stable() {
        assert_eq!(
            format_derust_route_select("vm", "vm-hako-reference", "hako-skeleton", "strict-dev-prefer"),
            "[derust-route/select] backend=vm lane=vm-hako-reference source=hako-skeleton reason=strict-dev-prefer"
        );
    }

    #[test]
    fn format_vm_compat_bypass_freeze_is_stable() {
        assert_eq!(
            format_vm_compat_bypass_freeze("vm-fallback"),
            "[freeze:contract][vm-route/compat-bypass] route=vm-fallback require=NYASH_VM_USE_FALLBACK=1"
        );
    }

    #[test]
    fn vm_compat_fallback_allowed_only_when_explicit() {
        assert!(!is_vm_compat_fallback_allowed(false));
        assert!(is_vm_compat_fallback_allowed(true));
    }

    #[test]
    fn decide_vm_compat_fallback_guard_rejects_without_flag() {
        let plan = decide_vm_compat_fallback_guard(false);
        assert_eq!(plan.action, VmCompatGuardAction::Reject);
        assert_eq!(plan.reason, "require:NYASH_VM_USE_FALLBACK=1");
    }

    #[test]
    fn decide_vm_compat_fallback_guard_allows_with_flag() {
        let plan = decide_vm_compat_fallback_guard(true);
        assert_eq!(plan.action, VmCompatGuardAction::Allow);
        assert_eq!(plan.reason, "env:NYASH_VM_USE_FALLBACK=1");
    }

    #[test]
    fn decide_derust_route_source_strict_default_is_hako_skeleton() {
        assert_eq!(
            decide_derust_route_source("vm", true, false, true),
            "hako-skeleton"
        );
    }

    #[test]
    fn decide_derust_route_source_strict_explicit_rust_thin() {
        assert_eq!(
            decide_derust_route_source("vm", true, false, false),
            "rust-thin-explicit"
        );
    }

    #[test]
    fn decide_derust_route_source_non_strict_is_rust_thin() {
        assert_eq!(
            decide_derust_route_source("vm", false, false, false),
            "rust-thin"
        );
    }
}
