//! Phase 47-A: Generic step scheduling for break / if_phi_join loops
//!
//! Determines evaluation order for loop steps (header cond, body init, break check, etc).
//! Used by break, if_phi_join, and loop_continue_only lowering helpers.
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
pub(crate) enum LoopBreakStepKind {
    // LoopBreak steps
    HeaderCond, // loop(cond)
    BodyInit,   // local ch = ...
    BreakCheck, // if (cond) break
    Updates,    // sum = sum + 1
    Tail,       // i = i + 1

    // Phase 47-A: if_phi_join steps
    IfCond,      // if (cond) in body
    ThenUpdates, // carrier updates in then branch
    ElseUpdates, // carrier updates in else branch (if any)

    // Phase 48-A: loop_continue_only steps
    ContinueCheck, // if (cond) continue
}

impl LoopBreakStepKind {
    fn as_str(&self) -> &'static str {
        match self {
            LoopBreakStepKind::HeaderCond => "header-cond",
            LoopBreakStepKind::BodyInit => "body-init",
            LoopBreakStepKind::BreakCheck => "break",
            LoopBreakStepKind::Updates => "updates",
            LoopBreakStepKind::Tail => "tail",
            // Phase 47-A: if_phi_join steps
            LoopBreakStepKind::IfCond => "if-cond",
            LoopBreakStepKind::ThenUpdates => "then-updates",
            LoopBreakStepKind::ElseUpdates => "else-updates",
            // Phase 48-A: loop_continue_only steps
            LoopBreakStepKind::ContinueCheck => "continue-check",
        }
    }
}

/// Data-driven schedule for loop_break lowering.
#[derive(Debug, Clone)]
pub(crate) struct LoopBreakStepSchedule {
    steps: Vec<LoopBreakStepKind>,
    reason: &'static str,
}

impl LoopBreakStepSchedule {
    pub(crate) fn iter(&self) -> impl Iterator<Item = LoopBreakStepKind> + '_ {
        self.steps.iter().copied()
    }

    pub(crate) fn reason(&self) -> &'static str {
        self.reason
    }

    pub(crate) fn steps(&self) -> &[LoopBreakStepKind] {
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

/// Facts about loop_break that drive step scheduling.
///
/// This struct is intentionally "facts only" (no decision).
#[derive(Debug, Clone)]
pub(crate) struct LoopBreakScheduleFacts {
    pub has_body_local_init: bool,
    pub has_loop_local_carrier: bool,
    pub has_condition_only_recipe: bool,
    pub has_body_local_derived_recipe: bool,
}

pub(crate) struct LoopBreakScheduleFactsBox;

impl LoopBreakScheduleFactsBox {
    pub(crate) fn gather(
        body_local_env: Option<&LoopBodyLocalEnv>,
        carrier_info: &CarrierInfo,
        has_condition_only_recipe: bool,
        has_body_local_derived_recipe: bool,
        has_allowed_body_locals_in_conditions: bool,
    ) -> LoopBreakScheduleFacts {
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

        LoopBreakScheduleFacts {
            has_body_local_init,
            has_loop_local_carrier,
            has_condition_only_recipe,
            has_body_local_derived_recipe,
        }
    }
}

/// Decide loop_break schedule based on loop characteristics (SSOT).
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
pub(crate) fn decide_loop_break_schedule(facts: &LoopBreakScheduleFacts) -> ScheduleDecision {
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

/// Build a schedule for loop_break lowering.
///
/// - Default loop_break: header → break → body-init → updates → tail
/// - Body-local break dependency (DigitPos/_atoi style):
///   header → body-init → break → updates → tail
pub(crate) fn build_loop_break_schedule_from_decision(
    decision: &ScheduleDecision,
) -> LoopBreakStepSchedule {
    let schedule = if decision.body_init_first {
        LoopBreakStepSchedule {
            steps: vec![
                LoopBreakStepKind::HeaderCond,
                LoopBreakStepKind::BodyInit,
                LoopBreakStepKind::BreakCheck,
                LoopBreakStepKind::Updates,
                LoopBreakStepKind::Tail,
            ],
            reason: decision.reason,
        }
    } else {
        LoopBreakStepSchedule {
            steps: vec![
                LoopBreakStepKind::HeaderCond,
                LoopBreakStepKind::BreakCheck,
                LoopBreakStepKind::BodyInit,
                LoopBreakStepKind::Updates,
                LoopBreakStepKind::Tail,
            ],
            reason: decision.reason,
        }
    };

    log_schedule_from_decision(decision, &schedule);
    schedule
}

fn log_schedule_from_decision(decision: &ScheduleDecision, schedule: &LoopBreakStepSchedule) {
    if !(env::joinir_dev_enabled() || joinir_test_debug_enabled()) {
        return;
    }

    let steps_desc = schedule
        .steps()
        .iter()
        .map(LoopBreakStepKind::as_str)
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

/// Phase 47-A: Generate step schedule for if_phi_join loops
#[allow(dead_code)]
pub(crate) fn if_phi_join_schedule() -> Vec<LoopBreakStepKind> {
    vec![
        LoopBreakStepKind::HeaderCond,  // loop(i < n)
        LoopBreakStepKind::IfCond,      // if (i > 0)
        LoopBreakStepKind::ThenUpdates, // sum = sum + i
        // ElseUpdates omitted for minimal (no else branch)
        LoopBreakStepKind::Tail, // i = i + 1
    ]
}

/// Phase 48-A: Generate step schedule for loop_continue_only loops
#[allow(dead_code)]
pub(crate) fn loop_continue_only_schedule() -> Vec<LoopBreakStepKind> {
    vec![
        LoopBreakStepKind::HeaderCond,    // loop(i < n)
        LoopBreakStepKind::ContinueCheck, // if (i == 2) continue (skip processing)
        LoopBreakStepKind::Updates,       // count = count + 1 (processing)
        LoopBreakStepKind::Tail,          // i = i + 1
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
            promoted_body_locals: vec![],
        }
    }

    #[test]
    fn default_schedule_break_before_body_init() {
        let facts = LoopBreakScheduleFactsBox::gather(None, &carrier_info(vec![]), false, false, false);
        let decision = decide_loop_break_schedule(&facts);
        let schedule = build_loop_break_schedule_from_decision(&decision);
        assert_eq!(
            schedule.steps(),
            &[
                LoopBreakStepKind::HeaderCond,
                LoopBreakStepKind::BreakCheck,
                LoopBreakStepKind::BodyInit,
                LoopBreakStepKind::Updates,
                LoopBreakStepKind::Tail
            ]
        );
        assert_eq!(schedule.reason(), "default schedule");
    }

    #[test]
    fn body_local_moves_init_before_break() {
        let mut body_env = LoopBodyLocalEnv::new();
        body_env.insert("tmp".to_string(), ValueId(5));

        let facts = LoopBreakScheduleFactsBox::gather(
            Some(&body_env),
            &carrier_info(vec![carrier(false)]),
            false,
            false,
            false,
        );
        let decision = decide_loop_break_schedule(&facts);
        let schedule = build_loop_break_schedule_from_decision(&decision);
        assert_eq!(
            schedule.steps(),
            &[
                LoopBreakStepKind::HeaderCond,
                LoopBreakStepKind::BodyInit,
                LoopBreakStepKind::BreakCheck,
                LoopBreakStepKind::Updates,
                LoopBreakStepKind::Tail
            ]
        );
        assert_eq!(schedule.reason(), "body-local variables require init before break");
    }

    #[test]
    fn loop_local_carrier_triggers_body_first() {
        let facts =
            LoopBreakScheduleFactsBox::gather(None, &carrier_info(vec![carrier(true)]), false, false, false);
        let decision = decide_loop_break_schedule(&facts);
        let schedule = build_loop_break_schedule_from_decision(&decision);
        assert_eq!(
            schedule.steps(),
            &[
                LoopBreakStepKind::HeaderCond,
                LoopBreakStepKind::BodyInit,
                LoopBreakStepKind::BreakCheck,
                LoopBreakStepKind::Updates,
                LoopBreakStepKind::Tail
            ]
        );
        assert_eq!(schedule.reason(), "loop-local carrier requires init before break");
    }

    /// Phase 93 P0: ConditionOnly recipe triggers body-init before break
    #[test]
    fn condition_only_recipe_triggers_body_first() {
        // Empty body_local_env but has condition_only_recipe
        let facts = LoopBreakScheduleFactsBox::gather(None, &carrier_info(vec![]), true, false, false);
        let decision = decide_loop_break_schedule(&facts);
        let schedule = build_loop_break_schedule_from_decision(&decision);
        assert_eq!(
            schedule.steps(),
            &[
                LoopBreakStepKind::HeaderCond,
                LoopBreakStepKind::BodyInit,
                LoopBreakStepKind::BreakCheck,
                LoopBreakStepKind::Updates,
                LoopBreakStepKind::Tail
            ]
        );
        assert_eq!(schedule.reason(), "ConditionOnly requires body-init before break");
    }

    #[test]
    fn allowed_body_local_deps_trigger_body_first_even_if_env_empty() {
        let facts = LoopBreakScheduleFactsBox::gather(None, &carrier_info(vec![]), false, false, true);
        let decision = decide_loop_break_schedule(&facts);
        let schedule = build_loop_break_schedule_from_decision(&decision);
        assert_eq!(
            schedule.steps(),
            &[
                LoopBreakStepKind::HeaderCond,
                LoopBreakStepKind::BodyInit,
                LoopBreakStepKind::BreakCheck,
                LoopBreakStepKind::Updates,
                LoopBreakStepKind::Tail
            ]
        );
        assert_eq!(schedule.reason(), "body-local variables require init before break");
    }

    #[test]
    fn test_if_phi_join_schedule() {
        let schedule = if_phi_join_schedule();
        assert_eq!(schedule.len(), 4);
        assert_eq!(schedule[0], LoopBreakStepKind::HeaderCond);
        assert_eq!(schedule[1], LoopBreakStepKind::IfCond);
        assert_eq!(schedule[2], LoopBreakStepKind::ThenUpdates);
        assert_eq!(schedule[3], LoopBreakStepKind::Tail);
    }

    #[test]
    fn test_loop_continue_only_schedule() {
        let schedule = loop_continue_only_schedule();
        assert_eq!(schedule.len(), 4);
        assert_eq!(schedule[0], LoopBreakStepKind::HeaderCond);
        assert_eq!(schedule[1], LoopBreakStepKind::ContinueCheck);
        assert_eq!(schedule[2], LoopBreakStepKind::Updates);
        assert_eq!(schedule[3], LoopBreakStepKind::Tail);
    }
}
