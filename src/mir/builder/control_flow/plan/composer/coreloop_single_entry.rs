//! Phase 29bb P1: CoreLoopComposer single entry (v0/v1/v2 selector).

use super::coreloop_gates::{
    coreloop_base_gate, coreloop_value_join_gate, exit_kinds_allow_return_only,
};
use super::coreloop_v0::try_compose_core_loop_v0;
use super::coreloop_v1::{
    try_compose_core_loop_v1_if_phi_join, try_compose_core_loop_v1_loop_break,
    try_compose_core_loop_v1_loop_true_early_exit,
};
use super::coreloop_v2_nested_minimal::try_compose_core_loop_v2_nested_minimal;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
    cond_profile_from_scan_shapes, match_scan_with_init_shape, ConditionShape, SplitScanShape,
};
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::{
    scan_direction_from_step_lit, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;

/// Unified ScanWithInit composer with all v0/v1 gate conditions preserved.
/// - v1 path always returns None (scan_with_init not supported with value_join)
/// - v0 path: coreloop_base_gate + shapes_match + scan_direction check
pub(super) fn try_compose_scan_with_init_unified(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopRouteContext,
) -> Result<Option<LoweredRecipe>, String> {
    // v1 は常に None → value_join_needed は拒否
    if facts.value_join_needed {
        return Ok(None);
    }

    // v0 gate: coreloop_base_gate
    if !coreloop_base_gate(facts) {
        return Ok(None);
    }

    // v0 gate: facts.scan_with_init 必須
    let Some(scan) = facts.facts.scan_with_init.as_ref() else {
        return Ok(None);
    };

    // v0 gate: exit_kinds_allow_return_only
    if !exit_kinds_allow_return_only(facts) {
        return Ok(None);
    }

    // v0 gate: scan_direction ±1 のみ
    let Some(_scan_direction) = scan_direction_from_step_lit(scan.step_lit) else {
        return Ok(None);
    };

    // v0 gate: shapes_match (完全移植)
    let shapes_match = if matches!(facts.facts.condition_shape, ConditionShape::Unknown) {
        true
    } else {
        let cond_profile = cond_profile_from_scan_shapes(
            &facts.facts.condition_shape,
            &facts.facts.step_shape,
        );
        match_scan_with_init_shape(
            &facts.facts.condition_shape,
            &facts.facts.step_shape,
            &cond_profile,
        )
        .is_some_and(|shape| {
            let haystack_matches = match shape.haystack_var.as_ref() {
                Some(haystack) => haystack == &scan.haystack,
                None => true,
            };
            let needle_matches = match shape.needle_var.as_ref() {
                Some(needle) => needle == &scan.needle,
                None => true,
            };
            let dynamic_matches = shape.dynamic_needle == scan.dynamic_needle
                || (scan.step_lit == -1 && scan.dynamic_needle);
            shape.idx_var == scan.loop_var
                && shape.step_lit == scan.step_lit
                && dynamic_matches
                && haystack_matches
                && needle_matches
        })
    };
    if !shapes_match {
        return Ok(None);
    }

    // gate 通過 → PlanNormalizer
    let core = RecipeComposer::compose_scan_with_init_recipe(builder, facts, ctx)
        .map_err(|e| e.to_string())?;
    Ok(Some(core))
}

/// Unified SplitScan composer with all v0/v1 gate conditions preserved.
/// - v1 path: uses coreloop_value_join_gate (can succeed with value_join)
/// - v0 path: uses coreloop_base_gate (no value_join)
pub(super) fn try_compose_split_scan_unified(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopRouteContext,
) -> Result<Option<LoweredRecipe>, String> {
    // v0/v1 分岐
    if facts.value_join_needed {
        // v1 path: coreloop_value_join_gate
        if !coreloop_value_join_gate(facts) {
            return Ok(None);
        }
    } else {
        // v0 path: coreloop_base_gate
        if !coreloop_base_gate(facts) {
            return Ok(None);
        }
    }

    // 共通: facts.split_scan 必須
    let Some(split_scan) = facts.facts.split_scan.as_ref() else {
        return Ok(None);
    };

    // 共通: exit_kinds_allow_return_only
    if !exit_kinds_allow_return_only(facts) {
        return Ok(None);
    }

    // 共通: SplitScanShape::Minimal
    if !matches!(split_scan.shape, SplitScanShape::Minimal) {
        return Ok(None);
    }

    // gate 通過 → PlanNormalizer
    let _ = split_scan;
    let core = RecipeComposer::compose_split_scan_recipe(builder, facts, ctx)
        .map_err(|e| e.to_string())?;
    Ok(Some(core))
}

pub(in crate::mir::builder) fn try_compose_core_loop_from_facts(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopRouteContext,
) -> Result<Option<LoweredRecipe>, String> {
    if facts.nested_loop {
        return try_compose_core_loop_v2_nested_minimal(builder, facts, ctx);
    }

    if facts.value_join_needed {
        if let Some(core) =
            try_compose_core_loop_v1_loop_break(builder, facts, ctx)?
        {
            return Ok(Some(core));
        }
        if let Some(core) =
            try_compose_core_loop_v1_if_phi_join(builder, facts, ctx)?
        {
            return Ok(Some(core));
        }
        if let Some(core) =
            try_compose_core_loop_v1_loop_true_early_exit(builder, facts, ctx)?
        {
            return Ok(Some(core));
        }
        if let Some(core) = try_compose_split_scan_unified(builder, facts, ctx)? {
            return Ok(Some(core));
        }
        if let Some(core) = try_compose_scan_with_init_unified(builder, facts, ctx)? {
            return Ok(Some(core));
        }
        return Ok(None);
    }

    try_compose_core_loop_v0(builder, facts, ctx)
}

#[cfg(test)]
mod tests {
    use super::try_compose_core_loop_from_facts;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
    use crate::mir::builder::control_flow::plan::facts::feature_facts::{
        LoopFeatureFacts, ValueJoinFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::LoopFacts;
    use crate::mir::builder::control_flow::plan::facts::loop_simple_while_facts::LoopSimpleWhileFacts;
    use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
        ConditionShape, SplitScanShape, StepShape,
    };
    use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{
        SkeletonFacts, SkeletonKind,
    };
    use crate::mir::builder::control_flow::plan::normalize::canonicalize_loop_facts;
    use crate::mir::builder::MirBuilder;
    use crate::mir::MirType;

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

    #[test]
    fn single_entry_prefers_nested_path() {
        let condition = ASTNode::BinaryOp {
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
        let features = LoopFeatureFacts {
            nested_loop: true,
            ..LoopFeatureFacts::default()
        };
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
                ..Default::default()
            },
            features,
            scan_with_init: None,
            split_scan: None,
            loop_simple_while: Some(LoopSimpleWhileFacts {
                loop_var: "i".to_string(),
                condition: condition.clone(),
                loop_increment: loop_increment.clone(),
            }),
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
            loop_bundle_resolver_v0: None,
            loop_collect_using_entries_v0: None,
            bool_predicate_scan: None,
            accum_const_loop: None,
            loop_break: None,
            loop_break_body_local: None,
            nested_loop_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("single_entry_nested".to_string());
        let init = builder.alloc_typed(MirType::Integer);
        builder
            .variable_ctx
            .variable_map
            .insert("i".to_string(), init);
        let ctx =
            LoopRouteContext::new(&condition, &[], "single_entry_nested", false, false);
        let composed =
            try_compose_core_loop_from_facts(&mut builder, &canonical, &ctx)
                .expect("Ok");
        assert!(
            composed.is_none(),
            "nested_loop must not fall back to v0/v1"
        );
    }

    #[test]
    fn single_entry_uses_value_join_path() {
        let condition = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let features = LoopFeatureFacts {
            value_join: Some(ValueJoinFacts { needed: true }),
            ..LoopFeatureFacts::default()
        };
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
                ..Default::default()
            },
            features,
            scan_with_init: None,
            split_scan: Some(
                crate::mir::builder::control_flow::plan::facts::loop_types::SplitScanFacts {
                    s_var: "s".to_string(),
                    sep_var: "sep".to_string(),
                    result_var: "result".to_string(),
                    i_var: "i".to_string(),
                    start_var: "start".to_string(),
                    shape: SplitScanShape::Minimal,
                },
            ),
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
            loop_bundle_resolver_v0: None,
            loop_collect_using_entries_v0: None,
            bool_predicate_scan: None,
            accum_const_loop: None,
            loop_break: None,
            loop_break_body_local: None,
            nested_loop_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("single_entry_value_join".to_string());
        let s_val = builder.alloc_typed(MirType::String);
        let sep_val = builder.alloc_typed(MirType::String);
        let result_val = builder.alloc_typed(MirType::Array(Box::new(MirType::String)));
        let i_val = builder.alloc_typed(MirType::Integer);
        let start_val = builder.alloc_typed(MirType::Integer);
        builder
            .variable_ctx
            .variable_map
            .insert("s".to_string(), s_val);
        builder
            .variable_ctx
            .variable_map
            .insert("sep".to_string(), sep_val);
        builder
            .variable_ctx
            .variable_map
            .insert("result".to_string(), result_val);
        builder
            .variable_ctx
            .variable_map
            .insert("i".to_string(), i_val);
        builder
            .variable_ctx
            .variable_map
            .insert("start".to_string(), start_val);
        let ctx = LoopRouteContext::new(
            &condition,
            &[],
            "single_entry_value_join",
            false,
            false,
        );
        let composed =
            try_compose_core_loop_from_facts(&mut builder, &canonical, &ctx)
                .expect("Ok");
        assert!(composed.is_some(), "value-join path should compose");
    }

    #[test]
    fn single_entry_uses_no_join_path() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(2)),
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
                ..Default::default()
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: None,
            loop_simple_while: Some(LoopSimpleWhileFacts {
                loop_var: "i".to_string(),
                condition: condition.clone(),
                loop_increment: loop_increment.clone(),
            }),
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
            loop_bundle_resolver_v0: None,
            loop_collect_using_entries_v0: None,
            bool_predicate_scan: None,
            accum_const_loop: None,
            loop_break: None,
            loop_break_body_local: None,
            nested_loop_minimal: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("single_entry_no_join".to_string());
        let init = builder.alloc_typed(MirType::Integer);
        builder
            .variable_ctx
            .variable_map
            .insert("i".to_string(), init);
        let ctx =
            LoopRouteContext::new(&condition, &[], "single_entry_no_join", false, false);
        let composed =
            try_compose_core_loop_from_facts(&mut builder, &canonical, &ctx)
                .expect("Ok");
        assert!(composed.is_some(), "no-join path should compose");
    }
}
