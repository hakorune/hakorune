use crate::mir::builder::control_flow::facts::{
    accum_const_loop_facts::AccumConstLoopFacts, bool_predicate_scan_facts::BoolPredicateScanFacts,
    loop_array_join_facts::LoopArrayJoinFacts, loop_char_map_facts::LoopCharMapFacts,
};
use crate::mir::builder::control_flow::lower::planner::Freeze;

pub(in crate::mir::builder) fn accept_via_cond_profile_loop_char_map(
    facts: &LoopCharMapFacts,
) -> Result<bool, Freeze> {
    accept_via_cond_profile_freeze_on_incomplete(&facts.cond_profile, "loop_char_map")
}

pub(in crate::mir::builder) fn accept_via_cond_profile_loop_array_join(
    facts: &LoopArrayJoinFacts,
) -> Result<bool, Freeze> {
    accept_via_cond_profile_freeze_on_incomplete(&facts.cond_profile, "loop_array_join")
}

pub(in crate::mir::builder) fn accept_via_cond_profile_bool_predicate_scan(
    facts: &BoolPredicateScanFacts,
) -> Result<bool, Freeze> {
    accept_via_cond_profile_freeze_on_incomplete(&facts.cond_profile, "bool_predicate_scan")
}

pub(in crate::mir::builder) fn accept_via_cond_profile_accum_const_loop(
    facts: &AccumConstLoopFacts,
) -> Result<bool, Freeze> {
    accept_via_cond_profile_freeze_on_incomplete(&facts.cond_profile, "accum_const_loop")
}

fn accept_via_cond_profile_freeze_on_incomplete(
    cond_profile: &crate::mir::policies::CondProfile,
    route: &str,
) -> Result<bool, Freeze> {
    if !cond_profile_is_complete_for_accept(cond_profile) {
        return Err(
            Freeze::contract("condprofile incomplete: legacy fallback disallowed")
                .with_hint(format!("route={}", route)),
        );
    }
    Ok(true)
}

fn cond_profile_is_complete_for_accept(cond_profile: &crate::mir::policies::CondProfile) -> bool {
    let (has_loop_var, has_cmp, has_bound, has_step) =
        cond_profile_param_flags_accept(cond_profile);
    matches!(
        cond_profile.skeleton,
        crate::mir::policies::CondSkeleton::LoopCond
    ) && has_loop_var
        && has_cmp
        && has_bound
        && has_step
}

fn cond_profile_param_flags_accept(
    cond_profile: &crate::mir::policies::CondProfile,
) -> (bool, bool, bool, bool) {
    let mut has_loop_var = false;
    let mut has_cmp = false;
    let mut has_bound = false;
    let mut has_step = false;

    for param in &cond_profile.params {
        match param {
            crate::mir::policies::CondParam::LoopVar(_) => has_loop_var = true,
            crate::mir::policies::CondParam::Cmp(_) => has_cmp = true,
            crate::mir::policies::CondParam::Bound(_) => has_bound = true,
            crate::mir::policies::CondParam::Step(_) => has_step = true,
        }
    }

    (has_loop_var, has_cmp, has_bound, has_step)
}
