//! Test-only observation of the Generic nested-loop depth-1 handoff.
//!
//! The observer follows the same order as `helpers::lower_nested_loop_plan`:
//! the specialized depth-1 fastpath is attempted first, and the Generic
//! recipe-adoption fallback is attempted only when the fastpath returns an
//! error.  It records Builder deltas but never publishes a plan or changes a
//! production route.

use super::nested_loop_recipe_adoption::try_compose_generic_nested_loop_recipe_adoption;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1::lower_nested_loop_depth1_any;
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum NestedStageResultV1 {
    Succeeded,
    ReturnedNone,
    ReturnedErr,
    NotObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum NestedFirstEffectOwnerV1 {
    None,
    Depth1Fastpath,
    GenericFallback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct NestedBuilderSnapshotV1 {
    pub(in crate::mir::builder) current_block: Option<BasicBlockId>,
    pub(in crate::mir::builder) block_count: usize,
    pub(in crate::mir::builder) next_value_id: Option<u32>,
    pub(in crate::mir::builder) variable_count: usize,
    pub(in crate::mir::builder) typed_value_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct NestedDepthObservationV1 {
    pub(in crate::mir::builder) fastpath: NestedStageResultV1,
    pub(in crate::mir::builder) fallback: NestedStageResultV1,
    pub(in crate::mir::builder) first_effect_owner: NestedFirstEffectOwnerV1,
    pub(in crate::mir::builder) before_fastpath: NestedBuilderSnapshotV1,
    pub(in crate::mir::builder) after_fastpath: NestedBuilderSnapshotV1,
    pub(in crate::mir::builder) before_fallback: Option<NestedBuilderSnapshotV1>,
    pub(in crate::mir::builder) after_fallback: Option<NestedBuilderSnapshotV1>,
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn less(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn both_source() -> (ASTNode, Vec<ASTNode>) {
    let inner = ASTNode::Loop {
        condition: Box::new(less(variable("j"), integer(3))),
        body: vec![assignment("j", add(variable("j"), integer(1)))],
        span: Span::unknown(),
    };
    (
        less(variable("i"), integer(3)),
        vec![inner, assignment("i", add(variable("i"), integer(1)))],
    )
}

fn inner_source() -> (ASTNode, Vec<ASTNode>) {
    let (outer_condition, outer_body) = both_source();
    let ASTNode::Loop {
        condition, body, ..
    } = &outer_body[0]
    else {
        panic!("Both fixture must begin with a nested Loop");
    };
    let _ = outer_condition;
    ((**condition).clone(), body.clone())
}

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("generic_nested_depth_observer/0".to_string());
    for name in ["i", "j"] {
        let value = builder.alloc_typed(MirType::Integer);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(name.to_string(), value);
    }
    builder
}

fn snapshot(builder: &MirBuilder) -> NestedBuilderSnapshotV1 {
    NestedBuilderSnapshotV1 {
        current_block: builder.function_state.current_block,
        block_count: builder
            .function_state
            .current_function
            .as_ref()
            .map(|function| function.blocks.len())
            .unwrap_or_default(),
        next_value_id: builder
            .function_state
            .current_function
            .as_ref()
            .map(|function| function.next_value_id),
        variable_count: builder.function_state.variable_ctx.variable_map.len(),
        typed_value_count: builder.function_state.type_ctx.value_types.len(),
    }
}

fn stage_result(result: Result<Option<LoweredRecipe>, String>) -> NestedStageResultV1 {
    match result {
        Ok(Some(_)) => NestedStageResultV1::Succeeded,
        Ok(None) => NestedStageResultV1::ReturnedNone,
        Err(_) => NestedStageResultV1::ReturnedErr,
    }
}

fn fallback_observation(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    strict_or_dev: bool,
    planner_required: bool,
) -> (
    NestedStageResultV1,
    NestedBuilderSnapshotV1,
    NestedBuilderSnapshotV1,
) {
    let nested_ctx = LoopRouteContext::new(
        condition,
        body,
        "generic_nested_depth_observer/0",
        false,
        false,
    );
    let before = snapshot(builder);
    let result = match try_build_outcome(&nested_ctx) {
        Ok(outcome) => try_compose_generic_nested_loop_recipe_adoption(
            builder,
            &outcome,
            &nested_ctx,
            strict_or_dev,
            planner_required,
        ),
        Err(error) => Err(error),
    };
    let after = snapshot(builder);
    (stage_result(result), before, after)
}

pub(in crate::mir::builder) fn observe_nested_depth1(
    strict_or_dev: bool,
    planner_required: bool,
) -> NestedDepthObservationV1 {
    assert!(!planner_required || strict_or_dev);
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let _config = crate::test_support::ScopedTestConfig::apply(&[
        ("HAKO_JOINIR_STRICT", strict_or_dev.then_some("1")),
        (
            "HAKO_JOINIR_PLANNER_REQUIRED",
            planner_required.then_some("1"),
        ),
        ("NYASH_JOINIR_STRICT", None),
    ]);
    let (condition, body) = inner_source();

    let mut fast_builder = seeded_builder();
    let _fast_scope = LexicalScopeGuard::new(&mut fast_builder);
    let before_fastpath = snapshot(&fast_builder);
    let fast_result = lower_nested_loop_depth1_any(
        &mut fast_builder,
        &condition,
        &body,
        "[test] generic nested loop",
    );
    let after_fastpath = snapshot(&fast_builder);
    let fastpath = match fast_result {
        Ok(_) => NestedStageResultV1::Succeeded,
        Err(_) => NestedStageResultV1::ReturnedErr,
    };
    let fastpath_effectful = before_fastpath != after_fastpath;

    if fastpath == NestedStageResultV1::Succeeded {
        return NestedDepthObservationV1 {
            fastpath,
            fallback: NestedStageResultV1::NotObserved,
            first_effect_owner: if fastpath_effectful {
                NestedFirstEffectOwnerV1::Depth1Fastpath
            } else {
                NestedFirstEffectOwnerV1::None
            },
            before_fastpath,
            after_fastpath,
            before_fallback: None,
            after_fallback: None,
        };
    }

    let mut fallback_builder = seeded_builder();
    let _fallback_scope = LexicalScopeGuard::new(&mut fallback_builder);
    let (fallback, before_fallback, after_fallback) = fallback_observation(
        &mut fallback_builder,
        &condition,
        &body,
        strict_or_dev,
        planner_required,
    );
    let fallback_effectful = before_fallback != after_fallback;
    NestedDepthObservationV1 {
        fastpath,
        fallback,
        first_effect_owner: if fastpath_effectful {
            NestedFirstEffectOwnerV1::Depth1Fastpath
        } else if fallback_effectful {
            NestedFirstEffectOwnerV1::GenericFallback
        } else {
            NestedFirstEffectOwnerV1::None
        },
        before_fastpath,
        after_fastpath,
        before_fallback: Some(before_fallback),
        after_fallback: Some(after_fallback),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_both_fixture_observes_depth1_owner_deterministically() {
        let release = observe_nested_depth1(false, false);
        let repeat = observe_nested_depth1(false, false);
        assert_eq!(release, repeat);
        assert_eq!(release.fastpath, NestedStageResultV1::Succeeded);
        assert_eq!(release.fallback, NestedStageResultV1::NotObserved);
        assert_eq!(
            release.first_effect_owner,
            NestedFirstEffectOwnerV1::Depth1Fastpath
        );
    }

    #[test]
    fn nested_both_fixture_observes_strict_and_planner_modes() {
        let strict = observe_nested_depth1(true, false);
        let planner = observe_nested_depth1(true, true);
        assert_eq!(strict.fastpath, NestedStageResultV1::Succeeded);
        assert_eq!(planner.fastpath, NestedStageResultV1::Succeeded);
        assert_eq!(strict.fallback, NestedStageResultV1::NotObserved);
        assert_eq!(planner.fallback, NestedStageResultV1::NotObserved);
        assert_eq!(
            planner.first_effect_owner,
            NestedFirstEffectOwnerV1::Depth1Fastpath
        );
    }
}
