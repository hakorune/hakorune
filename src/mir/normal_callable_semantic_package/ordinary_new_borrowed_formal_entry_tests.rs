use super::*;
use crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
use std::rc::Rc;

fn package(body: &str, main: &str) -> VerifiedNormalCallableSemanticPackageV1 {
    let source = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ {body} }} sink(q): i64 {{ return 0 }} }} static box Main {{ main() {{ {main} }} }}");
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .unwrap()
}

fn target(
    package: &VerifiedNormalCallableSemanticPackageV1,
) -> (
    FunctionOwnerIdV1,
    Vec<(u32, BindingRefV1, CallableParameterContractKindV1)>,
) {
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let owner = source.incoming[0].callee;
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == owner)
        .unwrap();
    (
        owner,
        contract
            .parameters
            .iter()
            .map(|row| (row.ordinal, row.binding, row.kind))
            .collect(),
    )
}

#[test]
fn installed_ordinary_loan_borrows_all_actual_domains_without_reclassifying_formal() {
    let source = "box Transport { birth() { } probe(p): i64 { return 0 }
        int_caller(): i64 { local recv = new Transport() local out = recv.probe(-7) return 0 }
        bool_caller(): i64 { local recv = new Transport() local out = recv.probe(true) return 0 }
        home_caller(): i64 { local recv = new Transport() local out = recv.probe(recv) return 0 }
    } static box Main { main() { return 0 } }";
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
    let (owner, parameters) = target(&package);
    let declaration = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == owner)
        .unwrap();
    let key = package
        .selected
        .key_for_batch_slot(declaration.batch_slot)
        .unwrap()
        .clone();
    let ledger = Rc::clone(&package.ordinary_new_claim_ledger);
    let mut context = crate::mir::builder::CompilationContext::new();
    let installed = package.prepare_install(&mut context).unwrap().commit();
    let mut port = installed.begin_lowering(&context).unwrap();
    port.with_selected_lowering_input(&key, |input| {
        let projection = ledger
            .borrowed_ordinary_entry_source_v1(&input)
            .unwrap()
            .unwrap();
        assert_eq!(projection.owner(), owner);
        assert_eq!(projection.formals(), &[(0, parameters[0].1)]);
        assert_eq!(projection.incoming().len(), 3);
        assert_eq!(
            projection.origins().get(&parameters[0].1),
            Some(&parameters[0].1)
        );
        assert_eq!(
            input.parameter_contracts().next().unwrap().2,
            CallableParameterContractKindV1::OpaqueHandle
        );
        let actuals: Vec<_> = projection
            .incoming()
            .iter()
            .flat_map(|(_, rows)| rows.iter())
            .collect();
        assert!(actuals.iter().any(|row| matches!(
            row.source,
            super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Integer(-7)
        )));
        assert!(actuals.iter().any(|row| matches!(
            row.source,
            super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Bool(true)
        )));
        assert!(actuals.iter().any(|row| matches!(
            row.source,
            super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::TypedHome { .. }
        )));
    })
    .unwrap();
}

#[test]
fn entry_rejects_failed_later_incoming_instead_of_using_first_caller() {
    let mut package = package("return 0", "local recv = new Transport() local a = recv.probe(0) local b = recv.probe(\"unsupported\") return 0");
    let (owner, parameters) = target(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    assert_eq!(
        ledger
            .borrowed_formal_actuals
            .values()
            .filter(|row| row.is_ok())
            .count(),
        1
    );
    let error = ledger
        .borrowed_entry_source_for_contract(owner, &parameters)
        .unwrap_err();
    assert!(
        error.contains("borrowed-actual/unsupported-or-unavailable"),
        "{error}"
    );
}

#[test]
fn entry_rejects_missing_actuals_and_truncated_rows() {
    for truncate in [false, true] {
        let mut package = package(
            "return 0",
            "local recv = new Transport() local a = recv.probe(0) return 0",
        );
        let (owner, parameters) = target(&package);
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        if truncate {
            ledger
                .borrowed_formal_actuals
                .values_mut()
                .next()
                .unwrap()
                .as_mut()
                .unwrap()
                .opaque_actuals = Box::new([]);
        } else {
            ledger.borrowed_formal_actuals.clear();
        }
        let error = ledger
            .borrowed_entry_source_for_contract(owner, &parameters)
            .unwrap_err();
        assert!(
            error.contains(if truncate {
                "borrowed-entry/actuals-cardinality"
            } else {
                "borrowed-entry/actuals-missing"
            }),
            "{error}"
        );
    }
}

#[test]
fn entry_rejects_actual_ordinal_site_and_formal_corruption() {
    for change in 0..3 {
        let mut package = package(
            "return 0",
            "local recv = new Transport() local a = recv.probe(0) local b = recv.probe(1) return 0",
        );
        let (owner, parameters) = target(&package);
        let foreign_formal = package
            .parameter_contracts
            .iter()
            .find(|row| row.owner != owner && !row.parameters.is_empty())
            .unwrap()
            .parameters[0]
            .binding;
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let foreign_site = ledger
            .borrowed_formal_actuals
            .keys()
            .last()
            .unwrap()
            .site()
            .clone();
        let caller_binding = ledger
            .borrowed_formal_actuals
            .keys()
            .next()
            .unwrap()
            .owner();
        let row = &mut ledger
            .borrowed_formal_actuals
            .values_mut()
            .next()
            .unwrap()
            .as_mut()
            .unwrap()
            .opaque_actuals[0];
        match change {
            0 => row.ordinal += 1,
            1 => row.site = foreign_site,
            _ => {
                assert_ne!(caller_binding, row.formal.owner());
                row.formal = foreign_formal;
            }
        }
        assert!(ledger
            .borrowed_entry_source_for_contract(owner, &parameters)
            .unwrap_err()
            .contains("borrowed-entry/actuals-identity"));
    }
}

#[test]
fn entry_rejects_formal_ordinal_and_duplicate_cardinality() {
    let package = package(
        "return 0",
        "local recv = new Transport() local a = recv.probe(0) return 0",
    );
    let (owner, parameters) = target(&package);
    let ledger = &package.ordinary_new_claim_ledger;
    let mut wrong = parameters.clone();
    wrong[0].0 += 1;
    assert!(ledger
        .borrowed_entry_source_for_contract(owner, &wrong)
        .unwrap_err()
        .contains("borrowed-entry/formal-identity"));
    let mut duplicate = parameters.clone();
    duplicate.push((1, parameters[0].1, parameters[0].2));
    assert!(ledger
        .borrowed_entry_source_for_contract(owner, &duplicate)
        .unwrap_err()
        .contains("borrowed-entry/incoming-formals"));
}

#[test]
fn nonopaque_entry_does_not_demand_unrelated_source_error() {
    let mut package = package(
        "return 0",
        "local recv = new Transport() local a = recv.probe(0) return 0",
    );
    let (owner, parameters) = target(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    ledger.borrowed_formal_source = Some(Err("selected-source-error".into()));
    assert!(ledger
        .borrowed_entry_source_for_contract(owner, &[])
        .unwrap()
        .is_none());
    assert_eq!(
        ledger
            .borrowed_entry_source_for_contract(owner, &parameters)
            .unwrap_err(),
        "selected-source-error"
    );
}

#[test]
fn outside_profile_remains_unselected_before_entry() {
    let package = package(
        "return p",
        "local recv = new Transport() local a = recv.probe(0) return 0",
    );
    let target_key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
            "Transport",
            "probe",
            1,
        ),
    );
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| package.selected.key_for_batch_slot(row.batch_slot) == Some(&target_key))
        .unwrap();
    let parameters: Vec<_> = contract
        .parameters
        .iter()
        .map(|row| (row.ordinal, row.binding, row.kind))
        .collect();
    assert!(package
        .ordinary_new_claim_ledger
        .borrowed_entry_source_for_contract(contract.owner, &parameters)
        .unwrap()
        .is_none());
}

#[test]
fn selected_entry_retains_copied_formal_forwarding_in_the_same_cohort() {
    let package = package(
        "local alias = p local recv = new Transport() local out = recv.sink(alias) return 0",
        "local recv = new Transport() local out = recv.probe(-1) return 0",
    );
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert_eq!(source.definitions.len(), 2);
    for owner in source.definitions.keys() {
        let contract = package
            .parameter_contracts
            .iter()
            .find(|row| row.owner == *owner)
            .unwrap();
        let parameters: Vec<_> = contract
            .parameters
            .iter()
            .map(|row| (row.ordinal, row.binding, row.kind))
            .collect();
        let projection = package
            .ordinary_new_claim_ledger
            .borrowed_entry_source_for_contract(*owner, &parameters)
            .unwrap()
            .unwrap();
        assert_eq!(projection.owner(), *owner);
        assert!(!projection.incoming().is_empty());
    }
    assert!(source.definitions.values().any(|row| row.origins.len() > 1));
}

#[test]
fn foreign_installed_instance_loan_cannot_be_classified_outside_profile() {
    let own = package(
        "return 0",
        "local recv = new Transport() local out = recv.probe(0) return 0",
    );
    let foreign = package(
        "return 0",
        "local recv = new Transport() local out = recv.probe(0) return 0",
    );
    let (owner, _) = target(&foreign);
    let declaration = foreign
        .parameter_contracts
        .iter()
        .find(|row| row.owner == owner)
        .unwrap();
    let key = foreign
        .selected
        .key_for_batch_slot(declaration.batch_slot)
        .unwrap()
        .clone();
    let mut context = crate::mir::builder::CompilationContext::new();
    let installed = foreign.prepare_install(&mut context).unwrap().commit();
    installed
        .begin_lowering(&context)
        .unwrap()
        .with_selected_lowering_input(&key, |input| {
            assert!(own
                .ordinary_new_claim_ledger
                .borrowed_ordinary_entry_source_v1(&input)
                .unwrap_err()
                .contains("borrowed-entry/foreign-source-loan"));
        })
        .unwrap();
}

#[test]
fn static_opaque_loan_does_not_demand_an_ordinary_instance_profile() {
    let source = "box Transport { birth() { } probe(p): i64 { return 0 } }
        static box Utility { probe(p): i64 { return 0 } }
        static box Main { main() { local recv = new Transport() local out = recv.probe(0) return 0 } }";
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
    Rc::get_mut(&mut package.ordinary_new_claim_ledger)
        .unwrap()
        .borrowed_formal_source = Some(Err("unrelated-source-error".into()));
    let ledger = Rc::clone(&package.ordinary_new_claim_ledger);
    let key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "Utility", "probe", 1,
        ),
    );
    let mut context = crate::mir::builder::CompilationContext::new();
    let installed = package.prepare_install(&mut context).unwrap().commit();
    installed
        .begin_lowering(&context)
        .unwrap()
        .with_selected_lowering_input(&key, |input| {
            assert!(ledger
                .borrowed_ordinary_entry_source_v1(&input)
                .unwrap()
                .is_none());
        })
        .unwrap();
}

#[test]
fn entry_value_recording_cannot_bypass_another_callee_actual_failure() {
    let mut package = package(
        "local alias = p local recv = new Transport() local out = recv.sink(alias) return 0",
        "local recv = new Transport() local out = recv.probe(-1) return 0",
    );
    let (owner, parameters) = target(&package);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let other = source
        .incoming
        .iter()
        .find(|row| row.callee != owner)
        .unwrap()
        .call
        .clone();
    *ledger.borrowed_formal_actuals.get_mut(&other).unwrap() =
        Err("other-callee-source-failure".into());
    assert_eq!(
        ledger
            .record_borrowed_ordinary_entry_values_v1(
                owner,
                Ok(vec![(0, parameters[0].1, crate::mir::ValueId::new(72))].into_boxed_slice())
            )
            .unwrap_err(),
        "other-callee-source-failure"
    );
    assert!(ledger
        .borrowed_ordinary_entry_values_v1(owner)
        .unwrap_err()
        .contains("entry-values-missing"));
}

#[test]
fn ordered_projection_corruption_is_terminal_for_entry_and_taken_call() {
    use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
    for change in 0..7 {
        let source = "box Transport { birth() { } probe(p, q: i64): i64 { return q } }
            static box Main { main() { local recv = new Transport()
                local a = recv.probe(true, 7) local b = recv.probe(false, 8) return 0 } }";
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
        let (owner, parameters) = target(&package);
        let foreign_binding = parameters[0].1;
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let call = ledger
            .borrowed_formal_actuals
            .keys()
            .next()
            .unwrap()
            .clone();
        let foreign_site = ledger
            .borrowed_formal_actuals
            .keys()
            .last()
            .unwrap()
            .site()
            .clone();
        let pending = ledger
            .borrowed_formal_actuals
            .get_mut(&call)
            .unwrap()
            .as_mut()
            .unwrap();
        let expected = match change {
            0 => {
                pending.ordered_arguments = Box::new([]);
                "ordered-arguments-cardinality"
            }
            1 => {
                let LocalCallArgumentV1::BorrowedActual { ordinal, .. } =
                    &mut pending.ordered_arguments[0]
                else {
                    panic!("opaque reference")
                };
                *ordinal += 1;
                "ordered-arguments-identity"
            }
            2 => {
                let LocalCallArgumentV1::BorrowedActual { site, .. } =
                    &mut pending.ordered_arguments[0]
                else {
                    panic!("opaque reference")
                };
                *site = foreign_site;
                "ordered-arguments-identity"
            }
            3 => {
                pending.ordered_arguments.swap(0, 1);
                "ordered-arguments-identity"
            }
            4 => {
                pending.ordered_arguments[1] = LocalCallArgumentV1::Bool(false);
                "ordered-arguments-identity"
            }
            5 => {
                pending.ordered_arguments[1] = LocalCallArgumentV1::Scalar(foreign_binding);
                "ordered-arguments-identity"
            }
            _ => {
                pending.ordered_arguments[0] = LocalCallArgumentV1::Integer(0);
                "ordered-arguments-identity"
            }
        };
        let error = ledger
            .borrowed_entry_source_for_contract(owner, &parameters)
            .unwrap_err();
        assert!(error.contains(expected), "{change}: {error}");
        let row = ledger
            .take_lexical_instance_call(call.owner(), call.site())
            .unwrap()
            .unwrap();
        let error = ledger.borrowed_call_actuals_v1(&row).unwrap_err();
        assert!(error.contains(expected), "{change}: {error}");
    }
}

fn mixed_projection_package(
    scalar_type: &str,
    main: &str,
) -> VerifiedNormalCallableSemanticPackageV1 {
    let source = format!("box Transport {{ birth() {{ }} probe(p, q: {scalar_type}): i64 {{ return q }} }} static box Main {{ main() {{ {main} }} }}");
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .unwrap()
}

#[test]
fn repeated_walk_drift_in_nonopaque_projection_poisons_entire_pending_call() {
    use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
    let package = mixed_projection_package(
        "i64",
        "local recv = new Transport() local out = recv.probe(true, 7) return 0",
    );
    let ledger = &package.ordinary_new_claim_ledger;
    let mut pending = ledger.borrowed_formal_actuals.clone();
    let (site, original) = pending.iter().next().unwrap();
    let site = site.clone();
    let mut changed = original.as_ref().unwrap().clone();
    changed.ordered_arguments[1] = LocalCallArgumentV1::Integer(8);
    super::super::borrowed_formal_actuals::stage_borrowed_call_actuals_v1(
        &mut pending,
        &site,
        Ok(Some(changed)),
    );
    assert!(pending[&site]
        .as_ref()
        .unwrap_err()
        .contains("repeated-walk-drift"));
    let original = ledger.borrowed_formal_actuals[&site]
        .as_ref()
        .unwrap()
        .clone();
    super::super::borrowed_formal_actuals::stage_borrowed_call_actuals_v1(
        &mut pending,
        &site,
        Ok(Some(original)),
    );
    assert!(pending[&site]
        .as_ref()
        .unwrap_err()
        .contains("repeated-walk-drift"));
}
