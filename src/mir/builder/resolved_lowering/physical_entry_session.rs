//! One consuming common-V2 physical-entry/session seam.
//!
//! The prepared input retains the same installed loan as the common-V2
//! admission.  This helper is the only caller-zero owner allowed to open the
//! unpublished function transaction, install the detached shell, adopt the
//! entry lanes, and discard the outer session on every exit.

use super::common_v2_session::with_common_v2_canonical_session_branded;
use crate::mir::builder::InvocationBranded;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::common_v2_physical_function_skeleton::PreparedPhysicalEntrySessionInputV1;
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;

/// Consume one prepared physical entry input and one same-loan common-V2
/// admission.  No session, Builder view, or sidecar escapes this callback.
fn with_common_v2_physical_entry_session_branded_with_effects<R>(
    builder: &mut MirBuilder,
    prepared: InvocationBranded<PreparedPhysicalEntrySessionInputV1<'_, '_, '_>>,
    expected_brand: ModuleInvocationBrandV1,
    callback: impl FnOnce(
        &mut super::common_v2_session::CommonV2CanonicalSessionRefV1<'_, '_>,
        &mut MirBuilder,
        &crate::mir::normal_callable_semantic_package::VerifiedS6CPhysicalFunctionEffectsV1,
        &crate::mir::normal_callable_semantic_package::S6CCommonV2PreSessionLoanRefV1<'_, '_, '_>,
    ) -> Result<R, String>,
) -> Result<R, String> {
    let invocation_brand = prepared.brand();
    if invocation_brand != expected_brand {
        return Err("common-V2 invocation brand mismatch".to_owned());
    }
    let mut prepared = prepared.into_payload();
    if builder.function_state.current_function.is_some()
        || builder.function_state.current_block.is_some()
    {
        return Err("physical entry session requires an empty Builder".to_owned());
    }

    let function_name = prepared.function_name().to_owned();

    prepared.with_admission(|prepared, admission, physical_effects, loan| {
        let source_input = admission.input();
        with_common_v2_canonical_session_branded(admission, invocation_brand, |mut common| {
            let (detached, descriptors, stamp) = prepared.take_install_parts();
            common.attach_physical_entry_stamp(stamp)?;
            let mut outer = builder.open_resolved_function_draft_seal_session_v1(&function_name);
            let result = (|| {
                let draft = outer.builder_view_mut_for_lowering();
                draft
                    .function_state
                    .resolved_binding_state
                    .install(source_input.function())?;
                draft.install_prepared_physical_function_skeleton(detached)?;
                common.adopt_physical_entry_lanes(draft, &descriptors)?;
                callback(&mut common, draft, physical_effects, loan)
            })();

            // The outer function transaction is the sole rollback owner.  It
            // restores the caller state and clears the unpublished shell even
            // when adoption or the callback rejects after partial mutation.
            outer.discard_unpublished();
            result
        })
    })?
}

fn with_common_v2_physical_entry_session_branded<R>(
    builder: &mut MirBuilder,
    prepared: InvocationBranded<PreparedPhysicalEntrySessionInputV1<'_, '_, '_>>,
    expected_brand: ModuleInvocationBrandV1,
    callback: impl FnOnce(
        &mut super::common_v2_session::CommonV2CanonicalSessionRefV1<'_, '_>,
        &mut MirBuilder,
    ) -> Result<R, String>,
) -> Result<R, String> {
    with_common_v2_physical_entry_session_branded_with_effects(
        builder,
        prepared,
        expected_brand,
        |common, builder, _physical_effects, _loan| callback(common, builder),
    )
}

#[cfg(not(test))]
pub(in crate::mir::builder) fn with_common_v2_physical_entry_session<R>(
    builder: &mut MirBuilder,
    prepared: InvocationBranded<PreparedPhysicalEntrySessionInputV1<'_, '_, '_>>,
    expected_brand: ModuleInvocationBrandV1,
    callback: impl FnOnce(
        &mut super::common_v2_session::CommonV2CanonicalSessionRefV1<'_, '_>,
        &mut MirBuilder,
    ) -> Result<R, String>,
) -> Result<R, String> {
    with_common_v2_physical_entry_session_branded(builder, prepared, expected_brand, callback)
}

#[cfg(test)]
pub(in crate::mir::builder) fn with_common_v2_physical_entry_session<R>(
    builder: &mut MirBuilder,
    prepared: InvocationBranded<PreparedPhysicalEntrySessionInputV1<'_, '_, '_>>,
    callback: impl FnOnce(
        &mut super::common_v2_session::CommonV2CanonicalSessionRefV1<'_, '_>,
        &mut MirBuilder,
    ) -> Result<R, String>,
) -> Result<R, String> {
    let expected_brand = prepared.brand();
    with_common_v2_physical_entry_session_branded(builder, prepared, expected_brand, callback)
}

#[cfg(test)]
pub(in crate::mir::builder) fn with_common_v2_physical_entry_session_expected_brand<R>(
    builder: &mut MirBuilder,
    prepared: InvocationBranded<PreparedPhysicalEntrySessionInputV1<'_, '_, '_>>,
    expected_brand: ModuleInvocationBrandV1,
    callback: impl FnOnce(
        &mut super::common_v2_session::CommonV2CanonicalSessionRefV1<'_, '_>,
        &mut MirBuilder,
    ) -> Result<R, String>,
) -> Result<R, String> {
    with_common_v2_physical_entry_session_branded(builder, prepared, expected_brand, callback)
}

#[cfg(test)]
pub(in crate::mir::builder) fn with_common_v2_physical_entry_session_with_s6c_effects<R>(
    builder: &mut MirBuilder,
    prepared: InvocationBranded<PreparedPhysicalEntrySessionInputV1<'_, '_, '_>>,
    callback: impl FnOnce(
        &mut super::common_v2_session::CommonV2CanonicalSessionRefV1<'_, '_>,
        &mut MirBuilder,
        &crate::mir::normal_callable_semantic_package::VerifiedS6CPhysicalFunctionEffectsV1,
    ) -> Result<R, String>,
) -> Result<R, String> {
    let expected_brand = prepared.brand();
    with_common_v2_physical_entry_session_branded_with_effects(
        builder,
        prepared,
        expected_brand,
        |common, builder, effects, _loan| callback(common, builder, effects),
    )
}

#[cfg(test)]
pub(in crate::mir::builder) fn with_common_v2_physical_entry_session_with_s6c_loan<R>(
    builder: &mut MirBuilder,
    prepared: InvocationBranded<PreparedPhysicalEntrySessionInputV1<'_, '_, '_>>,
    callback: impl FnOnce(
        &mut super::common_v2_session::CommonV2CanonicalSessionRefV1<'_, '_>,
        &mut MirBuilder,
        &crate::mir::normal_callable_semantic_package::S6CCommonV2PreSessionLoanRefV1<'_, '_, '_>,
    ) -> Result<R, String>,
) -> Result<R, String> {
    let expected_brand = prepared.brand();
    with_common_v2_physical_entry_session_branded_with_effects(
        builder,
        prepared,
        expected_brand,
        |common, builder, _effects, loan| callback(common, builder, loan),
    )
}
