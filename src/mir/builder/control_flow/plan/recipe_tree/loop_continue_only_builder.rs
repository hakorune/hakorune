//! Pattern4Continue recipe builder for Recipe-first verification

use crate::ast::{ASTNode, Span};
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::pattern4_continue_facts::Pattern4ContinueFacts;
use crate::mir::builder::control_flow::plan::recipe_tree::common::{ExitKind, IfMode};
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, IfContractKind, LoopKindV0, LoopV0Features, RecipeBodies, RecipeBlock,
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
pub struct LoopContinueOnlyRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

pub(in crate::mir::builder) fn build_loop_continue_only_recipe(
    loop_stmt: &ASTNode,
    loop_cond_view: CondBlockView,
    continue_cond_view: CondBlockView,
    facts: &Pattern4ContinueFacts,
) -> Option<LoopContinueOnlyRecipe> {
    let mut arena = RecipeBodies::new();

    // Body 0: loop statement itself
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));

    // Synthesize loop increment as an assignment statement (used on both paths).
    let loop_increment_stmt = ASTNode::Assignment {
        target: Box::new(dummy_var(&facts.loop_var)),
        value: Box::new(facts.loop_increment.clone()),
        span: dummy_span(),
    };

    // Body 1: continue path body (step + continue for Exit item reference)
    let continue_body_id = arena.register(RecipeBody::new(vec![
        loop_increment_stmt.clone(),
        ASTNode::Continue {
            span: dummy_span(),
        },
    ]));

    // Build combined body: [if_continue_stmt, carrier_updates..., loop_increment]
    let continue_if_stmt = ASTNode::If {
        condition: Box::new(facts.continue_condition.clone()),
        then_body: vec![ASTNode::Continue {
            span: dummy_span(),
        }],
        else_body: None,
        span: dummy_span(),
    };
    let mut combined_body = vec![continue_if_stmt];
    // Add carrier updates (sorted by key for determinism). Parts expects update
    // statements, not raw expressions, so synthesize carrier assignments.
    let mut keys: Vec<_> = facts.carrier_updates.keys().cloned().collect();
    keys.sort();
    for k in keys {
        combined_body.push(ASTNode::Assignment {
            target: Box::new(dummy_var(&k)),
            value: Box::new(facts.carrier_updates[&k].clone()),
            span: dummy_span(),
        });
    }
    combined_body.push(loop_increment_stmt);
    let combined_body_id = arena.register(RecipeBody::new(combined_body.clone()));

    // Build continue then block: [Stmt(step), Exit(Continue)]
    let continue_then_block = RecipeBlock::new(
        continue_body_id,
        vec![
            RecipeItem::Stmt(StmtRef::new(0)),
            RecipeItem::Exit {
                kind: ExitKind::Continue { depth: 1 },
                stmt: StmtRef::new(1),
            },
        ],
    );

    // Build IfV2 referencing stmt 0 in combined_body
    let continue_if_item = RecipeItem::IfV2 {
        if_stmt: StmtRef::new(0), // continue_if_stmt in combined_body
        cond_view: continue_cond_view,
        contract: IfContractKind::ExitOnly {
            mode: IfMode::ExitIf,
        },
        then_block: Box::new(continue_then_block),
        else_block: None,
    };

    // Build loop body items: [IfV2, Stmt(carrier_updates)..., Stmt(loop_increment)]
    let mut loop_body_items = vec![continue_if_item];
    for i in 1..combined_body.len() {
        loop_body_items.push(RecipeItem::Stmt(StmtRef::new(i)));
    }
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

    Some(LoopContinueOnlyRecipe { arena, root })
}

