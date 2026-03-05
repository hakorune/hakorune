//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use super::{
    build_loop_continue_only_recipe, LoopContinueOnlyRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
use crate::mir::builder::control_flow::plan::recipe_tree::{BlockContractKind, RecipeItem};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

impl RecipeComposer {

    /// Compose Pattern4Continue facts into LoweredRecipe via RecipeBlock (no normalizer).
    ///
    /// Used only in strict/dev + planner_required routing.
    pub fn compose_pattern4_continue_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "pattern4_continue_recipe";

        let pattern4_facts = facts.facts.pattern4_continue.clone().ok_or_else(|| {
            Freeze::contract("Pattern4Continue facts missing in compose_pattern4_continue_recipe")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_continue_only path=recipe_block");
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(pattern4_facts.condition.clone()),
            body: vec![],
            span: dummy_span,
        };

        let loop_cond_view = CondBlockView::from_expr(&pattern4_facts.condition);
        let continue_cond_view = CondBlockView::from_expr(&pattern4_facts.continue_condition);

        let Some(LoopContinueOnlyRecipe { arena, root }) = build_loop_continue_only_recipe(
            &loop_stmt,
            loop_cond_view,
            continue_cond_view,
            &pattern4_facts,
        ) else {
            return Err(Freeze::contract(
                "Pattern4Continue recipe missing (planner_required)",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::ExitAllowed, CTX).map_err(|e| {
            Freeze::contract("Pattern4Continue recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract("Pattern4Continue recipe root missing LoopV0"));
        };

        let RecipeItem::LoopV0 {
            cond_view,
            body_block,
            body_contract,
            ..
        } = loop_item
        else {
            return Err(Freeze::contract(
                "Pattern4Continue recipe root is not LoopV0",
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
        .map_err(|e| Freeze::contract(&format!("Pattern4Continue recipe lower failed: {e}")))
    }

}
