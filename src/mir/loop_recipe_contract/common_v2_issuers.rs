//! Caller-zero common V2 projections over one retained S6C source cohort.
//! The ingress validates; this module only projects logical rows, never physical IDs or a session.

use std::collections::BTreeSet;

use super::common_v2_after_boundary::{
    issue_s6c_v2_after_boundary_source_relation_v1, AfterBoundaryIssueRejectV1,
    VerifiedLoopV2AfterBoundarySourceRelationV1,
};
use super::common_v2_condition_operand_inventory::{
    issue_s6c_v2_condition_operand_inventory_v1, ConditionOperandInventoryRejectV1,
    PreparedLoopV2ConditionOperandInventoryV1,
};
use super::common_v2_condition_producer::{
    issue_s6c_v2_condition_producer_relation_v1, ConditionProducerRelationRejectV1,
    PreparedLoopV2ConditionProducerRelationV1,
};
use super::common_v2_continuation_relation::validate_continuation_relation;
use super::common_v2_initial_index_seed::{
    issue_s6c_v2_initial_index_seed_relation_v1, InitialIndexSeedRelationRejectV1,
    PreparedLoopV2InitialIndexSeedRelationV1,
};
use super::common_v2_layout_input::{
    issue_s6c_v2_layout_input, LayoutInputRejectV1, PreparedLoopV2PhysicalLayoutInputV1,
};
use super::common_v2_predicate_branch_plan::{
    issue_s6c_v2_predicate_branch_plan_v1, PredicateBranchPlanRejectV1,
    PreparedLoopV2PredicateBranchPlanV1,
};
use super::common_v2_return_read_co_seal::{
    issue_s6c_v2_return_read_co_seal_v1, CommonV2ReturnReadCoSealRefV1, ReturnReadCoSealRejectV1,
};
use super::ids::{LoopBlockKeyV1, LoopExitKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use super::join_sig::{LoopJoinBranchArmTransferRefV2, LoopJoinLogicalTransferViewV2};
use super::s6c_prephysical_ingress::{S6CPrephysicalIngressRefV2, S6CPrephysicalIngressRejectV2};
use super::s6c_return_source_binding::VerifiedS6CReturnSourceRecipeBindingV1;
use super::s6c_scan_with_init_joinir_output_rows::{S6CLogicalCallArgsV1, S6CLogicalItemV1};
use super::schema_v2::{LoopOperationExecutionClassV2, LoopOperationV2};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommonV2IssuerRejectV1 {
    Ingress(S6CPrephysicalIngressRejectV2),
    ForeignOwner,
    MissingOperation,
    DuplicateOperation,
    UnsupportedOperation,
    MissingControl,
    DuplicateControl,
    ControlRelation,
    ContinuationRelation,
    CoverageOverlap,
    S6CCoverage {
        operations: usize,
        controls: usize,
        placements: usize,
    },
    Layout(LayoutInputRejectV1),
    LayoutRelation,
    AfterBoundary(AfterBoundaryIssueRejectV1),
    PredicateBranch(PredicateBranchPlanRejectV1),
    ConditionProducer(ConditionProducerRelationRejectV1),
    ConditionOperandInventory(ConditionOperandInventoryRejectV1),
    InitialIndexSeed(InitialIndexSeedRelationRejectV1),
    ReturnReadCoSeal(ReturnReadCoSealRejectV1),
}

#[derive(Debug)]
pub(crate) struct PreparedLoopOperationRowV2<'source> {
    item: LoopItemKeyV1,
    block: super::ids::LoopBlockKeyV1,
    operation: LoopOperationV2,
    execution: LoopOperationExecutionClassV2,
    source: &'source S6CLogicalItemV1,
}

impl PreparedLoopOperationRowV2<'_> {
    pub(crate) const fn item(&self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) const fn block(&self) -> super::ids::LoopBlockKeyV1 {
        self.block
    }

    pub(crate) fn operation(&self) -> &LoopOperationV2 {
        &self.operation
    }

    /// Lend the already-issued S6C logical row to a physical consumer.
    /// This is a source projection only; it does not issue another semantic
    /// receipt or expose physical IDs.
    pub(crate) fn source(&self) -> &S6CLogicalItemV1 {
        self.source
    }

    pub(crate) const fn execution(&self) -> LoopOperationExecutionClassV2 {
        self.execution
    }
}

#[derive(Debug)]
pub(crate) struct PreparedLoopOperationProgramV2<'source> {
    owner: FunctionOwnerIdV1,
    rows: Box<[PreparedLoopOperationRowV2<'source>]>,
}

impl PreparedLoopOperationProgramV2<'_> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn rows(&self) -> &[PreparedLoopOperationRowV2<'_>] {
        &self.rows
    }

    pub(crate) fn operation_count(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreparedLoopControlPlacementV2 {
    If {
        item: LoopItemKeyV1,
        block: super::ids::LoopBlockKeyV1,
        condition: LoopValueKeyV1,
        then_block: super::ids::LoopBlockKeyV1,
        else_block: Option<super::ids::LoopBlockKeyV1>,
    },
    Exit {
        item: LoopItemKeyV1,
        block: super::ids::LoopBlockKeyV1,
        exit: LoopExitKeyV1,
        value: LoopValueKeyV1,
    },
}

impl PreparedLoopControlPlacementV2 {
    pub(crate) const fn item(self) -> LoopItemKeyV1 {
        match self {
            Self::If { item, .. } | Self::Exit { item, .. } => item,
        }
    }
}

#[derive(Debug)]
pub(crate) struct PreparedLoopControlTransferProgramV2<'source, 'join> {
    rows: Box<[PreparedLoopControlPlacementV2]>,
    transfer: &'join LoopJoinLogicalTransferViewV2<'join>,
    source: &'source [S6CLogicalItemV1],
}

impl PreparedLoopControlTransferProgramV2<'_, '_> {
    pub(crate) fn rows(&self) -> &[PreparedLoopControlPlacementV2] {
        &self.rows
    }

    pub(crate) const fn transfer(&self) -> &LoopJoinLogicalTransferViewV2<'_> {
        self.transfer
    }

    pub(crate) fn control_count(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VerifiedLoopV2EnvelopeCoverageV1 {
    operation_count: usize,
    control_count: usize,
    placement_count: usize,
}

impl VerifiedLoopV2EnvelopeCoverageV1 {
    pub(crate) const fn operation_count(self) -> usize {
        self.operation_count
    }

    pub(crate) const fn control_count(self) -> usize {
        self.control_count
    }

    pub(crate) const fn placement_count(self) -> usize {
        self.placement_count
    }
}

#[derive(Debug)]
pub(crate) struct PreparedLoopV2PreSessionEnvelopeV1<'source, 'join> {
    owner: FunctionOwnerIdV1,
    operations: PreparedLoopOperationProgramV2<'source>,
    control: PreparedLoopControlTransferProgramV2<'source, 'join>,
    layout: PreparedLoopV2PhysicalLayoutInputV1<'source>,
    after_boundary: VerifiedLoopV2AfterBoundarySourceRelationV1,
    predicate_branch: PreparedLoopV2PredicateBranchPlanV1,
    condition_producer: PreparedLoopV2ConditionProducerRelationV1,
    condition_operands: PreparedLoopV2ConditionOperandInventoryV1<'join>,
    initial_index_seed: PreparedLoopV2InitialIndexSeedRelationV1<'join>,
    return_source_binding: &'join VerifiedS6CReturnSourceRecipeBindingV1,
    return_read_co_seal: CommonV2ReturnReadCoSealRefV1<'join>,
    coverage: VerifiedLoopV2EnvelopeCoverageV1,
}

impl<'source, 'join> PreparedLoopV2PreSessionEnvelopeV1<'source, 'join> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn operations(&self) -> &PreparedLoopOperationProgramV2<'_> {
        &self.operations
    }

    pub(crate) fn control(&self) -> &PreparedLoopControlTransferProgramV2<'_, '_> {
        &self.control
    }

    pub(crate) fn layout<'borrow>(
        &'borrow self,
    ) -> &'borrow PreparedLoopV2PhysicalLayoutInputV1<'source> {
        &self.layout
    }

    pub(crate) const fn coverage(&self) -> VerifiedLoopV2EnvelopeCoverageV1 {
        self.coverage
    }

    pub(crate) fn after_boundary(&self) -> &VerifiedLoopV2AfterBoundarySourceRelationV1 {
        &self.after_boundary
    }

    pub(crate) fn predicate_branch(&self) -> &PreparedLoopV2PredicateBranchPlanV1 {
        &self.predicate_branch
    }

    pub(crate) fn condition_producer(&self) -> &PreparedLoopV2ConditionProducerRelationV1 {
        &self.condition_producer
    }

    pub(crate) fn condition_operands(&self) -> &PreparedLoopV2ConditionOperandInventoryV1<'join> {
        &self.condition_operands
    }

    pub(crate) fn initial_index_seed(&self) -> &PreparedLoopV2InitialIndexSeedRelationV1<'join> {
        &self.initial_index_seed
    }

    pub(crate) fn return_source_binding(&self) -> &'join VerifiedS6CReturnSourceRecipeBindingV1 {
        self.return_source_binding
    }

    pub(crate) fn return_read_co_seal(&self) -> &CommonV2ReturnReadCoSealRefV1<'join> {
        &self.return_read_co_seal
    }
}

pub(crate) fn issue_s6c_common_v2_pre_session_v1<'source, 'join>(
    ingress: S6CPrephysicalIngressRefV2<'_, 'source, 'join>,
    expected_owner: FunctionOwnerIdV1,
) -> Result<PreparedLoopV2PreSessionEnvelopeV1<'source, 'join>, CommonV2IssuerRejectV1> {
    if ingress.source_owner() != expected_owner {
        return Err(CommonV2IssuerRejectV1::ForeignOwner);
    }
    let initial_index_seed = issue_s6c_v2_initial_index_seed_relation_v1(ingress, expected_owner)
        .map_err(CommonV2IssuerRejectV1::InitialIndexSeed)?;
    let return_source_binding = ingress.return_source_binding();
    if return_source_binding.owner() != expected_owner {
        return Err(CommonV2IssuerRejectV1::ForeignOwner);
    }
    let operations = issue_operation_source(ingress)?;
    let control = issue_control_source(ingress)?;
    let layout = issue_s6c_v2_layout_input(ingress, expected_owner)
        .map_err(CommonV2IssuerRejectV1::Layout)?;
    let return_read_co_seal =
        issue_s6c_v2_return_read_co_seal_v1(ingress, &operations, &control, &layout)
            .map_err(CommonV2IssuerRejectV1::ReturnReadCoSeal)?;
    let after_boundary =
        issue_s6c_v2_after_boundary_source_relation_v1(ingress, &layout, expected_owner)
            .map_err(CommonV2IssuerRejectV1::AfterBoundary)?;
    let predicate_branch = issue_s6c_v2_predicate_branch_plan_v1(
        ingress,
        &layout,
        control.transfer(),
        &after_boundary,
        expected_owner,
    )
    .map_err(CommonV2IssuerRejectV1::PredicateBranch)?;
    let condition_producer = issue_s6c_v2_condition_producer_relation_v1(
        ingress,
        &operations,
        &predicate_branch,
        expected_owner,
    )
    .map_err(CommonV2IssuerRejectV1::ConditionProducer)?;
    let condition_operands = issue_s6c_v2_condition_operand_inventory_v1(
        ingress,
        &operations,
        &condition_producer,
        expected_owner,
    )
    .map_err(CommonV2IssuerRejectV1::ConditionOperandInventory)?;
    validate_layout_relation(&layout, &operations, &control)?;
    let coverage = issue_coverage(&operations, &control)?;
    if operations.operation_count() != 13
        || control.control_count() != 2
        || coverage.placement_count() != 15
        || ingress.logical_items().len() != 15
    {
        return Err(CommonV2IssuerRejectV1::S6CCoverage {
            operations: operations.operation_count(),
            controls: control.control_count(),
            placements: coverage.placement_count(),
        });
    }
    Ok(PreparedLoopV2PreSessionEnvelopeV1 {
        owner: expected_owner,
        operations,
        control,
        layout,
        after_boundary,
        predicate_branch,
        condition_producer,
        condition_operands,
        initial_index_seed,
        return_source_binding,
        return_read_co_seal,
        coverage,
    })
}

fn validate_layout_relation(
    layout: &PreparedLoopV2PhysicalLayoutInputV1<'_>,
    operations: &PreparedLoopOperationProgramV2<'_>,
    control: &PreparedLoopControlTransferProgramV2<'_, '_>,
) -> Result<(), CommonV2IssuerRejectV1> {
    let mut covered = BTreeSet::new();
    for segment in layout.segments() {
        for item in segment.items() {
            if !covered.insert(*item) {
                return Err(CommonV2IssuerRejectV1::LayoutRelation);
            }
        }
    }
    for row in operations.rows() {
        if !layout_item_matches_block(layout, row.block(), row.item()) {
            return Err(CommonV2IssuerRejectV1::LayoutRelation);
        }
    }
    for row in control.rows() {
        match row {
            PreparedLoopControlPlacementV2::If {
                item,
                block,
                then_block,
                else_block,
                ..
            } => {
                if !layout_item_matches_block(layout, *block, *item)
                    || layout.segment_for_block(*then_block).is_none()
                    || else_block.is_some_and(|target| layout.segment_for_block(target).is_none())
                {
                    return Err(CommonV2IssuerRejectV1::LayoutRelation);
                }
            }
            PreparedLoopControlPlacementV2::Exit { item, block, .. } => {
                if !layout_item_matches_block(layout, *block, *item) {
                    return Err(CommonV2IssuerRejectV1::LayoutRelation);
                }
            }
        }
    }
    let expected = operations
        .rows()
        .iter()
        .map(|row| row.item())
        .chain(control.rows().iter().map(|row| row.item()))
        .collect::<BTreeSet<_>>();
    if expected != covered || layout.item_count() != covered.len() {
        return Err(CommonV2IssuerRejectV1::LayoutRelation);
    }
    Ok(())
}

fn layout_item_matches_block(
    layout: &PreparedLoopV2PhysicalLayoutInputV1<'_>,
    block: LoopBlockKeyV1,
    item: LoopItemKeyV1,
) -> bool {
    layout
        .segment_for_block(block)
        .is_some_and(|segment| segment.items().contains(&item))
}

fn issue_operation_source<'source, 'join>(
    ingress: S6CPrephysicalIngressRefV2<'_, 'source, 'join>,
) -> Result<PreparedLoopOperationProgramV2<'source>, CommonV2IssuerRejectV1> {
    let mut seen = BTreeSet::new();
    let mut rows = Vec::new();
    for source in ingress.logical_items() {
        let Some((item, block, operation)) = project_operation(source)? else {
            continue;
        };
        if !seen.insert(item) {
            return Err(CommonV2IssuerRejectV1::DuplicateOperation);
        }
        let execution = operation.execution_class_v2();
        rows.push(PreparedLoopOperationRowV2 {
            item,
            block,
            operation,
            execution,
            source,
        });
    }
    if rows.is_empty() {
        return Err(CommonV2IssuerRejectV1::MissingOperation);
    }
    Ok(PreparedLoopOperationProgramV2 {
        owner: ingress.source_owner(),
        rows: rows.into_boxed_slice(),
    })
}

fn project_operation(
    source: &S6CLogicalItemV1,
) -> Result<
    Option<(LoopItemKeyV1, super::ids::LoopBlockKeyV1, LoopOperationV2)>,
    CommonV2IssuerRejectV1,
> {
    let projected = match *source {
        S6CLogicalItemV1::ReadBinding {
            binding, result, ..
        } => LoopOperationV2::ReadBinding { binding, result },
        S6CLogicalItemV1::ConstI64 { result, value, .. } => {
            LoopOperationV2::ConstI64 { result, value }
        }
        S6CLogicalItemV1::BinaryI64 {
            op,
            left,
            right,
            result,
            ..
        } => LoopOperationV2::BinaryI64 {
            op,
            left,
            right,
            result,
        },
        S6CLogicalItemV1::CompareI64 {
            op,
            left,
            right,
            result,
            ..
        } => LoopOperationV2::CompareI64 {
            op,
            left,
            right,
            result,
        },
        S6CLogicalItemV1::CallSlot(call) => LoopOperationV2::CallSlot {
            receiver: Some(call.receiver),
            args: match call.args {
                S6CLogicalCallArgsV1::Empty => Vec::new(),
                S6CLogicalCallArgsV1::Pair(args) => args.into_iter().collect(),
            },
            result: Some(call.result),
        },
        S6CLogicalItemV1::TextEq {
            left,
            right,
            result,
            ..
        } => LoopOperationV2::TextEq {
            left,
            right,
            result,
        },
        S6CLogicalItemV1::WriteBinding { binding, value, .. } => {
            LoopOperationV2::WriteBinding { binding, value }
        }
        S6CLogicalItemV1::If { .. } | S6CLogicalItemV1::Exit { .. } => return Ok(None),
    };
    let (item, block) = match *source {
        S6CLogicalItemV1::ReadBinding { item, block, .. }
        | S6CLogicalItemV1::ConstI64 { item, block, .. }
        | S6CLogicalItemV1::BinaryI64 { item, block, .. }
        | S6CLogicalItemV1::CompareI64 { item, block, .. }
        | S6CLogicalItemV1::TextEq { item, block, .. }
        | S6CLogicalItemV1::WriteBinding { item, block, .. } => (item, block),
        S6CLogicalItemV1::CallSlot(call) => (call.item, call.block),
        S6CLogicalItemV1::If { .. } | S6CLogicalItemV1::Exit { .. } => {
            return Err(CommonV2IssuerRejectV1::UnsupportedOperation)
        }
    };
    Ok(Some((item, block, projected)))
}

fn issue_control_source<'source, 'join>(
    ingress: S6CPrephysicalIngressRefV2<'_, 'source, 'join>,
) -> Result<PreparedLoopControlTransferProgramV2<'source, 'join>, CommonV2IssuerRejectV1> {
    let items = ingress.logical_items();
    let mut seen = BTreeSet::new();
    let mut rows = Vec::new();
    for source in items {
        let row = match *source {
            S6CLogicalItemV1::If {
                item,
                block,
                condition,
                then_block,
                else_block,
            } => PreparedLoopControlPlacementV2::If {
                item,
                block,
                condition,
                then_block,
                else_block,
            },
            S6CLogicalItemV1::Exit {
                item,
                block,
                exit,
                value,
            } => PreparedLoopControlPlacementV2::Exit {
                item,
                block,
                exit,
                value,
            },
            _ => continue,
        };
        if !seen.insert(row.item()) {
            return Err(CommonV2IssuerRejectV1::DuplicateControl);
        }
        rows.push(row);
    }
    let transfer = ingress.logical_transfer();
    validate_continuation_relation(
        transfer.branches(),
        ingress.logical_items(),
        ingress.logical_blocks(),
    )?;
    if rows.is_empty() {
        return Err(CommonV2IssuerRejectV1::MissingControl);
    }
    let branch_items = transfer
        .branches()
        .iter()
        .map(|branch| branch.if_item)
        .collect::<BTreeSet<_>>();
    let exit_items = transfer
        .branches()
        .iter()
        .flat_map(|branch| [branch.then_arm, branch.else_arm])
        .filter_map(|arm| match arm {
            LoopJoinBranchArmTransferRefV2::Exit(exit) => Some(exit.exit_item),
            LoopJoinBranchArmTransferRefV2::Fallthrough { .. } => None,
        })
        .collect::<BTreeSet<_>>();
    let row_items = rows.iter().map(|row| row.item()).collect::<BTreeSet<_>>();
    let if_items = rows
        .iter()
        .filter_map(|row| match row {
            PreparedLoopControlPlacementV2::If { item, .. } => Some(*item),
            PreparedLoopControlPlacementV2::Exit { .. } => None,
        })
        .collect::<BTreeSet<_>>();
    let exit_row_items = rows
        .iter()
        .filter_map(|row| match row {
            PreparedLoopControlPlacementV2::If { .. } => None,
            PreparedLoopControlPlacementV2::Exit { item, .. } => Some(*item),
        })
        .collect::<BTreeSet<_>>();
    if if_items != branch_items || exit_row_items != exit_items || row_items.len() != rows.len() {
        return Err(CommonV2IssuerRejectV1::ControlRelation);
    }
    Ok(PreparedLoopControlTransferProgramV2 {
        rows: rows.into_boxed_slice(),
        transfer,
        source: items,
    })
}

fn issue_coverage(
    operations: &PreparedLoopOperationProgramV2<'_>,
    control: &PreparedLoopControlTransferProgramV2<'_, '_>,
) -> Result<VerifiedLoopV2EnvelopeCoverageV1, CommonV2IssuerRejectV1> {
    let operation_items = operations
        .rows()
        .iter()
        .map(|row| row.item())
        .collect::<BTreeSet<_>>();
    let control_items = control
        .rows()
        .iter()
        .map(|row| row.item())
        .collect::<BTreeSet<_>>();
    if operation_items
        .intersection(&control_items)
        .next()
        .is_some()
    {
        return Err(CommonV2IssuerRejectV1::CoverageOverlap);
    }
    let placement_count = operation_items.len() + control_items.len();
    Ok(VerifiedLoopV2EnvelopeCoverageV1 {
        operation_count: operation_items.len(),
        control_count: control_items.len(),
        placement_count,
    })
}

#[cfg(test)]
#[path = "common_v2_condition_producer_tests.rs"]
mod condition_producer_tests;

#[cfg(test)]
#[path = "common_v2_condition_operand_inventory_tests.rs"]
mod condition_operand_inventory_tests;

#[cfg(test)]
#[path = "common_v2_issuers_tests.rs"]
mod tests;
