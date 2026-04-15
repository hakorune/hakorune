//! RecipeTree branch-local scope helpers.
//!
//! This owns the `RecipeBlock` recursion used by join-obligation filtering.

use super::{RecipeBlock, RecipeBodies, RecipeItem};
use crate::ast::ASTNode;
use std::collections::BTreeSet;

pub(in crate::mir::builder) fn collect_branch_local_vars_from_block_recursive(
    arena: &RecipeBodies,
    block: &RecipeBlock,
) -> BTreeSet<String> {
    let mut locals = BTreeSet::new();
    let Some(body) = arena.get(block.body_id) else {
        return locals;
    };
    for item in &block.items {
        match item {
            RecipeItem::Stmt(stmt_ref) => {
                if let Some(ASTNode::Local { variables, .. }) = body.get_ref(*stmt_ref) {
                    for name in variables {
                        locals.insert(name.clone());
                    }
                }
            }
            RecipeItem::IfV2 {
                then_block,
                else_block,
                ..
            } => {
                locals.extend(collect_branch_local_vars_from_block_recursive(
                    arena, then_block,
                ));
                if let Some(else_block) = else_block.as_deref() {
                    locals.extend(collect_branch_local_vars_from_block_recursive(
                        arena, else_block,
                    ));
                }
            }
            RecipeItem::LoopV0 { body_block, .. } => {
                locals.extend(collect_branch_local_vars_from_block_recursive(
                    arena, body_block,
                ));
            }
            _ => {}
        }
    }
    locals
}
