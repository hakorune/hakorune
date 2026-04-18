use super::*;
use crate::exports::string_view::StringViewBox;
use crate::test_support::with_env_var;
use nyash_rust::{
    box_trait::{IntegerBox, NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::sync::Arc;

#[test]
fn any_arg_to_index_prefers_boxed_integer_when_handle_points_integerbox() {
    let key: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(42));
    let key_h = handles::to_handle_arc(key) as i64;
    assert_eq!(any_arg_to_index(key_h), Some(42));
}

#[test]
fn any_arg_to_index_non_numeric_string_handle_falls_back_to_immediate() {
    let key: Arc<dyn NyashBox> = Arc::new(StringBox::new("not-an-int".to_string()));
    let key_h = handles::to_handle_arc(key) as i64;
    assert_eq!(any_arg_to_index(key_h), Some(key_h));
}

#[test]
fn any_arg_to_box_string_handle_preserves_handle_semantics_in_runtime_i64() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("alias-string".to_string()));
    let value_h = handles::to_handle_arc(value) as i64;

    let boxed = any_arg_to_box(value_h);
    let out = box_to_runtime_i64(boxed);
    assert!(out > 0);

    let out_obj = handles::get(out as u64).expect("runtime handle");
    let out_sb = out_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("runtime value should remain StringBox");
    assert_eq!(out_sb.value, "alias-string");
}

#[test]
fn borrowed_alias_caches_runtime_handle_for_unpublished_keep() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("cached-alias".to_string()));
    let alias = maybe_borrow_string_keep_with_epoch(
        SourceLifetimeKeep::string_box(value),
        0,
        handles::drop_epoch(),
    );

    let first = runtime_i64_from_box_ref(alias.as_ref());
    let second = runtime_i64_from_box_ref(alias.as_ref());

    assert!(first > 0);
    assert_eq!(first, second);

    let out_obj = handles::get(first as u64).expect("cached runtime handle");
    let out_sb = out_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("runtime value should remain StringBox");
    assert_eq!(out_sb.value, "cached-alias");
}

#[test]
fn any_arg_to_box_integer_handle_keeps_immediate_runtime_contract() {
    let value: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(77));
    let value_h = handles::to_handle_arc(value) as i64;
    let boxed = any_arg_to_box(value_h);
    assert_eq!(box_to_runtime_i64(boxed), 77);
}

#[test]
fn any_arg_to_box_array_fast_profile_reuses_live_source_handle_for_string() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("array-live".to_string()));
    let value_h = handles::to_handle_arc(value) as i64;
    let boxed = any_arg_to_box_with_profile(value_h, CodecProfile::ArrayFastBorrowString);
    assert_eq!(box_to_runtime_i64(boxed), value_h);
}

#[test]
fn any_arg_to_box_array_fast_profile_recreates_handle_when_source_was_dropped() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("array-recreate".to_string()));
    let value_h = handles::to_handle_arc(value) as i64;
    let boxed = any_arg_to_box_with_profile(value_h, CodecProfile::ArrayFastBorrowString);

    handles::drop_handle(value_h as u64);

    let out_h = box_to_runtime_i64(boxed);
    assert!(out_h > 0);
    let out_obj = handles::get(out_h as u64).expect("runtime handle after source drop");
    let out_sb = out_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("runtime value should remain StringBox");
    assert_eq!(out_sb.value, "array-recreate");
}

#[test]
fn any_arg_to_box_with_profile_array_fast_contract() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("profile".to_string()));
    let value_h = handles::to_handle_arc(value) as i64;

    let via_profile = any_arg_to_box_with_profile(value_h, CodecProfile::ArrayFastBorrowString);
    assert_eq!(box_to_runtime_i64(via_profile), value_h);
}

#[test]
fn any_arg_to_box_array_fast_profile_sets_borrowed_handle_metadata_for_string() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("borrowed".to_string()));
    let value_h = handles::to_handle_arc(value) as i64;

    let via_profile = any_arg_to_box_with_profile(value_h, CodecProfile::ArrayFastBorrowString);
    let borrowed = via_profile.borrowed_handle_source_fast();
    assert!(borrowed.is_some());
    assert_eq!(borrowed.map(|(h, _)| h), Some(value_h));
}

#[test]
fn any_arg_to_box_generic_profile_does_not_set_borrowed_handle_metadata() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("generic".to_string()));
    let value_h = handles::to_handle_arc(value) as i64;

    let via_generic = any_arg_to_box_with_profile(value_h, CodecProfile::Generic);
    assert_eq!(via_generic.borrowed_handle_source_fast(), None);
}

#[test]
fn any_arg_to_index_missing_handle_falls_back_to_immediate() {
    with_env_var("NYASH_HOST_HANDLE_ALLOC_POLICY", "none", || {
        let value: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(314));
        let h = handles::to_handle_arc(value) as i64;
        handles::drop_handle(h as u64);
        assert_eq!(any_arg_to_index(h), Some(h));
    });
}

#[test]
fn any_arg_to_box_with_profile_missing_handle_keeps_immediate_contract() {
    with_env_var("NYASH_HOST_HANDLE_ALLOC_POLICY", "none", || {
        let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("drop-me".to_string()));
        let h = handles::to_handle_arc(value) as i64;
        handles::drop_handle(h as u64);

        let via_generic = any_arg_to_box_with_profile(h, CodecProfile::Generic);
        let via_array_fast = any_arg_to_box_with_profile(h, CodecProfile::ArrayFastBorrowString);
        assert_eq!(box_to_runtime_i64(via_generic), h);
        assert_eq!(box_to_runtime_i64(via_array_fast), h);
    });
}

#[test]
fn materialize_owned_string_round_trips_as_live_string_handle() {
    let h = materialize_owned_string("owned-materialize".to_string());
    assert!(h > 0);
    let obj = handles::get(h as u64).expect("owned string handle");
    let sb = obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("owned string should remain StringBox");
    assert_eq!(sb.value, "owned-materialize");
}

#[test]
fn kernel_text_slot_freeze_publish_lifecycle_roundtrips() {
    let mut slot = KernelTextSlot::empty();
    assert_eq!(slot.state(), KernelTextSlotState::Empty);

    freeze_owned_string_into_slot(&mut slot, "owned-slot".to_string());
    assert_eq!(slot.state(), KernelTextSlotState::OwnedBytes);
    assert_eq!(
        with_kernel_text_slot_text(&slot, |text| text.to_string()).as_deref(),
        Some("owned-slot")
    );

    let h = publish_kernel_text_slot(&mut slot).expect("published handle");
    assert!(h > 0);
    assert_eq!(slot.state(), KernelTextSlotState::Published);
    assert!(with_kernel_text_slot_text(&slot, |text| text.len()).is_none());

    let obj = handles::get(h as u64).expect("published slot handle");
    let sb = obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("published slot should materialize as StringBox");
    assert_eq!(sb.value, "owned-slot");

    slot.clear();
    assert_eq!(slot.state(), KernelTextSlotState::Empty);
}

#[test]
fn kernel_text_slot_overwrite_replaces_owned_bytes() {
    let mut slot = KernelTextSlot::empty();
    freeze_owned_string_into_slot(&mut slot, "first-slot".to_string());
    freeze_owned_string_into_slot(&mut slot, "second-slot".to_string());

    assert_eq!(slot.state(), KernelTextSlotState::OwnedBytes);
    assert_eq!(
        with_kernel_text_slot_text(&slot, |text| text.to_string()).as_deref(),
        Some("second-slot")
    );
}

#[test]
fn kernel_text_slot_republish_returns_none_after_external_boundary() {
    let mut slot = KernelTextSlot::empty();
    freeze_owned_string_into_slot(&mut slot, "publish-once".to_string());

    let first = publish_kernel_text_slot(&mut slot).expect("first publish");
    assert!(first > 0);
    assert_eq!(slot.state(), KernelTextSlotState::Published);
    assert!(publish_kernel_text_slot(&mut slot).is_none());
    assert_eq!(slot.state(), KernelTextSlotState::Published);
}

#[test]
fn kernel_text_slot_objectize_boundary_consumes_owned_bytes_and_clears_slot() {
    let mut slot = KernelTextSlot::empty();
    freeze_owned_string_into_slot(&mut slot, "slot-objectize".to_string());

    let arc = objectize_kernel_text_slot_stable_box(&mut slot).expect("stable box");
    let sb = arc
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("objectized slot should materialize as StringBox");
    assert_eq!(sb.value, "slot-objectize");
    assert_eq!(slot.state(), KernelTextSlotState::Empty);
}

#[test]
fn with_array_store_str_source_non_string_handle_uses_other_object_contract() {
    let value: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(91));
    let value_h = handles::to_handle_arc(value) as i64;
    let source_kind = with_array_store_str_source(value_h, |source_kind, source| {
        assert!(matches!(source, ArrayStoreStrSource::OtherObject));
        source_kind
    });
    assert_eq!(source_kind, StringHandleSourceKind::OtherObject);
}

#[test]
fn with_array_store_str_source_missing_handle_uses_missing_contract() {
    with_env_var("NYASH_HOST_HANDLE_ALLOC_POLICY", "none", || {
        let value: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(12));
        let value_h = handles::to_handle_arc(value) as i64;
        handles::drop_handle(value_h as u64);
        let source_kind = with_array_store_str_source(value_h, |source_kind, source| {
            assert!(matches!(source, ArrayStoreStrSource::Missing));
            source_kind
        });
        assert_eq!(source_kind, StringHandleSourceKind::Missing);
    });
}

#[test]
fn retarget_borrowed_alias_from_verified_text_source_updates_slot() {
    let old_value: Arc<dyn NyashBox> = Arc::new(StringBox::new("retarget-old".to_string()));
    let new_value: Arc<dyn NyashBox> = Arc::new(StringBox::new("retarget-new".to_string()));
    let old_h = handles::to_handle_arc(old_value) as i64;
    let new_h = handles::to_handle_arc(new_value) as i64;
    let old_obj = handles::get(old_h as u64).expect("old string handle");
    let mut slot = store_string_box_from_source(old_h, Some(&old_obj), handles::drop_epoch());
    let source_text = with_array_store_str_source(new_h, |source_kind, source| {
        assert_eq!(source_kind, StringHandleSourceKind::StringLike);
        match source {
            ArrayStoreStrSource::StringLike(source_text) => source_text,
            _ => panic!("expected string-like source"),
        }
    });

    assert!(try_retarget_borrowed_string_slot_take_verified_text_source(
        &mut slot,
        new_h,
        source_text,
        handles::drop_epoch(),
    )
    .is_ok());
    assert!(
        slot.as_ref()
            .equals(&StringBox::new("retarget-new".to_string()))
            .value
    );
    assert_eq!(box_to_runtime_i64(slot), new_h);
}

#[test]
fn repeated_retarget_borrowed_alias_from_verified_text_source_keeps_latest_value() {
    let first: Arc<dyn NyashBox> = Arc::new(StringBox::new("retarget-0".to_string()));
    let first_h = handles::to_handle_arc(first) as i64;
    let first_obj = handles::get(first_h as u64).expect("first string handle");
    let mut slot = store_string_box_from_source(first_h, Some(&first_obj), handles::drop_epoch());

    for idx in 1..=40 {
        let next: Arc<dyn NyashBox> = Arc::new(StringBox::new(format!("retarget-{idx}")));
        let next_h = handles::to_handle_arc(next) as i64;
        let source_text = with_array_store_str_source(next_h, |source_kind, source| {
            assert_eq!(source_kind, StringHandleSourceKind::StringLike);
            match source {
                ArrayStoreStrSource::StringLike(source_text) => source_text,
                _ => panic!("expected string-like source"),
            }
        });

        assert!(try_retarget_borrowed_string_slot_take_verified_text_source(
            &mut slot,
            next_h,
            source_text,
            handles::drop_epoch(),
        )
        .is_ok());
        assert_eq!(box_to_runtime_i64(slot.as_ref().clone_box()), next_h);
        assert!(
            slot.as_ref()
                .equals(&StringBox::new(format!("retarget-{idx}")))
                .value
        );
    }
}

#[test]
fn verified_text_source_err_keeps_string_view_semantics() {
    let base: Arc<dyn NyashBox> = Arc::new(StringBox::new("view-keep-proof".to_string()));
    let base_h = handles::to_handle_arc(base.clone()) as i64;
    let view: Arc<dyn NyashBox> = Arc::new(StringViewBox::new(base_h, base, 5, 9));
    let view_h = handles::to_handle_arc(view) as i64;
    let source_text = with_array_store_str_source(view_h, |source_kind, source| {
        assert_eq!(source_kind, StringHandleSourceKind::StringLike);
        match source {
            ArrayStoreStrSource::StringLike(source_text) => source_text,
            _ => panic!("expected string-like source"),
        }
    });
    let mut slot: Box<dyn NyashBox> = Box::new(IntegerBox::new(7));

    let source_text = try_retarget_borrowed_string_slot_take_verified_text_source(
        &mut slot,
        view_h,
        source_text,
        handles::drop_epoch(),
    )
    .expect_err("non-borrowed slot should return source text");
    let boxed =
        store_string_box_from_verified_text_source(view_h, source_text, handles::drop_epoch());
    let sb = boxed
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("string view source should still materialize as StringBox");
    assert_eq!(sb.value, "keep");
}

#[test]
fn store_string_box_from_source_keep_owned_keeps_borrowed_alias_for_string_handles() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("store-owned-keep".to_string()));
    let value_h = handles::to_handle_arc(value) as i64;
    let source_obj = handles::get(value_h as u64).expect("source string handle");
    let boxed = store_string_box_from_source_keep_owned(
        value_h,
        SourceLifetimeKeep::string_box(source_obj),
        handles::drop_epoch(),
    );
    assert_eq!(box_to_runtime_i64(boxed), value_h);
}

#[test]
fn store_string_box_from_source_prefers_borrowed_alias_for_string_handles() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("store-alias".to_string()));
    let value_h = handles::to_handle_arc(value) as i64;
    let source_obj = handles::get(value_h as u64).expect("source string handle");
    let boxed = store_string_box_from_source(value_h, Some(&source_obj), handles::drop_epoch());
    assert!(boxed.as_any().downcast_ref::<BorrowedHandleBox>().is_some());
    assert!(
        boxed
            .as_ref()
            .equals(&StringBox::new("store-alias".to_string()))
            .value
    );
    assert!(
        !boxed
            .as_ref()
            .equals(&StringBox::new("store-alias-miss".to_string()))
            .value
    );
    assert_eq!(box_to_runtime_i64(boxed), value_h);
}

#[test]
fn store_string_box_from_source_keep_keeps_borrowed_alias_for_string_handles() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("store-alias-fast".to_string()));
    let value_h = handles::to_handle_arc(value) as i64;
    let source_obj = handles::get(value_h as u64).expect("source string handle");
    let keep = SourceLifetimeKeep::string_box(source_obj);
    let boxed = store_string_box_from_source_keep(value_h, &keep, handles::drop_epoch());
    assert_eq!(box_to_runtime_i64(boxed), value_h);
}

#[test]
fn borrowed_alias_equals_same_text_from_distinct_sources() {
    let left: Arc<dyn NyashBox> = Arc::new(StringBox::new("alias-same".to_string()));
    let right: Arc<dyn NyashBox> = Arc::new(StringBox::new("alias-same".to_string()));
    let left_h = handles::to_handle_arc(left) as i64;
    let right_h = handles::to_handle_arc(right) as i64;
    let left_obj = handles::get(left_h as u64).expect("left string handle");
    let right_obj = handles::get(right_h as u64).expect("right string handle");
    let left_alias = store_string_box_from_source(left_h, Some(&left_obj), handles::drop_epoch());
    let right_alias =
        store_string_box_from_source(right_h, Some(&right_obj), handles::drop_epoch());
    assert!(left_alias.as_ref().equals(right_alias.as_ref()).value);
}

#[test]
fn store_string_box_from_source_keep_materializes_string_view_sources() {
    let base: Arc<dyn NyashBox> = Arc::new(StringBox::new("view-materialize".to_string()));
    let base_h = handles::to_handle_arc(base.clone()) as i64;
    let view: Arc<dyn NyashBox> = Arc::new(StringViewBox::new(
        base_h,
        base,
        0,
        "view-materialize".len(),
    ));
    let keep = SourceLifetimeKeep::string_view(view);
    let boxed = store_string_box_from_source_keep(base_h, &keep, handles::drop_epoch());
    let sb = boxed
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("string view should materialize as StringBox");
    assert_eq!(sb.value, "view-materialize");
}

#[test]
fn store_string_box_from_source_keeps_immediate_contract_for_non_string_sources() {
    let value: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(91));
    let value_h = handles::to_handle_arc(value) as i64;
    let source_obj = handles::get(value_h as u64).expect("source integer handle");
    let boxed = store_string_box_from_source(value_h, Some(&source_obj), handles::drop_epoch());
    assert_eq!(box_to_runtime_i64(boxed), value_h);
}
