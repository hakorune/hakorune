//! Shape predicates for RecipeBlock verification.
//!
//! These helpers are verifier-only classification logic. They do not inspect
//! the AST arena and must not lower or mutate state.

use crate::mir::builder::control_flow::plan::recipe_tree::{
    IfContractKind, IfMode, RecipeBlock, RecipeItem,
};

pub(super) fn is_block_exit_only_item(item: &RecipeItem) -> bool {
    #[allow(unreachable_patterns)]
    match item {
        RecipeItem::Exit { .. } => true,
        RecipeItem::Stmt(_) => false,
        RecipeItem::IfV2 { contract, .. } if matches!(contract, IfContractKind::Join) => false,
        RecipeItem::IfV2 {
            contract,
            then_block,
            else_block,
            ..
        } => {
            let IfContractKind::ExitOnly { mode } = contract else {
                return false;
            };
            match mode {
                // ExitIf exits only on the `then` path; it must not be treated as a
                // block-exit item (trailing items are allowed).
                IfMode::ExitIf => false,
                IfMode::ExitAll => else_block
                    .as_deref()
                    .is_some_and(|eb| is_exit_only_block(then_block) && is_exit_only_block(eb)),
                // ElseOnlyExit: then falls through, so not a block-exit item.
                IfMode::ElseOnlyExit => false,
            }
        }
        _ => false,
    }
}

pub(super) fn is_exit_only_block(block: &RecipeBlock) -> bool {
    block.items.last().is_some_and(is_block_exit_only_item)
        && block
            .items
            .iter()
            .enumerate()
            .all(|(i, it)| !(is_block_exit_only_item(it) && i + 1 < block.items.len()))
}
