use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    ObjectReturnAcquisitionV1, TerminalReturnedSourceV1, TerminalValueReturnV1,
};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

fn package(received: bool, nullable: bool) -> VerifiedNormalCallableSemanticPackageV1 {
    let make = if nullable {
        "if size > 0 { return new Token() } return null"
    } else {
        "return new Token()"
    };
    let relay = if received {
        "local item = me.make(size) local sibling = new Token() return item"
    } else {
        "local first = new Token() local second = new Token() return me.make(size)"
    };
    issue_with_brand_catalog(&format!("box Token {{}} box Maker {{ make(size: i64) {{ {make} }} relay(size: i64) {{ {relay} }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
}
fn value(package: &VerifiedNormalCallableSemanticPackageV1) -> TerminalValueReturnV1 {
    let ledger = &package.ordinary_new_claim_ledger;
    let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Maker", "relay", 1);
    let owner = ledger.callable_result_classes.outcomes(&key).unwrap()[0]
        .site()
        .owner();
    let relations = ledger.terminal_relations_for_owner(owner);
    let [TerminalRelationV1::Value(value)] = relations.as_slice() else {
        panic!("owned return: {relations:?}")
    };
    (*value).clone()
}

#[test]
fn source_object_return_retains_direct_original_arguments_and_pending_boundary() {
    for nullable in [false, true] {
        let package = package(false, nullable);
        let value = value(&package);
        let ledger = &package.ordinary_new_claim_ledger;
        let TerminalReturnedSourceV1::OwnedCall(obligation) = value.returned() else {
            panic!("owned call")
        };
        let ObjectReturnAcquisitionV1::Direct {
            argument_sites,
            fault_homes,
        } = obligation.acquisition()
        else {
            panic!("direct")
        };
        assert_eq!(argument_sites.len(), 1);
        assert_eq!(argument_sites[0].owner(), value.owner());
        let row = package
            .result_contracts
            .rows()
            .find(|row| row.borrow().completion().owner() == value.owner())
            .unwrap();
        package
            .batch()
            .with_lowering_input(row.batch_slot(), |input| {
                let call = input
                    .function()
                    .method_call(obligation.qualification().call().site())
                    .unwrap();
                assert_eq!(
                    argument_sites.as_ref(),
                    call.arguments()
                        .iter()
                        .map(|arg| crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                            input.owner(),
                            arg.site().clone()
                        ))
                        .collect::<Vec<_>>()
                        .as_slice()
                );
            })
            .unwrap();

        assert_eq!(fault_homes.as_ref(), obligation.exit_homes());
        assert_eq!(fault_homes.len(), 2);
        assert_eq!(
            ledger
                .callable_result_classes
                .object_return_qualification(obligation.qualification().value())
                .as_ref(),
            Some(obligation.qualification())
        );
        assert!(ledger.has_pending_object_return_v1(value.owner()));
        assert!(ledger
            .validate_no_pending_object_returns_v1()
            .unwrap_err()
            .contains("object-return-handoff-unavailable"));
        assert!(ledger
            .object_return_construction_ready_v1(value.owner())
            .unwrap());
        assert_eq!(
            ledger
                .prepare_root_home_exit(value.owner(), value.return_site().node())
                .unwrap_err(),
            "[freeze:contract][ordinary-new/local-commit/root-home-not-installed]"
        );
    }
}

#[test]
fn source_object_return_retains_received_call_and_distinct_fault_exit_homes() {
    for nullable in [false, true] {
        let package = package(true, nullable);
        let value = value(&package);
        let ledger = &package.ordinary_new_claim_ledger;
        let TerminalReturnedSourceV1::OwnedCall(obligation) = value.returned() else {
            panic!("owned call")
        };
        let ObjectReturnAcquisitionV1::Received(original) = obligation.acquisition() else {
            panic!("received")
        };
        let (_, binding) = original.local_binding().unwrap();
        assert!(!original.prior_homes().contains(&binding));
        assert_eq!(obligation.exit_homes().last(), Some(&binding));
        assert_eq!(obligation.exit_homes().len(), 2);
        assert_eq!(original.site(), obligation.qualification().call());
        assert!(ledger.has_pending_object_return_v1(value.owner()));
        assert!(ledger.validate_no_pending_object_returns_v1().is_err());
        assert!(ledger
            .object_return_construction_ready_v1(value.owner())
            .unwrap());
        assert_eq!(
            ledger
                .prepare_root_home_exit(value.owner(), value.return_site().node())
                .unwrap_err(),
            "[freeze:contract][ordinary-new/local-commit/root-home-not-installed]"
        );
    }
}

#[test]
fn root_only_pending_object_return_survives_same_owner_empty_index_and_artifact_entries() {
    let mut package = package(false, false);
    let value = value(&package);
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    std::rc::Rc::make_mut(&mut ledger.terminal_relation).clear();
    ledger.terminal_relation_index.clear();
    std::rc::Rc::make_mut(&mut ledger.terminal_relation).insert(
        value.return_site().clone(),
        TerminalRelationV1::Value(value.clone()),
    );
    ledger.terminal_relation_index.insert(
        value.owner(),
        std::rc::Rc::new(std::collections::BTreeMap::new()),
    );
    assert!(ledger.has_pending_object_return_v1(value.owner()));
    assert!(ledger.validate_no_pending_object_returns_v1().is_err());
    assert!(!ledger
        .prepare_root_home_exit(value.owner(), value.return_site().node())
        .unwrap());
    let module = crate::mir::MirModule::new("pending-source-only".into());
    // Finishing an empty child set grants no whole-module artifact authority.
    assert!(ledger
        .validate_finalized_child_functions(&module, true)
        .unwrap()
        .is_empty());
    let function = crate::mir::MirFunction::new(
        crate::mir::FunctionSignature {
            name: "pending-source-only".into(),
            params: vec![],
            return_type: crate::mir::MirType::Void,
            effects: crate::mir::EffectMask::PURE,
        },
        crate::mir::BasicBlockId(0),
    );
    assert!(ledger
        .validate_artifact_after_compiler_finishing(&function)
        .unwrap_err()
        .contains("artifact-root-not-checked"));
    let issue = package
        .ordinary_new_claim_ledger
        .seal_finalized_root_birth_handoff(
            "pending-source-only".into(),
            &module,
            &std::collections::BTreeSet::new(),
            None,
        )
        .err()
        .expect("unfinalized source cannot bypass whole-module handoff");
    assert!(issue.contains("artifact-root-not-finished"));
}

#[test]
fn zero_argument_direct_object_return_is_pending_source_not_empty_argument_permission() {
    const SOURCE: &str = "box Token {} box Maker { make() { return new Token() } relay() { return me.make() } } static box Main { main() { return 0 } }";
    let package = issue_with_brand_catalog(SOURCE).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Maker", "relay", 0);
    let owner = ledger.callable_result_classes.outcomes(&key).unwrap()[0]
        .site()
        .owner();
    let relations = ledger.terminal_relations_for_owner(owner);
    let [TerminalRelationV1::Value(value)] = relations.as_slice() else {
        panic!("original pending source")
    };
    let TerminalReturnedSourceV1::OwnedCall(obligation) = value.returned() else {
        panic!("owned call")
    };
    let ObjectReturnAcquisitionV1::Direct { argument_sites, .. } = obligation.acquisition() else {
        panic!("direct")
    };
    assert!(argument_sites.is_empty());
    assert!(ledger.has_pending_object_return_v1(owner));
    assert!(ledger.validate_no_pending_object_returns_v1().is_err());
    assert!(ledger.object_return_construction_ready_v1(owner).unwrap());
    assert!(ledger
        .prepare_root_home_exit(owner, value.return_site().node())
        .unwrap());
    assert!(ledger.validate_no_pending_object_returns_v1().is_err());

    // Zero actuals do not replace the original source completion proof.
    let mut missing = issue_with_brand_catalog(SOURCE).unwrap();
    let owner = missing
        .ordinary_new_claim_ledger
        .callable_result_classes
        .outcomes(&key)
        .unwrap()[0]
        .site()
        .owner();
    let exit = missing
        .ordinary_new_claim_ledger
        .terminal_relations_for_owner(owner)[0]
        .return_site()
        .clone();
    let ledger = std::rc::Rc::get_mut(&mut missing.ordinary_new_claim_ledger).unwrap();
    ledger.completion_index.remove(&owner);
    assert!(!ledger.prepare_root_home_exit(owner, exit.node()).unwrap());
    assert!(ledger.validate_no_pending_object_returns_v1().is_err());
}

#[test]
fn foreign_selected_return_qualification_refuses_without_scalar_retry() {
    const SOURCE: &str = "box Token {} box Maker { make() { return new Token() } relay() { return me.make() } } static box Main { main() { return 0 } }";
    let package = issue_with_brand_catalog(SOURCE).unwrap();
    let foreign = issue_with_brand_catalog(SOURCE).unwrap();
    let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Maker", "relay", 0);
    let ledger = &package.ordinary_new_claim_ledger;
    let site = ledger.callable_result_classes.outcomes(&key).unwrap()[0].site();
    let foreign_site = foreign
        .ordinary_new_claim_ledger
        .callable_result_classes
        .outcomes(&key)
        .unwrap()[0]
        .site();
    let foreign_loan = foreign
        .ordinary_new_claim_ledger
        .callable_result_classes
        .object_return_qualification(foreign_site)
        .unwrap();
    let row = package
        .result_contracts
        .rows()
        .find(|row| row.borrow().completion().owner() == site.owner())
        .unwrap();
    let contract = row.borrow();
    let mut asks = 0;
    package.batch().with_lowering_input(row.batch_slot(), |input| {
        let (_, flow, relations, _, _) = crate::mir::resolved_semantics::home_new_prefix::scan_new_home_flow(
            input, &std::collections::BTreeMap::new(), std::iter::empty(), None,
            contract.completion().explicit_sites(), contract.completion().implicit_body_end().is_some(),
            &std::collections::BTreeSet::new(),
            &mut |_, _, _, _, _| Ok::<_, std::convert::Infallible>(false),
            &mut |_, _| Ok(false),
            &mut |_| -> Result<bool, std::convert::Infallible> { panic!("no scalar terminal retry") },
            &mut |_| Ok(false), &mut |_| Ok(false),
            &mut |_| -> Result<bool, std::convert::Infallible> { panic!("no borrowed i64 terminal retry") },
            &mut |_| Ok(false), &mut |_| Ok(false), &mut |_| Ok(None),
            &mut |_, _, _, _, _| Ok(false), &mut |_, _, _, _, _| Ok(false),
            &mut |_, _, _, _, _| Ok(false), &mut |_, _, _, _, _| Ok(false),
            &mut |_, _, _, _, _| Ok(false), &mut |_, _| Ok(None),
            &mut |_, request| {
                if matches!(request, crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1::ScalarArguments) {
                    panic!("no scalar argument retry");
                }
                Ok(None)
            }, &mut |_, _| Ok(false), &mut |_| Ok(None),
            &mut |requested| { assert_eq!(requested, site); asks += 1; Ok(Some(foreign_loan.clone())) },
        ).unwrap();
        assert!(flow.terminal_homes().is_err());
        assert!(relations.is_empty());
    }).unwrap();
    assert_eq!(asks, 1);
}
