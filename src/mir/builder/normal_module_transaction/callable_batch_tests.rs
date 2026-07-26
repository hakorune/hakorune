use crate::mir::MirBuilder;

use super::callable_draft_prefix::completed_for_main_physical;
use super::*;

#[test]
fn batch_seals_helpers_source_main_and_physical_entry_in_one_schema() {
    let mut builder = MirBuilder::new();
    let prefix = builder
        .prepare_normal_helper_draft_prefix_v1(
            completed_for_main_physical(&["beta", "alpha"]).into_tx0_handoff(),
        )
        .expect("helper prefix");
    let prepared = builder
        .prepare_normal_callable_main_physical_v1(prefix)
        .expect("Main plus physical drafts")
        .seal_normal_callable_batch_v1()
        .expect("schema batch");

    let rows = prepared.schema().rows();
    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0].symbol(), "main/0");
    assert_eq!(rows[1].symbol(), "alpha/1");
    assert_eq!(rows[2].symbol(), "beta/1");
    assert_eq!(rows[3].symbol(), "main");
    assert_eq!(prepared.drafts().helpers().drafts().len(), 2);
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
}

#[test]
fn schema_rejection_retains_prepared_drafts_and_builder_reuses() {
    let mut builder = MirBuilder::new();
    let prefix = builder
        .prepare_normal_helper_draft_prefix_v1(
            completed_for_main_physical(&["helper"]).into_tx0_handoff(),
        )
        .expect("helper prefix");
    let prepared = builder
        .prepare_normal_callable_main_physical_v1(prefix)
        .expect("Main plus physical drafts");
    let rejected = reject_normal_callable_batch_for_test(prepared);
    assert!(matches!(
        rejected.error(),
        NormalCallableBatchErrorV1::Schema(
            NormalModuleTransactionSchemaErrorV1::DuplicateSourceMain
        )
    ));
    assert_eq!(rejected.retained_helper_count(), 1);
    rejected.discard();

    let next = builder
        .prepare_normal_helper_draft_prefix_v1(
            completed_for_main_physical(&["later"]).into_tx0_handoff(),
        )
        .expect("later helper prefix");
    builder
        .prepare_normal_callable_main_physical_v1(next)
        .expect("later Main plus physical drafts")
        .seal_normal_callable_batch_v1()
        .expect("later schema batch");
}
