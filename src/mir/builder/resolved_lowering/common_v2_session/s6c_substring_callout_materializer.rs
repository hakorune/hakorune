//! Canonical MIR lifecycle for the source-backed S6C Substring CallSlot.
//!
//! This child consumes the existing target/admission and operand receipts and
//! writes only through the canonical CFG/SSA owners.  It accepts no runtime
//! wire or lease token.  The caller's unpublished function transaction remains
//! the rollback boundary when the callback or a later terminal rejects.

use super::super::common_v2_segment_block_allocation::PreparedSegmentBlockReceiptV1;
use super::s6c_text_eq_occurrence::S6CTextEqOccurrencePhysicalViewV1;
use super::CommonV2CanonicalSessionRefV1;
use crate::mir::builder::MirBuilder;
use crate::mir::checked_callout::{CheckedCallOutNormalShapeV1, CheckedCallOutSiteIdV1};
use crate::mir::loop_recipe_contract::{LoopBlockKeyV1, LoopValueKeyV1};
use crate::mir::normal_callable_semantic_package::VerifiedS6CPhysicalFunctionEffectsV1;
use crate::mir::{MirType, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CommonV2SubstringCallOutMirMaterializerRejectV1 {
    AlreadyIssued,
    OwnerMismatch,
    ReceiverShapeMismatch,
    BodySegmentMismatch,
    SegmentScopeMismatch,
    OperandTypeMismatch,
    Occurrence(String),
    SitePlan(String),
    Block(String),
    CallOut(String),
    NormalResult(String),
    Fault(String),
    End(String),
    Callback(String),
}

/// Opaque, callback-scoped Normal result.  The physical ValueId and source V9
/// key stay paired with the admitted site and landing; no raw tuple escapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct CommonV2SubstringCallOutNormalResultRefV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    site: CheckedCallOutSiteIdV1,
    source_result: LoopValueKeyV1,
    normal_block: crate::mir::BasicBlockId,
    value: ValueId,
}

impl CommonV2SubstringCallOutNormalResultRefV1 {
    pub(in crate::mir::builder) const fn owner(
        self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn site(self) -> CheckedCallOutSiteIdV1 {
        self.site
    }

    pub(in crate::mir::builder) const fn source_result(self) -> LoopValueKeyV1 {
        self.source_result
    }

    pub(in crate::mir::builder) const fn normal_block(self) -> crate::mir::BasicBlockId {
        self.normal_block
    }

    pub(in crate::mir::builder) const fn value(self) -> ValueId {
        self.value
    }
}

/// One callback-scoped product that binds the canonical V9 NormalResult to
/// the already-issued source TextEq occurrence and ExactText entry sidecar.
/// The occurrence view contains no physical lane values or runtime wire.
#[derive(Debug)]
pub(in crate::mir::builder) struct CommonV2SubstringCallOutExactTextCoSealRefV1<'view> {
    normal: CommonV2SubstringCallOutNormalResultRefV1,
    occurrence: S6CTextEqOccurrencePhysicalViewV1<'view>,
}

impl CommonV2SubstringCallOutExactTextCoSealRefV1<'_> {
    pub(in crate::mir::builder) const fn normal_result(
        &self,
    ) -> CommonV2SubstringCallOutNormalResultRefV1 {
        self.normal
    }

    pub(in crate::mir::builder) const fn occurrence(
        &self,
    ) -> &S6CTextEqOccurrencePhysicalViewV1<'_> {
        &self.occurrence
    }
}

pub(super) fn emit<R>(
    session: &mut CommonV2CanonicalSessionRefV1<'_, '_>,
    builder: &mut MirBuilder,
    segment: &PreparedSegmentBlockReceiptV1,
    physical_effects: &VerifiedS6CPhysicalFunctionEffectsV1,
    body_key: LoopBlockKeyV1,
    body_block: crate::mir::BasicBlockId,
    receiver_key: LoopValueKeyV1,
    receiver: ValueId,
    index_key: LoopValueKeyV1,
    index: ValueId,
    end_key: LoopValueKeyV1,
    end_value: ValueId,
    source_result: LoopValueKeyV1,
    callback: impl for<'view> FnOnce(
        &mut MirBuilder,
        CommonV2SubstringCallOutExactTextCoSealRefV1<'view>,
    ) -> Result<R, String>,
) -> Result<R, CommonV2SubstringCallOutMirMaterializerRejectV1> {
    if session.s6c_substring_callout_mir_issued {
        return Err(CommonV2SubstringCallOutMirMaterializerRejectV1::AlreadyIssued);
    }

    let owner = session.owner();
    if physical_effects.owner() != owner
        || segment.owner() != owner
        || session.envelope.owner() != owner
    {
        return Err(CommonV2SubstringCallOutMirMaterializerRejectV1::OwnerMismatch);
    }
    if !session.session.owns_segment_receipt(segment) {
        return Err(CommonV2SubstringCallOutMirMaterializerRejectV1::SegmentScopeMismatch);
    }
    if receiver_key.raw() != 0
        || index_key.raw() != 6
        || end_key.raw() != 8
        || source_result.raw() != 9
    {
        return Err(CommonV2SubstringCallOutMirMaterializerRejectV1::ReceiverShapeMismatch);
    }
    let mut body_rows = segment
        .rows()
        .iter()
        .filter(|row| row.logical_block() == body_key);
    let Some(body_row) = body_rows.next() else {
        return Err(CommonV2SubstringCallOutMirMaterializerRejectV1::BodySegmentMismatch);
    };
    if body_rows.next().is_some() || body_row.physical_block() != body_block {
        return Err(CommonV2SubstringCallOutMirMaterializerRejectV1::BodySegmentMismatch);
    }

    for value in [receiver, index, end_value] {
        if builder.function_state.type_ctx.get_type(value) != Some(&MirType::Integer) {
            return Err(CommonV2SubstringCallOutMirMaterializerRejectV1::OperandTypeMismatch);
        }
    }

    session.s6c_substring_callout_mir_issued = true;
    session
        .with_s6c_text_eq_occurrence(segment, |session, occurrence| {
            validate_occurrence(&occurrence, owner, body_key, body_block, source_result)?;
            session
                .with_s6c_substring_callout_admission(physical_effects, |session, admission| {
                    admission.consume_for_canonical_materializer(|target, site_plan, end_ref| {
                        if target.owner() != owner
                            || target.block() != body_key
                            || target.result() != source_result
                            || end_ref.owner() != owner
                            || end_ref.result() != source_result
                        {
                            return Err("Substring target/End/source relation mismatch".to_owned());
                        }
                        let site = site_plan.site_id();
                        let site_shape = site_plan.normal_shape();
                        install_site_plan(session, builder, site_plan)?;

                        let normal = session
                            .session
                            .create_unpublished_block(builder)
                            .map_err(|error| format!("{error:?}"))?;
                        let fault = session
                            .session
                            .create_unpublished_block(builder)
                            .map_err(|error| format!("{error:?}"))?;
                        session
                            .session
                            .cfg
                            .emit_checked_callout(
                                builder
                                    .function_state
                                    .current_function
                                    .as_mut()
                                    .ok_or_else(|| "missing current function".to_owned())?,
                                body_block,
                                site,
                                receiver,
                                vec![index, end_value],
                                normal,
                                fault,
                            )
                            .map_err(|error| format!("{error:?}"))?;

                        select_block(session, builder, fault)
                            .map_err(|error| format!("{error:?}"))?;
                        session
                            .session
                            .cfg
                            .emit_checked_callout_fault(
                                builder
                                    .function_state
                                    .current_function
                                    .as_mut()
                                    .ok_or_else(|| "missing current function".to_owned())?,
                                fault,
                                site,
                            )
                            .map_err(|error| format!("{error:?}"))?;

                        select_block(session, builder, normal)
                            .map_err(|error| format!("{error:?}"))?;
                        let result_type =
                            result_type(site_shape).map_err(|error| format!("{error:?}"))?;
                        let projection = session
                            .session
                            .define_checked_callout_normal_result(
                                builder,
                                body_block,
                                normal,
                                site,
                                result_type,
                            )
                            .map_err(|error| format!("{error:?}"))?;
                        let result = CommonV2SubstringCallOutNormalResultRefV1 {
                            owner,
                            site,
                            source_result,
                            normal_block: normal,
                            value: projection.dst(),
                        };
                        let callback_result = callback(
                            builder,
                            CommonV2SubstringCallOutExactTextCoSealRefV1 {
                                normal: result,
                                occurrence,
                            },
                        )?;
                        let lease_slot = match site_shape {
                            CheckedCallOutNormalShapeV1::EndAuthorizedHandle { lease_slot } => {
                                lease_slot
                            }
                            CheckedCallOutNormalShapeV1::ImmediateI64 => {
                                return Err("Substring site lost EndAuthorized shape".to_owned())
                            }
                        };
                        session
                            .session
                            .emit_checked_callout_end(builder, normal, site, lease_slot)
                            .map_err(|error| format!("{error:?}"))?;
                        Ok(callback_result)
                    })
                })
                .map_err(|error| format!("{error:?}"))
        })
        .map_err(|error| match error {
            super::s6c_text_eq_occurrence::S6CTextEqOccurrenceViewRejectV1::Callback(detail) => {
                CommonV2SubstringCallOutMirMaterializerRejectV1::Callback(detail)
            }
            other => {
                CommonV2SubstringCallOutMirMaterializerRejectV1::Occurrence(format!("{other:?}"))
            }
        })
}

fn validate_occurrence(
    occurrence: &S6CTextEqOccurrencePhysicalViewV1<'_>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    body_key: LoopBlockKeyV1,
    body_block: crate::mir::BasicBlockId,
    source_result: LoopValueKeyV1,
) -> Result<(), String> {
    if occurrence.owner() != owner
        || occurrence.text_eq_block() != body_key
        || occurrence.physical_block() != body_block
        || occurrence.text_eq_left() != source_result
        || occurrence.if_condition() != occurrence.text_eq_result()
    {
        return Err("ExactText occurrence/V9 source relation mismatch".to_owned());
    }
    Ok(())
}

fn install_site_plan(
    session: &mut CommonV2CanonicalSessionRefV1<'_, '_>,
    builder: &mut MirBuilder,
    plan: crate::mir::checked_callout::CheckedCallOutSitePlanV1,
) -> Result<(), String> {
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| "checked callout site plan requires current function".to_owned())?;
    if session
        .session
        .physical_entry_stamp()
        .map_err(|error| error.to_string())?
        .owner()
        != session.owner()
    {
        return Err("checked callout physical entry owner mismatch".to_owned());
    }
    function
        .metadata
        .admit_checked_callout_plan(plan)
        .map_err(|error| format!("{error:?}"))
}

fn select_block(
    session: &mut CommonV2CanonicalSessionRefV1<'_, '_>,
    builder: &mut MirBuilder,
    block: crate::mir::BasicBlockId,
) -> Result<(), CommonV2SubstringCallOutMirMaterializerRejectV1> {
    session
        .session
        .cfg
        .select_block(builder, block)
        .map_err(|error| CommonV2SubstringCallOutMirMaterializerRejectV1::Block(error.to_string()))
}

fn result_type(
    shape: CheckedCallOutNormalShapeV1,
) -> Result<MirType, CommonV2SubstringCallOutMirMaterializerRejectV1> {
    match shape {
        CheckedCallOutNormalShapeV1::EndAuthorizedHandle { .. } => {
            crate::mir::route_value_type_publication::route_return_shape_value_type(Some(
                "string_handle",
            ))
            .ok_or_else(|| {
                CommonV2SubstringCallOutMirMaterializerRejectV1::NormalResult(
                    "missing string_handle MIR type".to_owned(),
                )
            })
        }
        CheckedCallOutNormalShapeV1::ImmediateI64 => Err(
            CommonV2SubstringCallOutMirMaterializerRejectV1::NormalResult(
                "Substring cannot use ImmediateI64".to_owned(),
            ),
        ),
    }
}
