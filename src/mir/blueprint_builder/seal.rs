//! Close physical functions; no source admission or plan selection.
use super::lower::Draft;
pub(super) fn close(drafts: &mut [Draft]) -> Result<(), String> {
    for draft in drafts {
        draft.function.update_cfg();
        draft.function.prune_unreachable_blocks();
        if draft
            .function
            .blocks
            .values()
            .any(|block| block.terminator.is_none())
        {
            return Err("blueprint/internal/unclosed-block".into());
        }
        draft.function.update_cfg();
    }
    Ok(())
}
