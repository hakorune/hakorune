//! Builder-free physical segment layout derived from the verified Recipe.
//!
//! This is a private compatibility product.  `LoopRecipeV1` and
//! `LoopJoinSigV1` remain the logical authorities; this module only derives
//! ordered segments, item placement, and nested-loop resume targets.

use std::collections::{BTreeMap, BTreeSet};

use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::join_sig::{
    LoopJoinBranchArmTransferRefV1, LoopJoinEdgeRoleV1, LoopJoinLogicalTransferRejectV1,
    LoopJoinLogicalTransferViewV1, LoopJoinPortV1,
};
use super::operation_physical_demand::PreparedLoopOperationProgramV1;
use super::physical_transfer::{
    bind_backedge, bind_nested_loop, bind_predicate, LoopPhysicalTransferBindingRejectV1,
};
use super::schema::{LoopConditionV1, LoopRecipeItemV1, LoopRecipeV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct LoopPhysicalSegmentKeyV1 {
    loop_key: LoopNodeKeyV1,
    block: LoopBlockKeyV1,
    ordinal: u32,
}

impl LoopPhysicalSegmentKeyV1 {
    const fn new(loop_key: LoopNodeKeyV1, block: LoopBlockKeyV1, ordinal: u32) -> Self {
        Self {
            loop_key,
            block,
            ordinal,
        }
    }

    pub(crate) const fn loop_key(self) -> LoopNodeKeyV1 {
        self.loop_key
    }

    pub(crate) const fn block(self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(crate) const fn ordinal(self) -> u32 {
        self.ordinal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPhysicalTargetV1 {
    Segment(LoopPhysicalSegmentKeyV1),
    OpenRootAfter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPhysicalTransferV1 {
    Jump {
        target: LoopPhysicalTargetV1,
    },
    Predicate {
        condition: LoopValueKeyV1,
        on_true: LoopPhysicalSegmentKeyV1,
        on_false: LoopPhysicalTargetV1,
    },
    OpenNestedLoop {
        loop_key: LoopNodeKeyV1,
        entry: LoopPhysicalSegmentKeyV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopPhysicalSegmentRoleV1 {
    Header,
    Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedLoopControlSegmentV1 {
    key: LoopPhysicalSegmentKeyV1,
    role: LoopPhysicalSegmentRoleV1,
    operations: Box<[LoopItemKeyV1]>,
    transfer: LoopPhysicalTransferV1,
}

impl PreparedLoopControlSegmentV1 {
    pub(crate) const fn key(&self) -> LoopPhysicalSegmentKeyV1 {
        self.key
    }

    pub(crate) const fn role(&self) -> LoopPhysicalSegmentRoleV1 {
        self.role
    }

    pub(crate) fn operations(&self) -> &[LoopItemKeyV1] {
        &self.operations
    }

    pub(crate) const fn transfer(&self) -> LoopPhysicalTransferV1 {
        self.transfer
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopPhysicalLayoutCoverageReceiptV1 {
    item_count: usize,
    operation_count: usize,
    segment_count: usize,
}

impl LoopPhysicalLayoutCoverageReceiptV1 {
    pub(crate) const fn item_count(self) -> usize {
        self.item_count
    }

    pub(crate) const fn operation_count(self) -> usize {
        self.operation_count
    }

    pub(crate) const fn segment_count(self) -> usize {
        self.segment_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopPhysicalLayoutRejectV1 {
    MissingLoop(LoopNodeKeyV1),
    MissingBlock(LoopBlockKeyV1),
    DuplicateLoop(LoopNodeKeyV1),
    DuplicateBlock(LoopBlockKeyV1),
    DuplicateItem(LoopItemKeyV1),
    UnsupportedAlways(LoopNodeKeyV1),
    UnsupportedExit(LoopItemKeyV1),
    BackedgeMissing(LoopNodeKeyV1),
    BranchConditionMismatch(LoopItemKeyV1),
    BranchContinuationMismatch(LoopItemKeyV1),
    BranchExitMismatch(LoopItemKeyV1),
    BranchCoverage {
        expected: Box<[(LoopNodeKeyV1, LoopItemKeyV1)]>,
        consumed: Box<[(LoopNodeKeyV1, LoopItemKeyV1)]>,
    },
    ScheduleOrderMismatch {
        expected: Box<[LoopItemKeyV1]>,
        found: Box<[LoopItemKeyV1]>,
    },
    IncompleteCoverage {
        expected: usize,
        found: usize,
    },
    Transfer(LoopJoinLogicalTransferRejectV1),
    TransferBinding(LoopPhysicalTransferBindingRejectV1),
}

#[derive(Debug)]
pub(crate) struct PreparedLoopPhysicalLayoutV1 {
    program: PreparedLoopOperationProgramV1,
    entry_segment: LoopPhysicalSegmentKeyV1,
    segments: Box<[PreparedLoopControlSegmentV1]>,
    coverage: LoopPhysicalLayoutCoverageReceiptV1,
}

impl PreparedLoopPhysicalLayoutV1 {
    pub(crate) fn from_program(
        program: PreparedLoopOperationProgramV1,
    ) -> Result<Self, LoopPhysicalLayoutRejectV1> {
        let transfer_view = program
            .demand()
            .operation_effect()
            .core()
            .join_sig()
            .logical_transfer_view();
        let recipe = program
            .demand()
            .operation_effect()
            .core()
            .recipe()
            .as_recipe();
        let (entry_segment, segments, visited_items, operation_items) = {
            let mut builder = LayoutBuilder::new(recipe, &transfer_view);
            let entry_segment = builder.entry_key(recipe.root_loop)?;
            builder.build_loop(recipe.root_loop, LoopPhysicalTargetV1::OpenRootAfter)?;
            let (segments, visited_items, operation_items) = builder.finish()?;
            (entry_segment, segments, visited_items, operation_items)
        };
        let schedule = program
            .schedule()
            .iter()
            .map(|row| row.item())
            .collect::<Vec<_>>();
        let derived = operation_items.iter().copied().collect::<Vec<_>>();
        if schedule != derived {
            return Err(LoopPhysicalLayoutRejectV1::ScheduleOrderMismatch {
                expected: derived.into_boxed_slice(),
                found: schedule.into_boxed_slice(),
            });
        }
        let expected_items = recipe
            .items
            .iter()
            .map(|row| row.key)
            .collect::<BTreeSet<_>>();
        if visited_items != expected_items {
            return Err(LoopPhysicalLayoutRejectV1::IncompleteCoverage {
                expected: recipe.items.len(),
                found: visited_items.len(),
            });
        }
        Ok(Self {
            coverage: LoopPhysicalLayoutCoverageReceiptV1 {
                item_count: visited_items.len(),
                operation_count: operation_items.len(),
                segment_count: segments.len(),
            },
            program,
            entry_segment,
            segments: segments.into_boxed_slice(),
        })
    }

    pub(crate) fn program(&self) -> &PreparedLoopOperationProgramV1 {
        &self.program
    }

    pub(crate) fn segments(&self) -> &[PreparedLoopControlSegmentV1] {
        &self.segments
    }

    pub(crate) const fn entry_segment(&self) -> LoopPhysicalSegmentKeyV1 {
        self.entry_segment
    }

    pub(crate) const fn coverage(&self) -> LoopPhysicalLayoutCoverageReceiptV1 {
        self.coverage
    }
}

struct LayoutBuilder<'a> {
    recipe: &'a LoopRecipeV1,
    transfers: &'a LoopJoinLogicalTransferViewV1<'a>,
    item_rows: BTreeMap<LoopItemKeyV1, LoopRecipeItemV1>,
    segments: Vec<PreparedLoopControlSegmentV1>,
    visited_loops: BTreeSet<LoopNodeKeyV1>,
    visited_blocks: BTreeSet<LoopBlockKeyV1>,
    visited_items: BTreeSet<LoopItemKeyV1>,
    visited_branches: BTreeSet<(LoopNodeKeyV1, LoopItemKeyV1)>,
    operation_items: Vec<LoopItemKeyV1>,
    after_targets: BTreeMap<LoopNodeKeyV1, LoopPhysicalTargetV1>,
}

impl<'a> LayoutBuilder<'a> {
    fn new(recipe: &'a LoopRecipeV1, transfers: &'a LoopJoinLogicalTransferViewV1<'a>) -> Self {
        Self {
            recipe,
            transfers,
            item_rows: recipe
                .items
                .iter()
                .map(|row| (row.key, row.item.clone()))
                .collect(),
            segments: Vec::new(),
            visited_loops: BTreeSet::new(),
            visited_blocks: BTreeSet::new(),
            visited_items: BTreeSet::new(),
            visited_branches: BTreeSet::new(),
            operation_items: Vec::new(),
            after_targets: BTreeMap::new(),
        }
    }

    fn finish(
        self,
    ) -> Result<
        (
            Vec<PreparedLoopControlSegmentV1>,
            BTreeSet<LoopItemKeyV1>,
            Vec<LoopItemKeyV1>,
        ),
        LoopPhysicalLayoutRejectV1,
    > {
        let expected_branches = self
            .transfers
            .branches()
            .iter()
            .map(|branch| (branch.owner_loop, branch.if_item))
            .collect::<BTreeSet<_>>();
        if expected_branches != self.visited_branches {
            return Err(LoopPhysicalLayoutRejectV1::BranchCoverage {
                expected: expected_branches.into_iter().collect(),
                consumed: self.visited_branches.into_iter().collect(),
            });
        }
        if self.visited_loops.len() != self.recipe.loops.len()
            || self.visited_blocks.len() != self.recipe.blocks.len()
            || self.visited_items.len() != self.recipe.items.len()
        {
            return Err(LoopPhysicalLayoutRejectV1::IncompleteCoverage {
                expected: self.recipe.items.len(),
                found: self.visited_items.len(),
            });
        }
        Ok((self.segments, self.visited_items, self.operation_items))
    }

    fn entry_key(
        &self,
        loop_key: LoopNodeKeyV1,
    ) -> Result<LoopPhysicalSegmentKeyV1, LoopPhysicalLayoutRejectV1> {
        let enter = self
            .transfers
            .require(loop_key, LoopJoinEdgeRoleV1::Enter)
            .map_err(LoopPhysicalLayoutRejectV1::Transfer)?;
        require_ports(
            enter.loop_key,
            enter.role,
            enter.from,
            enter.to,
            LoopJoinPortV1::Preheader,
            LoopJoinPortV1::Header,
        )?;
        let node = self
            .recipe
            .loops
            .iter()
            .find(|row| row.key == loop_key)
            .ok_or(LoopPhysicalLayoutRejectV1::MissingLoop(loop_key))?;
        match node.condition {
            LoopConditionV1::Predicate { .. } => {
                let predicate = self
                    .transfers
                    .require(loop_key, LoopJoinEdgeRoleV1::PredicateTrue)
                    .map_err(LoopPhysicalLayoutRejectV1::Transfer)?;
                let Some((block, _)) = predicate.condition else {
                    return Err(LoopPhysicalLayoutRejectV1::UnsupportedAlways(loop_key));
                };
                Ok(LoopPhysicalSegmentKeyV1::new(loop_key, block, 0))
            }
            LoopConditionV1::Always => {
                let body_entry = self
                    .transfers
                    .require(loop_key, LoopJoinEdgeRoleV1::BodyEntry)
                    .map_err(LoopPhysicalLayoutRejectV1::Transfer)?;
                require_ports(
                    body_entry.loop_key,
                    body_entry.role,
                    body_entry.from,
                    body_entry.to,
                    LoopJoinPortV1::Header,
                    LoopJoinPortV1::Body,
                )?;
                Ok(LoopPhysicalSegmentKeyV1::new(loop_key, node.body, 0))
            }
        }
    }

    fn build_loop(
        &mut self,
        loop_key: LoopNodeKeyV1,
        after_target: LoopPhysicalTargetV1,
    ) -> Result<(), LoopPhysicalLayoutRejectV1> {
        if !self.visited_loops.insert(loop_key) {
            return Err(LoopPhysicalLayoutRejectV1::DuplicateLoop(loop_key));
        }
        let node = self
            .recipe
            .loops
            .iter()
            .find(|row| row.key == loop_key)
            .ok_or(LoopPhysicalLayoutRejectV1::MissingLoop(loop_key))?;
        self.after_targets.insert(loop_key, after_target);
        let body_entry = LoopPhysicalSegmentKeyV1::new(loop_key, node.body, 0);
        let entry = match node.condition {
            LoopConditionV1::Predicate { .. } => {
                let predicate_true = self
                    .transfers
                    .require(loop_key, LoopJoinEdgeRoleV1::PredicateTrue)
                    .map_err(LoopPhysicalLayoutRejectV1::Transfer)?;
                let predicate_false = self
                    .transfers
                    .require(loop_key, LoopJoinEdgeRoleV1::PredicateFalse)
                    .map_err(LoopPhysicalLayoutRejectV1::Transfer)?;
                let Some((condition_block, _)) = predicate_true.condition else {
                    return Err(LoopPhysicalLayoutRejectV1::UnsupportedAlways(loop_key));
                };
                let predicate =
                    bind_predicate(predicate_true, predicate_false, body_entry, after_target)
                        .map_err(LoopPhysicalLayoutRejectV1::TransferBinding)?;
                self.build_block(
                    loop_key,
                    condition_block,
                    LoopPhysicalSegmentRoleV1::Header,
                    BlockTailV1::Finish(predicate),
                    None,
                )?;
                LoopPhysicalSegmentKeyV1::new(loop_key, condition_block, 0)
            }
            LoopConditionV1::Always => {
                let body_entry_edge = self
                    .transfers
                    .require(loop_key, LoopJoinEdgeRoleV1::BodyEntry)
                    .map_err(LoopPhysicalLayoutRejectV1::Transfer)?;
                require_ports(
                    body_entry_edge.loop_key,
                    body_entry_edge.role,
                    body_entry_edge.from,
                    body_entry_edge.to,
                    LoopJoinPortV1::Header,
                    LoopJoinPortV1::Body,
                )?;
                body_entry
            }
        };
        let tail = match self
            .transfers
            .require(loop_key, LoopJoinEdgeRoleV1::Backedge)
        {
            Ok(backedge) => BlockTailV1::Finish(
                bind_backedge(backedge, LoopPhysicalTargetV1::Segment(entry))
                    .map_err(LoopPhysicalLayoutRejectV1::TransferBinding)?,
            ),
            Err(LoopJoinLogicalTransferRejectV1::MissingBoundary { .. }) => {
                BlockTailV1::DeadTail { loop_key }
            }
            Err(reject) => return Err(LoopPhysicalLayoutRejectV1::Transfer(reject)),
        };
        self.build_block(loop_key, node.body, LoopPhysicalSegmentRoleV1::Body, tail, None)
    }

    fn build_block(
        &mut self,
        loop_key: LoopNodeKeyV1,
        block_key: LoopBlockKeyV1,
        role: LoopPhysicalSegmentRoleV1,
        tail: BlockTailV1,
        expected_exit: Option<LoopItemKeyV1>,
    ) -> Result<(), LoopPhysicalLayoutRejectV1> {
        if !self.visited_blocks.insert(block_key) {
            return Err(LoopPhysicalLayoutRejectV1::DuplicateBlock(block_key));
        }
        let block = self
            .recipe
            .blocks
            .iter()
            .find(|row| row.key == block_key)
            .ok_or(LoopPhysicalLayoutRejectV1::MissingBlock(block_key))?;
        let items = block.items.clone();
        let mut ordinal = 0;
        let mut operations = Vec::new();
        let mut consumed_exit = None;
        let mut last_was_if = false;
        for (index, item) in items.iter().copied().enumerate() {
            last_was_if = false;
            if !self.visited_items.insert(item) {
                return Err(LoopPhysicalLayoutRejectV1::DuplicateItem(item));
            }
            match self.item_rows.get(&item).cloned() {
                Some(LoopRecipeItemV1::Operation { .. }) => {
                    self.operation_items.push(item);
                    operations.push(item);
                }
                Some(LoopRecipeItemV1::Loop { loop_key: child }) => {
                    let current = LoopPhysicalSegmentKeyV1::new(loop_key, block_key, ordinal);
                    let resume = LoopPhysicalSegmentKeyV1::new(loop_key, block_key, ordinal + 1);
                    let child_entry = self.entry_key(child)?;
                    let child_enter = self
                        .transfers
                        .require(child, LoopJoinEdgeRoleV1::Enter)
                        .map_err(LoopPhysicalLayoutRejectV1::Transfer)?;
                    let nested_transfer = bind_nested_loop(child_enter, child, child_entry)
                        .map_err(LoopPhysicalLayoutRejectV1::TransferBinding)?;
                    self.segments.push(PreparedLoopControlSegmentV1 {
                        key: current,
                        role,
                        operations: operations.into_boxed_slice(),
                        transfer: nested_transfer,
                    });
                    operations = Vec::new();
                    ordinal += 1;
                    self.build_loop(child, LoopPhysicalTargetV1::Segment(resume))?;
                }
                Some(LoopRecipeItemV1::If {
                    condition,
                    then_block,
                    else_block,
                }) => {
                    let branch = self
                        .transfers
                        .require_branch(loop_key, item)
                        .map_err(LoopPhysicalLayoutRejectV1::Transfer)?;
                    if !self.visited_branches.insert((loop_key, item)) {
                        return Err(LoopPhysicalLayoutRejectV1::BranchCoverage {
                            expected: vec![(loop_key, item)].into_boxed_slice(),
                            consumed: self.visited_branches.iter().copied().collect(),
                        });
                    }
                    if branch.condition != condition {
                        return Err(LoopPhysicalLayoutRejectV1::BranchConditionMismatch(item));
                    }
                    let continuation = items
                        .get(index + 1)
                        .map(|_| LoopPhysicalSegmentKeyV1::new(loop_key, block_key, ordinal + 1));
                    let expected_continuation = items.get(index + 1).copied().map(|next| {
                        super::join_sig::LoopJoinNextItemV1 {
                            block: block_key,
                            item: next,
                        }
                    });
                    let then_finish = branch_arm_finish(
                        branch.then_arm,
                        loop_key,
                        item,
                        expected_continuation,
                        continuation,
                        self,
                    )?;
                    let else_finish = branch_arm_finish(
                        branch.else_arm,
                        loop_key,
                        item,
                        expected_continuation,
                        continuation,
                        self,
                    )?;
                    if then_block == block_key
                        || else_block.is_some_and(|block| block == block_key || block == then_block)
                    {
                        return Err(LoopPhysicalLayoutRejectV1::BranchContinuationMismatch(item));
                    }
                    let then_target = LoopPhysicalSegmentKeyV1::new(loop_key, then_block, 0);
                    let else_target = match else_block {
                        Some(block) => LoopPhysicalTargetV1::Segment(
                            LoopPhysicalSegmentKeyV1::new(loop_key, block, 0),
                        ),
                        None => match else_finish {
                            BranchArmFinishV1::Fallthrough(target) => target,
                            BranchArmFinishV1::Exit { .. } => {
                                return Err(LoopPhysicalLayoutRejectV1::BranchExitMismatch(item));
                            }
                        },
                    };
                    if else_block.is_none()
                        && !matches!(
                            branch.else_arm,
                            LoopJoinBranchArmTransferRefV1::Fallthrough { .. }
                        )
                    {
                        return Err(LoopPhysicalLayoutRejectV1::BranchExitMismatch(item));
                    }
                    self.segments.push(PreparedLoopControlSegmentV1 {
                        key: LoopPhysicalSegmentKeyV1::new(loop_key, block_key, ordinal),
                        role,
                        operations: operations.into_boxed_slice(),
                        transfer: LoopPhysicalTransferV1::Predicate {
                            condition,
                            on_true: then_target,
                            on_false: else_target,
                        },
                    });
                    operations = Vec::new();
                    ordinal += 1;
                    last_was_if = true;
                    self.build_block(
                        loop_key,
                        then_block,
                        LoopPhysicalSegmentRoleV1::Body,
                        BlockTailV1::Finish(then_finish.transfer()),
                        then_finish.exit_item(),
                    )?;
                    if let Some(else_block) = else_block {
                        self.build_block(
                            loop_key,
                            else_block,
                            LoopPhysicalSegmentRoleV1::Body,
                            BlockTailV1::Finish(else_finish.transfer()),
                            else_finish.exit_item(),
                        )?;
                    }
                }
                Some(LoopRecipeItemV1::Exit { .. }) => {
                    if expected_exit != Some(item)
                        || items.last().copied() != Some(item)
                        || consumed_exit.replace(item).is_some()
                    {
                        return Err(LoopPhysicalLayoutRejectV1::UnsupportedExit(item));
                    }
                }
                None => {
                    return Err(LoopPhysicalLayoutRejectV1::MissingBlock(block_key));
                }
            }
        }
        if expected_exit != consumed_exit {
            return Err(LoopPhysicalLayoutRejectV1::BranchExitMismatch(
                expected_exit.unwrap_or_else(|| {
                    items
                        .last()
                        .copied()
                        .unwrap_or(LoopItemKeyV1::new(u32::MAX))
                }),
            ));
        }
        let transfer = match tail {
            BlockTailV1::Finish(transfer) => transfer,
            BlockTailV1::DeadTail { loop_key } => {
                // Bounded dead-tail admission: a loop without a Backedge edge
                // is physicalizable only when the body's final item is an
                // `if` whose arms both exit (no reachable continuation). The
                // arm blocks already carry the only exits, so the trailing
                // segment is skipped instead of minting a synthetic transfer.
                if !operations.is_empty() || !last_was_if {
                    return Err(LoopPhysicalLayoutRejectV1::BackedgeMissing(loop_key));
                }
                return Ok(());
            }
        };
        self.segments.push(PreparedLoopControlSegmentV1 {
            key: LoopPhysicalSegmentKeyV1::new(loop_key, block_key, ordinal),
            role,
            operations: operations.into_boxed_slice(),
            transfer,
        });
        Ok(())
    }
}

enum BlockTailV1 {
    Finish(LoopPhysicalTransferV1),
    DeadTail { loop_key: LoopNodeKeyV1 },
}

#[derive(Clone, Copy)]
enum BranchArmFinishV1 {
    Fallthrough(LoopPhysicalTargetV1),
    Exit {
        transfer: LoopPhysicalTransferV1,
        item: LoopItemKeyV1,
    },
}

impl BranchArmFinishV1 {
    fn transfer(&self) -> LoopPhysicalTransferV1 {
        match self {
            Self::Fallthrough(target) => LoopPhysicalTransferV1::Jump { target: *target },
            Self::Exit { transfer, .. } => *transfer,
        }
    }

    fn exit_item(&self) -> Option<LoopItemKeyV1> {
        match self {
            Self::Fallthrough(_) => None,
            Self::Exit { item, .. } => Some(*item),
        }
    }
}

fn branch_arm_finish(
    arm: LoopJoinBranchArmTransferRefV1<'_>,
    owner_loop: LoopNodeKeyV1,
    if_item: LoopItemKeyV1,
    expected_continuation: Option<super::join_sig::LoopJoinNextItemV1>,
    continuation_target: Option<LoopPhysicalSegmentKeyV1>,
    builder: &LayoutBuilder<'_>,
) -> Result<BranchArmFinishV1, LoopPhysicalLayoutRejectV1> {
    match arm {
        LoopJoinBranchArmTransferRefV1::Fallthrough { continuation, .. } => {
            if Some(continuation) != expected_continuation {
                return Err(LoopPhysicalLayoutRejectV1::BranchContinuationMismatch(
                    if_item,
                ));
            }
            let target = continuation_target.ok_or(
                LoopPhysicalLayoutRejectV1::BranchContinuationMismatch(if_item),
            )?;
            Ok(BranchArmFinishV1::Fallthrough(
                LoopPhysicalTargetV1::Segment(target),
            ))
        }
        LoopJoinBranchArmTransferRefV1::Exit(exit) => {
            // Bounded boundary: a branch-arm `continue` jumps back to the
            // owning loop's entry (condition block for predicate loops, body
            // for `loop(true)`); a branch-arm `break` jumps to the after
            // target recorded for its target loop (the root's OpenRootAfter
            // or an ancestor's resume segment). Return arms and
            // foreign-target continues stay typed-rejected.
            match exit.role {
                LoopJoinEdgeRoleV1::Continue if exit.target_loop == owner_loop => {
                    let entry = builder.entry_key(exit.target_loop)?;
                    Ok(BranchArmFinishV1::Exit {
                        transfer: LoopPhysicalTransferV1::Jump {
                            target: LoopPhysicalTargetV1::Segment(entry),
                        },
                        item: exit.exit_item,
                    })
                }
                LoopJoinEdgeRoleV1::Break => {
                    let target = builder
                        .after_targets
                        .get(&exit.target_loop)
                        .copied()
                        .ok_or(LoopPhysicalLayoutRejectV1::BranchExitMismatch(
                            if_item,
                        ))?;
                    Ok(BranchArmFinishV1::Exit {
                        transfer: LoopPhysicalTransferV1::Jump { target },
                        item: exit.exit_item,
                    })
                }
                _ => Err(LoopPhysicalLayoutRejectV1::BranchExitMismatch(if_item)),
            }
        }
    }
}

fn require_ports(
    loop_key: LoopNodeKeyV1,
    role: LoopJoinEdgeRoleV1,
    from: LoopJoinPortV1,
    to: LoopJoinPortV1,
    expected_from: LoopJoinPortV1,
    expected_to: LoopJoinPortV1,
) -> Result<(), LoopPhysicalLayoutRejectV1> {
    if from == expected_from && to == expected_to {
        return Ok(());
    }
    Err(LoopPhysicalLayoutRejectV1::TransferBinding(
        LoopPhysicalTransferBindingRejectV1::PortMismatch {
            loop_key,
            role,
            expected_from,
            expected_to,
            found_from: from,
            found_to: to,
        },
    ))
}

#[cfg(test)]
#[path = "physical_layout_tests.rs"]
mod tests;
