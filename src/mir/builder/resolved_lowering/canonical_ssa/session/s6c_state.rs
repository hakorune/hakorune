//! Storage-only state for the canonical session's existing S6C lifecycle.
//!
//! The parent session remains the owner and issuer.  This child only keeps
//! state that is already validated by the parent lifecycle boundaries.

use crate::mir::BasicBlockId;

use super::residence_lifecycle::PinnedTextResidenceLifecycleStateV1;

pub(super) struct CanonicalSsaS6cStateV1 {
    deferred_cursor_blocks: Option<[BasicBlockId; 5]>,
    pinned_text_residence: Option<PinnedTextResidenceLifecycleStateV1>,
}

impl CanonicalSsaS6cStateV1 {
    pub(super) const fn new() -> Self {
        Self {
            deferred_cursor_blocks: None,
            pinned_text_residence: None,
        }
    }

    pub(super) fn replace_deferred_cursor_blocks(
        &mut self,
        blocks: [BasicBlockId; 5],
    ) -> Option<[BasicBlockId; 5]> {
        self.deferred_cursor_blocks.replace(blocks)
    }

    pub(super) fn take_deferred_cursor_blocks(&mut self) -> Option<[BasicBlockId; 5]> {
        self.deferred_cursor_blocks.take()
    }

    pub(super) fn pinned_text_residence(&self) -> Option<&PinnedTextResidenceLifecycleStateV1> {
        self.pinned_text_residence.as_ref()
    }

    pub(super) fn set_pinned_text_residence(&mut self, state: PinnedTextResidenceLifecycleStateV1) {
        self.pinned_text_residence = Some(state);
    }

    pub(super) fn pinned_text_residence_mut(
        &mut self,
    ) -> Option<&mut PinnedTextResidenceLifecycleStateV1> {
        self.pinned_text_residence.as_mut()
    }
}
