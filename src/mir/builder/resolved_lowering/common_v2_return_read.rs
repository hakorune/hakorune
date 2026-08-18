//! Physical item-9 Return-read receipt for the common V2 session.
//!
//! This is the first effect-bearing consumer of the existing logical
//! Return-read co-seal. It joins that source relation with the same-session
//! segment rows and continuation target before asking canonical identity/SSA
//! for the read. The outer unpublished function transaction remains the only
//! rollback owner; branch/Return CFG writing is intentionally not here.

use super::super::common_v2_if_continuation_target::{
    issue_if_continuation_target, IfContinuationPhysicalTargetRejectV1,
};
use super::super::common_v2_segment_block_allocation::{
    PreparedSegmentBlockReceiptV1, SegmentBlockReceiptRowV1,
};
use super::CommonV2CanonicalSessionRefV1;
use crate::mir::loop_recipe_contract::{
    CommonV2ReturnReadCoSealRefV1, LoopJoinBranchExitTargetV2, LoopJoinNextItemV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, RegionId, ResolvedExitSiteV1,
};
use crate::mir::{BasicBlockId, MirBuilder, MirType, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum ReturnReadPhysicalReceiptRejectV1 {
    AlreadyIssued,
    OwnerMismatch,
    MissingPhysicalEntryStamp,
    CoSealMismatch,
    SegmentOwnerMismatch,
    SegmentMissing,
    SegmentDuplicate,
    SegmentMismatch,
    SegmentScopeMismatch,
    Target(IfContinuationPhysicalTargetRejectV1),
    TargetMismatch,
    CanonicalRead(String),
    CanonicalReadMismatch,
    ReturnTypeMismatch(Option<MirType>),
    Completion(String),
    Identity(String),
    Callback(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OwnedContinuationPlacement {
    if_item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    continuation: LoopJoinNextItemV1,
    source_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    source_split_ordinal: u32,
    physical_block: BasicBlockId,
    stamp_owner: FunctionOwnerIdV1,
}

/// Callback-scoped physical evidence for item 9 and its item-10 terminal.
///
/// The receipt owns the mutable session borrow so callers cannot detach the
/// canonical read from the source/layout/Join cohort. It records no branch or
/// Return instruction; DraftSeal/CFG writers remain later mechanical owners.
pub(in crate::mir::builder) struct CommonV2ReturnReadPhysicalReceiptV1<'receipt, 'source, 'envelope>
{
    _session: &'receipt mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    owner: FunctionOwnerIdV1,
    binding: BindingRefV1,
    return_item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    logical_result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    then_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    then_split_ordinal: u32,
    then_physical_block: BasicBlockId,
    if_item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    if_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    if_condition: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    if_split_ordinal: u32,
    if_physical_block: BasicBlockId,
    continuation: LoopJoinNextItemV1,
    continuation_physical_block: BasicBlockId,
    exit_item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    terminal_block: BasicBlockId,
    target_function: RegionId,
    join_target: LoopJoinBranchExitTargetV2,
    physical_value: ValueId,
    segment_brand: super::super::common_v2_segment_block_allocation::SegmentBlockAllocationBrandV1,
}

impl CommonV2ReturnReadPhysicalReceiptV1<'_, '_, '_> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(in crate::mir::builder) const fn return_item(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopItemKeyV1 {
        self.return_item
    }

    pub(in crate::mir::builder) const fn logical_result(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.logical_result
    }

    pub(in crate::mir::builder) const fn then_block(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopBlockKeyV1 {
        self.then_block
    }

    pub(in crate::mir::builder) const fn then_split_ordinal(&self) -> u32 {
        self.then_split_ordinal
    }

    pub(in crate::mir::builder) const fn then_physical_block(&self) -> BasicBlockId {
        self.then_physical_block
    }

    pub(in crate::mir::builder) const fn if_item(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopItemKeyV1 {
        self.if_item
    }

    pub(in crate::mir::builder) const fn if_block(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopBlockKeyV1 {
        self.if_block
    }

    pub(in crate::mir::builder) const fn if_condition(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.if_condition
    }

    pub(in crate::mir::builder) const fn if_split_ordinal(&self) -> u32 {
        self.if_split_ordinal
    }

    pub(in crate::mir::builder) const fn if_physical_block(&self) -> BasicBlockId {
        self.if_physical_block
    }

    pub(in crate::mir::builder) const fn continuation(&self) -> LoopJoinNextItemV1 {
        self.continuation
    }

    pub(in crate::mir::builder) const fn continuation_physical_block(&self) -> BasicBlockId {
        self.continuation_physical_block
    }

    pub(in crate::mir::builder) const fn exit_item(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopItemKeyV1 {
        self.exit_item
    }

    pub(in crate::mir::builder) const fn terminal_block(&self) -> BasicBlockId {
        self.terminal_block
    }

    pub(in crate::mir::builder) const fn target_function(&self) -> RegionId {
        self.target_function
    }

    pub(in crate::mir::builder) const fn join_target(&self) -> LoopJoinBranchExitTargetV2 {
        self.join_target
    }

    pub(in crate::mir::builder) const fn physical_value(&self) -> ValueId {
        self.physical_value
    }

    pub(in crate::mir::builder) fn segment_brand(
        &self,
    ) -> super::super::common_v2_segment_block_allocation::SegmentBlockAllocationBrandV1 {
        self.segment_brand.clone()
    }
}

impl<'source, 'envelope> CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    /// Consume the existing co-seal, segment allocation, and continuation
    /// target as one physical Return-read receipt. The canonical identity read
    /// and Completion/identity mark are the only effects in this slice.
    pub(in crate::mir::builder) fn with_return_read_physical_receipt<'receipt, R>(
        &'receipt mut self,
        builder: &mut MirBuilder,
        segment_receipt: &PreparedSegmentBlockReceiptV1,
        callback: impl FnOnce(
            &mut MirBuilder,
            CommonV2ReturnReadPhysicalReceiptV1<'receipt, 'source, 'envelope>,
        ) -> Result<R, String>,
    ) -> Result<R, ReturnReadPhysicalReceiptRejectV1> {
        if self.return_read_physical_issued {
            return Err(ReturnReadPhysicalReceiptRejectV1::AlreadyIssued);
        }
        let owner = self.session.owner();
        let co_seal: &CommonV2ReturnReadCoSealRefV1<'_> = self.envelope.return_read_co_seal();
        if co_seal.owner() != owner || self.envelope.owner() != owner {
            return Err(ReturnReadPhysicalReceiptRejectV1::OwnerMismatch);
        }
        let stamp_owner = self
            .session
            .physical_entry_stamp()
            .map_err(|_| ReturnReadPhysicalReceiptRejectV1::MissingPhysicalEntryStamp)?
            .owner();
        if stamp_owner != owner || segment_receipt.owner() != owner {
            return Err(ReturnReadPhysicalReceiptRejectV1::OwnerMismatch);
        }
        if !self.session.owns_segment_receipt(segment_receipt) {
            return Err(ReturnReadPhysicalReceiptRejectV1::SegmentScopeMismatch);
        }

        let return_segment = self
            .envelope
            .layout()
            .segment_for_block(co_seal.return_block())
            .ok_or(ReturnReadPhysicalReceiptRejectV1::CoSealMismatch)?;
        let if_segment = self
            .envelope
            .layout()
            .segment_for_block(co_seal.if_block())
            .ok_or(ReturnReadPhysicalReceiptRejectV1::CoSealMismatch)?;
        if return_segment.split_ordinal() != co_seal.return_split_ordinal()
            || if_segment.split_ordinal() != co_seal.if_split_ordinal()
            || !return_segment.items().contains(&co_seal.return_item())
            || !return_segment.items().contains(&co_seal.join_exit_item())
            || !if_segment.items().contains(&co_seal.if_item())
        {
            return Err(ReturnReadPhysicalReceiptRejectV1::CoSealMismatch);
        }

        fn unique_row(
            receipt: &PreparedSegmentBlockReceiptV1,
            block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
        ) -> Result<SegmentBlockReceiptRowV1, ReturnReadPhysicalReceiptRejectV1> {
            let mut rows = receipt
                .rows()
                .iter()
                .filter(|row| row.logical_block() == block);
            let Some(row) = rows.next() else {
                return Err(ReturnReadPhysicalReceiptRejectV1::SegmentMissing);
            };
            if rows.next().is_some() {
                return Err(ReturnReadPhysicalReceiptRejectV1::SegmentDuplicate);
            }
            Ok(*row)
        }

        let return_row = unique_row(segment_receipt, co_seal.return_block())?;
        let if_row = unique_row(segment_receipt, co_seal.if_block())?;
        if return_row.split_ordinal() != co_seal.return_split_ordinal()
            || if_row.split_ordinal() != co_seal.if_split_ordinal()
            || return_row.loop_key() != if_row.loop_key()
        {
            return Err(ReturnReadPhysicalReceiptRejectV1::SegmentMismatch);
        }

        // Poison this unpublished session before allocating or reading. Any
        // later error is terminal to the outer transaction, not a retry path.
        self.return_read_physical_issued = true;
        let placement = issue_if_continuation_target(
            &mut self.session,
            self.envelope,
            segment_receipt,
            builder,
            |_, target| {
                Ok(OwnedContinuationPlacement {
                    if_item: target.if_item(),
                    continuation: target.continuation(),
                    source_block: target.source_block(),
                    source_split_ordinal: target.source_split_ordinal(),
                    physical_block: target.physical_block(),
                    stamp_owner: target.stamp_owner(),
                })
            },
        )
        .map_err(ReturnReadPhysicalReceiptRejectV1::Target)?;
        let continuation = co_seal.continuation();
        if placement.if_item != co_seal.if_item()
            || placement.continuation != continuation
            || placement.source_block != co_seal.if_block()
            || placement.source_split_ordinal != co_seal.if_split_ordinal()
            || placement.stamp_owner != owner
        {
            return Err(ReturnReadPhysicalReceiptRejectV1::TargetMismatch);
        }

        let source = co_seal.source_binding();
        let binding = source.source_binding();
        self.session
            .identity
            .claim_variable_use_binding(source.return_value(), binding)
            .map_err(ReturnReadPhysicalReceiptRejectV1::CanonicalRead)?;
        let read = if self.session.physical_entry_seal_deferred() {
            let deferred_entry = self
                .session
                .physical_execution_entry(builder)
                .map_err(ReturnReadPhysicalReceiptRejectV1::CanonicalRead)?;
            self.session
                .identity
                .read_entry_receipt_with_deferred_entry(
                    builder,
                    &mut self.session.phis,
                    return_row.physical_block(),
                    binding,
                    deferred_entry,
                )
        } else {
            self.session.identity.read_entry_receipt(
                builder,
                &mut self.session.phis,
                return_row.physical_block(),
                binding,
            )
        }
        .map_err(ReturnReadPhysicalReceiptRejectV1::CanonicalRead)?;
        if read.owner() != owner
            || read.binding() != binding
            || read.physical_block() != return_row.physical_block()
        {
            return Err(ReturnReadPhysicalReceiptRejectV1::CanonicalReadMismatch);
        }
        let found = builder
            .function_state
            .type_ctx
            .get_type(read.physical_value())
            .cloned();
        if found != Some(MirType::Integer) {
            self.session
                .publish_physical_value_type(builder, read.physical_value(), MirType::Integer)
                .map_err(|_| ReturnReadPhysicalReceiptRejectV1::ReturnTypeMismatch(found))?;
        }
        self.session
            .completion
            .claim_explicit_return(
                source.return_site(),
                co_seal.target_function(),
                return_row.physical_block(),
                read.physical_value(),
            )
            .map_err(ReturnReadPhysicalReceiptRejectV1::Completion)?;
        self.session
            .identity
            .mark_return(ResolvedExitSiteV1::Statement(source.return_site().clone()))
            .map_err(ReturnReadPhysicalReceiptRejectV1::Identity)?;

        let receipt = CommonV2ReturnReadPhysicalReceiptV1 {
            _session: self,
            owner,
            binding,
            return_item: co_seal.return_item(),
            logical_result: co_seal.return_value(),
            then_block: co_seal.return_block(),
            then_split_ordinal: return_row.split_ordinal(),
            then_physical_block: return_row.physical_block(),
            if_item: co_seal.if_item(),
            if_block: co_seal.if_block(),
            if_condition: co_seal.if_condition(),
            if_split_ordinal: if_row.split_ordinal(),
            if_physical_block: if_row.physical_block(),
            continuation,
            continuation_physical_block: placement.physical_block,
            exit_item: co_seal.join_exit_item(),
            terminal_block: return_row.physical_block(),
            target_function: co_seal.target_function(),
            join_target: co_seal.join_target(),
            physical_value: read.physical_value(),
            segment_brand: segment_receipt.brand(),
        };
        callback(builder, receipt).map_err(ReturnReadPhysicalReceiptRejectV1::Callback)
    }
}
