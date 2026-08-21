//! Focused evidence for the one-shot Loop result ledger lifecycle.

use super::operation_ledger::{
    LoopOperationValueDefinitionSourceV1, LoopOperationValueLedgerReserveRejectV1,
    LoopOperationValueLedgerV1, LoopOperationValueReceiptV1,
};
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopValueClassV1, LoopValueKeyV1};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlockId, ValueId};

struct TestDefinition(ValueId);

impl LoopOperationValueDefinitionSourceV1 for TestDefinition {
    fn physical_value(&self) -> ValueId {
        self.0
    }
}

fn owner() -> FunctionOwnerIdV1 {
    crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
        .expect("owner issuer")
        .issue()
        .expect("owner")
}

fn published_receipt(
    owner: FunctionOwnerIdV1,
    key: LoopValueKeyV1,
    physical_block: BasicBlockId,
    physical_value: ValueId,
) -> LoopOperationValueReceiptV1 {
    LoopOperationValueReceiptV1::new(
        owner,
        key,
        LoopValueClassV1::I64,
        LoopItemKeyV1::new(4),
        physical_block,
        physical_value,
    )
}

#[test]
fn reservation_commit_publishes_one_full_receipt() {
    let owner = owner();
    let key = LoopValueKeyV1::new(12);
    let physical_block = BasicBlockId::new(3);
    let physical_value = ValueId::new(44);
    let mut ledger = LoopOperationValueLedgerV1::new_for_owner(owner);

    let pending = ledger
        .reserve_result(
            owner,
            key,
            LoopValueClassV1::I64,
            LoopItemKeyV1::new(4),
            physical_block,
        )
        .expect("vacant result slot");
    let receipt = pending.commit(TestDefinition(physical_value));

    assert_eq!(
        receipt,
        published_receipt(owner, key, physical_block, physical_value)
    );
    assert_eq!(ledger.receipt(key), Some(receipt));
    assert_eq!(ledger.get(key), Some(physical_value));
}

#[test]
fn strict_reservation_requires_an_owner_bound_ledger() {
    let owner = owner();
    let key = LoopValueKeyV1::new(11);
    let mut legacy = LoopOperationValueLedgerV1::default();

    let error = legacy
        .reserve_result(
            owner,
            key,
            LoopValueClassV1::I64,
            LoopItemKeyV1::new(3),
            BasicBlockId::new(2),
        )
        .err()
        .expect("legacy ledger must not open a strict reservation");
    assert_eq!(
        error,
        LoopOperationValueLedgerReserveRejectV1::OwnerUnbound(key)
    );
}

#[test]
fn rejected_reservation_preserves_existing_published_receipt() {
    let existing_owner = owner();
    let requested_owner = owner();
    let key = LoopValueKeyV1::new(13);
    let existing = published_receipt(existing_owner, key, BasicBlockId::new(4), ValueId::new(45));
    let mut ledger = LoopOperationValueLedgerV1::new_for_owner(existing_owner);
    ledger.publish(existing).expect("existing publication");

    let error = ledger
        .reserve_result(
            existing_owner,
            key,
            LoopValueClassV1::I64,
            LoopItemKeyV1::new(9),
            BasicBlockId::new(5),
        )
        .err()
        .expect("published key must reject");
    assert_eq!(
        error,
        LoopOperationValueLedgerReserveRejectV1::AlreadyPublished(key)
    );
    let error = ledger
        .reserve_result(
            requested_owner,
            key,
            LoopValueClassV1::I64,
            LoopItemKeyV1::new(9),
            BasicBlockId::new(5),
        )
        .err()
        .expect("foreign owner must reject");
    assert_eq!(
        error,
        LoopOperationValueLedgerReserveRejectV1::ForeignOwner {
            key,
            requested: requested_owner,
            existing: existing_owner,
        }
    );
    assert_eq!(ledger.receipt(key), Some(existing));
}

#[test]
fn dropped_reservation_poison_is_terminal_and_not_reusable() {
    let owner = owner();
    let key = LoopValueKeyV1::new(14);
    let mut ledger = LoopOperationValueLedgerV1::new_for_owner(owner);

    let pending = ledger
        .reserve_result(
            owner,
            key,
            LoopValueClassV1::I64,
            LoopItemKeyV1::new(6),
            BasicBlockId::new(6),
        )
        .expect("vacant result slot");
    drop(pending);

    assert_eq!(ledger.receipt(key), None);
    let error = ledger
        .reserve_result(
            owner,
            key,
            LoopValueClassV1::I64,
            LoopItemKeyV1::new(6),
            BasicBlockId::new(6),
        )
        .err()
        .expect("poisoned key must reject");
    assert_eq!(
        error,
        LoopOperationValueLedgerReserveRejectV1::Poisoned(key)
    );
    assert!(ledger
        .publish(published_receipt(
            owner,
            key,
            BasicBlockId::new(6),
            ValueId::new(46),
        ))
        .is_err());
}
