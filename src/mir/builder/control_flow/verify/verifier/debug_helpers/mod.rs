#[cfg(debug_assertions)]
use crate::mir::builder::control_flow::facts::scan_shapes::cond_profile_from_scan_shapes;
use crate::mir::builder::control_flow::facts::scan_shapes::StepShape;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;

#[cfg(debug_assertions)]
#[allow(dead_code)] // Phase 291x-126: umbrella probe retained for targeted verifier debugging.
pub(in crate::mir::builder) fn debug_observe_cond_profile(facts: &CanonicalLoopFacts) {
    let cond_profile =
        cond_profile_from_scan_shapes(&facts.facts.condition_shape, &facts.facts.step_shape);
    debug_observe_cond_profile_value(&cond_profile);
    let _ = cond_profile;
}

#[cfg(debug_assertions)]
pub(in crate::mir::builder) fn debug_observe_cond_profile_value(
    cond_profile: &crate::mir::policies::CondProfile,
) {
    use crate::config::env::joinir_dev;
    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[condprofile] skeleton={:?} params={}",
            cond_profile.skeleton,
            cond_profile.params.len()
        ));
    }
}

#[cfg(not(debug_assertions))]
#[allow(dead_code)] // Phase 291x-126: keep release stub paired with debug probe.
pub(in crate::mir::builder) fn debug_observe_cond_profile(_facts: &CanonicalLoopFacts) {}

#[cfg(not(debug_assertions))]
pub(in crate::mir::builder) fn debug_observe_cond_profile_value(
    _cond_profile: &crate::mir::policies::CondProfile,
) {
}

#[cfg(debug_assertions)]
pub(in crate::mir::builder) fn debug_observe_cond_profile_step_mismatch(
    step_shape: &StepShape,
    cond_profile: &crate::mir::policies::CondProfile,
) {
    use crate::config::env::joinir_dev;
    use crate::mir::policies::{CondParam, StepExpr};

    let step_k = match step_shape {
        StepShape::AssignAddConst { k, .. } => *k,
        _ => return,
    };

    let profile_step = cond_profile.params.iter().find_map(|param| {
        if let CondParam::Step(step) = param {
            Some(step.clone())
        } else {
            None
        }
    });

    if matches!(profile_step, Some(StepExpr::Delta(k)) if k == step_k) {
        return;
    }

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[condprofile:step_mismatch] step_k={} profile_step={:?}",
            step_k, profile_step
        ));
    }
}

#[cfg(not(debug_assertions))]
pub(in crate::mir::builder) fn debug_observe_cond_profile_step_mismatch(
    _step_shape: &StepShape,
    _cond_profile: &crate::mir::policies::CondProfile,
) {
}

#[cfg(debug_assertions)]
pub(in crate::mir::builder) fn debug_observe_cond_profile_completeness(
    cond_profile: &crate::mir::policies::CondProfile,
) {
    use crate::config::env::joinir_dev;

    let complete = cond_profile_is_complete(cond_profile);
    let (has_loop_var, has_cmp, has_bound, has_step) = cond_profile_param_flags(cond_profile);

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        if complete {
            ring0.log.debug("[condprofile:complete]");
        } else {
            let mut missing = Vec::new();
            if !has_loop_var {
                missing.push("LoopVar");
            }
            if !has_cmp {
                missing.push("Cmp");
            }
            if !has_bound {
                missing.push("Bound");
            }
            if !has_step {
                missing.push("Step");
            }
            let missing = if missing.is_empty() {
                "Skeleton".to_string()
            } else {
                missing.join(",")
            };
            ring0
                .log
                .debug(&format!("[condprofile:incomplete] missing={}", missing));
        }
    }
}

#[cfg(not(debug_assertions))]
pub(in crate::mir::builder) fn debug_observe_cond_profile_completeness(
    _cond_profile: &crate::mir::policies::CondProfile,
) {
}

#[cfg(debug_assertions)]
pub(in crate::mir::builder) fn debug_observe_cond_profile_priority(
    cond_profile: &crate::mir::policies::CondProfile,
) {
    use crate::config::env::joinir_dev;
    if !joinir_dev::debug_enabled() {
        return;
    }
    if cond_profile_is_complete(cond_profile) {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug("[condprofile:priority] condprofile");
    } else {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug("[condprofile:priority] legacy");
    }
}

#[cfg(not(debug_assertions))]
pub(in crate::mir::builder) fn debug_observe_cond_profile_priority(
    _cond_profile: &crate::mir::policies::CondProfile,
) {
}

#[cfg(debug_assertions)]
pub(in crate::mir::builder) fn debug_observe_cond_profile_idx_var(
    facts_loop_var: &str,
    cond_profile: &crate::mir::policies::CondProfile,
) {
    use crate::config::env::joinir_dev;
    use crate::mir::policies::CondParam;

    let profile_loop_var = cond_profile.params.iter().find_map(|param| {
        if let CondParam::LoopVar(name) = param {
            Some(name.as_str())
        } else {
            None
        }
    });

    if joinir_dev::debug_enabled() {
        let (profile_value, matched) = match profile_loop_var {
            Some(name) => (name, name == facts_loop_var),
            None => ("<missing>", false),
        };
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[condprofile:idx_var] facts={} profile={} match={}",
            facts_loop_var, profile_value, matched
        ));
    }
}

#[cfg(not(debug_assertions))]
pub(in crate::mir::builder) fn debug_observe_cond_profile_idx_var(
    _facts_loop_var: &str,
    _cond_profile: &crate::mir::policies::CondProfile,
) {
}

#[cfg(debug_assertions)]
fn cond_profile_is_complete(cond_profile: &crate::mir::policies::CondProfile) -> bool {
    let (has_loop_var, has_cmp, has_bound, has_step) = cond_profile_param_flags(cond_profile);

    matches!(
        cond_profile.skeleton,
        crate::mir::policies::CondSkeleton::LoopCond
    ) && has_loop_var
        && has_cmp
        && has_bound
        && has_step
}

#[cfg(debug_assertions)]
fn cond_profile_param_flags(
    cond_profile: &crate::mir::policies::CondProfile,
) -> (bool, bool, bool, bool) {
    use crate::mir::policies::CondParam;

    let mut has_loop_var = false;
    let mut has_cmp = false;
    let mut has_bound = false;
    let mut has_step = false;

    for param in &cond_profile.params {
        match param {
            CondParam::LoopVar(_) => has_loop_var = true,
            CondParam::Cmp(_) => has_cmp = true,
            CondParam::Bound(_) => has_bound = true,
            CondParam::Step(_) => has_step = true,
        }
    }

    (has_loop_var, has_cmp, has_bound, has_step)
}
