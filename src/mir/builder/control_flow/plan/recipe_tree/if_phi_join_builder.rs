//! IfPhiJoin recipe builder for Recipe-first verification

use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::IfPhiJoinFacts;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, IfContractKind, LoopKindV0, LoopV0Features, RecipeBlock, RecipeBodies,
    RecipeItem,
};
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

fn dummy_span() -> Span {
    Span::new(0, 0, 0, 0)
}

fn dummy_var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: dummy_span(),
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct IfPhiJoinRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

pub(in crate::mir::builder) fn build_if_phi_join_recipe(
    loop_stmt: &ASTNode,
    loop_cond_view: CondBlockView,
    if_cond_view: CondBlockView,
    facts: &IfPhiJoinFacts,
) -> Option<IfPhiJoinRecipe> {
    let mut arena = RecipeBodies::new();

    // Body statements: if-else + loop_increment. Parts expects update statements,
    // not raw expressions, so synthesize carrier/step assignments.
    let then_update_stmt = ASTNode::Assignment {
        target: Box::new(dummy_var(&facts.carrier_var)),
        value: Box::new(facts.then_update.clone()),
        span: dummy_span(),
    };
    let else_update_stmt = ASTNode::Assignment {
        target: Box::new(dummy_var(&facts.carrier_var)),
        value: Box::new(facts.else_update.clone()),
        span: dummy_span(),
    };
    let loop_increment_stmt = ASTNode::Assignment {
        target: Box::new(dummy_var(&facts.loop_var)),
        value: Box::new(facts.loop_increment.clone()),
        span: dummy_span(),
    };
    let if_stmt = ASTNode::If {
        condition: Box::new(facts.if_condition.clone()),
        then_body: vec![then_update_stmt.clone()],
        else_body: Some(vec![else_update_stmt.clone()]),
        span: dummy_span(),
    };
    let body_stmts = vec![if_stmt, loop_increment_stmt.clone()];
    let body_id = arena.register(RecipeBody::new(body_stmts));

    // then/else blocks (single stmt each)
    let then_id = arena.register(RecipeBody::new(vec![then_update_stmt]));
    let then_block = RecipeBlock::new(then_id, vec![RecipeItem::Stmt(StmtRef::new(0))]);

    let else_id = arena.register(RecipeBody::new(vec![else_update_stmt]));
    let else_block = RecipeBlock::new(else_id, vec![RecipeItem::Stmt(StmtRef::new(0))]);

    // IfV2 with Join contract (both branches fallthrough)
    let if_item = RecipeItem::IfV2 {
        if_stmt: StmtRef::new(0),
        cond_view: if_cond_view,
        contract: IfContractKind::Join,
        then_block: Box::new(then_block),
        else_block: Some(Box::new(else_block)),
    };

    // Loop body block: [IfV2, Stmt(loop_increment)]
    let loop_body_block =
        RecipeBlock::new(body_id, vec![if_item, RecipeItem::Stmt(StmtRef::new(1))]);

    // Root: LoopV0 with NoExit contract
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));
    let root = RecipeBlock::new(
        loop_body_id,
        vec![RecipeItem::LoopV0 {
            loop_stmt: StmtRef::new(0),
            kind: LoopKindV0::WhileLike,
            cond_view: loop_cond_view,
            body_block: Box::new(loop_body_block),
            body_contract: BlockContractKind::NoExit,
            features: LoopV0Features::default(),
        }],
    );

    Some(IfPhiJoinRecipe { arena, root })
}
