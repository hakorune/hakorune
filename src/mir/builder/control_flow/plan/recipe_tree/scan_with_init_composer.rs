//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::check_block_contract;
use crate::mir::builder::control_flow::plan::recipe_tree::scan_with_init_builder::{
    build_scan_with_init_recipe, ScanWithInitRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{BlockContractKind, RecipeItem};
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

fn build_scan_with_init_loop_condition(
    facts: &crate::mir::builder::control_flow::plan::facts::loop_types::ScanWithInitFacts,
) -> ASTNode {
    let span = Span::unknown();
    let loop_var = ASTNode::Variable {
        name: facts.loop_var.clone(),
        span,
    };

    if facts.step_lit < 0 {
        return ASTNode::BinaryOp {
            operator: BinaryOperator::GreaterEqual,
            left: Box::new(loop_var),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span,
            }),
            span,
        };
    }

    if facts.dynamic_needle {
        let left = ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: facts.haystack.clone(),
                span,
            }),
            method: "length".to_string(),
            arguments: vec![],
            span,
        };
        let right = ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: facts.needle.clone(),
                span,
            }),
            method: "length".to_string(),
            arguments: vec![],
            span,
        };
        let bound = ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            left: Box::new(left),
            right: Box::new(right),
            span,
        };
        return ASTNode::BinaryOp {
            operator: BinaryOperator::LessEqual,
            left: Box::new(loop_var),
            right: Box::new(bound),
            span,
        };
    }

    let right = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: facts.haystack.clone(),
            span,
        }),
        method: "length".to_string(),
        arguments: vec![],
        span,
    };
    ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(loop_var),
        right: Box::new(right),
        span,
    }
}

impl RecipeComposer {
    /// Compose scan-with-init facts into LoweredRecipe via RecipeBlock (no normalizer).
    pub fn compose_scan_with_init_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "scan_with_init_recipe";

        let scan_facts = facts.facts.scan_with_init.clone().ok_or_else(|| {
            Freeze::contract("ScanWithInit facts missing in compose_scan_with_init_recipe")
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug("[recipe:compose] route=scan_with_init path=recipe_block");
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_condition = build_scan_with_init_loop_condition(&scan_facts);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(loop_condition.clone()),
            body: vec![],
            span: dummy_span,
        };
        let loop_cond_view = CondBlockView::from_expr(&loop_condition);

        let Some(ScanWithInitRecipe { arena, root }) =
            build_scan_with_init_recipe(&loop_stmt, loop_cond_view, &scan_facts)
        else {
            return Err(Freeze::contract("ScanWithInit recipe build returned None"));
        };

        check_block_contract(&arena, &root, BlockContractKind::ExitAllowed, CTX).map_err(|e| {
            Freeze::contract("ScanWithInit recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract("ScanWithInit recipe root missing LoopV0"));
        };

        let RecipeItem::LoopV0 {
            cond_view,
            body_block,
            body_contract,
            ..
        } = loop_item
        else {
            return Err(Freeze::contract("ScanWithInit recipe root is not LoopV0"));
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
        .map_err(|e| Freeze::contract(&format!("ScanWithInit recipe lower failed: {e}")))
    }
}
