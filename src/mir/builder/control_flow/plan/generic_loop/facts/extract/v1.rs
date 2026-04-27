use crate::ast::{ASTNode, BinaryOperator, Span};
use crate::mir::builder::control_flow::facts::canon::generic_loop::{
    canon_condition_for_generic_loop_v0, canon_loop_increment_for_var, classify_step_placement,
    matches_loop_increment, StepPlacement,
};
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::facts::stmt_view::flatten_scope_boxes;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::{
    try_build_exit_allowed_block_recipe, ExitAllowedBlockRecipe,
};
use crate::mir::builder::control_flow::plan::facts::reject_reason::{
    handoff_tables, log_reject, RejectReason,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::{RecipeBlock, RecipeBodies};
use crate::mir::builder::control_flow::plan::single_planner::PlanRuleId;
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::policies::BodyLoweringPolicy;

use super::super::super::body_check::shape_resolution::{
    check_body_generic_v0, detect_generic_loop_v1_shape,
};
use super::super::super::body_check::step_validation::{
    has_control_flow_after_step, validate_break_else_if_step, validate_continue_if_step,
    validate_in_body_step_v1,
};
use super::super::super::facts_helpers::reject_or_none;
use super::super::super::facts_types::GenericLoopV1Facts;
use super::collection::{
    body_has_break_or_continue_stmt, collect_loop_var_candidates_from_body, has_continue_recursive,
};

#[derive(Default)]
struct V1RejectCounters {
    no_increment: usize,
    step_placement_none: usize,
    control_flow_after_step: usize,
    step_invalid: usize,
    recipe_unbuildable: usize,
    v0_guard: usize,
}

struct StepResolution {
    loop_increment: ASTNode,
    use_body_managed_step: bool,
}

enum StepResolutionErr {
    NoIncrement,
    StepPlacementNone,
    ControlFlowAfterStep,
    StepInvalid,
    AbortNone,
    Freeze(Freeze),
}

/// Attempts to extract generic loop v1 facts from a loop condition and body
pub(in crate::mir::builder) fn try_extract_generic_loop_v1_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<GenericLoopV1Facts>, Freeze> {
    let flat_body = flatten_scope_boxes(body);
    let strict = crate::config::env::joinir_dev::strict_enabled();
    let strict_or_dev = strict || crate::config::env::joinir_dev_enabled();
    let debug_enabled = crate::config::env::joinir_dev::debug_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    // Release route also needs var-step extraction for selfhost loops where
    // index progression uses non-literal steps (e.g. i = next_i / i = k).
    let allow_var_step = true;
    let has_break_or_continue = flat_body.iter().any(body_has_break_or_continue_stmt);
    let base_body_lowering_policy = if planner_required && !has_break_or_continue {
        BodyLoweringPolicy::ExitAllowed {
            allow_join_if: false,
        }
    } else {
        BodyLoweringPolicy::RecipeOnly
    };

    // Release route also needs extended condition canonicalization for short-circuit
    // forms used by selfhost fixtures (e.g. `a < b && call(...)`).
    let Some(canon) = canon_condition_for_generic_loop_v0(condition, true) else {
        log_reject(
            "generic_loop_v1",
            RejectReason::UnsupportedCondition,
            handoff_tables::for_generic_loop,
        );
        return Ok(None);
    };
    let cond_profile = canon.cond_profile.clone();
    let preferred_loop_var = preferred_loop_var_from_condition(condition);

    let mut loop_var_candidates = canon.loop_var_candidates;
    if loop_var_candidates.is_empty() {
        loop_var_candidates = collect_loop_var_candidates_from_body(&flat_body);
    }

    let raw_candidates = loop_var_candidates.len();
    let mut reject = V1RejectCounters::default();

    let mut matches = Vec::new();
    let mut logged_planner_first = false;
    for loop_var in &loop_var_candidates {
        let StepResolution {
            loop_increment,
            use_body_managed_step,
        } = match resolve_step_for_candidate(
            loop_var,
            preferred_loop_var.as_deref(),
            &flat_body,
            allow_var_step,
            planner_required,
            strict,
        ) {
            Ok(step) => step,
            Err(StepResolutionErr::NoIncrement) => {
                reject.no_increment += 1;
                continue;
            }
            Err(StepResolutionErr::StepPlacementNone) => {
                reject.step_placement_none += 1;
                continue;
            }
            Err(StepResolutionErr::ControlFlowAfterStep) => {
                reject.control_flow_after_step += 1;
                continue;
            }
            Err(StepResolutionErr::StepInvalid) => {
                reject.step_invalid += 1;
                continue;
            }
            Err(StepResolutionErr::AbortNone) => {
                return Ok(None);
            }
            Err(StepResolutionErr::Freeze(freeze)) => {
                return Err(freeze);
            }
        };

        let shape_id =
            detect_generic_loop_v1_shape(&flat_body, loop_var, &loop_increment, condition)?;

        let mut body_lowering_policy = base_body_lowering_policy;
        let body_for_recipe: Vec<ASTNode> = if use_body_managed_step {
            flat_body.clone()
        } else {
            flat_body
                .iter()
                .filter(|stmt| !matches_loop_increment(stmt, loop_var, &loop_increment))
                .cloned()
                .collect()
        };
        let body_exit_allowed = if body_for_recipe.is_empty() {
            let mut arena = RecipeBodies::new();
            let body_id = arena.register(RecipeBody::new(Vec::new()));
            let block = RecipeBlock::new(body_id, Vec::new());
            Some(ExitAllowedBlockRecipe { arena, block })
        } else {
            try_build_exit_allowed_block_recipe(&body_for_recipe, true)
        };
        if planner_required && body_exit_allowed.is_none() {
            reject.recipe_unbuildable += 1;
            continue;
        }
        let body_no_exit = try_build_no_exit_block_recipe(&body_for_recipe, true);
        if matches!(body_lowering_policy, BodyLoweringPolicy::ExitAllowed { .. })
            && body_exit_allowed.is_none()
        {
            body_lowering_policy = BodyLoweringPolicy::RecipeOnly;
        }
        if shape_id.is_none() && body_exit_allowed.is_none() {
            if check_body_generic_v0(&flat_body, loop_var, &loop_increment).is_some() {
                reject.v0_guard += 1;
                continue;
            }
        }

        if planner_required
            && (shape_id.is_some() || body_exit_allowed.is_some())
            && !logged_planner_first
        {
            if strict_or_dev {
                let msg = crate::mir::builder::control_flow::plan::planner::tags::planner_first_tag_with_label(
                    PlanRuleId::LoopSimpleWhile,
                );
                if crate::config::env::joinir_dev::strict_planner_required_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    let _ = ring0.io.stderr_write(format!("{}\n", msg).as_bytes());
                } else if debug_enabled {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&msg);
                }
            }
            logged_planner_first = true;
        }

        matches.push(GenericLoopV1Facts {
            loop_var: loop_var.clone(),
            condition: condition.clone(),
            loop_increment,
            body: RecipeBody::new(flat_body.clone()),
            shape_id,
            body_lowering_policy,
            body_exit_allowed: body_exit_allowed.clone(),
            body_no_exit,
            cond_profile: cond_profile.clone(),
        });
    }

    if matches.is_empty() {
        if debug_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/trace:loop_var_candidates] ctx=generic_loop_v1 raw={} filtered={} reasons=no_increment={} step_placement_none={} control_flow_after_step={} step_invalid={} recipe_unbuildable={} v0_guard={}",
                raw_candidates,
                matches.len(),
                reject.no_increment,
                reject.step_placement_none,
                reject.control_flow_after_step,
                reject.step_invalid,
                reject.recipe_unbuildable,
                reject.v0_guard
            ));
        }
        if planner_required && reject.recipe_unbuildable > 0 {
            return Err(Freeze::unsupported(
                "generic_loop_v1: cannot build recipe for body",
            ));
        }
        let detailed_reject = format!(
            "box=generic_loop_v1 reason=no_valid_loop_var_candidates raw={} no_increment={} step_placement_none={} control_flow_after_step={} step_invalid={} recipe_unbuildable={} v0_guard={}",
            raw_candidates,
            reject.no_increment,
            reject.step_placement_none,
            reject.control_flow_after_step,
            reject.step_invalid,
            reject.recipe_unbuildable,
            reject.v0_guard
        );
        log_reject(
            "generic_loop_v1",
            RejectReason::NoValidLoopVarCandidates,
            handoff_tables::for_generic_loop,
        );
        crate::mir::builder::control_flow::plan::facts::reject_reason::set_last_plan_reject_detail(
            detailed_reject,
        );
        return Ok(None);
    }

    if matches.len() > 1 {
        log_reject(
            "generic_loop_v1",
            RejectReason::AmbiguousLoopVarCandidates,
            handoff_tables::for_generic_loop,
        );
        return reject_or_none(
            strict,
            RejectReason::AmbiguousLoopVarCandidates.as_freeze_message(),
        );
    }

    Ok(Some(matches.remove(0)))
}

fn resolve_step_for_candidate(
    loop_var: &str,
    preferred_loop_var: Option<&str>,
    flat_body: &[ASTNode],
    allow_var_step: bool,
    planner_required: bool,
    strict: bool,
) -> Result<StepResolution, StepResolutionErr> {
    let mut use_body_managed_step = false;
    let mut loop_increment = if let Some(increment) =
        canon_loop_increment_for_var(flat_body, loop_var, allow_var_step)
    {
        increment
    } else if should_use_body_managed_step(loop_var, preferred_loop_var, flat_body) {
        use_body_managed_step = true;
        ASTNode::Variable {
            name: loop_var.to_string(),
            span: Span::unknown(),
        }
    } else {
        return Err(StepResolutionErr::NoIncrement);
    };

    if use_body_managed_step {
        return Ok(StepResolution {
            loop_increment,
            use_body_managed_step: true,
        });
    }

    let step_decision = classify_step_placement(flat_body, loop_var, &loop_increment);
    if let Some(reason) = step_decision.reject_reason {
        log_reject("generic_loop_v1", reason, handoff_tables::for_generic_loop);
        match reject_or_none::<()>(strict, reason.as_freeze_message()) {
            Ok(None) => return Err(StepResolutionErr::AbortNone),
            Ok(Some(_)) => {
                return Err(StepResolutionErr::Freeze(Freeze::bug(
                    "generic_loop_v1: reject_or_none returned Some unexpectedly",
                )))
            }
            Err(freeze) => return Err(StepResolutionErr::Freeze(freeze)),
        }
    }
    let Some(step_placement) = step_decision.placement else {
        return Err(StepResolutionErr::StepPlacementNone);
    };

    if planner_required
        && matches!(step_placement, StepPlacement::InBody(idx) if has_control_flow_after_step(flat_body, idx))
    {
        return Err(StepResolutionErr::ControlFlowAfterStep);
    }

    let step_ok = match step_placement {
        StepPlacement::InBody(idx) => {
            validate_in_body_step_v1(flat_body, idx, loop_var, &loop_increment, strict)
                .map_err(StepResolutionErr::Freeze)?
        }
        StepPlacement::InContinueIf(idx) => {
            validate_continue_if_step(flat_body, idx, strict).map_err(StepResolutionErr::Freeze)?
        }
        StepPlacement::InBreakElseIf(idx) => validate_break_else_if_step(flat_body, idx, strict)
            .map_err(StepResolutionErr::Freeze)?,
        _ => true,
    };

    if !step_ok {
        if should_use_body_managed_step(loop_var, preferred_loop_var, flat_body) {
            use_body_managed_step = true;
            loop_increment = ASTNode::Variable {
                name: loop_var.to_string(),
                span: Span::unknown(),
            };
        } else {
            return Err(StepResolutionErr::StepInvalid);
        }
    }

    Ok(StepResolution {
        loop_increment,
        use_body_managed_step,
    })
}

/// Recipe-based hint for generic_loop_v1
pub(in crate::mir::builder) fn has_generic_loop_v1_recipe_hint(
    condition: &ASTNode,
    body: &[ASTNode],
) -> bool {
    let flat_body = flatten_scope_boxes(body);
    let allow_var_step = true;

    let Some(canon) = canon_condition_for_generic_loop_v0(condition, true) else {
        return false;
    };

    let mut candidates = canon.loop_var_candidates;

    if candidates.is_empty() {
        candidates = collect_loop_var_candidates_from_body(&flat_body);
    }
    if candidates.is_empty() {
        return false;
    }

    for loop_var in &candidates {
        let Some(loop_increment) =
            canon_loop_increment_for_var(&flat_body, loop_var, allow_var_step)
        else {
            continue;
        };

        let step_decision = classify_step_placement(&flat_body, loop_var, &loop_increment);
        let placement_ok = match step_decision.placement {
            Some(StepPlacement::Last) => true,
            Some(StepPlacement::InContinueIf(_)) | Some(StepPlacement::InBreakElseIf(_)) => true,
            Some(StepPlacement::InBody(_)) => false,
            None => false,
        };
        if !placement_ok {
            continue;
        }

        let has_nested_loop = flat_body.iter().any(|stmt| {
            matches!(
                stmt,
                ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. }
            )
        });
        if has_nested_loop {
            continue;
        }
        let has_continue = flat_body.iter().any(has_continue_recursive);
        if has_continue {
            continue;
        }

        let body_for_recipe: Vec<ASTNode> = flat_body
            .iter()
            .filter(|stmt| !matches_loop_increment(stmt, loop_var, &loop_increment))
            .cloned()
            .collect();
        let body_exit_allowed = if body_for_recipe.is_empty() {
            true
        } else {
            try_build_exit_allowed_block_recipe(&body_for_recipe, false).is_some()
        };
        if body_exit_allowed {
            return true;
        }
    }
    false
}

fn preferred_loop_var_from_condition(condition: &ASTNode) -> Option<String> {
    match condition {
        ASTNode::UnaryOp { operand, .. } => preferred_loop_var_from_condition(operand),
        ASTNode::BinaryOp {
            operator: BinaryOperator::And | BinaryOperator::Or,
            left,
            ..
        } => preferred_loop_var_from_condition(left),
        ASTNode::BinaryOp { left, .. } => extract_var_candidate_from_expr(left),
        _ => None,
    }
}

fn extract_var_candidate_from_expr(expr: &ASTNode) -> Option<String> {
    match expr {
        ASTNode::Variable { name, .. } => Some(name.clone()),
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add | BinaryOperator::Subtract,
            left,
            right,
            ..
        } => {
            if let ASTNode::Variable { name, .. } = left.as_ref() {
                return Some(name.clone());
            }
            if let ASTNode::Variable { name, .. } = right.as_ref() {
                return Some(name.clone());
            }
            None
        }
        _ => None,
    }
}

fn should_use_body_managed_step(
    loop_var: &str,
    preferred_loop_var: Option<&str>,
    body: &[ASTNode],
) -> bool {
    if preferred_loop_var != Some(loop_var) {
        return false;
    }
    has_top_level_loop_var_assignment(body, loop_var)
        || body.iter().any(ASTNode::contains_break_continue)
}

fn has_top_level_loop_var_assignment(body: &[ASTNode], loop_var: &str) -> bool {
    body.iter().any(|stmt| {
        matches!(
            stmt,
            ASTNode::Assignment { target, .. }
                if matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == loop_var)
        )
    })
}
