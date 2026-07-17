use std::collections::BTreeMap;

use super::*;

fn bb(id: u32) -> BasicBlockId {
    BasicBlockId(id)
}

fn value(id: u32) -> ValueId {
    ValueId(id)
}

fn types(rows: &[(u32, MirType)]) -> BTreeMap<ValueId, MirType> {
    rows.iter()
        .map(|(id, ty)| (value(*id), ty.clone()))
        .collect()
}

fn decide(
    inputs: &[(BasicBlockId, ValueId)],
    value_types: &BTreeMap<ValueId, MirType>,
    existing: Option<&MirType>,
    hint: Option<&MirType>,
) -> Result<PreparedPhiTypePublicationV1, PhiConcreteTypeConflictV1> {
    PhiTransientTypeDecisionV1::prepare(value(99), inputs, value_types, existing, hint)
}

#[test]
fn phi_type_publication_unanimous_exact_types_prepare_publish() {
    for ty in [
        MirType::Integer,
        MirType::Bool,
        MirType::Float,
        MirType::Box("Owner".to_string()),
        MirType::Void,
    ] {
        let input_types = types(&[(1, ty.clone()), (2, ty.clone())]);
        let inputs = [(bb(1), value(1)), (bb(2), value(2))];
        assert_eq!(
            decide(&inputs, &input_types, None, None).unwrap(),
            PreparedPhiTypePublicationV1::Publish(ty.clone())
        );
        assert_eq!(
            decide(&inputs, &input_types, Some(&MirType::Unknown), None).unwrap(),
            PreparedPhiTypePublicationV1::Publish(ty.clone())
        );
        assert_eq!(
            decide(&inputs, &input_types, None, Some(&ty)).unwrap(),
            PreparedPhiTypePublicationV1::Publish(ty.clone())
        );
        assert_eq!(
            decide(&inputs, &input_types, None, Some(&MirType::Unknown),).unwrap(),
            PreparedPhiTypePublicationV1::Publish(ty)
        );
    }
}

#[test]
fn phi_type_publication_single_input_and_matching_destination_are_exact() {
    let input_types = types(&[(7, MirType::String)]);
    let inputs = [(bb(4), value(7))];
    assert_eq!(
        decide(&inputs, &input_types, None, None).unwrap(),
        PreparedPhiTypePublicationV1::Publish(MirType::String)
    );
    assert_eq!(
        decide(
            &inputs,
            &input_types,
            Some(&MirType::String),
            Some(&MirType::String),
        )
        .unwrap(),
        PreparedPhiTypePublicationV1::Idempotent(MirType::String)
    );
}

#[test]
fn phi_type_publication_nonfacts_do_not_manufacture_candidate() {
    assert_eq!(
        decide(&[], &BTreeMap::new(), None, Some(&MirType::Integer)).unwrap(),
        PreparedPhiTypePublicationV1::NoPublication(PhiTypeNoPublicationReasonV1::EmptyInputs)
    );

    let missing_types = types(&[(2, MirType::Integer)]);
    assert_eq!(
        decide(
            &[(bb(2), value(2)), (bb(1), value(1))],
            &missing_types,
            None,
            Some(&MirType::Integer),
        )
        .unwrap(),
        PreparedPhiTypePublicationV1::NoPublication(
            PhiTypeNoPublicationReasonV1::MissingInputType {
                predecessor: bb(1),
                value: value(1),
            }
        )
    );

    let unknown_types = types(&[(1, MirType::Unknown), (2, MirType::Integer)]);
    assert_eq!(
        decide(
            &[(bb(1), value(1)), (bb(2), value(2))],
            &unknown_types,
            None,
            None,
        )
        .unwrap(),
        PreparedPhiTypePublicationV1::NoPublication(
            PhiTypeNoPublicationReasonV1::UnknownInputType {
                predecessor: bb(1),
                value: value(1),
            }
        )
    );

    let heterogeneous = types(&[(1, MirType::Integer), (2, MirType::String)]);
    assert_eq!(
        decide(
            &[(bb(1), value(1)), (bb(2), value(2))],
            &heterogeneous,
            None,
            None,
        )
        .unwrap(),
        PreparedPhiTypePublicationV1::NoPublication(
            PhiTypeNoPublicationReasonV1::HeterogeneousInputTypes
        )
    );
}

#[test]
fn phi_type_publication_existing_concrete_type_is_preserved_for_nonfacts() {
    let input_types = types(&[(1, MirType::Integer)]);
    let inputs = [(bb(1), value(1)), (bb(2), value(2))];
    assert_eq!(
        decide(
            &inputs,
            &input_types,
            Some(&MirType::Integer),
            Some(&MirType::Integer),
        )
        .unwrap(),
        PreparedPhiTypePublicationV1::PreserveExisting {
            existing: MirType::Integer,
            reason: PhiTypeNoPublicationReasonV1::MissingInputType {
                predecessor: bb(2),
                value: value(2),
            },
        }
    );
}

#[test]
fn phi_type_publication_equal_rank_concrete_constraints_conflict() {
    let input_types = types(&[(1, MirType::String)]);
    let inputs = [(bb(3), value(1))];
    let destination_hint = decide(
        &inputs,
        &input_types,
        Some(&MirType::Integer),
        Some(&MirType::String),
    )
    .unwrap_err();
    assert_eq!(
        destination_hint,
        PhiConcreteTypeConflictV1 {
            dst: value(99),
            first_site: PhiTypeFactSiteV1::ExistingDestination,
            first_type: MirType::Integer,
            second_site: PhiTypeFactSiteV1::ExplicitTypeHint,
            second_type: MirType::String,
        }
    );

    let destination_incoming =
        decide(&inputs, &input_types, Some(&MirType::Integer), None).unwrap_err();
    assert_eq!(
        destination_incoming.second_site,
        PhiTypeFactSiteV1::Incoming {
            predecessor: bb(3),
            value: value(1),
        }
    );

    let hint_incoming = decide(&inputs, &input_types, None, Some(&MirType::Integer)).unwrap_err();
    assert_eq!(
        hint_incoming.first_site,
        PhiTypeFactSiteV1::ExplicitTypeHint
    );
}

#[test]
fn phi_type_publication_known_conflict_precedes_no_publication() {
    let input_types = types(&[(1, MirType::Integer), (2, MirType::String)]);
    let inputs = [(bb(1), value(1)), (bb(2), value(2)), (bb(3), value(3))];
    let conflict = decide(&inputs, &input_types, None, Some(&MirType::Integer)).unwrap_err();
    assert_eq!(conflict.first_site, PhiTypeFactSiteV1::ExplicitTypeHint);
    assert_eq!(
        conflict.second_site,
        PhiTypeFactSiteV1::Incoming {
            predecessor: bb(2),
            value: value(2),
        }
    );
}

#[test]
fn phi_type_publication_conflict_witness_is_input_order_independent() {
    let input_types = types(&[
        (10, MirType::String),
        (20, MirType::Float),
        (30, MirType::Integer),
    ]);
    let first = decide(
        &[(bb(3), value(30)), (bb(2), value(20)), (bb(1), value(10))],
        &input_types,
        None,
        Some(&MirType::Bool),
    )
    .unwrap_err();
    let second = decide(
        &[(bb(2), value(20)), (bb(1), value(10)), (bb(3), value(30))],
        &input_types,
        None,
        Some(&MirType::Bool),
    )
    .unwrap_err();
    assert_eq!(first, second);
    assert_eq!(
        first.second_site,
        PhiTypeFactSiteV1::Incoming {
            predecessor: bb(1),
            value: value(10),
        }
    );
}

#[test]
fn phi_type_publication_commit_writes_only_publish() {
    let dst = value(99);
    let unrelated = value(7);
    let mut value_types = types(&[(7, MirType::Bool), (99, MirType::Unknown)]);
    commit_prepared_phi_type(
        &mut value_types,
        dst,
        PreparedPhiTypePublicationV1::Publish(MirType::Integer),
    );
    assert_eq!(value_types.get(&dst), Some(&MirType::Integer));
    assert_eq!(value_types.get(&unrelated), Some(&MirType::Bool));

    let stable = value_types.clone();
    for prepared in [
        PreparedPhiTypePublicationV1::Idempotent(MirType::Integer),
        PreparedPhiTypePublicationV1::PreserveExisting {
            existing: MirType::Integer,
            reason: PhiTypeNoPublicationReasonV1::EmptyInputs,
        },
        PreparedPhiTypePublicationV1::NoPublication(PhiTypeNoPublicationReasonV1::EmptyInputs),
    ] {
        commit_prepared_phi_type(&mut value_types, dst, prepared);
        assert_eq!(value_types, stable);
    }
}

#[test]
fn phi_type_publication_conflict_display_uses_stable_tag() {
    let input_types = types(&[(1, MirType::String)]);
    let error = decide(
        &[(bb(1), value(1))],
        &input_types,
        Some(&MirType::Integer),
        None,
    )
    .unwrap_err();
    assert!(error
        .to_string()
        .starts_with("[freeze:contract][phi_type_publication/concrete_fact_conflict]"));
}
