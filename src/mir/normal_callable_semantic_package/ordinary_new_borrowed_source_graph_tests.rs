//! Original rows and callee vetoes share one immutable inventory.
use super::super::super::borrowed_formal_uses::{borrow_call_sources_v1, BorrowedCallSourceLoanV1};
use super::*;
use std::rc::Rc;
type Package =
    crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
fn package(source: &str) -> Package {
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        source,
    )
    .unwrap()
}
const INSTANCE: &str = "box Heap { lookup(p) { return 0 } unused(q) { return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) return 0 } }";
const STATIC: &str = "static box Layout { class_id(p) { local alias: i64 = p return 0 } } static box Main { main() { local k = Layout.class_id(7) return 0 } }";

#[test]
fn object_source_inventory_retains_unqualified_sibling_input_without_return_authority() {
    let package = package("box Token {} box Maker { make(size: i64) { return new Token() } relay() { return me.make(7) } ignored() { local item = me.make(8) return 0 } } static box Main { main() { return 0 } }");
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap();
    let ingress = source.as_ref().unwrap();
    let rows: Vec<_> = ingress.source_incoming.exact_rows().collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows.iter()
            .filter(|row| row.source.object_return_sources().is_some())
            .count(),
        1
    );
    let sibling = rows
        .iter()
        .find(|row| row.source.object_return_sources().is_none())
        .unwrap();
    let target = sibling.source.require_instance().unwrap();
    assert!(ingress.source_incoming.has_object_input_callee_v1(target));
    let actuals = package
        .ordinary_new_claim_ledger
        .borrowed_formal_actuals
        .get(&sibling.call)
        .unwrap()
        .as_ref()
        .unwrap();
    assert_eq!(
        actuals.ordered_arguments.as_ref(),
        &[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(8)]
    );
    assert!(actuals.require_executable_v1().is_ok());
    assert!(target.object_return_sources().is_none());
    let mut foreign = target.clone();
    foreign.target_batch_slot += 1;
    assert!(!ingress.source_incoming.has_object_input_callee_v1(&foreign));
    let mut pending = BTreeMap::new();
    super::super::super::reject_borrowed_actuals_for_owner_v1(
        source,
        sibling.call.owner(),
        &mut pending,
        "failed-sibling-walk".into(),
    );
    assert_eq!(
        pending.get(&sibling.call).unwrap().as_ref().unwrap_err(),
        "failed-sibling-walk"
    );
}

#[test]
fn object_source_inventory_keeps_zero_and_i64_inputs_for_direct_and_received() {
    for (parameters, arguments) in [("", ""), ("size: i64", "7")] {
        for result in [
            format!("return me.make({arguments})"),
            format!("local out = me.make({arguments}) return out"),
        ] {
            let source = format!("box Token {{}} box Maker {{ make({parameters}) {{ return new Token() }} relay() {{ {result} }} }} static box Main {{ main() {{ return 0 }} }}");
            let package = package(&source);
            let ingress = package
                .ordinary_new_claim_ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            let rows: Vec<_> = ingress
                .source_incoming
                .exact_rows()
                .filter(|row| row.source.object_return_sources().is_some())
                .collect();
            assert_eq!(rows.len(), 1, "{source}");
            let row = rows[0];
            assert!(row.arguments.is_empty(), "opaque subset only");
            assert_eq!(
                row.source.argument_sites().len(),
                usize::from(!parameters.is_empty())
            );
            assert!(ingress.source_incoming.owners.contains(&row.callee));
            assert!(!ingress.definitions.contains_key(&row.callee));
            assert!(ingress
                .incoming
                .iter()
                .all(|final_row| final_row.callee != row.callee));
            let owners = [row.callee].into_iter().collect();
            assert_eq!(ingress.source_incoming.project(&owners).unwrap().len(), 1);
            let actuals = package
                .ordinary_new_claim_ledger
                .borrowed_formal_actuals
                .get(&row.call)
                .expect("original source probe observes typed input")
                .as_ref()
                .expect("original source actuals remain retained");
            assert_eq!(
                actuals.ordered_arguments.len(),
                row.source.argument_sites().len()
            );
            assert!(actuals.require_executable_v1().is_ok());
        }
    }
}

#[test]
fn object_source_inventory_excludes_non_i64_typed_contract() {
    let package = package("box Token {} box Maker { make(size: usize) { return new Token() } relay() { return me.make(7) } } static box Main { main() { return 0 } }");
    let ingress = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert!(ingress
        .source_incoming
        .exact_rows()
        .all(|row| row.source.object_return_sources().is_none()));
}

#[test]
fn object_source_inventory_preserves_unresolved_outside_and_global_vetoes() {
    let package = package("box Token {} box Maker { make(size: i64) { return new Token() } relay() { return me.make(7) } again() { local out = me.make(8) return out } } static box Main { main() { return 0 } }");
    let ingress = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let rows: Vec<_> = ingress
        .source_incoming
        .exact_rows()
        .filter(|row| row.source.object_return_sources().is_some())
        .collect();
    assert_eq!(rows.len(), 2);
    let owners = [rows[0].callee].into_iter().collect();
    let scopes = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .collect();
    let calls = BTreeMap::from([(
        rows[0].call.clone(),
        rows[0].source.require_instance().unwrap(),
    )]);
    let omitted = inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &ingress.definitions,
        &package.parameter_contracts,
        &calls,
        &scopes,
        None,
    )
    .unwrap();
    assert!(omitted.owners.contains(&rows[0].callee));
    assert_eq!(
        omitted.project(&owners).unwrap_err(),
        BorrowedIncomingDraftErrorV1::UnresolvedCaller(rows[1].call.clone())
    );
    assert!(omitted.vetoed_owners().contains(&rows[0].callee));
    let calls = rows
        .iter()
        .map(|row| (row.call.clone(), row.source.require_instance().unwrap()))
        .collect();
    let mut scopes = scopes;
    scopes.remove(&rows[1].call.owner());
    let mut outside = inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &ingress.definitions,
        &package.parameter_contracts,
        &calls,
        &scopes,
        None,
    )
    .unwrap();
    assert_eq!(
        outside.project(&owners).unwrap_err(),
        BorrowedIncomingDraftErrorV1::OutsideOrdinaryScope(rows[1].call.clone())
    );
    outside.observations.clear();
    outside
        .observations
        .push_legacy((None, Err(BorrowedIncomingDraftErrorV1::BatchLoan)));
    assert!(outside.vetoed_owners().contains(&rows[0].callee));
    assert_eq!(
        outside.project(&owners).unwrap_err(),
        BorrowedIncomingDraftErrorV1::BatchLoan
    );
}

#[test]
fn object_source_inventory_separates_same_selector_on_distinct_receivers() {
    let package = package("box Token {} box Maker { make() { return new Token() } relay() { return me.make() } } box Other { make() { return new Token() } relay() { local out = me.make() return out } } static box Main { main() { return 0 } }");
    let ingress = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let rows: Vec<_> = ingress
        .source_incoming
        .exact_rows()
        .filter(|row| row.source.object_return_sources().is_some())
        .collect();
    assert_eq!(rows.len(), 2);
    assert_ne!(rows[0].callee, rows[1].callee);
    for row in rows {
        let projected = ingress
            .source_incoming
            .project(&[row.callee].into_iter().collect())
            .unwrap();
        assert_eq!(projected.len(), 1);
        assert_eq!(projected[0].call, row.call);
    }
}

#[test]
fn source_graph_retains_raw_no_incoming_and_projects_final_callee_only() {
    let package = package(INSTANCE);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let inventory = &source.source_incoming;
    assert_eq!(inventory.owners.len(), 2);
    assert_eq!(inventory.exact_rows().count(), 1);
    assert!(inventory
        .legacy_observations()
        .any(|(_, row)| matches!(row, Err(BorrowedIncomingDraftErrorV1::NoIncoming(_)))));
    assert!(inventory.project(&inventory.owners).is_err());
    let final_owners = source.definitions.keys().copied().collect();
    let projected = inventory.project(&final_owners).unwrap();
    assert_eq!(projected.len(), 1);
    assert_eq!(projected[0].call, source.incoming[0].call);
    assert!(matches!(
        projected[0].source,
        BorrowedIncomingSourceV1::Instance(_)
    ));
    assert_eq!(
        projected[0].source.require_instance().unwrap(),
        source.incoming[0].source.require_instance().unwrap()
    );
    // A genuine Birth is outside ordinary declarations but still contributes
    // its original call to THIS retained census, without execution permission.
    for body in ["return 0", "return null", "", "local scratch = 0", "return", "if true { return } local scratch = 0", "local a = new Page() local b = new Page()", "print(7)", "print(7) return"] {
        let program = format!("box Page {{ id: i64 = 0 birth() {{}} seed() {{ {body} }} }} box Pool {{ page: Page birth() {{ me.page = new Page() me.page.seed() }} }} static box Main {{ main() {{ local pool = new Pool() return 0 }} }}");
        let package = self::package(&program);
        let constructor = package.instance_constructors().rows().iter().find(|row| row.box_name() == "Pool").unwrap();
        let source = constructor.construction_source().as_ref().unwrap();
        let crate::mir::normal_callable_semantic_package::instance_construction::ConstructionSourceStepV1::StoredChildCall(request) = &source.steps()[1] else { panic!("original child call") };
        let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let inventory = &ingress.source_incoming;
        let mut matching = inventory.exact_rows().filter(|row| &row.call == request.site());
        let row = matching.next().expect("Birth call in the sole incoming census");
        assert!(matching.next().is_none());
        let unit_candidate = body != "return 0" && body != "return null";
        assert_eq!(row.source.is_pending(), !unit_candidate);
        assert_eq!(row.source.require_instance().is_ok(), unit_candidate);
        assert_eq!(row.source.target().namespace(), hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod);
        assert_eq!(row.source.target().owner(), "Page");
        assert_eq!(row.source.target().name(), "seed");
        assert_eq!(row.source.argument_sites(), request.arguments());
        assert!(row.arguments.is_empty());
        assert!(inventory.owners.contains(&row.callee));
        assert!(!inventory.source_vetoed_owners_v1().contains(&row.callee));
        assert_eq!(inventory.vetoed_owners().contains(&row.callee), !unit_candidate);
        assert!(inventory.legacy_observations().all(|(_, observed)| !matches!(observed,
            Err(BorrowedIncomingDraftErrorV1::NoIncoming(owner)) if owner == &row.callee)));
        assert!(!package.batch().declarations().any(|declaration| declaration.owner() == row.call.owner()));
        assert_eq!(constructor.construction().is_ok(), unit_candidate);
        let unit = row.source.unit_discard_source_v1();
        if body == "return 0" || body == "return null" {
            assert!(unit.is_none(), "value Return never becomes a Unit request");
        } else {
            let unit = unit.expect("original Unit discard request retained");
            assert!(unit.is_unit_candidate_v1());
            let completion = package.result_contracts.row(row.source.target_batch_slot()).unwrap().borrow();
            assert!(unit.matches_unit_completion_v1(completion.completion()), "{body}");
            if body == "local a = new Page() local b = new Page()" {
                let (original_body, original_end) = completion.completion().implicit_body_end().unwrap();
                let flow = completion.completion().cleanup().root_flow().expect("same original Home walk");
                let implicit = flow.implicit_exit().expect("same implicit key").expect("complete original Home obligations");
                assert_eq!(implicit.implicit_origin(), Some((original_body, original_end)));
                let mut locals = package.batch().with_lowering_input(row.source.target_batch_slot(), |input| {
                    input.function().bindings().filter_map(|(binding, record)| match record.kind() {
                        crate::mir::resolved_semantics::BindingKindV1::Local { ordinal } => Some((ordinal, binding)),
                        _ => None,
                    }).collect::<Vec<_>>()
                }).unwrap();
                locals.sort_by_key(|(ordinal, _)| *ordinal);
                let expected: Vec<_> = locals.into_iter().rev().map(|(_, binding)| binding).collect();
                assert_eq!(implicit.homes(), expected.as_slice(), "same LIFO as explicit exits");
                assert_eq!(expected.len(), 2);
                assert!(!flow.all_exits_ready(), "physical implicit consumer remains closed");
            }

            assert!(!unit.matches_unit_completion_v1(package.ordinary_new_claim_ledger.root_completion_for_test()));
            let remade = super::super::super::borrowed_formal_result::original::prepare_original_returns_v1(
                package.batch(), &package.selected, &package.parameter_contracts,
            );
            let remade = remade[&row.callee].as_ref().unwrap().as_ref().unwrap();
            assert_ne!(unit, remade, "equal syntax does not replace the retained source Rc");
            let target = row.source.require_instance().unwrap();
            let loan = row.source.constructor_unit_source_v1(target).unwrap().expect("same original constructor step");
            assert!(std::rc::Rc::ptr_eq(loan.source(), source));
            for change in 0..3 {
                let mut drift = target.clone();
                match change {
                    0 => {
                        let super::super::super::LexicalInstanceCallReceiverV1::StoredOwnedChild { child, field, .. } = &mut drift.receiver
                            else { panic!("original stored child"); };
                        *child = field.object();
                    }
                    1 => drift.receiver_site = drift.call_site.site().clone(),
                    _ => drift.argument_sites = vec![drift.receiver_site.clone()].into_boxed_slice(),
                }
                assert!(row.source.constructor_unit_source_v1(&drift).is_err(), "constructor Unit drift {change}");
            }
            let ledger = &package.ordinary_new_claim_ledger;
            let completion = ledger.unit_completion_source_v1(row.callee).unwrap().unwrap();
            assert!(std::ptr::eq(completion.as_ref(), package.result_contracts.row(target.target_batch_slot()).unwrap().borrow().completion()));
            assert!(ledger.unit_discard_source_v1(&row.call).is_none(), "no invented caller Home observation");
            assert!(ledger.lexical_instance_call_covered(row.call.owner(), row.call.site()));
            assert!(constructor.construction().is_ok(), "same late source-qualified ordered plan");
        }
    }
    // Equal Unit syntax outside Birth is not this construction-source capability.
    let ordinary = self::package("box Page { id: i64 = 0 birth() {} seed() {} } box Pool { page: Page birth() { me.page = new Page() } untouched() { me.page.seed() } } static box Main { main() { local pool = new Pool() return 0 } }");
    let ingress = ordinary.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    let row = ingress.source_incoming.exact_rows().find(|row| row.source.unit_discard_source_v1().is_some()).unwrap();
    assert!(row.source.is_pending(), "ordinary stored Unit stays outside constructor selection");
    assert!(row.source.require_instance().is_err());
    assert!(ingress.source_incoming.vetoed_owners().contains(&row.callee));
    // The same selector/arity in a provider argument is an original Static
    // edge, independently of the constructor's pending stored Instance edge.
    let package = self::package("static box Layout { seed() { return 8 } } box Page { id: i64 birth(n) { me.id = n } seed(): i64 { return 0 } } box Pool { page: Page birth() { me.page = new Page(Layout.seed()) me.page.seed() } } static box Main { main() { local pool = new Pool() return 0 } }");
    let constructor = package.instance_constructors().rows().iter().find(|row| row.box_name() == "Pool").unwrap();
    let source = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    let inventory = &source.source_incoming;
    let mut static_calls = inventory.exact_rows().filter(|row| row.source.target().owner() == "Layout");
    let static_call = static_calls.next().expect("original constructor provider Static call");
    assert!(static_calls.next().is_none());
    let BorrowedIncomingSourceV1::Static(original) = &static_call.source else { panic!("Static namespace") };
    assert!(original.is_qualified_i64_source_v1());
    assert_eq!(original.caller(), constructor.published_birth_key().unwrap());
    assert!(original.argument_sites().is_empty());
    assert!(Rc::ptr_eq(original, inventory.static_observations()[&static_call.call].as_ref().unwrap()));
    assert_eq!(inventory.project(&[static_call.callee].into_iter().collect()).unwrap().len(), 1);
    assert!(inventory.has_unsupported_static_context(static_call.callee));
    assert!(!source.definitions.contains_key(&static_call.callee));
    let stored = inventory.exact_rows().find(|row| row.source.target().owner() == "Page").unwrap();
    assert!(stored.source.is_pending());
    assert!(inventory.project(&[stored.callee].into_iter().collect()).is_err());
    let package = self::package("static box Layout { seed(unused) { return 8 } } box Page { id: i64 birth(a, b) { me.id = a } } box Pool { page: Page = new Page(Layout.seed(7), Layout.seed(7)) birth() {} } static box Main { main() { return 0 } }");
    let ingress = package.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    let originals: Vec<_> = ingress.source_incoming.exact_rows().filter_map(|row| match &row.source {
        BorrowedIncomingSourceV1::Static(source) if source.target().owner() == "Layout" => Some(source),
        _ => None,
    }).collect();
    assert_eq!(originals.len(), 2);
    let callee = originals[0].callee_owner();
    assert!(ingress.source_incoming.has_unsupported_static_context(callee));
    assert!(!ingress.definitions.contains_key(&callee));
    assert!(ingress.source_only_definitions.contains_key(&callee));
    assert!(ingress.incoming.iter().all(|row| row.callee != callee));
    assert_eq!(ingress.source_incoming.project(&[callee].into_iter().collect()).unwrap().len(), 2);
    assert!(ingress.candidate_integer_agreement(originals[0].parameters()[0].binding),
        "provider source inputs still contribute to the same integer domain");
    // Adding an ordinary caller never makes a partial borrowed cohort that
    // omits the two provider calls. All three remain original source inputs.
    let mixed = self::package("static box Layout { seed(unused) { return 8 } } box Page { id: i64 birth(a, b) { me.id = a } } box Pool { page: Page = new Page(Layout.seed(7), Layout.seed(7)) birth() {} } static box Main { main() { local value = Layout.seed(7) return 0 } }");
    let source = mixed.ordinary_new_claim_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    let mut matching = source.source_incoming.exact_rows().filter(|row| row.source.target().owner() == "Layout");
    let first = matching.next().unwrap();
    assert_eq!(matching.count(), 2);
    assert!(source.source_incoming.has_unsupported_static_context(first.callee));
    assert!(!source.definitions.contains_key(&first.callee));
    assert!(source.incoming.iter().all(|row| row.callee != first.callee));

}

#[test]
fn source_graph_empty_declaration_requires_clean_registered_scan_sentinel() {
    let package = package(INSTANCE);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let original = &source.source_incoming;
    let owner = original
        .legacy_observations()
        .find_map(|(owner, row)| {
            matches!(row, Err(BorrowedIncomingDraftErrorV1::NoIncoming(_)))
                .then_some(*owner)
                .flatten()
        })
        .unwrap();
    let call = source.incoming[0].clone();
    for mutation in 0..9 {
        let mut inventory = BorrowedIncomingInventoryV1 {
            source_owners: original.source_owners.clone(),
            owners: original.owners.clone(),
            unsupported_static_spelling: Default::default(),
            unsupported_static_context: Default::default(),
            static_observations: BTreeMap::new(),
            observations: original.observations.clone(),
        };
        match mutation {
            0 => {}
            1 => inventory
                .observations
                .retain(|row| row.original.0 != Some(owner)),
            2 => inventory.observations.push_legacy((
                Some(owner),
                Err(BorrowedIncomingDraftErrorV1::NoIncoming(owner)),
            )),
            3 => {
                let row = inventory
                    .observations
                    .iter_mut().map(|row| &mut row.original)
                    .find(|(label, _)| *label == Some(owner))
                    .unwrap();
                row.1 = Err(BorrowedIncomingDraftErrorV1::NoIncoming(call.callee));
            }
            4 => inventory.observations.push_legacy((
                Some(owner),
                Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(
                    call.call.clone(),
                )),
            )),
            5 => inventory
                .observations
                .push_legacy((None, Err(BorrowedIncomingDraftErrorV1::BatchLoan))),
            6 => {
                inventory.owners.remove(&owner);
            }
            7 => {
                let mut real = call.clone();
                real.callee = owner;
                inventory.observations.push_legacy((Some(owner), Ok(real)));
            }
            8 => inventory.observations.push_legacy((
                Some(owner),
                Err(BorrowedIncomingDraftErrorV1::OutsideOrdinaryScope(
                    call.call.clone(),
                )),
            )),
            _ => unreachable!(),
        }
        let declaration = inventory.project_declaration_v1(owner);
        if mutation == 0 {
            assert!(declaration.unwrap().is_empty());
            assert!(
                matches!(inventory.project(&std::collections::BTreeSet::from([owner])),
                Err(BorrowedIncomingDraftErrorV1::NoIncoming(observed)) if observed == owner)
            );
            assert_eq!(
                inventory.project_declaration_v1(call.callee).unwrap().len(),
                1
            );
        } else {
            assert!(
                declaration.is_err(),
                "mutation {mutation} forged an empty declaration"
            );
        }
    }
}

#[test]
fn source_graph_selected_callee_keeps_every_caller_veto_and_global_batch_fault() {
    let package = package(INSTANCE);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let row = source.incoming[0].clone();
    let owners = std::collections::BTreeSet::from([row.callee]);
    for error in [
        BorrowedIncomingDraftErrorV1::UnresolvedCaller(row.call.clone()),
        BorrowedIncomingDraftErrorV1::OutsideOrdinaryScope(row.call.clone()),
        BorrowedIncomingDraftErrorV1::CallIdentity(row.call.clone()),
        BorrowedIncomingDraftErrorV1::NoIncoming(row.callee),
    ] {
        let inventory = BorrowedIncomingInventoryV1 {
            unsupported_static_spelling: Default::default(),
            unsupported_static_context: Default::default(),
            source_owners: owners.clone(),
            owners: owners.clone(),
            static_observations: BTreeMap::new(),
            observations: test_observations(vec![
                (Some(row.callee), Ok(row.clone())),
                (Some(row.callee), Err(error.clone())),
            ]),
        };
        assert_eq!(inventory.project(&owners).unwrap_err(), error);
        assert!(inventory.project(&Default::default()).unwrap().is_empty());
    }
    let inventory = BorrowedIncomingInventoryV1 {
        unsupported_static_spelling: Default::default(),
        unsupported_static_context: Default::default(),
        source_owners: owners.clone(),
        owners: owners.clone(),
        static_observations: BTreeMap::new(),
        observations: test_observations(vec![(None, Err(BorrowedIncomingDraftErrorV1::BatchLoan))]),
    };
    assert_eq!(inventory.vetoed_owners(), inventory.owners);
    for selected in [owners, Default::default()] {
        assert_eq!(
            inventory.project(&selected).unwrap_err(),
            BorrowedIncomingDraftErrorV1::BatchLoan
        );
    }
}

#[test]
fn source_graph_static_owner_preserves_rc_and_refuses_instance_authority() {
    let package = package(STATIC);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let original = source
        .source_incoming
        .static_observations()
        .values()
        .next()
        .unwrap()
        .as_ref()
        .unwrap();
    let owned = BorrowedIncomingSourceV1::Static(Rc::clone(original));
    assert_eq!(owned.call_site(), original.call_site());
    assert_eq!(owned.target(), original.target());
    assert_eq!(owned.argument_sites(), original.argument_sites());
    assert!(
        matches!(owned.clone(), BorrowedIncomingSourceV1::Static(row) if Rc::ptr_eq(&row, original))
    );
    assert!(
        matches!(owned.as_loan().unwrap(), BorrowedCallSourceLoanV1::Static(row) if std::ptr::eq(row, original.as_ref()))
    );
    assert!(owned
        .require_instance()
        .unwrap_err()
        .contains("instance-source-required"));
    assert!(owned.object_return_sources().is_none());
    assert!(owned.object_source_forwards().is_none());
    assert!(owned.object_producer_dependencies().is_none());
    assert!(owned.stored_receiver().is_none());
    assert!(source.definitions.is_empty());
    assert!(source.incoming.is_empty());
}

#[test]
fn source_graph_common_loan_refuses_instance_collision_and_equal_reissued_static() {
    let static_package = package(STATIC);
    let source = static_package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let original = source
        .source_incoming
        .static_observations()
        .values()
        .next()
        .unwrap()
        .as_ref()
        .unwrap();
    let instance_package = package(INSTANCE);
    let instance = instance_package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .incoming[0]
        .source
        .require_instance()
        .unwrap();
    let collision = BTreeMap::from([(original.call_site().clone(), instance)]);
    assert!(borrow_call_sources_v1(&collision, [original.as_ref()])
        .unwrap_err()
        .contains("conflicting-source-call"));
    let main = super::super::super::borrow_app_main_source_v1(
        static_package.batch(),
        static_package.catalog.catalog().source_backed_app_main(),
    )
    .unwrap()
    .unwrap();
    let reissued = static_package
        .batch()
        .with_lowering_input(main.batch_slot(), |input| {
            let (_, call) = input.function().method_calls().next().unwrap();
            static_package
                .source_static_claims_for_test
                .incoming_source(
                    main.catalog_key(),
                    original.call_site(),
                    call,
                    &static_package.selected,
                    &static_package.parameter_contracts,
                    Some(&main),
                )
                .unwrap()
                .unwrap()
                .retain(input).unwrap()
        })
        .unwrap();
    assert!(!Rc::ptr_eq(original, &reissued));
    assert!(
        borrow_call_sources_v1(&BTreeMap::new(), [original.as_ref(), reissued.as_ref()]).is_err()
    );
    assert_eq!(
        borrow_call_sources_v1(&BTreeMap::new(), [original.as_ref(), original.as_ref()])
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn source_graph_original_scan_retains_one_exact_caller_and_missing_sibling_veto() {
    let package = package("box Heap { lookup(p) { return 0 } } static box Main { main() { local heap = new Heap() local a = heap.lookup(7) local b = heap.lookup(8) return 0 } }");
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert_eq!(source.incoming.len(), 2);
    let row = &source.incoming[0];
    let omitted = &source.incoming[1];
    let calls = BTreeMap::from([(row.call.clone(), row.source.require_instance().unwrap())]);
    let scopes = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .collect();
    let inventory = inventory_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &source.definitions,
        &package.parameter_contracts,
        &calls,
        &scopes,
        None,
    )
    .unwrap();
    assert_eq!(inventory.exact_rows().count(), 1);
    assert!(inventory.legacy_observations().any(|(owner, observation)| *owner == Some(omitted.callee) && matches!(observation, Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(site)) if site == &omitted.call)));
    assert_eq!(
        inventory
            .project(&source.definitions.keys().copied().collect())
            .unwrap_err(),
        BorrowedIncomingDraftErrorV1::UnresolvedCaller(omitted.call.clone())
    );
}

#[test]
fn source_graph_partial_scan_preserves_global_loan_fault_before_orphan_check() {
    let package = package("static box Layout { class_id(p) { return 0 } } box Heap { lookup(p) { local k = Layout.class_id(p) return 0 } } static box Main { main() { local heap = new Heap() local k = heap.lookup(7) return 0 } }");
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    assert_eq!(source.static_arguments.len(), 1);
    let context = StaticIncomingContextV1 {
        claims: &package.source_static_claims_for_test,
        arguments: &source.static_arguments,
        main: None,
    };
    let mut inventory = BorrowedIncomingInventoryV1 {
        unsupported_static_spelling: Default::default(),
        unsupported_static_context: Default::default(),
        source_owners: Default::default(),
        owners: Default::default(),
        static_observations: BTreeMap::new(),
        observations: test_observations(vec![(None, Err(BorrowedIncomingDraftErrorV1::BatchLoan))]),
    };
    inventory
        .corroborate_scan_completeness(&Default::default(), Some(&context), true)
        .unwrap();
    assert_eq!(
        inventory.project(&Default::default()).unwrap_err(),
        BorrowedIncomingDraftErrorV1::BatchLoan
    );
    inventory
        .corroborate_scan_completeness(&Default::default(), None, false)
        .unwrap();
    inventory.observations.clear();
    assert_eq!(
        inventory
            .corroborate_scan_completeness(&Default::default(), None, false)
            .unwrap_err(),
        BorrowedIncomingDraftErrorV1::SourceIdentity
    );
    assert_eq!(
        inventory
            .corroborate_scan_completeness(&Default::default(), Some(&context), true)
            .unwrap_err(),
        BorrowedIncomingDraftErrorV1::SourceIdentity
    );
}

#[test]
fn source_graph_final_instance_correspondence_rejects_owned_source_mutation() {
    let mut package = package(INSTANCE);
    let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let prepared = ledger.lexical_instance_calls.borrow().values().filter_map(|slot| {
        match slot {
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::LexicalInstanceCallDispositionSlotV1::Ready(row) => Some(Ok(Some(row.source_target().clone()))),
            _ => None,
        }
    }).collect::<Vec<_>>();
    assert!(!prepared.is_empty(), "original issued affine targets");
    let source = ledger
        .borrowed_formal_source
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    source.corroborate_source_targets(&prepared).unwrap();
    source.incoming[0].source.instance_mut().target_batch_slot += 1;
    assert!(source
        .corroborate_source_targets(&prepared)
        .unwrap_err()
        .contains("final-incoming-drift"));
}

fn test_observations(rows: Vec<(Option<FunctionOwnerIdV1>, Result<BorrowedIncomingCallDraftV1, BorrowedIncomingDraftErrorV1>)>) -> Vec<inventory::IncomingObservationV1> {
    rows.into_iter().map(Into::into).collect()
}
trait LegacyObservationTestPushV1 {
    fn push_legacy(&mut self, row: (Option<FunctionOwnerIdV1>, Result<BorrowedIncomingCallDraftV1, BorrowedIncomingDraftErrorV1>));
}
impl LegacyObservationTestPushV1 for Vec<inventory::IncomingObservationV1> {
    fn push_legacy(&mut self, row: (Option<FunctionOwnerIdV1>, Result<BorrowedIncomingCallDraftV1, BorrowedIncomingDraftErrorV1>)) { self.push(row.into()); }
}
