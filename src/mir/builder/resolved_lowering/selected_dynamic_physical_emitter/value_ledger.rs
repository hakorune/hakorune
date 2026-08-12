//! Session-private logical-to-physical value receipts for the selected canary.
//!
//! This ledger transports values already emitted by the physical session. It
//! does not choose Recipe order, block targets, result classes, or lifecycle;
//! those facts remain owned by the preflight plan and executable admission.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::DynamicV2PhysicalRepresentationV1;
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopValueKeyV1};
use crate::mir::{BasicBlockId, ValueId};

use super::targets::DynamicV2OpaquePhysicalTargetV1;
use super::DynamicV2PhysicalSessionBrandV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicV2PhysicalValueLedgerRejectV1 {
    ForeignTarget,
    DuplicateProducer,
    DuplicateResult,
    MissingResult,
    RepresentationMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DynamicV2PhysicalValueEntryV1 {
    producer: LoopItemKeyV1,
    result: LoopValueKeyV1,
    block: BasicBlockId,
    value: ValueId,
    representation: DynamicV2PhysicalRepresentationV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DynamicV2PhysicalValueViewV1 {
    producer: LoopItemKeyV1,
    result: LoopValueKeyV1,
    block: BasicBlockId,
    value: ValueId,
    representation: DynamicV2PhysicalRepresentationV1,
}

impl DynamicV2PhysicalValueViewV1 {
    pub(super) const fn producer(self) -> LoopItemKeyV1 {
        self.producer
    }

    pub(super) const fn result(self) -> LoopValueKeyV1 {
        self.result
    }

    pub(super) const fn block(self) -> BasicBlockId {
        self.block
    }

    pub(super) const fn value(self) -> ValueId {
        self.value
    }

    pub(super) const fn representation(self) -> DynamicV2PhysicalRepresentationV1 {
        self.representation
    }
}

/// The selected physical session owns this move-only ledger. There is no
/// parts/split API, so a published value cannot be paired with another session.
#[derive(Debug)]
pub(super) struct DynamicV2PhysicalValueLedgerV1 {
    brand: DynamicV2PhysicalSessionBrandV1,
    producers: BTreeSet<LoopItemKeyV1>,
    entries: BTreeMap<LoopValueKeyV1, DynamicV2PhysicalValueEntryV1>,
}

impl DynamicV2PhysicalValueLedgerV1 {
    pub(super) fn new(brand: &DynamicV2PhysicalSessionBrandV1) -> Self {
        Self {
            brand: DynamicV2PhysicalSessionBrandV1(Arc::clone(&brand.0)),
            producers: BTreeSet::new(),
            entries: BTreeMap::new(),
        }
    }

    pub(super) fn publish(
        &mut self,
        producer: LoopItemKeyV1,
        result: LoopValueKeyV1,
        target: &DynamicV2OpaquePhysicalTargetV1,
        value: ValueId,
        representation: DynamicV2PhysicalRepresentationV1,
    ) -> Result<(), DynamicV2PhysicalValueLedgerRejectV1> {
        if !target.matches(&self.brand) {
            return Err(DynamicV2PhysicalValueLedgerRejectV1::ForeignTarget);
        }
        if self.producers.contains(&producer) {
            return Err(DynamicV2PhysicalValueLedgerRejectV1::DuplicateProducer);
        }
        if self.entries.contains_key(&result) {
            return Err(DynamicV2PhysicalValueLedgerRejectV1::DuplicateResult);
        }
        self.producers.insert(producer);
        self.entries.insert(
            result,
            DynamicV2PhysicalValueEntryV1 {
                producer,
                result,
                block: target.block(),
                value,
                representation,
            },
        );
        Ok(())
    }

    pub(super) fn with_value<R>(
        &self,
        result: LoopValueKeyV1,
        expected_representation: DynamicV2PhysicalRepresentationV1,
        callback: impl for<'a> FnOnce(&'a DynamicV2PhysicalValueViewV1) -> R,
    ) -> Result<R, DynamicV2PhysicalValueLedgerRejectV1> {
        let entry = self
            .entries
            .get(&result)
            .ok_or(DynamicV2PhysicalValueLedgerRejectV1::MissingResult)?;
        if entry.representation != expected_representation {
            return Err(DynamicV2PhysicalValueLedgerRejectV1::RepresentationMismatch);
        }
        let view = DynamicV2PhysicalValueViewV1 {
            producer: entry.producer,
            result: entry.result,
            block: entry.block,
            value: entry.value,
            representation: entry.representation,
        };
        Ok(callback(&view))
    }
}
