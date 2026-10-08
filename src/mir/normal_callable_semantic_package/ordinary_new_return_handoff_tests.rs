use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};
use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;

fn package(
    received: bool,
    opaque: bool,
    nullable: bool,
) -> VerifiedNormalCallableSemanticPackageV1 {
    let formal = if opaque { "size" } else { "size: i64" };
    let body = if nullable {
        "if size > 0 { return new Token() } return null"
    } else {
        "return new Token()"
    };
    let relay = if received {
        "local item = me.make(7) local sibling = new Token() return item"
    } else {
        "local first = new Token() return me.make(7)"
    };
    issue_with_brand_catalog(&format!("box Token {{}} box Maker {{ make({formal}) {{ {body} }} relay() {{ {relay} }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
}
fn terminal(
    package: &VerifiedNormalCallableSemanticPackageV1,
    method: &str,
) -> TerminalValueReturnV1 {
    let ledger = &package.ordinary_new_claim_ledger;
    let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Maker", method, 0,
    );
    let owner = ledger.callable_result_classes.outcomes(&key).unwrap()[0]
        .site()
        .owner();
    let rows = ledger.terminal_relations_for_owner(owner);
    let [TerminalRelationV1::Value(value)] = rows.as_slice() else {
        panic!("terminal")
    };
    (*value).clone()
}
fn check(
    package: &VerifiedNormalCallableSemanticPackageV1,
    value: &TerminalValueReturnV1,
) -> Result<ObjectReturnHandoffAvailabilityV1, String> {
    package
        .ordinary_new_claim_ledger
        .checked_object_return_handoff_v1(
            value,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
        )
}

#[test]
fn completed_object_handoff_joins_typed_and_opaque_direct_received_original_leaves() {
    for opaque in [false, true] {
        for received in [false, true] {
            let package = package(received, opaque, false);
            let value = terminal(&package, "relay");
            let ObjectReturnHandoffAvailabilityV1::Verified(proof) =
                check(&package, &value).unwrap()
            else {
                panic!("proof")
            };
            assert!(matches!(
                proof.alternatives(),
                [VerifiedObjectReturnAlternativeV1::Leaf(
                    VerifiedObjectReturnLeafV1::Fresh { .. }
                )]
            ));
            let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
                panic!("owned")
            };
            assert_eq!(proof.acquisition().original(), original.as_ref());
            assert!(package
                .ordinary_new_claim_ledger
                .validate_no_pending_object_returns_v1()
                .is_err());
        }
    }
}

#[test]
fn completed_object_handoff_covers_fresh_and_null_in_original_order() {
    let package = package(false, false, true);
    let value = terminal(&package, "relay");
    let ObjectReturnHandoffAvailabilityV1::Verified(proof) = check(&package, &value).unwrap()
    else {
        panic!("proof")
    };
    assert!(matches!(
        proof.alternatives(),
        [
            VerifiedObjectReturnAlternativeV1::Leaf(VerifiedObjectReturnLeafV1::Fresh { .. }),
            VerifiedObjectReturnAlternativeV1::Leaf(VerifiedObjectReturnLeafV1::Null)
        ]
    ));
}

#[test]
fn completed_object_handoff_retains_nested_original_call_ancestry() {
    let package = issue_with_brand_catalog("box Token {} box Maker { make() { return new Token() } hop() { return me.make() } relay() { return me.hop() } } static box Main { main() { return 0 } }").unwrap();
    let value = terminal(&package, "relay");
    let ObjectReturnHandoffAvailabilityV1::Verified(proof) = check(&package, &value).unwrap()
    else {
        panic!("proof")
    };
    let [VerifiedObjectReturnAlternativeV1::Call(child)] = proof.alternatives() else {
        panic!("child")
    };
    assert!(matches!(
        child.alternatives(),
        [VerifiedObjectReturnAlternativeV1::Leaf(
            VerifiedObjectReturnLeafV1::Fresh { .. }
        )]
    ));
}

#[test]
fn completed_object_handoff_missing_callee_exit_is_wholly_unavailable() {
    let mut package = package(false, false, true);
    let value = terminal(&package, "relay");
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
        panic!("owned")
    };
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let owner = source
        .object_source_target_at_v1(original.qualification().call())
        .unwrap()
        .unwrap()
        .callee_owner();
    let rows = Rc::make_mut(ledger.terminal_relation_index.get_mut(&owner).unwrap());
    let exit = rows.keys().next().unwrap().clone();
    rows.remove(&exit);
    assert!(matches!(
        check(&package, &value).unwrap(),
        ObjectReturnHandoffAvailabilityV1::Unavailable(
            ObjectReturnHandoffUnavailableV1::Acquisition
        )
    ));
}

#[test]
fn completed_object_handoff_cycle_guard_does_not_cache_or_leave_path_entries() {
    let package = package(false, false, false);
    let value = terminal(&package, "relay");
    let identity = (value.owner(), value.return_site().clone());
    let mut path = BTreeSet::from([identity.clone()]);
    let mut memo = ObjectReturnHandoffMemoV1::default();
    assert!(matches!(
        package
            .ordinary_new_claim_ledger
            .checked_object_return_handoff_path_v1(
                &value,
                &package.selected,
                &package.parameter_contracts,
                &package.physical_signature,
                &mut path,
                &mut memo,
            )
            .unwrap(),
        ObjectReturnHandoffAvailabilityV1::Unavailable(
            ObjectReturnHandoffUnavailableV1::RecursivePath
        )
    ));
    assert_eq!(path, BTreeSet::from([identity]));
    assert!(memo.completed.is_empty());
}

#[test]
fn completed_object_handoff_memo_shares_one_pass_proof_and_rejects_terminal_drift() {
    let package = package(false, false, false);
    let value = terminal(&package, "relay");
    let ledger = &package.ordinary_new_claim_ledger;
    let mut memo = ObjectReturnHandoffMemoV1::default();
    let ObjectReturnHandoffAvailabilityV1::Verified(first) = ledger
        .checked_object_return_handoff_with_memo_v1(
            &value,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
            &mut memo,
        )
        .unwrap()
    else {
        panic!("first")
    };
    let ObjectReturnHandoffAvailabilityV1::Verified(second) = ledger
        .checked_object_return_handoff_with_memo_v1(
            &value,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
            &mut memo,
        )
        .unwrap()
    else {
        panic!("second")
    };
    assert!(Rc::ptr_eq(&first, &second));
    let foreign = terminal(&self::package(false, false, false), "relay");
    memo.completed
        .get_mut(&(value.owner(), value.return_site().clone()))
        .unwrap()
        .0 = foreign;
    assert!(ledger
        .checked_object_return_handoff_with_memo_v1(
            &value,
            &package.selected,
            &package.parameter_contracts,
            &package.physical_signature,
            &mut memo,
        )
        .unwrap_err()
        .contains("handoff-memo-terminal-identity"));
}

#[test]
fn completed_object_handoff_missing_leaf_does_not_hide_later_corrupt_sibling() {
    let package = issue_with_brand_catalog("box Token {} box Maker { make(size: i64) { if size > 0 { return new Token() } return new Token() } relay() { local first = new Token() return me.make(7) } } static box Main { main() { return 0 } }").unwrap();
    let value = terminal(&package, "relay");
    let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
        panic!("owned")
    };
    let children: Vec<_> = original
        .qualification()
        .witnesses()
        .iter()
        .map(|row| {
            let ResultWitnessStepV1::Call { callee, .. } = row.step() else {
                panic!("call")
            };
            callee.site().clone()
        })
        .collect();
    assert_eq!(children.len(), 2);
    let ledger = &package.ordinary_new_claim_ledger;
    let mut claims = ledger.result_claims.borrow_mut();
    let removed = claims.remove(&children[0]).unwrap();
    let foreign_prefix = removed.home_prefix().unwrap().clone();
    drop(claims);
    assert!(matches!(
        check(&package, &value).unwrap(),
        ObjectReturnHandoffAvailabilityV1::Unavailable(ObjectReturnHandoffUnavailableV1::Leaf)
    ));
    let mut claims = ledger.result_claims.borrow_mut();
    claims.get_mut(&children[1]).unwrap().home_prefix = Ok(foreign_prefix);
    drop(claims);
    assert!(check(&package, &value)
        .unwrap_err()
        .contains("leaf-prefix-identity"));
}

#[test]
fn completed_object_teardown_uses_same_fresh_descriptor_for_direct_received_and_nested_calls() {
    for received in [false, true] {
        for opaque in [false, true] {
            for nullable in [false, true] {
                let package = package(received, opaque, nullable);
                let value = terminal(&package, "relay");
                let ObjectReturnHandoffAvailabilityV1::Verified(proof) =
                    check(&package, &value).unwrap()
                else {
                    panic!("sealed proof");
                };
                let (descriptor, observed_null) = proof.teardown().descriptor().unwrap();
                assert_eq!(observed_null, nullable);
                let VerifiedObjectReturnAlternativeV1::Leaf(VerifiedObjectReturnLeafV1::Fresh {
                    teardown,
                    ..
                }) = &proof.alternatives()[0]
                else {
                    panic!("original Fresh");
                };
                assert_eq!(descriptor, teardown);
                assert!(package
                    .ordinary_new_claim_ledger
                    .validate_no_pending_object_returns_v1()
                    .is_err());
            }
        }
    }
    let package = issue_with_brand_catalog("box Token {} box Maker { make() { return new Token() } hop() { return me.make() } relay() { return me.hop() } } static box Main { main() { return 0 } }").unwrap();
    let value = terminal(&package, "relay");
    let ObjectReturnHandoffAvailabilityV1::Verified(proof) = check(&package, &value).unwrap()
    else {
        panic!("proof");
    };
    let [VerifiedObjectReturnAlternativeV1::Call(child)] = proof.alternatives() else {
        panic!("child");
    };
    assert_eq!(proof.teardown().descriptor(), child.teardown().descriptor());
}

#[test]
fn completed_object_teardown_rejects_foreign_fresh_and_all_null_without_new_claims() {
    // Both source descriptors belong to this same issued cohort.
    let original = issue_with_brand_catalog("box Token {} box Other {} box Maker { make(size: i64) { if size > 0 { return new Token() } return null } relay() { return me.make(7) } other() { return new Other() } otherRelay() { return me.other() } } static box Main { main() { return 0 } }").unwrap();
    let value = terminal(&original, "relay");
    let ObjectReturnHandoffAvailabilityV1::Verified(proof) = check(&original, &value).unwrap()
    else {
        panic!("proof");
    };
    let null = &proof.alternatives()[1];
    assert!(super::teardown::reduce(std::slice::from_ref(null))
        .unwrap()
        .descriptor()
        .is_none());
    let value = terminal(&original, "otherRelay");
    let ObjectReturnHandoffAvailabilityV1::Verified(other_proof) =
        check(&original, &value).unwrap()
    else {
        panic!("other proof");
    };
    let mixed = [
        VerifiedObjectReturnAlternativeV1::Call(Rc::clone(&proof)),
        VerifiedObjectReturnAlternativeV1::Call(other_proof),
    ];
    assert!(super::teardown::reduce(&mixed)
        .unwrap_err()
        .contains("descriptor-disagreement"));
}

#[test]
fn completed_object_teardown_owned_array_is_exact_and_nullable_fields_stay_unavailable() {
    for nullable in [false, true] {
        let body = if nullable {
            "if size > 0 { return new Token() } return null"
        } else {
            "return new Token()"
        };
        let package = issue_with_brand_catalog(&format!("box Token {{ items: ArrayBox = new ArrayBox() birth() {{}} }} box Maker {{ make(size: i64) {{ {body} }} relay() {{ return me.make(7) }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap();
        let value = terminal(&package, "relay");
        let ObjectReturnHandoffAvailabilityV1::Verified(proof) = check(&package, &value).unwrap()
        else {
            panic!("proof");
        };
        assert_eq!(proof.teardown().descriptor().is_some(), !nullable);
        assert!(package
            .ordinary_new_claim_ledger
            .validate_no_pending_object_returns_v1()
            .is_err());
    }
}

#[test]
fn completed_object_teardown_unavailable_child_retains_descriptor_and_null_ancestry() {
    let package = issue_with_brand_catalog("box Token { items: ArrayBox = new ArrayBox() birth() {} } box Other {} box Maker { make(size: i64) { if size > 0 { return new Token() } return null } relay() { return me.make(7) } other() { return new Other() } otherRelay() { return me.other() } } static box Main { main() { return 0 } }").unwrap();
    let value = terminal(&package, "relay");
    let ObjectReturnHandoffAvailabilityV1::Verified(unsupported) = check(&package, &value).unwrap()
    else {
        panic!("source proof");
    };
    assert!(matches!(
        unsupported.teardown(),
        ObjectReturnTeardownAvailabilityV1::Unavailable {
            descriptor: Some(_),
            nullable: true,
            unsupported: true,
        }
    ));
    // Wrapping an unavailable owned-null ancestry never loses that evidence.
    let nested = [VerifiedObjectReturnAlternativeV1::Call(Rc::clone(
        &unsupported,
    ))];
    assert!(matches!(
        super::teardown::reduce(&nested).unwrap(),
        ObjectReturnTeardownAvailabilityV1::Unavailable {
            descriptor: Some(_),
            nullable: true,
            unsupported: true,
        }
    ));
    let value = terminal(&package, "otherRelay");
    let ObjectReturnHandoffAvailabilityV1::Verified(other) = check(&package, &value).unwrap()
    else {
        panic!("other source proof");
    };
    for reversed in [false, true] {
        let mut mixed = vec![
            VerifiedObjectReturnAlternativeV1::Call(Rc::clone(&unsupported)),
            VerifiedObjectReturnAlternativeV1::Call(Rc::clone(&other)),
        ];
        if reversed {
            mixed.reverse();
        }
        assert!(super::teardown::reduce(&mixed)
            .unwrap_err()
            .contains("descriptor-disagreement"));
    }
}
