//! Caller-zero S6C handoff from the common physical entry to DraftSeal.
//!
//! This probe is deliberately test-only. It exercises the existing
//! canonical finish/DraftSeal owner without selecting a production caller or
//! adding a Residence/lifecycle authority.

use crate::mir::builder::pinned_text_invocation_binding::PreparedPinnedTextPhysicalEntryIngressV1;
use crate::mir::builder::InvocationBranded;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::common_v2_physical_function_skeleton::PreparedPhysicalEntrySessionInputV1;
use crate::mir::normal_callable_semantic_package::S6CCommonV2PreSessionLoanRefV1;
use crate::mir::pinned_text_residence_lifecycle::PreparedPinnedTextResidenceLifecycleV1;
use crate::mir::MirFunction;

use super::common_v2_session::{
    with_common_v2_canonical_session_branded_finish, CommonV2CanonicalSessionRefV1,
};
use crate::mir::normal_callable_semantic_package::VerifiedS6CPhysicalFunctionEffectsV1;

/// Keep one physical-entry shell, canonical session, and DraftSeal owner in
/// the same unpublished transaction. The callback must finish the S6C cursor
/// at its After block; this wrapper then consumes the existing canonical
/// terminal and prepares exactly the source-authorized two exits.
pub(in crate::mir::builder) fn with_common_v2_s6c_physical_entry_draft_seal(
    builder: &mut MirBuilder,
    prepared: InvocationBranded<PreparedPhysicalEntrySessionInputV1<'_, '_, '_>>,
    callback: impl FnOnce(
        &mut CommonV2CanonicalSessionRefV1<'_, '_>,
        &mut MirBuilder,
        &VerifiedS6CPhysicalFunctionEffectsV1,
        &S6CCommonV2PreSessionLoanRefV1<'_, '_, '_>,
    ) -> Result<(), String>,
) -> Result<MirFunction, String> {
    let invocation_brand = prepared.brand();
    let mut prepared = prepared.into_payload();
    if builder.function_state.current_function.is_some()
        || builder.function_state.current_block.is_some()
    {
        return Err("physical entry session requires an empty Builder".to_owned());
    }
    let function_name = prepared.function_name().to_owned();

    prepared.with_admission(|prepared, admission, physical_effects, loan| {
        let source_input = admission.input();
        let tail_site = loan
            .callable()
            .with_completion(|completion| completion.tail_site().clone());
        let (detached, descriptors, stamp) = prepared.take_install_parts();
        let mut outer = Some(builder.open_resolved_function_draft_seal_session_v1(&function_name));
        let result = (|| {
            let ready = with_common_v2_canonical_session_branded_finish(
                admission,
                invocation_brand,
                outer
                    .as_mut()
                    .expect("S6C draft owner remains before canonical install")
                    .builder_view_mut_for_lowering(),
                |common, draft| {
                    common.attach_physical_entry_stamp(stamp)?;
                    draft
                        .function_state
                        .resolved_binding_state
                        .install(source_input.function())?;
                    draft.install_prepared_physical_function_skeleton(detached)?;
                    common.adopt_physical_entry_lanes(draft, &descriptors)?;
                    callback(common, draft, physical_effects, loan)?;
                    draft
                        .function_state
                        .current_block
                        .ok_or_else(|| "S6C cursor did not select an After block".to_owned())
                },
            )?;
            let open = ready.open(
                outer
                    .take()
                    .expect("S6C draft owner moves into DraftSeal exactly once"),
            );
            let prepared = match open.prepare_exact_two(&tail_site) {
                Ok(prepared) => prepared,
                Err(rejected) => {
                    let detail = format!("{:?}", rejected.error());
                    rejected.discard();
                    return Err(detail);
                }
            };
            Ok(prepared.commit().consume_non_authority_evidence())
        })();
        if let Some(outer) = outer {
            // The outer draft remains the sole rollback owner until DraftSeal
            // consumes it on the successful path above.
            outer.discard_unpublished();
        }
        result
    })
}

/// Caller-zero physical ingress probe.  Unlike the generic handoff above,
/// this path retains the session-owned target/frame provenance until the
/// canonical S6C plan table has been populated.  The final frame contract is
/// then issued exactly once from the current function's metadata immediately
/// before the existing DraftSeal owner opens.
pub(in crate::mir::builder) fn with_common_v2_s6c_pinned_text_physical_entry_draft_seal(
    builder: &mut MirBuilder,
    ingress: PreparedPinnedTextPhysicalEntryIngressV1<'_, '_, '_, '_>,
    callback: impl FnOnce(
        &mut CommonV2CanonicalSessionRefV1<'_, '_>,
        &mut MirBuilder,
        &VerifiedS6CPhysicalFunctionEffectsV1,
        &S6CCommonV2PreSessionLoanRefV1<'_, '_, '_>,
    ) -> Result<(), String>,
) -> Result<MirFunction, String> {
    ingress.consume_for_draft_seal(|prepared, binding, frame_ingress| {
        let invocation_brand = prepared.brand();
        let mut prepared = prepared.into_payload();
        if builder.function_state.current_function.is_some()
            || builder.function_state.current_block.is_some()
        {
            return Err("physical entry session requires an empty Builder".to_owned());
        }
        let function_name = prepared.function_name().to_owned();

        prepared.with_admission(|prepared, admission, physical_effects, loan| {
            let source_input = admission.input();
            let tail_site = loan
                .callable()
                .with_completion(|completion| completion.tail_site().clone());
            let (detached, descriptors, stamp) = prepared.take_install_parts();
            let mut outer =
                Some(builder.open_resolved_function_draft_seal_session_v1(&function_name));
            let result = (|| {
                let mut finish_capability = None;
                let ready = with_common_v2_canonical_session_branded_finish(
                    admission,
                    invocation_brand,
                    outer
                        .as_mut()
                        .expect("S6C draft owner remains before canonical install")
                        .builder_view_mut_for_lowering(),
                    |common, draft| {
                        common.attach_physical_entry_stamp(stamp)?;
                        draft
                            .function_state
                            .resolved_binding_state
                            .install(source_input.function())?;
                        draft.install_prepared_physical_function_skeleton(detached)?;
                        common.adopt_physical_entry_lanes(draft, &descriptors)?;
                        let function_entry = draft
                            .function_state
                            .current_function
                            .as_ref()
                            .expect("canonical function remains after skeleton install")
                            .entry_block;
                        let execution_entry =
                            common.issue_physical_entry_execution_boundary(draft)?;
                        let trap_block = common.create_unpublished_block(draft)?;
                        common.defer_physical_entry_seal()?;
                        common.select_block(draft, execution_entry)?;
                        callback(common, draft, physical_effects, loan)?;
                        let after_block = draft
                            .function_state
                            .current_block
                            .ok_or_else(|| "S6C cursor did not select an After block".to_owned())?;
                        let frame = binding
                            .finalize_backend_frame(frame_ingress, loan, draft)
                            .map_err(|error| format!("{error:?}"))?;
                        draft
                            .function_state
                            .current_function
                            .as_mut()
                            .expect("canonical function remains before Residence Enter")
                            .metadata
                            .pinned_text_backend_frame_contract = Some(frame);
                        let carrier = {
                            let function = draft
                                .function_state
                                .current_function
                                .as_ref()
                                .expect("canonical function remains for Residence carrier");
                            let frame = function
                                .metadata
                                .pinned_text_backend_frame_contract
                                .as_ref()
                                .ok_or_else(|| "pinned-Text frame was not installed".to_owned())?;
                            PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(
                                common.owner(),
                                &function.metadata.pinned_text_access_plans,
                                frame.borrow(),
                                execution_entry,
                                trap_block,
                            )
                            .map_err(|error| format!("{error:?}"))?
                        };
                        common.select_block(draft, function_entry)?;
                        finish_capability =
                            Some(common.emit_pinned_text_residence_enter(draft, carrier)?);
                        common.select_block(draft, after_block)?;
                        common.seal_deferred_physical_entry(draft)?;
                        common.seal_deferred_s6c_cursor_blocks(draft)?;
                        Ok(after_block)
                    },
                )?;
                let finish = finish_capability
                    .take()
                    .ok_or_else(|| "Residence Enter did not issue Finish capability".to_owned())?;

                let open = ready.open(
                    outer
                        .take()
                        .expect("S6C draft owner moves into DraftSeal exactly once"),
                );
                let prepared = match open.prepare_exact_two_with_pinned_text(&tail_site, finish) {
                    Ok(prepared) => prepared,
                    Err(rejected) => {
                        let detail = format!("{:?}", rejected.error());
                        rejected.discard();
                        return Err(detail);
                    }
                };
                Ok(prepared.commit().consume_non_authority_evidence())
            })();
            if let Some(outer) = outer {
                outer.discard_unpublished();
            }
            result
        })
    })
}
