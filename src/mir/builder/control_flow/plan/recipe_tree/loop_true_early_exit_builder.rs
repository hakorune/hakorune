//! LoopTrueEarlyExit recipe builder for Recipe-first verification

use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::domain::LoopTrueEarlyExitKind;
use crate::mir::builder::control_flow::plan::facts::LoopTrueEarlyExitFacts;
use crate::mir::builder::control_flow::plan::recipe_tree::common::{ExitKind, IfMode};
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, IfContractKind, LoopKindV0, LoopV0Features, RecipeBlock, RecipeBodies,
    RecipeItem,
};
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

fn dummy_span() -> Span {
    Span::new(0, 0, 0, 0)
}

#[derive(Debug)]
pub struct LoopTrueEarlyExitRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

pub(in crate::mir::builder) fn build_loop_true_early_exit_recipe(
    loop_stmt: &ASTNode,
    exit_cond_view: CondBlockView,
    facts: &LoopTrueEarlyExitFacts,
) -> Option<LoopTrueEarlyExitRecipe> {
    let mut arena = RecipeBodies::new();

    // Body 0: loop statement itself
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));

    // Determine exit kind and exit statement
    let (exit_kind, exit_stmt) = match facts.exit_kind {
        LoopTrueEarlyExitKind::Return => {
            let stmt = if let Some(ref val) = facts.exit_value {
                ASTNode::Return {
                    value: Some(Box::new(val.clone())),
                    span: dummy_span(),
                }
            } else {
                ASTNode::Return {
                    value: None,
                    span: dummy_span(),
                }
            };
            (ExitKind::Return, stmt)
        }
        LoopTrueEarlyExitKind::Break => (
            ExitKind::Break { depth: 1 },
            ASTNode::Break { span: dummy_span() },
        ),
    };

    // Body 1: exit statement (for Exit item)
    let exit_body_id = arena.register(RecipeBody::new(vec![exit_stmt.clone()]));

    // Build combined body: [if_exit_stmt, carrier_update?, loop_increment]
    let exit_if_stmt = ASTNode::If {
        condition: Box::new(facts.exit_condition.clone()),
        then_body: vec![exit_stmt],
        else_body: None,
        span: dummy_span(),
    };
    let mut combined_body = vec![exit_if_stmt];
    if let (Some(carrier_var), Some(update)) = (&facts.carrier_var, &facts.carrier_update) {
        combined_body.push(ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: carrier_var.clone(),
                span: dummy_span(),
            }),
            value: Box::new(update.clone()),
            span: dummy_span(),
        });
    }
    combined_body.push(ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: facts.loop_var.clone(),
            span: dummy_span(),
        }),
        value: Box::new(facts.loop_increment.clone()),
        span: dummy_span(),
    });
    let combined_body_id = arena.register(RecipeBody::new(combined_body.clone()));

    // Build exit then block: [Exit(Return or Break)]
    let exit_then_block = RecipeBlock::new(
        exit_body_id,
        vec![RecipeItem::Exit {
            kind: exit_kind,
            stmt: StmtRef::new(0),
        }],
    );

    // Build IfV2 referencing stmt 0 in combined_body
    let exit_if_item = RecipeItem::IfV2 {
        if_stmt: StmtRef::new(0),
        cond_view: exit_cond_view,
        contract: IfContractKind::ExitOnly {
            mode: IfMode::ExitIf,
        },
        then_block: Box::new(exit_then_block),
        else_block: None,
    };

    // Build loop body items: [IfV2, Stmt(carrier_update)?, Stmt(loop_increment)]
    let mut loop_body_items = vec![exit_if_item];
    for i in 1..combined_body.len() {
        loop_body_items.push(RecipeItem::Stmt(StmtRef::new(i)));
    }
    let loop_body_block = RecipeBlock::new(combined_body_id, loop_body_items);

    // Build root LoopV0 (InfiniteLike for loop(true))
    // Use true literal for cond_view (loop(true) condition)
    let true_cond = ASTNode::Literal {
        value: crate::ast::LiteralValue::Bool(true),
        span: dummy_span(),
    };
    let root = RecipeBlock::new(
        loop_body_id,
        vec![RecipeItem::LoopV0 {
            loop_stmt: StmtRef::new(0),
            kind: LoopKindV0::Infinite,
            cond_view: CondBlockView::from_expr(&true_cond),
            body_block: Box::new(loop_body_block),
            body_contract: BlockContractKind::ExitAllowed,
            features: LoopV0Features::default(),
        }],
    );

    Some(LoopTrueEarlyExitRecipe { arena, root })
}
