//! Phase 47-A: Generic step scheduling for Pattern2/Pattern3 loops
//!
//! Determines evaluation order for loop steps (header cond, body init, break check, etc).
//! Used by both P2 (break) and P3 (if-sum) patterns.
//!
//! This keeps the lowerer focused on emitting fragments, while this box decides
//! how to interleave them (e.g., body-local init before break checks when the
//! break depends on body-local values).

use crate::config::env;
use crate::config::env::joinir_dev::joinir_test_debug_enabled;
use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, CarrierInit};
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::runtime::get_global_ring0;

/// Steps that can be reordered by the scheduler.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Pattern2StepKind {
    // P2 (Pattern2 Break) steps
    HeaderCond, // loop(cond)
    BodyInit,   // local ch = ...
    BreakCheck, // if (cond) break
    Updates,    // sum = sum + 1
    Tail,       // i = i + 1

    // Phase 47-A: P3 (Pattern3 If-Sum) steps
    IfCond,      // if (cond) in body
    ThenUpdates, // carrier updates in then branch
    ElseUpdates, // carrier updates in else branch (if any)

    // Phase 48-A: P4 (Pattern4 Continue) steps
    ContinueCheck, // if (cond) continue
}

impl Pattern2StepKind {
    fn as_str(&self) -> &'static str {
        match self {
            Pattern2StepKind::HeaderCond => "header-cond",
            Pattern2StepKind::BodyInit => "body-init",
            Pattern2StepKind::BreakCheck => "break",
            Pattern2StepKind::Updates => "updates",
            Pattern2StepKind::Tail => "tail",
            // Phase 47-A: P3 steps
            Pattern2StepKind::IfCond => "if-cond",
            Pattern2StepKind::ThenUpdates => "then-updates",
            Pattern2StepKind::ElseUpdates => "else-updates",
            // Phase 48-A: P4 steps
            Pattern2StepKind::ContinueCheck => "continue-check",
        }
    }
}

/// Data-driven schedule for Pattern 2 lowering.
#[derive(Debug, Clone)]
pub(crate) struct Pattern2StepSchedule {
    steps: Vec<Pattern2StepKind>,
    reason: &'static str,
}

impl Pattern2StepSchedule {
    pub(crate) fn iter(&self) -> impl Iterator<Item = Pattern2StepKind> + '_ {
        self.steps.iter().copied()
    }

    pub(crate) fn reason(&self) -> &'static str {
        self.reason
    }

    pub(crate) fn steps(&self) -> &[Pattern2StepKind] {
        &self.steps
    }
}

/// Schedule decision result with reasoning (SSOT)
#[derive(Debug, Clone)]
pub(crate) struct ScheduleDecision {
    /// Whether body-init should come before break check
    pub body_init_first: bool,
    /// Human-readable reason for this decision
    pub reason: &'static str,
    /// Debug context for logging
    pub debug_ctx: ScheduleDebugContext,
}

/// Debug context for schedule decisions
#[derive(Debug, Clone)]
pub(crate) struct ScheduleDebugContext {
    pub has_body_local_init: bool,
    pub has_loop_local_carrier: bool,
    pub has_condition_only_recipe: bool,
    pub has_body_local_derived_recipe: bool,
}

/// Facts about Pattern2 that drive step scheduling.
///
/// This struct is intentionally "facts only" (no decision).
#[derive(Debug, Clone)]
pub(crate) struct Pattern2ScheduleFacts {
    pub has_body_local_init: bool,
    pub has_loop_local_carrier: bool,
    pub has_condition_only_recipe: bool,
    pub has_body_local_derived_recipe: bool,
}

pub(crate) struct Pattern2ScheduleFactsBox;

impl Pattern2ScheduleFactsBox {
    pub(crate) fn gather(
        body_local_env: Option<&LoopBodyLocalEnv>,
        carrier_info: &CarrierInfo,
        has_condition_only_recipe: bool,
        has_body_local_derived_recipe: bool,
        has_allowed_body_locals_in_conditions: bool,
    ) -> Pattern2ScheduleFacts {
        // NOTE: `body_local_env` may be empty here because it's populated after schedule
        // decision (Phase 191 body-local init lowering happens later).
        //
        // For Phase 92+ patterns where conditions reference allowed body-local variables
        // (e.g., `ch == ""`), we must still schedule BodyInit before BreakCheck.
        let has_body_local_init = body_local_env.map(|env| !env.is_empty()).unwrap_or(false)
            || has_allowed_body_locals_in_conditions;
        let has_loop_local_carrier = carrier_info
            .carriers
            .iter()
            .any(|c| matches!(c.init, CarrierInit::LoopLocalZero));

        Pattern2ScheduleFacts {
            has_body_local_init,
            has_loop_local_carrier,
            has_condition_only_recipe,
            has_body_local_derived_recipe,
        }
    }
}

/// Decide Pattern2 schedule based on loop characteristics (SSOT).
///
/// Phase 93 Refactoring: Single source of truth for schedule decisions
///
/// # Decision Logic
///
/// Body-init comes BEFORE break check if any of these conditions are true:
/// 1. ConditionOnly recipe exists (derived slots need recalculation)
/// 2. Body-local variables exist (break condition depends on them)
/// 3. Loop-local carriers exist (need initialization before use)
///
/// # Arguments
///
/// * `body_local_env` - Body-local variable environment
/// * `carrier_info` - Carrier information (for loop-local detection)
/// * `has_condition_only_recipe` - Whether ConditionOnly derived slots exist
///
/// # Returns
///
/// `ScheduleDecision` with decision, reason, and debug context
pub(crate) fn decide_pattern2_schedule(facts: &Pattern2ScheduleFacts) -> ScheduleDecision {
    let body_init_first = facts.has_condition_only_recipe
        || facts.has_body_local_derived_recipe
        || facts.has_body_local_init
        || facts.has_loop_local_carrier;

    let reason = if facts.has_condition_only_recipe {
        "ConditionOnly requires body-init before break"
    } else if facts.has_body_local_derived_recipe {
        "BodyLocalDerived requires body-init before break"
    } else if facts.has_body_local_init {
        "body-local variables require init before break"
    } else if facts.has_loop_local_carrier {
        "loop-local carrier requires init before break"
    } else {
        "default schedule"
    };

    ScheduleDecision {
        body_init_first,
        reason,
        debug_ctx: ScheduleDebugContext {
            has_body_local_init: facts.has_body_local_init,
            has_loop_local_carrier: facts.has_loop_local_carrier,
            has_condition_only_recipe: facts.has_condition_only_recipe,
            has_body_local_derived_recipe: facts.has_body_local_derived_recipe,
        },
    }
}

/// Build a schedule for Pattern 2 lowering.
///
/// - Default P2: header → break → body-init → updates → tail
/// - Body-local break dependency (DigitPos/_atoi style):
///   header → body-init → break → updates → tail
pub(crate) fn build_pattern2_schedule_from_decision(
    decision: &ScheduleDecision,
) -> Pattern2StepSchedule {
    let schedule = if decision.body_init_first {
        Pattern2StepSchedule {
            steps: vec![
                Pattern2StepKind::HeaderCond,
                Pattern2StepKind::BodyInit,
                Pattern2StepKind::BreakCheck,
                Pattern2StepKind::Updates,
                Pattern2StepKind::Tail,
            ],
            reason: decision.reason,
        }
    } else {
        Pattern2StepSchedule {
            steps: vec![
                Pattern2StepKind::HeaderCond,
                Pattern2StepKind::BreakCheck,
                Pattern2StepKind::BodyInit,
                Pattern2StepKind::Updates,
                Pattern2StepKind::Tail,
            ],
            reason: decision.reason,
        }
    };

    log_schedule_from_decision(decision, &schedule);
    schedule
}

fn log_schedule_from_decision(decision: &ScheduleDecision, schedule: &Pattern2StepSchedule) {
    if !(env::joinir_dev_enabled() || joinir_test_debug_enabled()) {
        return;
    }

    let steps_desc = schedule
        .steps()
        .iter()
        .map(Pattern2StepKind::as_str)
        .collect::<Vec<_>>()
        .join(" -> ");

    get_global_ring0().log.debug(&format!(
        "[phase93/schedule] steps={} reason={} ctx={{body_local_init={}, loop_local_carrier={}, condition_only={}, body_local_derived={}}}",
        steps_desc,
        schedule.reason(),
        decision.debug_ctx.has_body_local_init,
        decision.debug_ctx.has_loop_local_carrier,
        decision.debug_ctx.has_condition_only_recipe,
        decision.debug_ctx.has_body_local_derived_recipe
    ));
}

/// Phase 47-A: Generate step schedule for Pattern3 (if-sum) loops
#[allow(dead_code)]
pub(crate) fn pattern3_if_sum_schedule() -> Vec<Pattern2StepKind> {
    vec![
        Pattern2StepKind::HeaderCond,  // loop(i < n)
        Pattern2StepKind::IfCond,      // if (i > 0)
        Pattern2StepKind::ThenUpdates, // sum = sum + i
        // ElseUpdates omitted for minimal (no else branch)
        Pattern2StepKind::Tail, // i = i + 1
    ]
}

/// Phase 48-A: Generate step schedule for Pattern4 (continue) loops
#[allow(dead_code)]
pub(crate) fn pattern4_continue_schedule() -> Vec<Pattern2StepKind> {
    vec![
        Pattern2StepKind::HeaderCond,    // loop(i < n)
        Pattern2StepKind::ContinueCheck, // if (i == 2) continue (skip processing)
        Pattern2StepKind::Updates,       // count = count + 1 (processing)
        Pattern2StepKind::Tail,          // i = i + 1
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::lowering::carrier_info::{CarrierRole, CarrierVar};
    use crate::mir::ValueId;

    fn carrier(loop_local: bool) -> CarrierVar {
        let init = if loop_local {
            CarrierInit::LoopLocalZero
        } else {
            CarrierInit::FromHost
        };
        CarrierVar::with_role_and_init("c".to_string(), ValueId(1), CarrierRole::LoopState, init)
    }

    fn carrier_info(carriers: Vec<CarrierVar>) -> CarrierInfo {
        CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(0),
            carriers,
            trim_helper: None,
            promoted_loopbodylocals: vec![],
            #[cfg(feature = "normalized_dev")]
            promoted_bindings: std::collections::BTreeMap::new(),
        }
    }

    #[test]
    fn default_schedule_break_before_body_init() {
        let facts = Pattern2ScheduleFactsBox::gather(None, &carrier_info(vec![]), false, false, false);
        let decision = decide_pattern2_schedule(&facts);
        let schedule = build_pattern2_schedule_from_decision(&decision);
        assert_eq!(
            schedule.steps(),
            &[
                Pattern2StepKind::HeaderCond,
                Pattern2StepKind::BreakCheck,
                Pattern2StepKind::BodyInit,
                Pattern2StepKind::Updates,
                Pattern2StepKind::Tail
            ]
        );
        assert_eq!(schedule.reason(), "default schedule");
    }

    #[test]
    fn body_local_moves_init_before_break() {
        let mut body_env = LoopBodyLocalEnv::new();
        body_env.insert("tmp".to_string(), ValueId(5));

        let facts = Pattern2ScheduleFactsBox::gather(
            Some(&body_env),
            &carrier_info(vec![carrier(false)]),
            false,
            false,
            false,
        );
        let decision = decide_pattern2_schedule(&facts);
        let schedule = build_pattern2_schedule_from_decision(&decision);
        assert_eq!(
            schedule.steps(),
            &[
                Pattern2StepKind::HeaderCond,
                Pattern2StepKind::BodyInit,
                Pattern2StepKind::BreakCheck,
                Pattern2StepKind::Updates,
                Pattern2StepKind::Tail
            ]
        );
        assert_eq!(schedule.reason(), "body-local variables require init before break");
    }

    #[test]
    fn loop_local_carrier_triggers_body_first() {
        let facts =
            Pattern2ScheduleFactsBox::gather(None, &carrier_info(vec![carrier(true)]), false, false, false);
        let decision = decide_pattern2_schedule(&facts);
        let schedule = build_pattern2_schedule_from_decision(&decision);
        assert_eq!(
            schedule.steps(),
            &[
                Pattern2StepKind::HeaderCond,
                Pattern2StepKind::BodyInit,
                Pattern2StepKind::BreakCheck,
                Pattern2StepKind::Updates,
                Pattern2StepKind::Tail
            ]
        );
        assert_eq!(schedule.reason(), "loop-local carrier requires init before break");
    }

    /// Phase 93 P0: ConditionOnly recipe triggers body-init before break
    #[test]
    fn condition_only_recipe_triggers_body_first() {
        // Empty body_local_env but has condition_only_recipe
        let facts = Pattern2ScheduleFactsBox::gather(None, &carrier_info(vec![]), true, false, false);
        let decision = decide_pattern2_schedule(&facts);
        let schedule = build_pattern2_schedule_from_decision(&decision);
        assert_eq!(
            schedule.steps(),
            &[
                Pattern2StepKind::HeaderCond,
                Pattern2StepKind::BodyInit,
                Pattern2StepKind::BreakCheck,
                Pattern2StepKind::Updates,
                Pattern2StepKind::Tail
            ]
        );
        assert_eq!(schedule.reason(), "ConditionOnly requires body-init before break");
    }

    #[test]
    fn allowed_body_local_deps_trigger_body_first_even_if_env_empty() {
        let facts = Pattern2ScheduleFactsBox::gather(None, &carrier_info(vec![]), false, false, true);
        let decision = decide_pattern2_schedule(&facts);
        let schedule = build_pattern2_schedule_from_decision(&decision);
        assert_eq!(
            schedule.steps(),
            &[
                Pattern2StepKind::HeaderCond,
                Pattern2StepKind::BodyInit,
                Pattern2StepKind::BreakCheck,
                Pattern2StepKind::Updates,
                Pattern2StepKind::Tail
            ]
        );
        assert_eq!(schedule.reason(), "body-local variables require init before break");
    }

    #[test]
    fn test_pattern3_if_sum_schedule() {
        let schedule = pattern3_if_sum_schedule();
        assert_eq!(schedule.len(), 4);
        assert_eq!(schedule[0], Pattern2StepKind::HeaderCond);
        assert_eq!(schedule[1], Pattern2StepKind::IfCond);
        assert_eq!(schedule[2], Pattern2StepKind::ThenUpdates);
        assert_eq!(schedule[3], Pattern2StepKind::Tail);
    }

    #[test]
    fn test_pattern4_continue_schedule() {
        let schedule = pattern4_continue_schedule();
        assert_eq!(schedule.len(), 4);
        assert_eq!(schedule[0], Pattern2StepKind::HeaderCond);
        assert_eq!(schedule[1], Pattern2StepKind::ContinueCheck);
        assert_eq!(schedule[2], Pattern2StepKind::Updates);
        assert_eq!(schedule[3], Pattern2StepKind::Tail);
    }
}
