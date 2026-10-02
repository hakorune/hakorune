//! Real borrowed source joined to the existing local materialization owner.
use super::*;
use crate::mir::builder::stmts::{CompletedLocalBindingV1, CompletedLocalStatementV1};
use crate::mir::{BasicBlockId, MirInstruction};

fn state_with_body(body: &str) -> CallableSemanticLoweringState {
    let mut state = state_from_package(package_with_body("p", "true", body), 1);
    state.install_entry_values(&entry(51, 72)).unwrap();
    state
}

fn completed(source: u32, destination: u32, copied: bool) -> CompletedLocalStatementV1 {
    let row = CompletedLocalBindingV1::new(0, ValueId(source), ValueId(destination))
        .with_copy(copied.then_some((
            BasicBlockId(0),
            MirInstruction::Copy {
                src: ValueId(source),
                dst: ValueId(destination),
            },
        )))
        .unwrap();
    CompletedLocalStatementV1::from_parts(ValueId(destination), vec![row])
}

#[test]
fn borrowed_alias_materialization_retains_unused_chain_on_original_entry() {
    let mut state = state_with_body("local a = p local b = a return 0");
    let sites: Vec<_> = state.locals.keys().cloned().collect();
    state
        .record_completed_local(&sites[0], &completed(72, 73, true))
        .unwrap();
    state
        .record_completed_local(&sites[1], &completed(73, 74, true))
        .unwrap();
    let ledger = state.ordinary_new_claim_ledger.as_ref().unwrap();
    // The state may drop all local read slots; the original entry still owns
    // the physical observations, including aliases that never become actuals.
    state.values.clear();
    let mut loans = Vec::new();
    ledger
        .with_borrowed_ordinary_alias_copies_v1(
            state.owner,
            |site, declaration, binding, formal, value, copies| {
                assert_eq!(site.owner(), state.owner);
                assert!(matches!(declaration, SourceBindingSiteV1::Local { .. }));
                assert_eq!(formal, state.parameters[0]);
                loans.push((binding, value, copies.to_vec()));
                Ok(())
            },
        )
        .unwrap();
    assert_eq!(loans.len(), 2);
    assert_eq!(loans[0].1, ValueId(73));
    assert_eq!(loans[0].2.len(), 1);
    assert_eq!(loans[1].1, ValueId(74));
    assert_eq!(loans[1].2.len(), 2);
    assert_eq!(loans[0].2[0], loans[1].2[0]);
}

#[test]
fn borrowed_alias_materialization_retains_original_zero_copy_proof() {
    let mut state = state_with_body("local alias = p return 0");
    let site = state.locals.keys().next().unwrap().clone();
    let relation = state.local_initializer(&site, 0).unwrap().clone();
    let source = state.parameters[0];
    let row = completed(72, 72, false);
    let proof = state
        .values
        .local_provenance(source, &row.bindings()[0])
        .unwrap();
    // Retention accepts the original zero-Copy proof. This does not claim
    // dynamic-origin local completion accepts ValueId reuse: that existing
    // owner separately rejects LocalBindingMismatch and remains unchanged.
    state
        .ordinary_new_claim_ledger
        .as_ref()
        .unwrap()
        .record_borrowed_ordinary_alias_v1(
            state.owner,
            &relation,
            Some(source),
            ValueId(72),
            Some(&proof),
        )
        .unwrap();
    let mut visited = 0;
    state
        .ordinary_new_claim_ledger
        .as_ref()
        .unwrap()
        .with_borrowed_ordinary_alias_copies_v1(
            state.owner,
            |_, _, binding, formal, value, copies| {
                assert_ne!(binding, formal);
                assert_eq!(value, ValueId(72));
                assert!(copies.is_empty());
                visited += 1;
                Ok(())
            },
        )
        .unwrap();
    assert_eq!(visited, 1);
}

#[test]
fn borrowed_alias_materialization_refuses_missing_and_foreign_original_proof() {
    for mutation in 0..4 {
        let mut state = state_with_body("local alias = p return 0");
        let site = state.locals.keys().next().unwrap().clone();
        let relation = state.local_initializer(&site, 0).unwrap().clone();
        let source = state.parameters[0];
        let row = completed(72, 73, true);
        let proof = state
            .values
            .local_provenance(source, &row.bindings()[0])
            .unwrap();
        let ledger = state.ordinary_new_claim_ledger.as_ref().unwrap();
        let error = ledger.record_borrowed_ordinary_alias_v1(
            state.owner,
            &relation,
            if mutation == 1 {
                Some(relation.binding())
            } else {
                Some(source)
            },
            if mutation == 2 {
                ValueId(74)
            } else {
                ValueId(73)
            },
            if mutation == 0 { None } else { Some(&proof) },
        );
        if mutation == 3 {
            error.unwrap();
            assert!(state
                .record_completed_local(&site, &row)
                .unwrap_err()
                .contains("duplicate-materialization"));
        } else {
            assert!(error.is_err(), "mutation {mutation}");
            let mut seen = 0;
            ledger
                .with_borrowed_ordinary_alias_copies_v1(state.owner, |_, _, _, _, _, _| {
                    seen += 1;
                    Ok(())
                })
                .unwrap();
            assert_eq!(seen, 0, "rejection must not publish an alias row");
        }
    }
}

#[test]
fn borrowed_alias_materialization_refuses_unproved_value_materialization() {
    let mut state = state_with_body("local alias = p return 0");
    let site = state.locals.keys().next().unwrap().clone();
    // This completion reports a value not produced by the original initializer.
    let error = state
        .record_completed_local(&site, &completed(999, 73, true))
        .unwrap_err();
    assert!(error.contains("proof-missing"), "{error}");
}
