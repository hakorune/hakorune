//! Source capability retention never grants the Indexed-only runtime admission.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::builder::CompilationContext;

fn assert_install_stop(package: super::VerifiedNormalCallableSemanticPackageV1) {
    let mut context = CompilationContext::new();
    let error = match package.prepare_install(&mut context) {
        Err((_, error)) => error,
        Ok(_) => panic!("source capability cannot enable physical Map execution"),
    };
    assert!(matches!(
        error,
        super::install::NormalCallableSemanticPackageInstallIssueV1::MapLifecycleConsumerMissing
    ));
    assert!(context.callable_declaration_catalog_vacant());
}

#[test]
fn ordinary_i64_formal_repeated_values_keep_one_completion_and_map_cleanup() {
    let package = issue(
        "static box Work { stash(value: i64): i64 {
        local m = %{\"a\" => value, \"b\" => value} return 30
    } }",
    )
    .unwrap();
    let row = package.result_contracts.rows().next().unwrap();
    let contract = row.borrow();
    let completion = contract.completion_for_test();
    let flow = completion.cleanup().root_flow().unwrap();
    let map = flow.maps()[0].complete().unwrap();
    assert_eq!(map.entries().len(), 2);
    assert!(map
        .entries()
        .iter()
        .all(|entry| entry.transfer_home().is_none()));
    assert!(map.entries()[0].binding().is_some());
    assert_eq!(map.entries()[0].binding(), map.entries()[1].binding());
    assert_eq!(map.allocation_fault().count(), 0);
    assert_eq!(map.outer_after_installs(2).unwrap().count(), 0);
    assert_eq!(flow.terminal_homes().unwrap(), [map.destination()]);
    assert!(
        !package.ordinary_new_claim_ledger.has_map_source(map.site()),
        "ordinary Completion is not duplicated in the root ledger"
    );
    assert_install_stop(package);
}

#[test]
fn borrowed_formals_are_allowed_unused_but_do_not_issue_map_ownership() {
    for annotation in ["", ": StringBox"] {
        for (entry, complete) in [("30", true), ("value", false)] {
            let package = issue(&format!(
                "static box Work {{ stash(value{annotation}): i64 {{
                local m = %{{\"v\" => {entry}}} return 30
            }} }}"
            ))
            .unwrap();
            let row = package.result_contracts.rows().next().unwrap();
            let contract = row.borrow();
            let flow = contract
                .completion_for_test()
                .cleanup()
                .root_flow()
                .unwrap();
            assert_eq!(flow.maps()[0].complete().is_some(), complete);
            assert_install_stop(package);
        }
    }
}

#[test]
fn root_value_source_is_complete_but_private_emission_stops_before_progress() {
    for body in [
        "local m = %{\"v\" => 30} return 30",
        "local value = true local m = %{\"v\" => value} return 30",
    ] {
        let package = issue(&format!("static box Main {{ main() {{ {body} }} }}")).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let flow = ledger
            .root_completion_for_test()
            .cleanup()
            .root_flow()
            .unwrap();
        let map = flow.maps()[0].complete().unwrap();
        let declaration = package
            .batch()
            .declarations()
            .find(|d| d.owner() == map.site().owner())
            .unwrap();
        let relation = package
            .batch()
            .with_lowering_input(declaration.batch_slot(), |input| {
                input
                    .function()
                    .expression_source()
                    .initializers()
                    .find(|r| r.initializer_site() == Some(map.site().site()))
                    .unwrap()
                    .clone()
            })
            .unwrap();
        for _ in 0..2 {
            let error = ledger
                .begin_map_emission(map.site(), &relation)
                .unwrap_err();
            assert!(error.contains("map-value-consumer-missing"), "{error}");
        }
        let error = ledger
            .map_candidate_object(&map.entries()[0], crate::mir::ValueId::new(0))
            .unwrap_err();
        assert!(error.contains("map-value-consumer-missing"));
        assert_install_stop(package);
    }
}

#[test]
fn formal_projection_missing_duplicate_and_foreign_bindings_are_unavailable() {
    use crate::mir::resolved_semantics::home_new_prefix::{
        scan_new_home_flow, HomePrefixUnavailableV1,
    };
    let package = issue(
        "static box Work {
        stash(value: i64): i64 { local m = %{\"v\" => value} return 30 }
        other(foreign: i64): i64 { return 30 }
    }",
    )
    .unwrap();
    let row = package
        .result_contracts
        .rows()
        .find(|row| {
            row.borrow()
                .completion_for_test()
                .cleanup()
                .root_flow()
                .is_some()
        })
        .unwrap();
    let contract = row.borrow();
    let terminal = contract.completion_for_test().explicit_site();
    let own = package
        .parameter_contracts
        .iter()
        .find(|p| p.batch_slot == row.batch_slot())
        .unwrap();
    let foreign = package
        .parameter_contracts
        .iter()
        .find(|p| p.owner != own.owner)
        .unwrap();
    let good = (
        own.parameters[0].ordinal,
        own.parameters[0].binding,
        own.parameters[0].kind.home_demand(),
    );
    for parameters in [
        vec![],
        vec![good, good],
        vec![(good.0, foreign.parameters[0].binding, good.2)],
    ] {
        package
            .batch()
            .with_lowering_input(row.batch_slot(), |input| {
                let (_, flow, _, _) = scan_new_home_flow(
                    input,
                    &std::collections::BTreeMap::new(),
                    parameters,
                    terminal,
                    &mut |_, _, _, _, _| Ok::<_, std::convert::Infallible>(false),
                    &mut |_, _| Ok(false),
                )
                .unwrap();
                assert!(flow.maps().iter().all(|map| map.complete().is_none()));
                assert!(matches!(
                    flow.terminal_homes(),
                    Err(HomePrefixUnavailableV1::EntryDemandMissing)
                ));
            })
            .unwrap();
    }
}

#[test]
fn mixed_value_replacement_keeps_home_transfer_positions() {
    let package = issue(
        "box Page {} static box Main { main() {
        local a = new Page() local b = new Page()
        local m = %{\"key\" => a, \"key\" => 30, \"key\" => b} return 30
    } }",
    )
    .unwrap();
    let flow = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    let map = flow.maps()[0].complete().unwrap();
    let [first, value, last] = map.entries() else {
        panic!("three entries");
    };
    assert!(first.transfer_home().is_some());
    assert!(value.transfer_home().is_none());
    assert!(last.transfer_home().is_some());
    assert_eq!(value.displaced(), Some(first.site()));
    assert_eq!(last.displaced(), Some(value.site()));
    assert_eq!(
        map.outer_after_installs(1).unwrap().collect::<Vec<_>>(),
        [last.binding().unwrap()]
    );
    assert_eq!(
        map.outer_after_installs(2).unwrap().collect::<Vec<_>>(),
        [last.binding().unwrap()]
    );
    assert_eq!(map.outer_after_installs(3).unwrap().count(), 0);
    assert_eq!(flow.terminal_homes().unwrap(), [map.destination()]);
    assert_install_stop(package);
}
