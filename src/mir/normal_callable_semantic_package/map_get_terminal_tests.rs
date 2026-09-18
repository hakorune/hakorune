//! Focused tests for `return <map>.get("<key>")` — the bounded readable-Map
//! terminal. The relation records owner, sites, receiver binding/class, and
//! the sealed literal key; it owns no physical value, kernel symbol, ABI, or
//! backend authority.
//!
//! Owned locals keep their End obligation after the read; `: MapBox` formals
//! are caller-owned storage borrowed read-only — they must never gain an End
//! obligation, ride the map-local return lane, or transfer ownership back.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::resolved_semantics::home_new_prefix::{
    HomePrefixUnavailableV1, TerminalMapGetReceiverClassV1, TerminalRelationV1,
};

/// Covered obligations admit install; the physical execution boundary
/// stays downstream at lowering.
fn assert_install_admits(package: super::VerifiedNormalCallableSemanticPackageV1) {
    let mut context = crate::mir::builder::CompilationContext::new();
    assert!(package.prepare_install(&mut context).is_ok());
}

fn contract_for<'a>(
    package: &'a super::VerifiedNormalCallableSemanticPackageV1,
    parameters: u32,
) -> super::result_contract::CallableResultContractRefV1<'a> {
    let owner = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == parameters)
        .expect("declaration row")
        .owner();
    package
        .result_contracts
        .rows()
        .find(|row| row.borrow().owner() == owner)
        .expect("contract row")
        .borrow()
}

#[test]
fn owned_local_map_get_issues_terminal_relation_and_keeps_cleanup() {
    let package = issue(
        "static box Main { main() { local m = %{\"k\" => 7} return m.get(\"k\") } }",
    )
    .expect("owned map-get package");
    // The AppMain root carries no result-contract row; its relation lives on
    // the claim ledger keyed by the root owner.
    let ledger = &package.ordinary_new_claim_ledger;
    let completion = ledger.root_completion_for_test();
    let owner = completion.owner();
    let Some(TerminalRelationV1::MapGet(relation)) = ledger.terminal_relation_for_owner(owner)
    else {
        panic!("owned terminal map-get issues the MapGet relation");
    };
    assert_eq!(relation.owner(), owner);
    assert_eq!(
        relation.receiver_class(),
        TerminalMapGetReceiverClassV1::OwnedLocal
    );
    assert_eq!(relation.key(), "k");
    // The read does not consume the map; the owned local still owes its End.
    let flow = completion.cleanup().root_flow().expect("root flow");
    assert!(flow.terminal_homes().unwrap().contains(&relation.receiver()));
    assert_install_admits(package);
}

#[test]
fn owned_local_map_get_in_child_owner_keeps_cleanup() {
    let package = issue(
        "static box Work { read() { local m = %{\"k\" => 7} return m.get(\"k\") } }
         static box Main { main() { return 0 } }",
    )
    .expect("child-owned map-get package");
    let contract = package
        .result_contracts
        .rows()
        .map(|row| row.borrow())
        .find(|row| {
            matches!(row.terminal_relation(), Some(TerminalRelationV1::MapGet(_)))
        })
        .expect("child contract row");
    let Some(TerminalRelationV1::MapGet(relation)) = contract.terminal_relation() else {
        panic!("child terminal map-get issues the MapGet relation");
    };
    assert_eq!(
        relation.receiver_class(),
        TerminalMapGetReceiverClassV1::OwnedLocal
    );
    let flow = contract
        .completion()
        .cleanup()
        .root_flow()
        .expect("root flow");
    assert!(flow.terminal_homes().unwrap().contains(&relation.receiver()));
}

#[test]
fn borrowed_map_parameter_map_get_issues_terminal_relation_without_cleanup() {
    let package = issue(
        "static box Work { read_k(m: MapBox): i64 { return m.get(\"k\") } }
         static box Main { main() { return 0 } }",
    )
    .expect("borrowed map-get package");
    let contract = contract_for(&package, 1);
    let Some(TerminalRelationV1::MapGet(relation)) = contract.terminal_relation() else {
        panic!("borrowed terminal map-get issues the MapGet relation");
    };
    assert_eq!(relation.owner(), contract.owner());
    assert_eq!(
        relation.receiver_class(),
        TerminalMapGetReceiverClassV1::BorrowedParameter
    );
    assert_eq!(relation.key(), "k");
    // Caller-owned storage: the callee owes no End for the borrowed formal.
    let flow = contract
        .completion()
        .cleanup()
        .root_flow()
        .expect("root flow");
    assert!(flow.terminal_homes().unwrap().is_empty());
}

/// `return <borrowed map>` is an escape: the borrowed formal can never ride
/// the map-local return lane, so the terminal stays uncovered and fails
/// closed at the named boundary.
#[test]
fn borrowed_map_parameter_return_stays_uncovered() {
    let package = issue(
        "static box Work { read_k(m: MapBox) { return m } }
         static box Main { main() { return 0 } }",
    )
    .expect("borrowed-map return package");
    let contract = contract_for(&package, 1);
    assert!(
        contract.terminal_relation().is_none(),
        "borrowed-map return issues no relation"
    );
    assert!(
        matches!(
            contract
                .completion()
                .cleanup()
                .root_flow()
                .expect("root flow")
                .terminal_homes(),
            Err(HomePrefixUnavailableV1::ReturnValueNotCovered(_))
        ),
        "borrowed-map return stays uncovered"
    );
}

/// A map carrying a `Borrowed` handle entry stays unreadable: the store lane
/// conflates the handle's i64 into `CheckedMapPayload::I64`, so a checked get
/// would silently misread it.  Until T1-γ's payload tags land, the seal
/// refuses the combination rather than letting the read decide.
#[test]
fn owned_local_map_get_with_borrowed_handle_entry_stays_uncovered() {
    let package = issue(
        "static box Work { read_k(h) { local m = %{\"k\" => h} return m.get(\"k\") } }
         static box Main { main() { return 0 } }",
    )
    .expect("borrowed-entry map-get package");
    let contract = contract_for(&package, 1);
    assert!(
        contract.terminal_relation().is_none(),
        "borrowed-entry map issues no MapGet relation"
    );
    assert!(
        matches!(
            contract
                .completion()
                .cleanup()
                .root_flow()
                .expect("root flow")
                .terminal_homes(),
            Err(HomePrefixUnavailableV1::ReturnValueNotCovered(_))
        ),
        "borrowed-entry map stays uncovered"
    );
}

/// A call-returned map local (`local m = make_map()`) carries no `%{...}`
/// flow observation, so its entry classes cannot be verified at this
/// seal — the read stays uncovered rather than trusting unseen entries.
/// The uncovered terminal then fails the caller's own lifecycle-edge
/// admission (`terminal_homes` is `Err`), so the package rejects
/// fail-closed instead of emitting an unchecked read.
#[test]
fn call_returned_map_get_stays_uncovered() {
    let result = issue(
        "static box Main { main() { local r = use_map(10) return 30 } use_map(seed: i64): i64 { local m = make_map() return m.get(\"k\") } make_map() { return %{\"a\" => 1} } }",
    );
    assert!(
        matches!(
            result,
            Err(super::NormalCallableSemanticPackageIssueV1::DirectCall {
                _error: super::issuer::DirectCallDispositionIssueV1::Loan(
                    super::direct_call_loan::DirectCallLoanErrorV1::LifecycleSourceMismatch
                ),
            })
        ),
        "call-returned map read rejects the package: {result:?}"
    );
}

/// The key is a sealed source literal. A parameter-bound or computed key
/// stays uncovered rather than widening the contract.
#[test]
fn map_get_with_non_literal_key_stays_uncovered() {
    let package = issue(
        "static box Work { read_k(m: MapBox, k) { return m.get(k) } }
         static box Main { main() { return 0 } }",
    )
    .expect("non-literal key package");
    let contract = contract_for(&package, 2);
    assert!(
        contract.terminal_relation().is_none(),
        "non-literal key issues no relation"
    );
    assert!(
        matches!(
            contract
                .completion()
                .cleanup()
                .root_flow()
                .expect("root flow")
                .terminal_homes(),
            Err(HomePrefixUnavailableV1::ReturnValueNotCovered(_))
        ),
        "non-literal key stays uncovered"
    );
}

/// Only the `get` selector is readable. Other selectors on the same map
/// receiver stay uncovered — writes and mutations are outside this lane.
#[test]
fn map_terminal_with_non_get_selector_stays_uncovered() {
    let package = issue(
        "static box Work { write_k(m: MapBox) { return m.set(\"k\", 1) } }
         static box Main { main() { return 0 } }",
    )
    .expect("non-get selector package");
    let contract = contract_for(&package, 1);
    assert!(
        contract.terminal_relation().is_none(),
        "non-get selector issues no relation"
    );
    assert!(
        matches!(
            contract
                .completion()
                .cleanup()
                .root_flow()
                .expect("root flow")
                .terminal_homes(),
            Err(HomePrefixUnavailableV1::ReturnValueNotCovered(_))
        ),
        "non-get selector stays uncovered"
    );
}
