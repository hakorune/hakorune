//! Original Static incoming joins source agreement without transport activation.
use super::*;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::collections::BTreeSet;
use std::rc::Rc;
type Package =
    crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
fn package(source: &str) -> Package {
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        source,
    )
    .unwrap()
}
fn ingress(
    package: &Package,
) -> &super::super::super::borrowed_formal_source::PreparedBorrowedFormalIngressV1 {
    package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
}
fn formal(package: &Package, class: &str, name: &str) -> BindingRefV1 {
    let key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::static_box_method(class, name, 1),
    );
    let slot = package.selected.batch_slot(&key).unwrap();
    package
        .parameter_contracts
        .iter()
        .find(|row| row.batch_slot == slot)
        .unwrap()
        .parameters[0]
        .binding
}
const SOURCE: &str = "static box Layout { pick(p) { return 0 } } box Heap { lookup(size) { local k = Layout.pick(size) return 0 } } static box Main { main() { local heap = new Heap() local a = heap.lookup(7) local b = Layout.pick(8) return 0 } }";

#[test]
fn static_source_domain_keeps_full_original_incoming_and_same_rc_without_transport() {
    let package = package(SOURCE);
    let source = ingress(&package);
    let p = formal(&package, "Layout", "pick");
    assert!(source.candidate_integer_agreement(p));
    assert!(!source.formal_integer_agreement(p));
    assert!(!source.contains_definition_for_test(p.owner()));
    assert_eq!(
        source.candidate_input_inventory_for_test(p.owner()),
        (2, false)
    );
    let fact = source.static_arguments.values().next().unwrap();
    assert!(source.candidate_integer_agreement(fact.formal()));
    let rows: Vec<_> = source
        .source_incoming
        .exact_rows()
        .filter(|row| row.callee == p.owner())
        .collect();
    assert_eq!(rows.len(), 2);
    for row in rows {
        let BorrowedIncomingSourceV1::QualifiedStatic(retained) = &row.source else {
            panic!("original Static source kind")
        };
        assert!(Rc::ptr_eq(
            retained,
            source.source_incoming.static_observations()[&row.call]
                .as_ref()
                .unwrap()
        ));
        assert!(row.source.require_instance().is_err());
        assert_eq!(row.arguments[0].2, p);
        if row.call == *fact.call() {
            assert!(Rc::ptr_eq(retained, fact.retained_call_source()));
        }
    }
    assert!(source.incoming.is_empty());
    assert!(source.forwards.is_empty());
    assert!(source.object_views.is_empty());
}

#[test]
fn static_source_domain_never_omits_mixed_original_actuals() {
    for actual in ["true", "null", "local_value"] {
        let program = SOURCE.replace(
            "local b = Layout.pick(8)",
            &format!("local local_value = 8 local b = Layout.pick({actual})"),
        );
        let package = package(&program);
        let source = ingress(&package);
        let p = formal(&package, "Layout", "pick");
        assert!(!source.candidate_integer_agreement(p), "{actual}");
        assert_eq!(
            source.candidate_input_inventory_for_test(p.owner()),
            (2, false)
        );
        assert!(!source.contains_definition_for_test(p.owner()));
        assert!(source.incoming.is_empty());
    }
}

#[test]
fn static_source_domain_missing_claim_or_unsupported_body_never_seeds_a_callee() {
    for body in [
        "return true",
        "p = true return 0",
        "local captured = fn() { return p } return 0",
    ] {
        let program = SOURCE.replace("pick(p) { return 0 }", &format!("pick(p) {{ {body} }}"));
        let package = package(&program);
        let source = ingress(&package);
        let p = formal(&package, "Layout", "pick");
        assert!(!source.candidate_integer_agreement(p), "{body}");
        assert_eq!(
            source.candidate_input_inventory_for_test(p.owner()),
            (0, false)
        );
        assert!(!source.contains_definition_for_test(p.owner()));
    }
}

fn raw_target(package: &Package) -> BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1> {
    let p = formal(package, "Layout", "pick");
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == p.owner())
        .unwrap();
    let draft = package
        .batch()
        .with_lowering_input(contract.batch_slot, |input| {
            super::super::draft_borrowed_formal_uses_v1(
                input,
                contract,
                &package.instance_constructors,
                None,
            )
            .unwrap()
        })
        .unwrap();
    BTreeMap::from([(contract.owner, draft)])
}

#[test]
fn static_source_domain_missing_main_authority_keeps_good_row_and_callee_veto() {
    let package = package(SOURCE);
    let source = ingress(&package);
    let definitions = raw_target(&package);
    let calls: BTreeMap<_, _> = source
        .source_incoming
        .exact_rows()
        .filter_map(|row| {
            row.source
                .instance()
                .map(|original| (row.call.clone(), original))
        })
        .collect();
    let scope = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .collect();
    let context = StaticIncomingContextV1 {
        claims: &package.source_static_claims_for_test,
        arguments: &source.static_arguments,
        main: None,
    };
    let inventory = inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &definitions,
        &package.parameter_contracts,
        &calls,
        &scope,
        Some(&context),
    )
    .unwrap();
    assert_eq!(inventory.exact_rows().count(), 1);
    let p = formal(&package, "Layout", "pick");
    assert!(inventory.vetoed_owners().contains(&p.owner()));
    let Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(missing)) =
        inventory.project(&BTreeSet::from([p.owner()]))
    else {
        panic!("unavailable Main authority is not omitted")
    };
    assert_ne!(
        missing,
        *source.static_arguments.values().next().unwrap().call()
    );
}

#[test]
fn static_source_domain_rejects_foreign_owner_ordinal_and_mode_contracts() {
    let package = package(SOURCE);
    let source = ingress(&package);
    let p = formal(&package, "Layout", "pick");
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == p.owner())
        .unwrap();
    for (owner, ordinal, mode) in [
        (
            source
                .static_arguments
                .values()
                .next()
                .unwrap()
                .formal()
                .owner(),
            0,
            contract.mode,
        ),
        (p.owner(), 1, contract.mode),
        (
            p.owner(),
            0,
            CallableParameterDeclarationModeV1::InstanceBoxMethod,
        ),
    ] {
        let mut contracts: Vec<_> = package.parameter_contracts.iter().map(|row| {
            crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1 {
                owner: row.owner, batch_slot: row.batch_slot, mode: row.mode, parameters: row.parameters.iter().map(|formal| {
                    crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1 {
                        ordinal: formal.ordinal, binding: formal.binding, kind: formal.kind.clone(),
                    }
                }).collect(),
            }
        }).collect();
        let target = contracts
            .iter_mut()
            .find(|row| row.owner == p.owner())
            .unwrap();
        target.owner = owner;
        target.parameters[0].ordinal = ordinal;
        target.mode = mode;
        let scope = package
            .batch()
            .declarations()
            .map(|row| row.owner())
            .collect();
        assert!(matches!(
            inventory_borrowed_incoming_calls_v1(
                package.batch(),
                &package.selected,
                &raw_target(&package),
                &contracts,
                &BTreeMap::new(),
                &scope,
                None
            ),
            Err(BorrowedIncomingDraftErrorV1::SourceIdentity)
        ));
    }
}

#[test]
fn static_source_domain_proven_instance_namespace_does_not_poison_static_agreement() {
    let package = package("box Transport { pick(p) { local k = Layout.other(p) return 0 } } static box Layout { other(q) { return 0 } pick(p) { return 0 } } static box Main { main() { local recv = new Transport() local a = recv.pick(true) local b = Layout.pick(7) return 0 } }");
    let source = ingress(&package);
    let p = formal(&package, "Layout", "pick");
    assert!(source.candidate_integer_agreement(p));
    assert_eq!(
        source.candidate_input_inventory_for_test(p.owner()),
        (1, false)
    );
    assert!(!source.contains_definition_for_test(p.owner()));
}

#[test]
fn static_source_domain_miskeyed_instance_loan_cannot_suppress_static_incoming() {
    let package = package("box Transport { pick(p) { local k = Layout.other(p) return 0 } } static box Layout { other(q) { return 0 } pick(p) { return 0 } } static box Main { main() { local recv = new Transport() local a = recv.pick(true) local b = Layout.pick(7) return 0 } }");
    let source = ingress(&package);
    let instance = source
        .source_incoming
        .exact_rows()
        .find_map(|row| row.source.instance())
        .unwrap();
    let static_site = source
        .source_incoming
        .static_observations()
        .iter()
        .find(|(_, row)| {
            row.as_ref()
                .is_ok_and(|source| source.target().name() == "pick")
        })
        .unwrap()
        .0;
    assert_ne!(instance.call_site(), static_site);
    let calls = BTreeMap::from([(static_site.clone(), instance)]);
    let scope = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .collect();
    assert!(
        matches!(inventory_borrowed_incoming_calls_v1(package.batch(), &package.selected,
        &raw_target(&package), &package.parameter_contracts, &calls, &scope, None),
        Err(BorrowedIncomingDraftErrorV1::CallIdentity(ref site)) if site == static_site)
    );
}

#[test]
fn static_source_domain_declared_object_does_not_gain_pending_result_or_entry() {
    let package = package("box Token { value: i64 birth() { me.value = 7 } } box Reader { read(p) { return p.value } } static box Layout { pick(p: Token) { local recv = new Reader() local ignored = recv.read(p) return 0 } } static box Main { main() { local token = new Token() local out = Layout.pick(token) return 0 } }");
    let source = ingress(&package);
    let p = formal(&package, "Layout", "pick");
    assert_eq!(
        source.candidate_input_inventory_for_test(p.owner()),
        (1, false)
    );
    assert!(!source.contains_definition_for_test(p.owner()));
    assert!(source.formal_object_view(p).is_none());
    assert!(!package
        .ordinary_new_claim_ledger
        .borrowed_i64_results_for_test()
        .iter()
        .any(|(owner, _)| *owner == p.owner()));
}
