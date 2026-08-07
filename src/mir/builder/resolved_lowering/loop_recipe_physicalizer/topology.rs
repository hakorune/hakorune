//! Caller-zero common Loop topology physicalizer.
//!
//! P0 deliberately allocates only the recursive physical block skeleton and
//! returns the root After continuation. Operation emission is not part of
//! this module until an item-keyed exact source/effect projection exists.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::VerifiedLoopPhysicalBoundaryV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ReadyLoopEntryRowV1 {
    key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    binding: BindingRefV1,
    value: ValueId,
}

impl ReadyLoopEntryRowV1 {
    pub(super) const fn new(
        key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Self {
        Self {
            key,
            binding,
            value,
        }
    }
}

#[derive(Debug)]
pub(super) struct ReadyLoopEntryV1 {
    owner: FunctionOwnerIdV1,
    preheader: BasicBlockId,
    pub(super) rows: Box<[ReadyLoopEntryRowV1]>,
}

impl ReadyLoopEntryV1 {
    pub(super) fn new_for_test(
        owner: FunctionOwnerIdV1,
        preheader: BasicBlockId,
        rows: Vec<ReadyLoopEntryRowV1>,
    ) -> Self {
        Self {
            owner,
            preheader,
            rows: rows.into_boxed_slice(),
        }
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn preheader(&self) -> BasicBlockId {
        self.preheader
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum LoopPhysicalBlockRoleV1 {
    Preheader,
    Header,
    Body,
    Step,
    After,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LoopPhysicalBlockRowV1 {
    loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    logical_block: Option<crate::mir::loop_recipe_contract::LoopBlockKeyV1>,
    role: LoopPhysicalBlockRoleV1,
    physical_block: BasicBlockId,
}

impl LoopPhysicalBlockRowV1 {
    pub(super) const fn new(
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
        logical_block: Option<crate::mir::loop_recipe_contract::LoopBlockKeyV1>,
        role: LoopPhysicalBlockRoleV1,
        physical_block: BasicBlockId,
    ) -> Self {
        Self {
            loop_key,
            logical_block,
            role,
            physical_block,
        }
    }

    pub(super) const fn loop_key(self) -> crate::mir::loop_recipe_contract::LoopNodeKeyV1 {
        self.loop_key
    }

    pub(super) const fn logical_block(
        self,
    ) -> Option<crate::mir::loop_recipe_contract::LoopBlockKeyV1> {
        self.logical_block
    }

    pub(super) const fn role(self) -> LoopPhysicalBlockRoleV1 {
        self.role
    }

    pub(super) const fn physical_block(self) -> BasicBlockId {
        self.physical_block
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopPhysicalBlockReceiptRejectV1 {
    ForeignLoop {
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    },
    DuplicatePlacement {
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
        role: LoopPhysicalBlockRoleV1,
    },
    DuplicateLogicalBlock {
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
        block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    },
    MissingRole {
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
        role: LoopPhysicalBlockRoleV1,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct LoopPhysicalBlockReceiptV1 {
    owner: FunctionOwnerIdV1,
    preheader: BasicBlockId,
    rows: Box<[LoopPhysicalBlockRowV1]>,
}

impl LoopPhysicalBlockReceiptV1 {
    pub(super) fn issue(
        owner: FunctionOwnerIdV1,
        preheader: BasicBlockId,
        loop_keys: &[crate::mir::loop_recipe_contract::LoopNodeKeyV1],
        rows: Vec<LoopPhysicalBlockRowV1>,
    ) -> Result<Self, LoopPhysicalBlockReceiptRejectV1> {
        let expected_loops = loop_keys.iter().copied().collect::<BTreeSet<_>>();
        let mut placements = BTreeSet::new();
        let mut logical_blocks = BTreeSet::new();
        for row in &rows {
            if !expected_loops.contains(&row.loop_key) {
                return Err(LoopPhysicalBlockReceiptRejectV1::ForeignLoop {
                    loop_key: row.loop_key,
                });
            }
            if !placements.insert((row.loop_key, row.role)) {
                return Err(LoopPhysicalBlockReceiptRejectV1::DuplicatePlacement {
                    loop_key: row.loop_key,
                    role: row.role,
                });
            }
            if let Some(block) = row.logical_block {
                if !logical_blocks.insert((row.loop_key, block)) {
                    return Err(LoopPhysicalBlockReceiptRejectV1::DuplicateLogicalBlock {
                        loop_key: row.loop_key,
                        block,
                    });
                }
            }
        }
        for &loop_key in loop_keys {
            for role in [
                LoopPhysicalBlockRoleV1::Preheader,
                LoopPhysicalBlockRoleV1::Header,
                LoopPhysicalBlockRoleV1::Body,
                LoopPhysicalBlockRoleV1::Step,
                LoopPhysicalBlockRoleV1::After,
            ] {
                if !placements.contains(&(loop_key, role)) {
                    return Err(LoopPhysicalBlockReceiptRejectV1::MissingRole { loop_key, role });
                }
            }
        }
        Ok(Self {
            owner,
            preheader,
            rows: rows.into_boxed_slice(),
        })
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn preheader(&self) -> BasicBlockId {
        self.preheader
    }

    pub(super) fn rows(&self) -> &[LoopPhysicalBlockRowV1] {
        &self.rows
    }

    pub(super) fn lookup(
        &self,
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
        role: LoopPhysicalBlockRoleV1,
    ) -> Option<BasicBlockId> {
        self.rows
            .iter()
            .find(|row| row.loop_key == loop_key && row.role == role)
            .map(|row| row.physical_block)
    }

    pub(super) fn lookup_logical(
        &self,
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
        block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    ) -> Option<BasicBlockId> {
        self.rows
            .iter()
            .find(|row| row.loop_key == loop_key && row.logical_block == Some(block))
            .map(|row| row.physical_block)
    }

    pub(super) fn loop_count(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.loop_key)
            .collect::<BTreeSet<_>>()
            .len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoopPhysicalizerRejectV1 {
    MissingFunction,
    PreheaderMissing(BasicBlockId),
    EntryOwnerMismatch,
    EntryKeyMismatch(crate::mir::loop_recipe_contract::LoopValueKeyV1),
    EntryBindingOwnerMismatch(BindingRefV1),
    EntryBindingMissing(BindingRefV1),
    ParentTopologyMissing(crate::mir::loop_recipe_contract::LoopNodeKeyV1),
    AfterOwnerMismatch,
    BlockReceipt(LoopPhysicalBlockReceiptRejectV1),
    BlockAllocation(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AllocatedLoopBlocksV1 {
    loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    preheader: BasicBlockId,
    header: BasicBlockId,
    body: BasicBlockId,
    step: BasicBlockId,
    after: BasicBlockId,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct LoopAfterContinuationReceiptV1 {
    root_loop: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    block_receipt: LoopPhysicalBlockReceiptV1,
}

impl LoopAfterContinuationReceiptV1 {
    pub(super) fn owner(&self) -> FunctionOwnerIdV1 {
        self.block_receipt.owner()
    }

    pub(super) fn root_after(&self) -> BasicBlockId {
        self.block_receipt
            .lookup(self.root_loop, LoopPhysicalBlockRoleV1::After)
            .expect("validated root After row")
    }

    pub(super) fn root_loop(&self) -> crate::mir::loop_recipe_contract::LoopNodeKeyV1 {
        self.root_loop
    }

    pub(super) fn loop_count(&self) -> usize {
        self.block_receipt.loop_count()
    }

    pub(super) fn after_for(
        &self,
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    ) -> Option<BasicBlockId> {
        self.block_receipt
            .lookup(loop_key, LoopPhysicalBlockRoleV1::After)
    }

    pub(super) fn preheader_for(
        &self,
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    ) -> Option<BasicBlockId> {
        self.block_receipt
            .lookup(loop_key, LoopPhysicalBlockRoleV1::Preheader)
    }

    pub(super) fn block_receipt(&self) -> &LoopPhysicalBlockReceiptV1 {
        &self.block_receipt
    }
}

/// Borrowed canonical services. This is a service bundle, not a second
/// physical/SSA owner; callers pass the existing session's CFG service.
pub(super) struct LoopPhysicalServicesV1<'a> {
    pub(super) builder: &'a mut MirBuilder,
    cfg: &'a mut CanonicalCfgSessionV1,
}

impl<'a> LoopPhysicalServicesV1<'a> {
    pub(super) fn new(builder: &'a mut MirBuilder, cfg: &'a mut CanonicalCfgSessionV1) -> Self {
        Self { builder, cfg }
    }

    fn allocate_block(&mut self) -> Result<BasicBlockId, LoopPhysicalizerRejectV1> {
        let block = self.builder.next_block_id();
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .ok_or(LoopPhysicalizerRejectV1::MissingFunction)?;
        self.cfg
            .create_block(function, block)
            .map_err(|error| LoopPhysicalizerRejectV1::BlockAllocation(error.to_string()))?;
        Ok(block)
    }
}

pub(super) fn physicalize_topology_v1(
    boundary: VerifiedLoopPhysicalBoundaryV1,
    entry: ReadyLoopEntryV1,
    services: &mut LoopPhysicalServicesV1<'_>,
) -> Result<LoopAfterContinuationReceiptV1, LoopPhysicalizerRejectV1> {
    let owner = boundary.core().owner();
    validate_entry(&boundary, &entry, services)?;
    if boundary.after().loop_key() != boundary.recipe().root_loop() {
        return Err(LoopPhysicalizerRejectV1::AfterOwnerMismatch);
    }

    let recipe = boundary.recipe();
    let mut physical = BTreeMap::new();
    for node in &recipe.as_recipe().loops {
        let preheader = match node.parent {
            Some(parent) => physical
                .get(&parent)
                .map(|blocks: &AllocatedLoopBlocksV1| blocks.body)
                .ok_or(LoopPhysicalizerRejectV1::ParentTopologyMissing(parent))?,
            None => entry.preheader,
        };
        let blocks = AllocatedLoopBlocksV1 {
            loop_key: node.key,
            preheader,
            header: services.allocate_block()?,
            body: services.allocate_block()?,
            step: services.allocate_block()?,
            after: services.allocate_block()?,
        };
        physical.insert(node.key, blocks);
    }

    let mut rows = Vec::new();
    for node in &recipe.as_recipe().loops {
        let blocks = physical
            .get(&node.key)
            .copied()
            .ok_or(LoopPhysicalizerRejectV1::AfterOwnerMismatch)?;
        let condition_block = match node.condition {
            crate::mir::loop_recipe_contract::LoopConditionV1::Always => None,
            crate::mir::loop_recipe_contract::LoopConditionV1::Predicate { block, .. } => {
                Some(block)
            }
        };
        rows.extend([
            LoopPhysicalBlockRowV1::new(
                node.key,
                None,
                LoopPhysicalBlockRoleV1::Preheader,
                blocks.preheader,
            ),
            LoopPhysicalBlockRowV1::new(
                node.key,
                condition_block,
                LoopPhysicalBlockRoleV1::Header,
                blocks.header,
            ),
            LoopPhysicalBlockRowV1::new(
                node.key,
                Some(node.body),
                LoopPhysicalBlockRoleV1::Body,
                blocks.body,
            ),
            LoopPhysicalBlockRowV1::new(node.key, None, LoopPhysicalBlockRoleV1::Step, blocks.step),
            LoopPhysicalBlockRowV1::new(
                node.key,
                None,
                LoopPhysicalBlockRoleV1::After,
                blocks.after,
            ),
        ]);
    }
    let loop_keys = recipe
        .as_recipe()
        .loops
        .iter()
        .map(|node| node.key)
        .collect::<Vec<_>>();
    let block_receipt = LoopPhysicalBlockReceiptV1::issue(owner, entry.preheader, &loop_keys, rows)
        .map_err(LoopPhysicalizerRejectV1::BlockReceipt)?;
    Ok(LoopAfterContinuationReceiptV1 {
        root_loop: recipe.root_loop(),
        block_receipt,
    })
}

fn validate_entry(
    boundary: &VerifiedLoopPhysicalBoundaryV1,
    entry: &ReadyLoopEntryV1,
    services: &LoopPhysicalServicesV1<'_>,
) -> Result<(), LoopPhysicalizerRejectV1> {
    let owner = boundary.core().owner();
    if entry.owner != owner {
        return Err(LoopPhysicalizerRejectV1::EntryOwnerMismatch);
    }
    if entry.rows.iter().any(|row| row.binding.owner() != owner) {
        let binding = entry
            .rows
            .iter()
            .find(|row| row.binding.owner() != owner)
            .expect("row found")
            .binding;
        return Err(LoopPhysicalizerRejectV1::EntryBindingOwnerMismatch(binding));
    }
    let function = services
        .builder
        .function_state
        .current_function
        .as_ref()
        .ok_or(LoopPhysicalizerRejectV1::MissingFunction)?;
    if function.get_block(entry.preheader).is_none() {
        return Err(LoopPhysicalizerRejectV1::PreheaderMissing(entry.preheader));
    }
    let expected = boundary
        .recipe()
        .as_recipe()
        .inputs
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    for row in &entry.rows {
        if !seen.insert(row.key) || !expected.contains(&row.key) {
            return Err(LoopPhysicalizerRejectV1::EntryKeyMismatch(row.key));
        }
        if !boundary
            .core()
            .binding_relations()
            .iter()
            .any(|relation| relation.source_binding() == row.binding)
        {
            return Err(LoopPhysicalizerRejectV1::EntryBindingMissing(row.binding));
        }
    }
    if seen != expected {
        return Err(LoopPhysicalizerRejectV1::EntryKeyMismatch(
            expected
                .difference(&seen)
                .next()
                .copied()
                .expect("set mismatch has missing key"),
        ));
    }
    Ok(())
}
