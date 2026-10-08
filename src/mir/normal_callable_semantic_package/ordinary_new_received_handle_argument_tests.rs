use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog as issue,
    VerifiedNormalCallableSemanticPackageV1 as Package,
};
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;

fn package(formal: &str, actual: &str, nullable: bool) -> Package {
    let body = if nullable {
        "if size > 0 { return new Token() } return null"
    } else {
        "return new Token()"
    };
    issue(&format!("box Spare {{}} box Token {{}} box Maker {{ make({formal}) {{ {body} }} }} static box Main {{ main() {{ local maker = new Maker() local spare = new Spare() local item = maker.make({actual}) return item }} }}")).unwrap()
}

fn original(
    package: &Package,
) -> (
    LexicalInstanceCallSourceTargetV1,
    Vec<OrdinaryNewCandidate>,
    BindingRefV1,
) {
    let ledger = &package.ordinary_new_claim_ledger;
    let completion = ledger.root_completion.as_ref().unwrap().as_ref().unwrap();
    let flow = completion.cleanup().root_flow().unwrap();
    let observation = &flow.local_calls()[0];
    let target = ledger.lexical_instance_calls.borrow().get(observation.site()).and_then(|slot| match slot {
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalInstanceCallDispositionSlotV1::Ready(row) => Some(row.source_target().clone()),
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalInstanceCallDispositionSlotV1::SourcePending(target) => Some(target.clone()),
            _ => None,
        }).unwrap();
    let slot = package
        .batch()
        .declarations()
        .find(|row| row.owner() == completion.owner())
        .unwrap()
        .batch_slot();
    let mut candidates = super::super::super::source_claims::prepare_local_candidates_by_slot_v1(
        package.batch(),
        &package.selected,
        &package.instance_constructors,
        Some(slot),
        None,
    );
    (
        target,
        candidates.remove(&slot).unwrap().unwrap(),
        observation.local_binding().unwrap().1,
    )
}

#[test]
fn received_handle_source_observation_preserves_opaque_typed_zero_and_original_homes() {
    for (formal, actual, opaque) in [
        ("size", "7", true),
        ("size: i64", "7", false),
        ("", "", false),
    ] {
        let package = package(formal, actual, false);
        let ledger = &package.ordinary_new_claim_ledger;
        let flow = ledger
            .root_completion
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .cleanup()
            .root_flow()
            .unwrap();
        assert_eq!(flow.local_calls().len(), 1);
        let observation = &flow.local_calls()[0];
        assert_eq!(
            observation.result(),
            crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Handle
        );
        assert_eq!(observation.prior_homes().len(), 2);
        let (declaration, destination) = observation.local_binding().unwrap();
        let (target, candidates, original_destination) = original(&package);
        assert_eq!(destination, original_destination);
        assert!(candidates
            .iter()
            .all(|candidate| candidate.destination != destination));
        assert_eq!(
            observation.arguments().len(),
            usize::from(!actual.is_empty())
        );
        if opaque {
            assert!(
                matches!(observation.arguments(), [LocalCallArgumentV1::BorrowedActual { ordinal: 0, site }] if target.argument_sites() == [site.clone()])
            );
        } else if !actual.is_empty() {
            assert_eq!(observation.arguments(), [LocalCallArgumentV1::Integer(7)]);
        }
        let slot = package
            .batch()
            .declarations()
            .find(|row| row.owner() == observation.owner())
            .unwrap()
            .batch_slot();
        package
            .batch()
            .with_lowering_input(slot, |input| {
                let initializer = input
                    .function()
                    .expression_source()
                    .initializer(declaration)
                    .unwrap();
                assert_eq!(initializer.binding(), destination);
                assert_eq!(
                    initializer.initializer_site(),
                    Some(observation.site().site())
                );
            })
            .unwrap();
    }
}

#[test]
fn received_handle_plural_refuses_late_phase_arguments_unavailable_and_error() {
    let facts = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::source_result_facts_for_test(
        "box Token {} box Door { make() { return new Token() } relay(flag: i64) { local item = me.make() if flag == 0 { return item } return item } } static box Main { main() { return 0 } }"
    );
    let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Door", "relay", 1,
    );
    let loan = facts
        .object_return_qualification(facts.outcomes(&key).unwrap()[0].site())
        .unwrap();
    let loans = facts.qualifications_at_call(loan.call());
    assert_eq!(loans.len(), 2);
    for mutation in 0..5 {
        let mut visits = Vec::new();
        let result = corroborate_all_arguments_v1(&loans, &mut |original| {
            visits.push(original.clone());
            if visits.len() == 2 {
                return match mutation {
                    1 => Ok(ObjectCallSourceSupportV1::Observed(Box::new([]))),
                    2 => Ok(ObjectCallSourceSupportV1::SourceOnly(
                        vec![LocalCallArgumentV1::Integer(7)].into_boxed_slice(),
                    )),
                    3 => Ok(ObjectCallSourceSupportV1::Unavailable),
                    4 => Err("late-original-error".into()),
                    _ => Ok(ObjectCallSourceSupportV1::SourceOnly(Box::new([]))),
                };
            }
            Ok(ObjectCallSourceSupportV1::SourceOnly(Box::new([])))
        });
        assert_eq!(visits, loans.as_ref());
        if mutation == 0 {
            assert_eq!(
                result.unwrap(),
                ObjectCallSourceSupportV1::SourceOnly(Box::new([]))
            );
        } else {
            let issue = result.unwrap_err();
            assert!(issue.contains(match mutation {
                1 | 2 => "arguments-disagree",
                3 => "arguments-unavailable",
                4 => "late-original-error",
                _ => unreachable!(),
            }));
        }
    }
}

#[test]
fn received_handle_selected_target_errors_never_change_pending_or_retry_old_lane() {
    let package = package("size", "7", false);
    let ledger = &package.ordinary_new_claim_ledger;
    let (target, candidates, destination) = original(&package);
    let source = ledger.borrowed_formal_source.as_ref().unwrap();
    let mut pending = ledger.borrowed_formal_actuals.clone();
    let snapshot = pending.clone();
    let cases = [
        (Err("selected-target-error".into()), "selected-target-error"),
        (Ok(Vec::new()), "target-missing"),
        (
            Ok(vec![Ok(Some(target.clone())), Ok(Some(target.clone()))]),
            "target-not-unique",
        ),
        (
            Ok(vec![
                Ok(Some(target.clone())),
                Err("late-target-error".into()),
            ]),
            "late-target-error",
        ),
    ];
    for (targets, reason) in cases {
        let mut projected = 0;
        let error = received_handle_arguments_v1(
            &targets,
            &candidates,
            &ledger.callable_result_classes,
            target.call_site(),
            destination,
            &mut |_, _| {
                projected += 1;
                unreachable!("target refusal before argument demand")
            },
        )
        .unwrap_err();
        assert!(error.contains(reason), "{error}");
        assert_eq!(projected, 0);
        let result = borrowed_call_arguments_callback_v1(
            &targets,
            &package.parameter_contracts,
            &candidates,
            None,
            source,
            &Ok(Default::default()),
            &mut pending,
            &ledger.borrowed_i64_results,
            &mut |_| None,
            target.call_site(),
            BorrowedCallActualRequestV1::ReceivedHandleArguments(destination),
            &ledger.callable_result_classes,
            &ledger.receiver_call_observations,
        );
        assert!(format!("{:?}", result.unwrap_err()).contains(reason));
        assert_eq!(pending, snapshot);
    }
    let empty = result_class_claim::OrdinaryNewResultClassClaimsV1::new();
    assert!(received_handle_arguments_v1(
        &Err("unrelated-error".into()),
        &[],
        &empty,
        target.call_site(),
        destination,
        &mut |_, _| unreachable!()
    )
    .unwrap()
    .is_none());
    assert!(received_handle_arguments_v1(
        &Ok(vec![Ok(Some(target.clone()))]),
        &candidates,
        &empty,
        target.call_site(),
        destination,
        &mut |_, _| unreachable!()
    )
    .unwrap_err()
    .contains("target-qualification"));
    let nullable = self::package("size", "7", true);
    let (nullable_target, nullable_candidates, nullable_destination) = original(&nullable);
    assert!(received_handle_arguments_v1(
        &Err("unrelated-error".into()),
        &nullable_candidates,
        &nullable.ordinary_new_claim_ledger.callable_result_classes,
        nullable_target.call_site(),
        nullable_destination,
        &mut |_, _| unreachable!()
    )
    .unwrap()
    .is_none());
}
