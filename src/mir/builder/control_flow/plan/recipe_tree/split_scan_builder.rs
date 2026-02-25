//! Pattern7 SplitScan recipe builder for Recipe-first verification

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::loop_types::SplitScanFacts;
use super::build_stmt_only_block;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, IfContractKind, LoopKindV0, LoopV0Features, RecipeBodies, RecipeBlock,
    RecipeItem,
};
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

fn dummy_span() -> Span {
    Span::new(0, 0, 0, 0)
}

#[derive(Debug)]
pub struct SplitScanRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

pub(in crate::mir::builder) fn build_split_scan_recipe(
    loop_stmt: &ASTNode,
    loop_cond_view: CondBlockView,
    facts: &SplitScanFacts,
) -> Option<SplitScanRecipe> {
    let mut arena = RecipeBodies::new();

    // Body 0: loop statement itself
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));

    // If statement (match separator)
    let match_condition = build_match_condition(facts);
    let then_body = build_then_body(facts);
    let else_body = build_else_body(facts);
    let if_stmt = ASTNode::If {
        condition: Box::new(match_condition.clone()),
        then_body: then_body.clone(),
        else_body: Some(else_body.clone()),
        span: dummy_span(),
    };

    // Root body contains only the if statement
    let if_body_id = arena.register(RecipeBody::new(vec![if_stmt]));

    // Then/else blocks
    let then_body_len = then_body.len();
    let then_body_id = arena.register(RecipeBody::new(then_body));
    let then_block = build_stmt_only_block(then_body_id, then_body_len);

    let else_body_len = else_body.len();
    let else_body_id = arena.register(RecipeBody::new(else_body));
    let else_block = build_stmt_only_block(else_body_id, else_body_len);

    let if_item = RecipeItem::IfV2 {
        if_stmt: StmtRef::new(0),
        cond_view: CondBlockView::from_expr(&match_condition),
        contract: IfContractKind::Join,
        then_block: Box::new(then_block),
        else_block: Some(Box::new(else_block)),
    };

    let loop_body_block = RecipeBlock::new(if_body_id, vec![if_item]);

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

    Some(SplitScanRecipe { arena, root })
}


fn build_match_condition(facts: &SplitScanFacts) -> ASTNode {
    let span = Span::unknown();
    let i_var = ASTNode::Variable {
        name: facts.i_var.clone(),
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
    let end_expr = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(i_var.clone()),
        right: Box::new(sep_len),
        span,
    };
    let substring_call = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: facts.s_var.clone(),
            span,
        }),
        method: "substring".to_string(),
        arguments: vec![i_var, end_expr],
        span,
    };

    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(substring_call),
        right: Box::new(ASTNode::Variable {
            name: facts.sep_var.clone(),
            span,
        }),
        span,
    }
}

fn build_then_body(facts: &SplitScanFacts) -> Vec<ASTNode> {
    let span = Span::unknown();

    let push_stmt = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: facts.result_var.clone(),
            span,
        }),
        method: "push".to_string(),
        arguments: vec![ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: facts.s_var.clone(),
                span,
            }),
            method: "substring".to_string(),
            arguments: vec![
                ASTNode::Variable {
                    name: facts.start_var.clone(),
                    span,
                },
                ASTNode::Variable {
                    name: facts.i_var.clone(),
                    span,
                },
            ],
            span,
        }],
        span,
    };

    let start_update = ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: facts.start_var.clone(),
            span,
        }),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: facts.i_var.clone(),
                span,
            }),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: facts.sep_var.clone(),
                    span,
                }),
                method: "length".to_string(),
                arguments: vec![],
                span,
            }),
            span,
        }),
        span,
    };

    let i_set_to_start = ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: facts.i_var.clone(),
            span,
        }),
        value: Box::new(ASTNode::Variable {
            name: facts.start_var.clone(),
            span,
        }),
        span,
    };

    vec![push_stmt, start_update, i_set_to_start]
}

fn build_else_body(facts: &SplitScanFacts) -> Vec<ASTNode> {
    let span = Span::unknown();
    vec![ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: facts.i_var.clone(),
            span,
        }),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: facts.i_var.clone(),
                span,
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span,
            }),
            span,
        }),
        span,
    }]
}
