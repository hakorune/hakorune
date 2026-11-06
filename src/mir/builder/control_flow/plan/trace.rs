use crate::mir::builder::control_flow::plan::single_planner::PlanRuleId;

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
