use super::*;
use crate::mir::builder::stmts::{CompletedLocalBindingV1, CompletedLocalStatementV1};
use crate::mir::{BasicBlockId, MirInstruction};

fn copied_local(initializer: ValueId, local: ValueId) -> CompletedLocalStatementV1 {
    CompletedLocalStatementV1::from_parts(
        local,
        vec![CompletedLocalBindingV1::new(0, initializer, local)
            .with_copy(Some((
                BasicBlockId::new(0),
                MirInstruction::Copy {
                    dst: local,
                    src: initializer,
                },
            )))
            .unwrap()],
    )
}

#[test]
fn source_alias_chain_retains_original_formal_and_invalidates_on_rebind() {
    let (mut state, _, formal) = materialized_fixture(
        "function caller(first: i64) { local a = first local b = a return b }",
    );
    let sites: Vec<_> = state.locals.keys().cloned().collect();
    assert_eq!(sites.len(), 2);
    state
        .record_completed_local(&sites[0], &copied_local(ValueId::new(77), ValueId::new(78)))
        .unwrap();
    state
        .record_completed_local(&sites[1], &copied_local(ValueId::new(78), ValueId::new(79)))
        .unwrap();
    let alias = state.locals[&sites[1]][0];
    let proof = state.values.provenance(&alias).unwrap();
    assert!(proof.matches(formal, ValueId::new(77)));
    assert!(!proof.matches(formal, ValueId::new(78)));
    let foreign = state.locals[&sites[0]][0];
    assert!(!proof.matches(foreign, ValueId::new(77)));
    let snapshot = state.source_values_snapshot();
    // Even publishing the same bits cannot retain an old borrowing proof.
    state.values.insert(alias, ValueId::new(79));
    assert!(state.values.provenance(&alias).is_none());
    state.restore_source_values(snapshot);
    assert!(state
        .values
        .provenance(&alias)
        .unwrap()
        .matches(formal, ValueId::new(77)));
}

#[test]
fn source_alias_does_not_prove_forwarding_from_unmatched_initializer() {
    let (mut state, _, _) =
        materialized_fixture("function caller(first: i64) { local a = first return a }");
    let site = state.locals.keys().next().unwrap().clone();
    state
        .record_completed_local(&site, &copied_local(ValueId::new(88), ValueId::new(89)))
        .unwrap();
    let alias = state.locals[&site][0];
    assert_eq!(state.values.get(&alias), Some(&ValueId::new(89)));
    assert!(state.values.provenance(&alias).is_none());
    let read_site = state
        .variables
        .iter()
        .find(|(_, binding)| **binding == alias)
        .unwrap()
        .0
        .clone();
    let read = state
        .take_exact_lexical_read(state.owner(), &read_site, alias)
        .unwrap();
    assert!(!read.proves_forwarded_entry(state.parameters[0], ValueId::new(77)));
}

#[test]
fn source_reuse_preserves_identity_but_contract_write_has_no_copy_proof() {
    for local in [ValueId::new(77), ValueId::new(78)] {
        let (mut state, _, formal) =
            materialized_fixture("function caller(first: i64) { local a = first return a }");
        let site = state.locals.keys().next().unwrap().clone();
        let completed = CompletedLocalStatementV1::from_parts(
            local,
            vec![CompletedLocalBindingV1::new(0, ValueId::new(77), local)],
        );
        state.record_completed_local(&site, &completed).unwrap();
        let alias = state.locals[&site][0];
        match state.values.provenance(&alias) {
            Some(proof) => {
                assert_eq!(local, ValueId::new(77));
                assert!(proof.matches(formal, ValueId::new(77)));
            }
            None => assert_eq!(local, ValueId::new(78)),
        }
    }
}

#[test]
fn completed_local_rejects_foreign_ordinal_before_publication() {
    let (mut state, _, _) =
        materialized_fixture("function caller(first: i64) { local a = first return a }");
    let site = state.locals.keys().next().unwrap().clone();
    let completed = CompletedLocalStatementV1::from_parts(
        ValueId::new(78),
        vec![CompletedLocalBindingV1::new(
            1,
            ValueId::new(77),
            ValueId::new(78),
        )],
    );
    let error = state.record_completed_local(&site, &completed).unwrap_err();
    assert!(error.contains("local-materialization-mismatch"));
    assert!(!state.materialized_locals.contains(&site));
    assert!(state.values.get(&state.locals[&site][0]).is_none());
}

#[test]
fn later_initializer_rebind_does_not_reject_an_earlier_local() {
    let (mut state, _, formal) =
        materialized_fixture("function caller(first: i64) { local a = first return a }");
    let site = state.locals.keys().next().unwrap().clone();
    // The earlier initializer read 77; a later initializer has published 88.
    state.values.insert(formal, ValueId::new(88));
    state
        .record_completed_local(&site, &copied_local(ValueId::new(77), ValueId::new(79)))
        .unwrap();
    let alias = state.locals[&site][0];
    assert_eq!(state.values.get(&alias), Some(&ValueId::new(79)));
    assert!(state.values.provenance(&alias).is_none());
    assert_eq!(state.values.get(&formal), Some(&ValueId::new(88)));
}

#[test]
fn exact_forwarded_copy_loan_checks_original_root_chain_and_read_value() {
    for length in 0..=2 {
        let (mut state, owner, formal) = materialized_fixture(
            "function caller(first: i64) { local a = first local b = a return b }",
        );
        let sites: Vec<_> = state.locals.keys().cloned().collect();
        state
            .record_completed_local(&sites[0], &copied_local(ValueId(77), ValueId(78)))
            .unwrap();
        state
            .record_completed_local(&sites[1], &copied_local(ValueId(78), ValueId(79)))
            .unwrap();
        let binding = if length == 0 {
            formal
        } else {
            state.locals[&sites[length - 1]][0]
        };
        let site = state
            .variables
            .iter()
            .find(|(_, b)| **b == binding)
            .unwrap()
            .0
            .clone();
        let read = state
            .take_exact_lexical_read(owner, &site, binding)
            .unwrap();
        let copies = read.loan_forwarded_copies(formal, ValueId(77)).unwrap();
        assert_eq!(copies.len(), length);
        assert!(read.loan_forwarded_copies(formal, ValueId(88)).is_err());
        if length != 0 {
            assert!(read.loan_forwarded_copies(binding, ValueId(77)).is_err());
        }
        state.values.insert(binding, ValueId(200));
        assert_eq!(
            read.loan_forwarded_copies(formal, ValueId(77)).unwrap(),
            copies
        );
        assert!(state.values.provenance(&binding).is_none());
    }
}

#[test]
fn reused_forwarded_local_lends_empty_copy_chain() {
    let (mut state, owner, formal) =
        materialized_fixture("function caller(first: i64) { local a = first return a }");
    let site = state.locals.keys().next().unwrap().clone();
    state
        .record_completed_local(
            &site,
            &CompletedLocalStatementV1::from_parts(
                ValueId(77),
                vec![CompletedLocalBindingV1::new(0, ValueId(77), ValueId(77))],
            ),
        )
        .unwrap();
    let alias = state.locals[&site][0];
    let site = state
        .variables
        .iter()
        .find(|(_, b)| **b == alias)
        .unwrap()
        .0
        .clone();
    let read = state.take_exact_lexical_read(owner, &site, alias).unwrap();
    assert!(read
        .loan_forwarded_copies(formal, ValueId(77))
        .unwrap()
        .is_empty());
}
