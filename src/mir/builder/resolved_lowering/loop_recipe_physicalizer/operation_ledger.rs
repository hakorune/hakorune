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

    pub(super) const fn physical_block(self) -> BasicBlockId {
        self.physical_block
    }

    pub(super) const fn physical_value(self) -> ValueId {
        self.physical_value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LoopOperationValueLedgerRejectV1(pub(super) LoopValueKeyV1);

#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct LoopOperationValueLedgerV1 {
    values: BTreeMap<LoopValueKeyV1, LoopOperationValueReceiptV1>,
}

impl LoopOperationValueLedgerV1 {
    pub(super) fn contains(&self, key: LoopValueKeyV1) -> bool {
        self.values.contains_key(&key)
    }

    pub(super) fn publish(
        &mut self,
        receipt: LoopOperationValueReceiptV1,
    ) -> Result<(), LoopOperationValueLedgerRejectV1> {
        match self.values.entry(receipt.key()) {
            Entry::Vacant(slot) => {
                slot.insert(receipt);
                Ok(())
            }
            Entry::Occupied(_) => Err(LoopOperationValueLedgerRejectV1(receipt.key())),
        }
    }

    pub(super) fn get(&self, key: LoopValueKeyV1) -> Option<ValueId> {
        self.receipt(key).map(LoopOperationValueReceiptV1::physical_value)
    }

    pub(super) fn receipt(&self, key: LoopValueKeyV1) -> Option<LoopOperationValueReceiptV1> {
        self.values.get(&key).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    #[test]
    fn published_loop_value_receipt_survives_duplicate_key_rejection() {
        let owner = FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
        let key = LoopValueKeyV1::new(12);
        let original = LoopOperationValueReceiptV1::new(
            owner, key, LoopValueClassV1::I64, LoopItemKeyV1::new(4),
            BasicBlockId::new(3), ValueId::new(44),
        );
        let duplicate = LoopOperationValueReceiptV1::new(
            owner, key, LoopValueClassV1::Bool, LoopItemKeyV1::new(5),
            BasicBlockId::new(6), ValueId::new(7),
        );
        let mut ledger = LoopOperationValueLedgerV1::default();
        ledger.publish(original).unwrap();

        assert_eq!(ledger.publish(duplicate), Err(LoopOperationValueLedgerRejectV1(key)));
        assert_eq!(ledger.receipt(key), Some(original));
        assert_eq!(ledger.get(key), Some(original.physical_value()));
    }
}
