//! AccumConstLoop recipe builder for Recipe-first verification

use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::AccumConstLoopFacts;
use super::build_stmt_only_block;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, LoopKindV0, LoopV0Features, RecipeBodies, RecipeBlock, RecipeItem,
};
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

fn dummy_span() -> Span {
    Span::new(0, 0, 0, 0)
}

#[derive(Debug)]
pub struct AccumConstLoopRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

pub(in crate::mir::builder) fn build_accum_const_loop_recipe(
    loop_stmt: &ASTNode,
    loop_cond_view: CondBlockView,
    facts: &AccumConstLoopFacts,
) -> Option<AccumConstLoopRecipe> {
    let mut arena = RecipeBodies::new();

    // Body 0: loop statement itself
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));

    // Loop body: assign the derived expressions back to their variables.
    let acc_assign = ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: facts.acc_var.clone(),
            span: dummy_span(),
        }),
        value: Box::new(facts.acc_update.clone()),
        span: dummy_span(),
    };
    let loop_assign = ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: facts.loop_var.clone(),
            span: dummy_span(),
        }),
        value: Box::new(facts.loop_increment.clone()),
        span: dummy_span(),
    };
    let body = vec![acc_assign, loop_assign];
    let body_id = arena.register(RecipeBody::new(body));
    let body_block = build_stmt_only_block(body_id, 2);

    let root = RecipeBlock::new(
        loop_body_id,
        vec![RecipeItem::LoopV0 {
            loop_stmt: StmtRef::new(0),
            kind: LoopKindV0::WhileLike,
            cond_view: loop_cond_view,
            body_block: Box::new(body_block),
            body_contract: BlockContractKind::StmtOnly,
            features: LoopV0Features::default(),
        }],
    );

    Some(AccumConstLoopRecipe { arena, root })
}
