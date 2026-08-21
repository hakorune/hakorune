//! Opaque value transport for one prepared Loop operation schedule.
//!
//! This is not BindingSSA and not a second ownership ledger. It only carries
//! verified operation results from one leaf to a later operand consumer.

use std::collections::{btree_map::Entry, BTreeMap};

use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopValueClassV1, LoopValueKeyV1};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlockId, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LoopOperationValueReceiptV1 {
    owner: FunctionOwnerIdV1,
    key: LoopValueKeyV1,
    class: LoopValueClassV1,
    producer_item: LoopItemKeyV1,
    physical_block: BasicBlockId,
    physical_value: ValueId,
}

impl LoopOperationValueReceiptV1 {
    pub(super) const fn new(
        owner: FunctionOwnerIdV1,
        key: LoopValueKeyV1,
        class: LoopValueClassV1,
        producer_item: LoopItemKeyV1,
        physical_block: BasicBlockId,
        physical_value: ValueId,
    ) -> Self {
        Self {
            owner,
            key,
            class,
            producer_item,
            physical_block,
            physical_value,
        }
    }

    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn key(self) -> LoopValueKeyV1 {
        self.key
    }

    pub(super) const fn class(self) -> LoopValueClassV1 {
        self.class
    }

    pub(super) const fn producer_item(self) -> LoopItemKeyV1 {
        self.producer_item
    }

    pub(super) const fn physical_block(self) -> BasicBlockId {
        self.physical_block
    }

    pub(super) const fn physical_value(self) -> ValueId {
        self.physical_value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LoopOperationValueLedgerRejectV1(pub(super) LoopValueKeyV1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoopOperationValueReservationV1 {
    owner: FunctionOwnerIdV1,
    key: LoopValueKeyV1,
    class: LoopValueClassV1,
    producer_item: LoopItemKeyV1,
    physical_block: BasicBlockId,
}

impl LoopOperationValueReservationV1 {
    fn into_receipt(self, physical_value: ValueId) -> LoopOperationValueReceiptV1 {
        LoopOperationValueReceiptV1::new(
            self.owner,
            self.key,
            self.class,
            self.producer_item,
            self.physical_block,
            physical_value,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoopOperationValuePoisonedV1 {
    owner: FunctionOwnerIdV1,
    key: LoopValueKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopOperationValueLedgerBindingV1 {
    LegacyUnbound,
    Canonical(FunctionOwnerIdV1),
}

#[derive(Debug, PartialEq, Eq)]
enum LoopOperationValueSlotV1 {
    Reserved(LoopOperationValueReservationV1),
    Published(LoopOperationValueReceiptV1),
    Poisoned(LoopOperationValuePoisonedV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LoopOperationValueLedgerReserveRejectV1 {
    OwnerUnbound(LoopValueKeyV1),
    ForeignOwner {
        key: LoopValueKeyV1,
        requested: FunctionOwnerIdV1,
        existing: FunctionOwnerIdV1,
    },
    AlreadyReserved(LoopValueKeyV1),
    AlreadyPublished(LoopValueKeyV1),
    Poisoned(LoopValueKeyV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct LoopOperationValueLedgerV1 {
    binding: LoopOperationValueLedgerBindingV1,
    values: BTreeMap<LoopValueKeyV1, LoopOperationValueSlotV1>,
}

/// A private affine reservation for one result key.
///
/// The mutable ledger borrow prevents another reservation or publication from
/// observing the slot while the strict writer is preparing its append. A
/// dropped token poisons the slot instead of reopening it for repair/retry.
pub(super) struct PendingLoopValuePublishV1<'ledger> {
    slot: &'ledger mut LoopOperationValueSlotV1,
    key: LoopValueKeyV1,
    committed: bool,
}

/// A strict writer-owned definition source consumed by ledger commit.
///
/// The ledger receives this mechanical payload only after the writer has
/// prepared its own exact target/definition relation. The trait keeps the
/// raw `ValueId` out of the reservation API and does not issue source meaning.
pub(super) trait LoopOperationValueDefinitionSourceV1 {
    fn physical_value(&self) -> ValueId;
}

impl LoopOperationValueLedgerV1 {
    pub(super) fn new_for_owner(owner: FunctionOwnerIdV1) -> Self {
        Self {
            binding: LoopOperationValueLedgerBindingV1::Canonical(owner),
            values: BTreeMap::new(),
        }
    }

    pub(super) fn contains(&self, key: LoopValueKeyV1) -> bool {
        self.values.contains_key(&key)
    }

    pub(super) fn publish(
        &mut self,
        receipt: LoopOperationValueReceiptV1,
    ) -> Result<(), LoopOperationValueLedgerRejectV1> {
        if let LoopOperationValueLedgerBindingV1::Canonical(owner) = self.binding {
            if receipt.owner() != owner {
                return Err(LoopOperationValueLedgerRejectV1(receipt.key()));
            }
        }
        match self.values.entry(receipt.key()) {
            Entry::Vacant(slot) => {
                slot.insert(LoopOperationValueSlotV1::Published(receipt));
                Ok(())
            }
            Entry::Occupied(_) => Err(LoopOperationValueLedgerRejectV1(receipt.key())),
        }
    }

    pub(super) fn get(&self, key: LoopValueKeyV1) -> Option<ValueId> {
        match self.values.get(&key) {
            Some(LoopOperationValueSlotV1::Published(receipt)) => Some(receipt.physical_value()),
            _ => None,
        }
    }

    pub(super) fn receipt(&self, key: LoopValueKeyV1) -> Option<LoopOperationValueReceiptV1> {
        match self.values.get(&key) {
            Some(LoopOperationValueSlotV1::Published(receipt)) => Some(*receipt),
            _ => None,
        }
    }

    pub(super) fn reserve_result(
        &mut self,
        owner: FunctionOwnerIdV1,
        key: LoopValueKeyV1,
        class: LoopValueClassV1,
        producer_item: LoopItemKeyV1,
        physical_block: BasicBlockId,
    ) -> Result<PendingLoopValuePublishV1<'_>, LoopOperationValueLedgerReserveRejectV1> {
        match self.binding {
            LoopOperationValueLedgerBindingV1::LegacyUnbound => {
                return Err(LoopOperationValueLedgerReserveRejectV1::OwnerUnbound(key));
            }
            LoopOperationValueLedgerBindingV1::Canonical(expected) if expected != owner => {
                return Err(LoopOperationValueLedgerReserveRejectV1::ForeignOwner {
                    key,
                    requested: owner,
                    existing: expected,
                });
            }
            LoopOperationValueLedgerBindingV1::Canonical(_) => {}
        }
        match self.values.entry(key) {
            Entry::Vacant(slot) => {
                let slot = slot.insert(LoopOperationValueSlotV1::Reserved(
                    LoopOperationValueReservationV1 {
                        owner,
                        key,
                        class,
                        producer_item,
                        physical_block,
                    },
                ));
                Ok(PendingLoopValuePublishV1 {
                    slot,
                    key,
                    committed: false,
                })
            }
            Entry::Occupied(slot) => match slot.get() {
                LoopOperationValueSlotV1::Reserved(reservation) if reservation.owner != owner => {
                    Err(LoopOperationValueLedgerReserveRejectV1::ForeignOwner {
                        key,
                        requested: owner,
                        existing: reservation.owner,
                    })
                }
                LoopOperationValueSlotV1::Reserved(_) => Err(
                    LoopOperationValueLedgerReserveRejectV1::AlreadyReserved(key),
                ),
                LoopOperationValueSlotV1::Published(receipt) if receipt.owner() != owner => {
                    Err(LoopOperationValueLedgerReserveRejectV1::ForeignOwner {
                        key,
                        requested: owner,
                        existing: receipt.owner(),
                    })
                }
                LoopOperationValueSlotV1::Published(_) => Err(
                    LoopOperationValueLedgerReserveRejectV1::AlreadyPublished(key),
                ),
                LoopOperationValueSlotV1::Poisoned(_) => {
                    Err(LoopOperationValueLedgerReserveRejectV1::Poisoned(key))
                }
            },
        }
    }
}

impl PendingLoopValuePublishV1<'_> {
    /// Publish the reserved metadata with the value returned by the sole
    /// physical writer. The slot and all metadata were fixed at reservation;
    /// this method has no fallible validation or map insertion left to do.
    pub(super) fn commit<D: LoopOperationValueDefinitionSourceV1>(
        mut self,
        definition: D,
    ) -> LoopOperationValueReceiptV1 {
        let reservation = match &*self.slot {
            LoopOperationValueSlotV1::Reserved(reservation) => *reservation,
            _ => unreachable!("pending result reservation must remain reserved"),
        };
        debug_assert_eq!(reservation.key, self.key);
        let receipt = reservation.into_receipt(definition.physical_value());
        match &mut *self.slot {
            slot @ LoopOperationValueSlotV1::Reserved(_) => {
                *slot = LoopOperationValueSlotV1::Published(receipt);
            }
            _ => unreachable!("pending result reservation must remain in the ledger"),
        }
        self.committed = true;
        receipt
    }
}

impl Drop for PendingLoopValuePublishV1<'_> {
    fn drop(&mut self) {
        if self.committed {
            return;
        }
        let (owner, key) = match &mut *self.slot {
            LoopOperationValueSlotV1::Reserved(reservation) => (reservation.owner, reservation.key),
            _ => return,
        };
        *self.slot =
            LoopOperationValueSlotV1::Poisoned(LoopOperationValuePoisonedV1 { owner, key });
    }
}

impl Default for LoopOperationValueLedgerV1 {
    fn default() -> Self {
        Self {
            binding: LoopOperationValueLedgerBindingV1::LegacyUnbound,
            values: BTreeMap::new(),
        }
    }
}
