use super::remat_fact::{
    test_support, CandidateFactReservationErrorV1, CandidateFunctionFactSessionV1,
    ExactProducerFamilyV1, ExactProducerReceiptErrorV1, ExactProducerReceiptLookupErrorV1,
    OpenExactProducerReceiptLedgerV1, PhiRematExactTypeProjectionErrorV1,
    PreparedPhiRematExactTypeProjectionV1,
};
use crate::mir::{BasicBlockId, MirType, ValueId};

fn const_definition(
    generation: u64,
    value: u32,
) -> super::remat_fact::ProducerDefinitionIdentityV1 {
    test_support::definition(
        test_support::generation(generation),
        ValueId::new(value),
        ExactProducerFamilyV1::Const,
        41,
    )
}

#[test]
fn exact_receipt_ledger_rejects_unknown_foreign_and_duplicate_rows() {
    let generation = test_support::generation(7);
    let definition = const_definition(7, 3);
    let mut ledger = OpenExactProducerReceiptLedgerV1::new(generation);

    assert_eq!(
        ledger.record_success(definition, MirType::Unknown),
        Err(ExactProducerReceiptErrorV1::UnknownIsNotExact)
    );
    assert_eq!(
        ledger.record_success(const_definition(8, 3), MirType::Integer),
        Err(ExactProducerReceiptErrorV1::ForeignGeneration {
            ledger: generation,
            definition: test_support::generation(8),
        })
    );
    ledger.record_success(definition, MirType::Integer).unwrap();
    assert_eq!(
        ledger.record_success(definition, MirType::Integer),
        Err(ExactProducerReceiptErrorV1::DuplicateDefinition(definition))
    );
}

#[test]
fn sealed_ledger_requires_the_exact_definition_and_generation() {
    let generation = test_support::generation(7);
    let definition = const_definition(7, 3);
    let mut open = OpenExactProducerReceiptLedgerV1::new(generation);
    open.record_success(definition, MirType::Void).unwrap();
    let ledger = open.seal();

    assert_eq!(
        ledger.lookup(definition).unwrap().exact_type(),
        &MirType::Void
    );
    assert_eq!(
        ledger.lookup(const_definition(7, 4)),
        Err(ExactProducerReceiptLookupErrorV1::MissingProducerReceipt(
            const_definition(7, 4)
        ))
    );
    assert_eq!(
        ledger.lookup(const_definition(8, 3)),
        Err(ExactProducerReceiptLookupErrorV1::ForeignGeneration {
            ledger: generation,
            definition: test_support::generation(8),
        })
    );
}

#[test]
fn projection_co_seals_exact_receipt_node_and_reserved_destination() {
    let generation = test_support::generation(7);
    let definition = const_definition(7, 3);
    let mut open = OpenExactProducerReceiptLedgerV1::new(generation);
    open.record_success(definition, MirType::Integer).unwrap();
    let ledger = open.seal();
    let mut session = CandidateFunctionFactSessionV1::new(generation);
    let reservation = session.reserve_fresh_destination(ValueId::new(19)).unwrap();

    let projection = PreparedPhiRematExactTypeProjectionV1::prepare(
        &ledger,
        test_support::node(BasicBlockId::new(2), definition),
        reservation,
    )
    .unwrap();
    assert_eq!(
        projection.test_parts(),
        (
            ValueId::new(3),
            BasicBlockId::new(2),
            ValueId::new(19),
            &MirType::Integer
        )
    );
}

#[test]
fn projection_rejects_missing_receipt_before_candidate_mutation_owner_exists() {
    let generation = test_support::generation(7);
    let definition = const_definition(7, 3);
    let ledger = OpenExactProducerReceiptLedgerV1::new(generation).seal();
    let mut session = CandidateFunctionFactSessionV1::new(generation);
    let reservation = session.reserve_fresh_destination(ValueId::new(19)).unwrap();

    assert!(matches!(
        PreparedPhiRematExactTypeProjectionV1::prepare(
            &ledger,
            test_support::node(BasicBlockId::new(2), definition),
            reservation,
        ),
        Err(PhiRematExactTypeProjectionErrorV1::Receipt(
            ExactProducerReceiptLookupErrorV1::MissingProducerReceipt(actual)
        )) if actual == definition
    ));
}

#[test]
fn candidate_session_reserves_each_fresh_destination_once() {
    let mut session = CandidateFunctionFactSessionV1::new(test_support::generation(7));
    session.reserve_fresh_destination(ValueId::new(19)).unwrap();
    assert!(matches!(
        session.reserve_fresh_destination(ValueId::new(19)),
        Err(CandidateFactReservationErrorV1::DuplicateDestination(value))
            if value == ValueId::new(19)
    ));
}
