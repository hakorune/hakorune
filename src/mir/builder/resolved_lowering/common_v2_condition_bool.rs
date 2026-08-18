//! Canonical physical materialization of the common V2 condition Bool.
//!
//! This child consumes the same-session Length receipt, reads the source
//! index binding through canonical identity/SSA, and emits one `Less` Compare.
//! It deliberately does not emit a branch, edge, terminator, or loop CFG.

use crate::mir::builder::emission::loop_operation;
use crate::mir::loop_recipe_contract::S6CScalarScanSourceRefV1;
use crate::mir::loop_recipe_contract::{LoopCompareI64OpV2, PreparedLoopV2ConditionOperandKindV1};
use crate::mir::{CompareOp, MirBuilder, MirType, ValueId};

use super::CanonicalLengthCallResultReceiptV1;
use super::CommonV2CanonicalSessionRefV1;
use super::{CommonV2ReturnReadPhysicalReceiptV1, ReturnReadPhysicalReceiptRejectV1};
use super::{CommonV2S6CCursorCfgReceiptV1, CommonV2S6CCursorCfgRejectV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ConditionBoolCursorCfgHandoffRejectV1 {
    OwnerMismatch,
    SegmentScopeMismatch,
    Bridge(String),
    Root(String),
    Cursor(String),
    Leaf(String),
    Materializer(CommonV2S6CCursorCfgRejectV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ConditionBoolMaterializationRejectV1 {
    AlreadyIssued,
    OwnerMismatch,
    ProducerMismatch,
    OperandInventoryMismatch,
    OperandRowsMissing,
    OperandRowShape,
    OperandType {
        left: Option<MirType>,
        right: Option<MirType>,
    },
    MissingLeftBinding,
    LeftRead(String),
    PhysicalValue(String),
    Compare(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ConditionBoolReturnReadRejectV1 {
    SegmentScopeMismatch,
    ConditionLogicalMismatch,
    ConditionPhysicalBlockMismatch,
    ReturnRead(ReturnReadPhysicalReceiptRejectV1),
}

/// One callback-scoped physical Bool result. The receipt owns the exclusive
/// canonical-session borrow so its operands cannot be detached and re-paired.
pub(in crate::mir::builder) struct CanonicalConditionBoolResultReceiptV1<
    'bool_result,
    'source,
    'envelope,
> {
    _session: &'bool_result mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    condition_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    producer_item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    logical_result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    physical_block: crate::mir::BasicBlockId,
    left: ValueId,
    right: ValueId,
    destination: ValueId,
    segment_brand: super::super::common_v2_segment_block_allocation::SegmentBlockAllocationBrandV1,
}

impl<'bool_result, 'source, 'envelope>
    CanonicalConditionBoolResultReceiptV1<'bool_result, 'source, 'envelope>
{
    pub(in crate::mir::builder) const fn owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn condition_block(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopBlockKeyV1 {
        self.condition_block
    }

    pub(in crate::mir::builder) const fn producer_item(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopItemKeyV1 {
        self.producer_item
    }

    pub(in crate::mir::builder) const fn logical_result(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.logical_result
    }

    pub(in crate::mir::builder) const fn physical_block(&self) -> crate::mir::BasicBlockId {
        self.physical_block
    }

    pub(in crate::mir::builder) const fn left(&self) -> ValueId {
        self.left
    }

    pub(in crate::mir::builder) const fn right(&self) -> ValueId {
        self.right
    }

    pub(in crate::mir::builder) const fn destination(&self) -> ValueId {
        self.destination
    }

    /// Consume the typed V5 condition receipt directly into the existing
    /// cursor CFG materializer.  The raw Bool `ValueId` never crosses this
    /// method boundary; source, entry bridge, cursor, leaf, and CFG remain
    /// one same-session handoff.
    pub(in crate::mir::builder::resolved_lowering) fn consume_s6c_cursor_cfg(
        self,
        builder: &mut MirBuilder,
        scope: &super::super::common_v2_segment_block_allocation::CommonV2SharedSegmentScopeV1,
        source: S6CScalarScanSourceRefV1<'_, '_, '_>,
    ) -> Result<
        CommonV2S6CCursorCfgReceiptV1<'bool_result, 'source, 'envelope>,
        ConditionBoolCursorCfgHandoffRejectV1,
    > {
        let CanonicalConditionBoolResultReceiptV1 {
            _session: session,
            owner,
            segment_brand,
            destination,
            ..
        } = self;
        if source.owner() != owner {
            return Err(ConditionBoolCursorCfgHandoffRejectV1::OwnerMismatch);
        }
        if !scope.receipt().belongs_to(&segment_brand)
            || !session.session.owns_segment_receipt(scope.receipt())
        {
            return Err(ConditionBoolCursorCfgHandoffRejectV1::SegmentScopeMismatch);
        }
        let bridge = session
            .session
            .issue_s6c_textref_entry_bridge_plan()
            .map_err(|error| ConditionBoolCursorCfgHandoffRejectV1::Bridge(format!("{error:?}")))?;
        let admission = crate::mir::builder::resolved_lowering::
            issue_common_v2_s6c_text_content_root_admission_v1(source, bridge)
            .map_err(|error| ConditionBoolCursorCfgHandoffRejectV1::Root(format!("{error:?}")))?;
        let cursor =
            crate::mir::builder::resolved_lowering::issue_common_v2_s6c_text_cursor_preheader_v1(
                admission,
            )
            .map_err(|error| ConditionBoolCursorCfgHandoffRejectV1::Cursor(format!("{error:?}")))?;
        let leaf = session
            .consume_s6c_scalar_equality_leaf(cursor)
            .map_err(|error| ConditionBoolCursorCfgHandoffRejectV1::Leaf(format!("{error:?}")))?;
        super::s6c_cursor_cfg::materialize_common_v2_s6c_cursor_cfg_v1(
            leaf,
            builder,
            scope,
            destination,
        )
        .map_err(ConditionBoolCursorCfgHandoffRejectV1::Materializer)
    }

    /// Consume this Bool receipt and the exact shared segment scope into the
    /// existing Return-read physical receipt. No branch or Return is written.
    pub(in crate::mir::builder) fn with_return_read_physical_receipt<R>(
        self,
        builder: &mut MirBuilder,
        scope: super::super::common_v2_segment_block_allocation::CommonV2SharedSegmentScopeV1,
        callback: impl FnOnce(
            &mut MirBuilder,
            CommonV2ReturnReadPhysicalReceiptV1<'bool_result, 'source, 'envelope>,
        ) -> Result<R, String>,
    ) -> Result<R, ConditionBoolReturnReadRejectV1> {
        let CanonicalConditionBoolResultReceiptV1 {
            _session: session,
            owner,
            condition_block,
            logical_result,
            physical_block,
            segment_brand,
            ..
        } = self;
        let segment_receipt = scope.receipt();
        if !segment_receipt.belongs_to(&segment_brand)
            || !session.session.owns_segment_receipt(segment_receipt)
        {
            return Err(ConditionBoolReturnReadRejectV1::SegmentScopeMismatch);
        }
        if session.envelope.owner() != owner
            || session.envelope.return_read_co_seal().if_condition() != logical_result
        {
            return Err(ConditionBoolReturnReadRejectV1::ConditionLogicalMismatch);
        }
        session
            .with_return_read_physical_receipt(builder, segment_receipt, |builder, receipt| {
                if receipt.owner() != owner
                    || receipt.if_block() != condition_block
                    || receipt.if_physical_block() != physical_block
                {
                    return Err("condition/Return-read physical block mismatch".to_owned());
                }
                callback(builder, receipt)
            })
            .map_err(ConditionBoolReturnReadRejectV1::ReturnRead)
    }
}

impl<'call, 'source, 'envelope> CanonicalLengthCallResultReceiptV1<'call, 'source, 'envelope> {
    /// Consume this Length result and emit the one canonical condition Bool.
    /// The Length receipt is the only recovery path to the mutable session;
    /// callers cannot pass a second session or detached operand ValueIds.
    pub(in crate::mir::builder) fn consume_for_condition_bool(
        self,
        builder: &mut MirBuilder,
    ) -> Result<
        CanonicalConditionBoolResultReceiptV1<'call, 'source, 'envelope>,
        ConditionBoolMaterializationRejectV1,
    > {
        let (session, owner, condition_block, physical_block, length_destination, segment_brand) =
            self.into_condition_bool_parts();

        if session.condition_bool_issued {
            return Err(ConditionBoolMaterializationRejectV1::AlreadyIssued);
        }
        let session_owner = session.session.owner();
        let (producer_condition_block, producer_item, producer_result, producer_right) = {
            let producer = session.envelope.condition_producer();
            (
                producer.condition_block(),
                producer.producer_item(),
                producer.result(),
                producer.right(),
            )
        };
        let inventory = session.envelope.condition_operands();
        if owner != session_owner
            || inventory.owner() != session_owner
            || session.envelope.owner() != session_owner
        {
            return Err(ConditionBoolMaterializationRejectV1::OwnerMismatch);
        }
        let producer = session.envelope.condition_producer();
        if producer_condition_block != condition_block
            || producer.op() != LoopCompareI64OpV2::Less
            || producer.class() != crate::mir::loop_recipe_contract::LoopValueClassV2::Bool
        {
            return Err(ConditionBoolMaterializationRejectV1::ProducerMismatch);
        }
        let binding = {
            let rows = inventory.rows();
            let (Some(left_row), Some(right_row)) = (rows.first(), rows.get(1)) else {
                return Err(ConditionBoolMaterializationRejectV1::OperandRowsMissing);
            };
            if left_row.block() != condition_block
                || right_row.block() != condition_block
                || left_row.class() != crate::mir::loop_recipe_contract::LoopValueClassV2::I64
                || right_row.class() != crate::mir::loop_recipe_contract::LoopValueClassV2::I64
                || right_row.value() != producer_right
            {
                return Err(ConditionBoolMaterializationRejectV1::OperandRowShape);
            }
            let binding = match left_row.kind() {
                PreparedLoopV2ConditionOperandKindV1::ReadBinding { binding } => binding,
                PreparedLoopV2ConditionOperandKindV1::LengthCall { .. } => {
                    return Err(ConditionBoolMaterializationRejectV1::MissingLeftBinding)
                }
            };
            if !matches!(
                right_row.kind(),
                PreparedLoopV2ConditionOperandKindV1::LengthCall { .. }
            ) {
                return Err(ConditionBoolMaterializationRejectV1::OperandRowShape);
            }
            binding
        };

        let entry_block = session
            .session
            .entry_block(builder)
            .map_err(ConditionBoolMaterializationRejectV1::LeftRead)?;
        let read = session
            .session
            .identity
            .read_entry_receipt(builder, &mut session.session.phis, entry_block, binding)
            .map_err(ConditionBoolMaterializationRejectV1::LeftRead)?;
        if read.owner() != session_owner
            || read.binding() != binding
            || read.physical_block() != entry_block
            || builder
                .function_state
                .type_ctx
                .get_type(read.physical_value())
                != Some(&MirType::Integer)
            || builder.function_state.type_ctx.get_type(length_destination)
                != Some(&MirType::Integer)
        {
            return Err(ConditionBoolMaterializationRejectV1::OperandType {
                left: builder
                    .function_state
                    .type_ctx
                    .get_type(read.physical_value())
                    .cloned(),
                right: builder
                    .function_state
                    .type_ctx
                    .get_type(length_destination)
                    .cloned(),
            });
        }

        session.condition_bool_issued = true;
        let destination = session
            .session
            .issue_physical_value_id(builder)
            .map_err(ConditionBoolMaterializationRejectV1::PhysicalValue)?;
        loop_operation::emit_compare_i64_at_with_dst(
            builder,
            physical_block,
            destination,
            CompareOp::Lt,
            read.physical_value(),
            length_destination,
        )
        .map_err(ConditionBoolMaterializationRejectV1::Compare)?;
        session
            .session
            .publish_physical_value_type(builder, destination, MirType::Bool)
            .map_err(ConditionBoolMaterializationRejectV1::PhysicalValue)?;

        Ok(CanonicalConditionBoolResultReceiptV1 {
            _session: session,
            owner,
            condition_block,
            producer_item,
            logical_result: producer_result,
            physical_block,
            left: read.physical_value(),
            right: length_destination,
            destination,
            segment_brand,
        })
    }
}
