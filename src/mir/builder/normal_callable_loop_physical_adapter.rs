//! Source-backed GenericLoopV1 physical adapter.
//!
//! This is the only named consumer for the first source-backed GenericLoopV1
//! Recipe.  Source/Facts issuance and route selection have already completed;
//! this adapter only composes the selected Facts into the existing CorePlan
//! vocabulary and lowers it through the route-neutral physical context.

use std::cell::RefCell;
use std::rc::Rc;

use crate::mir::builder::control_flow::plan::features::generic_loop_context::GenericLoopV1SourceLoweringContextV1;
use crate::mir::builder::control_flow::plan::lowerer::PlanLowerer;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::verify::PlanVerifier;
use crate::mir::builder::normal_callable_loop_source_facts::{
    CallableGenericLoopV1SemanticRecipeV1, CallableGenericLoopV1SemanticRecipeViewRejectV1,
};
use crate::mir::builder::normal_callable_loop_source_port::CallableLoopSourceExpressionPortV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::{MirBuilder, UnpublishedCallableLoopRootScopeV1, ValueId};

pub(in crate::mir::builder) struct CallableGenericLoopV1PhysicalAdapterV1;

impl CallableGenericLoopV1PhysicalAdapterV1 {
    pub(in crate::mir::builder) fn lower(
        builder: &mut MirBuilder,
        _root_scope: &mut UnpublishedCallableLoopRootScopeV1,
        recipe: CallableGenericLoopV1SemanticRecipeV1<'_>,
        callable_ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
    ) -> Result<ValueId, String> {
        let lowered = recipe
            .with_source_relation_view_once(|view| -> Result<ValueId, String> {
                if callable_ledger.borrow().owner() != view.owner() {
                    return Err(
                        "[freeze:contract][callable-loop/relation-ledger-owner-mismatch]"
                            .to_owned(),
                    );
                }
                let port = CallableLoopSourceExpressionPortV1::new(callable_ledger);
                let condition = port.expr(&view.generic().condition, view.condition_source())?;
                let body = port.body(&view.generic().body.body, view.body_source())?;
                let context =
                    GenericLoopV1SourceLoweringContextV1::new(view.debug(), view.in_static_box());
                let plan = RecipeComposer::compose_source_generic_loop_v1_recipe_with_port(
                    builder,
                    view.facts(),
                    &port,
                    condition,
                    body,
                )
                .map_err(|error| format!("[freeze:contract][callable-loop/recipe] {error}"))?;
                PlanVerifier::verify(&plan)
                    .map_err(|error| format!("[freeze:contract][callable-loop/verify] {error}"))?;
                PlanLowerer::lower(builder, plan, &context)
                    .map_err(|error| format!("[freeze:contract][callable-loop/lower] {error}"))?
                    .ok_or_else(|| "[freeze:contract][callable-loop/lower-no-value]".to_owned())
            })
            .map_err(|error: CallableGenericLoopV1SemanticRecipeViewRejectV1| {
                format!("[freeze:contract][callable-loop/semantic-view] {error:?}")
            })?;
        lowered
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn lower_for_test(
        builder: &mut MirBuilder,
        recipe: CallableGenericLoopV1SemanticRecipeV1<'_>,
    ) -> Result<ValueId, String> {
        let mut root_scope = UnpublishedCallableLoopRootScopeV1::for_test();
        Self::lower_legacy_for_test(builder, &mut root_scope, recipe)
    }

    #[cfg(test)]
    fn lower_legacy_for_test(
        builder: &mut MirBuilder,
        _root_scope: &mut UnpublishedCallableLoopRootScopeV1,
        recipe: CallableGenericLoopV1SemanticRecipeV1<'_>,
    ) -> Result<ValueId, String> {
        recipe
            .with_view(|view| {
                let context =
                    GenericLoopV1SourceLoweringContextV1::new(view.debug(), view.in_static_box());
                let plan = RecipeComposer::compose_source_generic_loop_v1_recipe(
                    builder,
                    view.facts(),
                    &context,
                )
                .map_err(|error| format!("[freeze:contract][callable-loop/recipe] {error}"))?;
                PlanVerifier::verify(&plan)
                    .map_err(|error| format!("[freeze:contract][callable-loop/verify] {error}"))?;
                PlanLowerer::lower(builder, plan, &context)
                    .map_err(|error| format!("[freeze:contract][callable-loop/lower] {error}"))?
                    .ok_or_else(|| "[freeze:contract][callable-loop/lower-no-value]".to_owned())
            })
            .map_err(|error: CallableGenericLoopV1SemanticRecipeViewRejectV1| {
                format!("[freeze:contract][callable-loop/semantic-view] {error:?}")
            })?
    }
}
