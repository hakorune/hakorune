use super::*;

#[test]
fn borrowed_source_error_demands_parameter_target_but_not_strict_sibling() {
    let source = "box ParamStore { birth() { } readData(p): i64 { return 0 } readStrict(q: i64): i64 { return q } }
        box ParamManifest { birth() { } materialize(store): i64 {
            local out = store.readData(0) local strict = store.readStrict(1) return 0
        } }
        static box Main { main() {
            local store = new ParamStore() local manifest = new ParamManifest()
            local out = manifest.materialize(store) return 0
        } }";
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    // Final issuance consumes the preparation; its Ready rows retain the
    // identical source targets. Borrow those rows without another resolver.
    let slots = ledger.lexical_instance_calls.borrow();
    let targets: super::super::super::lexical_instance_call::PreparedLexicalInstanceCallSourceTargetsV1 = Ok(slots.values().filter_map(|slot| match slot {
        crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalInstanceCallDispositionSlotV1::Ready(row) => Some(Ok(Some(row.source_target().clone()))),
        _ => None,
    }).collect());
    for (name, borrowed) in [("readData", true), ("readStrict", false)] {
        let target = targets
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|row| row.as_ref().ok().and_then(Option::as_ref))
            .find(|target| target.target().name() == name)
            .unwrap();
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| row.owner == target.call_site().owner())
            .unwrap();
        package
            .batch()
            .with_lowering_input(contract.batch_slot, |input| {
                assert!(matches!(
                    input
                        .function()
                        .binding(target.receiver_binding().unwrap())
                        .unwrap()
                        .kind(),
                    BindingKindV1::Parameter { .. }
                ));
            })
            .unwrap();
        let mut pending = BTreeMap::new();
        let result = borrowed_call_arguments_callback_v1(package.batch(), &package.selected,
            &targets, &[],
            &package.parameter_contracts,
            &[],
            None,
            &Err("source-sentinel".into()),
            &Ok(std::collections::BTreeSet::new()),
            &mut pending,
            &crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedResultCohortV1::default(),
            &mut |_| None,
            target.call_site(),
            BorrowedCallActualRequestV1::ScalarArguments,
            &ledger.callable_result_classes,
            &ledger.receiver_call_observations, &ledger.field_write_claims, None,
        );
        if borrowed {
            assert!(
                matches!(result, Err(OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { issue, .. }) if issue == "source-sentinel")
            );
        } else {
            assert!(result.unwrap().is_none());
        }
        assert!(
            pending.is_empty(),
            "demand does not stage a second inventory"
        );
    }
}
