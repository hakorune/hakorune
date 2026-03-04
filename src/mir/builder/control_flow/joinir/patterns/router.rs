//! Pattern Router - Plan/Composer routing for loop patterns
//!
//! Phase 29ap P12: Legacy loop table removed (plan/composer SSOT only)
//!
//! # Architecture
//!
//! - single_planner derives a DomainPlan + facts outcome (SSOT)
//! - composer adopts CorePlan (strict/dev shadow or release adopt)
//! - PlanLowerer emits MIR from CorePlan (emit_frag SSOT)
//!
//! # Adding New Patterns
//!
//! 1. Add Facts/Planner extraction in plan layer
//! 2. Normalize/verify in plan normalizer/verifier
//! 3. Compose CorePlan in composer (shadow/release adopt as needed)
//! 4. Keep router unchanged (it only delegates to plan/composer)

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use crate::mir::loop_pattern_detection::LoopPatternKind;

// Phase 273 P1: Import Plan components (DomainPlan → Normalizer → Verifier → Lowerer)
use super::registry;
use crate::mir::builder::control_flow::plan::composer;
use crate::mir::builder::control_flow::plan::expectations;
use crate::mir::builder::control_flow::plan::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::LoopCondBreakAcceptKind;
use crate::mir::builder::control_flow::plan::facts::reject_reason;
use crate::mir::builder::control_flow::plan::lowerer::PlanLowerer;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::observability::flowbox_tags::{self, FlowboxVia};
use crate::mir::builder::control_flow::plan::planner::{Freeze, PlanBuildOutcome};
use crate::mir::builder::control_flow::plan::single_planner;
use crate::mir::builder::control_flow::plan::verifier::PlanVerifier;
use crate::mir::builder::control_flow::plan::CorePlan;

/// Phase 92 P0-2: Import LoopSkeleton for Option A
use crate::mir::loop_canonicalizer::LoopSkeleton;

/// Context passed to pattern detect/lower functions
pub(crate) struct LoopPatternContext<'a> {
    /// Loop condition AST node
    pub condition: &'a ASTNode,

    /// Loop body statements
    pub body: &'a [ASTNode],

    /// Current function name (for routing)
    pub func_name: &'a str,

    /// Debug logging enabled
    pub debug: bool,

    /// In static box context? (affects Pattern8 routing)
    pub in_static_box: bool,

    /// Phase 192: Pattern classification based on features
    pub pattern_kind: LoopPatternKind,

    /// Phase 200-C: Optional function body AST for capture analysis
    /// None if not available, Some(&[ASTNode]) if function body is accessible
    pub fn_body: Option<&'a [ASTNode]>,

    /// Phase 92 P0-2: Optional LoopSkeleton from canonicalizer
    /// This provides ConditionalStep information for Pattern2 lowering.
    /// None if canonicalizer hasn't run yet (backward compatibility).
    /// SSOT Principle: Avoid re-detecting ConditionalStep in lowering phase.
    pub skeleton: Option<&'a LoopSkeleton>,

    /// Phase 188.3: Cached StepTree max_loop_depth for Pattern6
    /// None if not computed, Some(depth) if Pattern6 candidate
    /// Avoids re-building StepTree in lowering phase
    pub step_tree_max_loop_depth: Option<u32>,
}

impl<'a> LoopPatternContext<'a> {
    /// Create new context from routing parameters
    ///
    /// Automatically detects continue/break statements in body
    /// Extracts features and classifies pattern from AST
    /// Detects infinite loop condition
    /// Uses choose_pattern_kind() SSOT entry point
    pub(crate) fn new(
        condition: &'a ASTNode,
        body: &'a [ASTNode],
        func_name: &'a str,
        debug: bool,
        in_static_box: bool,
    ) -> Self {
        // Phase 137-6-S1: Use SSOT pattern selection entry point
        use crate::mir::builder::control_flow::joinir::routing::choose_pattern_kind;
        let pattern_kind = choose_pattern_kind(condition, body);

        Self {
            condition,
            body,
            func_name,
            debug,
            in_static_box,
            pattern_kind,
            fn_body: None,                  // Phase 200-C: Default to None
            skeleton: None,                 // Phase 92 P0-2: Default to None
            step_tree_max_loop_depth: None, // Phase 188.3: Default to None
        }
    }

    /// Phase 200-C: Create context with fn_body for capture analysis
    pub(crate) fn with_fn_body(
        condition: &'a ASTNode,
        body: &'a [ASTNode],
        func_name: &'a str,
        debug: bool,
        in_static_box: bool,
        fn_body: &'a [ASTNode],
    ) -> Self {
        let mut ctx = Self::new(condition, body, func_name, debug, in_static_box);
        ctx.fn_body = Some(fn_body);
        ctx
    }

}

// Phase 29ai P5: Plan extractor routing moved to `plan::single_planner`.

/// Route loop patterns via plan/composer SSOT.
///
/// Returns Ok(Some(value_id)) if a plan matched and lowered successfully.
/// Returns Ok(None) if no plan matched.
/// Returns Err if a plan matched but lowering failed.
///
/// # Router Architecture (Plan/Composer SSOT)
///
/// The plan line (Extractor → Normalizer → Verifier → Lowerer) is the
/// operational SSOT for loop routing (Phase 273+).
///
/// Plan-based architecture (Phase 273 P1-P3):
/// - extract_*_plan() → DomainPlan (pure extraction, no builder)
/// - PlanVerifier::verify() → fail-fast validation
/// - PlanLowerer::lower() → MIR emission (pattern-agnostic, emit_frag SSOT)
///
/// SSOT Entry Points:
/// - Pattern6: src/mir/builder/control_flow/plan/normalizer.rs (ScanWithInit normalization)
/// - Pattern7: src/mir/builder/control_flow/plan/normalizer.rs (SplitScan normalization)
pub(super) fn lower_verified_core_plan(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    strict_or_dev: bool,
    facts: Option<&CanonicalLoopFacts>,
    core_plan: CorePlan,
    via: FlowboxVia,
) -> Result<Option<ValueId>, String> {
    PlanVerifier::verify(&core_plan)?;
    flowbox_tags::emit_flowbox_adopt_tag_from_coreplan(strict_or_dev, &core_plan, facts, via);
    PlanLowerer::lower(builder, core_plan, ctx)
}

fn lower_shadow_adopt_pre_plan(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
    strict_or_dev: bool,
    outcome: &PlanBuildOutcome,
    allow_generic_loop: bool,
) -> Result<Option<ValueId>, String> {
    let Some(pre_plan) =
        composer::try_shadow_adopt_pre_plan(builder, ctx, outcome, allow_generic_loop)?
    else {
        return Ok(None);
    };

    match pre_plan {
        composer::PrePlanShadowOutcome::Adopt(adopt) => {
            let composer::ShadowAdoptOutcome {
                core_plan,
                emit_flowbox_adopt_tag,
            } = adopt;
            // Gate sentinel: in strict+planner_required mode, always emit FlowBox adopt tags
            // so planner-first gates can validate routing without enabling global debug logs.
            if emit_flowbox_adopt_tag
                || crate::config::env::joinir_dev::strict_planner_required_enabled()
            {
                return lower_verified_core_plan(
                    builder,
                    ctx,
                    strict_or_dev,
                    outcome.facts.as_ref(),
                    core_plan,
                    FlowboxVia::Shadow,
                );
            }
            PlanVerifier::verify(&core_plan)?;
            PlanLowerer::lower(builder, core_plan, ctx)
        }
        composer::PrePlanShadowOutcome::GuardError(err) => {
            flowbox_tags::emit_flowbox_freeze_tag_from_facts(
                strict_or_dev,
                "unstructured",
                outcome.facts.as_ref(),
            );
            if crate::config::env::joinir_dev::debug_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!("{}", err));
            }
            Err(err)
        }
    }
}

fn freeze_expected_plan(
    strict_or_dev: bool,
    facts: Option<&CanonicalLoopFacts>,
    tag: &'static str,
    message: &'static str,
) -> String {
    flowbox_tags::emit_flowbox_freeze_contract(strict_or_dev, tag, facts, message)
}

pub(crate) fn route_loop_pattern(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
) -> Result<Option<ValueId>, String> {
    use super::super::trace;

    // Phase 29ai P5: Single entrypoint for plan extraction (router has no rule table).
    let (domain_plan, outcome) = single_planner::try_build_domain_plan_with_outcome(ctx)?;
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    // loopbodylocal flowbox tagging is handled in the recipe-first Pattern2Break path
    // and must not depend on domain_plan.
    let has_loopbodylocal = outcome
        .facts
        .as_ref()
        .and_then(|f| f.facts.pattern2_loopbodylocal.as_ref())
        .is_some();

    let env = registry::RouterEnv {
        strict_or_dev,
        planner_required,
        has_loopbodylocal,
    };
    let debug_enabled = crate::config::env::joinir_dev::debug_enabled();
    let trace_entry_route = |route: &str| {
        if debug_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/trace:entry_route] ctx=loop_router route={}",
                route
            ));
        }
    };

    if strict_or_dev && planner_required {
        if debug_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/trace:entry_candidates_gate] strict_or_dev={} planner_required={} debug_enabled={}",
                strict_or_dev, planner_required, debug_enabled
            ));
        }
        let candidates = registry::collect_candidates(outcome.facts.as_ref());
        if debug_enabled {
            let list = if candidates.is_empty() {
                "none".to_string()
            } else {
                candidates.join(",")
            };
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[plan/trace:entry_candidates] ctx=loop_router candidates={}",
                list
            ));
        }
        if candidates.len() > 1 {
            return Err(Freeze::contract(&format!(
                "entry_ambiguous: candidates={}",
                candidates.join(",")
            ))
            .to_string());
        }
    }

    // In release, keep nested-loop recipe-first blocked by default.
    // Exception: loop_cond_break_continue with explicit exit-driven accept kinds.
    let release_recipe_first_allowed = if !detect_nested_loop(ctx.body) {
        true
    } else {
        outcome
            .facts
            .as_ref()
            .is_some_and(|facts| {
                if !(facts.exit_usage.has_break && facts.exit_usage.has_continue)
                    || facts.exit_usage.has_return
                {
                    return false;
                }
                let Some(loop_cond) = facts.facts.loop_cond_break_continue.as_ref() else {
                    return false;
                };
                !matches!(
                    loop_cond.accept_kind,
                    LoopCondBreakAcceptKind::NestedLoopOnly
                        | LoopCondBreakAcceptKind::ProgramBlockNoExit
                )
            })
    };
    if let Some(value) = registry::try_route_recipe_first(builder, ctx, &outcome, &env)? {
        if strict_or_dev || release_recipe_first_allowed {
            trace_entry_route("recipe_first");
            return Ok(Some(value));
        }
    }

    // recipe-first paths are handled by registry above.

    // Phase-1: recipe-first paths return above; reaching here means no recipe-first match.
    // Phase-2: do not attempt shadow_adopt if recipe-first matched (entry is locked earlier).
    // Phase-3: release also returns above when recipe-first matched (shadow_adopt fallback skipped).
    if strict_or_dev {
        if let Some(value) = lower_shadow_adopt_pre_plan(
            builder,
            ctx,
            strict_or_dev,
            &outcome,
            domain_plan.is_none(),
        )? {
            trace_entry_route("shadow_adopt");
            return Ok(Some(value));
        }
    }

    // Release fallback adopt is legacy; keep it scoped to planner-none routes only.
    if !strict_or_dev && domain_plan.is_none() {
        if let Some(core_plan) = composer::try_release_adopt_pre_plan(builder, ctx, &outcome, true)?
        {
            trace_entry_route("release_adopt");
            return lower_verified_core_plan(
                builder,
                ctx,
                strict_or_dev,
                outcome.facts.as_ref(),
                core_plan,
                FlowboxVia::Release,
            );
        }
    }

    if let Some(domain_plan) = domain_plan {
        if strict_or_dev && expectations::should_expect_shadow_adopt(&domain_plan, &outcome, ctx) {
            return Err(freeze_expected_plan(
                strict_or_dev,
                outcome.facts.as_ref(),
                "composer_reject",
                "composer reject for expected plan",
            ));
        }

        if strict_or_dev && expectations::should_expect_plan(&outcome, ctx) {
            return Err(freeze_expected_plan(
                strict_or_dev,
                outcome.facts.as_ref(),
                "composer_reject",
                "fallback to lower_via_plan for expected plan",
            ));
        }

        if strict_or_dev {
            return Err(freeze_expected_plan(
                strict_or_dev,
                outcome.facts.as_ref(),
                "legacy_forbidden",
                "legacy lower_via_plan is release-only",
            ));
        }

        // legacy::lower_via_plan was a no-op (always Ok(None)).
        // Keep the same release routing outcome without the legacy module hop.
        trace_entry_route("legacy");
        return Ok(None);
    }

    if strict_or_dev && expectations::should_expect_plan(&outcome, ctx) {
        return Err(freeze_expected_plan(
            strict_or_dev,
            outcome.facts.as_ref(),
            "planner_none",
            "planner returned None for expected loop facts",
        ));
    }

    // No pattern matched - return None (caller will handle error)
    let candidate_names = registry::collect_candidates(outcome.facts.as_ref());
    let candidate_text = if candidate_names.is_empty() {
        "none".to_string()
    } else {
        candidate_names.join(",")
    };
    reject_reason::set_last_plan_reject_detail_if_absent(format!(
        "route_exhausted func={} pattern_kind={:?} facts_present={} candidates={}",
        ctx.func_name,
        ctx.pattern_kind,
        outcome.facts.is_some(),
        candidate_text
    ));

    if ctx.debug {
        trace::trace().debug(
            "route",
            &format!(
                "route=none (no pattern matched) func='{}' pattern_kind={:?} (exhausted: plan+joinir)",
                ctx.func_name, ctx.pattern_kind
            ),
        );
    }
    trace_entry_route("none");
    Ok(None)
}
