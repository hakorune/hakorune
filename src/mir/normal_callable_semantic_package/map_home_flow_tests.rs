use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::builder::CompilationContext;
use crate::mir::resolved_semantics::home_new_prefix::{MapDestinationV1, MapValueSource};
use crate::mir::resolved_semantics::{
    OwnedExprSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

fn source(body: &str) -> String {
    format!("box Page {{}} static box Main {{ main() {{ {body} }} }}")
}

#[test]
fn explicit_move_preserves_the_original_home_order_and_source() {
    let package = issue(&source(
        "local a = new Page() local z = new Page() local b = move a return 30",
    )).expect("source Home flow issues");
    let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
    let flow = completion.cleanup().root_flow().expect("root Home flow");
    let moves = flow.expression_plans().iter()
        .filter_map(|plan| plan.move_initializer_v1()).collect::<Vec<_>>();
    let [(plan, from, acquisition)] = moves.as_slice() else {
            panic!("one checked move initializer");
        };
    let destination = plan.root().initializer().unwrap().binding();
    let homes = flow.terminal_homes().expect("normal exit is covered");
    assert_eq!(homes.len(), 2);
    assert_eq!(homes[1].binding(), destination);
    assert_eq!(homes[1].stable_origin(), Some(*from));
    assert_eq!(acquisition.owner(), destination.owner());
    assert_ne!(homes[0].binding(), destination);
}

#[test]
fn explicit_move_rejects_later_reads_and_a_second_move() {
    use crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1;
    for body in [
        "local a = new Page() local b = move a local c = a return 30",
        "local a = new Page() local b = move a local c = move a return 30",
    ] {
        let error = match issue(&source(body)) {
            Err(error) => error,
            Ok(_) => panic!("moved-out binding must be rejected"),
        };
        assert!(matches!(error, super::NormalCallableSemanticPackageIssueV1::OrdinaryNew {
            _error: super::ordinary_new_coseal::OrdinaryNewCoSealIssueV1::RootTerminalSource(
                HomePrefixUnavailableV1::UseAfterMove(_))
        }), "{body}: {error:?}");
    }
}

#[test]
fn consecutive_explicit_moves_retain_one_original_acquisition() {
    let package = issue(&source(
        "local a = new Page() local b = move a local c = move b return 30",
    )).expect("successive moves have one source acquisition");
    let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
    let flow = completion.cleanup().root_flow().expect("root Home flow");
    let moves = flow.expression_plans().iter()
        .filter_map(|plan| plan.move_initializer_v1()).collect::<Vec<_>>();
    let [(first, from, acquisition), (second, second_from, second_acquisition)] = moves.as_slice() else {
        panic!("two checked move initializers");
    };
    assert_eq!(first.root().initializer().unwrap().binding(), *second_from);
    assert_eq!(*acquisition, *second_acquisition);
    let homes = flow.terminal_homes().expect("normal exit is covered");
    assert_eq!(homes.len(), 1);
    assert_eq!(homes[0].binding(), second.root().initializer().unwrap().binding());
    assert_eq!(homes[0].stable_origin(), Some(*from));
}

#[test]
fn non_app_main_map_owner_installs_with_covered_obligations() {
    let package = issue(
        "static box Helper { use(value: i64): i64 {
            local h = %{\"a\" => value} return 30
        } }
        static box Main { main() {
            local m = %{\"a\" => 1} return 30
        } }",
    )
    .expect("mixed AppMain and ordinary Map source");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .map(|prepared| prepared.commit())
        .expect("covered per-owner obligations admit install");
    let undertaking = installed
        .map_lifecycle_undertaking()
        .expect("Map owners seal one undertaking");
    assert_eq!(undertaking.owners().len(), 2);
}

#[test]
fn declared_root_unissued_map_sites_stop_before_install() {
    for body in [
        "local m = %{\"nested\" => %{}} return 30",
        "Helpers.consume(%{}) return 30",
        "if value { local m = %{} } return 30",
    ] {
        let program = format!(
            "static box Helpers {{ consume(value) {{ return 30 }} run(value) {{ {body} }} }}
             static box Main {{ main() {{ return 30 }} }}"
        );
        let package = issue(&program).unwrap_or_else(|error| {
            panic!("declared-root Map source inventory: {body}: {error:?}")
        });
        let mut context = CompilationContext::new();
        let issue = match package.prepare_install(&mut context) {
            Err((_, issue)) => issue,
            Ok(_) => panic!("unissued Map installed: {body}"),
        };
        // The stop stays typed: an undescribable obligation keeps its
        // describe cause, an uncovered operation keeps the undertaking
        // cause — neither collapses into ConsumerMissing.
        assert!(
            matches!(issue,
            super::install::NormalCallableSemanticPackageInstallIssueV1::MapObligationDescribe(_)
                | super::install::NormalCallableSemanticPackageInstallIssueV1::MapLifecycleUndertaking(_)
            ),
            "{issue:?}: {body}"
        );
        assert!(context.callable_declaration_catalog_vacant());
    }
    // Covered shapes admit install: a sealed `return %{...}` carries only
    // ReturnHandoff, and a self-rooted formal borrowed into an in-owner
    // map is `OwnershipShare(Handle)` — the InstallValue lane stores the
    // formal's value under the `BorrowedHandle` tag while the owner
    // keeps it alive.
    for body in ["return %{}", "local m = %{\"v\" => value} return 30"] {
        let package = issue(&format!(
            "static box Helpers {{ consume(value) {{ return 30 }} run(value) {{ {body} }} }}
             static box Main {{ main() {{ return 30 }} }}",
        ))
        .unwrap();
        let mut context = CompilationContext::new();
        assert!(package.prepare_install(&mut context).is_ok(), "{body}");
    }
}

#[test]
fn map_completion_retains_transfer_replacement_and_fault_successors() {
    let package = issue(&source(
        "local a = new Page() local b = new Page() local c = new Page()
         local m = %{\"a\" => a, \"b\" => b, \"a\" => c} return 30",
    ))
    .unwrap();
    let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
    let flow = completion.cleanup().root_flow().unwrap();
    let [observation] = flow.maps() else {
        panic!("one source Map");
    };
    let map = observation.complete().unwrap();
    let [a, b, c] = map.entries() else {
        panic!("three entries");
    };
    let outer = |count| map.outer_after_installs(count).unwrap().map(|home| home.binding()).collect::<Vec<_>>();
    let live = |count| {
        map.live_after_installs(count)
            .unwrap()
            .cloned()
            .collect::<Vec<_>>()
    };
    assert_eq!(
        map.allocation_fault().map(|home| home.binding()).collect::<Vec<_>>(),
        [
            c.binding().unwrap(),
            b.binding().unwrap(),
            a.binding().unwrap()
        ]
    );
    assert_eq!(outer(0), map.allocation_fault().map(|home| home.binding()).collect::<Vec<_>>());
    assert_eq!(outer(1), [c.binding().unwrap(), b.binding().unwrap()]);
    assert!(live(0).is_empty());
    assert_eq!(live(1), [a.site().clone()]);
    assert_eq!(outer(2), [c.binding().unwrap()]);
    assert!(outer(3).is_empty());
    assert_eq!(live(2), [b.site().clone(), a.site().clone()]);
    assert_eq!(live(3), [c.site().clone(), b.site().clone()]);
    assert!(map.outer_after_installs(4).is_none());
    assert!(map.live_after_installs(4).is_none());
    assert_eq!(c.displaced(), Some(a.site()));
    assert!(a.displaced().is_none() && b.displaced().is_none());
    assert_eq!((a.key(), b.key(), c.key()), ("a", "b", "a"));
    assert_eq!(
        flow.terminal_homes().unwrap().iter().map(|home| home.binding()).collect::<Vec<_>>(),
        [map.local_binding().unwrap()]
    );
    let claims = package.ordinary_new_claim_ledger.pending_claims_for_test();
    for entry in map.entries() {
        assert!(claims.contains_key(entry.transfer_home().unwrap().0));
    }
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.owner() == map.site().owner())
        .unwrap();
    package
        .batch()
        .with_lowering_input(declaration.batch_slot(), |input| {
            assert_eq!(
                map.source_scope(),
                input.function().lowering_roots().body_pair().scope()
            );
            assert_eq!(map.target_function(), input.function().function_region());
            assert!(crate::mir::resolved_control_flow::map_source_outward(
                input,
                map.site(),
                a.binding().unwrap()
            )
            .is_err());
        })
        .unwrap();
    let foreign = issue(&source("local m = %{} return 0")).unwrap();
    let foreign_map = &foreign
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap()
        .maps()[0]
        .complete()
        .unwrap();
    package
        .batch()
        .with_lowering_input(declaration.batch_slot(), |input| {
            assert!(crate::mir::resolved_control_flow::map_source_outward(
                input,
                foreign_map.site(),
                foreign_map.local_binding().unwrap()
            )
            .is_err());
        })
        .unwrap();
}

#[test]
fn map_annotation_refusal_returns_same_source_product_and_keeps_catalog_vacant() {
    for body in [
        "local m: i64 = %{} return 30",
        "local a = new Page() local m: i64 = %{\"a\" => a} return 30",
    ] {
        let package = issue(&source(body)).unwrap();
        let site = package
            .ordinary_new_claim_ledger
            .root_completion_for_test()
            .cleanup()
            .root_flow()
            .unwrap()
            .maps()[0]
            .site()
            .clone();
        let mut context = CompilationContext::new();
        let returned = match package.prepare_install(&mut context) {
            Err((package, issue)) => {
                assert!(matches!(
                    issue,
                    super::install::NormalCallableSemanticPackageInstallIssueV1::MapLocalAnnotation(
                        _
                    )
                ));
                package
            }
            Ok(_) => panic!("Map annotation must reject before install"),
        };
        assert!(context.callable_declaration_catalog_vacant());
        let flow = returned
            .ordinary_new_claim_ledger
            .root_completion_for_test()
            .cleanup()
            .root_flow()
            .unwrap();
        assert_eq!(flow.maps()[0].complete().unwrap().site(), &site);
        assert!(flow.terminal_homes().is_ok());
    }
}

#[test]
fn map_candidate_alias_reuse_and_fresh_are_not_transfer_evidence() {
    for expression in [
        "%{\"a\" => alias}",
        "%{\"a\" => a, \"b\" => a}",
        "%{\"a\" => new Page()}",
    ] {
        let package = issue(&source(&format!(
            "local a = new Page() local alias = a local m = {expression} return 30",
        )))
        .unwrap();
        let flow = package
            .ordinary_new_claim_ledger
            .root_completion_for_test()
            .cleanup()
            .root_flow()
            .unwrap();
        assert!(
            flow.maps().iter().all(|row| row.complete().is_none()),
            "no partial Map publication: {expression}"
        );
        assert!(flow.terminal_homes().is_err());
        let mut context = CompilationContext::new();
        assert!(
            package.prepare_install(&mut context).is_err(),
            "first unavailable Map must not fall through"
        );
        assert!(context.callable_declaration_catalog_vacant());
    }
}

#[test]
fn map_transfer_invalidates_old_local_and_alias_field_observation() {
    for receiver in ["a", "alias"] {
        let package = issue(&format!(
            "box Page {{ value: i64\nbirth(value) {{ me.value = value }} }}
             static box Main {{ main() {{ local a = new Page(7) local alias = a
             local m = %{{\"a\" => a}} return {receiver}.value }} }}",
        ))
        .unwrap();
        let flow = package
            .ordinary_new_claim_ledger
            .root_completion_for_test()
            .cleanup()
            .root_flow()
            .unwrap();
        assert_eq!(flow.maps().len(), 1);
        assert!(flow.terminal_homes().is_err());
        assert!(package
            .ordinary_new_claim_ledger
            .requires_map_lifecycle_consumer());
        // The completed Map row survives the failed exit analysis; the
        // undertaking co-seals exit evidence, so install must reject
        // before any catalog mutation — never admit on the row alone.
        let mut context = CompilationContext::new();
        assert!(package.prepare_install(&mut context).is_err(), "{receiver}");
        assert!(context.callable_declaration_catalog_vacant());
    }
}

#[test]
fn unavailable_prefix_cannot_skip_map_install_stop() {
    let body = "local a = new Page() a = a local m = %{} return 30";
    let package = issue(&source(body)).unwrap();
    let flow = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    assert!(
        !flow.maps().is_empty(),
        "Map cannot disappear from an incomplete flow: {body}"
    );
    let mut context = CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_err());
    assert!(context.callable_declaration_catalog_vacant());
}

#[test]
fn implicit_exit_map_owner_rejects_without_terminal_evidence() {
    // An implicit exit issues no terminal Homes set and no terminal
    // relation — the undertaking cannot co-seal cleanup evidence that
    // was never issued, so the owner stops at preflight. Unit exits
    // (`return`) carry sealed evidence and stay admitted.
    for body in ["local m = %{}", "local a = new Page() local m = %{}"] {
        let package = issue(&source(body)).unwrap();
        let mut context = CompilationContext::new();
        assert!(package.prepare_install(&mut context).is_err(), "{body}");
        assert!(context.callable_declaration_catalog_vacant());
    }
}

#[test]
fn map_delta_preserves_untransferred_homes_and_later_new_fault_order() {
    let package = issue(&source(
        "local prior = %{} local p = new Page() local a = new Page()
         local m = %{\"a\" => a} local d = new Page() return 30",
    ))
    .unwrap();
    let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
    let flow = completion.cleanup().root_flow().unwrap();
    let [prior, current] = flow.maps() else {
        panic!("two Maps");
    };
    let prior = prior.complete().unwrap();
    let current = current.complete().unwrap();
    assert_eq!(prior.allocation_fault().count(), 0);
    assert_eq!(prior.outer_after_installs(0).unwrap().count(), 0);
    assert!(prior.outer_after_installs(1).is_none());
    let [entry] = current.entries() else {
        panic!("one transfer");
    };
    let terminal: Vec<_> = flow.terminal_homes().unwrap().iter().map(|home| home.binding()).collect();
    assert_eq!(terminal.len(), 4);
    assert_eq!(terminal[1], current.local_binding().unwrap());
    assert_eq!(terminal[3], prior.local_binding().unwrap());
    assert_eq!(
        current.outer_after_installs(0).unwrap().map(|home| home.binding()).collect::<Vec<_>>(),
        [
            entry.binding().unwrap(),
            terminal[2],
            prior.local_binding().unwrap()
        ]
    );
    assert_eq!(
        current.outer_after_installs(1).unwrap().map(|home| home.binding()).collect::<Vec<_>>(),
        [terminal[2], prior.local_binding().unwrap()]
    );
    let claims = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let later = claims
        .values()
        .find(|claim| {
            claim
                .home_prefix()
                .is_ok_and(|prefix| prefix.destination() == terminal[0])
        })
        .expect("later New source prefix");
    assert_eq!(later.home_prefix().unwrap().prior_homes().iter().map(|home| home.binding()).collect::<Vec<_>>(), &terminal[1..]);
}

#[test]
fn map_install_accepts_complete_unannotated_root_and_aliases() {
    for body in [
        "local m = %{} return 30",
        "local m = %{} local alias = m local again = alias return 30",
        "local a = new Page() local m = %{\"a\" => a} return 30",
    ] {
        let package = issue(&source(body)).unwrap();
        let mut context = CompilationContext::new();
        package
            .prepare_install(&mut context)
            .expect("ready source Map cohort");
        assert!(
            context.callable_declaration_catalog_vacant(),
            "preflight does not commit"
        );
    }
}

#[test]
fn opaque_entry_classes_reject_at_preflight_before_catalog() {
    // `[...]`/`%{...}` child values have no consumer install lane; the
    // undertaking describes `EntryStore(Opaque)` and verify fails
    // before catalog mutation — the lowering lane's
    // `map-value-consumer-missing` is never reached. String entries are
    // the declared `EntryStore(Text)` lane, not this reject set.
    for entry in ["[1, 2]", "%{\"x\" => 1}"] {
        let package = issue(&format!(
            "static box Work {{ make() {{ return %{{\"op\" => {entry} }} }} }}
             static box Main {{ main() {{ return 30 }} }}",
        ))
        .unwrap();
        let mut context = CompilationContext::new();
        assert!(package.prepare_install(&mut context).is_err(), "{entry}");
        assert!(context.callable_declaration_catalog_vacant());
    }
}

#[test]
fn map_install_rejects_annotated_aliases() {
    for body in [
        "local m: Array<i64> = %{} return 30",
        "local m = %{} local alias: i64 = m return 30",
        "local m = %{} local alias = m local again: Array<i64> = alias return 30",
    ] {
        let package = issue(&source(body)).unwrap();
        let mut context = CompilationContext::new();
        assert!(package.prepare_install(&mut context).is_err(), "{body}");
        assert!(context.callable_declaration_catalog_vacant());
    }
}

#[test]
fn unit_exit_and_unready_new_owners_install_with_covered_obligations() {
    // Unit exits and sibling `new` claims are not Map lifecycle
    // obligations: covered obligations admit install and each lane keeps
    // its own named rejection downstream.
    let package = issue(&source("local m = %{} return")).unwrap();
    let mut context = CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
    let package = issue("box Bad { value } static box Main { main() { local m = %{} local bad = new Bad() return 30 } }").unwrap();
    let mut context = CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

#[test]
fn map_annotation_preflight_preserves_malformed_diagnostic() {
    let package = issue(&source("local m: Array<bogus> = %{} return 30")).unwrap();
    let mut context = CompilationContext::new();
    let error = match package.prepare_install(&mut context) {
        Err((_, error)) => error,
        Ok(_) => panic!("malformed annotation accepted"),
    };
    let super::install::NormalCallableSemanticPackageInstallIssueV1::MapLocalAnnotation(message) =
        error
    else {
        panic!("Local annotation owner must report malformed annotation");
    };
    assert!(message.contains("bogus"), "{message}");
    assert!(context.callable_declaration_catalog_vacant());
}

#[test]
fn map_preflight_keeps_non_map_numeric_locals_and_foreign_same_name_separate() {
    let program = "static box Other { run() { local m: i64 = 30 return 30 } }
        static box Main { main() {
            local value: i64 = 30 local alias: i64 = value local m = %{} return 30
        } }";
    let package = issue(program).unwrap();
    let map_binding = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap()
        .maps()[0]
        .complete()
        .unwrap()
        .local_binding()
        .unwrap();
    let mut numeric_bindings = Vec::new();
    for declaration in package.batch().declarations() {
        package
            .batch()
            .with_lowering_input(declaration.batch_slot(), |input| {
                numeric_bindings.extend(
                    input
                        .function()
                        .expression_source()
                        .initializers()
                        .filter(|row| row.declared_type_name() == Some("i64"))
                        .map(|row| row.binding()),
                );
            })
            .unwrap();
    }
    assert_eq!(numeric_bindings.len(), 3);
    assert!(numeric_bindings
        .iter()
        .all(|binding| *binding != map_binding));
    assert!(numeric_bindings
        .iter()
        .any(|binding| binding.owner() != map_binding.owner()));
    let mut context = CompilationContext::new();
    package
        .prepare_install(&mut context)
        .expect("numeric locals are not Map aliases");
    assert!(context.callable_declaration_catalog_vacant());
}

#[test]
fn return_boundary_map_carries_exact_exit_membership() {
    let package = issue(&source("local a = new Page() return %{\"a\" => a}"))
        .expect("return-boundary Map source");
    let completion = package.ordinary_new_claim_ledger.root_completion_for_test();
    let flow = completion.cleanup().root_flow().unwrap();
    let [observation] = flow.maps() else {
        panic!("one source Map");
    };
    let map = observation.complete().unwrap();
    assert_eq!(map.local_binding(), None);
    let expected_return = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(1),
    ]));
    assert_eq!(
        map.destination(),
        &MapDestinationV1::ReturnBoundary(expected_return)
    );
    let [entry] = map.entries() else {
        panic!("one entry");
    };
    assert!(entry.transfer_home().is_some());
    // The returned Map's value coverage is proven by the Value relation;
    // `a` was consumed into the map and nothing stays for terminal cleanup.
    assert!(flow.terminal_homes().unwrap().is_empty());
    let Some(crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Value(relation)) =
        package
            .ordinary_new_claim_ledger
            .sole_terminal_relation_for_owner(completion.owner())
    else {
        panic!("return-boundary Map terminal relation");
    };
    assert!(matches!(
        relation.returned(),
        crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1::MapLiteral(
            site
        ) if site.site() == map.site().site()
    ));
}

#[test]
fn return_boundary_outward_rejects_foreign_and_non_return_membership() {
    let package = issue(&source("local m = %{\"a\" => 1} return 30")).unwrap();
    let flow = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    let local_map = flow.maps()[0].complete().unwrap();
    let return_map_package = issue(&source("return %{\"a\" => 1}")).unwrap();
    let return_flow = return_map_package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    let return_map = return_flow.maps()[0].complete().unwrap();
    let MapDestinationV1::ReturnBoundary(return_site) = return_map.destination() else {
        panic!("return-boundary destination");
    };
    package.batch().declarations().for_each(|declaration| {
        package
            .batch()
            .with_lowering_input(declaration.batch_slot(), |input| {
                // A local-initializer Map site is not a return value.
                assert!(crate::mir::resolved_control_flow::map_return_outward(
                    input,
                    local_map.site(),
                    return_site
                )
                .is_err());
                // A foreign return-boundary site is not this function's exit.
                assert!(crate::mir::resolved_control_flow::map_return_outward(
                    input,
                    return_map.site(),
                    return_site
                )
                .is_err());
            })
            .unwrap();
    });
}

#[test]
fn return_boundary_map_admits_string_literal_and_borrowed_param_handle() {
    let package = issue(
        "static box Work { make(args) { return %{\"op\" => \"const\", \"args\" => args} } }
         static box Main { main() { return 30 } }",
    )
    .expect("return-boundary Map with string and param-handle entries");
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == 1)
        .expect("Work::make declaration");
    let site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]));
    let owned = OwnedExprSiteV1::new(declaration.owner(), site);
    let map = package
        .ordinary_new_claim_ledger
        .map_flow(&owned)
        .expect("return-boundary Map row completes");
    assert!(matches!(
        map.destination(),
        MapDestinationV1::ReturnBoundary(_)
    ));
    let [op, args] = map.entries() else {
        panic!("two entries");
    };
    assert_eq!(op.key(), "op");
    assert_eq!(
        op.value_source(),
        Some(&MapValueSource::String("const".into()))
    );
    assert_eq!(args.key(), "args");
    let Some(MapValueSource::BorrowedHandle(root)) = args.value_source() else {
        panic!("param handle stays a borrowed root, never a home");
    };
    assert_eq!(args.transfer_home(), None);
    assert_eq!(args.binding(), Some(*root));
    // Neither new class claims a scalar kind; downstream stays fail-closed.
    assert!(map.entries().iter().all(|entry| entry
        .value_source()
        .unwrap()
        .scalar_kind()
        .is_none()));
}

#[test]
fn return_boundary_map_rejects_uncovered_entry_value_classes() {
    for (body, expected_maps) in [
        // a live Home element inside an array is a transfer question,
        // never a leaf
        ("local p = new Page() return %{\"a\" => [p]}", 1usize),
        // alias of a live Home is a transfer question, not a borrow
        ("local p = new Page() local a = p return %{\"x\" => a}", 1),
        // an uninitialized local has no live object to reference
        ("local u return %{\"x\" => u}", 1),
    ] {
        let package = issue(&source(body)).unwrap();
        let flow = package
            .ordinary_new_claim_ledger
            .root_completion_for_test()
            .cleanup()
            .root_flow()
            .unwrap();
        assert_eq!(flow.maps().len(), expected_maps, "{body}");
        let observation = flow.maps().last().unwrap();
        assert!(
            observation.complete().is_none(),
            "{body} must stay Unavailable"
        );
    }
}

#[test]
fn return_boundary_map_reuses_one_borrowed_param_across_entries() {
    let package = issue(
        "static box Work { make(args) { return %{\"a\" => args, \"b\" => args} } }
         static box Main { main() { return 30 } }",
    )
    .unwrap();
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == 1)
        .unwrap();
    let site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]));
    let owned = OwnedExprSiteV1::new(declaration.owner(), site);
    let map = package
        .ordinary_new_claim_ledger
        .map_flow(&owned)
        .expect("repeated borrow completes");
    let [a, b] = map.entries() else {
        panic!("two entries");
    };
    assert_eq!(a.binding(), b.binding());
    assert!(a.transfer_home().is_none() && b.transfer_home().is_none());
}

#[test]
fn stored_map_get_keeps_its_original_local_source() {
    use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SelectedNormalCallableKeyV1};
    use crate::mir::resolved_semantics::home_new_prefix::MapHomeObservation;
    let source = r#"
        box Item { value: i64 birth() { me.value = 1 } }
        box Holder {
            init { stats }
            birth() { me.stats = new MapBox() }
            add(key) { local value = new Item() me.stats.set(key, value) return 0 }
            probe() { local key = "seed" local item = me.stats.get(key) if item == null { return 0 } local v = item.value return 0 }
        }
        static box Main { main() { return 0 } }
    "#;
    let package = issue(source).expect("source Map and holder");
    let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Holder", "probe", 0);
    let slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key)).unwrap();
    package.batch().with_lowering_input(slot, |input| {
        let flow = package.ordinary_new_claim_ledger.completion_for_owner(input.owner())
            .expect("probe completion").cleanup().root_flow().expect("probe source flow");
        let frame = flow.maps().iter().find_map(|row| match row {
            MapHomeObservation::FieldGet(frame) => Some(frame), _ => None,
        }).expect("original get retained");
        let consumer = package.ordinary_new_claim_ledger
            .stored_map_get_local_source_v1(input.owner(), frame.statement().node())
            .expect("consumer rechecks source").expect("consumer receives original get");
        assert!(std::rc::Rc::ptr_eq(frame, &consumer));
        assert!(flow.expression_plans().iter().filter_map(|plan| plan.evaluation())
            .flat_map(|evaluation| evaluation.operations())
            .filter_map(|operation| operation.nullable_null_v1())
            .any(|(_, _, observed)| std::rc::Rc::ptr_eq(frame, observed)),
            "null check consumes the original get frame");
        assert!(flow.all_exits_ready(), "field initializer and final return must be covered");
        let field_site = input.body_shape().unwrap().expressions().iter().find_map(|shape| match shape {
            crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { site, field, .. }
                if field.as_ref() == "value" => Some(OwnedExprSiteV1::new(input.owner(), site.clone())),
            _ => None,
        }).expect("original field read");
        assert!(package.ordinary_new_claim_ledger.staged_local_field_source(&field_site).is_some(),
            "nonnull field read reaches the existing source consumer");
    }).unwrap();
    let changed = source.replace("if item == null", "me.add(key) if item == null");
    let rejected = issue(&changed).expect("changed Map source is observed");
    let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Holder", "probe", 0);
    let slot = rejected.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key)).unwrap();
    rejected.batch().with_lowering_input(slot, |input| {
        let flow = rejected.ordinary_new_claim_ledger.completion_for_owner(input.owner())
            .expect("changed probe completion").cleanup().root_flow().expect("changed probe source flow");
        assert!(!flow.maps().iter().any(|row| matches!(row, MapHomeObservation::FieldGet(_))),
            "intervening call must not issue a passive get frame");
    }).unwrap();
}
