//! Source-backed condition producer relation for common V2.
//!
//! This is a logical, physical-ID-free relation.  It ties the resolver's
//! loop condition to exactly one CompareI64 source item and its generic
//! operation projection.  No ValueId, branch emission, or CFG mutation is
//! issued here.

use super::common_v2_issuers::PreparedLoopOperationProgramV2;
use super::common_v2_predicate_branch_plan::PreparedLoopV2PredicateBranchPlanV1;
use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::s6c_prephysical_ingress::S6CPrephysicalIngressRefV2;
use super::s6c_scan_with_init_joinir_output_rows::S6CLogicalItemV1;
use super::schema_v2::{LoopCompareI64OpV2, LoopValueClassV2};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConditionProducerRelationRejectV1 {
    ForeignOwner,
    MissingLoop,
    ConditionBlockMismatch,
    MissingProducer,
    DuplicateProducer,
    ProducerBlockMismatch,
    ProducerOperationMismatch,
    ProducerResultMismatch,
    OperandValueMissing,
    OperandClassMismatch,
    OperationRowMissing,
    DuplicateOperationRow,
    OperationRowMismatch,
}

/// Exact source-to-operation relation for the loop predicate producer.
///
/// The producer item is retained as a logical identity only.  A later
/// materializer must obtain the physical result from the canonical session.
#[derive(Debug)]
pub(crate) struct PreparedLoopV2ConditionProducerRelationV1 {
    owner: FunctionOwnerIdV1,
    loop_key: LoopNodeKeyV1,
    condition_block: LoopBlockKeyV1,
    producer_item: LoopItemKeyV1,
    left: LoopValueKeyV1,
    right: LoopValueKeyV1,
    result: LoopValueKeyV1,
    op: LoopCompareI64OpV2,
    class: LoopValueClassV2,
}

impl PreparedLoopV2ConditionProducerRelationV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn loop_key(&self) -> LoopNodeKeyV1 {
        self.loop_key
    }

    pub(crate) const fn condition_block(&self) -> LoopBlockKeyV1 {
        self.condition_block
    }

    pub(crate) const fn producer_item(&self) -> LoopItemKeyV1 {
        self.producer_item
    }

    pub(crate) const fn left(&self) -> LoopValueKeyV1 {
        self.left
    }

    pub(crate) const fn right(&self) -> LoopValueKeyV1 {
        self.right
    }

    pub(crate) const fn result(&self) -> LoopValueKeyV1 {
        self.result
    }

    pub(crate) const fn op(&self) -> LoopCompareI64OpV2 {
        self.op
    }

    pub(crate) const fn class(&self) -> LoopValueClassV2 {
        self.class
    }
}

pub(crate) fn issue_s6c_v2_condition_producer_relation_v1<'source, 'join>(
    ingress: S6CPrephysicalIngressRefV2<'_, 'source, 'join>,
    operations: &PreparedLoopOperationProgramV2<'source>,
    branch: &PreparedLoopV2PredicateBranchPlanV1,
    expected_owner: FunctionOwnerIdV1,
) -> Result<PreparedLoopV2ConditionProducerRelationV1, ConditionProducerRelationRejectV1> {
    if ingress.source_owner() != expected_owner
        || operations.owner() != expected_owner
        || branch.owner() != expected_owner
    {
        return Err(ConditionProducerRelationRejectV1::ForeignOwner);
    }

    let condition = branch.condition();
    let root = ingress
        .logical_loops()
        .iter()
        .find(|row| row.key == branch.loop_key())
        .ok_or(ConditionProducerRelationRejectV1::MissingLoop)?;
    if root.condition_block != condition.block() {
        return Err(ConditionProducerRelationRejectV1::ConditionBlockMismatch);
    }
    if condition.class() != LoopValueClassV2::Bool || root.condition_value != condition.value() {
        return Err(ConditionProducerRelationRejectV1::ProducerResultMismatch);
    }

    let mut producer = None;
    for item in ingress.logical_items() {
        let S6CLogicalItemV1::CompareI64 {
            item,
            block,
            op,
            left,
            right,
            result,
        } = *item
        else {
            continue;
        };
        if result != condition.value() {
            continue;
        }
        if producer.is_some() {
            return Err(ConditionProducerRelationRejectV1::DuplicateProducer);
        }
        producer = Some((item, block, op, left, right, result));
    }
    let (producer_item, producer_block, op, left, right, result) =
        producer.ok_or(ConditionProducerRelationRejectV1::MissingProducer)?;
    if producer_block != condition.block() {
        return Err(ConditionProducerRelationRejectV1::ProducerBlockMismatch);
    }
    if op != LoopCompareI64OpV2::Less {
        return Err(ConditionProducerRelationRejectV1::ProducerOperationMismatch);
    }

    let values = ingress.source.logical().rows().values();
    for value in [left, right] {
        let Some(row) = values.iter().find(|row| row.key == value) else {
            return Err(ConditionProducerRelationRejectV1::OperandValueMissing);
        };
        if row.class != LoopValueClassV2::I64 {
            return Err(ConditionProducerRelationRejectV1::OperandClassMismatch);
        }
    }

    let mut operation_row = None;
    for row in operations.rows() {
        if row.item() != producer_item {
            continue;
        }
        if operation_row.is_some() {
            return Err(ConditionProducerRelationRejectV1::DuplicateOperationRow);
        }
        operation_row = Some(row);
    }
    let row = operation_row.ok_or(ConditionProducerRelationRejectV1::OperationRowMissing)?;
    if row.block() != producer_block {
        return Err(ConditionProducerRelationRejectV1::OperationRowMismatch);
    }
    let matches = matches!(
        row.operation(),
        super::schema_v2::LoopOperationV2::CompareI64 {
            op: row_op,
            left: row_left,
            right: row_right,
            result: row_result,
        } if *row_op == op && *row_left == left && *row_right == right && *row_result == result
    );
    if !matches {
        return Err(ConditionProducerRelationRejectV1::OperationRowMismatch);
    }

    Ok(PreparedLoopV2ConditionProducerRelationV1 {
        owner: expected_owner,
        loop_key: branch.loop_key(),
        condition_block: condition.block(),
        producer_item,
        left,
        right,
        result,
        op,
        class: condition.class(),
    })
}
