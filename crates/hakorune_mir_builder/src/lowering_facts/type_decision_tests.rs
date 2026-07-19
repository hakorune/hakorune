use super::{PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1};
use hakorune_mir_core::MirType;

#[test]
fn nonfacts_publish_exact_types_including_void() {
    for existing in [None, Some(MirType::Unknown)] {
        for proposed in [MirType::Integer, MirType::Void] {
            assert_eq!(
                TypeFactDecisionV1::prepare(existing.as_ref(), Some(&proposed)).unwrap(),
                PreparedTypeFactPublicationV1::Publish(proposed)
            );
        }
    }
}

#[test]
fn matching_exact_fact_is_idempotent() {
    let exact = MirType::Box("Owner".to_string());
    assert_eq!(
        TypeFactDecisionV1::prepare(Some(&exact), Some(&exact)).unwrap(),
        PreparedTypeFactPublicationV1::Idempotent(exact)
    );
}

#[test]
fn absent_proposal_never_materializes_unknown() {
    assert_eq!(
        TypeFactDecisionV1::prepare(None, None).unwrap(),
        PreparedTypeFactPublicationV1::NoPublication
    );

    let unknown = MirType::Unknown;
    assert_eq!(
        TypeFactDecisionV1::prepare(Some(&unknown), None).unwrap(),
        PreparedTypeFactPublicationV1::NoPublication
    );
}

#[test]
fn absent_proposal_preserves_an_exact_fact() {
    let exact = MirType::String;
    assert_eq!(
        TypeFactDecisionV1::prepare(Some(&exact), None).unwrap(),
        PreparedTypeFactPublicationV1::PreserveExisting(exact)
    );
}

#[test]
fn different_exact_facts_fail_without_last_write_wins() {
    let error =
        TypeFactDecisionV1::prepare(Some(&MirType::Integer), Some(&MirType::String)).unwrap_err();
    assert_eq!(
        error,
        TypeFactDecisionErrorV1::ConcreteFactConflict {
            existing: MirType::Integer,
            proposed: MirType::String,
        }
    );
    assert!(error
        .to_string()
        .starts_with("[freeze:contract][lowering_facts/type_decision/concrete_fact_conflict]"));
}

#[test]
fn explicit_unknown_proposal_is_rejected() {
    let unknown = MirType::Unknown;
    assert_eq!(
        TypeFactDecisionV1::prepare(Some(&MirType::Integer), Some(&unknown)).unwrap_err(),
        TypeFactDecisionErrorV1::UnknownProposal {
            existing: Some(MirType::Integer),
        }
    );
    assert_eq!(
        TypeFactDecisionV1::prepare(None, Some(&unknown)).unwrap_err(),
        TypeFactDecisionErrorV1::UnknownProposal { existing: None }
    );
}
