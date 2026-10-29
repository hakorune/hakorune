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
    let slot = package.selected.batch_slot(&key).or_else(|| {
        let crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key) = &key else { return None; };
        package.batch().declarations().find(|row|
            package.source_backed_callable_catalog().omitted_closed_app_static_for_identity(row.identity()) == Some(key))
            .map(|row| row.batch_slot())
    }).unwrap();
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
        use super::super::super::borrowed_formal_source::SourceValueAlternativeV1 as Value;
        let expected = match actual { "true" => BTreeSet::from([Value::SignedI64, Value::Other]),
            "null" => BTreeSet::from([Value::SignedI64, Value::Null]), _ => BTreeSet::from([Value::SignedI64]) };
        assert_eq!(source.source_value_sum_v1(p).unwrap().alternatives(), &expected);
    }
    // Omitted wrappers retain original Text/scalar/null/unknown and result
    // dependencies without asking the old executable ABI to accept Text.
    use super::super::super::borrowed_formal_source::SourceValueAlternativeV1 as Value;
    for (setup, actual, expected) in [
        ("", "\"text\"", Some(BTreeSet::from([Value::Text]))),
        ("local value = \"text\"", "value", Some(BTreeSet::from([Value::Text]))),
        ("local value = \"text\" value = 7", "value", Some(BTreeSet::from([Value::SignedI64]))),
        ("local value", "value", None),
        ("local value = \"text\" local capture = fn() { return value }", "value", None),
        ("", "Values.text()", Some(BTreeSet::from([Value::Text]))),
        ("", "Values.identity(7)", Some(BTreeSet::from([Value::SignedI64]))),
        ("", "Values.identity(true)", None),
        ("", "Values.unknown()", None),
    ] {
        let text = format!("static box Values {{ sink(p) {{ return 0 }} text() {{ return \"result\" }} identity(x) {{ return x }} unknown() {{ return null }} unused() {{ {setup} local n = Values.sink({actual}) return 0 }} }} static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let source = ingress(&package);
        let formal = formal(&package, "Values", "sink");
        assert_eq!(source.source_value_sum_v1(formal).map(|value| value.alternatives().clone()), expected, "{setup}; {actual}");
        assert!(!source.contains_definition_for_test(formal.owner()), "source domain cannot select an entry");
    }

    for (body, incoming, expected) in [
        ("local n = Values.sink(q) return 0", "local n = Values.chain(\"text\")", Some(Value::Text)),
        ("local alias = q local n = Values.sink(alias) return 0", "local n = Values.chain(\"text\")", Some(Value::Text)),
        ("q = 7 local n = Values.sink(q) return 0", "local n = Values.chain(\"text\")", Some(Value::SignedI64)),
        ("local capture = fn() { return q } local n = Values.sink(q) return 0", "local n = Values.chain(\"text\")", None),
        ("local n = Values.sink(q) return 0", "", None),
    ] {
        let text = format!("static box Values {{ sink(p) {{ return 0 }} chain(q) {{ {body} }} unused() {{ {incoming} return 0 }} }} static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let source = ingress(&package);
        let p = formal(&package, "Values", "sink");
        assert_eq!(source.source_value_sum_v1(p).map(|sum| sum.alternatives().clone()),
            expected.map(|value| BTreeSet::from([value])), "all original source-only Forward obligations: {body}/{incoming}");
        assert!(!source.checked_static_input(p));
    }

    for (actual, calls, expected) in [
        ("q.substring(0,1)", "Values.chain(\"text\")", Some(Value::Text)),
        ("q.substring(0,1)", "Values.chain(7)", None),
        ("q.substring(0,1)", "Values.chain(null)", None),
        ("q.substring(0,1)", "Values.chain(\"text\") local b = Values.chain(7)", None),
        ("q.substring(true,1)", "Values.chain(\"text\")", None),
        ("q.substring(0,null)", "Values.chain(\"text\")", None),
        ("q.substring(0,\"end\")", "Values.chain(\"text\")", None),
        ("q.substring(0,1)", "", None),
        ("q.length()", "Values.chain(\"text\")", Some(Value::SignedI64)),
        ("q.indexOf(\"t\")", "Values.chain(\"text\")", Some(Value::SignedI64)),
        ("q.indexOf(7)", "Values.chain(\"text\")", None),
        ("Values.substring(0,1)", "Values.chain(\"text\")", None),
    ] {
        let caller = if calls.is_empty() { "return 0".to_owned() } else { format!("local a = {calls} return 0") };
        let text = format!("static box Values {{ sink(p) {{ return 0 }} substring(a,b) {{ return null }} chain(q) {{ local n = Values.sink({actual}) return 0 }} unused() {{ {caller} }} }} static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let source = ingress(&package);
        let p = formal(&package, "Values", "sink");
        assert_eq!(source.source_value_sum_v1(p).map(|sum| sum.alternatives().clone()),
            expected.clone().map(|value| BTreeSet::from([value])), "{actual}; {calls}");
        let mut checked = 0;
        for declaration in package.batch().declarations() {
            package.batch().with_lowering_input(declaration.batch_slot(), |input| {
                for (site, method) in input.function().method_calls() {
                    if method.selector() != "substring" && method.selector() != "length" && method.selector() != "indexOf" { continue; }
                    let closed = source.closed_core_source_v1(&OwnedExprSiteV1::new(input.owner(), site.clone()));
                    assert_eq!(closed.is_some(), expected.is_some(), "closed original request: {actual}; {calls}");
                    if let Some(closed) = closed {
                        assert!(std::sync::Arc::ptr_eq(closed.method(), input.function().method_call_source_v1(site).unwrap()));
                        assert_eq!(closed.manifest().arity(), method.arity());
                    }
                    checked += 1;
                }
            }).unwrap();
        }
        assert_eq!(checked, 1);
        assert!(!source.checked_static_input(p), "closed Core source cannot grant an executable entry");
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
        assert_eq!(source.candidate_integer_agreement(p), passive_source, "{body}");
        assert!(!source.checked_static_input(p), "{body}");
        assert_eq!(
            source.candidate_input_inventory_for_test(p.owner()),
            (if passive_source { 2 } else { 0 }, false)
        );
        assert_eq!(source.source_definition_for(p.owner()).is_some(), passive_source);
        assert!(source.source_incoming.source_registered_owners_v1().contains(&p.owner()));
        assert_eq!(source.source_incoming.source_rows_v1().filter(|row| row.callee == p.owner()).count(), 2,
            "unsupported use/transport cannot erase original source incoming");
        // Original 7/8 inputs retain their source agreement independently of
        // Bool/Text result class. That class still forbids the I64 executable
        // cohort; source input agreement cannot fabricate an entry or result.
        assert!(!source.contains_definition_for_test(p.owner()));
    }
    // Nested declaration/Birth callers remain explicit unknowns, with the
    // SAME resolver row and ordered actuals; they cannot enter legacy ABI rows.
    for program in [
        SOURCE.replace("pick(p) { return 0 }", "pick(p) { return 0 } unused() { local nested = fn() { local n = Layout.pick(99) return 0 } return 0 }"),
        format!("box Outer {{ birth() {{ local nested = fn() {{ local n = Layout.pick(99) return 0 }} }} }} {}",
            SOURCE),
    ] {
        let package = package(&program);
        let source = ingress(&package);
        let owner = formal(&package, "Layout", "pick").owner();
        let census = &source.source_incoming;
        let unknown = census.observations.iter().find(|row|
            row.scope == inventory::IncomingObservationScopeV1::SourceOnly
                && row.original.0 == Some(owner)
                && matches!(row.original.1, Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(_))))
            .expect("nested caller must not vanish");
        let method = unknown.method_source.as_ref().expect("unknown retains actuals");
        assert_eq!(method.selector(), "pick");
        assert_eq!(method.arguments().len(), 1);
        assert_eq!(method.arguments()[0].ordinal(), 0);
        let mut matched = false;
        for declaration in package.batch().declarations() {
            package.batch().with_lowering_input(declaration.batch_slot(), |root| {
                if root.forest().owner(method.owner()).is_some() {
                    package.batch().with_owner_lowering_input(declaration.batch_slot(), method.owner(), |nested| {
                        assert!(std::sync::Arc::ptr_eq(method, nested.function().method_call_source_v1(method.site()).unwrap()));
                        matched = true;
                    }).unwrap();
                }
            }).unwrap();
        }
        for birth in package.instance_constructors.rows() {
            if birth.forest().owner(method.owner()).is_some() {
                let nested = birth.lowering_input_for_owner(package.batch().source_ast(), method.owner()).unwrap();
                assert!(std::sync::Arc::ptr_eq(method, nested.function().method_call_source_v1(method.site()).unwrap()));
                matched = true;
            }
        }
        assert!(matched, "method Arc belongs to original nested forest");
        assert_eq!(census.exact_rows().filter(|row| row.callee == owner).count(), 2);
        assert!(census.sum_vetoed_owners_v1().contains(&owner));
        assert!(!census.vetoed_owners().contains(&owner));
        let receipt = census.source_refusals_v1();
        assert!(census.corroborates_refusals_v1(&receipt));
        let mut mutated = BorrowedIncomingInventoryV1 {
            revision: Default::default(),
            owners: census.owners.clone(), source_owners: census.source_owners.clone(),
            unsupported_static_spelling: census.unsupported_static_spelling.clone(),
            unsupported_static_context: census.unsupported_static_context.clone(),
            static_observations: census.static_observations.clone(), observations: census.observations.clone(),
        };
        let position = mutated.observations.iter().position(|row|
            row.method_source.as_ref().is_some_and(|row| std::sync::Arc::ptr_eq(row, method))).unwrap();
        mutated.observations[position].method_source = census.observations.iter()
            .filter_map(|row| row.method_source.as_ref()).find(|row| !std::sync::Arc::ptr_eq(row, method)).cloned();
        assert!(!mutated.corroborates_refusals_v1(&receipt), "same error cannot hide foreign source Arc");
        mutated.observations[position] = unknown.clone();
        mutated.observations.push(unknown.clone());
        let duplicate_receipt = mutated.source_refusals_v1();
        mutated.observations.pop();
        assert_eq!(mutated.sum_vetoed_owners_v1(), census.sum_vetoed_owners_v1());
        assert!(!mutated.corroborates_refusals_v1(&duplicate_receipt), "unchanged veto cannot hide a lost unknown");
    }
    // A source-only caller and legacy empty declaration are independent views
    // of the SAME observation store. Global errors preserve their scope too.
    let package = package(SOURCE);
    let row = ingress(&package).source_incoming.exact_rows().next().unwrap().clone();
    let owners = BTreeSet::from([row.callee]);
    use inventory::{IncomingObservationScopeV1 as Scope, IncomingObservationV1 as Observation};
    let mut census = BorrowedIncomingInventoryV1 {
        revision: Default::default(),
        owners: owners.clone(), source_owners: owners.clone(),
        unsupported_static_spelling: Default::default(), unsupported_static_context: Default::default(),
        static_observations: BTreeMap::new(),
        observations: vec![
            Observation::new(Scope::LegacyOnly, Some(row.callee), Err(BorrowedIncomingDraftErrorV1::NoIncoming(row.callee))),
            Observation::new(Scope::SourceOnly, Some(row.callee), Ok(row)),
        ],
    };
    let owner = *owners.first().unwrap();
    assert!(census.project_declaration_v1(owner).unwrap().is_empty());
    assert_eq!(census.exact_rows().count(), 0);
    assert_eq!(census.source_rows_v1().count(), 1);
    assert!(census.sum_vetoed_owners_v1().is_empty());
    census.observations.push(Observation::new(Scope::SourceOnly, None, Err(BorrowedIncomingDraftErrorV1::BatchLoan)));
    assert!(census.project(&Default::default()).unwrap().is_empty());
    assert_eq!(census.sum_vetoed_owners_v1(), owners);
    census.observations.pop();
    census.observations.push(Observation::new(Scope::LegacyOnly, None, Err(BorrowedIncomingDraftErrorV1::BatchLoan)));
    assert_eq!(census.project(&Default::default()).unwrap_err(), BorrowedIncomingDraftErrorV1::BatchLoan);
    assert!(census.sum_vetoed_owners_v1().is_empty());

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
    // An omitted caller still contributes its original Static source edge;
    // the same selector/arity on an Instance owner is another namespace.
    for (instance_call, target_selected) in [("", false), ("local n = recv.pick(7)", true)] {
    let text = "box Transport { pick(p) { return 0 } } static box Other { pick(p) { return 0 } } static box Layout { unused() { local n = Other.pick(99) return 0 } } static box Main { main() { local recv = new Transport() INSTANCE_CALL return 0 } }".replace("INSTANCE_CALL", instance_call);
    let omitted = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
        &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
    ).unwrap();
    let incoming = &ingress(&omitted).source_incoming;
    let row = incoming.source_rows_v1().find(|row| row.source.target().owner() == "Other")
        .expect("retain omitted caller's target despite Instance homonym");
    let unused = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::static_box_method("Layout", "unused", 0));
    assert!(omitted.selected.batch_slot(&unused).is_none());
    assert_eq!(omitted.selected.batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(row.source.target().clone())).is_some(), target_selected);
    assert!(incoming.exact_rows().all(|legacy| legacy.call != row.call));
    let instance_owner = omitted.parameter_contracts.iter().find(|row|
        row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod && row.parameters.len() == 1).unwrap().owner;
    assert!(!incoming.sum_vetoed_owners_v1().contains(&instance_owner));
    }
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
        assert!(!row.is_qualified_i64_source_v1());
        assert!(row.require_qualified_i64_source_v1().is_err());
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
    for condition in ["me.pick(p) > 0", "me.pick(p) > 0 && true"] {
        let wrong_pair = package_fn(&format!("static box Layout {{
            pick(p) {{ return 0 }}
            from_init(p) {{ local k = me.pick(p) return 0 }}
            cond(p) {{ if {condition} {{ return 0 }} return 0 }}
        }} static box Main {{ main() {{ return 0 }} }}"));
        let wrong_source = ingress(&wrong_pair);
        assert!(wrong_source.target_static.is_empty(),
            "initializer plus {condition} is not the exact Eq source cohort");
    }
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
    assert_eq!(observations.iter().filter(|row| row.is_qualified_i64_source_v1()).count(), 1);
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
                let observed = context.observe(declaration.batch_slot(), &site, call, input, &BTreeMap::new(),
                    &both.selected, &both.parameter_contracts, None).unwrap();
                match call.receiver() {
                    ResolvedMethodCallReceiverSourceV1::Lexical(_) => assert!(observed.is_none()),
                    ResolvedMethodCallReceiverSourceV1::CurrentOwner => {
                        let row = observed.unwrap().unwrap();
                        assert_eq!(row.caller().owner(), "Layout");
                        assert!(!row.is_qualified_i64_source_v1());
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

#[path = "ordinary_new_borrowed_static_point_premise_tests.rs"]
mod point_premise_tests;
