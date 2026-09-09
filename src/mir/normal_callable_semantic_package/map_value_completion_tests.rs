//! Source capability retention never grants ordinary callable runtime admission.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::builder::CompilationContext;
use crate::mir::resolved_semantics::home_new_prefix::{
    MapValueSource, SourceScalarKind, TerminalRelationV1,
};

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
    let completion = contract.completion();
    let header = package.physical_header.row(row.batch_slot(), &package.result_contracts).unwrap();
    assert!(std::ptr::eq(header.completion(), completion));
    assert!(header.completion().cleanup().crossed_scopes().is_empty());
    let flow = completion.cleanup().root_flow().unwrap();
    let map = flow.maps()[0].complete().unwrap();
    assert_eq!(map.entries().len(), 2);
    assert!(map
        .entries()
        .iter()
        .all(|entry| entry.transfer_home().is_none()));
    assert!(map.entries()[0].binding().is_some());
    assert_eq!(map.entries()[0].binding(), map.entries()[1].binding());
    assert!(
        matches!(
            map.entries()[0].value_source(),
            Some(MapValueSource::Local { kind: Some(SourceScalarKind::Integer), .. })
        ),
        "exact declaration I64 survives the source Home walk"
    );
    assert_eq!(map.allocation_fault().count(), 0);
    assert_eq!(map.outer_after_installs(2).unwrap().count(), 0);
    assert_eq!(flow.terminal_homes().unwrap(), [map.destination()]);
    let Some(TerminalRelationV1::IntegerLiteral(terminal)) =
        contract.terminal_relation()
    else {
        panic!("ordinary source terminal retained with Completion");
    };
    assert_eq!(terminal.value(), 30);
    assert_eq!(terminal.owner(), completion.owner());
    assert_eq!(Some(terminal.return_site()), completion.explicit_site());
    let indexed = package
        .ordinary_new_claim_ledger
        .completion_for_owner(map.site().owner())
        .expect("owner-indexed ordinary Completion");
    assert!(
        package.ordinary_new_claim_ledger.has_map_source(map.site()),
        "owner-indexed ordinary Completion exposes the exact Map source"
    );
    assert!(
        std::ptr::eq(indexed, completion),
        "the owner index borrows the result row's Completion"
    );
    assert_install_stop(package);
}

#[test]
fn ordinary_i64_formal_alias_preserves_source_kind_without_a_home() {
    let package = issue(
        "static box Work { stash(value: i64): i64 {
        local alias = value local m = %{\"a\" => alias, \"b\" => value} return 30
    } }",
    )
    .unwrap();
    let row = package.result_contracts.rows().next().unwrap();
    let contract = row.borrow();
    let flow = contract.completion().cleanup().root_flow().unwrap();
    let map = flow.maps()[0].complete().unwrap();
    assert_eq!(map.entries().len(), 2);
    assert_ne!(map.entries()[0].binding(), map.entries()[1].binding());
    for entry in map.entries() {
        assert!(matches!(
            entry.value_source(),
            Some(MapValueSource::Local { kind: Some(SourceScalarKind::Integer), .. })
        ));
        assert!(entry.transfer_home().is_none());
    }
    assert_eq!(flow.terminal_homes().unwrap(), [map.destination()]);
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
                .completion()
                .cleanup()
                .root_flow()
                .unwrap();
            assert_eq!(flow.maps()[0].complete().is_some(), complete);
            let header = package.physical_header.row(row.batch_slot(), &package.result_contracts).unwrap();
            assert!(std::ptr::eq(header.completion(), contract.completion()));
            if !complete {
                assert!(header.completion().cleanup().terminal_homes().unwrap().is_err());
                assert!(contract.terminal_relation().is_none());
            }
            assert_install_stop(package);
        }
    }
}

#[test]
fn root_known_value_enters_progress_without_becoming_a_home() {
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
        ledger.begin_map_emission(map.site(), &relation).unwrap();
        assert!(ledger.begin_map_emission(map.site(), &relation).unwrap_err()
            .contains("map-duplicate-emission"));
        let error = ledger
            .map_candidate_object(&map.entries()[0], crate::mir::ValueId::new(0))
            .unwrap_err();
        assert!(error.contains("map-value-consumer-missing"));

    }
}

#[test]
fn app_main_call_admits_one_ordinary_map_callee_value_owner() {
    let package = issue(
        "static box Main {
        main() { return helper(30) }
        helper(value: i64): i64 { local m = %{\"v\" => value} return 30 }
    }",
    )
    .unwrap();
    assert!(package.has_app_main_direct_call_loan());
    let rows: Vec<_> = package.result_contracts.rows().collect();
    assert_eq!(rows.len(), 1, "AppMain does not acquire an ordinary seed");
    let contract = rows[0].borrow();
    let Some(TerminalRelationV1::IntegerLiteral(terminal)) =
        contract.terminal_relation()
    else {
        panic!("callee exact terminal retained");
    };
    assert_eq!(terminal.value(), 30);
    assert_eq!(terminal.owner(), contract.owner());
    assert_eq!(Some(terminal.return_site()), contract.completion().explicit_site());
    let mut context = CompilationContext::new();
    package
        .prepare_install(&mut context)
        .expect("one ordinary child Map owner is admitted");
    assert!(context.callable_declaration_catalog_vacant());
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
                .completion()
                .cleanup()
                .root_flow()
                .is_some()
        })
        .unwrap();
    let contract = row.borrow();
    let terminal = contract.completion().explicit_site();
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
        own.parameters[0].kind,
    );
    for parameters in [
        vec![],
        vec![good, good],
        vec![(good.0, foreign.parameters[0].binding, good.2)],
        vec![(good.0 + 1, good.1, good.2)],
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
                    &mut |_| Ok(false),
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
    assert!(package.prepare_install(&mut CompilationContext::new()).is_ok());
}

#[test]
fn map_scalar_literals_and_aliases_retain_exact_source_evidence() {
    for (literal, expected, kind) in [
        ("0", MapValueSource::Integer(0), SourceScalarKind::Integer),
        (
            "9223372036854775807",
            MapValueSource::Integer(i64::MAX),
            SourceScalarKind::Integer,
        ),
        ("true", MapValueSource::Bool(true), SourceScalarKind::Bool),
        ("false", MapValueSource::Bool(false), SourceScalarKind::Bool),
    ] {
        let package = issue(&format!(
            "static box Main {{ main() {{
            local value = {literal} local alias = value local final_alias = alias
            local m = %{{\"k\" => {literal}, \"k\" => final_alias, \"b\" => final_alias}}
            return 30
        }} }}"
        ))
        .unwrap();
        let flow = package
            .ordinary_new_claim_ledger
            .root_completion_for_test()
            .cleanup()
            .root_flow()
            .unwrap();
        let map = flow.maps()[0].complete().unwrap();
        let [direct, first, repeated] = map.entries() else {
            panic!("three entries")
        };
        assert_eq!(direct.value_source(), Some(&expected));
        assert!(
            matches!(first.value_source(), Some(MapValueSource::Local { kind: Some(k), .. }) if *k == kind)
        );
        assert_eq!(first.value_source(), repeated.value_source());
        assert_eq!(first.displaced(), Some(direct.site()));
        assert!(map
            .entries()
            .iter()
            .all(|entry| entry.transfer_home().is_none()));
        assert!(package.prepare_install(&mut CompilationContext::new()).is_ok());
    }
}

#[test]
fn source_write_does_not_reuse_a_stale_scalar_kind() {
    let package = issue(
        "static box Main { main() {
        local value = 30 value = true local m = %{\"k\" => value} return 30
    } }",
    )
    .unwrap();
    let flow = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    assert!(flow.maps().iter().all(|map| map.complete().is_none()));
    assert!(flow.terminal_homes().is_err());
    assert_install_stop(package);
}
