use crate::mir::MirBuilder;

use super::callable_draft_prefix::completed_for_main_physical;
use super::*;

#[test]
fn main_and_physical_extend_the_completed_helper_prefix_without_a_batch() {
    let mut builder = MirBuilder::new();
    let prefix = builder
        .prepare_normal_helper_draft_prefix_v1(
            completed_for_main_physical(&["helper"]).into_tx0_handoff(),
        )
        .expect("helper prefix");
    let prepared = builder
        .prepare_normal_callable_main_physical_v1(prefix)
        .expect("Main and physical drafts");

    assert_eq!(prepared.helpers().drafts().len(), 1);
    assert_eq!(prepared.source().draft().signature.name, "main/0");
    assert_eq!(prepared.physical().draft().signature.name, "main");
    assert_eq!(prepared.relation().entry().physical_symbol(), "main");
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
}

#[test]
fn injected_main_stages_retain_helpers_and_restore_builder_for_later_success() {
    let mut builder = MirBuilder::new();
    for stage in [
        NormalCallableMainPhysicalTestStageV1::SourceLowering,
        NormalCallableMainPhysicalTestStageV1::PhysicalThunk,
    ] {
        let prefix = builder
            .prepare_normal_helper_draft_prefix_v1(
                completed_for_main_physical(&["helper"]).into_tx0_handoff(),
            )
            .expect("helper prefix");
        let rejected =
            reject_normal_callable_main_physical_at_stage_for_test(&mut builder, prefix, stage);
        assert_eq!(rejected.retained_helper_count(), 1);
        assert!(builder.function_state.current_function.is_none());
        assert!(builder.function_state.current_block.is_none());
        rejected.discard();

        let next = builder
            .prepare_normal_helper_draft_prefix_v1(
                completed_for_main_physical(&["later"]).into_tx0_handoff(),
            )
            .expect("later helper prefix");
        let prepared = builder
            .prepare_normal_callable_main_physical_v1(next)
            .expect("later Main and physical drafts");
        assert_eq!(prepared.helpers().drafts()[0].key().name(), "later");
    }
}
