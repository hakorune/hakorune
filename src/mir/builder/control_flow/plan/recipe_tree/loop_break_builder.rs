//! LoopBreak recipe builder (Recipe-first SSOT).
//!
//! Converts LoopBreakFacts into a RecipeBlock structure
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
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::LoopBreakFacts;
use crate::mir::builder::control_flow::plan::recipe_tree::common::{ExitKind, IfMode};
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, IfContractKind, LoopKindV0, LoopV0Features, RecipeBlock, RecipeBodies,
    RecipeItem,
};
use crate::mir::builder::control_flow::plan::LoopBreakStepPlacement;
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;

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

fn should_dedupe_carrier_update(facts: &LoopBreakFacts) -> bool {
    // Keep dedupe conservative: only collapse when both update expressions are exactly the same AST.
    facts.carrier_var == facts.loop_var && facts.carrier_update_in_body == facts.loop_increment
}

/// LoopBreak recipe (arena + root block).
#[derive(Debug)]
pub(in crate::mir::builder) struct LoopBreakRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

/// Build a RecipeBlock for LoopBreak from Facts.
///
/// Returns None if the facts cannot be represented as a valid Recipe shape.
///
/// # Arguments
/// * `loop_stmt` - The loop AST node
/// * `loop_cond_view` - CondBlockView for the loop condition
/// * `break_cond_view` - CondBlockView for the break condition
/// * `facts` - LoopBreakFacts extracted from AST
pub(in crate::mir::builder) fn build_loop_break_recipe(
    loop_stmt: &ASTNode,
    loop_cond_view: CondBlockView,
    break_cond_view: CondBlockView,
    facts: &LoopBreakFacts,
) -> Option<LoopBreakRecipe> {
    let mut arena = RecipeBodies::new();

    // Body 0: loop statement itself
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));

    // Body 1: break-then body (optional carrier update + break)
    let mut break_then_body = Vec::new();
    if let Some(carrier_update_in_break) = &facts.carrier_update_in_break {
        break_then_body.push(ASTNode::Assignment {
            target: Box::new(dummy_var(&facts.carrier_var)),
            value: Box::new(carrier_update_in_break.clone()),
            span: dummy_span(),
        });
    }
    break_then_body.push(ASTNode::Break { span: dummy_span() });
    let break_body_id = arena.register(RecipeBody::new(break_then_body));

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
    // When carrier_var and loop_var point to the same variable with the same update expression,
    // applying both statements causes a double-step bug (e.g. i := f(i); i := f(i) again).
    // Keep a single step statement in that case.
    let dedupe_carrier_update = should_dedupe_carrier_update(facts);

    let (combined_body, if_idx, carrier_idx, step_idx) = match facts.step_placement {
        LoopBreakStepPlacement::Last => {
            let mut body = vec![break_if_stmt, loop_increment_stmt];
            let if_idx = 0usize;
            let step_idx = 1usize;
            let carrier_idx = if dedupe_carrier_update {
                None
            } else {
                body.insert(1, carrier_update_stmt);
                Some(1usize)
            };
            let step_idx = if dedupe_carrier_update {
                step_idx
            } else {
                2usize
            };
            (body, if_idx, carrier_idx, step_idx)
        }
        LoopBreakStepPlacement::BeforeBreak => {
            let mut body = vec![loop_increment_stmt, break_if_stmt];
            let step_idx = 0usize;
            let if_idx = 1usize;
            let carrier_idx = if dedupe_carrier_update {
                None
            } else {
                body.push(carrier_update_stmt);
                Some(2usize)
            };
            (body, if_idx, carrier_idx, step_idx)
        }
    };
    let combined_body_id = arena.register(RecipeBody::new(combined_body));

    // Build break-then block:
    // - without carrier_update_in_break: [Exit(Break)]
    // - with carrier_update_in_break: [Stmt(update), Exit(Break)]
    let break_then_items = if facts.carrier_update_in_break.is_some() {
        vec![
            RecipeItem::Stmt(StmtRef::new(0)),
            RecipeItem::Exit {
                kind: ExitKind::Break { depth: 1 },
                stmt: StmtRef::new(1),
            },
        ]
    } else {
        vec![RecipeItem::Exit {
            kind: ExitKind::Break { depth: 1 },
            stmt: StmtRef::new(0),
        }]
    };
    let break_then_block = RecipeBlock::new(break_body_id, break_then_items);

    // Build IfV2 referencing stmt 0 in combined_body
    let break_if_item = RecipeItem::IfV2 {
        if_stmt: StmtRef::new(if_idx), // break_if_stmt in combined_body
        cond_view: break_cond_view,
        contract: IfContractKind::ExitOnly {
            mode: IfMode::ExitIf,
        },
        then_block: Box::new(break_then_block),
        else_block: None,
    };

    // Build loop body block. Item order determines evaluation order, so
    // step-before-break must place the step stmt ahead of the break-if.
    let loop_body_items = match facts.step_placement {
        LoopBreakStepPlacement::Last => {
            let mut items = vec![break_if_item];
            if let Some(carrier_idx) = carrier_idx {
                items.push(RecipeItem::Stmt(StmtRef::new(carrier_idx))); // carrier_update_in_body
            }
            items.push(RecipeItem::Stmt(StmtRef::new(step_idx))); // loop_increment
            items
        }
        LoopBreakStepPlacement::BeforeBreak => {
            let mut items = vec![RecipeItem::Stmt(StmtRef::new(step_idx)), break_if_item]; // loop_increment, break-if
            if let Some(carrier_idx) = carrier_idx {
                items.push(RecipeItem::Stmt(StmtRef::new(carrier_idx))); // carrier_update_in_body
            }
            items
        }
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
    fn test_build_loop_break_recipe_supports_carrier_update_in_break() {
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
        let facts = LoopBreakFacts {
            loop_var: "i".to_string(),
            carrier_var: "sum".to_string(),
            loop_condition: cond.clone(),
            break_condition: break_cond.clone(),
            carrier_update_in_break: Some(carrier_update_in_break),
            carrier_update_in_body,
            loop_increment,
            step_placement: crate::mir::builder::control_flow::plan::LoopBreakStepPlacement::Last,
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

        assert!(
            result.is_some(),
            "Should build recipe for carrier_update_in_break"
        );
    }

    #[test]
    fn dedupes_double_update_when_carrier_and_loop_var_are_identical() {
        let span = dummy_span();
        let cond = ASTNode::Variable {
            name: "cond".to_string(),
            span,
        };
        let break_cond = ASTNode::Variable {
            name: "break_cond".to_string(),
            span,
        };
        let step_expr = ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span,
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span,
            }),
            span,
        };
        let facts = LoopBreakFacts {
            loop_var: "i".to_string(),
            carrier_var: "i".to_string(),
            loop_condition: cond.clone(),
            break_condition: break_cond.clone(),
            carrier_update_in_break: None,
            carrier_update_in_body: step_expr.clone(),
            loop_increment: step_expr,
            step_placement: crate::mir::builder::control_flow::plan::LoopBreakStepPlacement::Last,
        };
        let loop_stmt = ASTNode::Loop {
            condition: Box::new(cond),
            body: vec![],
            span: dummy_span(),
        };

        let recipe = build_loop_break_recipe(
            &loop_stmt,
            CondBlockView::from_expr(&facts.loop_condition),
            CondBlockView::from_expr(&facts.break_condition),
            &facts,
        )
        .expect("recipe should be built");

        let loop_item = recipe.root.items.first().expect("root should contain loop");
        let crate::mir::builder::control_flow::plan::recipe_tree::RecipeItem::LoopV0 {
            body_block,
            ..
        } = loop_item
        else {
            panic!("expected LoopV0 root item");
        };

        assert_eq!(
            body_block.items.len(),
            2,
            "break-if + step only (no duplicate carrier update)"
        );
        assert!(matches!(
            body_block.items.as_slice(),
            [
                crate::mir::builder::control_flow::plan::recipe_tree::RecipeItem::IfV2 { .. },
                crate::mir::builder::control_flow::plan::recipe_tree::RecipeItem::Stmt(_)
            ]
        ));
    }
}
