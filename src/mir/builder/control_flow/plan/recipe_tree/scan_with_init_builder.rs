//! ScanWithInit recipe builder for Recipe-first verification

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::loop_types::ScanWithInitFacts;
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
pub(super) struct ScanWithInitRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

pub(super) fn build_scan_with_init_recipe(
    loop_stmt: &ASTNode,
    loop_cond_view: CondBlockView,
    facts: &ScanWithInitFacts,
) -> Option<ScanWithInitRecipe> {
    let mut arena = RecipeBodies::new();

    // Body 0: loop statement itself
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));

    // Exit statement (return loop_var)
    let exit_stmt = ASTNode::Return {
        value: Some(Box::new(ASTNode::Variable {
            name: facts.loop_var.clone(),
            span: dummy_span(),
        })),
        span: dummy_span(),
    };
    let exit_body_id = arena.register(RecipeBody::new(vec![exit_stmt.clone()]));

    // Build combined body: [if_match_return, step]
    let match_condition = build_match_condition(facts);
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

    // IfV2 with ExitOnly contract
    let if_item = RecipeItem::IfV2 {
        if_stmt: StmtRef::new(0),
        cond_view: CondBlockView::from_expr(&match_condition),
        contract: IfContractKind::ExitOnly {
            mode: IfMode::ExitIf,
        },
        then_block: Box::new(exit_then_block),
        else_block: None,
    };

    // Loop body block: [IfV2, Stmt(step)]
    let loop_body_block = RecipeBlock::new(
        combined_body_id,
        vec![if_item, RecipeItem::Stmt(StmtRef::new(1))],
    );

    // Root LoopV0
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

    Some(ScanWithInitRecipe { arena, root })
}

fn build_match_condition(facts: &ScanWithInitFacts) -> ASTNode {
    let span = Span::unknown();
    let loop_var = ASTNode::Variable {
        name: facts.loop_var.clone(),
        span,
    };

    let end_expr = if facts.dynamic_needle {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(loop_var.clone()),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: facts.needle.clone(),
                    span,
                }),
                method: "length".to_string(),
                arguments: vec![],
                span,
            }),
            span,
        }
    } else {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(loop_var.clone()),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span,
            }),
            span,
        }
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

    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(substring_call),
        right: Box::new(ASTNode::Variable {
            name: facts.needle.clone(),
            span,
        }),
        span,
    }
}

fn build_step_stmt(facts: &ScanWithInitFacts) -> ASTNode {
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
