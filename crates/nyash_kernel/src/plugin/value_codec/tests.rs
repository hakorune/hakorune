use super::*;
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
fn store_string_box_from_source_prefers_borrowed_alias_for_string_handles() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("store-alias".to_string()));
    let value_h = handles::to_handle_arc(value) as i64;
    let source_obj = handles::get(value_h as u64).expect("source string handle");
    let boxed = store_string_box_from_source(value_h, Some(&source_obj), handles::drop_epoch());
    assert_eq!(box_to_runtime_i64(boxed), value_h);
}

#[test]
fn store_string_box_from_string_source_keeps_borrowed_alias_for_string_handles() {
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("store-alias-fast".to_string()));
    let value_h = handles::to_handle_arc(value) as i64;
    let source_obj = handles::get(value_h as u64).expect("source string handle");
    let boxed = store_string_box_from_string_source(value_h, &source_obj, handles::drop_epoch());
    assert_eq!(box_to_runtime_i64(boxed), value_h);
}

#[test]
fn store_string_box_from_source_keeps_immediate_contract_for_non_string_sources() {
    let value: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(91));
    let value_h = handles::to_handle_arc(value) as i64;
    let source_obj = handles::get(value_h as u64).expect("source integer handle");
    let boxed = store_string_box_from_source(value_h, Some(&source_obj), handles::drop_epoch());
    assert_eq!(box_to_runtime_i64(boxed), value_h);
}
