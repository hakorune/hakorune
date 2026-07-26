use crate::mir::MirBuilder;

use super::callable_draft_prefix::completed_for_main_physical;
use super::*;

#[test]
fn commit_consumes_the_preflighted_batch_into_an_opaque_candidate() {
    let mut builder = MirBuilder::new();
    let prefix = builder
        .prepare_normal_helper_draft_prefix_v1(
            completed_for_main_physical(&["helper"]).into_tx0_handoff(),
        )
        .unwrap();
    let candidate = builder
        .prepare_normal_callable_main_physical_v1(prefix)
        .unwrap()
        .seal_normal_callable_batch_v1()
        .unwrap()
        .prepare_normal_callable_commit_v1()
        .unwrap()
        .commit();
    assert_eq!(candidate.module().functions.len(), 3);
    assert!(candidate.module().get_function("helper/1").is_some());
    assert!(candidate.module().get_function("main/0").is_some());
    assert!(candidate.module().get_function("main").is_some());
}

#[test]
fn precommit_rejection_keeps_publication_zero_and_builder_reusable() {
    let mut builder = MirBuilder::new();
    let prefix = builder
        .prepare_normal_helper_draft_prefix_v1(
            completed_for_main_physical(&["helper"]).into_tx0_handoff(),
        )
        .unwrap();
    let batch = builder
        .prepare_normal_callable_main_physical_v1(prefix)
        .unwrap()
        .seal_normal_callable_batch_v1()
        .unwrap();
    let rejected = reject_normal_callable_commit_for_test(batch);
    assert!(matches!(
        rejected.error(),
        NormalCallableCommitErrorV1::Correspondence
    ));
    rejected.discard();

    let prefix = builder
        .prepare_normal_helper_draft_prefix_v1(
            completed_for_main_physical(&["later"]).into_tx0_handoff(),
        )
        .unwrap();
    let candidate = builder
        .prepare_normal_callable_main_physical_v1(prefix)
        .unwrap()
        .seal_normal_callable_batch_v1()
        .unwrap()
        .prepare_normal_callable_commit_v1()
        .unwrap()
        .commit();
    assert!(candidate.module().get_function("later/1").is_some());
}
