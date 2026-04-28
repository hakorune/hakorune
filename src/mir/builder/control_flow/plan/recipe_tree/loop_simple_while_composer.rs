//! Split from composer.rs (behavior-preserving module split).

use super::verified::check_block_contract;
use super::RecipeComposer;
use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::array_join_builder::{
    build_array_join_recipe, ArrayJoinRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::char_map_builder::{
    build_char_map_recipe, CharMapRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::loop_simple_while_builder::{
    build_loop_simple_while_recipe, LoopSimpleWhileRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{BlockContractKind, RecipeItem};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

impl RecipeComposer {
    /// Compose loop-simple-while facts into LoweredRecipe via RecipeBlock (no normalizer).
    pub fn compose_loop_simple_while_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "loop_simple_while_recipe";

        let simple_while_facts = facts.facts.loop_simple_while().cloned().ok_or_else(|| {
            Freeze::contract("LoopSimpleWhile facts missing in compose_loop_simple_while_recipe")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_simple_while path=recipe_block");
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(simple_while_facts.condition.clone()),
            body: vec![],
            span: dummy_span,
        };

        // LoopSimpleWhile facts carry only the increment expression; rebuild the stmt.
        let loop_inc_stmt = ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: simple_while_facts.loop_var.clone(),
                span: dummy_span,
            }),
            value: Box::new(simple_while_facts.loop_increment.clone()),
            span: dummy_span,
        };
        let body = vec![loop_inc_stmt];

        let loop_cond_view = CondBlockView::from_expr(&simple_while_facts.condition);

        let Some(LoopSimpleWhileRecipe { arena, root }) =
            build_loop_simple_while_recipe(&loop_stmt, loop_cond_view, &body)
        else {
            return Err(Freeze::contract(
                "LoopSimpleWhile recipe missing (planner_required)",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::NoExit, CTX).map_err(|e| {
            Freeze::contract("LoopSimpleWhile recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract(
                "LoopSimpleWhile recipe root missing LoopV0",
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
                "LoopSimpleWhile recipe root is not LoopV0",
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
        .map_err(|e| Freeze::contract(&format!("LoopSimpleWhile recipe lower failed: {e}")))
    }

    /// Compose loop-char-map facts into LoweredRecipe via RecipeBlock (no normalizer).
    pub fn compose_loop_char_map_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "loop_char_map_recipe";

        let char_map_facts = facts.facts.loop_char_map().cloned().ok_or_else(|| {
            Freeze::contract("LoopCharMap facts missing in compose_loop_char_map_recipe")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_char_map path=recipe_block");
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(char_map_facts.condition.clone()),
            body: vec![],
            span: dummy_span,
        };

        let loop_cond_view = CondBlockView::from_expr(&char_map_facts.condition);

        let Some(CharMapRecipe { arena, root }) =
            build_char_map_recipe(&loop_stmt, loop_cond_view, &char_map_facts)
        else {
            return Err(Freeze::contract(
                "LoopCharMap recipe missing (planner_required)",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::NoExit, CTX).map_err(|e| {
            Freeze::contract("LoopCharMap recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract("LoopCharMap recipe root missing LoopV0"));
        };

        let RecipeItem::LoopV0 {
            cond_view,
            body_block,
            body_contract,
            ..
        } = loop_item
        else {
            return Err(Freeze::contract("LoopCharMap recipe root is not LoopV0"));
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
        .map_err(|e| Freeze::contract(&format!("LoopCharMap recipe lower failed: {e}")))
    }

    /// Compose loop-array-join facts into LoweredRecipe via RecipeBlock (no normalizer).
    pub fn compose_loop_array_join_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "loop_array_join_recipe";

        let array_join_facts = facts.facts.loop_array_join().cloned().ok_or_else(|| {
            Freeze::contract("LoopArrayJoin facts missing in compose_loop_array_join_recipe")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=loop_array_join path=recipe_block");
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(array_join_facts.condition.clone()),
            body: vec![],
            span: dummy_span,
        };

        let loop_cond_view = CondBlockView::from_expr(&array_join_facts.condition);
        let if_cond_view = CondBlockView::from_expr(&array_join_facts.if_condition);

        let Some(ArrayJoinRecipe { arena, root }) =
            build_array_join_recipe(&loop_stmt, loop_cond_view, if_cond_view, &array_join_facts)
        else {
            return Err(Freeze::contract(
                "LoopArrayJoin recipe missing (planner_required)",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::NoExit, CTX).map_err(|e| {
            Freeze::contract("LoopArrayJoin recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract("LoopArrayJoin recipe root missing LoopV0"));
        };

        let RecipeItem::LoopV0 {
            cond_view,
            body_block,
            body_contract,
            ..
        } = loop_item
        else {
            return Err(Freeze::contract("LoopArrayJoin recipe root is not LoopV0"));
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
        .map_err(|e| Freeze::contract(&format!("LoopArrayJoin recipe lower failed: {e}")))
    }
}
