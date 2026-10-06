use super::*;

fn package(
    body: &str,
    sink: &str,
    main: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let source = format!(
        "box Transport {{ birth() {{ }} probe(p): i64 {{ {body} }} \
         sink(q): i64 {{ {sink} }} }} static box Main {{ main() {{ {main} }} }}"
    );
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .expect("source-backed ordinary package")
}

#[test]
fn package_issuer_prepares_ignored_formal_without_changing_source_kind() {
    let package = package(
        "return 0",
        "return 0",
        "local recv = new Transport() local out = recv.probe(0) return 0",
    );
    let prepared = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .expect("production source preparation")
        .as_ref()
        .expect("complete source cohort");
    assert_eq!(prepared.definitions.len(), 1);
    assert_eq!(prepared.incoming.len(), 1);
    assert!(prepared.forwards.is_empty());
    let callee = prepared.incoming[0].callee;
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.owner == callee)
        .unwrap();
    assert_eq!(
        contract.parameters[0].kind,
        CallableParameterContractKindV1::OpaqueHandle
    );
    assert!(prepared.definitions[&callee].uses.is_empty());
}

#[test]
fn finite_mutual_forwarding_is_prepared_by_the_real_package_issuer() {
    let package = package(
        "local alias = p local recv = new Transport() local out = recv.sink(alias) return 0",
        "local recv = new Transport() local out = recv.probe(q) return 0",
        "local recv = new Transport() local out = recv.probe(-1) return 0",
    );
    let prepared = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .expect("complete finite graph");
    assert_eq!(prepared.definitions.len(), 2);
    assert_eq!(prepared.forwards.len(), 2);
    assert_eq!(prepared.incoming.len(), 3);
}

#[test]
fn legitimate_nontransport_use_is_outside_profile_before_selection() {
    for body in ["return p", "p = 1 return 0", "local a: i64 = p return 0"] {
        let package = package(
            body,
            "return 0",
            "local recv = new Transport() local out = recv.probe(0) return 0",
        );
        let prepared = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .expect("profile-outside is not source corruption");
        assert!(prepared.definitions.is_empty(), "body: {body}");
        assert!(prepared.incoming.is_empty());
    }
}

#[test]
fn forwarding_to_outside_profile_removes_dependent_cohort_before_selection() {
    let package = package(
        "local recv = new Transport() local out = recv.sink(p) return 0",
        "return q",
        "local recv = new Transport() local out = recv.probe(0) return 0",
    );
    let prepared = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .expect("finite profile pruning");
    assert!(prepared.definitions.is_empty());
    assert!(prepared.forwards.is_empty());
}

#[test]
fn source_contract_identity_corruption_is_not_profile_outside() {
    let package = package(
        "return 0",
        "return 0",
        "local recv = new Transport() local out = recv.probe(0) return 0",
    );
    let slots = package
        .ordinary_new_claim_ledger
        .lexical_instance_calls
        .borrow();
    let prepared = Ok(slots
        .values()
        .filter_map(|slot| match slot {
            crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1::Ready(row) => {
                Some(Ok(Some(row.source_target().clone())))
            }
            _ => None,
        })
        .collect());
    let contract = package
        .parameter_contracts
        .iter()
        .find(|row| row.parameters.len() == 1)
        .unwrap();
    let mut contracts = package
        .parameter_contracts
        .iter()
        .map(|row| OwnedCallableParameterContractDeclarationV1 {
            owner: row.owner,
            batch_slot: row.batch_slot,
            mode: row.mode,
            parameters: row
                .parameters
                .iter()
                .map(
                    |formal| crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1 {
                        ordinal: formal.ordinal,
                        binding: formal.binding,
                        kind: formal.kind.clone(),
                    },
                )
                .collect(),
        })
        .collect::<Vec<_>>();
    contracts
        .iter_mut()
        .find(|row| row.owner == contract.owner)
        .unwrap()
        .parameters[0]
        .ordinal = 1;
    let empty_loans =
        crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1::issue(Box::new([]));
    let empty_candidates = BTreeMap::new();
    let empty_classes = BTreeMap::new();
    let error = prepare_borrowed_formal_ingress_v1(
        package.batch(),
        &package.selected,
        &contracts,
        &prepared,
        None,
        None,
        &empty_loans,
        package.instance_constructors(),
        &empty_candidates,
        &empty_classes,
    )
    .expect_err("sealed identity mismatch");
    assert!(error.contains("borrowed-formal/source-identity"), "{error}");
}

#[test]
fn selected_cohort_unresolved_incoming_is_retained_as_named_error() {
    let source = "box Transport { birth() { } probe(p): i64 { local other = null local bad = other.probe(1) return 0 } sink(q): i64 { return 0 } } static box Main { main() { local recv = new Transport() local good = recv.probe(0) return 0 } }";
    let error = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).err().expect("unresolved selected incoming is terminal before continuation");
    let error = format!("{error:?}");
    assert!(
        error.contains("borrowed-formal/incoming-coverage"),
        "{error}"
    );
    assert!(error.contains("UnresolvedCaller"), "{error}");
}

#[test]
fn final_corroboration_rejects_changed_incoming_argument_ordinal() {
    let package = package(
        "return 0",
        "return 0",
        "local recv = new Transport() local out = recv.probe(0) return 0",
    );
    let original = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let caller = original.incoming[0].call.owner();
    let app_main_slot = package
        .batch()
        .declarations()
        .find(|row| row.owner() == caller)
        .unwrap()
        .batch_slot();
    let slots = package
        .ordinary_new_claim_ledger
        .lexical_instance_calls
        .borrow();
    let source_rows = slots.values().filter_map(|slot| match slot {
        crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1::Ready(row) => Some(Ok(Some(row.source_target().clone()))),
        _ => None,
    }).collect::<Vec<_>>();
    let prepared = Ok(source_rows);
    let empty_loans =
        crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1::issue(Box::new([]));
    let empty_candidates = BTreeMap::new();
    let empty_classes = BTreeMap::new();
    let mut borrowed = prepare_borrowed_formal_ingress_v1(
        package.batch(),
        &package.selected,
        &package.parameter_contracts,
        &prepared,
        Some(app_main_slot),
        None,
        &empty_loans,
        package.instance_constructors(),
        &empty_candidates,
        &empty_classes,
    )
    .unwrap();
    borrowed
        .corroborate_source_targets(prepared.as_ref().unwrap())
        .unwrap();
    borrowed.incoming[0].arguments[0].0 = 1;
    let error = borrowed
        .corroborate_source_targets(prepared.as_ref().unwrap())
        .unwrap_err();
    assert!(
        error.contains("borrowed-formal/final-incoming-drift"),
        "{error}"
    );
}

fn field_receiver_package(
    check_body: &str,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let source = format!(
        "box Probe {{ sizes: ArrayBox = new ArrayBox() birth() {{ }} \
         check(p) {{ {check_body} }} }} \
         box Outer {{ probe: Probe birth() {{ me.probe = new Probe() }} \
         run() {{ return me.probe.check(9) }} }} \
         static box Main {{ main() {{ local outer = new Outer() return outer.run() }} }}"
    );
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::
        issue_with_brand_catalog(&source)
    .expect("field-receiver package")
}

fn formal_owner_named(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
    name: &str,
) -> super::FunctionOwnerIdV1 {
    package
        .parameter_contracts
        .iter()
        .find(|row| {
            package
                .batch()
                .with_lowering_input(row.batch_slot, |input| {
                    row.parameters.iter().any(|formal| {
                        input
                            .function()
                            .binding(formal.binding)
                            .is_some_and(|binding| binding.diagnostic_name() == name)
                    })
                })
                .unwrap_or(false)
        })
        .unwrap_or_else(|| panic!("{name} formal contract"))
        .owner
}

fn view_sites_of(
    prepared: &PreparedBorrowedFormalIngressV1,
    owner: super::FunctionOwnerIdV1,
) -> Vec<&crate::mir::resolved_semantics::SourceNodeSiteV1> {
    prepared
        .dominated_view_sites
        .iter()
        .filter(|site| site.owner() == owner)
        .map(|site| site.site().node())
        .collect()
}

/// `me.<field>` receiver calls never enter the lexical incoming map, so a
/// field-receiver callee stays outside the borrowed transport profile —
/// yet its sealed draft still admits dominated-view value uses. The
/// prefix scanner's consult reads classification output, not transport
/// membership: the `.set` element value and the `+` operand both carry
/// dominated-view rows while `definitions` holds no row for the owner.
#[test]
fn field_receiver_callee_keeps_dominated_view_sites_outside_transport() {
    let package = field_receiver_package(
        "if p > 5 { return 1 } local i = me.sizes.get(0) \
         me.sizes.set(i, p) return p + 1",
    );
    let prepared = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .expect("production source preparation")
        .as_ref()
        .expect("complete source cohort");
    let check_owner = formal_owner_named(&package, "p");
    assert!(
        !prepared.definitions.contains_key(&check_owner),
        "`me.probe.check` is a field-receiver edge — no lexical incoming call"
    );
    let sites = view_sites_of(prepared, check_owner);
    assert_eq!(sites.len(), 2, "`.set` value and `+` operand rows: {sites:?}");
    assert!(
        sites.iter().any(|site| site.segments().last()
            == Some(&crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(1))),
        "`.set` element-value leaf admitted: {sites:?}"
    );
    assert!(
        sites.iter().any(|site| site.segments().ends_with(&[
            crate::mir::resolved_semantics::SourcePathSegmentV1::Value,
            crate::mir::resolved_semantics::SourcePathSegmentV1::Lhs,
        ])),
        "dominated `+` operand leaf admitted: {sites:?}"
    );
}

/// The `.get`-result index arm names only the exact sole initializer —
/// it never propagates through a plain local copy. `j = i` is not a
/// `.get` initializer, so the `.set` element value stays an unresolved
/// argument and carries no dominated-view row; the `+` operand is still
/// admitted under the same guard.
#[test]
fn get_result_index_rejects_plain_copy_of_get_result() {
    let package = field_receiver_package(
        "if p > 5 { return 1 } local i = me.sizes.get(0) local j = i \
         me.sizes.set(j, p) return p + 1",
    );
    let prepared = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .expect("production source preparation")
        .as_ref()
        .expect("complete source cohort");
    let check_owner = formal_owner_named(&package, "p");
    assert!(!prepared.definitions.contains_key(&check_owner));
    let sites = view_sites_of(prepared, check_owner);
    assert_eq!(
        sites.len(),
        1,
        "only the `+` operand keeps a dominated-view row: {sites:?}"
    );
    assert!(
        sites.iter().any(|site| site.segments().ends_with(&[
            crate::mir::resolved_semantics::SourcePathSegmentV1::Value,
            crate::mir::resolved_semantics::SourcePathSegmentV1::Lhs,
        ])),
        "dominated `+` operand leaf admitted: {sites:?}"
    );
    assert!(
        !sites.iter().any(|site| site.segments().last()
            == Some(&crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(1))),
        "no `.set` element-value row through a plain copy: {sites:?}"
    );
}
