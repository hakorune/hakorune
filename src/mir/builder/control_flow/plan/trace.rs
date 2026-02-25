use crate::mir::builder::control_flow::plan::single_planner::PlanRuleId;

pub(in crate::mir::builder) fn trace_candidate_push(rule: &'static str) {
    if !crate::config::env::joinir_trace_enabled() {
        return;
    }
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!("[plan/trace] stage=candidate_push rule={}", rule));
}

pub(in crate::mir::builder) fn trace_candidate_finalize_none() {
    if !crate::config::env::joinir_trace_enabled() {
        return;
    }
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug("[plan/trace] stage=candidate_finalize result=none");
}

pub(in crate::mir::builder) fn trace_candidate_finalize_some(rule: &'static str) {
    if !crate::config::env::joinir_trace_enabled() {
        return;
    }
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[plan/trace] stage=candidate_finalize result=some rule={}",
        rule
    ));
}

pub(in crate::mir::builder) fn trace_candidate_finalize_ambiguous(count: usize, rules: &str) {
    if !crate::config::env::joinir_trace_enabled() {
        return;
    }
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[plan/trace] stage=candidate_finalize result=ambiguous count={} rules={}",
        count, rules
    ));
}

pub(in crate::mir::builder) fn trace_try_take_planner(
    kind: PlanRuleId,
    planner_present: bool,
    taken: bool,
) {
    if !crate::config::env::joinir_trace_enabled() {
        return;
    }
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[plan/trace] stage=try_take_planner kind={:?} planner_present={} result={}",
        kind,
        planner_present,
        if taken { "taken" } else { "skip" }
    ));
}

/// Trace pattern shadow pick (diagnostic only)
pub(in crate::mir::builder) fn trace_pattern_shadow_pick(
    shadow_rule: &str,
    candidate_count: usize,
) {
    if !crate::config::env::joinir_trace_enabled() {
        return;
    }
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[plan/trace] pattern_shadow pick={} from {} candidates",
        shadow_rule, candidate_count
    ));
}
