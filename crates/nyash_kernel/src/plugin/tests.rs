use super::*;
use crate::c_string::cstring;
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
