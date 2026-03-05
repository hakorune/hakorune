//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use super::{
    build_array_join_recipe, build_char_map_recipe,
    build_loop_simple_while_recipe, ArrayJoinRecipe, CharMapRecipe,
    LoopSimpleWhileRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
use crate::mir::builder::control_flow::plan::recipe_tree::{BlockContractKind, RecipeItem};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

impl RecipeComposer {

    /// Compose Pattern1SimpleWhile facts into LoweredRecipe via RecipeBlock (no normalizer).
    pub fn compose_loop_simple_while_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "pattern1_simple_while_recipe";

        let pattern1_facts = facts.facts.pattern1_simplewhile.clone().ok_or_else(|| {
            Freeze::contract(
                "Pattern1SimpleWhile facts missing in compose_loop_simple_while_recipe",
            )
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_simple_while path=recipe_block");
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(pattern1_facts.condition.clone()),
            body: vec![],
            span: dummy_span,
        };

        // Pattern1SimpleWhile carries only the increment expression; rebuild the stmt.
        let loop_inc_stmt = ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: pattern1_facts.loop_var.clone(),
                span: dummy_span,
            }),
            value: Box::new(pattern1_facts.loop_increment.clone()),
            span: dummy_span,
        };
        let body = vec![loop_inc_stmt];

        let loop_cond_view = CondBlockView::from_expr(&pattern1_facts.condition);

        let Some(LoopSimpleWhileRecipe { arena, root }) =
            build_loop_simple_while_recipe(&loop_stmt, loop_cond_view, &body)
        else {
            return Err(Freeze::contract(
                "Pattern1SimpleWhile recipe missing (planner_required)",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::NoExit, CTX).map_err(|e| {
            Freeze::contract("Pattern1SimpleWhile recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract("Pattern1SimpleWhile recipe root missing LoopV0"));
        };

        let RecipeItem::LoopV0 {
            cond_view,
            body_block,
            body_contract,
            ..
        } = loop_item
        else {
            return Err(Freeze::contract(
                "Pattern1SimpleWhile recipe root is not LoopV0",
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
        .map_err(|e| Freeze::contract(&format!("Pattern1SimpleWhile recipe lower failed: {e}")))
    }

    /// Compose Pattern1CharMap facts into LoweredRecipe via RecipeBlock (no normalizer).
    pub fn compose_loop_char_map_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "pattern1_char_map_recipe";

        let pattern1_facts = facts.facts.pattern1_char_map.clone().ok_or_else(|| {
            Freeze::contract("Pattern1CharMap facts missing in compose_loop_char_map_recipe")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_char_map path=recipe_block");
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(pattern1_facts.condition.clone()),
            body: vec![],
            span: dummy_span,
        };

        let loop_cond_view = CondBlockView::from_expr(&pattern1_facts.condition);

        let Some(CharMapRecipe { arena, root }) =
            build_char_map_recipe(&loop_stmt, loop_cond_view, &pattern1_facts)
        else {
            return Err(Freeze::contract(
                "Pattern1CharMap recipe missing (planner_required)",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::NoExit, CTX).map_err(|e| {
            Freeze::contract("Pattern1CharMap recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract("Pattern1CharMap recipe root missing LoopV0"));
        };

        let RecipeItem::LoopV0 {
            cond_view,
            body_block,
            body_contract,
            ..
        } = loop_item
        else {
            return Err(Freeze::contract(
                "Pattern1CharMap recipe root is not LoopV0",
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
        .map_err(|e| Freeze::contract(&format!("Pattern1CharMap recipe lower failed: {e}")))
    }

    /// Compose Pattern1ArrayJoin facts into LoweredRecipe via RecipeBlock (no normalizer).
    pub fn compose_loop_array_join_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "pattern1_array_join_recipe";

        let pattern1_facts = facts.facts.pattern1_array_join.clone().ok_or_else(|| {
            Freeze::contract(
                "Pattern1ArrayJoin facts missing in compose_loop_array_join_recipe",
            )
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_array_join path=recipe_block");
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(pattern1_facts.condition.clone()),
            body: vec![],
            span: dummy_span,
        };

        let loop_cond_view = CondBlockView::from_expr(&pattern1_facts.condition);
        let if_cond_view = CondBlockView::from_expr(&pattern1_facts.if_condition);

        let Some(ArrayJoinRecipe { arena, root }) = build_array_join_recipe(
            &loop_stmt,
            loop_cond_view,
            if_cond_view,
            &pattern1_facts,
        ) else {
            return Err(Freeze::contract(
                "Pattern1ArrayJoin recipe missing (planner_required)",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::NoExit, CTX).map_err(|e| {
            Freeze::contract("Pattern1ArrayJoin recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract("Pattern1ArrayJoin recipe root missing LoopV0"));
        };

        let RecipeItem::LoopV0 {
            cond_view,
            body_block,
            body_contract,
            ..
        } = loop_item
        else {
            return Err(Freeze::contract(
                "Pattern1ArrayJoin recipe root is not LoopV0",
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
        .map_err(|e| Freeze::contract(&format!("Pattern1ArrayJoin recipe lower failed: {e}")))
    }

}
