//! Loop Route Router - plan/composer entry routing for loop lowering.
//!
//! Phase 29ap P12: Legacy loop table removed (plan/composer SSOT only)
//!
//! # Architecture
//!
//! - single_planner derives facts/recipe outcome (SSOT)
//! - composer provides strict/dev pre-plan guards + explicit compose helpers
//! - PlanLowerer emits MIR from CorePlan (emit_frag SSOT)
//!
//! # Adding New Loop Routes
//!
//! 1. Add Facts/Planner extraction in plan layer
//! 2. Normalize/verify in plan normalizer/verifier
//! 3. Compose CorePlan in composer (shadow/release adopt as needed)
//! 4. Keep router unchanged (it only delegates to plan/composer)

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use crate::mir::loop_route_detection::LoopRouteKind;

// Phase 273 P1: Import Plan components (facts/recipe outcome -> verifier -> lowerer)
use super::registry;
use crate::mir::builder::control_flow::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::control_flow::facts::reject_reason;
use crate::mir::builder::control_flow::lower::expectations;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::lower::{
    try_build_outcome, CorePlan, Freeze, PlanBuildOutcome, PlanLowerer,
};
use crate::mir::builder::control_flow::plan::composer::shadow_pre_plan_guard_error;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::{self, FlowboxVia};
use crate::mir::builder::control_flow::verify::PlanVerifier;

/// Phase 92 P0-2: Import LoopSkeleton for Option A
use crate::mir::loop_canonicalizer::LoopSkeleton;

/// Context passed to loop route detection/lowering functions.
pub(crate) struct LoopRouteContext<'a> {
    /// Loop condition AST node
    pub condition: &'a ASTNode,

    /// Loop body statements
    pub body: &'a [ASTNode],

    /// Current function name (for routing)
    pub func_name: &'a str,

    /// Debug logging enabled
    pub debug: bool,

    /// In static box context? (affects scan-predicate route behavior)
    pub in_static_box: bool,

    /// Phase 192: Loop route classification based on features.
    pub route_kind: LoopRouteKind,

    /// Phase 200-C: Optional function body AST for capture analysis
    /// None if not available, Some(&[ASTNode]) if function body is accessible
    pub fn_body: Option<&'a [ASTNode]>,

    /// Phase 92 P0-2: Optional LoopSkeleton from canonicalizer
    /// This provides ConditionalStep information for loop-break recipe lowering.
    /// None if canonicalizer hasn't run yet (backward compatibility).
    /// SSOT Principle: Avoid re-detecting ConditionalStep in lowering phase.
    pub skeleton: Option<&'a LoopSkeleton>,

    /// Phase 188.3: Cached StepTree max_loop_depth for nested-loop minimal routes.
    /// None if not computed, Some(depth) when nested-loop candidate is present.
    /// Avoids re-building StepTree in lowering phase
    pub step_tree_max_loop_depth: Option<u32>,
}

impl<'a> LoopRouteContext<'a> {
    /// Create new context from routing parameters
    ///
    /// Automatically detects continue/break statements in body
    /// Extracts features and classifies route kind from AST
    /// Detects infinite loop condition
    /// Uses choose_route_kind() SSOT entry point
    pub(crate) fn new(
        condition: &'a ASTNode,
        body: &'a [ASTNode],
        func_name: &'a str,
        debug: bool,
        in_static_box: bool,
    ) -> Self {
        // Phase 137-6-S1: Use SSOT route selection entry point
        use crate::mir::builder::control_flow::joinir::routing::choose_route_kind;
        let route_kind = choose_route_kind(condition, body);

        Self {
            condition,
            body,
            func_name,
            debug,
            in_static_box,
            route_kind,
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

/// Route loops via plan/composer SSOT.
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
/// - try_build_outcome() → facts/recipe outcome (pure extraction, no builder)
/// - PlanVerifier::verify() → fail-fast validation
/// - PlanLowerer::lower() → MIR emission (route-agnostic, emit_frag SSOT)
///
/// SSOT entry points:
/// - `scan_with_init`: `src/mir/builder/control_flow/plan/normalizer.rs`
/// - `split_scan`: `src/mir/builder/control_flow/plan/normalizer.rs`
pub(super) fn lower_verified_core_plan(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    strict_or_dev: bool,
    facts: Option<&CanonicalLoopFacts>,
    core_plan: CorePlan,
    via: FlowboxVia,
) -> Result<Option<ValueId>, String> {
    PlanVerifier::verify(&core_plan)?;
    flowbox_tags::emit_flowbox_adopt_tag_from_coreplan(strict_or_dev, &core_plan, facts, via);
    PlanLowerer::lower(builder, core_plan, ctx)
}

fn enforce_shadow_adopt_pre_plan_guard(
    ctx: &LoopRouteContext,
    strict_or_dev: bool,
    outcome: &PlanBuildOutcome,
) -> Result<(), String> {
    let Some(err) = shadow_pre_plan_guard_error(ctx, outcome) else {
        return Ok(());
    };
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

fn freeze_expected_plan(
    strict_or_dev: bool,
    facts: Option<&CanonicalLoopFacts>,
    tag: &'static str,
    message: &'static str,
) -> String {
    flowbox_tags::emit_flowbox_freeze_contract(strict_or_dev, tag, facts, message)
}

fn release_allows_nested_recipe_first(outcome: &PlanBuildOutcome) -> bool {
    let Some(facts) = outcome.facts.as_ref() else {
        return false;
    };

    if facts.facts.nested_loop_minimal().is_some() {
        return true;
    }
    if facts.facts.generic_loop_v1().is_some() || facts.facts.generic_loop_v0().is_some() {
        return true;
    }

    // Phase C15/C16 scan families already have recipe-first pipelines and fast gates.
    // Keep release nested-loop policy aligned with those migrated routes.
    if facts.facts.loop_scan_methods_v0().is_some()
        || facts.facts.loop_scan_methods_block_v0().is_some()
        || facts.facts.loop_scan_phi_vars_v0().is_some()
        || facts.facts.loop_scan_v0().is_some()
        || facts.facts.loop_collect_using_entries_v0().is_some()
        || facts.facts.loop_bundle_resolver_v0().is_some()
    {
        return true;
    }

    if facts.facts.loop_cond_continue_only().is_some() {
        return true;
    }

    if !(facts.exit_usage.has_break && facts.exit_usage.has_continue) || facts.exit_usage.has_return
    {
        return false;
    }
    let Some(loop_cond) = facts.facts.loop_cond_break_continue() else {
        return false;
    };
    loop_cond.release_allowed()
}

pub(crate) fn route_loop(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
) -> Result<Option<ValueId>, String> {
    use super::super::trace;

    // Phase 29ai P5: Single entrypoint for plan extraction (router has no rule table).
    let outcome = try_build_outcome(ctx)?;
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    let planner_required =
        strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
    // body-local flowbox tagging is handled in the recipe-first loop_break_recipe path
    // and must not depend on legacy planner-only state.
    let has_body_local = outcome
        .facts
        .as_ref()
        .and_then(|f| f.facts.loop_break_body_local())
        .is_some();

    let env = registry::RouterEnv {
        strict_or_dev,
        planner_required,
        has_body_local,
    };
    let allow_shadow_fallback = outcome.recipe_contract.is_none();
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
    // Exceptions:
    // - nested_loop_minimal facts (same compose contract as release_adopt nested-minimal lane)
    // - generic_loop_v{1,0} facts (recipe-first best-effort; only no-match `Ok(None)` continues routing, `Err` propagates)
    // - migrated scan families (Phase C15/C16 recipe-first pipelines are already gated)
    // - loop_cond_break_continue with explicit exit-driven accept kinds.
    let release_recipe_first_allowed = if !detect_nested_loop(ctx.body) {
        true
    } else {
        release_allows_nested_recipe_first(&outcome)
    };
    let recipe_first_allowed = strict_or_dev || release_recipe_first_allowed;
    if recipe_first_allowed {
        if let Some(value) = registry::try_route_recipe_first(builder, ctx, &outcome, &env)? {
            trace_entry_route("recipe_first");
            return Ok(Some(value));
        }
    }

    // recipe-first paths are handled by registry above.

    // Phase-1: recipe-first paths return above; reaching here means no recipe-first match.
    // Phase-2: shadow pre-plan is guard-only (no adopt path in router).
    // Phase-3: release also returns above when recipe-first matched (shadow guard skipped).
    if strict_or_dev && allow_shadow_fallback {
        enforce_shadow_adopt_pre_plan_guard(ctx, strict_or_dev, &outcome)?;
    }

    if strict_or_dev && expectations::should_expect_plan(&outcome, ctx) {
        return Err(freeze_expected_plan(
            strict_or_dev,
            outcome.facts.as_ref(),
            "planner_none",
            "planner returned None for expected loop facts",
        ));
    }

    // No route matched - return None (caller will handle error)
    let candidate_names = registry::collect_candidates(outcome.facts.as_ref());
    let candidate_text = if candidate_names.is_empty() {
        "none".to_string()
    } else {
        candidate_names.join(",")
    };
    reject_reason::set_last_plan_reject_detail_if_absent(format!(
        "route_exhausted func={} loop_kind={} facts_present={} candidates={}",
        ctx.func_name,
        ctx.route_kind.semantic_label(),
        outcome.facts.is_some(),
        candidate_text
    ));

    if ctx.debug {
        trace::trace().debug(
            "route",
            &format!(
                "route=none (no route matched) func='{}' loop_kind={} (exhausted: plan+joinir)",
                ctx.func_name,
                ctx.route_kind.semantic_label()
            ),
        );
    }
    trace_entry_route("none");
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::release_allows_nested_recipe_first;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::facts::feature_facts::LoopFeatureFacts;
    use crate::mir::builder::control_flow::facts::loop_scan_methods_block_v0::try_extract_loop_scan_methods_block_v0_facts;
    use crate::mir::builder::control_flow::facts::loop_scan_methods_v0::try_extract_loop_scan_methods_v0_facts;
    use crate::mir::builder::control_flow::facts::scan_shapes::{ConditionShape, StepShape};
    use crate::mir::builder::control_flow::facts::skeleton_facts::{SkeletonFacts, SkeletonKind};
    use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
    use crate::mir::builder::control_flow::lower::PlanBuildOutcome;
    use crate::mir::builder::control_flow::plan::facts::loop_types::LoopFacts;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn string(value: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(value.to_string()),
            span: Span::unknown(),
        }
    }

    fn binop(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn local(name: &str, init: Option<ASTNode>) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![init.map(Box::new)],
            span: Span::unknown(),
        }
    }

    fn sample_condition() -> ASTNode {
        binop(BinaryOperator::Less, var("i"), var("n"))
    }

    fn base_loop_facts() -> LoopFacts {
        LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
                ..Default::default()
            },
            features: LoopFeatureFacts {
                nested_loop: true,
                ..Default::default()
            },
            scan_with_init: None,
            split_scan: None,
            loop_simple_while: None,
            loop_char_map: None,
            loop_array_join: None,
            string_is_integer: None,
            starts_with: None,
            int_to_str: None,
            escape_map: None,
            split_lines: None,
            skip_whitespace: None,
            generic_loop_v0: None,
            generic_loop_v1: None,
            if_phi_join: None,
            loop_continue_only: None,
            loop_true_early_exit: None,
            loop_true_break_continue: None,
            loop_cond_break_continue: None,
            loop_cond_continue_only: None,
            loop_cond_continue_with_return: None,
            loop_cond_return_in_body: None,
            loop_scan_v0: None,
            loop_scan_methods_block_v0: None,
            loop_scan_methods_v0: None,
            loop_scan_phi_vars_v0: None,
            loop_collect_using_entries_v0: None,
            loop_bundle_resolver_v0: None,
            nested_loop_minimal: None,
            bool_predicate_scan: None,
            accum_const_loop: None,
            loop_break: None,
            loop_break_body_local: None,
        }
    }

    fn nested_outcome_with_block_facts() -> PlanBuildOutcome {
        let mut facts = base_loop_facts();
        let condition = sample_condition();
        let inner_loop_body = vec![
            ASTNode::If {
                condition: Box::new(binop(
                    BinaryOperator::Equal,
                    ASTNode::MethodCall {
                        object: Box::new(var("s")),
                        method: "substring".to_string(),
                        arguments: vec![var("j"), binop(BinaryOperator::Add, var("j"), var("m"))],
                        span: Span::unknown(),
                    },
                    var("pat"),
                )),
                then_body: vec![
                    assign(var("k"), var("j")),
                    ASTNode::Break {
                        span: Span::unknown(),
                    },
                ],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("j"), binop(BinaryOperator::Add, var("j"), int(1))),
        ];
        let body = vec![
            local("next_i", Some(int(0))),
            local("k", Some(int(0))),
            local("name_start", Some(int(0))),
            ASTNode::Program {
                statements: vec![
                    local("pat", Some(string("p"))),
                    local("m", Some(int(1))),
                    local("j", Some(int(0))),
                    ASTNode::Loop {
                        condition: Box::new(binop(
                            BinaryOperator::LessEqual,
                            binop(BinaryOperator::Add, var("j"), var("m")),
                            var("n"),
                        )),
                        body: inner_loop_body,
                        span: Span::unknown(),
                    },
                ],
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::LessEqual, var("next_i"), var("i"))),
                then_body: vec![assign(
                    var("next_i"),
                    binop(BinaryOperator::Add, var("i"), int(1)),
                )],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("i"), var("next_i")),
        ];
        facts.loop_scan_methods_block_v0 = Some(
            try_extract_loop_scan_methods_block_v0_facts(&condition, &body)
                .expect("extract ok")
                .expect("block facts"),
        );
        PlanBuildOutcome {
            facts: Some(canonicalize_loop_facts(facts)),
            recipe_contract: None,
        }
    }

    fn nested_outcome_with_scan_methods_facts() -> PlanBuildOutcome {
        let mut facts = base_loop_facts();
        let condition = sample_condition();
        let body = vec![
            local("next_i", Some(int(0))),
            local("j", Some(int(0))),
            local("m", Some(int(0))),
            ASTNode::Loop {
                condition: Box::new(binop(
                    BinaryOperator::LessEqual,
                    binop(BinaryOperator::Add, var("j"), var("m")),
                    var("n"),
                )),
                body: vec![assign(
                    var("j"),
                    binop(BinaryOperator::Add, var("j"), int(1)),
                )],
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(binop(BinaryOperator::LessEqual, var("next_i"), var("i"))),
                then_body: vec![assign(
                    var("next_i"),
                    binop(BinaryOperator::Add, var("i"), int(1)),
                )],
                else_body: None,
                span: Span::unknown(),
            },
            assign(var("i"), var("next_i")),
        ];
        facts.loop_scan_methods_v0 = Some(
            try_extract_loop_scan_methods_v0_facts(&condition, &body)
                .expect("extract ok")
                .expect("scan methods facts"),
        );
        PlanBuildOutcome {
            facts: Some(canonicalize_loop_facts(facts)),
            recipe_contract: None,
        }
    }

    #[test]
    fn release_nested_recipe_first_allows_scan_methods_block_family() {
        assert!(release_allows_nested_recipe_first(
            &nested_outcome_with_block_facts()
        ));
    }

    #[test]
    fn release_nested_recipe_first_allows_scan_methods_v0_family() {
        assert!(release_allows_nested_recipe_first(
            &nested_outcome_with_scan_methods_facts()
        ));
    }

    #[test]
    fn release_nested_recipe_first_rejects_unclassified_nested_loop() {
        let outcome = PlanBuildOutcome {
            facts: Some(canonicalize_loop_facts(base_loop_facts())),
            recipe_contract: None,
        };
        assert!(!release_allows_nested_recipe_first(&outcome));
    }
}
