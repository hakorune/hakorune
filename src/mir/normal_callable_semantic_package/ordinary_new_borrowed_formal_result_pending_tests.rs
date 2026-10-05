//! Pending source evidence cannot regain permission through a result annotation.
use super::*;

#[test]
fn pending_stored_result_rejects_projection_and_completion_permission() {
    for annotated in [false, true] {
        let result = if annotated { ": i64" } else { "" };
        let text = format!(
            "box Item {{ value: i64 birth() {{ me.value = 5 }} }}
            box Leaf {{ flag: i64 birth() {{ me.flag = 0 }}
                read(p){result} {{ if p == null {{ return 7 }} return p.value }} }}
            box Parent {{ child: Leaf birth() {{ me.child = new Leaf() }}
                run(p: Item){result} {{ return me.child.read(p) }} }}
            static box Main {{ main() {{ local parent = new Parent() local item = new Item()
                return parent.run(item) }} }}"
        );
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text)
            .expect("healthy opaque callee with declared forwarding caller");
        let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let target = source
            .incoming
            .iter()
            .find(|row| row.source.stored_receiver().is_some())
            .expect("source-issued stored receiver")
            .source
            .clone();
        let prepared: Vec<_> = source
            .incoming
            .iter()
            .map(|row| Ok(Some(row.source.clone())))
            .collect();
        let proof = ledger
            .borrowed_i64_results
            .get_mut(&target.callee_owner())
            .unwrap()
            .as_mut()
            .unwrap();
        assert!(proof.contract_corroborated);
        proof.phase = BorrowedResultSourcePhaseV1::Pending {
            fields: Box::new([]),
            calls: Box::new([]),
        };
        let issue = super::super::super::borrowed_formal_actuals::project_pending_borrowed_i64_arguments_v1(
            ledger.borrowed_formal_source.as_ref().unwrap(), &ledger.borrowed_formal_actuals,
            &ledger.borrowed_i64_results, target.call_site()).unwrap_err();
        assert!(
            issue.contains("borrowed-result/source-not-sealed"),
            "{issue}"
        );
        ledger.corroborate_borrowed_result_cohort_v1(&prepared, &package.result_contracts);
        let issue = ledger
            .borrowed_i64_results
            .get(&target.callee_owner())
            .unwrap()
            .as_ref()
            .unwrap_err();
        assert!(
            issue.contains("borrowed-result/source-not-sealed"),
            "{issue}"
        );
    }
}

#[test]
fn grounded_weak_stored_receiver_retains_issuance_failure() {
    let text = "box Item { value: i64 birth() { me.value = 5 } }
        box Leaf { flag: i64 birth() { me.flag = 0 }
            read(p) { if p == null { return 7 } return p.value } }
        box Parent { weak child: Leaf birth() { me.child = new Leaf() }
            run(p: Item) { return me.child.read(p) } }
        static box Main { main() { local parent = new Parent() local item = new Item()
            return parent.run(item) } }";
    let issue = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text)
        .expect_err("a weak provider is not an owned stored receiver");
    let issue = format!("{issue:?}");
    assert!(
        issue.contains("stored-child/receiver-unavailable"),
        "{issue}"
    );
    assert!(issue.contains("BorrowedFormalIngress"), "{issue}");
}

#[test]
fn conflicting_conditional_classes_do_not_depend_on_wrapper_order() {
    let first = "first(p: A) { return me.child.read(p) }";
    let second = "second(p: B) { return me.child.read(p) }";
    for methods in [format!("{first} {second}"), format!("{second} {first}")] {
        let text = format!(
            "box A {{ value: i64 birth() {{ me.value = 5 }} }}
            box B {{ other: i64 birth() {{ me.other = 6 }} }}
            box Leaf {{ flag: i64 birth() {{ me.flag = 0 }}
                read(p) {{ if p == null {{ return 7 }} return p.value }} }}
            box Parent {{ child: Leaf birth() {{ me.child = new Leaf() }} {methods} }}
            static box Main {{ main() {{ local parent = new Parent() local a = new A()
                local b = new B() local ignored = parent.second(b) return parent.first(a) }} }}"
        );
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text)
            .expect("unselected conditional conflict retains the existing outside-profile boundary");
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        assert!(
            source.definitions.is_empty(),
            "no conditional candidate enters strict ingress"
        );
        assert!(source.incoming.is_empty());
        assert!(source.object_views.is_empty());
        assert!(
            ledger.borrowed_i64_results.is_empty(),
            "no result permission is minted"
        );
        let calls = ledger.lexical_instance_calls.borrow();
        let mut lexical = 0;
        for row in calls.values() {
            if let crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1::Ready(row) = row {
                assert!(row.source_target().stored_receiver().is_none(), "no stored call is promoted");
                lexical += 1;
            }
        }
        assert_eq!(
            lexical, 2,
            "both existing main lexical calls remain selected"
        );
    }
}

#[test]
fn selected_stored_result_retains_foreign_actual_ingress_failure() {
    let text = "box Item { value: i64 birth() { me.value = 5 } }
        box Other { value: i64 birth() { me.value = 6 } }
        box Leaf { flag: i64 birth() { me.flag = 0 }
            read(p) { if p == null { return 7 } return p.value } }
        box Parent { child: Leaf birth() { me.child = new Leaf() }
            run(p: Item) { return me.child.read(p) } }
        static box Main { main() { local parent = new Parent() local item = new Other()
            return parent.run(item) } }";
    let issue = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text)
        .expect_err("foreign class cannot satisfy the declared incoming view");
    let issue = format!("{issue:?}");
    assert!(
        issue.contains("borrowed-view/declared-object-class"),
        "{issue}"
    );
    assert!(
        !issue.contains("stored-child/result-source-missing"),
        "{issue}"
    );
}
