use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1,
};

use crate::mir::{MirType, ValueId};

use super::preloop_nested_result_receipt::EmittedNestedInstanceCallV1;
use super::preloop_nested_result_type::PreparedPreloopNestedIntegerPublicationV1;

fn emitted(destination: u32) -> EmittedNestedInstanceCallV1 {
    EmittedNestedInstanceCallV1::from_destination_for_test(ValueId::new(destination))
}

#[test]
fn missing_or_unknown_destination_prepares_exact_integer_publication() {
    for existing in [None, Some(&MirType::Unknown)] {
        let prepared =
            PreparedPreloopNestedIntegerPublicationV1::prepare(emitted(41), existing).unwrap();
        assert_eq!(prepared.destination(), ValueId::new(41));
        assert_eq!(
            prepared.publication(),
            &PreparedTypeFactPublicationV1::Publish(MirType::Integer)
        );
        prepared.discard();
    }
}

#[test]
fn matching_integer_prepares_idempotence_without_a_second_policy() {
    let prepared =
        PreparedPreloopNestedIntegerPublicationV1::prepare(emitted(42), Some(&MirType::Integer))
            .unwrap();
    assert_eq!(
        prepared.publication(),
        &PreparedTypeFactPublicationV1::Idempotent(MirType::Integer)
    );
    prepared.discard();
}

#[test]
fn concrete_conflict_retains_receipt_and_exact_cause() {
    let rejected =
        PreparedPreloopNestedIntegerPublicationV1::prepare(emitted(43), Some(&MirType::Bool))
            .unwrap_err();
    assert_eq!(rejected.destination(), ValueId::new(43));
    assert_eq!(
        rejected.cause(),
        &TypeFactDecisionErrorV1::ConcreteFactConflict {
            existing: MirType::Bool,
            proposed: MirType::Integer,
        }
    );
    assert!(rejected
        .bounded_report()
        .starts_with("[preloop-nested/type-publication-rejected]"));
    rejected.discard();
}
