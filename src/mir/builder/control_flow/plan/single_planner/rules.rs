//! Phase 29ai P5: Rule list (order SSOT) + guards
//!
//! `PLAN_RULE_ORDER` is intentionally single-plan-only.
//! Router-level recipe entries still emit planner-first tags via `PlanRuleId`.

use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;

use crate::mir::builder::control_flow::plan::facts::pattern2_loopbodylocal_facts::LoopBodyLocalShape;
use crate::mir::builder::control_flow::plan::loop_plan_label;
use crate::mir::builder::control_flow::plan::planner::{self, PlanBuildOutcome, PlannerContext};
use crate::mir::builder::control_flow::plan::trace as plan_trace;

use super::rule_order::{planner_rule_semantic_label, rule_name, PlanRuleId, PLAN_RULE_ORDER};

struct PlannerGate {
    strict_or_dev: bool,
    planner_required: bool,
}

impl PlannerGate {
    fn new() -> Self {
        let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
            || crate::config::env::joinir_dev_enabled();
        let planner_required =
            strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
        Self {
            strict_or_dev,
            planner_required,
        }
    }

    fn log_planner_first(&self, rule_id: PlanRuleId) {
        // Gate sentinel: in strict+planner_required mode, emit stable, prefix-free tags
        // so planner-first smokes can validate routing with HAKO_JOINIR_DEBUG=0.
        if self.planner_required {
            let ring0 = crate::runtime::get_global_ring0();
            let msg =
                crate::mir::builder::control_flow::plan::planner::tags::planner_first_tag_with_label(
                    rule_id,
                );
            let _ = ring0.io.stderr_write(format!("{}\n", msg).as_bytes());
        }
    }
}

fn freeze_planner_required_none(ctx: &LoopRouteContext) -> String {
    let mut msg = format!(
        "planner required, but planner returned None (legacy fallback forbidden): func={} cond={} body_len={}",
        ctx.func_name,
        ctx.condition.node_type(),
        ctx.body.len()
    );
    if let Some(detail) =
        crate::mir::builder::control_flow::plan::facts::reject_reason::take_last_plan_reject_detail(
        )
    {
        msg.push_str(&format!("\nDetail: [joinir/reject_detail] {detail}"));
    }
    planner::Freeze::contract(&msg)
        .with_hint(
            "Disable HAKO_JOINIR_PLANNER_REQUIRED, or extend Facts→Planner coverage for this case.",
        )
        .to_string()
}

fn is_recipe_only_rule(rule_id: PlanRuleId) -> bool {
    matches!(rule_id, PlanRuleId::LoopCondContinueWithReturn)
}

fn debug_log_recipe_only_entry(rule_id: PlanRuleId) {
    if !crate::config::env::joinir_dev::debug_enabled() {
        return;
    }
    let ring0 = crate::runtime::get_global_ring0();
    let label = planner_rule_semantic_label(rule_id);
    ring0.log.debug(&format!(
        "[recipe:entry] {}: recipe-only (planner payload suppressed)",
        label
    ));
}

fn promotion_hint_tag(shape: &LoopBodyLocalShape) -> &'static str {
    match shape {
        LoopBodyLocalShape::TrimSeg { .. } => "[plan/loop_break/promotion_hint:TrimSeg]",
        LoopBodyLocalShape::DigitPos { .. } => "[plan/loop_break/promotion_hint:DigitPos]",
    }
}

fn emit_loop_break_promotion_hint_tag(promotion_shape: Option<&LoopBodyLocalShape>) {
    let Some(shape) = promotion_shape else {
        return;
    };
    if !crate::config::env::joinir_dev::strict_enabled() {
        return;
    }

    let tag = promotion_hint_tag(shape);
    let ring0 = crate::runtime::get_global_ring0();
    // Gate sentinel: promotion hints are validated by strict-only smokes.
    // Emit prefix-free to stderr so it does not depend on `NYASH_RING0_LOG_LEVEL`.
    let _ = ring0.io.stderr_write(format!("{}\n", tag).as_bytes());
}

pub(super) fn try_build_outcome(ctx: &LoopRouteContext) -> Result<PlanBuildOutcome, String> {
    use crate::mir::builder::control_flow::joinir::trace;

    let gate = PlannerGate::new();

    let planner_ctx = PlannerContext {
        pattern_kind: ctx.skeleton.map(|_| ctx.route_kind),
        in_static_box: ctx.in_static_box,
        debug: ctx.debug,
    };
    let mut outcome = planner::build_plan_with_facts_ctx(&planner_ctx, ctx.condition, ctx.body)
        .map_err(|freeze| freeze.to_string())?;
    let planner_present = outcome.plan.is_some();

    // Phase B: Recipe-first parallel path (planner_required only)
    if gate.planner_required {
        if let Some(ref facts) = outcome.facts {
            use crate::mir::builder::control_flow::plan::recipe_tree::contracts::RecipeContractKind;
            use crate::mir::builder::control_flow::plan::recipe_tree::RecipeMatcher;

            let contract =
                RecipeMatcher::try_match_loop(facts).map_err(|freeze| freeze.to_string())?;
            if crate::config::env::joinir_dev::debug_enabled() {
                if let Some(ref c) = contract {
                    let (has_break, has_continue, has_return) = match &c.kind {
                        RecipeContractKind::LoopWithExit {
                            has_break,
                            has_continue,
                            has_return,
                        } => (*has_break, *has_continue, *has_return),
                        _ => (false, false, false),
                    };
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[recipe:match] kind=LoopWithExit break={} continue={} return={}",
                        has_break, has_continue, has_return
                    ));
                }
            }
            outcome.recipe_contract = contract;
        }
    } else if let Some(ref facts) = outcome.facts {
        if facts.facts.scan_with_init.is_some()
            || facts.facts.split_scan.is_some()
            || facts.facts.pattern1_array_join.is_some()
            || facts.facts.pattern5_infinite_early_exit.is_some()
        {
            use crate::mir::builder::control_flow::plan::recipe_tree::RecipeMatcher;
            if gate.strict_or_dev {
                let contract =
                    RecipeMatcher::try_match_loop(facts).map_err(|freeze| freeze.to_string())?;
                outcome.recipe_contract = contract;
            } else if let Ok(contract) = RecipeMatcher::try_match_loop(facts) {
                outcome.recipe_contract = contract;
            }
        }
    }

    if gate.planner_required && !planner_present && outcome.facts.is_none() {
        return Err(freeze_planner_required_none(ctx));
    }

    let promotion_facts = outcome
        .facts
        .as_ref()
        .and_then(|facts| facts.facts.pattern2_loopbodylocal.as_ref())
        .map(|facts| &facts.shape);

    emit_loop_break_promotion_hint_tag(promotion_facts);

    for rule_id in PLAN_RULE_ORDER {
        let rule_id = *rule_id;
        let name = rule_name(rule_id);
        let planner_hit = planner_hits_rule(planner_present, rule_id);

        // Recipe-only rules route through compose path (planner payload suppressed).
        if planner_hit && is_recipe_only_rule(rule_id) {
            gate.log_planner_first(rule_id);
            debug_log_recipe_only_entry(rule_id);
            outcome.plan = None;
            return Ok(outcome);
        }

        if gate.planner_required && planner_hit {
            if let Some(plan) = outcome.plan.as_ref() {
                return Err(planner::Freeze::contract(&format!(
                    "planner_required forbids plan label={}",
                    loop_plan_label(plan)
                ))
                .to_string());
            }
        }

        if planner_hit {
            gate.log_planner_first(rule_id);
            if outcome.plan.is_some() {
                let log_msg = format!("route=plan strategy=extract rule={}", name);
                trace::trace().route("route", &log_msg, true);
                return Ok(outcome);
            }
        } else if !gate.planner_required && ctx.debug {
            let debug_msg = format!("{} extraction returned None, trying next rule", name);
            trace::trace().debug("route", &debug_msg);
        }
    }

    Ok(outcome)
}

fn planner_hits_rule(planner_present: bool, kind: PlanRuleId) -> bool {
    let hit = planner_matches_rule_kind(planner_present, kind);
    plan_trace::trace_try_take_planner(kind, planner_present, hit);
    hit
}

fn planner_matches_rule_kind(planner_present: bool, kind: PlanRuleId) -> bool {
    planner_present && matches!(kind, PlanRuleId::LoopCondContinueWithReturn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_only_rule_is_single_plan_payload_only() {
        assert!(is_recipe_only_rule(PlanRuleId::LoopCondContinueWithReturn));
        assert!(!is_recipe_only_rule(PlanRuleId::LoopBreakRecipe));
        assert!(!is_recipe_only_rule(PlanRuleId::IfPhiJoin));
    }

    #[test]
    fn loop_cond_continue_with_return_is_always_recipe_only() {
        assert!(is_recipe_only_rule(PlanRuleId::LoopCondContinueWithReturn));
    }

    #[test]
    fn promotion_hint_tag_matches_shape() {
        let trim = LoopBodyLocalShape::TrimSeg {
            s_var: "s".to_string(),
            i_var: "i".to_string(),
        };
        let digit = LoopBodyLocalShape::DigitPos {
            digits_var: "digits".to_string(),
            ch_var: "ch".to_string(),
        };
        assert_eq!(
            promotion_hint_tag(&trim),
            "[plan/loop_break/promotion_hint:TrimSeg]"
        );
        assert_eq!(
            promotion_hint_tag(&digit),
            "[plan/loop_break/promotion_hint:DigitPos]"
        );
    }
}
