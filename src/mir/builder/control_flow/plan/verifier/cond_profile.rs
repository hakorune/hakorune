use crate::mir::builder::control_flow::plan::planner::Freeze;

pub(in crate::mir::builder) fn accept_via_cond_profile_pattern1_char_map(
    facts: &crate::mir::builder::control_flow::plan::facts::pattern1_char_map_facts::Pattern1CharMapFacts,
) -> Result<bool, Freeze> {
    accept_via_cond_profile_freeze_on_incomplete(&facts.cond_profile, "pattern1_char_map")
}

pub(in crate::mir::builder) fn accept_via_cond_profile_pattern1_array_join(
    facts: &crate::mir::builder::control_flow::plan::facts::pattern1_array_join_facts::Pattern1ArrayJoinFacts,
) -> Result<bool, Freeze> {
    accept_via_cond_profile_freeze_on_incomplete(&facts.cond_profile, "pattern1_array_join")
}

pub(in crate::mir::builder) fn accept_via_cond_profile_pattern8_bool_predicate_scan(
    facts: &crate::mir::builder::control_flow::plan::facts::pattern8_bool_predicate_scan_facts::Pattern8BoolPredicateScanFacts,
) -> Result<bool, Freeze> {
    accept_via_cond_profile_freeze_on_incomplete(&facts.cond_profile, "pattern8_bool_predicate_scan")
}

pub(in crate::mir::builder) fn accept_via_cond_profile_pattern9_accum_const_loop(
    facts: &crate::mir::builder::control_flow::plan::facts::pattern9_accum_const_loop_facts::Pattern9AccumConstLoopFacts,
) -> Result<bool, Freeze> {
    accept_via_cond_profile_freeze_on_incomplete(&facts.cond_profile, "pattern9_accum_const_loop")
}

fn accept_via_cond_profile_freeze_on_incomplete(
    cond_profile: &crate::mir::policies::CondProfile,
    pattern: &str,
) -> Result<bool, Freeze> {
    if !cond_profile_is_complete_for_accept(cond_profile) {
        return Err(Freeze::contract(
            "condprofile incomplete: legacy fallback disallowed",
        )
        .with_hint(format!("pattern={}", pattern)));
    }
    Ok(true)
}

fn cond_profile_is_complete_for_accept(
    cond_profile: &crate::mir::policies::CondProfile,
) -> bool {
    let (has_loop_var, has_cmp, has_bound, has_step) = cond_profile_param_flags_accept(cond_profile);
    matches!(cond_profile.skeleton, crate::mir::policies::CondSkeleton::LoopCond)
        && has_loop_var
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
