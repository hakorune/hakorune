//! BoolPredicateScan recipe builder for Recipe-first verification

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::BoolPredicateScanFacts;
use crate::mir::builder::control_flow::plan::recipe_tree::common::{ExitKind, IfMode};
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, IfContractKind, LoopKindV0, LoopV0Features, RecipeBlock, RecipeBodies,
    RecipeItem,
};
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;

fn dummy_span() -> Span {
    Span::new(0, 0, 0, 0)
}

#[derive(Debug)]
pub struct BoolPredicateScanRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

pub(in crate::mir::builder) fn build_bool_predicate_scan_recipe(
    loop_stmt: &ASTNode,
    loop_cond_view: CondBlockView,
    facts: &BoolPredicateScanFacts,
) -> Option<BoolPredicateScanRecipe> {
    let mut arena = RecipeBodies::new();

    // Body 0: loop statement itself
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));

    // Exit statement (return false)
    let exit_stmt = ASTNode::Return {
        value: Some(Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(false),
            span: dummy_span(),
        })),
        span: dummy_span(),
    };
    let exit_body_id = arena.register(RecipeBody::new(vec![exit_stmt.clone()]));

    // Build combined body: [if_not_predicate_return_false, step]
    let match_condition = build_not_predicate_condition(facts);
    let if_stmt = ASTNode::If {
        condition: Box::new(match_condition.clone()),
        then_body: vec![exit_stmt],
        else_body: None,
        span: dummy_span(),
    };
    let step_stmt = build_step_stmt(facts);
    let combined_body = vec![if_stmt, step_stmt];
    let combined_body_id = arena.register(RecipeBody::new(combined_body.clone()));

    // Exit-only then block
    let exit_then_block = RecipeBlock::new(
        exit_body_id,
        vec![RecipeItem::Exit {
            kind: ExitKind::Return,
            stmt: StmtRef::new(0),
        }],
    );

    let if_item = RecipeItem::IfV2 {
        if_stmt: StmtRef::new(0),
        cond_view: CondBlockView::from_expr(&match_condition),
        contract: IfContractKind::ExitOnly {
            mode: IfMode::ExitIf,
        },
        then_block: Box::new(exit_then_block),
        else_block: None,
    };

    let loop_body_block = RecipeBlock::new(
        combined_body_id,
        vec![if_item, RecipeItem::Stmt(StmtRef::new(1))],
    );

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

    Some(BoolPredicateScanRecipe { arena, root })
}

fn build_not_predicate_condition(facts: &BoolPredicateScanFacts) -> ASTNode {
    let span = Span::unknown();
    let loop_var = ASTNode::Variable {
        name: facts.loop_var.clone(),
        span,
    };
    let end_expr = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(loop_var.clone()),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span,
        }),
        span,
    };
    let substring_call = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: facts.haystack.clone(),
            span,
        }),
        method: "substring".to_string(),
        arguments: vec![loop_var, end_expr],
        span,
    };

    let receiver = if facts.predicate_receiver == "me" {
        ASTNode::Me { span }
    } else {
        ASTNode::Variable {
            name: facts.predicate_receiver.clone(),
            span,
        }
    };

    let predicate_call = ASTNode::MethodCall {
        object: Box::new(receiver),
        method: facts.predicate_method.clone(),
        arguments: vec![substring_call],
        span,
    };

    ASTNode::UnaryOp {
        operator: UnaryOperator::Not,
        operand: Box::new(predicate_call),
        span,
    }
}

fn build_step_stmt(facts: &BoolPredicateScanFacts) -> ASTNode {
    let span = Span::unknown();
    let loop_var = ASTNode::Variable {
        name: facts.loop_var.clone(),
        span,
    };

    let (op, step_lit) = if facts.step_lit < 0 {
        (BinaryOperator::Subtract, -facts.step_lit)
    } else {
        (BinaryOperator::Add, facts.step_lit)
    };

    ASTNode::Assignment {
        target: Box::new(loop_var.clone()),
        value: Box::new(ASTNode::BinaryOp {
            operator: op,
            left: Box::new(loop_var),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(step_lit),
                span,
            }),
            span,
        }),
        span,
    }
}
