use super::super::*;
use crate::test_support::with_env_var;
use nyash_rust::{
    box_trait::{IntegerBox, NyashBox, StringBox},
    boxes::array::ArrayBox,
    runtime::host_handles as handles,
};
use std::sync::Arc;

#[test]
fn runtime_data_dispatch_array_push_get_index_zero() {
    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("abc".to_string()));
    let value_handle = handles::to_handle_arc(value) as i64;

    let new_len = nyash_runtime_data_push_hh(array_handle, value_handle);
    assert_eq!(new_len, 1);

    let got_handle = nyash_runtime_data_get_hh(array_handle, 0);
    assert!(got_handle > 0, "array get should return a valid handle");
    let got_obj = handles::get(got_handle as u64).expect("array get handle");
    let got_str = got_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("array get value must be StringBox");
    assert_eq!(got_str.value, "abc");
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 0), 1);
}

#[test]
fn runtime_data_dispatch_array_negative_index_contract() {
    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;

    assert_eq!(nyash_runtime_data_push_hh(array_handle, -11), 1);
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -11);

    // Compat contract: negative index is immediate 0 (no handle allocation / no mutation).
    assert_eq!(nyash_runtime_data_get_hh(array_handle, -1), 0);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, -1), 0);
    assert_eq!(nyash_runtime_data_set_hhh(array_handle, -1, 99), 0);

    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -11);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 1), 0);
}

#[test]
fn runtime_data_dispatch_array_set_index_contract() {
    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;

    assert_eq!(nyash_array_length_h(array_handle), 0);

    // idx == len appends and reports success.
    assert_eq!(nyash_runtime_data_set_hhh(array_handle, 0, -10), 1);
    assert_eq!(nyash_array_length_h(array_handle), 1);
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -10);

    // idx < len overwrites and reports success.
    assert_eq!(nyash_runtime_data_set_hhh(array_handle, 0, -11), 1);
    assert_eq!(nyash_array_length_h(array_handle), 1);
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -11);

    // has_hh contract: idx in-range => 1, idx == len => 0.
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 0), 1);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 1), 0);

    // idx > len rejects write and keeps length unchanged.
    assert_eq!(nyash_runtime_data_set_hhh(array_handle, 2, -99), 0);
    assert_eq!(nyash_array_length_h(array_handle), 1);
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -11);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 1), 0);
}

#[test]
fn runtime_data_dispatch_array_positive_immediate_index_contract() {
    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;

    assert_eq!(nyash_runtime_data_push_hh(array_handle, -10), 1);
    assert_eq!(nyash_runtime_data_push_hh(array_handle, -20), 2);

    // Positive immediate indices must not be blocked by unrelated live handles.
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 1), -20);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 1), 1);

    // Integer-handle key remains supported.
    let key_one: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(1));
    let key_one_handle = handles::to_handle_arc(key_one) as i64;
    assert_eq!(nyash_runtime_data_get_hh(array_handle, key_one_handle), -20);

    // Inserting a live handle in the registry must not disturb existing positive immediate lookups.
    let _guard = with_env_var("NYASH_VM_USE_FALLBACK", "1", || ());
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 1), -20);
}
