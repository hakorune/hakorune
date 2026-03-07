//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use super::{
    build_loop_break_recipe, LoopBreakRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
use crate::mir::builder::control_flow::plan::recipe_tree::{BlockContractKind, RecipeItem};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::{LoweredRecipe, LoopBreakStepPlacement};
use crate::mir::builder::MirBuilder;

fn then_body_has_top_level_break(then_body: &[ASTNode]) -> bool {
    then_body.iter().any(|stmt| matches!(stmt, ASTNode::Break { .. }))
}

fn body_has_step_before_break(body: &[ASTNode], loop_var: &str) -> bool {
    if body.len() < 2 {
        return false;
    }
    let step_is_first = matches!(
        &body[0],
        ASTNode::Assignment { target, .. }
            if matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == loop_var)
    );
    let break_if_is_second = matches!(
        &body[1],
        ASTNode::If {
            then_body,
            else_body: None,
            ..
        } if then_body_has_top_level_break(then_body)
    );
    step_is_first && break_if_is_second
}

impl RecipeComposer {

    /// Compose loop-break facts into LoweredRecipe via RecipeBlock (no normalizer).
    ///
    /// Used only in strict/dev + planner_required routing.
    pub fn compose_loop_break_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "loop_break_recipe";

        let mut loop_break_facts = facts
            .facts
            .loop_break()
            .cloned()
            .ok_or_else(|| Freeze::contract("LoopBreak facts missing in compose_loop_break_recipe"))?;

        // Planner-required strict mode: recover step-before-break placement from the body shape.
        if body_has_step_before_break(ctx.body, &loop_break_facts.loop_var) {
            loop_break_facts.step_placement = LoopBreakStepPlacement::BeforeBreak;
        }

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_break path=recipe_block");
        }

        // Structure-only loop stmt used to build the recipe tree.
        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(loop_break_facts.loop_condition.clone()),
            body: vec![],
            span: dummy_span,
        };

        let loop_cond_view = CondBlockView::from_expr(&loop_break_facts.loop_condition);
        let break_cond_view = CondBlockView::from_expr(&loop_break_facts.break_condition);

        let Some(LoopBreakRecipe { arena, root }) = build_loop_break_recipe(
            &loop_stmt,
            loop_cond_view,
            break_cond_view,
            &loop_break_facts,
        ) else {
            return Err(Freeze::contract(
                "LoopBreak recipe missing (planner_required)",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::NoExit, CTX).map_err(|e| {
            Freeze::contract("LoopBreak recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract("LoopBreak recipe root missing LoopV0"));
        };

        let RecipeItem::LoopV0 {
            cond_view,
            body_block,
            body_contract,
            ..
        } = loop_item
        else {
            return Err(Freeze::contract(
                "LoopBreak recipe root is not LoopV0",
            ));
        };

        let mut current_bindings = builder.variable_ctx.variable_map.clone();
        parts::entry::lower_loop_v0(
            builder,
            &mut current_bindings,
            cond_view,
            *body_contract,
            &arena,
            body_block,
            CTX,
        )
        .map_err(|e| Freeze::contract(&format!("LoopBreak recipe lower failed: {e}")))
    }

}
