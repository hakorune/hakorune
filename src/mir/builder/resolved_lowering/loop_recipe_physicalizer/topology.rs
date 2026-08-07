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
    BlockAllocation(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoopPhysicalBlocksV1 {
    loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    preheader: BasicBlockId,
    header: BasicBlockId,
    body: BasicBlockId,
    step: BasicBlockId,
    after: BasicBlockId,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct LoopAfterContinuationReceiptV1 {
    owner: FunctionOwnerIdV1,
    root_loop: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    root_after: BasicBlockId,
    loops: Box<[LoopPhysicalBlocksV1]>,
}

impl LoopAfterContinuationReceiptV1 {
    pub(super) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) fn root_after(&self) -> BasicBlockId {
        self.root_after
    }

    pub(super) fn root_loop(&self) -> crate::mir::loop_recipe_contract::LoopNodeKeyV1 {
        self.root_loop
    }

    pub(super) fn loop_count(&self) -> usize {
        self.loops.len()
    }

    pub(super) fn after_for(
        &self,
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    ) -> Option<BasicBlockId> {
        self.loops
            .iter()
            .find(|blocks| blocks.loop_key == loop_key)
            .map(|blocks| blocks.after)
    }

    pub(super) fn preheader_for(
        &self,
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    ) -> Option<BasicBlockId> {
        self.loops
            .iter()
            .find(|blocks| blocks.loop_key == loop_key)
            .map(|blocks| blocks.preheader)
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
                .map(|blocks: &LoopPhysicalBlocksV1| blocks.body)
                .ok_or(LoopPhysicalizerRejectV1::ParentTopologyMissing(parent))?,
            None => entry.preheader,
        };
        let blocks = LoopPhysicalBlocksV1 {
            loop_key: node.key,
            preheader,
            header: services.allocate_block()?,
            body: services.allocate_block()?,
            step: services.allocate_block()?,
            after: services.allocate_block()?,
        };
        physical.insert(node.key, blocks);
    }

    let root = physical
        .get(&recipe.root_loop())
        .copied()
        .ok_or(LoopPhysicalizerRejectV1::AfterOwnerMismatch)?;
    let loops = physical
        .into_values()
        .collect::<Vec<_>>()
        .into_boxed_slice();
    Ok(LoopAfterContinuationReceiptV1 {
        owner,
        root_loop: recipe.root_loop(),
        root_after: root.after,
        loops,
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
