//! Same-cohort logical/layout/Join co-seal for the S6C Return-read arm.
//!
//! This view copies no physical identity and issues no new semantic meaning.
//! It only proves that the existing source-to-Recipe/Join binding agrees with
//! the common operation/control/layout rows before a later canonical session
//! can consider physical materialization.

use super::common_v2_issuers::{
    PreparedLoopControlPlacementV2, PreparedLoopControlTransferProgramV2,
    PreparedLoopOperationProgramV2,
};
use super::common_v2_layout_input::PreparedLoopV2PhysicalLayoutInputV1;
use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use super::join_sig::{
    LoopJoinBranchArmTransferRefV2, LoopJoinBranchExitTargetV2, LoopJoinEdgeRoleV1,
    LoopJoinNextItemV1,
};
use super::s6c_prephysical_ingress::S6CPrephysicalIngressRefV2;
use super::s6c_return_source_binding::VerifiedS6CReturnSourceRecipeBindingV1;
use super::schema_v2::LoopOperationV2;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, RegionId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReturnReadCoSealRejectV1 {
    ForeignOwner,
    MissingReturnOperation,
    DuplicateReturnOperation,
    ReturnOperationMismatch,
    MissingIfPlacement,
    DuplicateIfPlacement,
    IfPlacementMismatch,
    MissingExitPlacement,
    DuplicateExitPlacement,
    ExitPlacementMismatch,
    MissingReturnSegment,
    MissingIfSegment,
    SegmentMismatch,
    BranchMissing,
    BranchDuplicate,
    BranchMismatch,
    ContinuationMissing,
    ContinuationNotStrict,
    ContinuationIsControl,
}

/// Callback-scoped evidence that existing logical rows agree before physical
/// Return-read lowering.  It is deliberately not `Clone` and carries no
/// `BasicBlockId`, `ValueId`, edge, terminator, or Completion claim.
#[derive(Debug)]
pub(crate) struct CommonV2ReturnReadCoSealRefV1<'source> {
    owner: FunctionOwnerIdV1,
    source_binding: &'source VerifiedS6CReturnSourceRecipeBindingV1,
    return_item: LoopItemKeyV1,
    return_block: LoopBlockKeyV1,
    return_value: LoopValueKeyV1,
    return_split_ordinal: u32,
    if_item: LoopItemKeyV1,
    if_block: LoopBlockKeyV1,
    if_condition: LoopValueKeyV1,
    if_split_ordinal: u32,
    continuation: LoopJoinNextItemV1,
    join_exit_item: LoopItemKeyV1,
    join_target: LoopJoinBranchExitTargetV2,
    target_function: RegionId,
}

impl CommonV2ReturnReadCoSealRefV1<'_> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn source_binding(&self) -> &VerifiedS6CReturnSourceRecipeBindingV1 {
        self.source_binding
    }

    pub(crate) const fn return_item(&self) -> LoopItemKeyV1 {
        self.return_item
    }

    pub(crate) const fn return_block(&self) -> LoopBlockKeyV1 {
        self.return_block
    }

    pub(crate) const fn return_value(&self) -> LoopValueKeyV1 {
        self.return_value
    }

    pub(crate) const fn return_split_ordinal(&self) -> u32 {
        self.return_split_ordinal
    }

    pub(crate) const fn if_item(&self) -> LoopItemKeyV1 {
        self.if_item
    }

    pub(crate) const fn if_block(&self) -> LoopBlockKeyV1 {
        self.if_block
    }

    pub(crate) const fn if_condition(&self) -> LoopValueKeyV1 {
        self.if_condition
    }

    pub(crate) const fn if_split_ordinal(&self) -> u32 {
        self.if_split_ordinal
    }

    pub(crate) const fn continuation(&self) -> LoopJoinNextItemV1 {
        self.continuation
    }

    pub(crate) const fn join_exit_item(&self) -> LoopItemKeyV1 {
        self.join_exit_item
    }

    pub(crate) const fn join_target(&self) -> LoopJoinBranchExitTargetV2 {
        self.join_target
    }

    pub(crate) const fn target_function(&self) -> RegionId {
        self.target_function
    }
}

pub(crate) fn issue_s6c_v2_return_read_co_seal_v1<'rows, 'facts>(
    ingress: S6CPrephysicalIngressRefV2<'_, 'rows, 'facts>,
    operations: &PreparedLoopOperationProgramV2<'rows>,
    control: &PreparedLoopControlTransferProgramV2<'rows, 'facts>,
    layout: &PreparedLoopV2PhysicalLayoutInputV1<'rows>,
) -> Result<CommonV2ReturnReadCoSealRefV1<'facts>, ReturnReadCoSealRejectV1> {
    let owner = ingress.source_owner();
    let binding = ingress.return_source_binding();
    let target_function = ingress.completion().target_function();
    if owner != operations.owner() || owner != layout.owner() || owner != binding.owner() {
        return Err(ReturnReadCoSealRejectV1::ForeignOwner);
    }

    let return_item = binding.recipe_return_item();
    let return_rows = operations
        .rows()
        .iter()
        .filter(|row| row.item() == return_item)
        .collect::<Vec<_>>();
    let return_row = match return_rows.as_slice() {
        [] => return Err(ReturnReadCoSealRejectV1::MissingReturnOperation),
        [_] => return_rows[0],
        _ => return Err(ReturnReadCoSealRejectV1::DuplicateReturnOperation),
    };
    if return_row.block() != binding.recipe_return_block()
        || !matches!(
            return_row.operation(),
            LoopOperationV2::ReadBinding { binding: actual, result }
                if *actual == ingress.index_binding()
                    && *result == binding.recipe_return_value()
        )
    {
        return Err(ReturnReadCoSealRejectV1::ReturnOperationMismatch);
    }

    let if_item = binding.recipe_if_item();
    let if_rows = control
        .rows()
        .iter()
        .filter(|row| row.item() == if_item)
        .collect::<Vec<_>>();
    let (if_block, if_condition, then_block) = match if_rows.as_slice() {
        [] => return Err(ReturnReadCoSealRejectV1::MissingIfPlacement),
        [_] => match if_rows[0] {
            PreparedLoopControlPlacementV2::If {
                block,
                condition,
                then_block,
                else_block: None,
                ..
            } => (*block, *condition, *then_block),
            _ => return Err(ReturnReadCoSealRejectV1::IfPlacementMismatch),
        },
        _ => return Err(ReturnReadCoSealRejectV1::DuplicateIfPlacement),
    };
    if if_block != binding.recipe_if_block() || then_block != binding.recipe_then_block() {
        return Err(ReturnReadCoSealRejectV1::IfPlacementMismatch);
    }

    let exit_item = binding.join_exit_item();
    let exit_rows = control
        .rows()
        .iter()
        .filter(|row| row.item() == exit_item)
        .collect::<Vec<_>>();
    match exit_rows.as_slice() {
        [] => return Err(ReturnReadCoSealRejectV1::MissingExitPlacement),
        [_] => match exit_rows[0] {
            PreparedLoopControlPlacementV2::Exit {
                block, exit, value, ..
            } if *block == binding.recipe_return_block()
                && *exit == binding.recipe_exit()
                && *value == binding.recipe_return_value() => {}
            _ => return Err(ReturnReadCoSealRejectV1::ExitPlacementMismatch),
        },
        _ => return Err(ReturnReadCoSealRejectV1::DuplicateExitPlacement),
    }

    let return_segment = layout
        .segment_for_block(binding.recipe_return_block())
        .ok_or(ReturnReadCoSealRejectV1::MissingReturnSegment)?;
    let if_segment = layout
        .segment_for_block(if_block)
        .ok_or(ReturnReadCoSealRejectV1::MissingIfSegment)?;
    if !return_segment.items().contains(&return_item)
        || !return_segment.items().contains(&exit_item)
        || !if_segment.items().contains(&if_item)
    {
        return Err(ReturnReadCoSealRejectV1::SegmentMismatch);
    }

    let branches = control
        .transfer()
        .branches()
        .iter()
        .filter(|branch| branch.if_item == if_item)
        .collect::<Vec<_>>();
    let branch = match branches.as_slice() {
        [] => return Err(ReturnReadCoSealRejectV1::BranchMissing),
        [branch] => *branch,
        _ => return Err(ReturnReadCoSealRejectV1::BranchDuplicate),
    };
    let continuation = match (branch.then_arm, branch.else_arm) {
        (
            LoopJoinBranchArmTransferRefV2::Exit(exit),
            LoopJoinBranchArmTransferRefV2::Fallthrough { continuation, .. },
        ) if branch.condition == if_condition
            && exit.exit_item == exit_item
            && exit.role == LoopJoinEdgeRoleV1::Return
            && exit.target == LoopJoinBranchExitTargetV2::FunctionExit =>
        {
            continuation
        }
        _ => return Err(ReturnReadCoSealRejectV1::BranchMismatch),
    };
    if continuation.block != if_block {
        return Err(ReturnReadCoSealRejectV1::ContinuationMissing);
    }
    let Some(if_position) = if_segment.items().iter().position(|item| *item == if_item) else {
        return Err(ReturnReadCoSealRejectV1::ContinuationMissing);
    };
    let Some(continuation_position) = if_segment
        .items()
        .iter()
        .position(|item| *item == continuation.item)
    else {
        return Err(ReturnReadCoSealRejectV1::ContinuationMissing);
    };
    if continuation_position <= if_position {
        return Err(ReturnReadCoSealRejectV1::ContinuationNotStrict);
    }
    if control
        .rows()
        .iter()
        .any(|row| row.item() == continuation.item)
    {
        return Err(ReturnReadCoSealRejectV1::ContinuationIsControl);
    }

    Ok(CommonV2ReturnReadCoSealRefV1 {
        owner,
        source_binding: binding,
        return_item,
        return_block: binding.recipe_return_block(),
        return_value: binding.recipe_return_value(),
        return_split_ordinal: return_segment.split_ordinal(),
        if_item,
        if_block,
        if_condition,
        if_split_ordinal: if_segment.split_ordinal(),
        continuation,
        join_exit_item: exit_item,
        join_target: binding.join_target(),
        target_function,
    })
}
