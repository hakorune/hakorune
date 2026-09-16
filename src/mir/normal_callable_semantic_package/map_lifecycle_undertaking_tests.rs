use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::map_lifecycle_undertaking::{
    verify_map_lifecycle_undertaking, MapLifecycleConsumerCapabilityV1,
    MapLifecycleOperationV1 as Op, MapLifecycleUndertakingIssueV1,
};

fn operations_of(
    package: &super::VerifiedNormalCallableSemanticPackageV1,
    owner_index: usize,
    site_index: usize,
) -> std::collections::BTreeSet<Op> {
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    obligations[owner_index].sites()[site_index]
        .operations()
        .collect()
}

#[test]
fn local_binding_map_describes_create_store_and_both_cleanups() {
    let package = issue("static box Main { main() { local m = %{\"a\" => 1} return 0 } }")
        .expect("local map package");
    let operations = operations_of(&package, 0, 0);
    assert_eq!(
        operations,
        std::collections::BTreeSet::from([
            Op::ValueCreate,
            Op::EntryStore,
            Op::NormalCleanup,
            Op::FaultCleanup,
        ])
    );
}

#[test]
fn return_boundary_map_describes_return_handoff() {
    let package = issue(
        "static box Work { make() { return %{\"a\" => 1} } }
         static box Main { main() { return 30 } }",
    )
    .expect("return-position map package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    assert_eq!(obligations.len(), 1);
    let operations = operations_of(&package, 0, 0);
    assert!(operations.contains(&Op::ReturnHandoff));
    assert!(!operations.contains(&Op::SlotHandoff));
    assert!(!operations.contains(&Op::ArgumentHandoff));
}

#[test]
fn nested_map_child_describes_slot_handoff() {
    let package = issue(
        "static box Work { make(args) { return %{\"a\" => %{\"x\" => args}, \"b\" => \"s\"} } }
         static box Main { main() { return 30 } }",
    )
    .expect("nested map package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let [owner] = obligations.as_ref() else {
        panic!("one map-owning owner");
    };
    assert_eq!(owner.sites().len(), 2);
    let parent = &owner.sites()[0];
    let child = &owner.sites()[1];
    assert!(parent.operations().any(|op| op == Op::ReturnHandoff));
    assert!(child.operations().any(|op| op == Op::SlotHandoff));
    assert!(!child.operations().any(|op| op == Op::ReturnHandoff));
}

#[test]
fn call_argument_map_owner_stops_at_describe_without_exit_evidence() {
    // A `%{...}` call argument makes the owning call unavailable to the
    // i64-call prefix (literal-argument coverage only), so the owner has
    // no sealed terminal evidence — describe must stop, never emit a
    // partially known obligation set. The `ArgumentHandoff` operation
    // stays in the contract vocabulary for the future call-argument
    // lane; nothing reachable can declare it today.
    for body in [
        "local r = Helpers.consume(%{\"a\" => flag}, 7) return 30",
        "return Helpers.consume(%{\"a\" => flag}, 7)",
    ] {
        let package = issue(&format!(
            "static box Helpers {{ consume(a, b) {{ return 30 }} run(flag) {{ {body} }} }}
             static box Main {{ main() {{ return 30 }} }}",
        ))
        .expect("call-argument map package");
        assert!(matches!(
            package.describe_map_lifecycle_obligations(),
            Err(
                super::map_lifecycle_undertaking::MapObligationDescribeIssueV1::OwnerTerminalHomesUnavailable { .. }
            )
        ), "{body}");
    }
}

#[test]
fn returned_map_local_describes_return_handoff_on_the_local_site() {
    // `local m = %{}; return m` carries the map through a LocalBinding
    // destination — the handoff obligation comes from the sealed
    // terminal relation's `MapLocal` returned source, not from the
    // destination class.
    let package = issue(
        "static box Work { make() { local m = %{\"a\" => 1} return m } }
         static box Main { main() { return 30 } }",
    )
    .expect("map-local return package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let [owner] = obligations.as_ref() else {
        panic!("one map-owning owner");
    };
    let [site] = owner.sites() else {
        panic!("one map site");
    };
    assert!(matches!(
        site.destination(),
        crate::mir::resolved_semantics::home_new_prefix::MapDestinationV1::LocalBinding(_)
    ));
    assert!(site.operations().any(|op| op == Op::ReturnHandoff));
}

#[test]
fn transfer_home_entry_describes_ownership_transfer() {
    let package = issue(
        "static box Work { make() { local p = new Page() return %{\"h\" => p} } }
         static box Main { main() { return 30 } }
         box Page {}",
    )
    .expect("home-transfer map package");
    let operations = operations_of(&package, 0, 0);
    assert!(operations.contains(&Op::OwnershipTransfer));
}

#[test]
fn map_local_entry_describes_ownership_share_and_borrow_evidence() {
    let package = issue(
        "static box Work { make() {
            local m = %{\"a\" => 1} return %{\"c\" => m} } }
         static box Main { main() { return 30 } }",
    )
    .expect("map-local borrow package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let [owner] = obligations.as_ref() else {
        panic!("one map-owning owner");
    };
    assert_eq!(owner.sites().len(), 2);
    let returned = owner
        .sites()
        .iter()
        .find(|site| site.operations().any(|op| op == Op::ReturnHandoff))
        .expect("the returned map site");
    assert!(returned.operations().any(|op| op == Op::OwnershipShare));
    let [borrow] = returned.borrows() else {
        panic!("one borrow row");
    };
    // The borrow evidence keeps the exact binding for the C5b liveness check.
    let _ = borrow.binding();
    let local = owner
        .sites()
        .iter()
        .find(|site| site.operations().all(|op| op != Op::ReturnHandoff))
        .expect("the map local site");
    assert!(!local.operations().any(|op| op == Op::OwnershipShare));
}

#[test]
fn duplicate_key_entry_describes_entry_displace() {
    let package = issue(
        "static box Work { make() { return %{\"a\" => 1, \"a\" => 2} } }
         static box Main { main() { return 30 } }",
    )
    .expect("duplicate-key map package");
    let operations = operations_of(&package, 0, 0);
    assert!(operations.contains(&Op::EntryDisplace));
}

#[test]
fn undertaking_seals_when_capability_covers_every_obligation() {
    let package = issue(
        "static box Work { make() { local p = new Page()
            return %{\"a\" => %{\"x\" => 1}, \"h\" => p} } }
         static box Main { main() { return 30 } }
         box Page {}",
    )
    .expect("package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let capability = MapLifecycleConsumerCapabilityV1::covering([
        Op::ValueCreate,
        Op::EntryStore,
        Op::EntryDisplace,
        Op::OwnershipTransfer,
        Op::OwnershipShare,
        Op::SlotHandoff,
        Op::ReturnHandoff,
        Op::ArgumentHandoff,
        Op::ContainedHandoff,
        Op::NormalCleanup,
        Op::FaultCleanup,
    ]);
    let undertaking = verify_map_lifecycle_undertaking(&obligations, capability).unwrap();
    assert_eq!(undertaking.owners().len(), 1);
}

#[test]
fn undertaking_rejects_a_capability_gap_at_the_exact_site() {
    let package = issue(
        "static box Work { make() { return %{\"a\" => 1} } }
         static box Main { main() { return 30 } }",
    )
    .expect("package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let capability = MapLifecycleConsumerCapabilityV1::covering([
        Op::ValueCreate,
        Op::EntryStore,
        Op::NormalCleanup,
        Op::FaultCleanup,
    ]);
    let error = verify_map_lifecycle_undertaking(&obligations, capability).unwrap_err();
    assert!(matches!(
        error,
        MapLifecycleUndertakingIssueV1::UncoveredOperation {
            operation: Op::ReturnHandoff,
            ..
        }
    ));
}

#[test]
fn undertaking_rejects_a_vacuous_obligation_set() {
    let package = issue("static box Main { main() { return 30 } }").expect("package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    assert!(obligations.is_empty());
    let capability = MapLifecycleConsumerCapabilityV1::covering([Op::ValueCreate]);
    assert!(matches!(
        verify_map_lifecycle_undertaking(&obligations, capability).unwrap_err(),
        MapLifecycleUndertakingIssueV1::EmptyUndertaking
    ));
}

#[test]
fn sealed_membership_enumerates_every_map_owning_owner() {
    let package = issue(
        "static box Alpha { make() { return %{\"a\" => 1} } }
         static box Beta { build() { return %{\"b\" => 2} } }
         static box Main { main() { return 30 } }",
    )
    .expect("two map-owning owners");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    assert_eq!(obligations.len(), 2);
    assert_ne!(obligations[0].owner(), obligations[1].owner());
}
