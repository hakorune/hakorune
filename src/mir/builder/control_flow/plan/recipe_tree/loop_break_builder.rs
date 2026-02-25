//! Pattern2Break Recipe Builder (Recipe-first SSOT).
//!
//! Converts Pattern2BreakFacts into a RecipeBlock structure
//! for verification and lowering via parts::entry.
//!
//! Structure:
//! ```text
//! LoopV0 {
//!     body_block: [
//!         IfV2 { contract: ExitOnly(ExitIf), then: [Exit(Break)] }
//!         Stmt (carrier_update_in_body)
//!         Stmt (loop_increment)
//!     ]
//! }
//! ```

use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::pattern2_break_types::Pattern2BreakFacts;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, IfContractKind, LoopKindV0, LoopV0Features,
    RecipeBodies, RecipeBlock, RecipeItem,
};
use crate::mir::builder::control_flow::plan::recipe_tree::common::{ExitKind, IfMode};
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::builder::control_flow::plan::Pattern2StepPlacement;

/// Dummy span for synthetic AST nodes.
fn dummy_span() -> Span {
    Span::new(0, 0, 0, 0)
}

fn dummy_var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: dummy_span(),
    }
}

/// LoopBreak recipe (arena + root block).
#[derive(Debug)]
pub(in crate::mir::builder) struct LoopBreakRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

/// Build a RecipeBlock for Pattern2Break from Facts.
///
/// Returns None if the facts cannot be represented as a valid Recipe
/// (e.g., carrier_update_in_break is present - not supported in minimal impl).
///
/// # Arguments
/// * `loop_stmt` - The loop AST node
/// * `loop_cond_view` - CondBlockView for the loop condition
/// * `break_cond_view` - CondBlockView for the break condition
/// * `facts` - Pattern2BreakFacts extracted from AST
pub(in crate::mir::builder) fn build_loop_break_recipe(
    loop_stmt: &ASTNode,
    loop_cond_view: CondBlockView,
    break_cond_view: CondBlockView,
    facts: &Pattern2BreakFacts,
) -> Option<LoopBreakRecipe> {
    // Minimal implementation: only support simple case without carrier_update_in_break
    if facts.carrier_update_in_break.is_some() {
        return None;
    }

    let mut arena = RecipeBodies::new();

    // Body 0: loop statement itself
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));

    // Body 1: break statement (for Exit item)
    let break_body_id = arena.register(RecipeBody::new(vec![ASTNode::Break { span: dummy_span() }]));

    // Body 2: combined loop body [if_stmt, carrier_update, loop_increment]
    let break_if_stmt = ASTNode::If {
        condition: Box::new(facts.break_condition.clone()),
        then_body: vec![ASTNode::Break { span: dummy_span() }],
        else_body: None,
        span: dummy_span(),
    };
    // Parts expects update statements, not raw expressions.
    let carrier_update_stmt = ASTNode::Assignment {
        target: Box::new(dummy_var(&facts.carrier_var)),
        value: Box::new(facts.carrier_update_in_body.clone()),
        span: dummy_span(),
    };
    let loop_increment_stmt = ASTNode::Assignment {
        target: Box::new(dummy_var(&facts.loop_var)),
        value: Box::new(facts.loop_increment.clone()),
        span: dummy_span(),
    };
    let (combined_body, if_idx, carrier_idx, step_idx) = match facts.step_placement {
        Pattern2StepPlacement::Last => (
            vec![break_if_stmt, carrier_update_stmt, loop_increment_stmt],
            0,
            1,
            2,
        ),
        Pattern2StepPlacement::BeforeBreak => (
            vec![loop_increment_stmt, break_if_stmt, carrier_update_stmt],
            1,
            2,
            0,
        ),
    };
    let combined_body_id = arena.register(RecipeBody::new(combined_body));

    // Build break then block: [Exit(Break)]
    let break_then_block = RecipeBlock::new(
        break_body_id,
        vec![RecipeItem::Exit {
            kind: ExitKind::Break { depth: 1 },
            stmt: StmtRef::new(0),
        }],
    );

    // Build IfV2 referencing stmt 0 in combined_body
    let break_if_item = RecipeItem::IfV2 {
        if_stmt: StmtRef::new(if_idx), // break_if_stmt in combined_body
        cond_view: break_cond_view,
        contract: IfContractKind::ExitOnly { mode: IfMode::ExitIf },
        then_block: Box::new(break_then_block),
        else_block: None,
    };

    // Build loop body block. Item order determines evaluation order, so
    // step-before-break must place the step stmt ahead of the break-if.
    let loop_body_items = match facts.step_placement {
        Pattern2StepPlacement::Last => vec![
            break_if_item,
            RecipeItem::Stmt(StmtRef::new(carrier_idx)), // carrier_update_in_body
            RecipeItem::Stmt(StmtRef::new(step_idx)),    // loop_increment
        ],
        Pattern2StepPlacement::BeforeBreak => vec![
            RecipeItem::Stmt(StmtRef::new(step_idx)),    // loop_increment
            break_if_item,
            RecipeItem::Stmt(StmtRef::new(carrier_idx)), // carrier_update_in_body
        ],
    };
    let loop_body_block = RecipeBlock::new(combined_body_id, loop_body_items);

    // Build root LoopV0
    let root = RecipeBlock::new(
        loop_body_id,
        vec![RecipeItem::LoopV0 {
            loop_stmt: StmtRef::new(0),
            kind: LoopKindV0::WhileLike,
            cond_view: loop_cond_view,
            body_block: Box::new(loop_body_block),
            body_contract: BlockContractKind::ExitAllowed,
            features: LoopV0Features::default(),
        }],
    );

    Some(LoopBreakRecipe { arena, root })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::LiteralValue;

    #[test]
    fn test_build_loop_break_recipe_returns_none_for_carrier_in_break() {
        let span = dummy_span();
        let cond = ASTNode::Variable {
            name: "cond".to_string(),
            span,
        };
        let break_cond = ASTNode::Variable {
            name: "break_cond".to_string(),
            span,
        };
        let carrier_update_in_break = ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span,
        };
        let carrier_update_in_body = ASTNode::Literal {
            value: LiteralValue::Integer(2),
            span,
        };
        let loop_increment = ASTNode::Literal {
            value: LiteralValue::Integer(3),
            span,
        };
        let facts = Pattern2BreakFacts {
            loop_var: "i".to_string(),
            carrier_var: "sum".to_string(),
            loop_condition: cond.clone(),
            break_condition: break_cond.clone(),
            carrier_update_in_break: Some(carrier_update_in_break),
            carrier_update_in_body,
            loop_increment,
            step_placement: crate::mir::builder::control_flow::plan::Pattern2StepPlacement::Last,
        };

        let loop_stmt = ASTNode::Loop {
            condition: Box::new(cond),
            body: vec![],
            span: dummy_span(),
        };

        let result = build_loop_break_recipe(
            &loop_stmt,
            CondBlockView::from_expr(&facts.loop_condition),
            CondBlockView::from_expr(&facts.break_condition),
            &facts,
        );

        assert!(result.is_none(), "Should return None for carrier_update_in_break");
    }
}
