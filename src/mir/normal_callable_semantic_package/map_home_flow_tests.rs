use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::builder::CompilationContext;

fn source(body: &str) -> String {
    format!("box Page {{}} static box Main {{ main() {{ {body} }} }}")
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
    let outer = |count| map.outer_after_installs(count).unwrap().collect::<Vec<_>>();
    let live = |count| {
        map.live_after_installs(count)
            .unwrap()
            .cloned()
            .collect::<Vec<_>>()
    };
    assert_eq!(
        map.allocation_fault().collect::<Vec<_>>(),
        [c.binding(), b.binding(), a.binding()]
    );
    assert_eq!(outer(0), map.allocation_fault().collect::<Vec<_>>());
    assert_eq!(outer(1), [c.binding(), b.binding()]);
    assert!(live(0).is_empty());
    assert_eq!(live(1), [a.site().clone()]);
    assert_eq!(outer(2), [c.binding()]);
    assert!(outer(3).is_empty());
    assert_eq!(live(2), [b.site().clone(), a.site().clone()]);
    assert_eq!(live(3), [c.site().clone(), b.site().clone()]);
    assert!(map.outer_after_installs(4).is_none());
    assert!(map.live_after_installs(4).is_none());
    assert_eq!(c.displaced(), Some(a.site()));
    assert!(a.displaced().is_none() && b.displaced().is_none());
    assert_eq!((a.key(), b.key(), c.key()), ("a", "b", "a"));
    assert_eq!(flow.terminal_homes().unwrap(), [map.destination()]);
    let claims = package.ordinary_new_claim_ledger.pending_claims_for_test();
    for entry in map.entries() {
        assert!(claims.contains_key(entry.acquisition()));
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
                a.binding()
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
                foreign_map.destination()
            )
            .is_err());
        })
        .unwrap();
}

#[test]
fn map_install_stop_returns_same_source_product_and_keeps_catalog_vacant() {
    for body in [
        "local m = %{} return 30",
        "local a = new Page() local m = %{\"a\" => a} return 30",
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
            Err(package) => package,
            Ok(_) => panic!("Map source is not permission to install an unconnected consumer"),
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
fn map_candidate_alias_reuse_fresh_and_nested_are_not_transfer_evidence() {
    for expression in [
        "%{\"a\" => alias}",
        "%{\"a\" => a, \"b\" => a}",
        "%{\"a\" => new Page()}",
        "%{\"a\" => %{}}",
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
    }
}

#[test]
fn unavailable_prefix_and_implicit_exit_cannot_skip_map_install_stop() {
    for body in [
        "local m = %{}",
        "local a = new Page() local m = %{}",
        "local a = new Page() a = a local m = %{} return 30",
    ] {
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
    let terminal = flow.terminal_homes().unwrap();
    assert_eq!(terminal.len(), 4);
    assert_eq!(terminal[1], current.destination());
    assert_eq!(terminal[3], prior.destination());
    assert_eq!(
        current.outer_after_installs(0).unwrap().collect::<Vec<_>>(),
        [entry.binding(), terminal[2], prior.destination()]
    );
    assert_eq!(
        current.outer_after_installs(1).unwrap().collect::<Vec<_>>(),
        [terminal[2], prior.destination()]
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
    assert_eq!(later.home_prefix().unwrap().prior_homes(), &terminal[1..]);
}
