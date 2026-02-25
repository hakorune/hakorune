//! Pattern1SimpleWhile Recipe Builder (Recipe-first migration Phase C11).
//!
//! Converts Pattern1SimpleWhileFacts into a RecipeBlock structure.
//!
//! Structure:
//! ```text
//! LoopV0 {
//!     kind: WhileLike
//!     cond_view: <loop condition>
//!     body_block: StmtOnly [
//!         Stmt (loop increment)
//!     ]
//!     body_contract: StmtOnly
//! }
//! ```

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, LoopKindV0, LoopV0Features, RecipeBodies, RecipeBlock, RecipeItem,
};
use super::build_stmt_only_block;
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

/// LoopSimpleWhile recipe (arena + root block).
#[derive(Debug)]
pub(in crate::mir::builder) struct LoopSimpleWhileRecipe {
    pub arena: RecipeBodies,
    pub root: RecipeBlock,
}

/// Build a RecipeBlock for LoopSimpleWhile from Facts.
///
/// LoopSimpleWhile constraints:
/// - No break, continue, or return statements
/// - No if-else statements
/// - Step-only body (loop increment only)
/// - Step must be +1
///
/// # Arguments
/// * `loop_stmt` - The loop AST node
/// * `cond_view` - CondBlockView for the loop condition
/// * `body` - Loop body statements (from facts extraction)
pub(in crate::mir::builder) fn build_loop_simple_while_recipe(
    loop_stmt: &ASTNode,
    cond_view: CondBlockView,
    body: &[ASTNode],
) -> Option<LoopSimpleWhileRecipe> {
    // Validate body is stmt-only (no control flow)
    // This should already be validated by facts extraction
    if body.is_empty() {
        return None;
    }

    let mut arena = RecipeBodies::new();

    // Body 0: loop statement itself
    let loop_body_id = arena.register(RecipeBody::new(vec![loop_stmt.clone()]));

    // Body 1: loop body (stmt-only, increment only)
    let nested_body_id = arena.register(RecipeBody::new(body.to_vec()));

    // Build nested block (StmtOnly)
    let nested_block = build_stmt_only_block(nested_body_id, body.len());

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

    Some(LoopSimpleWhileRecipe { arena, root })
}
