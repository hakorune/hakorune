use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::map_lifecycle_undertaking::{
    verify_map_lifecycle_undertaking, MapLifecycleConsumerCapabilityV1,
    MapLifecycleOperationV1 as Op, MapLifecycleUndertakingIssueV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    MapEntryBorrowKindV1 as BorrowKind, MapEntryStoreClassV1 as StoreClass,
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
            Op::EntryStore(StoreClass::Scalar),
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
    // A `%{...}` argument under a non-terminal local call leaves the owner
    // without sealed terminal evidence — describe must stop, never emit a
    // partially known obligation set.
    let package = issue(
        "static box Helpers { consume(a, b) { return 30 } run(flag) { local r = Helpers.consume(%{\"a\" => flag}, 7) return 30 } }
         static box Main { main() { return 30 } }",
    )
    .expect("call-argument map package");
    assert!(matches!(
        package.describe_map_lifecycle_obligations(),
        Err(
            super::map_lifecycle_undertaking::MapObligationDescribeIssueV1::OwnerTerminalHomesUnavailable { .. }
        )
    ));
}

#[test]
fn terminal_qualified_call_argument_describes_the_named_handoff() {
    // `return <qualified call>` is its own OpaqueCall terminal class: the
    // owner has sealed terminal evidence, so describe reaches the argument
    // map and names `ArgumentHandoff` instead of flattening the missing
    // terminal into unavailable homes. Verify still cannot declare it.
    let package = issue(
        "static box Helpers { consume(a, b) { return 30 } run(flag) { return Helpers.consume(%{\"a\" => flag}, 7) } }
         static box Main { main() { return 30 } }",
    )
    .expect("terminal call-argument map package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("opaque-call terminal admits describe");
    let [owner] = obligations.as_ref() else {
        panic!("one map-owning owner");
    };
    let [site] = owner.sites() else {
        panic!("one map site");
    };
    assert!(site.operations().any(|op| op == Op::ArgumentHandoff));
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
    assert!(returned
        .operations()
        .any(|op| op == Op::OwnershipShare(BorrowKind::MapLocal)));
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
    assert!(!local
        .operations()
        .any(|op| matches!(op, Op::OwnershipShare(_))));
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
        Op::EntryStore(StoreClass::Scalar),
        Op::EntryStore(StoreClass::Transferred),
        Op::EntryStore(StoreClass::Opaque),
        Op::EntryDisplace,
        Op::OwnershipTransfer,
        Op::OwnershipShare(BorrowKind::Handle),
        Op::OwnershipShare(BorrowKind::MapLocal),
        Op::OwnershipShare(BorrowKind::Local),
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
        Op::EntryStore(StoreClass::Scalar),
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

#[test]
fn self_rooted_handle_entry_describes_handle_ownership_share() {
    // An untyped formal is an `OpaqueHandle` contract — a self-rooted
    // parameter handle. Its map entry is a `Borrowed` store class whose
    // sharing obligation carries the sealed leaf's `Handle` kind.
    let package = issue(
        "static box Work { stash(h) { local m = %{\"v\" => h} return 0 } }
         static box Main { main() { return 30 } }",
    )
    .expect("handle-borrow package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let [owner] = obligations.as_ref() else {
        panic!("one map-owning owner");
    };
    let [site] = owner.sites() else {
        panic!("one map site");
    };
    assert!(site
        .operations()
        .any(|op| op == Op::OwnershipShare(BorrowKind::Handle)));
    assert!(!site
        .operations()
        .any(|op| matches!(op, Op::EntryStore(StoreClass::Borrowed))));
    let [borrow] = site.borrows() else {
        panic!("one borrow row");
    };
    let _ = borrow.binding();
}

#[test]
fn declared_capability_seals_an_in_owner_handle_borrow() {
    // The same site the builder consumer now declares: a handle-borrowed
    // entry on a non-escaping map seals.
    let package = issue(
        "static box Work { stash(h) { local m = %{\"v\" => h} return 0 } }
         static box Main { main() { return 30 } }",
    )
    .expect("handle-borrow package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let capability = MapLifecycleConsumerCapabilityV1::covering([
        Op::ValueCreate,
        Op::EntryStore(StoreClass::Scalar),
        Op::EntryStore(StoreClass::Transferred),
        Op::EntryDisplace,
        Op::OwnershipTransfer,
        Op::OwnershipShare(BorrowKind::Handle),
        Op::ReturnHandoff,
        Op::NormalCleanup,
        Op::FaultCleanup,
    ]);
    let undertaking = verify_map_lifecycle_undertaking(&obligations, capability).unwrap();
    assert_eq!(undertaking.owners().len(), 1);
}

#[test]
fn verify_rejects_a_borrowed_entry_that_escapes_through_a_handoff() {
    // `local m = %{"v" => h}; return m` carries the handle borrow across
    // the return boundary. Every named operation is covered, but the
    // sealed vocabulary cannot prove the borrow target outlives the map
    // once it leaves the owner — the escape fails before emission.
    let package = issue(
        "static box Work { stash(h) { local m = %{\"v\" => h} return m } }
         static box Main { main() { return 30 } }",
    )
    .expect("escaping handle-borrow package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let capability = MapLifecycleConsumerCapabilityV1::covering([
        Op::ValueCreate,
        Op::EntryStore(StoreClass::Scalar),
        Op::EntryStore(StoreClass::Transferred),
        Op::EntryDisplace,
        Op::OwnershipTransfer,
        Op::OwnershipShare(BorrowKind::Handle),
        Op::ReturnHandoff,
        Op::NormalCleanup,
        Op::FaultCleanup,
    ]);
    let error = verify_map_lifecycle_undertaking(&obligations, capability).unwrap_err();
    assert!(matches!(
        error,
        MapLifecycleUndertakingIssueV1::BorrowedEntryEscape { .. }
    ));
}

#[test]
fn verify_rejects_map_local_and_kindless_local_borrow_kinds() {
    // `local a = %{...}` borrowed into a sibling map is a `MapLocal`
    // borrow — the declared `Handle` lane does not cover it, and a live
    // map-storage reference has no physical lane today.
    let package = issue(
        "static box Work { make() {
            local a = %{\"a\" => 1} local m = %{\"c\" => a} return 0 } }
         static box Main { main() { return 30 } }",
    )
    .expect("map-local borrow package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let capability = MapLifecycleConsumerCapabilityV1::covering([
        Op::ValueCreate,
        Op::EntryStore(StoreClass::Scalar),
        Op::EntryStore(StoreClass::Transferred),
        Op::EntryDisplace,
        Op::OwnershipTransfer,
        Op::OwnershipShare(BorrowKind::Handle),
        Op::ReturnHandoff,
        Op::NormalCleanup,
        Op::FaultCleanup,
    ]);
    let error = verify_map_lifecycle_undertaking(&obligations, capability).unwrap_err();
    assert!(matches!(
        error,
        MapLifecycleUndertakingIssueV1::UncoveredOperation {
            operation: Op::OwnershipShare(BorrowKind::MapLocal),
            ..
        }
    ));
}

#[test]
fn text_entry_describes_text_store_class() {
    // A string-literal entry seals its owned payload at Facts issuance:
    // describe classifies it `EntryStore(Text)`, never `Opaque`.
    let package = issue(
        "static box Work { make() { local m = %{\"name\" => \"main\"} return 0 } }
         static box Main { main() { return 30 } }",
    )
    .expect("text-entry package");
    let operations = operations_of(&package, 0, 0);
    assert!(operations.contains(&Op::EntryStore(StoreClass::Text)));
    assert!(!operations.contains(&Op::EntryStore(StoreClass::Opaque)));
}

#[test]
fn declared_capability_seals_a_text_entry() {
    // The declared consumer lane covers `EntryStore(Text)`: a
    // non-escaping map with a string-literal entry verifies.
    let package = issue(
        "static box Work { make() { local m = %{\"name\" => \"main\"} return 0 } }
         static box Main { main() { return 30 } }",
    )
    .expect("text-entry package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let capability = MapLifecycleConsumerCapabilityV1::covering([
        Op::ValueCreate,
        Op::EntryStore(StoreClass::Scalar),
        Op::EntryStore(StoreClass::Transferred),
        Op::EntryStore(StoreClass::Text),
        Op::EntryDisplace,
        Op::OwnershipTransfer,
        Op::OwnershipShare(BorrowKind::Handle),
        Op::ReturnHandoff,
        Op::NormalCleanup,
        Op::FaultCleanup,
    ]);
    let undertaking = verify_map_lifecycle_undertaking(&obligations, capability).unwrap();
    assert_eq!(undertaking.owners().len(), 1);
}

#[test]
fn verify_still_rejects_opaque_entry_classes() {
    // Non-empty arrays and nested-map children have no consumer lane:
    // they still describe `EntryStore(Opaque)` and fail verify.
    for entry in ["[1, 2]", "%{\"x\" => 1}"] {
        let package = issue(&format!(
            "static box Work {{ make() {{ local m = %{{\"op\" => {entry} }} return 0 }} }}
             static box Main {{ main() {{ return 30 }} }}",
        ))
        .expect("opaque-entry package");
        let obligations = package
            .describe_map_lifecycle_obligations()
            .expect("obligations describe");
        let capability = MapLifecycleConsumerCapabilityV1::covering([
            Op::ValueCreate,
            Op::EntryStore(StoreClass::Scalar),
            Op::EntryStore(StoreClass::Transferred),
            Op::EntryStore(StoreClass::Text),
            Op::EntryDisplace,
            Op::OwnershipTransfer,
            Op::OwnershipShare(BorrowKind::Handle),
            Op::ReturnHandoff,
            Op::NormalCleanup,
            Op::FaultCleanup,
        ]);
        let error = verify_map_lifecycle_undertaking(&obligations, capability).unwrap_err();
        assert!(
            matches!(
                error,
                MapLifecycleUndertakingIssueV1::UncoveredOperation {
                    operation: Op::EntryStore(StoreClass::Opaque),
                    ..
                }
            ),
            "{entry}: {error:?}"
        );
    }
}

#[test]
fn merged_route_shape_seals_text_and_empty_array_entries() {
    // The `local main` residual shape: a text entry and an empty-array
    // entry in one literal. Both are declared lanes — `[]` describes
    // `EntryStore(EmptyArray)`, never `Opaque`, and the declared
    // capability seals the site.
    let package = issue(
        "static box Work { make() {
            local m = %{\"name\" => \"main\", \"params\" => []} return 0 } }
         static box Main { main() { return 30 } }",
    )
    .expect("mixed-entry package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let [owner] = obligations.as_ref() else {
        panic!("one map-owning owner");
    };
    let [site] = owner.sites() else {
        panic!("one map site");
    };
    let operations: std::collections::BTreeSet<_> = site.operations().collect();
    assert!(operations.contains(&Op::EntryStore(StoreClass::Text)));
    assert!(operations.contains(&Op::EntryStore(StoreClass::EmptyArray)));
    assert!(!operations.contains(&Op::EntryStore(StoreClass::Opaque)));
    let capability = MapLifecycleConsumerCapabilityV1::covering([
        Op::ValueCreate,
        Op::EntryStore(StoreClass::Scalar),
        Op::EntryStore(StoreClass::Transferred),
        Op::EntryStore(StoreClass::Text),
        Op::EntryStore(StoreClass::EmptyArray),
        Op::EntryDisplace,
        Op::OwnershipTransfer,
        Op::OwnershipShare(BorrowKind::Handle),
        Op::ReturnHandoff,
        Op::NormalCleanup,
        Op::FaultCleanup,
    ]);
    let undertaking = verify_map_lifecycle_undertaking(&obligations, capability).unwrap();
    assert_eq!(undertaking.owners().len(), 1);
}

#[test]
fn entry_store_empty_array_still_requires_a_declared_operation() {
    // The marker lane is fail-closed like every other store class: a
    // capability missing `EntryStore(EmptyArray)` rejects at exactly that
    // operation, never silently treating the entry as owned.
    let package = issue(
        "static box Work { make() { local m = %{\"params\" => []} return 0 } }
         static box Main { main() { return 30 } }",
    )
    .expect("empty-array package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let capability = MapLifecycleConsumerCapabilityV1::covering([
        Op::ValueCreate,
        Op::EntryStore(StoreClass::Scalar),
        Op::EntryDisplace,
        Op::NormalCleanup,
        Op::FaultCleanup,
    ]);
    let error = verify_map_lifecycle_undertaking(&obligations, capability).unwrap_err();
    assert!(matches!(
        error,
        MapLifecycleUndertakingIssueV1::UncoveredOperation {
            operation: Op::EntryStore(StoreClass::EmptyArray),
            ..
        }
    ));
}

/// The full declared consumer lane, including every borrow kind — used to
/// isolate escape-admission behavior from capability coverage.
fn full_capability() -> MapLifecycleConsumerCapabilityV1 {
    MapLifecycleConsumerCapabilityV1::covering([
        Op::ValueCreate,
        Op::EntryStore(StoreClass::Scalar),
        Op::EntryStore(StoreClass::Transferred),
        Op::EntryStore(StoreClass::Text),
        Op::EntryStore(StoreClass::EmptyArray),
        Op::EntryDisplace,
        Op::OwnershipTransfer,
        Op::OwnershipShare(BorrowKind::Handle),
        Op::OwnershipShare(BorrowKind::MapLocal),
        Op::OwnershipShare(BorrowKind::Local),
        Op::SlotHandoff,
        Op::ReturnHandoff,
        Op::ArgumentHandoff,
        Op::ContainedHandoff,
        Op::NormalCleanup,
        Op::FaultCleanup,
    ])
}

#[test]
fn verify_admits_a_handle_borrow_on_the_argument_handoff_edge() {
    // `ArgumentHandoff` is the one proven edge: the install preflight
    // co-seals caller site+ordinal against the callee's `Map` formal and
    // read evidence, and the `BorrowedHandle` payload tag keeps the entry
    // from being misread as a scalar. A `Handle` borrow on that edge
    // verifies.
    let package = issue(
        "static box Helpers { consume(m) { return 30 } run(flag) { return Helpers.consume(%{\"a\" => flag}) } }
         static box Main { main() { return 30 } }",
    )
    .expect("argument-edge handle-borrow package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let [owner] = obligations.as_ref() else {
        panic!("one map-owning owner");
    };
    let [site] = owner.sites() else {
        panic!("one map site");
    };
    assert!(site.operations().any(|op| op == Op::ArgumentHandoff));
    assert!(site
        .operations()
        .any(|op| op == Op::OwnershipShare(BorrowKind::Handle)));
    verify_map_lifecycle_undertaking(&obligations, full_capability())
        .expect("handle borrow on the argument edge verifies");
}

#[test]
fn verify_still_rejects_a_handle_borrow_on_return_or_contained_handoffs() {
    // Only the argument edge carries a proven borrow contract. A handle
    // borrow riding `return` (ReturnBoundary or returned-local) still
    // escapes unproven, as does any contained handoff.
    for body in [
        "return %{\"v\" => h}",
        "local m = %{\"v\" => h} return m",
    ] {
        let package = issue(&format!(
            "static box Work {{ stash(h) {{ {body} }} }}
             static box Main {{ main() {{ return 30 }} }}",
        ))
        .expect("escaping handle-borrow package");
        let obligations = package
            .describe_map_lifecycle_obligations()
            .expect("obligations describe");
        let error =
            verify_map_lifecycle_undertaking(&obligations, full_capability()).unwrap_err();
        assert!(
            matches!(error, MapLifecycleUndertakingIssueV1::BorrowedEntryEscape { .. }),
            "{body}: {error:?}"
        );
    }
}

#[test]
fn verify_still_rejects_a_non_handle_borrow_on_the_argument_edge() {
    // `MapLocal`/`Local` borrows have no physical reference lane — they
    // escape unproven even on the argument edge (here with a capability
    // that declares every borrow kind, isolating the escape check from
    // coverage).
    let package = issue(
        "static box Helpers { consume(m) { return 30 } run() { local n = %{\"x\" => 1} return Helpers.consume(%{\"a\" => n}) } }
         static box Main { main() { return 30 } }",
    )
    .expect("argument-edge map-local borrow package");
    let obligations = package
        .describe_map_lifecycle_obligations()
        .expect("obligations describe");
    let error = verify_map_lifecycle_undertaking(&obligations, full_capability()).unwrap_err();
    assert!(matches!(
        error,
        MapLifecycleUndertakingIssueV1::BorrowedEntryEscape { .. }
    ));
}
