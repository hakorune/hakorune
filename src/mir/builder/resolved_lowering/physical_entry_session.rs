//! One consuming common-V2 physical-entry/session seam.
//!
//! The prepared input retains the same installed loan as the common-V2
//! admission.  This helper is the only caller-zero owner allowed to open the
//! unpublished function transaction, install the detached shell, adopt the
//! entry lanes, and discard the outer session on every exit.

use super::common_v2_session::with_common_v2_canonical_session;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::common_v2_physical_function_skeleton::PreparedPhysicalEntrySessionInputV1;

/// Consume one prepared physical entry input and one same-loan common-V2
/// admission.  No session, Builder view, or sidecar escapes this callback.
pub(in crate::mir::builder) fn with_common_v2_physical_entry_session<R>(
    builder: &mut MirBuilder,
    mut prepared: PreparedPhysicalEntrySessionInputV1<'_, '_, '_>,
    callback: impl FnOnce(
        &mut super::common_v2_session::CommonV2CanonicalSessionRefV1<'_, '_>,
        &mut MirBuilder,
    ) -> Result<R, String>,
) -> Result<R, String> {
    if builder.function_state.current_function.is_some()
        || builder.function_state.current_block.is_some()
    {
        return Err("physical entry session requires an empty Builder".to_owned());
    }

    let function_name = prepared.function_name().to_owned();

    prepared.with_admission(|prepared, admission| {
        let source_input = admission.input();
        with_common_v2_canonical_session(admission, |mut common| {
            let mut outer = builder.open_resolved_function_draft_seal_session_v1(&function_name);
            let result = (|| {
                let (detached, descriptors, _stamp) = prepared.take_install_parts();
                let draft = outer.builder_view_mut_for_lowering();
                draft
                    .function_state
                    .resolved_binding_state
                    .install(source_input.function())?;
                draft.install_prepared_physical_function_skeleton(detached)?;
                common.adopt_physical_entry_lanes(draft, &descriptors)?;
                callback(&mut common, draft)
            })();

            // The outer function transaction is the sole rollback owner.  It
            // restores the caller state and clears the unpublished shell even
            // when adoption or the callback rejects after partial mutation.
            outer.discard_unpublished();
            result
        })
    })?
}
