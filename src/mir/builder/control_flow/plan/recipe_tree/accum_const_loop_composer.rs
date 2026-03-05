//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use super::{
    build_accum_const_loop_recipe, AccumConstLoopRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
use crate::mir::builder::control_flow::plan::recipe_tree::{BlockContractKind, RecipeItem};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

impl RecipeComposer {

    /// Compose Pattern9 AccumConstLoop facts into LoweredRecipe via RecipeBlock (no normalizer).
    pub fn compose_pattern9_accum_const_loop_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "pattern9_accum_const_loop_recipe";

        let pattern9_facts = facts
            .facts
            .pattern9_accum_const_loop
            .clone()
            .ok_or_else(|| {
                Freeze::contract(
                    "Pattern9AccumConstLoop facts missing in compose_pattern9_accum_const_loop_recipe",
                )
            })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("[recipe:compose] pattern9_accum_const_loop: composing via RecipeBlock path"));
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(pattern9_facts.condition.clone()),
            body: vec![],
            span: dummy_span,
        };
        let loop_cond_view = CondBlockView::from_expr(&pattern9_facts.condition);
        crate::mir::builder::control_flow::plan::verifier::debug_observe_cond_profile_value(
            &pattern9_facts.cond_profile,
        );

        let Some(AccumConstLoopRecipe { arena, root }) =
            build_accum_const_loop_recipe(&loop_stmt, loop_cond_view, &pattern9_facts)
        else {
            return Err(Freeze::contract(
                "Pattern9AccumConstLoop recipe build returned None",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::NoExit, CTX).map_err(|e| {
            Freeze::contract("Pattern9AccumConstLoop recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract(
                "Pattern9AccumConstLoop recipe root missing LoopV0",
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
                "Pattern9AccumConstLoop recipe root is not LoopV0",
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
        .map_err(|e| {
            Freeze::contract(&format!(
                "Pattern9AccumConstLoop recipe lower failed: {e}"
            ))
        })
    }

}
