use super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog, VerifiedNormalCallableSemanticPackageV1,
};
use std::rc::Rc;

fn package(received: bool, opaque: bool) -> VerifiedNormalCallableSemanticPackageV1 {
    let formal = if opaque { "size" } else { "size: i64" };
    let relay = if received {
        "local item = me.make(7) local sibling = new Token() return item"
    } else {
        "local first = new Token() return me.make(7)"
    };
    issue_with_brand_catalog(&format!("box Token {{}} box Maker {{ make({formal}) {{ return new Token() }} relay() {{ {relay} }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
}
fn identity(
    package: &VerifiedNormalCallableSemanticPackageV1,
) -> (FunctionOwnerIdV1, SourceStmtSiteV1) {
    package
        .ordinary_new_claim_ledger
        .normal_return_dispositions
        .as_ref()
        .unwrap()
        .keys()
        .next()
        .unwrap()
        .clone()
}

#[test]
fn normal_projection_direct_received_typed_opaque_preserves_fault_and_order() {
    for received in [false, true] {
        for opaque in [false, true] {
            let package = package(received, opaque);
            let (owner, exit) = identity(&package);
            let ledger = &package.ordinary_new_claim_ledger;
            let projection = ledger
                .normal_exit_projection_v1(owner, &exit)
                .unwrap()
                .unwrap();
            let original = ledger.completion_index[&owner]
                .as_ref()
                .unwrap()
                .cleanup()
                .root_flow()
                .unwrap()
                .exit_row(&exit)
                .unwrap()
                .unwrap();
            assert_eq!(projection.fault_homes(), original.homes());
            assert_eq!(projection.covered_calls(), original.covered_calls());
            let Some(TerminalRelationV1::Value(value)) =
                ledger.terminal_relation_for_owner_at(owner, &exit)
            else {
                panic!("value")
            };
            let TerminalReturnedSourceV1::OwnedCall(obligation) = value.returned() else {
                panic!("owned")
            };
            if received {
                let ObjectReturnAcquisitionV1::Received(call) = obligation.acquisition() else {
                    panic!("received")
                };
                let (_, destination) = call.local_binding().unwrap();
                let expected: Vec<_> = original
                    .homes()
                    .iter()
                    .copied()
                    .filter(|home| *home != destination)
                    .collect();
                assert_eq!(original.homes().len(), 2);
                assert_eq!(projection.homes(), expected);
                assert_eq!(projection.homes().len(), 1);
            } else {
                assert_eq!(projection.homes(), original.homes());
                assert_eq!(projection.homes().as_ptr(), original.homes().as_ptr());
            }
            assert!(ledger.validate_no_pending_object_returns_v1().is_err());
        }
    }
}

#[test]
fn normal_projection_missing_unsealed_unavailable_disposition_never_lends() {
    for mutation in 0..3 {
        let mut package = package(true, false);
        let (owner, exit) = identity(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        match mutation {
            0 => ledger.normal_return_dispositions = None,
            1 => {
                ledger
                    .normal_return_dispositions
                    .as_mut()
                    .unwrap()
                    .remove(&(owner, exit.clone()));
            }
            2 => {
                ledger.normal_return_dispositions.as_mut().unwrap().insert(
                    (owner, exit.clone()),
                    NormalReturnDispositionV1::Unavailable(
                        super::super::ObjectReturnHandoffUnavailableV1::Acquisition,
                    ),
                );
            }
            _ => unreachable!(),
        }
        assert!(ledger
            .normal_exit_projection_v1(owner, &exit)
            .unwrap()
            .is_none());
    }
}

#[test]
fn normal_projection_changed_terminal_value_refuses_saved_proof() {
    let mut package = package(true, false);
    let (owner, exit) = identity(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let Some(TerminalRelationV1::Value(value)) =
        ledger.terminal_relation_for_owner_at(owner, &exit)
    else {
        panic!("value")
    };
    let TerminalReturnedSourceV1::OwnedCall(obligation) = value.returned() else {
        panic!("owned")
    };
    let changed = value.with_value_site_for_test(obligation.qualification().call().site().clone());
    Rc::make_mut(ledger.terminal_relation_index.get_mut(&owner).unwrap())
        .insert(exit.clone(), TerminalRelationV1::Value(changed));
    assert!(ledger
        .normal_exit_projection_v1(owner, &exit)
        .err()
        .unwrap()
        .contains("projection-proof-identity"));
}

#[test]
fn normal_projection_foreign_proof_refuses_even_same_shape() {
    let foreign = package(true, false);
    let foreign_id = identity(&foreign);
    let NormalReturnDispositionV1::Verified { proof } = &foreign
        .ordinary_new_claim_ledger
        .normal_return_dispositions
        .as_ref()
        .unwrap()[&foreign_id]
    else {
        panic!("proof")
    };
    let proof = Rc::clone(proof);
    let mut package = package(true, false);
    let (owner, exit) = identity(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.normal_return_dispositions.as_mut().unwrap().insert(
        (owner, exit.clone()),
        NormalReturnDispositionV1::Verified { proof },
    );
    assert!(ledger
        .normal_exit_projection_v1(owner, &exit)
        .err()
        .unwrap()
        .contains("projection-proof-identity"));
}

#[test]
fn normal_projection_received_destination_missing_or_duplicate_refuses() {
    let package = package(true, false);
    let (owner, exit) = identity(&package);
    let ledger = &package.ordinary_new_claim_ledger;
    let Some(TerminalRelationV1::Value(value)) =
        ledger.terminal_relation_for_owner_at(owner, &exit)
    else {
        panic!("value")
    };
    let TerminalReturnedSourceV1::OwnedCall(obligation) = value.returned() else {
        panic!("owned")
    };
    let ObjectReturnAcquisitionV1::Received(call) = obligation.acquisition() else {
        panic!("received")
    };
    let (_, destination) = call.local_binding().unwrap();
    let missing: Vec<_> = obligation
        .exit_homes()
        .iter()
        .copied()
        .filter(|home| *home != destination)
        .collect();
    let mut duplicate = obligation.exit_homes().to_vec();
    duplicate.push(destination);
    for homes in [missing, duplicate] {
        assert!(received_normal_homes(destination, &homes)
            .unwrap_err()
            .contains("projection-destination-count"));
        assert!(project_normal_homes(obligation, &homes)
            .unwrap_err()
            .contains("projection-exit-homes"));
    }
}

#[test]
fn normal_projection_non_owned_exit_uses_original_and_rejects_spurious_proof() {
    let mut package = package(false, false);
    let source_id = identity(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Maker", "make", 1,
    );
    let owner = ledger.callable_result_classes.outcomes(&key).unwrap()[0]
        .site()
        .owner();
    let exit = ledger.completion_index[&owner]
        .as_ref()
        .unwrap()
        .explicit_sites()[0]
        .clone();
    let projection = ledger
        .normal_exit_projection_v1(owner, &exit)
        .unwrap()
        .unwrap();
    assert_eq!(projection.homes(), projection.fault_homes());
    let disposition = ledger
        .normal_return_dispositions
        .as_mut()
        .unwrap()
        .remove(&source_id)
        .unwrap();
    ledger
        .normal_return_dispositions
        .as_mut()
        .unwrap()
        .insert((owner, exit.clone()), disposition);
    assert!(ledger
        .normal_exit_projection_v1(owner, &exit)
        .err()
        .unwrap()
        .contains("projection-spurious-disposition"));
}

#[test]
fn normal_projection_missing_index_or_terminal_never_retries_root_evidence() {
    for missing_completion in [false, true] {
        let mut package = package(true, false);
        let (owner, exit) = identity(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        if missing_completion {
            ledger.root_completion = Some(ledger.completion_index.remove(&owner).unwrap());
        } else {
            let relation = Rc::make_mut(ledger.terminal_relation_index.get_mut(&owner).unwrap())
                .remove(&exit)
                .unwrap();
            // Keep a root-spelled copy: the exact indexed hole remains missing.
            Rc::make_mut(&mut ledger.terminal_relation).insert(exit.clone(), relation);
        }
        assert!(ledger
            .normal_exit_projection_v1(owner, &exit)
            .unwrap()
            .is_none());
    }
}

#[test]
fn normal_projection_rejected_completion_preserves_original_cause() {
    let mut package = package(true, false);
    let (owner, exit) = identity(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.completion_index.insert(owner, Err(
        crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1::SourceNavigation(
            "original-projection-completion-error".into(),
        ),
    ));
    assert!(ledger
        .normal_exit_projection_v1(owner, &exit)
        .err()
        .unwrap()
        .contains("original-projection-completion-error"));
}

#[test]
fn bool_literal_exits_keep_distinct_source_relations_and_reject_missing_exit() {
    let mut package = issue_with_brand_catalog(
        "box Page { birth() {} } static box Predicate { accepts(size: i64) {
            local page = new Page()
            if size > 0 { return false }
            return true
        } } static box Main { main() { return 0 } }",
    )
    .expect("Bool source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let (owner, exits) = ledger
        .completion_index
        .iter()
        .find_map(|(owner, completion)| {
            let completion = completion.as_ref().ok()?;
            (completion.explicit_sites().len() == 2)
                .then(|| (*owner, completion.explicit_sites().to_vec()))
        })
        .expect("two Bool exits");
    let mut values = Vec::new();
    for exit in &exits {
        let Some(TerminalRelationV1::BoolLiteral(row)) =
            ledger.terminal_relation_for_owner_at(owner, exit)
        else {
            panic!(
                "exact Bool literal relation at {exit:?}: {:?}",
                ledger.terminal_relation_for_owner_at(owner, exit)
            );
        };
        assert_eq!(row.owner(), owner);
        assert_eq!(row.return_site(), exit);
        let projection = ledger
            .normal_exit_projection_v1(owner, exit)
            .unwrap()
            .expect("Bool exit Normal projection");
        let cleanup = ledger.completion_index[&owner]
            .as_ref()
            .unwrap()
            .cleanup();
        let flow = cleanup.root_flow().unwrap();
        let flow_row = flow.exit_row(exit).unwrap();
        let original = flow_row.as_ref().unwrap();
        assert_eq!(projection.homes(), original.homes());
        assert_eq!(projection.fault_homes(), original.homes());
        assert_eq!(original.homes().len(), 1);
        values.push(row.value());
    }
    values.sort();
    assert_eq!(values, [false, true]);

    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    Rc::make_mut(ledger.terminal_relation_index.get_mut(&owner).unwrap()).remove(&exits[0]);
    assert!(ledger
        .normal_exit_projection_v1(owner, &exits[0])
        .unwrap()
        .is_none());
    assert!(ledger
        .normal_exit_projection_v1(owner, &exits[1])
        .unwrap()
        .is_some());
    let sibling = ledger.terminal_relation_index[&owner][&exits[1]].clone();
    Rc::make_mut(ledger.terminal_relation_index.get_mut(&owner).unwrap())
        .insert(exits[0].clone(), sibling);
    assert!(ledger
        .normal_exit_projection_v1(owner, &exits[0])
        .err()
        .expect("wrong-site relation")
        .contains("projection-terminal-owner"));
}
