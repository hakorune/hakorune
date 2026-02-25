//! Phase 29ai P0: build_plan entrypoint skeleton

#![allow(dead_code)]

use crate::ast::ASTNode;

use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    CleanupKindFacts, ExitKindFacts, ExitUsageFacts,
};
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonKind;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::verifier::debug_assert_value_join_invariants;
use std::collections::BTreeSet;

use super::candidates::{CandidateSet, PlanCandidate};
use super::context::PlannerContext;
use super::outcome::build_plan_with_facts;
use super::Freeze;
use crate::mir::builder::control_flow::plan::{
    scan_direction_from_step_lit, DomainPlan, Pattern1ArrayJoinPlan, Pattern1CharMapPlan,
    Pattern1SimpleWhilePlan, Pattern2BreakPlan, Pattern2PromotionHint, Pattern3IfPhiPlan,
    Pattern4ContinuePlan, Pattern5ExitKind, Pattern5InfiniteEarlyExitPlan,
    Pattern8BoolPredicateScanPlan, Pattern9AccumConstLoopPlan, ScanDirection, ScanWithInitPlan,
    SplitScanPlan,
};
use crate::mir::loop_pattern_detection::LoopPatternKind;

/// Phase 29ai P0: External-ish SSOT entrypoint (skeleton)
///
/// P0: This intentionally returns `Ok(None)` for all inputs.
pub(in crate::mir::builder) fn build_plan(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<DomainPlan>, Freeze> {
    Ok(build_plan_with_facts(condition, body)?.plan)
}

pub(in crate::mir::builder) fn build_plan_from_facts(
    facts: CanonicalLoopFacts,
) -> Result<Option<DomainPlan>, Freeze> {
    build_plan_from_facts_ctx(&PlannerContext::default_for_legacy(), facts)
}

pub(in crate::mir::builder) fn build_plan_from_facts_ctx(
    ctx: &PlannerContext,
    facts: CanonicalLoopFacts,
) -> Result<Option<DomainPlan>, Freeze> {
    // Phase 29ai P3: CandidateSet-based boundary (SSOT)
    //
    // P3 note: Facts are currently `Ok(None)` (P0 skeleton), so this function is
    // unreachable in normal execution today. We still implement the SSOT
    // boundary here so that future Facts work cannot drift.

    match facts.skeleton_kind {
        SkeletonKind::Loop => {}
        _ => return Ok(None),
    }

    let allow_pattern1 = match ctx.pattern_kind {
        Some(LoopPatternKind::Pattern1SimpleWhile) | None => true,
        Some(_) => false,
    };
    let allow_pattern8 = true;

    let _skeleton_kind = infer_skeleton_kind(&facts);
    let _exit_usage = infer_exit_usage(&facts);
    debug_assert_cleanup_kinds_match_exit_kinds(
        &facts.cleanup_kinds_present,
        &facts.exit_kinds_present,
    );
    debug_assert_value_join_invariants(&facts);

    let mut candidates = CandidateSet::new();

    push_scan_with_init(&mut candidates, &facts);
    push_split_scan(&mut candidates, &facts);
    push_pattern2_break(&mut candidates, &facts);
    push_pattern3_ifphi(&mut candidates, &facts);
    push_pattern4_continue(&mut candidates, &facts);
    push_pattern5_infinite_early_exit(&mut candidates, &facts);
    push_pattern8_bool_predicate_scan(&mut candidates, &facts, allow_pattern8);
    push_pattern9_accum_const_loop(&mut candidates, &facts);
    push_pattern1_char_map(&mut candidates, &facts, allow_pattern1);
    push_pattern1_array_join(&mut candidates, &facts, allow_pattern1);
    push_pattern1_simplewhile(&mut candidates, &facts, allow_pattern1);
    if candidates.is_empty() {
        push_loop_true_break_continue(&mut candidates, &facts);
    }
    if candidates.is_empty() {
        push_loop_cond_break_continue(&mut candidates, &facts);
    }
    if candidates.is_empty() {
        push_generic_loop_v1(&mut candidates, &facts);
    }
    if candidates.is_empty() {
        push_generic_loop_v0(&mut candidates, &facts);
    }

    candidates.finalize()
}

fn infer_skeleton_kind(facts: &CanonicalLoopFacts) -> Option<SkeletonKind> {
    Some(facts.skeleton_kind)
}

fn infer_exit_usage(facts: &CanonicalLoopFacts) -> Option<ExitUsageFacts> {
    Some(facts.exit_usage.clone())
}

#[cfg(debug_assertions)]
fn debug_assert_exit_usage_matches_plan(
    plan: &DomainPlan,
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
    match plan {
        DomainPlan::Pattern1SimpleWhile(_) => {
            debug_assert!(
                !exit_usage.has_break
                    && !exit_usage.has_continue
                    && !exit_usage.has_return
                    && !exit_usage.has_unwind,
                "pattern1 requires no exit usage"
            );
        }
        DomainPlan::Pattern1CharMap(_) => {
            debug_assert!(
                !exit_usage.has_break
                    && !exit_usage.has_continue
                    && !exit_usage.has_return
                    && !exit_usage.has_unwind,
                "pattern1 char map requires no exit usage"
            );
        }
        DomainPlan::Pattern1ArrayJoin(_) => {
            debug_assert!(
                !exit_usage.has_break
                    && !exit_usage.has_continue
                    && !exit_usage.has_return
                    && !exit_usage.has_unwind,
                "pattern1 array join requires no exit usage"
            );
        }
        DomainPlan::Pattern2Break(_) => {
            debug_assert!(exit_usage.has_break, "pattern2 requires break usage");
        }
        DomainPlan::Pattern4Continue(_) => {
            debug_assert!(exit_usage.has_continue, "pattern4 requires continue usage");
        }
        DomainPlan::Pattern5InfiniteEarlyExit(plan) => match plan.exit_kind {
            Pattern5ExitKind::Return => {
                debug_assert!(exit_usage.has_return, "pattern5 return requires return usage");
            }
            Pattern5ExitKind::Break => {
                debug_assert!(exit_usage.has_break, "pattern5 break requires break usage");
            }
        },
        DomainPlan::LoopTrueBreakContinue(_) => {
            debug_assert!(
                !exit_usage.has_return,
                "loop_true_break_continue does not allow return"
            );
        }
        _ => {}
    }
}

#[cfg(not(debug_assertions))]
fn debug_assert_exit_usage_matches_plan(
    _plan: &DomainPlan,
    _exit_usage: &ExitUsageFacts,
    _exit_kinds_present: &BTreeSet<ExitKindFacts>,
) {
}

#[cfg(debug_assertions)]
fn debug_assert_cleanup_kinds_match_exit_kinds(
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
fn debug_assert_cleanup_kinds_match_exit_kinds(
    _cleanup_kinds_present: &BTreeSet<CleanupKindFacts>,
    _exit_kinds_present: &BTreeSet<ExitKindFacts>,
) {
}

fn push_scan_with_init(candidates: &mut CandidateSet, facts: &CanonicalLoopFacts) {
    let Some(scan) = &facts.facts.scan_with_init else {
        return;
    };
    let Some(scan_direction) = scan_direction_from_step_lit(scan.step_lit) else {
        return;
    };
    candidates.push(PlanCandidate {
        plan: DomainPlan::ScanWithInit(ScanWithInitPlan {
            loop_var: scan.loop_var.clone(),
            haystack: scan.haystack.clone(),
            needle: scan.needle.clone(),
            step_lit: scan.step_lit,
            early_return_expr: ASTNode::Variable {
                name: scan.loop_var.clone(),
                span: crate::ast::Span::unknown(),
            },
            not_found_return_lit: -1,
            scan_direction,
            dynamic_needle: scan.dynamic_needle,
        }),
        rule: "loop/scan_with_init",
    });
}

fn push_split_scan(candidates: &mut CandidateSet, facts: &CanonicalLoopFacts) {
    let Some(split_scan) = &facts.facts.split_scan else {
        return;
    };
    candidates.push(PlanCandidate {
        plan: DomainPlan::SplitScan(SplitScanPlan {
            s_var: split_scan.s_var.clone(),
            sep_var: split_scan.sep_var.clone(),
            result_var: split_scan.result_var.clone(),
            i_var: split_scan.i_var.clone(),
            start_var: split_scan.start_var.clone(),
        }),
        rule: "loop/split_scan",
    });
}

fn push_pattern2_break(candidates: &mut CandidateSet, facts: &CanonicalLoopFacts) {
    let Some(pattern2) = &facts.facts.pattern2_break else {
        return;
    };
    let promotion = facts
        .facts
        .pattern2_loopbodylocal
        .as_ref()
        .map(|facts| Pattern2PromotionHint::LoopBodyLocal(facts.clone()));
    let plan = DomainPlan::Pattern2Break(Pattern2BreakPlan {
        loop_var: pattern2.loop_var.clone(),
        carrier_var: pattern2.carrier_var.clone(),
        loop_condition: pattern2.loop_condition.clone(),
        break_condition: pattern2.break_condition.clone(),
        carrier_update_in_break: pattern2.carrier_update_in_break.clone(),
        carrier_update_in_body: pattern2.carrier_update_in_body.clone(),
        loop_increment: pattern2.loop_increment.clone(),
        promotion,
    });
    debug_assert_exit_usage_matches_plan(&plan, &facts.exit_usage, &facts.exit_kinds_present);
    candidates.push(PlanCandidate {
        plan,
        rule: "loop/pattern2_break",
    });
}

fn push_pattern3_ifphi(candidates: &mut CandidateSet, facts: &CanonicalLoopFacts) {
    let Some(pattern3) = &facts.facts.pattern3_ifphi else {
        return;
    };
    candidates.push(PlanCandidate {
        plan: DomainPlan::Pattern3IfPhi(Pattern3IfPhiPlan {
            loop_var: pattern3.loop_var.clone(),
            carrier_var: pattern3.carrier_var.clone(),
            condition: pattern3.condition.clone(),
            if_condition: pattern3.if_condition.clone(),
            then_update: pattern3.then_update.clone(),
            else_update: pattern3.else_update.clone(),
            loop_increment: pattern3.loop_increment.clone(),
        }),
        rule: "loop/pattern3_ifphi",
    });
}

fn push_pattern4_continue(candidates: &mut CandidateSet, facts: &CanonicalLoopFacts) {
    let Some(pattern4) = &facts.facts.pattern4_continue else {
        return;
    };
    let plan = DomainPlan::Pattern4Continue(Pattern4ContinuePlan {
        loop_var: pattern4.loop_var.clone(),
        carrier_vars: pattern4.carrier_updates.keys().cloned().collect(),
        condition: pattern4.condition.clone(),
        continue_condition: pattern4.continue_condition.clone(),
        carrier_updates: pattern4.carrier_updates.clone(),
        loop_increment: pattern4.loop_increment.clone(),
    });
    debug_assert_exit_usage_matches_plan(&plan, &facts.exit_usage, &facts.exit_kinds_present);
    candidates.push(PlanCandidate {
        plan,
        rule: "loop/pattern4_continue",
    });
}

fn push_pattern5_infinite_early_exit(candidates: &mut CandidateSet, facts: &CanonicalLoopFacts) {
    let Some(pattern5) = &facts.facts.pattern5_infinite_early_exit else {
        return;
    };
    let plan = DomainPlan::Pattern5InfiniteEarlyExit(Pattern5InfiniteEarlyExitPlan {
        loop_var: pattern5.loop_var.clone(),
        exit_kind: pattern5.exit_kind,
        exit_condition: pattern5.exit_condition.clone(),
        exit_value: pattern5.exit_value.clone(),
        carrier_var: pattern5.carrier_var.clone(),
        carrier_update: pattern5.carrier_update.clone(),
        loop_increment: pattern5.loop_increment.clone(),
    });
    debug_assert_exit_usage_matches_plan(&plan, &facts.exit_usage, &facts.exit_kinds_present);
    candidates.push(PlanCandidate {
        plan,
        rule: "loop/pattern5_infinite_early_exit",
    });
}

fn push_loop_true_break_continue(candidates: &mut CandidateSet, facts: &CanonicalLoopFacts) {
    // loop(true) break/continue is a strict/dev helper shape.
    // It must be a fallback-only candidate to avoid ambiguous overlaps with
    // established patterns (e.g. Pattern5 infinite early exit).
    if !candidates.is_empty() {
        return;
    }
    let Some(loop_true) = &facts.facts.loop_true_break_continue else {
        return;
    };
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    if !planner_required {
        return;
    }
    if facts.value_join_needed {
        return;
    }
    candidates.push(PlanCandidate {
        plan: DomainPlan::LoopTrueBreakContinue(loop_true.clone()),
        rule: "loop/loop_true_break_continue",
    });
}

fn push_loop_cond_break_continue(candidates: &mut CandidateSet, facts: &CanonicalLoopFacts) {
    let Some(loop_cond) = &facts.facts.loop_cond_break_continue else {
        return;
    };
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    if !planner_required {
        return;
    }
    candidates.push(PlanCandidate {
        plan: DomainPlan::LoopCondBreakContinue(loop_cond.clone()),
        rule: "loop/loop_cond_break_continue",
    });
}

fn push_pattern8_bool_predicate_scan(
    candidates: &mut CandidateSet,
    facts: &CanonicalLoopFacts,
    allow_pattern8: bool,
) {
    if !allow_pattern8 {
        return;
    }
    let Some(pattern8) = &facts.facts.pattern8_bool_predicate_scan else {
        return;
    };
    candidates.push(PlanCandidate {
        plan: DomainPlan::Pattern8BoolPredicateScan(Pattern8BoolPredicateScanPlan {
            loop_var: pattern8.loop_var.clone(),
            haystack: pattern8.haystack.clone(),
            predicate_receiver: pattern8.predicate_receiver.clone(),
            predicate_method: pattern8.predicate_method.clone(),
            condition: pattern8.condition.clone(),
            step_lit: pattern8.step_lit,
        }),
        rule: "loop/pattern8_bool_predicate_scan",
    });
}

fn push_pattern9_accum_const_loop(candidates: &mut CandidateSet, facts: &CanonicalLoopFacts) {
    let Some(pattern9) = &facts.facts.pattern9_accum_const_loop else {
        return;
    };
    candidates.push(PlanCandidate {
        plan: DomainPlan::Pattern9AccumConstLoop(Pattern9AccumConstLoopPlan {
            loop_var: pattern9.loop_var.clone(),
            acc_var: pattern9.acc_var.clone(),
            condition: pattern9.condition.clone(),
            acc_update: pattern9.acc_update.clone(),
            loop_increment: pattern9.loop_increment.clone(),
        }),
        rule: "loop/pattern9_accum_const_loop",
    });
}

fn push_pattern1_char_map(
    candidates: &mut CandidateSet,
    facts: &CanonicalLoopFacts,
    allow_pattern1: bool,
) {
    if !allow_pattern1 {
        return;
    }
    let Some(map) = &facts.facts.pattern1_char_map else {
        return;
    };

    candidates.push(PlanCandidate {
        plan: DomainPlan::Pattern1CharMap(Pattern1CharMapPlan {
            loop_var: map.loop_var.clone(),
            condition: map.condition.clone(),
            loop_increment: map.loop_increment.clone(),
            haystack_var: map.haystack_var.clone(),
            result_var: map.result_var.clone(),
            receiver_var: map.receiver_var.clone(),
            transform_method: map.transform_method.clone(),
        }),
        rule: "loop/pattern1_char_map",
    });
}

fn push_pattern1_array_join(
    candidates: &mut CandidateSet,
    facts: &CanonicalLoopFacts,
    allow_pattern1: bool,
) {
    if !allow_pattern1 {
        return;
    }
    let Some(join) = &facts.facts.pattern1_array_join else {
        return;
    };

    candidates.push(PlanCandidate {
        plan: DomainPlan::Pattern1ArrayJoin(Pattern1ArrayJoinPlan {
            loop_var: join.loop_var.clone(),
            condition: join.condition.clone(),
            if_condition: join.if_condition.clone(),
            loop_increment: join.loop_increment.clone(),
            array_var: join.array_var.clone(),
            result_var: join.result_var.clone(),
            separator_var: join.separator_var.clone(),
        }),
        rule: "loop/pattern1_array_join",
    });
}

fn push_pattern1_simplewhile(
    candidates: &mut CandidateSet,
    facts: &CanonicalLoopFacts,
    allow_pattern1: bool,
) {
    if !allow_pattern1 {
        return;
    }
    let Some(pattern1) = &facts.facts.pattern1_simplewhile else {
        return;
    };
    let plan = DomainPlan::Pattern1SimpleWhile(Pattern1SimpleWhilePlan {
        loop_var: pattern1.loop_var.clone(),
        condition: pattern1.condition.clone(),
        loop_increment: pattern1.loop_increment.clone(),
    });
    debug_assert_exit_usage_matches_plan(&plan, &facts.exit_usage, &facts.exit_kinds_present);
    candidates.push(PlanCandidate {
        plan,
        rule: "loop/pattern1_simplewhile",
    });
}

fn push_generic_loop_v0(candidates: &mut CandidateSet, facts: &CanonicalLoopFacts) {
    let Some(generic) = &facts.facts.generic_loop_v0 else {
        return;
    };
    // IMPORTANT: generic loop v0 must not bypass Pattern2 LoopBodyLocal contracts.
    // If Pattern2LoopBodyLocal facts exist, allow only Pattern2 line to decide
    // (strict/dev should freeze on violations; release should not silently widen semantics).
    if facts.value_join_needed || facts.facts.pattern2_loopbodylocal.is_some() {
        return;
    }
    candidates.push(PlanCandidate {
        plan: DomainPlan::GenericLoopV0(generic.clone()),
        rule: "loop/generic_v0",
    });
}

fn push_generic_loop_v1(candidates: &mut CandidateSet, facts: &CanonicalLoopFacts) {
    let Some(generic) = &facts.facts.generic_loop_v1 else {
        return;
    };
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    if !planner_required {
        return;
    }
    // IMPORTANT: generic loop v1 must not bypass Pattern2 LoopBodyLocal contracts.
    if facts.value_join_needed || facts.facts.pattern2_loopbodylocal.is_some() {
        return;
    }
    candidates.push(PlanCandidate {
        plan: DomainPlan::GenericLoopV1(generic.clone()),
        rule: "loop/generic_v1",
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
        ConditionShape, StepShape,
    };
    use crate::mir::builder::control_flow::plan::facts::feature_facts::{
        ExitKindFacts, ExitMapFacts, ExitUsageFacts, LoopFeatureFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::loop_facts::{
        LoopFacts, ScanWithInitFacts, SplitScanFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::pattern1_simplewhile_facts::{
        Pattern1SimpleWhileFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::pattern1_char_map_facts::{
        Pattern1CharMapFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::pattern1_array_join_facts::{
        Pattern1ArrayJoinFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::pattern3_ifphi_facts::{
        Pattern3IfPhiFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::pattern4_continue_facts::{
        Pattern4ContinueFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::pattern5_infinite_early_exit_facts::{
        Pattern5InfiniteEarlyExitFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::pattern8_bool_predicate_scan_facts::{
        Pattern8BoolPredicateScanFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::pattern9_accum_const_loop_facts::{
        Pattern9AccumConstLoopFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::pattern2_break_facts::Pattern2BreakFacts;
    use crate::mir::builder::control_flow::plan::facts::pattern2_loopbodylocal_facts::{
        LoopBodyLocalShape, Pattern2LoopBodyLocalFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{
        SkeletonFacts, SkeletonKind,
    };
    use crate::mir::builder::control_flow::plan::normalize::canonicalize_loop_facts;
    use crate::mir::builder::control_flow::plan::{
        Pattern2PromotionHint, Pattern5ExitKind, ScanDirection,
    };
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use std::collections::{BTreeMap, BTreeSet};

    fn v(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn feature_facts_with_usage(exit_usage: ExitUsageFacts) -> LoopFeatureFacts {
        let mut kinds_present = BTreeSet::new();
        if exit_usage.has_return {
            kinds_present.insert(ExitKindFacts::Return);
        }
        if exit_usage.has_break {
            kinds_present.insert(ExitKindFacts::Break);
        }
        if exit_usage.has_continue {
            kinds_present.insert(ExitKindFacts::Continue);
        }
        if exit_usage.has_unwind {
            kinds_present.insert(ExitKindFacts::Unwind);
        }
        let exit_map = if kinds_present.is_empty() {
            None
        } else {
            Some(ExitMapFacts { kinds_present })
        };
        LoopFeatureFacts {
            exit_usage,
            exit_map,
            value_join: None,
            cleanup: None,
            nested_loop: false,
        }
    }

    #[test]
    fn planner_builds_split_scan_plan_from_facts() {
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: Some(SplitScanFacts {
                s_var: "s".to_string(),
                sep_var: "separator".to_string(),
                result_var: "result".to_string(),
                i_var: "i".to_string(),
                start_var: "start".to_string(),
                shape: crate::mir::builder::control_flow::plan::facts::scan_shapes::SplitScanShape::Minimal,
            }),
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");

        match plan {
            Some(DomainPlan::SplitScan(plan)) => {
                assert_eq!(plan.s_var, "s");
                assert_eq!(plan.sep_var, "separator");
                assert_eq!(plan.result_var, "result");
                assert_eq!(plan.i_var, "i");
                assert_eq!(plan.start_var, "start");
            }
            other => panic!("expected split scan plan, got {:?}", other),
        }
    }

    #[test]
    fn planner_prefers_none_when_no_candidates() {
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");
        assert!(plan.is_none());
    }

    #[test]
    fn planner_retains_scan_with_init_plan_path() {
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: Some(ScanWithInitFacts {
                loop_var: "i".to_string(),
                haystack: "s".to_string(),
                needle: "ch".to_string(),
                step_lit: 1,
                dynamic_needle: false,
            }),
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");
        match plan {
            Some(DomainPlan::ScanWithInit(plan)) => {
                assert_eq!(plan.loop_var, "i");
            }
            other => panic!("expected scan_with_init plan, got {:?}", other),
        }
    }

    #[test]
    fn planner_sets_reverse_scan_direction_for_negative_step() {
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: Some(ScanWithInitFacts {
                loop_var: "i".to_string(),
                haystack: "s".to_string(),
                needle: "ch".to_string(),
                step_lit: -1,
                dynamic_needle: false,
            }),
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");
        match plan {
            Some(DomainPlan::ScanWithInit(plan)) => {
                assert_eq!(plan.step_lit, -1);
                assert_eq!(plan.scan_direction, ScanDirection::Reverse);
            }
            other => panic!("expected scan_with_init plan, got {:?}", other),
        }
    }

    #[test]
    fn planner_ignores_skeleton_and_feature_staging() {
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: feature_facts_with_usage(ExitUsageFacts {
                has_break: true,
                has_continue: false,
                has_return: false,
                has_unwind: false,
            }),
            scan_with_init: Some(ScanWithInitFacts {
                loop_var: "i".to_string(),
                haystack: "s".to_string(),
                needle: "ch".to_string(),
                step_lit: 1,
                dynamic_needle: false,
            }),
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");
        match plan {
            Some(DomainPlan::ScanWithInit(_)) => {}
            other => panic!("expected scan_with_init plan, got {:?}", other),
        }
    }

    #[test]
    fn planner_gates_non_loop_skeletons() {
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::If2,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: Some(ScanWithInitFacts {
                loop_var: "i".to_string(),
                haystack: "s".to_string(),
                needle: "ch".to_string(),
                step_lit: 1,
                dynamic_needle: false,
            }),
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts_ctx(&PlannerContext::default_for_legacy(), canonical)
            .expect("Ok");
        assert!(plan.is_none());
    }

    #[test]
    fn planner_builds_pattern1_simplewhile_plan_from_facts() {
        let loop_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(3)),
            span: Span::unknown(),
        };
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };

        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: Some(Pattern1SimpleWhileFacts {
                loop_var: "i".to_string(),
                condition: loop_condition.clone(),
                loop_increment: loop_increment.clone(),
            }),
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");

        match plan {
            Some(DomainPlan::Pattern1SimpleWhile(plan)) => {
                assert_eq!(plan.loop_var, "i");
            }
            other => panic!("expected pattern1 simplewhile plan, got {:?}", other),
        }
    }

    #[test]
    fn planner_builds_pattern1_char_map_plan_from_facts() {
        let loop_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };

        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: Some(Pattern1CharMapFacts {
                loop_var: "i".to_string(),
                condition: loop_condition.clone(),
                loop_increment: loop_increment.clone(),
                haystack_var: "s".to_string(),
                result_var: "result".to_string(),
                receiver_var: "me".to_string(),
                transform_method: "char_to_lower".to_string(),
            }),
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");

        match plan {
            Some(DomainPlan::Pattern1CharMap(plan)) => {
                assert_eq!(plan.loop_var, "i");
                assert_eq!(plan.haystack_var, "s");
                assert_eq!(plan.result_var, "result");
                assert_eq!(plan.transform_method, "char_to_lower");
            }
            other => panic!("expected pattern1 char map plan, got {:?}", other),
        }
    }

    #[test]
    fn planner_builds_pattern1_array_join_plan_from_facts() {
        let loop_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v("arr")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let if_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Greater,
            left: Box::new(v("i")),
            right: Box::new(lit_int(0)),
            span: Span::unknown(),
        };
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };

        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: Some(Pattern1ArrayJoinFacts {
                loop_var: "i".to_string(),
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
                condition: loop_condition.clone(),
                if_condition: if_condition.clone(),
                loop_increment: loop_increment.clone(),
                array_var: "arr".to_string(),
                result_var: "result".to_string(),
                separator_var: "sep".to_string(),
            }),
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");

        match plan {
            Some(DomainPlan::Pattern1ArrayJoin(plan)) => {
                assert_eq!(plan.loop_var, "i");
                assert_eq!(plan.array_var, "arr");
                assert_eq!(plan.result_var, "result");
                assert_eq!(plan.separator_var, "sep");
            }
            other => panic!("expected pattern1 array join plan, got {:?}", other),
        }
    }

    #[test]
    fn debug_exit_usage_invariant_pattern1_ok() {
        let loop_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(3)),
            span: Span::unknown(),
        };
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: Some(Pattern1SimpleWhileFacts {
                loop_var: "i".to_string(),
                condition: loop_condition.clone(),
                loop_increment: loop_increment.clone(),
            }),
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");
        assert!(matches!(plan, Some(DomainPlan::Pattern1SimpleWhile(_))));
    }

    #[test]
    fn debug_exit_usage_invariant_pattern2_ok() {
        let loop_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(3)),
            span: Span::unknown(),
        };
        let break_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(v("i")),
            right: Box::new(lit_int(2)),
            span: Span::unknown(),
        };
        let carrier_update = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("sum")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: feature_facts_with_usage(ExitUsageFacts {
                has_break: true,
                has_continue: false,
                has_return: false,
                has_unwind: false,
            }),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: Some(Pattern2BreakFacts {
                loop_var: "i".to_string(),
                carrier_var: "sum".to_string(),
                loop_condition,
                break_condition,
                carrier_update_in_break: None,
                carrier_update_in_body: carrier_update,
                loop_increment,
            }),
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");
        assert!(matches!(plan, Some(DomainPlan::Pattern2Break(_))));
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn debug_exit_usage_invariant_pattern1_panics_on_break_usage() {
        let loop_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(3)),
            span: Span::unknown(),
        };
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: feature_facts_with_usage(ExitUsageFacts {
                has_break: true,
                has_continue: false,
                has_return: false,
                has_unwind: false,
            }),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: Some(Pattern1SimpleWhileFacts {
                loop_var: "i".to_string(),
                condition: loop_condition.clone(),
                loop_increment: loop_increment.clone(),
            }),
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let _ = build_plan_from_facts(canonical).expect("Ok");
    }

    #[test]
    fn planner_builds_pattern3_ifphi_plan_from_facts() {
        let loop_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(3)),
            span: Span::unknown(),
        };
        let if_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Greater,
            left: Box::new(v("i")),
            right: Box::new(lit_int(0)),
            span: Span::unknown(),
        };
        let then_update = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("sum")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };
        let else_update = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("sum")),
            right: Box::new(lit_int(0)),
            span: Span::unknown(),
        };
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };

        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: Some(Pattern3IfPhiFacts {
                loop_var: "i".to_string(),
                carrier_var: "sum".to_string(),
                condition: loop_condition.clone(),
                if_condition: if_condition.clone(),
                then_update: then_update.clone(),
                else_update: else_update.clone(),
                loop_increment: loop_increment.clone(),
            }),
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");

        match plan {
            Some(DomainPlan::Pattern3IfPhi(plan)) => {
                assert_eq!(plan.loop_var, "i");
                assert_eq!(plan.carrier_var, "sum");
            }
            other => panic!("expected pattern3 if-phi plan, got {:?}", other),
        }
    }

    #[test]
    fn planner_builds_pattern4_continue_plan_from_facts() {
        let loop_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(6)),
            span: Span::unknown(),
        };
        let continue_condition = v("skip");
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };

        let mut carrier_updates = BTreeMap::new();
        carrier_updates.insert(
            "sum".to_string(),
            ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(v("sum")),
                right: Box::new(v("i")),
                span: Span::unknown(),
            },
        );

        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: feature_facts_with_usage(ExitUsageFacts {
                has_break: false,
                has_continue: true,
                has_return: false,
                has_unwind: false,
            }),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: Some(Pattern4ContinueFacts {
                loop_var: "i".to_string(),
                condition: loop_condition.clone(),
                continue_condition: continue_condition.clone(),
                carrier_updates: carrier_updates.clone(),
                loop_increment: loop_increment.clone(),
            }),
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");

        match plan {
            Some(DomainPlan::Pattern4Continue(plan)) => {
                assert_eq!(plan.loop_var, "i");
                assert_eq!(plan.carrier_vars, vec!["sum".to_string()]);
            }
            other => panic!("expected pattern4 continue plan, got {:?}", other),
        }
    }

    #[test]
    fn planner_builds_pattern5_infinite_early_exit_plan_from_facts() {
        let exit_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(v("i")),
            right: Box::new(lit_int(3)),
            span: Span::unknown(),
        };
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };

        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: feature_facts_with_usage(ExitUsageFacts {
                has_break: false,
                has_continue: false,
                has_return: true,
                has_unwind: false,
            }),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: Some(Pattern5InfiniteEarlyExitFacts {
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
                loop_var: "i".to_string(),
                exit_kind: Pattern5ExitKind::Return,
                exit_condition: exit_condition.clone(),
                exit_value: Some(v("sum")),
                carrier_var: None,
                carrier_update: None,
                loop_increment: loop_increment.clone(),
            }),
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");

        match plan {
            Some(DomainPlan::Pattern5InfiniteEarlyExit(plan)) => {
                assert_eq!(plan.loop_var, "i");
                assert_eq!(plan.exit_kind, Pattern5ExitKind::Return);
            }
            other => panic!("expected pattern5 infinite early exit plan, got {:?}", other),
        }
    }

    #[test]
    fn planner_builds_pattern8_bool_predicate_scan_plan_from_facts() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v("s")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: feature_facts_with_usage(ExitUsageFacts {
                has_break: true,
                has_continue: false,
                has_return: false,
                has_unwind: false,
            }),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: Some(Pattern8BoolPredicateScanFacts {
                loop_var: "i".to_string(),
                haystack: "s".to_string(),
                predicate_receiver: "me".to_string(),
                predicate_method: "is_digit".to_string(),
                condition: condition.clone(),
                step_lit: 1,
            }),
            pattern9_accum_const_loop: None,
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");

        match plan {
            Some(DomainPlan::Pattern8BoolPredicateScan(plan)) => {
                assert_eq!(plan.loop_var, "i");
                assert_eq!(plan.haystack, "s");
                assert_eq!(plan.predicate_method, "is_digit");
                assert_eq!(plan.step_lit, 1);
            }
            other => panic!("expected pattern8 bool predicate scan plan, got {:?}", other),
        }
    }

    #[test]
    fn planner_builds_pattern9_accum_const_loop_plan_from_facts() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(3)),
            span: Span::unknown(),
        };
        let acc_update = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("sum")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };

        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: Some(Pattern9AccumConstLoopFacts {
                loop_var: "i".to_string(),
                acc_var: "sum".to_string(),
                condition: condition.clone(),
                acc_update: acc_update.clone(),
                loop_increment: loop_increment.clone(),
            }),
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");

        match plan {
            Some(DomainPlan::Pattern9AccumConstLoop(plan)) => {
                assert_eq!(plan.loop_var, "i");
                assert_eq!(plan.acc_var, "sum");
            }
            other => panic!("expected pattern9 accum const loop plan, got {:?}", other),
        }
    }

    #[test]
    fn planner_sets_promotion_hint_for_pattern2_loopbodylocal() {
        let loop_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(3)),
            span: Span::unknown(),
        };
        let break_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(v("seg")),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::String(" ".to_string()),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let carrier_update = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("sum")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };

        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: None,
            pattern1_simplewhile: None,
            pattern1_char_map: None,
            pattern1_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            pattern3_ifphi: None,
            pattern4_continue: None,
            pattern5_infinite_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            pattern8_bool_predicate_scan: None,
            pattern9_accum_const_loop: None,
            pattern2_break: Some(Pattern2BreakFacts {
                loop_var: "i".to_string(),
                carrier_var: "sum".to_string(),
                loop_condition,
                break_condition,
                carrier_update_in_break: None,
                carrier_update_in_body: carrier_update,
                loop_increment,
            }),
            pattern2_loopbodylocal: Some(Pattern2LoopBodyLocalFacts {
                loop_var: "i".to_string(),
                loopbodylocal_var: "seg".to_string(),
                break_uses_loopbodylocal: true,
                shape: LoopBodyLocalShape::TrimSeg {
                    s_var: "s".to_string(),
                    i_var: "i".to_string(),
                },
            }),
            pattern6_nested_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts(canonical).expect("Ok");

        match plan {
            Some(DomainPlan::Pattern2Break(plan)) => {
                let promotion = plan.promotion.expect("promotion hint");
                match promotion {
                    Pattern2PromotionHint::LoopBodyLocal(facts) => {
                        assert_eq!(facts.loopbodylocal_var, "seg");
                    }
                }
            }
            other => panic!("expected pattern2 break plan, got {:?}", other),
        }
    }
}
