//! LoopCharMap recipe builder (Recipe-first migration Phase C12).
//!
//! Converts LoopCharMapFacts into a RecipeBlock structure.
//!
//! Structure:
//! ```text
//! LoopV0 {
//!     kind: WhileLike
//!     cond_view: <i < s.length()>
//!     body_block: StmtOnly [
//!         Stmt (local ch = haystack.substring(i, i+1))
//!         Stmt (result = result + receiver.method(ch))
//!         Stmt (loop_increment)
//!     ]
//!     body_contract: StmtOnly
//! }
//! ```

use super::build_stmt_only_block;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::LoopCharMapFacts;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, LoopKindV0, LoopV0Features, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;

/// CharMap recipe (arena + root block).
#[derive(Debug)]
pub(in crate::mir::builder) struct CharMapRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

/// Build a RecipeBlock for CharMap from Facts.
///
/// CharMap constraints:
/// - No break, continue, or return statements
/// - No if-else statements
/// - 3-statement body: substring extraction, result update, increment
/// - Step must be +1
///
/// NOTE: Body AST is reconstructed from Facts fields since LoopCharMapFacts
/// does not store the original body AST nodes.
///
/// # Arguments
/// * `loop_stmt` - The loop AST node
/// * `cond_view` - CondBlockView for the loop condition
/// * `facts` - LoopCharMapFacts containing loop parameters
pub(in crate::mir::builder) fn build_char_map_recipe(
    loop_stmt: &ASTNode,
    cond_view: CondBlockView,
    facts: &LoopCharMapFacts,
) -> Option<CharMapRecipe> {
    // Build body 3 statements from Facts
    let body = build_body_from_facts(facts);
    if body.len() != 3 {
        return None;
    }

    let mut arena = RecipeBodies::new();

    // Body 0: loop statement itself
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));

    // Body 1: loop body (3 statements)
    let nested_body_id = arena.register(RecipeBody::new(body));

    // Build nested block (StmtOnly, 3 statements)
    let nested_block = build_stmt_only_block(nested_body_id, 3);

    // Build root LoopV0
    let root = RecipeBlock::new(
        loop_body_id,
        vec![RecipeItem::LoopV0 {
            loop_stmt: StmtRef::new(0),
            kind: LoopKindV0::WhileLike,
            cond_view,
            body_block: Box::new(nested_block),
            body_contract: BlockContractKind::StmtOnly,
            features: LoopV0Features::default(),
        }],
    );

    Some(CharMapRecipe { arena, root })
}

/// Build body AST from Facts fields.
/// Returns 3 statements: substring extraction, result update, increment.
fn build_body_from_facts(facts: &LoopCharMapFacts) -> Vec<ASTNode> {
    use crate::ast::{BinaryOperator, LiteralValue, Span};

    let span = Span::unknown();
    let ch_var = "ch"; // Fixed name (not in Facts)

    // Statement 1: local ch = haystack.substring(i, i+1)
    let stmt1 = ASTNode::Local {
        variables: vec![ch_var.to_string()],
        initial_values: vec![Some(Box::new(ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: facts.haystack_var.clone(),
                span,
            }),
            method: "substring".to_string(),
            arguments: vec![
                ASTNode::Variable {
                    name: facts.loop_var.clone(),
                    span,
                },
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: facts.loop_var.clone(),
                        span,
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span,
                    }),
                    span,
                },
            ],
            span,
        }))],
        span,
    };

    // Statement 2: result = result + receiver.method(ch)
    let receiver = if facts.receiver_var == "me" {
        ASTNode::Me { span }
    } else {
        ASTNode::Variable {
            name: facts.receiver_var.clone(),
            span,
        }
    };

    let stmt2 = ASTNode::Assignment {
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
                object: Box::new(receiver),
                method: facts.transform_method.clone(),
                arguments: vec![ASTNode::Variable {
                    name: ch_var.to_string(),
                    span,
                }],
                span,
            }),
            span,
        }),
        span,
    };

    // Statement 3: rebuild the increment as a statement.
    let stmt3 = ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: facts.loop_var.clone(),
            span,
        }),
        value: Box::new(facts.loop_increment.clone()),
        span,
    };

    vec![stmt1, stmt2, stmt3]
}
