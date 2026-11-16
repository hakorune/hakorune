//! Close physical functions; never select a source route or repair a contract.
use super::lower::Draft;

pub(super) fn close(drafts: &mut [Draft]) -> Result<(), String> {
    for draft in drafts {
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
