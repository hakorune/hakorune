//! Canonical S6C cursor CFG/SSA materializer.
//!
//! This is the only effect-bearing consumer of the scalar-equality leaf.  It
//! reuses the existing segment allocation and Return-read co-seal, then asks
//! the canonical CFG/PHI/session owners to install one cursor loop.  V5 (the
//! outer length predicate) and V10 (the inner TextEq Bool) remain distinct.

use crate::mir::builder::emission::phi_lifecycle::PhiToken;
use crate::mir::builder::emission::{constant, loop_operation};
use crate::mir::builder::resolved_lowering::common_v2_segment_block_allocation::{
    CommonV2SharedSegmentScopeV1, PreparedSegmentBlockReceiptV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::S6CPrephysicalCompletionRefV2;
use crate::mir::pinned_text_access_plan::{PinnedTextAccessKindV1, PinnedTextRootIdV1};
use crate::mir::resolved_semantics::ResolvedExitSiteV1;
use crate::mir::{BasicBlockId, MirInstruction, MirType, ValueId};

use super::s6c_scalar_equality_leaf::{
    CommonV2S6CTextScalarEqualityLeafCapabilityV1, CommonV2S6CTextScalarEqualityLeafReceiptV1,
    CommonV2S6CTextScalarEqualityLeafShapeV1,
};
use super::CommonV2CanonicalSessionRefV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CommonV2S6CCursorCfgRejectV1 {
    AlreadyIssued,
    OwnerMismatch,
    MissingPhysicalEntryStamp,
    EntryMismatch,
    LeafShapeMismatch,
    SegmentOwnerMismatch,
    SegmentRow(String),
    After(String),
    Plan(String),
    Value(String),
    Phi(String),
    Edge(String),
    ReturnRead(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReturnPlacement {
    then_block: BasicBlockId,
    continuation_block: BasicBlockId,
}

/// Callback-scoped proof that the canonical cursor loop was installed.  The
/// session borrow keeps the PHIs, leaf, and Return-read relation paired until
/// the outer unpublished function transaction discards or advances the row.
pub(in crate::mir::builder) struct CommonV2S6CCursorCfgReceiptV1<'session, 'source, 'envelope> {
    _session: &'session mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    condition_block: BasicBlockId,
    body_block: BasicBlockId,
    after_block: BasicBlockId,
    subject_byte_len: ValueId,
    byte_offset_phi: ValueId,
    loop_condition: ValueId,
    width_value: ValueId,
    text_equal_value: ValueId,
    byte_next: ValueId,
    return_block: BasicBlockId,
    continuation_block: BasicBlockId,
}

impl CommonV2S6CCursorCfgReceiptV1<'_, '_, '_> {
    pub(in crate::mir::builder) const fn owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn condition_block(&self) -> BasicBlockId {
        self.condition_block
    }

    pub(in crate::mir::builder) const fn body_block(&self) -> BasicBlockId {
        self.body_block
    }

    pub(in crate::mir::builder) const fn after_block(&self) -> BasicBlockId {
        self.after_block
    }

    pub(in crate::mir::builder) const fn byte_offset_phi(&self) -> ValueId {
        self.byte_offset_phi
    }

    pub(in crate::mir::builder) const fn subject_byte_len(&self) -> ValueId {
        self.subject_byte_len
    }

    pub(in crate::mir::builder) const fn loop_condition(&self) -> ValueId {
        self.loop_condition
    }

    pub(in crate::mir::builder) const fn width_value(&self) -> ValueId {
        self.width_value
    }

    pub(in crate::mir::builder) const fn text_equal_value(&self) -> ValueId {
        self.text_equal_value
    }

    pub(in crate::mir::builder) const fn byte_next(&self) -> ValueId {
        self.byte_next
    }

    pub(in crate::mir::builder) const fn return_block(&self) -> BasicBlockId {
        self.return_block
    }

    pub(in crate::mir::builder) const fn continuation_block(&self) -> BasicBlockId {
        self.continuation_block
    }
}

fn unique_segment_row(
    receipt: &PreparedSegmentBlockReceiptV1,
    logical_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
) -> Result<crate::mir::builder::resolved_lowering::common_v2_segment_block_allocation::SegmentBlockReceiptRowV1, CommonV2S6CCursorCfgRejectV1>{
    let mut rows = receipt
        .rows()
        .iter()
        .filter(|row| row.logical_block() == logical_block);
    let row = rows
        .next()
        .copied()
        .ok_or_else(|| CommonV2S6CCursorCfgRejectV1::SegmentRow("row missing".to_owned()))?;
    if rows.next().is_some() {
        return Err(CommonV2S6CCursorCfgRejectV1::SegmentRow(
            "row duplicated".to_owned(),
        ));
    }
    Ok(row)
}

fn emit_branch(
    session: &mut CommonV2CanonicalSessionRefV1<'_, '_>,
    builder: &mut MirBuilder,
    source: BasicBlockId,
    condition: ValueId,
    then_block: BasicBlockId,
    else_block: BasicBlockId,
) -> Result<(), CommonV2S6CCursorCfgRejectV1> {
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| CommonV2S6CCursorCfgRejectV1::Edge("function missing".to_owned()))?;
    session
        .session
        .cfg
        .emit_branch(function, source, condition, then_block, else_block)
        .map_err(|error| CommonV2S6CCursorCfgRejectV1::Edge(error.to_string()))
}

fn emit_jump(
    session: &mut CommonV2CanonicalSessionRefV1<'_, '_>,
    builder: &mut MirBuilder,
    source: BasicBlockId,
    target: BasicBlockId,
) -> Result<(), CommonV2S6CCursorCfgRejectV1> {
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| CommonV2S6CCursorCfgRejectV1::Edge("function missing".to_owned()))?;
    session
        .session
        .cfg
        .emit_jump(function, source, target)
        .map_err(|error| CommonV2S6CCursorCfgRejectV1::Edge(error.to_string()))
}

fn issue_i64(
    session: &mut CommonV2CanonicalSessionRefV1<'_, '_>,
    builder: &mut MirBuilder,
    block: BasicBlockId,
    value: i64,
) -> Result<ValueId, CommonV2S6CCursorCfgRejectV1> {
    let dst = session
        .session
        .issue_physical_value_id(builder)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    constant::emit_integer_at_with_dst(builder, block, dst, value)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    session
        .session
        .publish_physical_value_type(builder, dst, MirType::Integer)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    Ok(dst)
}

fn issue_pinned_text(
    session: &mut CommonV2CanonicalSessionRefV1<'_, '_>,
    builder: &mut MirBuilder,
    block: BasicBlockId,
    stamp: u64,
    kind: PinnedTextAccessKindV1,
    result_type: MirType,
) -> Result<ValueId, CommonV2S6CCursorCfgRejectV1> {
    let plan = session
        .session
        .issue_pinned_text_plan(builder, stamp, kind)
        .map_err(CommonV2S6CCursorCfgRejectV1::Plan)?;
    let dst = session
        .session
        .issue_physical_value_id(builder)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    session
        .session
        .publish_physical_value_type(builder, dst, result_type)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    builder
        .emit_instruction_at(block, MirInstruction::PinnedTextOp { dst, plan, kind })
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    Ok(dst)
}

fn materialize_inner<'session, 'source, 'envelope>(
    session: &'session mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    capability: &CommonV2S6CTextScalarEqualityLeafCapabilityV1,
    builder: &mut MirBuilder,
    scope: &CommonV2SharedSegmentScopeV1,
    source: crate::mir::loop_recipe_contract::S6CScalarScanSourceRefV1<'_, '_, '_>,
    completion: S6CPrephysicalCompletionRefV2<'_>,
) -> Result<CommonV2S6CCursorCfgReceiptV1<'session, 'source, 'envelope>, CommonV2S6CCursorCfgRejectV1>
{
    if session.s6c_cursor_cfg_issued {
        return Err(CommonV2S6CCursorCfgRejectV1::AlreadyIssued);
    }
    let owner = session.session.owner();
    if capability.owner() != owner || session.envelope.owner() != owner {
        return Err(CommonV2S6CCursorCfgRejectV1::OwnerMismatch);
    }
    let sidecar_entry = session
        .session
        .physical_entry_sidecar_entry()
        .map_err(|_| CommonV2S6CCursorCfgRejectV1::MissingPhysicalEntryStamp)?;
    if capability.entry() != sidecar_entry {
        return Err(CommonV2S6CCursorCfgRejectV1::EntryMismatch);
    }
    let execution_entry = session
        .session
        .physical_execution_entry(builder)
        .map_err(|_| CommonV2S6CCursorCfgRejectV1::MissingPhysicalEntryStamp)?;
    if scope.receipt().owner() != owner {
        return Err(CommonV2S6CCursorCfgRejectV1::SegmentOwnerMismatch);
    }
    let [byte_len_shape, width_shape, eq_shape] = *capability.shapes();
    if byte_len_shape != (CommonV2S6CTextScalarEqualityLeafShapeV1::ByteLen { root_index: 0 })
        || width_shape != (CommonV2S6CTextScalarEqualityLeafShapeV1::Utf8WidthAt { root_index: 0 })
        || eq_shape
            != (CommonV2S6CTextScalarEqualityLeafShapeV1::Utf8ScalarSliceEqWholeText {
                lhs_root_index: 0,
                rhs_root_index: 1,
            })
    {
        return Err(CommonV2S6CCursorCfgRejectV1::LeafShapeMismatch);
    }

    // Poison the consumer before any physical mutation.  The outer draft
    // transaction owns all cleanup when a later canonical writer rejects.
    session.s6c_cursor_cfg_issued = true;
    let relation = capability.relation();
    let producer_block = session.envelope.condition_producer().condition_block();
    let co_seal = session.envelope.return_read_co_seal();
    if relation.text_equal_if() != co_seal.if_item()
        || relation.text_equal_result() != co_seal.if_condition()
    {
        return Err(CommonV2S6CCursorCfgRejectV1::ReturnRead(
            "TextEq item/result differs from the existing Return-read If co-seal".to_owned(),
        ));
    }
    let condition_row = unique_segment_row(scope.receipt(), producer_block)?;
    let body_row = unique_segment_row(scope.receipt(), co_seal.if_block())?;
    if condition_row.loop_key() != body_row.loop_key() {
        return Err(CommonV2S6CCursorCfgRejectV1::SegmentRow(
            "condition/body loop key differs".to_owned(),
        ));
    }

    let after = session
        .allocate_v2_after_block(builder, scope.receipt())
        .map_err(|error| CommonV2S6CCursorCfgRejectV1::After(format!("{error:?}")))?;
    let after_block = after.physical_block();
    drop(after);

    let entry_byte = issue_i64(
        session,
        builder,
        execution_entry,
        capability.initial().byte_offset() as i64,
    )?;
    let subject_byte_len = issue_pinned_text(
        session,
        builder,
        execution_entry,
        capability.root_plan_stamp(),
        PinnedTextAccessKindV1::ByteLen {
            root: PinnedTextRootIdV1::from_frame_row(capability.subject_root_index()),
        },
        MirType::Integer,
    )?;
    let byte_phi = session
        .session
        .issue_physical_value_id(builder)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    session
        .session
        .publish_physical_value_type(builder, byte_phi, MirType::Integer)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    let byte_token: PhiToken = session
        .session
        .phis
        .define_provisional_phi(
            builder,
            condition_row.physical_block(),
            byte_phi,
            "s6c:byte",
        )
        .map_err(CommonV2S6CCursorCfgRejectV1::Phi)?;
    let loop_condition = session
        .session
        .issue_physical_value_id(builder)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    loop_operation::emit_compare_i64_at_with_dst(
        builder,
        condition_row.physical_block(),
        loop_condition,
        crate::mir::CompareOp::Lt,
        byte_phi,
        subject_byte_len,
    )
    .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    session
        .session
        .publish_physical_value_type(builder, loop_condition, MirType::Bool)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;

    let width = issue_pinned_text(
        session,
        builder,
        body_row.physical_block(),
        capability.root_plan_stamp(),
        PinnedTextAccessKindV1::Utf8WidthAt {
            root: PinnedTextRootIdV1::from_frame_row(capability.subject_root_index()),
            byte_offset: byte_phi,
        },
        MirType::Integer,
    )?;
    let text_equal = issue_pinned_text(
        session,
        builder,
        body_row.physical_block(),
        capability.root_plan_stamp(),
        PinnedTextAccessKindV1::Utf8ScalarSliceEqWholeText {
            lhs_root: PinnedTextRootIdV1::from_frame_row(capability.subject_root_index()),
            lhs_byte_offset: byte_phi,
            lhs_width: width,
            rhs_root: PinnedTextRootIdV1::from_frame_row(capability.needle_root_index()),
        },
        MirType::Bool,
    )?;
    let return_placement = session
        .with_return_read_physical_receipt(builder, scope.receipt(), |_, receipt| {
            if receipt.if_physical_block() != body_row.physical_block()
                || receipt.if_condition() != relation.text_equal_result()
            {
                return Err("Return-read physical If differs from TextEq placement".to_owned());
            }
            Ok(ReturnPlacement {
                then_block: receipt.then_physical_block(),
                continuation_block: receipt.continuation_physical_block(),
            })
        })
        .map_err(|error| CommonV2S6CCursorCfgRejectV1::ReturnRead(format!("{error:?}")))?;

    let byte_next = session
        .session
        .issue_physical_value_id(builder)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    // Establish the complete cursor reachability skeleton before asking the
    // canonical Binding SSA reader for the loop-carried index.  The physical
    // entry may still be deferred to the later Residence Enter, but its
    // cursor successors are already canonical CFG facts in this session.
    emit_jump(
        session,
        builder,
        execution_entry,
        condition_row.physical_block(),
    )?;
    emit_branch(
        session,
        builder,
        condition_row.physical_block(),
        loop_condition,
        body_row.physical_block(),
        after_block,
    )?;
    emit_branch(
        session,
        builder,
        body_row.physical_block(),
        text_equal,
        return_placement.then_block,
        return_placement.continuation_block,
    )?;
    emit_jump(
        session,
        builder,
        return_placement.continuation_block,
        condition_row.physical_block(),
    )?;
    session
        .session
        .identity
        .claim_variable_use_binding(source.condition_index_site(), source.index_binding())
        .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;
    session
        .session
        .identity
        .claim_variable_use_binding(source.length().receiver_site(), source.subject_binding())
        .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;
    session
        .session
        .identity
        .claim_variable_use_binding(source.substring().receiver_site(), source.subject_binding())
        .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;
    let substring_index_site = source
        .substring()
        .arguments()
        .first()
        .map(|argument| argument.site())
        .ok_or_else(|| {
            CommonV2S6CCursorCfgRejectV1::ReturnRead(
                "S6C Substring index argument is missing".to_owned(),
            )
        })?;
    session
        .session
        .identity
        .claim_variable_use_binding(substring_index_site, source.index_binding())
        .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;
    session
        .session
        .identity
        .claim_variable_use_binding(source.slice_end_index_site(), source.index_binding())
        .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;
    session
        .session
        .identity
        .claim_variable_use_binding(source.text_equal_rhs_site(), source.needle_binding())
        .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;
    session
        .session
        .identity
        .claim_variable_use_binding(source.step_read_site(), source.index_binding())
        .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;
    let current_i = if session.session.physical_entry_seal_deferred() {
        session
            .session
            .identity
            .read_entry_receipt_with_deferred_entry(
                builder,
                &mut session.session.phis,
                return_placement.continuation_block,
                source.index_binding(),
                execution_entry,
            )
    } else {
        session.session.identity.read_entry_receipt(
            builder,
            &mut session.session.phis,
            return_placement.continuation_block,
            source.index_binding(),
        )
    }
    .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;
    match builder
        .function_state
        .type_ctx
        .get_type(current_i.physical_value())
        .cloned()
    {
        None | Some(MirType::Unknown) => session
            .session
            .publish_physical_value_type(builder, current_i.physical_value(), MirType::Integer)
            .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?,
        Some(MirType::Integer) => {}
        Some(other) => {
            return Err(CommonV2S6CCursorCfgRejectV1::ReturnRead(format!(
                "canonical index read is not i64: {other:?}"
            )))
        }
    }
    let index_next = session
        .session
        .issue_physical_value_id(builder)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    let one = issue_i64(session, builder, return_placement.continuation_block, 1)?;
    loop_operation::emit_add_i64_at_with_dst(
        builder,
        return_placement.continuation_block,
        byte_next,
        byte_phi,
        width,
    )
    .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    loop_operation::emit_add_i64_at_with_dst(
        builder,
        return_placement.continuation_block,
        index_next,
        current_i.physical_value(),
        one,
    )
    .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    session
        .session
        .publish_physical_value_type(builder, byte_next, MirType::Integer)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    session
        .session
        .publish_physical_value_type(builder, index_next, MirType::Integer)
        .map_err(CommonV2S6CCursorCfgRejectV1::Value)?;
    session
        .session
        .identity
        .define_assignment_exact(
            source.index_update().target_site(),
            source.index_binding(),
            return_placement.continuation_block,
            index_next,
        )
        .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;

    session
        .session
        .phis
        .patch_phi_inputs(
            builder,
            byte_token,
            vec![
                (execution_entry, entry_byte),
                (return_placement.continuation_block, byte_next),
            ],
            "s6c:byte",
        )
        .map_err(CommonV2S6CCursorCfgRejectV1::Phi)?;
    let tail_value = issue_i64(session, builder, after_block, -1)?;
    session
        .session
        .completion
        .claim_explicit_return(
            completion.tail_site(),
            completion.completion().target_function(),
            after_block,
            tail_value,
        )
        .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;
    session
        .session
        .identity
        .mark_return(ResolvedExitSiteV1::Statement(
            completion.tail_site().clone(),
        ))
        .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;

    if session.session.physical_entry_seal_deferred() {
        session
            .session
            .defer_s6c_cursor_seals([
                body_row.physical_block(),
                return_placement.continuation_block,
                condition_row.physical_block(),
                return_placement.then_block,
                after_block,
            ])
            .map_err(CommonV2S6CCursorCfgRejectV1::Edge)?;
    } else {
        let (
            entry_witness,
            body_witness,
            continuation_witness,
            condition_witness,
            then_witness,
            after_witness,
        ) = {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| CommonV2S6CCursorCfgRejectV1::Edge("function missing".to_owned()))?;
            let entry_witness = session
                .session
                .cfg
                .seal_block(function, execution_entry)
                .map_err(|error| CommonV2S6CCursorCfgRejectV1::Edge(error.to_string()))?;
            let body_witness = session
                .session
                .cfg
                .seal_block(function, body_row.physical_block())
                .map_err(|error| CommonV2S6CCursorCfgRejectV1::Edge(error.to_string()))?;
            let continuation_witness = session
                .session
                .cfg
                .seal_block(function, return_placement.continuation_block)
                .map_err(|error| CommonV2S6CCursorCfgRejectV1::Edge(error.to_string()))?;
            let condition_witness = session
                .session
                .cfg
                .seal_block(function, condition_row.physical_block())
                .map_err(|error| CommonV2S6CCursorCfgRejectV1::Edge(error.to_string()))?;
            let then_witness = session
                .session
                .cfg
                .seal_block(function, return_placement.then_block)
                .map_err(|error| CommonV2S6CCursorCfgRejectV1::Edge(error.to_string()))?;
            let after_witness = session
                .session
                .cfg
                .seal_block(function, after_block)
                .map_err(|error| CommonV2S6CCursorCfgRejectV1::Edge(error.to_string()))?;
            (
                entry_witness,
                body_witness,
                continuation_witness,
                condition_witness,
                then_witness,
                after_witness,
            )
        };
        for (block, witness) in [
            (execution_entry, entry_witness),
            (return_placement.then_block, then_witness),
            (after_block, after_witness),
            (body_row.physical_block(), body_witness),
            (return_placement.continuation_block, continuation_witness),
            (condition_row.physical_block(), condition_witness),
        ] {
            session
                .session
                .identity
                .seal_block(builder, &mut session.session.phis, block, &witness)
                .map_err(CommonV2S6CCursorCfgRejectV1::ReturnRead)?;
        }
    }
    session
        .session
        .cfg
        .select_block(builder, after_block)
        .map_err(|error| CommonV2S6CCursorCfgRejectV1::Edge(error.to_string()))?;

    Ok(CommonV2S6CCursorCfgReceiptV1 {
        _session: session,
        owner,
        condition_block: condition_row.physical_block(),
        body_block: body_row.physical_block(),
        after_block,
        subject_byte_len,
        byte_offset_phi: byte_phi,
        loop_condition,
        width_value: width,
        text_equal_value: text_equal,
        byte_next,
        return_block: return_placement.then_block,
        continuation_block: return_placement.continuation_block,
    })
}

/// Internal materializer used only by the same-session scalar-scan consumer.
/// V5 is issued here from the pinned subject byte length; no generic runtime
/// Length call or detached Bool `ValueId` is accepted.
pub(super) fn materialize_common_v2_s6c_cursor_cfg_v1<'session, 'source, 'envelope>(
    leaf: CommonV2S6CTextScalarEqualityLeafReceiptV1<'session, 'source, 'envelope>,
    builder: &mut MirBuilder,
    scope: &CommonV2SharedSegmentScopeV1,
    source: crate::mir::loop_recipe_contract::S6CScalarScanSourceRefV1<'_, '_, '_>,
    completion: S6CPrephysicalCompletionRefV2<'_>,
) -> Result<CommonV2S6CCursorCfgReceiptV1<'session, 'source, 'envelope>, CommonV2S6CCursorCfgRejectV1>
{
    leaf.with_session(|session, capability| {
        materialize_inner(session, capability, builder, scope, source, completion)
    })
}
