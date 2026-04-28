//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::check_block_contract;
use crate::mir::builder::control_flow::plan::recipe_tree::loop_true_early_exit_builder::{
    build_loop_true_early_exit_recipe, LoopTrueEarlyExitRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{BlockContractKind, RecipeItem};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

impl RecipeComposer {
    /// Compose loop(true) early-exit facts into LoweredRecipe via RecipeBlock (no normalizer).
    pub fn compose_loop_true_early_exit_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::ast::LiteralValue;
        use crate::config::env::joinir_dev;

        const CTX: &str = "loop_true_early_exit_recipe";

        let early_exit_facts = facts.facts.loop_true_early_exit().clone().ok_or_else(|| {
            Freeze::contract(
                "LoopTrueEarlyExit facts missing in compose_loop_true_early_exit_recipe",
            )
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_true_early_exit path=recipe_block");
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let true_cond = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: dummy_span,
        };
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(true_cond.clone()),
            body: vec![],
            span: dummy_span,
        };
        let exit_cond_view = CondBlockView::from_expr(&early_exit_facts.exit_condition);

        let Some(LoopTrueEarlyExitRecipe { arena, root }) =
            build_loop_true_early_exit_recipe(&loop_stmt, exit_cond_view, &early_exit_facts)
        else {
            return Err(Freeze::contract(
                "LoopTrueEarlyExit recipe build returned None",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::ExitAllowed, CTX).map_err(|e| {
            Freeze::contract("LoopTrueEarlyExit recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract(
                "LoopTrueEarlyExit recipe root missing LoopV0",
            ));
        };

        let RecipeItem::LoopV0 {
            cond_view,
            body_block,
            body_contract,
            ..
        } = loop_item
        else {
            return Err(Freeze::contract(
                "LoopTrueEarlyExit recipe root is not LoopV0",
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
        .map_err(|e| Freeze::contract(&format!("LoopTrueEarlyExit recipe lower failed: {e}")))
    }
}
