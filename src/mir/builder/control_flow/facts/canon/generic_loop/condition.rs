use crate::ast::ASTNode;
use crate::mir::policies::{CondParam, CondProfile, CondSkeleton};

use super::types::ConditionCanon;

pub(in crate::mir::builder) mod bound;
pub(in crate::mir::builder) mod candidates;

pub(crate) fn canon_condition_for_generic_loop_v0(
    condition: &ASTNode,
    allow_extended: bool,
) -> Option<ConditionCanon> {
    let candidates = if allow_extended {
        let mut candidates = Vec::new();
        if !candidates::collect_candidates_from_condition(condition, &mut candidates) {
            return None;
        }
        candidates
    } else {
        candidates::collect_candidates_from_top_level_comparison(condition)?
    };
    if candidates.is_empty() {
        return None;
    }
    let cond_profile = build_cond_profile(condition, &candidates);
    Some(ConditionCanon {
        loop_var_candidates: candidates,
        cond_profile,
    })
}

fn build_cond_profile(condition: &ASTNode, candidates: &[String]) -> CondProfile {
    let mut params = Vec::new();
    for candidate in candidates {
        params.push(CondParam::LoopVar(candidate.clone()));
    }
    if let Some(bound) = bound::extract_bound_from_condition(condition, candidates) {
        params.push(CondParam::Bound(bound));
    }
    CondProfile::new(CondSkeleton::LoopCond, params)
}
