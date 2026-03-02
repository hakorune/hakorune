//! Phase 29ai P5: Rule list (order SSOT) + guards
//!
//! IMPORTANT: Keep rule order identical to the legacy `PLAN_EXTRACTORS` table
//! (observability/behavior must not change).

use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;

use crate::mir::builder::control_flow::plan::facts::pattern2_loopbodylocal_facts::LoopBodyLocalShape;
use crate::mir::builder::control_flow::plan::planner::{self, PlanBuildOutcome, PlannerContext};
use crate::mir::builder::control_flow::plan::trace as plan_trace;
use crate::mir::builder::control_flow::plan::DomainPlan;

use super::rule_order::{rule_name, PlanRuleId, PLAN_RULE_ORDER};

struct PlannerGate {
    strict_or_dev: bool,
    planner_required: bool,
}

impl PlannerGate {
    fn new() -> Self {
        let strict_or_dev =
            crate::config::env::joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
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

fn freeze_planner_required_none(ctx: &LoopPatternContext) -> String {
    let mut msg = format!(
        "planner required, but planner returned None (legacy fallback forbidden): func={} cond={} body_len={}",
        ctx.func_name,
        ctx.condition.node_type(),
        ctx.body.len()
    );
    if let Some(detail) = crate::mir::builder::control_flow::plan::facts::reject_reason::take_last_plan_reject_detail() {
        msg.push_str(&format!("\nDetail: [joinir/reject_detail] {detail}"));
    }
    planner::Freeze::contract(&msg)
    .with_hint("Disable HAKO_JOINIR_PLANNER_REQUIRED, or extend Facts→Planner coverage for this case.")
    .to_string()
}

fn freeze_planner_required_mismatch() -> String {
    planner::Freeze::bug(
        "planner required, but DomainPlan did not match any rule (internal mismatch)",
    )
    .with_hint("Check DomainPlan variant ↔ PlanRuleId mapping in try_take_planner().")
    .to_string()
}

pub(super) fn try_build_domain_plan_with_outcome(
    ctx: &LoopPatternContext,
) -> Result<(Option<DomainPlan>, PlanBuildOutcome), String> {
    use crate::mir::builder::control_flow::joinir::trace;

    let gate = PlannerGate::new();

    let planner_ctx = PlannerContext {
        pattern_kind: ctx.skeleton.map(|_| ctx.pattern_kind),
        in_static_box: ctx.in_static_box,
        debug: ctx.debug,
    };
    let mut outcome = planner::build_plan_with_facts_ctx(&planner_ctx, ctx.condition, ctx.body)
        .map_err(|freeze| freeze.to_string())?;
    let planner_opt = outcome.plan.clone();

    // Phase B: Recipe-first parallel path (planner_required only)
    if gate.planner_required {
        if let Some(ref facts) = outcome.facts {
            use crate::mir::builder::control_flow::plan::recipe_tree::contracts::RecipeContractKind;
            use crate::mir::builder::control_flow::plan::recipe_tree::RecipeMatcher;

            let contract = RecipeMatcher::try_match_loop(facts)
                .map_err(|freeze| freeze.to_string())?;
            if crate::config::env::joinir_dev::debug_enabled() {
                if let Some(ref c) = contract {
                    let (has_break, has_continue, has_return) = match &c.kind {
                        RecipeContractKind::LoopWithExit { has_break, has_continue, has_return } => {
                            (*has_break, *has_continue, *has_return)
                        }
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
                let contract = RecipeMatcher::try_match_loop(facts)
                    .map_err(|freeze| freeze.to_string())?;
                outcome.recipe_contract = contract;
            } else if let Ok(contract) = RecipeMatcher::try_match_loop(facts) {
                outcome.recipe_contract = contract;
            }
        }
    }

    if gate.planner_required && planner_opt.is_none() && outcome.facts.is_none() {
        return Err(freeze_planner_required_none(ctx));
    }

    let promotion_facts = outcome
        .facts
        .as_ref()
        .and_then(|facts| facts.facts.pattern2_loopbodylocal.as_ref());

    let mut planner_matched_any_rule = false;
    for rule_id in PLAN_RULE_ORDER {
        let rule_id = *rule_id;
        let name = rule_name(rule_id);
        let planner_hit = try_take_planner(&planner_opt, rule_id);

        // Phase C: Pattern2Break requires recipe contract (planner_required only)
        if gate.planner_required && matches!(rule_id, PlanRuleId::Pattern2) {
            if planner_hit.is_some() && outcome.recipe_contract.is_none() {
                return Err(planner::Freeze::contract(
                    "Pattern2Break requires recipe_contract in planner_required mode",
                )
                .to_string());
            }
            if crate::config::env::joinir_dev::debug_enabled() && planner_hit.is_some() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug("[recipe:entry] pattern2_break: recipe_contract enforced");
            }
        }

        // Phase C4: Pattern2Break is recipe-only in planner_required mode
        // Return (None, outcome) so router uses Recipe-first compose path
        if gate.planner_required && matches!(rule_id, PlanRuleId::Pattern2) {
            if planner_hit.is_some() {
                // Phase C5: Emit planner_first tag for gate verification
                gate.log_planner_first(rule_id);
                if crate::config::env::joinir_dev::debug_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug("[recipe:entry] pattern2_break: recipe-only (domain_plan suppressed)");
                }
                return Ok((None, outcome));
            }
        }

        // Phase C8: Pattern3IfPhi is recipe-only in planner_required mode
        // Return (None, outcome) so router uses Recipe-first compose path
        if gate.planner_required && matches!(rule_id, PlanRuleId::Pattern3) {
            if planner_hit.is_some() {
                gate.log_planner_first(rule_id);
                if crate::config::env::joinir_dev::debug_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug("[recipe:entry] pattern3_ifphi: recipe-only (domain_plan suppressed)");
                }
                return Ok((None, outcome));
            }
        }

        // Phase C9: Pattern4Continue is recipe-only in planner_required mode
        // Return (None, outcome) so router uses Recipe-first compose path
        if gate.planner_required && matches!(rule_id, PlanRuleId::Pattern4) {
            if planner_hit.is_some() {
                gate.log_planner_first(rule_id);
                if crate::config::env::joinir_dev::debug_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug("[recipe:entry] pattern4_continue: recipe-only (domain_plan suppressed)");
                }
                return Ok((None, outcome));
            }
        }

        // Phase 29bq P2.x: LoopCondContinueWithReturn is recipe-only in all modes
        // Return (None, outcome) so router uses Recipe-first compose path
        if matches!(rule_id, PlanRuleId::LoopCondContinueWithReturn) {
            if planner_hit.is_some() {
                gate.log_planner_first(rule_id);
                if crate::config::env::joinir_dev::debug_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug("[recipe:entry] loop_cond_continue_with_return: recipe-only (domain_plan suppressed)");
                }
                return Ok((None, outcome));
            }
        }

        // Phase C10: Pattern5InfiniteEarlyExit is recipe-only in planner_required mode
        // Return (None, outcome) so router uses Recipe-first compose path
        if gate.planner_required && matches!(rule_id, PlanRuleId::Pattern5) {
            if planner_hit.is_some() {
                gate.log_planner_first(rule_id);
                if crate::config::env::joinir_dev::debug_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug("[recipe:entry] pattern5_infinite_early_exit: recipe-only (domain_plan suppressed)");
                }
                return Ok((None, outcome));
            }
        }

        if gate.planner_required {
            if let Some(domain) = planner_hit.as_ref() {
                return Err(planner::Freeze::contract(&format!(
                    "planner_required forbids DomainPlan: {:?}",
                    domain
                ))
                .to_string());
            }
        }

        let (plan_opt, log_none) = if planner_hit.is_some() {
            planner_matched_any_rule = true;
            gate.log_planner_first(rule_id);
            (planner_hit, false)
        } else if gate.planner_required {
            (None, false)
        } else {
            (fallback_extract(ctx, rule_id)?, true)
        };

        let promotion_tag = if matches!(rule_id, PlanRuleId::Pattern2)
            && crate::config::env::joinir_dev::strict_enabled()
        {
            promotion_facts.map(|facts| match facts.shape {
                LoopBodyLocalShape::TrimSeg { .. } => "[plan/pattern2/promotion_hint:TrimSeg]",
                LoopBodyLocalShape::DigitPos { .. } => "[plan/pattern2/promotion_hint:DigitPos]",
            })
        } else {
            None
        };

        if let Some(tag) = promotion_tag {
            let ring0 = crate::runtime::get_global_ring0();
            // Gate sentinel: promotion hints are validated by strict-only smokes.
            // Emit prefix-free to stderr so it does not depend on `NYASH_RING0_LOG_LEVEL`.
            let _ = ring0.io.stderr_write(format!("{}\n", tag).as_bytes());
        }

        if let Some(domain_plan) = plan_opt {
            let log_msg = format!("route=plan strategy=extract pattern={}", name);
            trace::trace().pattern("route", &log_msg, true);
            return Ok((Some(domain_plan), outcome));
        } else if log_none && ctx.debug {
            let debug_msg = format!("{} extraction returned None, trying next pattern", name);
            trace::trace().debug("route", &debug_msg);
        }
    }

    if gate.planner_required && planner_opt.is_some() && !planner_matched_any_rule {
        return Err(freeze_planner_required_mismatch());
    }

    Ok((None, outcome))
}

fn try_take_planner(planner_opt: &Option<DomainPlan>, kind: PlanRuleId) -> Option<DomainPlan> {
    use crate::mir::builder::control_flow::plan::DomainPlan;

    let planner_present = planner_opt.is_some();
    let taken = match kind {
        PlanRuleId::LoopCondContinueWithReturn => {
            if let Some(DomainPlan::LoopCondContinueWithReturn(_)) = planner_opt {
                planner_opt.clone()
            } else {
                None
            }
        }
        _ => None,
    };

    plan_trace::trace_try_take_planner(kind, planner_present, taken.is_some());
    taken
}

fn fallback_extract(
    _ctx: &LoopPatternContext,
    kind: PlanRuleId,
) -> Result<Option<DomainPlan>, String> {
    match kind {
        PlanRuleId::Pattern1 => Ok(None),
        PlanRuleId::Pattern2 => Ok(None),
        PlanRuleId::Pattern3 => Ok(None),
        PlanRuleId::Pattern4 => Ok(None),
        PlanRuleId::Pattern5 => Ok(None),
        PlanRuleId::LoopTrueBreak => Ok(None),
        PlanRuleId::LoopCondBreak => Ok(None),
        PlanRuleId::LoopCondContinueOnly => Ok(None),
        PlanRuleId::LoopCondContinueWithReturn => Ok(None),
        PlanRuleId::LoopCondReturnInBody => Ok(None),
        PlanRuleId::Pattern6 => Ok(None),
        PlanRuleId::Pattern7 => Ok(None),
        PlanRuleId::Pattern8 => Ok(None),
        PlanRuleId::Pattern9 => Ok(None),
    }
}
