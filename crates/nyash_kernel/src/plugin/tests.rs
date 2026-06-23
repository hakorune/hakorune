use super::*;
use crate::c_string::cstring;
use crate::exports::typed_object::{
    nyash_object_new_typed_hi, nyash_object_register_typed_layout_hi, nyash_object_type_id_h,
};
use nyash_rust::box_trait::{NyashBox, StringBox};
use nyash_rust::boxes::array::ArrayBox;
use nyash_rust::runtime::host_handles as handles;
use std::sync::Arc;

fn new_array_handle() -> i64 {
    let arr: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    handles::to_handle_arc(arr) as i64
}

fn new_string_handle(value: &str) -> i64 {
    let string_box: Arc<dyn NyashBox> = Arc::new(StringBox::new(value.to_string()));
    handles::to_handle_arc(string_box) as i64
}

#[test]
fn array_compat_push_and_get_roundtrip() {
    let handle = new_array_handle();
    assert_eq!(nyash_array_push_h(handle, 7), 1);
    assert_eq!(nyash_array_get_hi_alias(handle, 0), 7);
}

#[test]
fn array_slot_load_preserves_typed_object_carrier_bits() {
    let handle = new_array_handle();
    let type_id = 710_250_001;
    assert_eq!(nyash_object_register_typed_layout_hi(type_id, 1), 1);
    let object = nyash_object_new_typed_hi(type_id, 1);
    assert!(object < 0);
    assert_eq!(nyash_object_type_id_h(object), type_id);

    assert_eq!(nyash_array_set_hih_alias(handle, 0, object), 1);
    let loaded = nyash_array_get_hi_alias(handle, 0);
    assert_eq!(loaded, object);
    assert_eq!(nyash_object_type_id_h(loaded), type_id);
}

#[test]
fn array_slot_load_keeps_negative_carrier_bits_without_sign_inference() {
    let handle = new_array_handle();
    let type_id = 710_250_002;
    assert_eq!(nyash_object_register_typed_layout_hi(type_id, 1), 1);
    let object = nyash_object_new_typed_hi(type_id, 1);
    assert!(object < 0);

    assert_eq!(nyash_array_set_hih_alias(handle, 0, object), 1);
    assert_eq!(nyash_array_set_hii_alias(handle, 1, object), 1);

    let object_carrier = nyash_array_get_hi_alias(handle, 0);
    let scalar_carrier = nyash_array_get_hi_alias(handle, 1);
    assert_eq!(object_carrier, scalar_carrier);
    assert_eq!(nyash_object_type_id_h(object_carrier), type_id);
}

#[test]
fn array_slot_load_materializes_borrowed_string_after_source_drop() {
    let _guard = crate::test_support::handle_registry_test_lock();
    let handle = new_array_handle();
    let value_handle = new_string_handle("borrowed-array-slot");

    assert_eq!(nyash_array_set_hih_alias(handle, 0, value_handle), 1);
    handles::drop_handle(value_handle as u64);

    let loaded = nyash_array_get_hi_alias(handle, 0);
    assert!(loaded > 0);
    let object = handles::get(loaded as u64).expect("array loaded string handle");
    let string_box = object
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("array load should materialize StringBox");
    assert_eq!(string_box.value, "borrowed-array-slot");
}

#[test]
fn array_string_len_reads_text_slot_directly() {
    let handle = new_array_handle();
    let string_handle = new_string_handle("length");
    assert_eq!(nyash_array_set_his_alias(handle, 0, string_handle), 1);
    assert_eq!(nyash_array_string_len_hi_alias(handle, 0), 6);
}

#[test]
fn array_string_len_sum_region_reads_text_slots() {
    let handle = new_array_handle();
    let first = new_string_handle("ab");
    let second = new_string_handle("cde");

    assert_eq!(nyash_array_set_his_alias(handle, 0, first), 1);
    assert_eq!(nyash_array_set_his_alias(handle, 1, second), 1);
    assert_eq!(
        nyash_array_string_len_sum_region_hiii_alias(handle, 4, 2, 10),
        20
    );
}

#[test]
fn array_string_len_sum_region_reads_only_touched_row_domain() {
    let handle = new_array_handle();
    let first = new_string_handle("ab");

    assert_eq!(nyash_array_push_hh_alias(handle, first), 1);
    assert_eq!(
        nyash_array_string_len_sum_region_hiii_alias(handle, 1, 4, 10),
        12
    );
}

#[test]
fn array_text_indexof_const_found_count_region_counts_hits() {
    let handle = new_array_handle();
    let first = new_string_handle("line-a");
    let second = new_string_handle("none");
    let needle = cstring("line");

    assert_eq!(nyash_array_push_hh_alias(handle, first), 1);
    assert_eq!(nyash_array_push_hh_alias(handle, second), 2);
    assert_eq!(
        hako_array_text_indexof_const_found_count_region_alias(handle, 5, 2, needle.as_ptr(), 4,),
        3
    );
}

#[test]
fn array_string_indexof_suffix_store_len_sum_region_updates_and_sums_hits() {
    let handle = new_array_handle();
    let first = new_string_handle("line-a");
    let second = new_string_handle("none");
    let needle = cstring("line");
    let suffix = cstring("ln");

    assert_eq!(nyash_array_set_his_alias(handle, 0, first), 1);
    assert_eq!(nyash_array_set_his_alias(handle, 1, second), 1);
    assert_eq!(
        nyash_array_string_indexof_suffix_store_len_sum_region_hiisisi_alias(
            handle,
            5,
            2,
            needle.as_ptr(),
            4,
            suffix.as_ptr(),
            2,
        ),
        30
    );
    assert_eq!(nyash_array_string_len_hi_alias(handle, 0), 12);
    assert_eq!(nyash_array_string_len_hi_alias(handle, 1), 4);
}

#[test]
fn array_string_suffix_store_updates_text_lane() {
    let handle = new_array_handle();
    let seed_h = new_string_handle("line-seed");
    let suffix = cstring("xy");

    assert_eq!(nyash_array_set_his_alias(handle, 0, seed_h), 1);
    assert_eq!(
        crate::nyash_array_string_suffix_store_his_alias(handle, 0, suffix.as_ptr(),),
        1
    );
    assert_eq!(nyash_array_string_len_hi_alias(handle, 0), 11);
    assert_eq!(with_array_box(handle, |arr| arr.len()), Some(1));
}
