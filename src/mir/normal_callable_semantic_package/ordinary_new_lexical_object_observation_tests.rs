use super::*;
use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;
use std::rc::Rc;

#[test]
fn object_slot_indexed_caller_missing_or_refused_cannot_use_root_copy() {
    for rejected in [false, true] {
        let mut package = issue_with_brand_catalog("box Token {} box Maker { make(size: i64) { if size > 0 { return new Token() } return null } relay() { return me.make(7) } ignored() { local item = me.relay() return 0 } } static box Main { main() { return 0 } }").unwrap();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let mut slots = ledger.lexical_instance_calls.borrow_mut();
        let (site, slot) = slots.iter_mut().find(|(_, slot)| matches!(slot,
            LexicalInstanceCallDispositionSlotV1::Ready(row) if row.source.object_producer_dependencies().is_some())).unwrap();
        let site = site.clone();
        let LexicalInstanceCallDispositionSlotV1::Ready(row) = slot else {
            panic!("ready")
        };
        *slot = LexicalInstanceCallDispositionSlotV1::SourcePending(row.source.clone());
        drop(slots);
        ledger.root_completion = Some(ledger.completion_index.remove(&site.owner()).unwrap());
        if rejected {
            ledger.completion_index.insert(site.owner(), Err(crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::SourceNavigation("original-caller-refusal".into())));
        }
        let result = ledger.finish_object_lexical_slots_v1(
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
            &package.result_contracts,
        );
        if rejected {
            assert!(result.unwrap_err().contains("original-caller-refusal"));
        } else {
            result.unwrap();
        }
        assert!(matches!(
            ledger.lexical_instance_calls.borrow().get(&site),
            Some(LexicalInstanceCallDispositionSlotV1::SourcePending(_))
        ));
    }
}

#[test]
fn object_slot_original_observation_result_identity_and_duplicates_are_checked() {
    let package = issue_with_brand_catalog("box Token {} box Maker { make() { return new Token() } relay() { local item = me.make() return item } } static box Main { main() { return 0 } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let slots = ledger.lexical_instance_calls.borrow();
    let row = slots
        .values()
        .find_map(|slot| match slot {
            LexicalInstanceCallDispositionSlotV1::Ready(row)
                if row.source.object_return_sources().is_some() =>
            {
                Some(row)
            }
            _ => None,
        })
        .unwrap();
    let flow = ledger.completion_index[&row.call_site().owner()]
        .as_ref()
        .unwrap()
        .cleanup()
        .root_flow()
        .unwrap();
    let original = flow
        .local_calls()
        .iter()
        .find(|observation| observation.site() == row.call_site())
        .unwrap();
    assert!(corroborate_original_object_observations_v1(
        row.call_site(),
        Some(InvokeCallResultKind::Handle),
        std::slice::from_ref(original)
    )
    .unwrap());
    assert!(corroborate_original_object_observations_v1(
        row.call_site(),
        Some(InvokeCallResultKind::NullableHandle),
        std::slice::from_ref(original)
    )
    .unwrap_err()
    .contains("result-observation"));
    assert!(corroborate_original_object_observations_v1(
        row.call_site(),
        Some(InvokeCallResultKind::Handle),
        &[original.clone(), original.clone()]
    )
    .unwrap_err()
    .contains("observation-identity"));
    let (declaration, binding) = original.local_binding().unwrap();
    for role in [LocalCallResultClassV1::I64, LocalCallResultClassV1::Map] {
        let wrong = LocalCallObservationV1::issue(
            original.owner(),
            original.statement().clone(),
            original.site().clone(),
            declaration.clone(),
            binding,
            original.prior_homes().to_vec().into_boxed_slice(),
            original.arguments().to_vec().into_boxed_slice(),
            role,
        );
        assert!(
            corroborate_original_object_observations_v1(row.call_site(), None, &[wrong])
                .unwrap_err()
                .contains("result-observation")
        );
    }
}
