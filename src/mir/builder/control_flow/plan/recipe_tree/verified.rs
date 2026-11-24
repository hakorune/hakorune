//! Recipe shape checks for the surviving matcher.

use super::{BlockContractKind, IfContractKind, IfMode, RecipeBlock, RecipeBodies, RecipeItem};

pub(in crate::mir::builder::control_flow::plan::recipe_tree) fn check_block_contract(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    kind: BlockContractKind,
    context: &str,
) -> Result<(), String> {
    verify_block_contract_impl(arena, block, kind, context)
}



fn verify_block_contract_impl(
    arena: &RecipeBodies,
    block: &RecipeBlock,
    kind: BlockContractKind,
    context: &str,
) -> Result<(), String> {
    use crate::mir::builder::control_flow::plan::parts::verify as parts_verify;

    match kind {
        BlockContractKind::StmtOnly => {
            parts_verify::verify_stmt_only_block_contract_if_enabled(arena, block, context)?;
        }
        BlockContractKind::NoExit => {
            parts_verify::verify_no_exit_block_contract_if_enabled(arena, block, context)?;
        }
        BlockContractKind::ExitAllowed => {
            parts_verify::verify_block_contract_if_enabled(arena, block, context)?;
        }
        BlockContractKind::ExitOnly => {
            parts_verify::verify_block_contract_if_enabled(arena, block, context)?;
            if !is_exit_only_block(block) {
                return Err(format!(
                    "[freeze:contract][recipe] expected_exit_only_block: ctx={context}"
                ));
            }
        }
    }

    Ok(())
}




fn is_block_exit_only_item(item: &RecipeItem) -> bool {
    match item {
        RecipeItem::Exit { .. } => true,
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
                // Single-sided exit patterns fall through on one path.
                IfMode::ElseOnlyExit | IfMode::ThenOnlyExit => false,
            }
        }
        _ => false,
    }
}

fn is_exit_only_block(block: &RecipeBlock) -> bool {
    block.items.last().is_some_and(is_block_exit_only_item)
        && block
            .items
            .iter()
            .enumerate()
            .all(|(i, it)| !(is_block_exit_only_item(it) && i + 1 < block.items.len()))
}
