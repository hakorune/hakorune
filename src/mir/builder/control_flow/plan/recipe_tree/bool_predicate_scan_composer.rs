//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::bool_predicate_scan_builder::{
    build_bool_predicate_scan_recipe, BoolPredicateScanRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
use crate::mir::builder::control_flow::plan::recipe_tree::{BlockContractKind, RecipeItem};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

impl RecipeComposer {
    /// Compose bool-predicate-scan facts into LoweredRecipe via RecipeBlock (no normalizer).
    pub fn compose_bool_predicate_scan_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "bool_predicate_scan_recipe";

        let bool_scan_facts = facts.facts.bool_predicate_scan().clone().ok_or_else(|| {
            Freeze::contract(
                "BoolPredicateScan facts missing in compose_bool_predicate_scan_recipe",
            )
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=bool_predicate_scan path=recipe_block");
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(bool_scan_facts.condition.clone()),
            body: vec![],
            span: dummy_span,
        };
        let loop_cond_view = CondBlockView::from_expr(&bool_scan_facts.condition);
        crate::mir::builder::control_flow::verify::verifier::debug_observe_cond_profile_value(
            &bool_scan_facts.cond_profile,
        );

        let Some(BoolPredicateScanRecipe { arena, root }) =
            build_bool_predicate_scan_recipe(&loop_stmt, loop_cond_view, &bool_scan_facts)
        else {
            return Err(Freeze::contract(
                "BoolPredicateScan recipe build returned None",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::ExitAllowed, CTX).map_err(|e| {
            Freeze::contract("BoolPredicateScan recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract(
                "BoolPredicateScan recipe root missing LoopV0",
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
                "BoolPredicateScan recipe root is not LoopV0",
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
        .map_err(|e| Freeze::contract(&format!("BoolPredicateScan recipe lower failed: {e}")))
    }
}
