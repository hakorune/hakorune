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
fn current_owner_if_ge_zero_keeps_one_source_call_without_opening_transport() {
    use crate::mir::resolved_semantics::home_new_prefix::{
        LocalCallArgumentV1, LocalCallResultClassV1,
    };
    let source = |condition: &str| {
        format!("static box Layout {{ class_id(size) {{ return 0 }} accepts(size) {{ if {condition} {{ return true }} return false }} }} static box Main {{ main() {{ return 0 }} }}")
    };
    let accepted = package(&source("me.class_id(size) >= 0"));
    let original = ingress(&accepted)
        .source_incoming
        .static_observations()
        .values()
        .filter_map(|row| row.as_ref().ok())
        .find(|row| row.target().name() == "class_id")
        .expect("original CurrentOwner call");
    let observed = accepted
        .ordinary_new_claim_ledger
        .local_call_for_owner(original.call_site().owner(), original.call_site().site())
        .expect("selected Home source observation");
    assert_eq!(observed.site(), original.call_site());
    assert!(observed.is_expression_value());
    assert_eq!(observed.result(), LocalCallResultClassV1::I64);
    assert!(observed.prior_homes().is_empty());
    assert!(matches!(observed.arguments(),
        [LocalCallArgumentV1::BorrowedActual { ordinal: 0, site }]
            if Some(site) == original.argument_sites().first()));
    assert!(ingress(&accepted)
        .source_incoming
        .has_unsupported_static_context(original.callee_owner()));
    assert!(ingress(&accepted)
        .incoming
        .iter()
        .all(|row| row.callee != original.callee_owner()));

    for rejected in [
        "me.class_id(size) >= 1",
        "me.class_id(size) <= 0",
        "0 >= me.class_id(size)",
        "me.class_id(size) >= true",
    ] {
        let candidate = package(&source(rejected));
        let original = ingress(&candidate)
            .source_incoming
            .static_observations()
            .values()
            .filter_map(|row| row.as_ref().ok())
            .find(|row| row.target().name() == "class_id")
            .expect("rejected shape still has original call");
        assert!(
            candidate
                .ordinary_new_claim_ledger
                .local_call_for_owner(original.call_site().owner(), original.call_site().site())
                .is_none(),
            "{rejected}"
        );
    }

    // Opaque formal source spelling may carry Bool. This Home fact does not
    // assert an Integer payload or promote the retained SourceStatic phase.
    let dynamic_actual = package(&source("me.class_id(true) >= 0"));
    let original = ingress(&dynamic_actual)
        .source_incoming
        .static_observations()
        .values()
        .filter_map(|row| row.as_ref().ok())
        .find(|row| row.target().name() == "class_id")
        .unwrap();
    assert!(dynamic_actual
        .ordinary_new_claim_ledger
        .local_call_for_owner(original.call_site().owner(), original.call_site().site())
        .is_some_and(|call| call.is_expression_value()));
    assert!(ingress(&dynamic_actual)
        .incoming
        .iter()
        .all(|row| row.callee != original.callee_owner()));
}

#[test]
fn static_source_domain_keeps_full_original_incoming_and_same_rc_with_closed_transport() {
    let package = package(SOURCE);
    let source = ingress(&package);
    let p = formal(&package, "Layout", "pick");
    assert!(source.candidate_integer_agreement(p));
    assert!(!source.checked_static_input(p), "unused formal has no checked numeric use");
    assert!(source.formal_integer_agreement(p));
    assert!(source.contains_definition_for_test(p.owner()));
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
        let BorrowedIncomingSourceV1::Static(retained) = &row.source else {
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
    assert_eq!(source.incoming.len(), 3);
    assert_eq!(source.forwards.len(), 1);
    assert!(source.object_views.is_empty());

    let unchecked = self::package("static box Chain { pass(p) { return 0 } caller(q) { local n = me.pass(q) return 0 } } static box Main { main() { local n = Chain.caller(7) return 0 } }");
    let unchecked_source = ingress(&unchecked);
    assert!(!unchecked_source.checked_static_input(formal(&unchecked, "Chain", "pass")));
    assert!(!unchecked_source.checked_static_input(formal(&unchecked, "Chain", "caller")));
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
        assert!(source.contains_definition_for_test(p.owner()));
        assert_eq!(source.incoming.len(), 3);
    }
}

#[test]
fn static_source_domain_missing_claim_or_unsupported_body_never_seeds_a_callee() {
    for (body, passive_source) in [
        ("return true", true),
        ("return \"value\"", true),
        ("p = true return 0", false),
        ("local captured = fn() { return p } return 0", false),
    ] {
        let program = SOURCE.replace("pick(p) { return 0 }", &format!("pick(p) {{ {body} }}"));
        let package = package(&program);
        let source = ingress(&package);
        let p = formal(&package, "Layout", "pick");
        assert!(!source.candidate_integer_agreement(p), "{body}");
        assert!(!source.checked_static_input(p), "{body}");
        assert_eq!(
            source.candidate_input_inventory_for_test(p.owner()),
            (0, passive_source)
        );
        assert_eq!(source.source_definition_for(p.owner()).is_some(), passive_source);
        // A retained Bool/Text source keeps its missing-result veto; neither
        // that draft nor its veto grants executable entry or Integer agreement.
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
    assert!(source.contains_definition_for_test(p.owner()));
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

#[test]
fn static_source_domain_missing_static_authority_retains_instance_same_name_veto() {
    let package = package("box Transport { pick(p) { local k = Layout.other(p) return 0 } } static box Layout { other(q) { return 0 } pick(p) { return 0 } } static box Main { main() { local recv = new Transport() local a = recv.pick(true) local b = Layout.pick(7) return 0 } }");
    let source = ingress(&package);
    let instance = source
        .source_incoming
        .exact_rows()
        .find_map(|row| row.source.instance())
        .unwrap();
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == instance.callee_owner())
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
    let definitions = BTreeMap::from([(contract.owner, draft)]);
    let calls = BTreeMap::from([(instance.call_site().clone(), instance)]);
    let scope = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .collect();
    let inventory = inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &definitions,
        &package.parameter_contracts,
        &calls,
        &scope,
        None,
    )
    .unwrap();
    assert_eq!(inventory.exact_rows().count(), 1);
    assert!(inventory.vetoed_owners().contains(&contract.owner));
    assert!(matches!(
        inventory.project(&BTreeSet::from([contract.owner])),
        Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(_))
    ));
}

#[test]
fn current_owner_source_domain_retains_all_contexts_without_executable_entry() {
    use crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1;
    let package_fn = package;
    let package = package_fn("static box Layout {
        pick(p) { return 0 }
        from_init(p) { local k = me.pick(p) return 0 }
        tail(p) { return me.pick(p) }
        cond(p) { if me.pick(p) > 0 { return 0 } return 0 }
        looped(p) { loop(me.pick(p) > 0) { break } return 0 }
    } static box Main { main() { return 0 } }");
    let source = ingress(&package);
    let mut callers = BTreeSet::new();
    for (site, row) in source.source_incoming.static_observations() {
        let row = row.as_ref().expect("original current-owner observation");
        assert!(!row.is_qualified());
        assert!(row.require_qualified().is_err());
        assert!(row.current_owner_source().is_some());
        assert_eq!(row.call_site(), site);
        assert_eq!(row.caller().owner(), row.target().owner());
        assert_eq!(row.target().name(), "pick");
        assert_eq!(row.parameters().len(), 1);
        assert_eq!(row.argument_sites().len(), 1);
        callers.insert(row.caller().name().to_owned());
        assert!(source.incoming.iter().all(|incoming| &incoming.call != site));
        assert!(package.ordinary_new_claim_ledger.selected_static_local_source_v1(site).unwrap().is_none());
        if let Some(fact) = source.static_arguments.get(&(site.clone(), 0)) {
            assert!(Rc::ptr_eq(fact.retained_call_source(), row));
        }
    }
    assert_eq!(callers, ["from_init", "tail", "cond", "looped"].into_iter().map(str::to_owned).collect());
    assert!(source.incoming.is_empty());
    // A Qualified initializer cannot activate a callee with a source-only
    // CurrentOwner initializer, even without a noninitializer context veto.
    let mixed = package_fn("static box Layout {
        pick(p) { return 0 }
        relay(p) { local k = me.pick(p) return 0 }
    } static box Main { main() { local k = Layout.pick(7) return 0 } }");
    let mixed_source = ingress(&mixed);
    let pick = formal(&mixed, "Layout", "pick");
    let observations: Vec<_> = mixed_source.source_incoming.static_observations().values()
        .map(|row| row.as_ref().unwrap()).collect();
    assert_eq!(observations.len(), 2);
    assert_eq!(observations.iter().filter(|row| row.is_qualified()).count(), 1);
    assert!(!mixed_source.contains_definition_for_test(pick.owner()));
    assert!(mixed_source.incoming.iter().all(|row| row.callee != pick.owner()));
    let both = package_fn("box Node {
        pick(p) { return 0 }
        relay(p) { local k = me.pick(p) return 0 }
    } static box Layout {
        pick(p) { return 0 }
        relay(p) { local k = me.pick(p) return 0 }
    } static box Main { main() { return 0 } }");
    let arguments = BTreeMap::new();
    let context = StaticIncomingContextV1 {
        claims: &both.source_static_claims_for_test, arguments: &arguments, main: None,
    };
    let mut checked = 0;
    for declaration in both.batch().declarations() {
        both.batch().with_lowering_input(declaration.batch_slot(), |input| {
            for (site, call) in input.function().method_calls() {
                let site = OwnedExprSiteV1::new(input.owner(), site.clone());
                let observed = context.observe(declaration.batch_slot(), &site, call,
                    &both.selected, &both.parameter_contracts, None).unwrap();
                match call.receiver() {
                    ResolvedMethodCallReceiverSourceV1::Lexical(_) => assert!(observed.is_none()),
                    ResolvedMethodCallReceiverSourceV1::CurrentOwner => {
                        let row = observed.unwrap().unwrap();
                        assert_eq!(row.caller().owner(), "Layout");
                        assert!(!row.is_qualified());
                    }
                    other => panic!("unexpected original receiver {other:?}"),
                }
                checked += 1;
            }
        }).unwrap();
    }
    assert_eq!(checked, 2);
    // No retained argument Rc is available to discover a foreign index.
    // The same original catalog token must bind even a literal-only source.
    let literal = "static box Layout {
        pick(p) { return 0 }
        relay() { local k = me.pick(7) return 0 }
    } static box Main { main() { return 0 } }";
    let original = package_fn(literal);
    let foreign = package_fn(literal);
    assert!(ingress(&original).static_arguments.is_empty());
    assert!(!original.selected.catalog_brand().is_same(foreign.selected.catalog_brand()));
    let caller = CanonicalSameModuleCallableKeyV1::static_box_method("Layout", "relay", 0);
    let slot = original.selected.batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(caller.clone())).unwrap();
    original.batch().with_lowering_input(slot, |input| {
        let (site, call) = input.function().method_calls().next().unwrap();
        let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
        assert!(original.source_static_claims_for_test.incoming_source(&caller, &owned,
            call, &original.selected, &original.parameter_contracts, None).unwrap().is_some());
        assert!(matches!(foreign.source_static_claims_for_test.incoming_source(&caller, &owned,
            call, &original.selected, &original.parameter_contracts, None),
            Err(error) if error.contains("incoming-source-cohort")));
    }).unwrap();
}
