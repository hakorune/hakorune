//! LoopArrayJoin recipe builder (Recipe-first migration Phase C13).
//!
//! Structure:
//! LoopV0 {
//!     kind: WhileLike
//!     cond_view: <i < arr.length()>
//!     body_block: NoExit [
//!         IfV2 { Join, then: separator追加, else: None }
//!         Stmt (result = result + array.get(i))
//!         Stmt (loop_increment)
//!     ]
//!     body_contract: NoExit
//! }

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::LoopArrayJoinFacts;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, IfContractKind, LoopKindV0, LoopV0Features, RecipeBodies, RecipeBlock,
    RecipeItem,
};
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

#[derive(Debug)]
pub(in crate::mir::builder) struct ArrayJoinRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

pub(in crate::mir::builder) fn build_array_join_recipe(
    loop_stmt: &ASTNode,
    loop_cond_view: CondBlockView,
    if_cond_view: CondBlockView,
    facts: &LoopArrayJoinFacts,
) -> Option<ArrayJoinRecipe> {
    let body = build_body_from_facts(facts);
    if body.len() != 3 {
        return None;
    }

    let mut arena = RecipeBodies::new();

    // Body 0: loop statement itself
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));

    // Body 1: loop body (3 statements: if, assign, increment)
    let nested_body_id = arena.register(RecipeBody::new(body));

    // Body 2: then block (separator append - 1 statement inside the if)
    let then_body_id = arena.register(RecipeBody::new(vec![build_separator_append(facts)]));

    // Build then_block for IfV2 (1 statement)
    let then_block = RecipeBlock::new(then_body_id, vec![RecipeItem::Stmt(StmtRef::new(0))]);

    // Build IfV2 item
    let if_item = RecipeItem::IfV2 {
        if_stmt: StmtRef::new(0), // points to body[0] = if statement
        cond_view: if_cond_view,
        contract: IfContractKind::Join,
        then_block: Box::new(then_block),
        else_block: None, // fallthrough
    };

    // Build loop body block (NoExit: IfV2 + Stmt + Stmt)
    let nested_block = RecipeBlock::new(
        nested_body_id,
        vec![
            if_item,
            RecipeItem::Stmt(StmtRef::new(1)), // array.get append
            RecipeItem::Stmt(StmtRef::new(2)), // increment
        ],
    );

    // Build root LoopV0
    let root = RecipeBlock::new(
        loop_body_id,
        vec![RecipeItem::LoopV0 {
            loop_stmt: StmtRef::new(0),
            kind: LoopKindV0::WhileLike,
            cond_view: loop_cond_view,
            body_block: Box::new(nested_block),
            body_contract: BlockContractKind::NoExit,
            features: LoopV0Features::default(),
        }],
    );

    Some(ArrayJoinRecipe { arena, root })
}


/// Build separator append statement: result = result + separator
fn build_separator_append(facts: &LoopArrayJoinFacts) -> ASTNode {
    use crate::ast::{BinaryOperator, Span};
    let span = Span::new(0, 0, 0, 0);

    ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: facts.result_var.clone(),
            span,
        }),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: facts.result_var.clone(),
                span,
            }),
            right: Box::new(ASTNode::Variable {
                name: facts.separator_var.clone(),
                span,
            }),
            span,
        }),
        span,
    }
}

fn build_body_from_facts(facts: &LoopArrayJoinFacts) -> Vec<ASTNode> {
    use crate::ast::{BinaryOperator, Span};
    let span = Span::new(0, 0, 0, 0);

    // Stmt 0: if (i > 0) { result = result + separator }
    let stmt0 = ASTNode::If {
        condition: Box::new(facts.if_condition.clone()),
        then_body: vec![build_separator_append(facts)],
        else_body: None,
        span,
    };

    // Stmt 1: result = result + array.get(i)
    let stmt1 = ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: facts.result_var.clone(),
            span,
        }),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: facts.result_var.clone(),
                span,
            }),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: facts.array_var.clone(),
                    span,
                }),
                method: "get".to_string(),
                arguments: vec![ASTNode::Variable {
                    name: facts.loop_var.clone(),
                    span,
                }],
                span,
            }),
            span,
        }),
        span,
    };

    // Stmt 2: rebuild the increment as a statement.
    let stmt2 = ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: facts.loop_var.clone(),
            span,
        }),
        value: Box::new(facts.loop_increment.clone()),
        span,
    };

    vec![stmt0, stmt1, stmt2]
}
