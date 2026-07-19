//! Provider-neutral block driver for verified Parts associated-source items.
//!
//! This is the sole owner of block iteration, early terminal detection, and
//! the ExitOnly all-path postcondition. Source projection and item semantics
//! remain owned by `PartsAssociatedSourceV1` and the associated dispatcher.

use crate::mir::builder::control_flow::plan::LoweredRecipe;

use super::dispatch::{
    lower_verified_parts_associated_item, PartsAssociatedBlockModeV1,
    PartsAssociatedLoweringHooksV1,
};
use super::{PartsAssociatedSourceErrorV1, PartsAssociatedSourceV1};

pub(in crate::mir::builder::control_flow::plan::parts) fn lower_verified_parts_associated_block<
    S,
    H,
    RenderSourceError,
    IsTerminal,
>(
    source: &S,
    block: &S::BlockInput,
    mode: PartsAssociatedBlockModeV1,
    hooks: &mut H,
    error_prefix: &str,
    render_source_error: RenderSourceError,
    is_terminal: IsTerminal,
) -> Result<Vec<LoweredRecipe>, String>
where
    S: PartsAssociatedSourceV1,
    H: PartsAssociatedLoweringHooksV1<S, Output = Vec<LoweredRecipe>>,
    RenderSourceError: Fn(PartsAssociatedSourceErrorV1) -> String,
    IsTerminal: Fn(&[LoweredRecipe]) -> bool,
{
    let item_count = source.block_len(block).map_err(&render_source_error)?;
    let mut plans = Vec::new();

    for index in 0..item_count {
        let item = source.item(block, index).map_err(&render_source_error)?;
        plans.extend(lower_verified_parts_associated_item::<S, H>(
            mode,
            item,
            hooks,
            error_prefix,
        )?);
        if mode != PartsAssociatedBlockModeV1::ExitOnly && is_terminal(&plans) {
            break;
        }
    }

    if mode == PartsAssociatedBlockModeV1::ExitOnly && !is_terminal(&plans) {
        return Err(format!(
            "[freeze:contract][recipe] exit_only_block_must_end_with_exit: ctx={}",
            error_prefix
        ));
    }

    Ok(plans)
}
