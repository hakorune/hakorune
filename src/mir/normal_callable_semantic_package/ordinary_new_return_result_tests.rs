use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};

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
fn target(
    package: &VerifiedNormalCallableSemanticPackageV1,
    method: &str,
) -> LexicalInstanceCallSourceTargetV1 {
    let ledger = &package.ordinary_new_claim_ledger;
    let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Maker", method, 0,
    );
    let value = ledger.callable_result_classes.outcomes(&key).unwrap()[0].site();
    let loan = ledger
        .callable_result_classes
        .object_return_qualification(value)
        .unwrap();
    ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .object_source_target_at_v1(loan.call())
        .unwrap()
        .unwrap()
        .clone()
}
fn nested() -> VerifiedNormalCallableSemanticPackageV1 {
    issue_with_brand_catalog("box Token {} box Maker { make() { return new Token() } relay() { return me.make() } outer() { return me.relay() } } static box Main { main() { return 0 } }").unwrap()
}

#[test]
fn completed_object_result_joins_original_typed_opaque_direct_received_cohort() {
    for received in [false, true] {
        for opaque in [false, true] {
            for nullable in [false, true] {
                let package = package(received, opaque, nullable);
                let target = target(&package, "relay");
                let expected = if nullable {
                    InvokeCallResultKind::NullableHandle
                } else {
                    InvokeCallResultKind::Handle
                };
                assert_eq!(
                    package
                        .ordinary_new_claim_ledger
                        .checked_object_callee_result_v1(&target, &package.result_contracts)
                        .unwrap(),
                    Some(expected)
                );
                let (kind, teardown) = package
                    .ordinary_new_claim_ledger
                    .checked_object_callee_result_with_teardown_v1(
                        &target,
                        &package.result_contracts,
                    )
                    .unwrap()
                    .unwrap();
                assert_eq!(kind, expected);
                let (descriptor, observed_null) = teardown.descriptor().unwrap();
                assert_eq!(observed_null, nullable);
                assert!(descriptor.owned_children().is_none(), "Plain remains None");
                assert!(package
                    .ordinary_new_claim_ledger
                    .validate_no_pending_object_returns_v1()
                    .is_err());
            }
        }
    }
}

#[test]
fn completed_object_result_nullable_and_nested_call_cover_original_alternatives() {
    let package = package(false, false, true);
    let target = self::target(&package, "relay");
    assert_eq!(
        package
            .ordinary_new_claim_ledger
            .checked_object_callee_result_v1(&target, &package.result_contracts)
            .unwrap(),
        Some(InvokeCallResultKind::NullableHandle)
    );
    let package = nested();
    let target = self::target(&package, "outer");
    assert_eq!(
        package
            .ordinary_new_claim_ledger
            .checked_object_callee_result_v1(&target, &package.result_contracts)
            .unwrap(),
        Some(InvokeCallResultKind::Handle)
    );
}

#[test]
fn completed_object_result_foreign_cohort_refuses() {
    let package = package(false, false, false);
    let foreign = self::package(false, false, false);
    let target = target(&package, "relay");
    assert!(package
        .ordinary_new_claim_ledger
        .checked_object_callee_result_v1(&target, &foreign.result_contracts)
        .unwrap_err()
        .contains("result-cohort-owner"));
}

#[test]
fn completed_object_result_reissued_original_completion_is_not_same_authority() {
    let mut package = package(false, false, false);
    let target = target(&package, "relay");
    let reissued = package
        .batch
        .with_lowering_input(
            target.target_batch_slot(),
            crate::mir::resolved_control_flow::verify_function_completion_v1,
        )
        .unwrap()
        .unwrap();
    assert_eq!(reissued.owner(), target.callee_owner());
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    assert_eq!(
        reissued.explicit_sites(),
        ledger.completion_index[&target.callee_owner()]
            .as_ref()
            .unwrap()
            .explicit_sites()
    );
    ledger
        .completion_index
        .insert(target.callee_owner(), Ok(Rc::new(reissued)));
    assert!(ledger
        .checked_object_callee_result_v1(&target, &package.result_contracts)
        .unwrap_err()
        .contains("result-cohort-completion-identity"));
}

#[test]
fn completed_object_result_equal_recreated_terminal_table_is_not_original_authority() {
    let mut package = package(false, false, false);
    let target = target(&package, "relay");
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let table = ledger
        .terminal_relation_index
        .get_mut(&target.callee_owner())
        .unwrap();
    let replacement = Rc::new(table.as_ref().clone());
    assert_eq!(
        replacement.keys().collect::<Vec<_>>(),
        table.keys().collect::<Vec<_>>()
    );
    for (copy, original) in replacement.values().zip(table.values()) {
        use crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1;
        let (TerminalRelationV1::Value(copy), TerminalRelationV1::Value(original)) =
            (copy, original)
        else {
            panic!("original fresh value table")
        };
        assert_eq!(copy, original);
    }
    assert!(!Rc::ptr_eq(&replacement, table));
    *table = replacement;
    assert!(ledger
        .checked_object_callee_result_v1(&target, &package.result_contracts)
        .unwrap_err()
        .contains("result-cohort-terminal-identity"));
}

#[test]
fn completed_object_result_missing_and_foreign_callee_dispositions_are_distinct() {
    for foreign_proof in [false, true] {
        let mut package = nested();
        let target = target(&package, "outer");
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let values = ledger.terminal_relations_for_owner(target.callee_owner());
        let [crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Value(value)] =
            values.as_slice()
        else {
            panic!("callee terminal")
        };
        let identity = (value.owner(), value.return_site().clone());
        let replacement = if foreign_proof {
            let foreign = nested();
            let map = foreign
                .ordinary_new_claim_ledger
                .normal_return_dispositions
                .as_ref()
                .unwrap();
            let NormalReturnDispositionV1::Verified { proof } = map.values().next().unwrap() else {
                panic!("foreign proof")
            };
            Some(NormalReturnDispositionV1::Verified {
                proof: Rc::clone(proof),
            })
        } else {
            None
        };
        let map = ledger.normal_return_dispositions.as_mut().unwrap();
        assert!(map.remove(&identity).is_some());
        if let Some(replacement) = replacement {
            map.insert(identity, replacement);
        }
        let result = ledger.checked_object_callee_result_v1(&target, &package.result_contracts);
        if foreign_proof {
            assert!(result
                .unwrap_err()
                .contains("result-call-disposition-identity"));
        } else {
            assert_eq!(result.unwrap(), None);
        }
    }
}

#[test]
fn completed_object_result_retains_original_unqualified_producer_dependencies() {
    let package = issue_with_brand_catalog("box Token {} box Maker { make(size: i64) { if size > 0 { return new Token() } return null } relay() { return me.make(7) } outer() { return me.relay() } ignored() { local item = me.relay() return 0 } } static box Main { main() { return 0 } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let relay = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Maker", "relay", 0,
    );
    let (site, _) = ledger
        .receiver_call_observations
        .iter()
        .find(|(_, observation)| observation.callee() == &relay)
        .unwrap();
    let target = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .object_source_target_at_v1(site)
        .unwrap()
        .unwrap();
    assert!(target.object_return_sources().is_none());
    assert_eq!(
        target.object_producer_dependencies(),
        ledger
            .callable_result_classes
            .object_return_dependencies(target.target(), target.callee_owner())
            .as_deref()
    );
    assert!(ledger
        .callable_result_classes
        .object_return_dependencies(target.target(), target.callee_owner())
        .is_some());
    assert_eq!(
        ledger
            .checked_object_callee_result_v1(target, &package.result_contracts)
            .unwrap(),
        Some(InvokeCallResultKind::NullableHandle)
    );
}

#[test]
fn completed_object_result_null_first_keeps_nullable_kind_and_full_order() {
    let package = issue_with_brand_catalog("box Token {} box Maker { make(size: i64) { if size > 0 { return null } return new Token() } relay() { return me.make(7) } } static box Main { main() { return 0 } }").unwrap();
    let target = target(&package, "relay");
    assert_eq!(
        package
            .ordinary_new_claim_ledger
            .checked_object_callee_result_v1(&target, &package.result_contracts)
            .unwrap(),
        Some(InvokeCallResultKind::NullableHandle)
    );
}
