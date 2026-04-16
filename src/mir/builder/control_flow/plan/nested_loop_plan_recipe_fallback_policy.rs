//! Policy for nested-loop recipe fallback selection order.

use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum NestedLoopRecipeFallbackKind {
    ContinueWithReturn,
    BreakContinue,
}

pub(in crate::mir::builder) fn select_nested_loop_recipe_fallback(
    outcome: &PlanBuildOutcome,
    planner_required: bool,
) -> Option<NestedLoopRecipeFallbackKind> {
    if !planner_required {
        return None;
    }
    let facts = outcome.facts.as_ref()?;
    if facts.facts.loop_cond_continue_with_return.is_some() {
        return Some(NestedLoopRecipeFallbackKind::ContinueWithReturn);
    }
    if facts.facts.loop_cond_break_continue.is_some() {
        return Some(NestedLoopRecipeFallbackKind::BreakContinue);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
    use crate::mir::builder::control_flow::plan::facts::feature_facts::LoopFeatureFacts;
    use crate::mir::builder::control_flow::plan::facts::scan_shapes::{ConditionShape, StepShape};
    use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{
        SkeletonFacts, SkeletonKind,
    };
    use crate::mir::builder::control_flow::plan::facts::LoopFacts;
    use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::{
        LoopCondBreakAcceptKind, LoopCondBreakContinueFacts,
    };
    use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_facts::LoopCondContinueWithReturnFacts;
    use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;
    use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueRecipe;
    use crate::mir::builder::control_flow::recipes::loop_cond_continue_with_return::ContinueWithReturnRecipe;
    use crate::mir::policies::BodyLoweringPolicy;

    fn span() -> Span {
        Span::unknown()
    }

    fn bool_lit(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: span(),
        }
    }

    fn base_loop_facts() -> LoopFacts {
        LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
                feature_slots: vec![],
            },
            features: LoopFeatureFacts::default(),
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

    #[test]
    fn select_nested_loop_recipe_fallback_prefers_continue_with_return() {
        let mut facts = base_loop_facts();
        facts.loop_cond_continue_with_return = Some(LoopCondContinueWithReturnFacts {
            condition: bool_lit(true),
            recipe: ContinueWithReturnRecipe::new(vec![], vec![]),
        });
        facts.loop_cond_break_continue = Some(LoopCondBreakContinueFacts {
            accept_kind: LoopCondBreakAcceptKind::ContinueIf,
            propagate_nested_carriers: false,
            condition: bool_lit(true),
            recipe: LoopCondBreakContinueRecipe::new(vec![], vec![]),
            has_handled_guard_break: false,
            handled_var_name: None,
            continue_branches: vec![],
            body_lowering_policy: BodyLoweringPolicy::RecipeOnly,
            body_exit_allowed: None,
        });
        let outcome = PlanBuildOutcome {
            facts: Some(canonicalize_loop_facts(facts)),
            recipe_contract: None,
        };

        let selected = select_nested_loop_recipe_fallback(&outcome, true);

        assert_eq!(
            selected,
            Some(NestedLoopRecipeFallbackKind::ContinueWithReturn)
        );
    }

    #[test]
    fn select_nested_loop_recipe_fallback_skips_when_not_required() {
        let outcome = PlanBuildOutcome {
            facts: None,
            recipe_contract: None,
        };

        let selected = select_nested_loop_recipe_fallback(&outcome, false);

        assert_eq!(selected, None);
    }
}
