//! Narrow capture entry for the selected canonical Static Loop body.
use super::*;

impl PhysicalBoundary {
    /// Its finished sequence is still checked in full. Only this exact body
    /// block may carry a leading single-predecessor PHI at capture time.
    pub(in super::super) fn capture_selected_static_loop_with_source_copies(
        function: &MirFunction,
        bindings: &Bindings,
        copies: &[(ValueId, ValueId)],
        borrowed_copies: &Bindings,
        body_block: BasicBlockId,
    ) -> Result<Self, String> {
        Self::capture_with_phi_policy(
            function,
            bindings,
            copies,
            borrowed_copies,
            Some(body_block),
        )
    }
}
