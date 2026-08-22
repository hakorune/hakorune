//! Source-backed operand inventory for the common V2 condition producer.
//!
//! The inventory is deliberately physical-ID-free.  It records only the two
//! already-sealed logical producers needed by the S6C condition: the index
//! binding read and the length call result.  No ValueId, session, Builder, or
//! physical result is issued here.

use super::common_v2_condition_producer::PreparedLoopV2ConditionProducerRelationV1;
use super::common_v2_issuers::PreparedLoopOperationProgramV2;
use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use super::s6c_prephysical_ingress::S6CPrephysicalIngressRefV2;
use super::s6c_scan_with_init_joinir::S6CLogicalCallInputRefV1;
use super::s6c_scan_with_init_joinir_output_rows::{S6CLogicalCallArgsV1, S6CLogicalItemV1};
use super::schema_v2::{LoopOperationV2, LoopValueClassV2};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, ResolvedLoopPlacementV1};

#[derive(Debug, Clone, Copy)]
pub(crate) enum PreparedLoopV2ConditionOperandKindV1<'facts> {
    ReadBinding {
        binding: BindingRefV1,
    },
    LengthCall {
        source: S6CLogicalCallInputRefV1<'facts>,
    },
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PreparedLoopV2ConditionOperandRowV1<'facts> {
    value: LoopValueKeyV1,
    item: LoopItemKeyV1,
    block: LoopBlockKeyV1,
    class: LoopValueClassV2,
    kind: PreparedLoopV2ConditionOperandKindV1<'facts>,
}

impl<'facts> PreparedLoopV2ConditionOperandRowV1<'facts> {
    pub(crate) const fn value(self) -> LoopValueKeyV1 {
        self.value
    }

    pub(crate) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) const fn block(self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(crate) const fn class(self) -> LoopValueClassV2 {
        self.class
    }

    pub(crate) const fn kind(self) -> PreparedLoopV2ConditionOperandKindV1<'facts> {
        self.kind
    }
}

#[derive(Debug)]
pub(crate) struct PreparedLoopV2ConditionOperandInventoryV1<'facts> {
    owner: FunctionOwnerIdV1,
    condition_block: LoopBlockKeyV1,
    rows: [PreparedLoopV2ConditionOperandRowV1<'facts>; 2],
}

impl PreparedLoopV2ConditionOperandInventoryV1<'_> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn condition_block(&self) -> LoopBlockKeyV1 {
        self.condition_block
    }

    pub(crate) const fn rows(&self) -> &[PreparedLoopV2ConditionOperandRowV1<'_>; 2] {
        &self.rows
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConditionOperandInventoryRejectV1 {
    ForeignOwner,
    MissingLeft,
    DuplicateLeft,
    LeftShape,
    LeftOperationMissing,
    LeftOperationDuplicate,
    LeftOperationMismatch,
    MissingRight,
    DuplicateRight,
    RightShape,
    LengthSourceShape,
    RightOperationMissing,
    RightOperationDuplicate,
    RightOperationMismatch,
}

pub(crate) fn issue_s6c_v2_condition_operand_inventory_v1<'source, 'facts>(
    ingress: S6CPrephysicalIngressRefV2<'_, 'source, 'facts>,
    operations: &PreparedLoopOperationProgramV2<'source>,
    producer: &PreparedLoopV2ConditionProducerRelationV1,
    expected_owner: FunctionOwnerIdV1,
) -> Result<PreparedLoopV2ConditionOperandInventoryV1<'facts>, ConditionOperandInventoryRejectV1> {
    if ingress.source_owner() != expected_owner
        || operations.owner() != expected_owner
        || producer.owner() != expected_owner
    {
        return Err(ConditionOperandInventoryRejectV1::ForeignOwner);
    }

    let index_binding = ingress.input_bindings()[2];
    let index_binding_key = ingress.source.logical().roles().index_binding();
    let mut left = None;
    for item in ingress.logical_items() {
        let S6CLogicalItemV1::ReadBinding {
            item,
            block,
            binding,
            result,
        } = *item
        else {
            continue;
        };
        if result != producer.left() {
            continue;
        }
        if left.is_some() {
            return Err(ConditionOperandInventoryRejectV1::DuplicateLeft);
        }
        left = Some((item, block, binding, result));
    }
    let (left_item, left_block, left_binding, left_value) =
        left.ok_or(ConditionOperandInventoryRejectV1::MissingLeft)?;
    if left_block != producer.condition_block()
        || left_binding != index_binding_key
        || left_value != producer.left()
    {
        return Err(ConditionOperandInventoryRejectV1::LeftShape);
    }
    let mut left_operation = None;
    for row in operations.rows() {
        if row.item() != left_item {
            continue;
        }
        if left_operation.is_some() {
            return Err(ConditionOperandInventoryRejectV1::LeftOperationDuplicate);
        }
        left_operation = Some(row);
    }
    let left_operation =
        left_operation.ok_or(ConditionOperandInventoryRejectV1::LeftOperationMissing)?;
    if left_operation.block() != left_block
        || !matches!(
            left_operation.operation(),
            LoopOperationV2::ReadBinding { binding, result }
                if *binding == index_binding_key && *result == left_value
        )
    {
        return Err(ConditionOperandInventoryRejectV1::LeftOperationMismatch);
    }

    let length = ingress.source.logical().calls().length();
    let length_source = length.source();
    let length_row = length.row();
    if length_source.owner() != expected_owner
        || length_source.role() != super::s6c_scan_with_init_joinir::S6CLogicalCallRoleV1::Length
        || length_source.operation() != CoreMethodOp::StringLen
        || length_source.placement() != ResolvedLoopPlacementV1::Condition
        || length_source.arity() != 0
        || !length_source.arguments().is_empty()
    {
        return Err(ConditionOperandInventoryRejectV1::LengthSourceShape);
    }
    if length_row.block != producer.condition_block()
        || length_row.result != producer.right()
        || length_row.result_class != LoopValueClassV2::I64
        || length_row.receiver != ingress.source.logical().roles().subject_input()
        || !matches!(length_row.args, S6CLogicalCallArgsV1::Empty)
    {
        return Err(ConditionOperandInventoryRejectV1::RightShape);
    }
    let mut right_operation = None;
    for row in operations.rows() {
        if row.item() != length_row.item {
            continue;
        }
        if right_operation.is_some() {
            return Err(ConditionOperandInventoryRejectV1::RightOperationDuplicate);
        }
        right_operation = Some(row);
    }
    let right_operation =
        right_operation.ok_or(ConditionOperandInventoryRejectV1::RightOperationMissing)?;
    if right_operation.block() != length_row.block
        || !matches!(
            right_operation.operation(),
            LoopOperationV2::CallSlot {
                receiver: Some(receiver),
                args,
                result: Some(result),
            } if *receiver == length_row.receiver
                && args.is_empty()
                && *result == length_row.result
        )
    {
        return Err(ConditionOperandInventoryRejectV1::RightOperationMismatch);
    }

    Ok(PreparedLoopV2ConditionOperandInventoryV1 {
        owner: expected_owner,
        condition_block: producer.condition_block(),
        rows: [
            PreparedLoopV2ConditionOperandRowV1 {
                value: left_value,
                item: left_item,
                block: left_block,
                class: LoopValueClassV2::I64,
                kind: PreparedLoopV2ConditionOperandKindV1::ReadBinding {
                    binding: index_binding,
                },
            },
            PreparedLoopV2ConditionOperandRowV1 {
                value: length_row.result,
                item: length_row.item,
                block: length_row.block,
                class: length_row.result_class,
                kind: PreparedLoopV2ConditionOperandKindV1::LengthCall {
                    source: length_source,
                },
            },
        ],
    })
}
