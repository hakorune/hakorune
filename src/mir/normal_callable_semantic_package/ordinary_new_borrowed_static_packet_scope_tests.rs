use super::*;

#[test]
fn static_execution_scope_preserves_original_forward_dependencies() {
    for mutation in 0..10 {
        let mut package = package();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let forward = source.forwards.first().expect("original forwarding source");
        let owner = forward.callee_formal.owner();
        let dependency = forward.source_formal.owner();
        let call = forward.call.clone();
        let ordinal = forward.ordinal;
        let callee_formal = forward.callee_formal;
        let source_formal = forward.source_formal;
        let calls: Vec<_> = source.incoming.iter().collect();
        let owners = ledger
            .checked_static_execution_owners_v1(source, owner, &calls)
            .unwrap();
        assert!(owners.contains(&owner));
        assert!(owners.contains(&dependency));
        match mutation {
            0..=3 => {
                let rows = ledger
                    .borrowed_formal_actuals
                    .get_mut(&call)
                    .unwrap()
                    .as_mut()
                    .unwrap();
                let actual = rows
                    .opaque_actuals
                    .iter_mut()
                    .find(|row| row.ordinal == ordinal)
                    .unwrap();
                match mutation {
                    0 => actual.source = BorrowedFormalActualSourceV1::Integer(7),
                    1 => actual.site = call.site().clone(),
                    2 => actual.ordinal += 1,
                    3 => actual.formal = source_formal,
                    _ => unreachable!(),
                }
            }
            4..=9 => {
                let source = ledger
                    .borrowed_formal_source
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap();
                match mutation {
                    4 => source.forwards[0].binding = callee_formal,
                    5 => {
                        source.forwards[0].target =
                            crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
                                "Foreign", "pick", 1,
                            )
                    }
                    6 => {
                        source.definitions.remove(&dependency);
                    }
                    7 => source.forwards = Box::new([]),
                    8 => {
                        let f = &source.forwards[0];
                        let duplicate = super::super::super::super::borrowed_formal_uses::BorrowedForwardUseDraftRowV1 {
                            site: f.site.clone(), binding: f.binding, source_formal: f.source_formal,
                            call: f.call.clone(), target: f.target.clone(), ordinal: f.ordinal,
                            callee_formal: f.callee_formal,
                        };
                        let mut forwards = std::mem::take(&mut source.forwards).into_vec();
                        forwards.push(duplicate);
                        source.forwards = forwards.into_boxed_slice();
                    }
                    9 => source.forwards[0].site = call.clone(),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let calls: Vec<_> = source.incoming.iter().collect();
        let error = ledger
            .checked_static_execution_owners_v1(source, owner, &calls)
            .unwrap_err();
        let expected = if mutation == 2 {
            "forward-actual-identity"
        } else {
            "forward-source-drift"
        };
        assert!(error.contains(expected), "mutation {mutation}: {error}");
    }
}

#[test]
fn static_execution_scope_refuses_removed_original_incoming() {
    let package = package();
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let forward = source.forwards.first().expect("original forwarding source");
    let owner = forward.callee_formal.owner();
    let calls: Vec<_> = source
        .incoming
        .iter()
        .filter(|call| call.call != forward.call)
        .collect();
    assert!(ledger
        .checked_static_execution_owners_v1(source, owner, &calls)
        .unwrap_err()
        .contains("forward-incoming-identity"));
    let dependency = forward.source_formal.owner();
    let calls: Vec<_> = source
        .incoming
        .iter()
        .filter(|call| call.callee != dependency)
        .collect();
    assert!(ledger
        .checked_static_execution_owners_v1(source, owner, &calls)
        .unwrap_err()
        .contains("forward-entry-missing"));
    let literal = source
        .incoming
        .iter()
        .find(|call| call.callee == owner && call.call != forward.call)
        .unwrap();
    let calls: Vec<_> = source
        .incoming
        .iter()
        .filter(|call| call.call != literal.call)
        .collect();
    assert!(ledger
        .checked_static_execution_owners_v1(source, owner, &calls)
        .unwrap_err()
        .contains("incoming-coverage"));
}

fn scoped_object_package(
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let text = format!(
        "{TEXT} box Token {{}} box Maker {{
        make(p) {{ if p <= 0 {{ return null }} return new Token() }}
        relay(p) {{ if p <= 0 {{ return new Token() }} local item = me.make(p) return new Token() }}
        qualified() {{ return me.make(7) }}
    }}"
    );
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &text,
    )
    .expect("original guarded Object sibling and independent Static input")
}

#[test]
fn static_execution_scope_does_not_promote_unrelated_object_input() {
    for mutation in 0..7 {
        let mut package = scoped_object_package();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let passive = source
            .incoming
            .iter()
            .find(|call| {
                ledger.borrowed_formal_actuals[&call.call]
                    .as_ref()
                    .unwrap()
                    .require_executable_v1()
                    .is_err_and(|issue| issue.contains("source-only-object-actuals"))
            })
            .expect("unrelated Object input remains source-only");
        let selected = source
            .incoming
            .iter()
            .find(|call| {
                matches!(
                    &call.source,
                    super::super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(_)
                )
            })
            .expect("selected Static original");
        assert_ne!(selected.callee, passive.callee);
        let owner = selected.callee;
        let passive_site = passive.call.clone();
        let passive_argument = passive.arguments[0].clone();
        let passive_root = *source.source_only_definitions[&passive.call.owner()]
            .origins
            .values()
            .next()
            .expect("original caller formal");
        let executable_site = selected.call.clone();
        if mutation == 0 {
            ledger
                .checked_borrowed_entry_incoming_core_v1(source, owner, true)
                .unwrap();
            let super::super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(
                original,
            ) = &selected.source
            else {
                unreachable!()
            };
            assert!(ledger
                .borrowed_static_packet_actuals_v1(original)
                .unwrap()
                .is_some());
            assert!(ledger
                .checked_borrowed_entry_incoming_core_v1(source, passive.callee, true)
                .unwrap_err()
                .contains("source-only-object-actuals"));
            assert!(ledger
                .checked_borrowed_entry_incoming(source, owner)
                .unwrap_err()
                .contains("source-only-object-actuals"));
            assert!(ledger.borrowed_formal_actuals[&passive_site]
                .as_ref()
                .unwrap()
                .require_executable_v1()
                .is_err());
            continue;
        }
        match mutation {
            1 => {
                ledger.borrowed_formal_actuals.remove(&passive_site);
            }
            2 => {
                ledger
                    .borrowed_formal_actuals
                    .insert(passive_site.clone(), Err("unrelated-source-refusal".into()));
            }
            3 => {
                let mut fabricated = ledger.borrowed_formal_actuals[&executable_site]
                    .clone()
                    .unwrap();
                let (ordinal, site, formal) = &passive_argument;
                fabricated.opaque_actuals = Box::new([super::super::super::super::borrowed_formal_actuals::PreparedBorrowedFormalActualV1 {
                    ordinal: *ordinal, site: site.clone(), formal: *formal,
                    source: BorrowedFormalActualSourceV1::Forwarded {
                        binding: passive_root, formal: passive_root,
                    },
                }]);
                fabricated.ordered_arguments = Box::new([
                    crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::BorrowedActual {
                        ordinal: *ordinal, site: site.clone(),
                    },
                ]);
                ledger
                    .borrowed_formal_actuals
                    .insert(passive_site.clone(), Ok(fabricated));
            }
            4 => {
                let rows = ledger
                    .borrowed_formal_actuals
                    .get_mut(&passive_site)
                    .unwrap()
                    .as_mut()
                    .unwrap();
                rows.ordered_arguments[0] =
                    crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(
                        99,
                    );
            }
            5 => {
                let donor = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
                    "box Wrap { v: i64 birth(v) { me.v = v } } box Door { birth() {} probe(p): i64 { if p == null { return 0 } return p.v } bridge(h: Wrap): i64 { local recv = new Door() local out = recv.probe(h) return 0 } } static box Main { main() { local recv = new Door() local out = recv.bridge(null) return 0 } }"
                ).expect("existing declared Object view fixture");
                let donor_source = donor
                    .ordinary_new_claim_ledger
                    .borrowed_formal_source
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unwrap();
                let view = donor_source.object_views.values().next().unwrap().clone();
                ledger
                    .borrowed_formal_source
                    .as_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .object_views
                    .insert(passive_argument.2, view);
            }
            6 => {
                ledger.original_parameter_contracts = None;
            }
            _ => unreachable!(),
        }
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let error = ledger
            .checked_borrowed_entry_incoming_core_v1(source, owner, true)
            .unwrap_err();
        let expected = match mutation {
            1 => "actuals-missing",
            2 => "unrelated-source-refusal",
            3 => "source-phase-drift",
            4 => "source-snapshot-drift",
            5 => "object-view-drift",
            6 => "original-catalog-missing",
            _ => unreachable!(),
        };
        assert!(error.contains(expected), "mutation {mutation}: {error}");
        ledger.lifecycle_local_call_sites.borrow_mut().clear();
        assert!(ledger.co_seal_static_local_routes_v1().is_err());
        assert!(
            ledger.lifecycle_local_call_sites.borrow().is_empty(),
            "partial route mutation {mutation}"
        );
    }
}
