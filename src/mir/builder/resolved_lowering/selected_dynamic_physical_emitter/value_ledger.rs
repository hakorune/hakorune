//! Session-private logical-to-physical value receipts for the selected canary.
//!
//! This ledger transports values already emitted by the physical session. It
//! does not choose Recipe order, block targets, result classes, or lifecycle;
//! those facts remain owned by the preflight plan and executable admission.

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::mir::builder::builder_emit::CanonicalCompareDefinitionSourceV1;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::DynamicV2PhysicalRepresentationV1;
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopValueKeyV1};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlockId, ValueId};

use super::targets::DynamicV2OpaquePhysicalTargetV1;
use super::DynamicV2PhysicalSessionBrandV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicV2PhysicalValueLedgerRejectV1 {
    ForeignTarget,
    DuplicateProducer,
    DuplicateResult,
    ResultReserved,
    ResultPoisoned,
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
struct DynamicV2PhysicalValueReservationV1 {
    owner: FunctionOwnerIdV1,
    producer: LoopItemKeyV1,
    result: LoopValueKeyV1,
    block: BasicBlockId,
    value: ValueId,
    representation: DynamicV2PhysicalRepresentationV1,
}

#[derive(Debug, PartialEq, Eq)]
enum DynamicV2PhysicalValueSlotV1 {
    Reserved(DynamicV2PhysicalValueReservationV1),
    Published(DynamicV2PhysicalValueEntryV1),
    Poisoned {
        producer: LoopItemKeyV1,
        result: LoopValueKeyV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DynamicV2PhysicalValueViewV1 {
    producer: LoopItemKeyV1,
    result: LoopValueKeyV1,
    block: BasicBlockId,
    value: ValueId,
    representation: DynamicV2PhysicalRepresentationV1,
}

/// A private one-shot publication slot for a selected Dynamic result.
///
/// The reservation owns no source meaning: all producer, result, target, and
/// representation facts were admitted before it is created.  Dropping it
/// poisons the slot so the unpublished outer function session must discard;
/// the slot can never be reopened for a retry.
pub(super) struct PendingDynamicV2PhysicalValuePublishV1<'ledger> {
    slot: &'ledger mut DynamicV2PhysicalValueSlotV1,
    key: LoopValueKeyV1,
    committed: bool,
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

impl DynamicV2PhysicalValueEntryV1 {
    fn view(self) -> DynamicV2PhysicalValueViewV1 {
        DynamicV2PhysicalValueViewV1 {
            producer: self.producer,
            result: self.result,
            block: self.block,
            value: self.value,
            representation: self.representation,
        }
    }
}

/// The selected physical session owns this move-only ledger. There is no
/// parts/split API, so a published value cannot be paired with another session.
#[derive(Debug)]
pub(super) struct DynamicV2PhysicalValueLedgerV1 {
    brand: DynamicV2PhysicalSessionBrandV1,
    producers: BTreeSet<LoopItemKeyV1>,
    entries: BTreeMap<LoopValueKeyV1, DynamicV2PhysicalValueSlotV1>,
}

impl DynamicV2PhysicalValueLedgerV1 {
    pub(super) fn new(brand: &DynamicV2PhysicalSessionBrandV1) -> Self {
        Self {
            brand: DynamicV2PhysicalSessionBrandV1(Arc::clone(&brand.0), brand.1),
            producers: BTreeSet::new(),
            entries: BTreeMap::new(),
        }
    }

    pub(super) fn owner(&self) -> FunctionOwnerIdV1 {
        self.brand.owner()
    }

    pub(super) fn matches_brand(&self, brand: &DynamicV2PhysicalSessionBrandV1) -> bool {
        self.brand.matches(brand)
    }

    fn occupied_result_reject(
        &self,
        result: LoopValueKeyV1,
    ) -> Option<DynamicV2PhysicalValueLedgerRejectV1> {
        match self.entries.get(&result) {
            Some(DynamicV2PhysicalValueSlotV1::Reserved(_)) => {
                Some(DynamicV2PhysicalValueLedgerRejectV1::ResultReserved)
            }
            Some(DynamicV2PhysicalValueSlotV1::Published(_)) => {
                Some(DynamicV2PhysicalValueLedgerRejectV1::DuplicateResult)
            }
            Some(DynamicV2PhysicalValueSlotV1::Poisoned { .. }) => {
                Some(DynamicV2PhysicalValueLedgerRejectV1::ResultPoisoned)
            }
            None => None,
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
        if let Some(error) = self.occupied_result_reject(result) {
            return Err(error);
        }
        let Entry::Vacant(slot) = self.entries.entry(result) else {
            unreachable!("occupied Dynamic result was checked above")
        };
        self.producers.insert(producer);
        slot.insert(DynamicV2PhysicalValueSlotV1::Published(
            DynamicV2PhysicalValueEntryV1 {
                producer,
                result,
                block: target.block(),
                value,
                representation,
            },
        ));
        Ok(())
    }

    pub(super) fn reserve_result(
        &mut self,
        producer: LoopItemKeyV1,
        result: LoopValueKeyV1,
        target: &DynamicV2OpaquePhysicalTargetV1,
        value: ValueId,
        representation: DynamicV2PhysicalRepresentationV1,
    ) -> Result<PendingDynamicV2PhysicalValuePublishV1<'_>, DynamicV2PhysicalValueLedgerRejectV1>
    {
        if !target.matches(&self.brand) {
            return Err(DynamicV2PhysicalValueLedgerRejectV1::ForeignTarget);
        }
        if let Some(error) = self.occupied_result_reject(result) {
            return Err(error);
        }
        if self.producers.contains(&producer) {
            return Err(DynamicV2PhysicalValueLedgerRejectV1::DuplicateProducer);
        }
        match self.entries.entry(result) {
            Entry::Vacant(slot) => {
                self.producers.insert(producer);
                let slot = slot.insert(DynamicV2PhysicalValueSlotV1::Reserved(
                    DynamicV2PhysicalValueReservationV1 {
                        owner: self.brand.owner(),
                        producer,
                        result,
                        block: target.block(),
                        value,
                        representation,
                    },
                ));
                Ok(PendingDynamicV2PhysicalValuePublishV1 {
                    slot,
                    key: result,
                    committed: false,
                })
            }
            Entry::Occupied(_) => unreachable!("occupied Dynamic result was checked above"),
        }
    }

    pub(super) fn with_value<R>(
        &self,
        result: LoopValueKeyV1,
        expected_representation: DynamicV2PhysicalRepresentationV1,
        callback: impl for<'a> FnOnce(&'a DynamicV2PhysicalValueViewV1) -> R,
    ) -> Result<R, DynamicV2PhysicalValueLedgerRejectV1> {
        let entry = match self.entries.get(&result) {
            Some(DynamicV2PhysicalValueSlotV1::Published(entry)) => entry,
            Some(DynamicV2PhysicalValueSlotV1::Reserved(_)) => {
                return Err(DynamicV2PhysicalValueLedgerRejectV1::ResultReserved)
            }
            Some(DynamicV2PhysicalValueSlotV1::Poisoned { .. }) => {
                return Err(DynamicV2PhysicalValueLedgerRejectV1::ResultPoisoned)
            }
            None => return Err(DynamicV2PhysicalValueLedgerRejectV1::MissingResult),
        };
        if entry.representation != expected_representation {
            return Err(DynamicV2PhysicalValueLedgerRejectV1::RepresentationMismatch);
        }
        let view = (*entry).view();
        Ok(callback(&view))
    }

    #[cfg(test)]
    pub(super) fn with_value_for_test<R>(
        &self,
        result: LoopValueKeyV1,
        expected_representation: DynamicV2PhysicalRepresentationV1,
        callback: impl FnOnce(&DynamicV2PhysicalValueViewV1) -> R,
    ) -> Result<R, DynamicV2PhysicalValueLedgerRejectV1> {
        self.with_value(result, expected_representation, callback)
    }
}

impl PendingDynamicV2PhysicalValuePublishV1<'_> {
    /// Commit the reserved Dynamic result from the sole strict writer.
    /// Reservation fixed every metadata field, so this suffix is infallible.
    pub(super) fn commit(
        mut self,
        definition: &CanonicalCompareDefinitionSourceV1,
    ) -> DynamicV2PhysicalValueViewV1 {
        let reservation = match &*self.slot {
            DynamicV2PhysicalValueSlotV1::Reserved(reservation) => *reservation,
            _ => unreachable!("pending Dynamic result must remain reserved"),
        };
        assert_eq!(reservation.owner, definition.owner());
        assert_eq!(reservation.block, definition.target());
        assert_eq!(reservation.value, definition.physical_value());
        let entry = DynamicV2PhysicalValueEntryV1 {
            producer: reservation.producer,
            result: reservation.result,
            block: reservation.block,
            value: reservation.value,
            representation: reservation.representation,
        };
        let view = entry.view();
        match &mut *self.slot {
            slot @ DynamicV2PhysicalValueSlotV1::Reserved(_) => {
                *slot = DynamicV2PhysicalValueSlotV1::Published(entry);
            }
            _ => unreachable!("pending Dynamic result must remain in the ledger"),
        }
        self.committed = true;
        view
    }
}

impl Drop for PendingDynamicV2PhysicalValuePublishV1<'_> {
    fn drop(&mut self) {
        if self.committed {
            return;
        }
        let (producer, result) = match &*self.slot {
            DynamicV2PhysicalValueSlotV1::Reserved(reservation) => {
                (reservation.producer, reservation.result)
            }
            _ => return,
        };
        *self.slot = DynamicV2PhysicalValueSlotV1::Poisoned { producer, result };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    const PRODUCER: LoopItemKeyV1 = LoopItemKeyV1::new(9);
    const RESULT: LoopValueKeyV1 = LoopValueKeyV1::new(13);
    const OTHER_RESULT: LoopValueKeyV1 = LoopValueKeyV1::new(14);

    fn owner() -> FunctionOwnerIdV1 {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        issuer.issue().expect("function owner")
    }

    fn ledger_and_target() -> (
        DynamicV2PhysicalSessionBrandV1,
        DynamicV2PhysicalValueLedgerV1,
        DynamicV2OpaquePhysicalTargetV1,
    ) {
        let brand = DynamicV2PhysicalSessionBrandV1::for_owner(owner());
        let ledger = DynamicV2PhysicalValueLedgerV1::new(&brand);
        let target = DynamicV2OpaquePhysicalTargetV1::for_block(&brand, BasicBlockId::new(3));
        (brand, ledger, target)
    }

    #[test]
    fn published_dynamic_value_is_owner_and_brand_bound() {
        let (brand, mut ledger, target) = ledger_and_target();
        let value = ValueId::new(17);
        ledger
            .publish(
                PRODUCER,
                RESULT,
                &target,
                value,
                DynamicV2PhysicalRepresentationV1::ImmediateI64,
            )
            .expect("first publication");

        assert_eq!(ledger.owner(), brand.owner());
        assert!(ledger.matches_brand(&brand));
        let observed = ledger
            .with_value(
                RESULT,
                DynamicV2PhysicalRepresentationV1::ImmediateI64,
                |view| (view.producer(), view.block(), view.value()),
            )
            .expect("published value");
        assert_eq!(observed, (PRODUCER, target.block(), value));
        assert_eq!(
            ledger.publish(
                LoopItemKeyV1::new(10),
                RESULT,
                &target,
                value,
                DynamicV2PhysicalRepresentationV1::ImmediateI64,
            ),
            Err(DynamicV2PhysicalValueLedgerRejectV1::DuplicateResult)
        );
    }

    #[test]
    fn foreign_target_is_rejected_without_ledger_effect() {
        let (_brand, mut ledger, _target) = ledger_and_target();
        let foreign_brand = DynamicV2PhysicalSessionBrandV1::for_owner(owner());
        let foreign_target =
            DynamicV2OpaquePhysicalTargetV1::for_block(&foreign_brand, BasicBlockId::new(3));

        assert_eq!(
            ledger.publish(
                PRODUCER,
                RESULT,
                &foreign_target,
                ValueId::new(17),
                DynamicV2PhysicalRepresentationV1::ImmediateI64,
            ),
            Err(DynamicV2PhysicalValueLedgerRejectV1::ForeignTarget)
        );
        assert_eq!(
            ledger.with_value(
                RESULT,
                DynamicV2PhysicalRepresentationV1::ImmediateI64,
                |_| (),
            ),
            Err(DynamicV2PhysicalValueLedgerRejectV1::MissingResult)
        );
    }

    #[test]
    fn dropped_result_reservation_poisoned_and_cannot_retry() {
        let (_brand, mut ledger, target) = ledger_and_target();
        {
            let _pending = ledger
                .reserve_result(
                    PRODUCER,
                    RESULT,
                    &target,
                    ValueId::new(17),
                    DynamicV2PhysicalRepresentationV1::ImmediateBool,
                )
                .expect("result reservation");
        }

        assert_eq!(
            ledger.with_value(
                RESULT,
                DynamicV2PhysicalRepresentationV1::ImmediateBool,
                |_| (),
            ),
            Err(DynamicV2PhysicalValueLedgerRejectV1::ResultPoisoned)
        );
        assert!(matches!(
            ledger.reserve_result(
                PRODUCER,
                RESULT,
                &target,
                ValueId::new(17),
                DynamicV2PhysicalRepresentationV1::ImmediateBool,
            ),
            Err(DynamicV2PhysicalValueLedgerRejectV1::ResultPoisoned)
        ));
        assert!(matches!(
            ledger.reserve_result(
                PRODUCER,
                OTHER_RESULT,
                &target,
                ValueId::new(18),
                DynamicV2PhysicalRepresentationV1::ImmediateBool,
            ),
            Err(DynamicV2PhysicalValueLedgerRejectV1::DuplicateProducer)
        ));
    }
}
