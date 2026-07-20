//! FINALIZE0-VERIFY-SPLIT0-I0 function-finalizer lifecycle integration tests.
//!
//! These witnesses cover only the selected all-build function-finalizer seam.
//! Module completion and loop diagnostics intentionally retain their legacy
//! lifecycle helper until their later post-mutation boundaries are selected.

use crate::mir::builder::MirBuilder;
use crate::mir::{MirType, ValueId};
use hakorune_mir_core::MirValueKind;

fn stale_value() -> ValueId {
    ValueId::new(904)
}

fn builder_with_draft(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn insert_stale_lifecycle_rows(builder: &mut MirBuilder, value: ValueId) {
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, MirType::Integer);
    builder
        .function_state
        .type_ctx
        .value_kinds
        .insert(value, MirValueKind::Temporary);
    builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .insert(value, "FinalizeLifecycleOrigin".to_string());
}

#[test]
fn function_finalizer_normalizes_unretained_stale_rows_before_metadata_snapshot() {
    let mut builder = builder_with_draft("finalize-lifecycle-stale/0");
    let stale = stale_value();
    insert_stale_lifecycle_rows(&mut builder, stale);

    let draft = builder.finalize_function_draft(false).unwrap();

    assert!(!draft.metadata.value_types.contains_key(&stale));
    assert!(!builder
        .function_state
        .type_ctx
        .value_types
        .contains_key(&stale));
    assert!(!builder
        .function_state
        .type_ctx
        .value_kinds
        .contains_key(&stale));
    assert!(!builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .contains_key(&stale));
    assert!(builder.function_state.current_function.is_none());
}

#[test]
fn function_finalizer_rejects_retained_stale_rows_without_normalizer_commit() {
    let mut builder = builder_with_draft("finalize-lifecycle-retained/0");
    let stale = stale_value();
    insert_stale_lifecycle_rows(&mut builder, stale);
    builder
        .function_state
        .pin_slot_names
        .insert(stale, "retained-stale-pin".to_string());

    let error = builder.finalize_function_draft(false).unwrap_err();

    assert!(error.contains("[value_lifecycle/transient_stale_row_retained]"));
    assert!(builder.function_state.current_function.is_some());
    assert_eq!(
        builder.function_state.type_ctx.value_types.get(&stale),
        Some(&MirType::Integer)
    );
    assert_eq!(
        builder.function_state.type_ctx.value_kinds.get(&stale),
        Some(&MirValueKind::Temporary)
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&stale),
        Some(&"FinalizeLifecycleOrigin".to_string())
    );
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .metadata
        .value_types
        .is_empty());
}

#[test]
fn function_finalizer_keeps_no_stale_draft_completion_behavior() {
    let mut builder = builder_with_draft("finalize-lifecycle-clean/0");

    let draft = builder.finalize_function_draft(false).unwrap();

    assert_eq!(draft.signature.name, "finalize-lifecycle-clean/0");
    assert!(builder.function_state.current_function.is_none());
}
