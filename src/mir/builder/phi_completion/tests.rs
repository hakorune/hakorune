use std::collections::BTreeMap;

use super::{CompletedPhiV1, PhiCompletionPreparationErrorV1, PhiDraftV1, PreparedPhiCompletionV1};
use crate::mir::builder::phi_type_publication::{
    PhiConcreteTypeConflictV1, PreparedPhiTypePublicationV1,
};
use crate::mir::{BasicBlockId, MirType, ValueId};

fn block(id: u32) -> BasicBlockId {
    BasicBlockId::new(id)
}

fn value(id: u32) -> ValueId {
    ValueId::new(id)
}

fn draft() -> PhiDraftV1 {
    PhiDraftV1::new(block(9), value(99), None)
}

fn integer_types(values: &[u32]) -> BTreeMap<ValueId, MirType> {
    values
        .iter()
        .map(|id| (value(*id), MirType::Integer))
        .collect()
}

#[derive(Default)]
struct FakeCompletionPortV1 {
    instruction_commits: usize,
    type_commits: usize,
    fail_instruction: bool,
}

impl FakeCompletionPortV1 {
    fn commit(
        &mut self,
        prepared: PreparedPhiCompletionV1,
    ) -> Result<CompletedPhiV1, &'static str> {
        if self.fail_instruction {
            return Err("candidate instruction failure");
        }
        self.instruction_commits += 1;
        let completed = prepared.after_instruction_commit();
        self.type_commits += 1;
        Ok(completed)
    }

    fn commit_batch(
        &mut self,
        prepared: Vec<PreparedPhiCompletionV1>,
    ) -> Result<Vec<CompletedPhiV1>, &'static str> {
        if self.fail_instruction {
            return Err("candidate batch instruction failure");
        }
        self.instruction_commits += 1;
        let completed = prepared
            .into_iter()
            .map(PreparedPhiCompletionV1::after_instruction_commit)
            .collect::<Vec<_>>();
        self.type_commits += completed.len();
        Ok(completed)
    }
}

#[test]
fn completion_prepares_normalized_rows_and_commits_type_only_after_instruction_success() {
    let prepared = draft()
        .prepare(
            &[block(1), block(2)],
            &[(block(2), value(2)), (block(1), value(1))],
            &integer_types(&[1, 2]),
            None,
        )
        .unwrap();

    assert_eq!(prepared.draft().block(), block(9));
    assert_eq!(prepared.draft().dst(), value(99));
    assert_eq!(
        prepared.logical_inputs(),
        &[(block(1), value(1)), (block(2), value(2))]
    );
    assert_eq!(
        prepared.prepared_type(),
        &PreparedPhiTypePublicationV1::Publish(MirType::Integer)
    );

    let mut port = FakeCompletionPortV1::default();
    let completed = port.commit(prepared).unwrap();
    assert_eq!(port.instruction_commits, 1);
    assert_eq!(port.type_commits, 1);
    assert_eq!(
        completed.prepared_type(),
        &PreparedPhiTypePublicationV1::Publish(MirType::Integer)
    );
}

#[test]
fn raw_final_patch_and_batch_requests_have_identical_preparation() {
    let expected = [block(1), block(2)];
    let inputs = [(block(2), value(2)), (block(1), value(1))];
    let types = integer_types(&[1, 2]);

    let raw = draft().prepare(&expected, &inputs, &types, None).unwrap();
    let final_insert = draft().prepare(&expected, &inputs, &types, None).unwrap();
    let patch = draft().prepare(&expected, &inputs, &types, None).unwrap();
    let batch = draft().prepare(&expected, &inputs, &types, None).unwrap();

    assert_eq!(raw, final_insert);
    assert_eq!(raw, patch);
    assert_eq!(raw, batch);
}

#[test]
fn provisional_draft_has_no_prepared_or_completed_type_fact() {
    let draft = PhiDraftV1::new(block(3), value(7), Some(MirType::Integer));

    assert_eq!(draft.block(), block(3));
    assert_eq!(draft.dst(), value(7));
}

#[test]
fn duplicate_missing_and_phantom_rows_fail_before_a_completion_product() {
    let types = integer_types(&[1, 2, 3]);

    assert_eq!(
        draft()
            .prepare(
                &[block(1), block(2)],
                &[(block(1), value(1)), (block(1), value(2))],
                &types,
                None,
            )
            .unwrap_err(),
        PhiCompletionPreparationErrorV1::DuplicateIncomingPredecessor {
            predecessor: block(1),
        }
    );
    assert_eq!(
        draft()
            .prepare(&[block(1), block(2)], &[(block(1), value(1))], &types, None)
            .unwrap_err(),
        PhiCompletionPreparationErrorV1::MissingIncomingPredecessor {
            predecessor: block(2),
        }
    );
    assert_eq!(
        draft()
            .prepare(
                &[block(1)],
                &[(block(1), value(1)), (block(3), value(3))],
                &types,
                None,
            )
            .unwrap_err(),
        PhiCompletionPreparationErrorV1::PhantomIncomingPredecessor {
            predecessor: block(3),
        }
    );
}

#[test]
fn concrete_type_conflict_is_preserved_from_the_existing_decision_owner() {
    let error = draft()
        .prepare(
            &[block(1)],
            &[(block(1), value(1))],
            &integer_types(&[1]),
            Some(&MirType::String),
        )
        .unwrap_err();

    assert!(matches!(
        error,
        PhiCompletionPreparationErrorV1::ConcreteTypeConflict(PhiConcreteTypeConflictV1 { .. })
    ));
}

#[test]
fn failed_single_candidate_commit_never_commits_a_type() {
    let prepared = draft()
        .prepare(
            &[block(1)],
            &[(block(1), value(1))],
            &integer_types(&[1]),
            None,
        )
        .unwrap();
    let mut port = FakeCompletionPortV1 {
        fail_instruction: true,
        ..Default::default()
    };

    assert_eq!(port.commit(prepared), Err("candidate instruction failure"));
    assert_eq!(port.instruction_commits, 0);
    assert_eq!(port.type_commits, 0);
}

#[test]
fn failed_batch_candidate_commit_keeps_all_type_commits_at_zero() {
    let first = draft()
        .prepare(
            &[block(1)],
            &[(block(1), value(1))],
            &integer_types(&[1]),
            None,
        )
        .unwrap();
    let second = PhiDraftV1::new(block(9), value(100), None)
        .prepare(
            &[block(2)],
            &[(block(2), value(2))],
            &integer_types(&[2]),
            None,
        )
        .unwrap();
    let mut port = FakeCompletionPortV1 {
        fail_instruction: true,
        ..Default::default()
    };

    assert_eq!(
        port.commit_batch(vec![first, second]),
        Err("candidate batch instruction failure")
    );
    assert_eq!(port.instruction_commits, 0);
    assert_eq!(port.type_commits, 0);
}

#[test]
fn failed_batch_item_preparation_never_reaches_live_commit() {
    let first = draft()
        .prepare(
            &[block(1)],
            &[(block(1), value(1))],
            &integer_types(&[1]),
            None,
        )
        .unwrap();
    let second = PhiDraftV1::new(block(9), value(100), None).prepare(
        &[block(2)],
        &[(block(3), value(3))],
        &integer_types(&[3]),
        None,
    );
    let port = FakeCompletionPortV1::default();

    assert_eq!(
        second.unwrap_err(),
        PhiCompletionPreparationErrorV1::PhantomIncomingPredecessor {
            predecessor: block(3),
        }
    );
    assert_eq!(port.instruction_commits, 0);
    assert_eq!(port.type_commits, 0);
    drop(first);
}

#[test]
fn failed_patch_preparation_leaves_the_existing_draft_incomplete() {
    let draft = PhiDraftV1::new(block(3), value(7), None);
    let error = draft
        .prepare(
            &[block(1)],
            &[(block(2), value(2))],
            &integer_types(&[2]),
            None,
        )
        .unwrap_err();

    assert_eq!(
        error,
        PhiCompletionPreparationErrorV1::PhantomIncomingPredecessor {
            predecessor: block(2),
        }
    );
    assert_eq!(draft.block(), block(3));
    assert_eq!(draft.dst(), value(7));
}
