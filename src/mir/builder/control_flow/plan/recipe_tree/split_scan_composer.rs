//! Split from composer.rs (behavior-preserving module split).

use super::RecipeComposer;
use crate::ast::{ASTNode, BinaryOperator, Span};
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use super::{
    build_split_scan_recipe, SplitScanRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
use crate::mir::builder::control_flow::plan::recipe_tree::{BlockContractKind, RecipeItem};
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

fn build_split_scan_loop_condition(
    facts: &crate::mir::builder::control_flow::plan::facts::loop_types::SplitScanFacts,
) -> ASTNode {
    let span = Span::unknown();
    let left = ASTNode::Variable {
        name: facts.i_var.clone(),
        span,
    };
    let s_len = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: facts.s_var.clone(),
            span,
        }),
        method: "length".to_string(),
        arguments: vec![],
        span,
    };
    let sep_len = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: facts.sep_var.clone(),
            span,
        }),
        method: "length".to_string(),
        arguments: vec![],
        span,
    };
    let bound = ASTNode::BinaryOp {
        operator: BinaryOperator::Subtract,
        left: Box::new(s_len),
        right: Box::new(sep_len),
        span,
    };

    ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left: Box::new(left),
        right: Box::new(bound),
        span,
    }
}

impl RecipeComposer {

    /// Compose Pattern7 SplitScan facts into LoweredRecipe via RecipeBlock (no normalizer).
    pub fn compose_pattern7_split_scan_recipe(
        builder: &mut MirBuilder,
        facts: &CanonicalLoopFacts,
        _ctx: &LoopRouteContext,
    ) -> Result<LoweredRecipe, Freeze> {
        use crate::config::env::joinir_dev;

        const CTX: &str = "pattern7_split_scan_recipe";

        let split_facts = facts.facts.split_scan.clone().ok_or_else(|| {
            Freeze::contract(
                "Pattern7SplitScan facts missing in compose_pattern7_split_scan_recipe",
            )
        })?;

        if joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("[recipe:compose] pattern7_split_scan: composing via RecipeBlock path"));
        }

        let dummy_span = Span::new(0, 0, 0, 0);
        let loop_condition = build_split_scan_loop_condition(&split_facts);
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(loop_condition.clone()),
            body: vec![],
            span: dummy_span,
        };
        let loop_cond_view = CondBlockView::from_expr(&loop_condition);

        let Some(SplitScanRecipe { arena, root }) =
            build_split_scan_recipe(&loop_stmt, loop_cond_view, &split_facts)
        else {
            return Err(Freeze::contract(
                "Pattern7SplitScan recipe build returned None",
            ));
        };

        check_block_contract(&arena, &root, BlockContractKind::NoExit, CTX).map_err(|e| {
            Freeze::contract("Pattern7SplitScan recipe verification failed").with_hint(&e)
        })?;

        let Some(loop_item) = root.items.first() else {
            return Err(Freeze::contract(
                "Pattern7SplitScan recipe root missing LoopV0",
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
                "Pattern7SplitScan recipe root is not LoopV0",
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
                "Pattern7SplitScan recipe lower failed: {e}"
            ))
        })
    }

}
