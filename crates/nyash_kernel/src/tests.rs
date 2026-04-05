use super::*;
use crate::test_support::{with_env_var, with_env_vars};
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    boxes::file::FileBox,
    instance_v2::InstanceBox,
    runtime::{host_handles as handles, plugin_loader_v2::make_plugin_box_v2},
    value::NyashValue,
};
use std::ffi::CString;
use std::sync::Arc;

fn decode_string_like_handle(handle: i64) -> Option<String> {
    if handle <= 0 {
        return None;
    }
    let object = handles::get(handle as u64)?;
    if let Some(string_box) = object.as_any().downcast_ref::<StringBox>() {
        return Some(string_box.value.clone());
    }
    Some(object.to_string_box().value)
}

fn ensure_test_ring0() {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
}

fn with_filebox_from_handle<R>(handle: i64, f: impl FnOnce(&FileBox) -> R) -> R {
    let object = handles::get(handle as u64).expect("FileBox handle must exist");
    let filebox = object
        .as_any()
        .downcast_ref::<FileBox>()
        .expect("handle must be a FileBox");
    f(filebox)
}

fn string_handle(value: &str) -> i64 {
    handles::to_handle_arc(Arc::new(StringBox::new(value.to_string()))) as i64
}

fn instancebox_handle_with_field(field_name: &str, value: NyashValue) -> i64 {
    let instance = InstanceBox::from_declaration(
        "Point".to_string(),
        vec![field_name.to_string()],
        Default::default(),
    );
    instance
        .set_field_ng(field_name.to_string(), value)
        .expect("seed instance field");
    handles::to_handle_arc(Arc::new(instance)) as i64
}

fn dispatch_stage1_module(receiver_name: &str, method: &str, source_text: &str) -> i64 {
    let receiver_handle =
        handles::to_handle_arc(Arc::new(StringBox::new(receiver_name.to_string()))) as i64;
    let source_handle =
        handles::to_handle_arc(Arc::new(StringBox::new(source_text.to_string()))) as i64;
    crate::plugin::module_string_dispatch::try_dispatch(
        receiver_handle,
        method,
        1,
        source_handle,
        0,
    )
    .expect("stage1 direct dispatch")
}

unsafe extern "C" fn fake_i32(
    _t: u32,
    _m: u32,
    _i: u32,
    _a: *const u8,
    _al: usize,
    res: *mut u8,
    len: *mut usize,
) -> i32 {
    let mut buf = Vec::new();
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.push(2);
    buf.push(0);
    buf.extend_from_slice(&4u16.to_le_bytes());
    buf.extend_from_slice(&123i32.to_le_bytes());
    if res.is_null() || len.is_null() || unsafe { *len } < buf.len() {
        unsafe {
            if !len.is_null() {
                *len = buf.len();
            }
        }
        return -1;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(buf.as_ptr(), res, buf.len());
        *len = buf.len();
    }
    0
}

unsafe extern "C" fn fake_str(
    _t: u32,
    _m: u32,
    _i: u32,
    _a: *const u8,
    _al: usize,
    res: *mut u8,
    len: *mut usize,
) -> i32 {
    let s = b"hi";
    let mut buf = Vec::new();
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.extend_from_slice(&1u16.to_le_bytes());
    buf.push(7);
    buf.push(0);
    buf.extend_from_slice(&(s.len() as u16).to_le_bytes());
    buf.extend_from_slice(s);
    if res.is_null() || len.is_null() || unsafe { *len } < buf.len() {
        unsafe {
            if !len.is_null() {
                *len = buf.len();
            }
        }
        return -1;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(buf.as_ptr(), res, buf.len());
        *len = buf.len();
    }
    0
}

#[path = "tests/filebox.rs"]
mod filebox;
#[path = "tests/string.rs"]
mod string;

#[test]
fn future_spawn_instance_disable_rust_fallback_when_policy_is_off() {
    with_env_var("NYASH_VM_USE_FALLBACK", "0", || {
        crate::hako_forward_bridge::with_test_reset(|| {
            let result_handle = nyash_future_spawn_instance3_i64(1, 2, 3, 4);
            assert!(result_handle > 0);
            let result_text =
                decode_string_like_handle(result_handle).expect("hook-miss freeze string");
            assert!(result_text.contains("[freeze:contract][hako_forward/hook_miss]"));
            assert!(result_text.contains("route=future.spawn_instance3"));
        });
    });
}

#[test]
fn runtime_data_dispatch_array_push_get_index_zero() {
    use nyash_rust::boxes::array::ArrayBox;

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
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;

    assert_eq!(nyash_runtime_data_push_hh(array_handle, -11), 1);
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -11);

    // Legacy contract: negative index is immediate 0 (no handle allocation / no mutation).
    assert_eq!(nyash_runtime_data_get_hh(array_handle, -1), 0);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, -1), 0);
    assert_eq!(nyash_runtime_data_set_hhh(array_handle, -1, 99), 0);

    assert_eq!(nyash_runtime_data_get_hh(array_handle, 0), -11);
    assert_eq!(nyash_runtime_data_has_hh(array_handle, 1), 0);
}

#[test]
fn runtime_data_dispatch_array_set_index_contract() {
    use nyash_rust::boxes::array::ArrayBox;

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
    use nyash_rust::{box_trait::IntegerBox, boxes::array::ArrayBox};

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
    assert_eq!(nyash_runtime_data_has_hh(array_handle, key_one_handle), 1);
    assert_eq!(
        nyash_runtime_data_set_hhh(array_handle, key_one_handle, -30),
        1
    );
    assert_eq!(nyash_runtime_data_get_hh(array_handle, 1), -30);
}

#[test]
fn array_runtime_data_route_hh_contract_roundtrip() {
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;
    let key_zero = 0i64;

    let v1: Arc<dyn NyashBox> = Arc::new(StringBox::new("route-a".to_string()));
    let v1_h = handles::to_handle_arc(v1) as i64;

    assert_eq!(nyash_array_push_hh_alias(array_handle, v1_h), 1);
    assert_eq!(nyash_array_has_hh_alias(array_handle, key_zero), 1);

    let got_h = nyash_array_get_hh_alias(array_handle, key_zero);
    assert!(got_h > 0);
    let got_obj = handles::get(got_h as u64).expect("array.get_hh result handle");
    let got_str = got_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("array.get_hh result should be StringBox");
    assert_eq!(got_str.value, "route-a");

    let v2: Arc<dyn NyashBox> = Arc::new(StringBox::new("route-b".to_string()));
    let v2_h = handles::to_handle_arc(v2) as i64;
    assert_eq!(nyash_array_set_hhh_alias(array_handle, key_zero, v2_h), 1);

    let got2_h = nyash_array_get_hh_alias(array_handle, key_zero);
    assert!(got2_h > 0);
    let got2_obj = handles::get(got2_h as u64).expect("array.get_hh result handle 2");
    let got2_str = got2_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("array.get_hh result should be StringBox");
    assert_eq!(got2_str.value, "route-b");

    assert_eq!(nyash_array_get_hh_alias(array_handle, -1), 0);
    assert_eq!(nyash_array_has_hh_alias(array_handle, -1), 0);
    assert_eq!(nyash_array_set_hhh_alias(array_handle, -1, v1_h), 0);
}

#[test]
fn array_runtime_data_route_hi_contract_roundtrip() {
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;
    let key_zero = 0i64;

    let v1: Arc<dyn NyashBox> = Arc::new(StringBox::new("route-hi-a".to_string()));
    let v1_h = handles::to_handle_arc(v1) as i64;

    assert_eq!(nyash_array_push_hh_alias(array_handle, v1_h), 1);
    assert_eq!(nyash_array_has_hi_alias(array_handle, key_zero), 1);

    let got_h = nyash_array_get_hi_alias(array_handle, key_zero);
    assert!(got_h > 0);
    let got_obj = handles::get(got_h as u64).expect("array.get_hi result handle");
    let got_str = got_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("array.get_hi result should be StringBox");
    assert_eq!(got_str.value, "route-hi-a");

    let v2: Arc<dyn NyashBox> = Arc::new(StringBox::new("route-hi-b".to_string()));
    let v2_h = handles::to_handle_arc(v2) as i64;
    assert_eq!(nyash_array_set_hih_alias(array_handle, key_zero, v2_h), 1);

    let got2_h = nyash_array_get_hi_alias(array_handle, key_zero);
    assert!(got2_h > 0);
    let got2_obj = handles::get(got2_h as u64).expect("array.get_hi result handle 2");
    let got2_str = got2_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("array.get_hi result should be StringBox");
    assert_eq!(got2_str.value, "route-hi-b");

    assert_eq!(nyash_array_get_hi_alias(array_handle, -1), 0);
    assert_eq!(nyash_array_has_hi_alias(array_handle, -1), 0);
    assert_eq!(nyash_array_set_hih_alias(array_handle, -1, v1_h), 0);
}

#[test]
fn array_runtime_data_route_hii_contract_roundtrip() {
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;

    assert_eq!(nyash_array_set_hii_alias(array_handle, 0, 41), 1);
    assert_eq!(nyash_array_get_hi_alias(array_handle, 0), 41);
    assert_eq!(nyash_array_set_hii_alias(array_handle, 0, 42), 1);
    assert_eq!(nyash_array_get_hi_alias(array_handle, 0), 42);
    assert_eq!(nyash_array_set_hii_alias(array_handle, -1, 7), 0);
}

#[test]
fn array_get_hi_bool_returns_i64_contract() {
    use nyash_rust::{box_trait::BoolBox, boxes::array::ArrayBox};

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;
    let bool_obj: Arc<dyn NyashBox> = Arc::new(BoolBox::new(true));
    let bool_h = handles::to_handle_arc(bool_obj) as i64;

    assert_eq!(nyash_array_push_hh_alias(array_handle, bool_h), 1);
    assert_eq!(nyash_array_get_hi_alias(array_handle, 0), 1);
}

#[test]
fn array_set_h_legacy_return_code_contract() {
    use nyash_rust::boxes::array::ArrayBox;

    let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
    let array_handle = handles::to_handle_arc(array) as i64;

    // Legacy ABI: set_h always reports completion via return=0.
    assert_eq!(nyash_array_set_h(array_handle, 0, 41), 0);
    assert_eq!(nyash_array_get_h(array_handle, 0), 41);
    assert_eq!(nyash_array_length_h(array_handle), 1);

    // Gap write is ignored but still returns 0.
    assert_eq!(nyash_array_set_h(array_handle, 2, 99), 0);
    assert_eq!(nyash_array_length_h(array_handle), 1);

    // Exact-end write appends.
    assert_eq!(nyash_array_set_h(array_handle, 1, 42), 0);
    assert_eq!(nyash_array_get_h(array_handle, 1), 42);
    assert_eq!(nyash_array_length_h(array_handle), 2);
}

#[test]
fn runtime_v0_slice_invalid_handle_contract() {
    // V0 slice contract: invalid/negative handles are immediate zero path.
    assert_eq!(nyash_string_len_h(0), 0);
    assert_eq!(nyash_array_get_hi_alias(0, 0), 0);
    assert_eq!(nyash_array_get_hi_alias(-1, 0), 0);
    assert_eq!(nyash_array_set_hii_alias(0, 0, 1), 0);
    assert_eq!(nyash_array_set_hii_alias(-1, 0, 1), 0);
}

#[test]
fn runtime_data_dispatch_map_set_get_has() {
    use nyash_rust::{box_trait::IntegerBox, boxes::map_box::MapBox};

    let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
    let map_handle = handles::to_handle_arc(map) as i64;
    let key: Arc<dyn NyashBox> = Arc::new(StringBox::new("k".to_string()));
    let key_handle = handles::to_handle_arc(key) as i64;
    let expected = 42;
    let value: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(expected));
    let value_handle = handles::to_handle_arc(value) as i64;

    assert_eq!(
        nyash_runtime_data_set_hhh(map_handle, key_handle, value_handle),
        1
    );
    assert_eq!(nyash_runtime_data_has_hh(map_handle, key_handle), 1);

    let value = nyash_runtime_data_get_hh(map_handle, key_handle);
    if value == expected {
        return;
    }
    assert!(value > 0, "map get should return raw int or valid handle");
    let value_obj = handles::get(value as u64).expect("map get handle");
    let value_int = value_obj
        .as_any()
        .downcast_ref::<nyash_rust::box_trait::IntegerBox>()
        .expect("map value handle must wrap IntegerBox");
    assert_eq!(value_int.value, expected);
}

#[test]
fn runtime_data_dispatch_map_push_missing_key_contract() {
    use nyash_rust::{box_trait::IntegerBox, boxes::map_box::MapBox};

    let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
    let map_handle = handles::to_handle_arc(map) as i64;

    assert_eq!(nyash_runtime_data_push_hh(map_handle, -1), 0);

    let missing_key: Arc<dyn NyashBox> = Arc::new(StringBox::new("missing".to_string()));
    let missing_key_handle = handles::to_handle_arc(missing_key) as i64;
    assert_eq!(nyash_runtime_data_has_hh(map_handle, missing_key_handle), 0);
    assert_eq!(nyash_runtime_data_get_hh(map_handle, missing_key_handle), 0);

    let present_key: Arc<dyn NyashBox> = Arc::new(StringBox::new("present".to_string()));
    let present_key_handle = handles::to_handle_arc(present_key) as i64;
    let expected = 314;
    let value: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(expected));
    let value_handle = handles::to_handle_arc(value) as i64;

    assert_eq!(
        nyash_runtime_data_set_hhh(map_handle, present_key_handle, value_handle),
        1
    );
    assert_eq!(nyash_runtime_data_has_hh(map_handle, present_key_handle), 1);

    let got = nyash_runtime_data_get_hh(map_handle, present_key_handle);
    if got == expected {
        return;
    }
    assert!(got > 0, "map get should return raw int or valid handle");
    let got_obj = handles::get(got as u64).expect("map get handle");
    let got_int = got_obj
        .as_any()
        .downcast_ref::<IntegerBox>()
        .expect("map get handle must wrap IntegerBox");
    assert_eq!(got_int.value, expected);
}

#[test]
fn map_set_h_legacy_completion_code_and_mutation_roundtrip() {
    use nyash_rust::boxes::map_box::MapBox;

    let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
    let map_handle = handles::to_handle_arc(map) as i64;
    let key = -70001;

    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("legacy-set-h".to_string()));
    let value_handle = handles::to_handle_arc(value) as i64;

    assert_eq!(nyash_map_set_h(map_handle, key, value_handle), 0);
    assert_eq!(nyash_map_has_hh(map_handle, key), 1);

    let got_handle = nyash_map_get_hh(map_handle, key);
    assert!(got_handle > 0, "map get_hh must return a value handle");
    let got_obj = handles::get(got_handle as u64).expect("map get_hh handle");
    let got_value = got_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("map value must be StringBox");
    assert_eq!(got_value.value, "legacy-set-h");
}

#[test]
fn map_set_hh_legacy_completion_code_and_mutation_roundtrip() {
    use nyash_rust::boxes::map_box::MapBox;

    let map: Arc<dyn NyashBox> = Arc::new(MapBox::new());
    let map_handle = handles::to_handle_arc(map) as i64;

    let key: Arc<dyn NyashBox> = Arc::new(StringBox::new("legacy-key-hh".to_string()));
    let key_handle = handles::to_handle_arc(key) as i64;
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("legacy-value-hh".to_string()));
    let value_handle = handles::to_handle_arc(value) as i64;

    assert_eq!(nyash_map_set_hh(map_handle, key_handle, value_handle), 0);
    assert_eq!(nyash_map_has_hh(map_handle, key_handle), 1);

    let got_handle = nyash_map_get_hh(map_handle, key_handle);
    assert!(got_handle > 0, "map get_hh must return a value handle");
    let got_obj = handles::get(got_handle as u64).expect("map get_hh handle");
    let got_value = got_obj
        .as_any()
        .downcast_ref::<StringBox>()
        .expect("map value must be StringBox");
    assert_eq!(got_value.value, "legacy-value-hh");
}

#[test]
fn map_invalid_handle_fail_safe_contract() {
    assert_eq!(nyash_map_size_h(0), 0);
    assert_eq!(nyash_map_get_h(0, 1), 0);
    assert_eq!(nyash_map_get_hh(0, 1), 0);
    assert_eq!(nyash_map_has_h(0, 1), 0);
    assert_eq!(nyash_map_has_hh(0, 1), 0);
    assert_eq!(nyash_map_set_h(0, 1, 2), 0);
    assert_eq!(nyash_map_set_hh(0, 1, 2), 0);

    assert_eq!(nyash_map_size_h(-1), 0);
    assert_eq!(nyash_map_get_h(-1, 1), 0);
    assert_eq!(nyash_map_get_hh(-1, 1), 0);
    assert_eq!(nyash_map_has_h(-1, 1), 0);
    assert_eq!(nyash_map_has_hh(-1, 1), 0);
    assert_eq!(nyash_map_set_h(-1, 1, 2), 0);
    assert_eq!(nyash_map_set_hh(-1, 1, 2), 0);
}

#[test]
fn intarray_invalid_and_bounds_contract() {
    assert_eq!(nyash_intarray_len_h(0), 0);
    assert_eq!(nyash_intarray_get_hi(0, 0), 0);
    assert_eq!(nyash_intarray_set_hii(0, 0, 1), 1);

    let h = nyash_intarray_new_h(2);
    assert!(h > 0);
    assert_eq!(nyash_intarray_len_h(h), 2);
    assert_eq!(nyash_intarray_set_hii(h, 0, 10), 0);
    assert_eq!(nyash_intarray_set_hii(h, 1, 20), 0);
    assert_eq!(nyash_intarray_get_hi(h, 0), 10);
    assert_eq!(nyash_intarray_get_hi(h, 1), 20);

    assert_eq!(nyash_intarray_set_hii(h, -1, 99), 1);
    assert_eq!(nyash_intarray_set_hii(h, 2, 99), 1);
    assert_eq!(nyash_intarray_get_hi(h, -1), 0);
    assert_eq!(nyash_intarray_get_hi(h, 2), 0);
}

#[test]
fn console_null_and_invalid_handle_contract() {
    assert_eq!(nyash_console_log_export(std::ptr::null()), 0);
    assert_eq!(nyash_console_warn_export(std::ptr::null()), 0);
    assert_eq!(nyash_console_error_export(std::ptr::null()), 0);
    assert_eq!(nyash_debug_trace_export(std::ptr::null()), 0);

    assert_eq!(nyash_console_log_handle(0), 0);
    assert_eq!(nyash_console_warn_handle(0), 0);
    assert_eq!(nyash_console_error_handle(0), 0);
    assert_eq!(nyash_debug_trace_handle(0), 0);
}

#[test]
fn runtime_data_dispatch_invalid_receiver_returns_zero() {
    let receiver: Arc<dyn NyashBox> = Arc::new(StringBox::new("not-array-or-map".to_string()));
    let receiver_handle = handles::to_handle_arc(receiver) as i64;
    let key: Arc<dyn NyashBox> = Arc::new(StringBox::new("k".to_string()));
    let key_handle = handles::to_handle_arc(key) as i64;
    let value: Arc<dyn NyashBox> = Arc::new(StringBox::new("v".to_string()));
    let value_handle = handles::to_handle_arc(value) as i64;

    assert_eq!(nyash_runtime_data_get_hh(receiver_handle, key_handle), 0);
    assert_eq!(
        nyash_runtime_data_set_hhh(receiver_handle, key_handle, value_handle),
        0
    );
    assert_eq!(nyash_runtime_data_has_hh(receiver_handle, key_handle), 0);
    assert_eq!(nyash_runtime_data_push_hh(receiver_handle, value_handle), 0);
}

#[test]
fn handle_lifecycle_retain_h_zero_is_noop() {
    assert_eq!(nyrt_handle_retain_h(0), 0);
}

#[test]
fn handle_lifecycle_retain_release_h_roundtrip() {
    let object: Arc<dyn NyashBox> = Arc::new(StringBox::new("phase29y".to_string()));
    let handle = handles::to_handle_arc(object) as i64;
    assert!(handle > 0);

    let retained = nyrt_handle_retain_h(handle);
    assert!(retained > 0);
    assert!(
        handles::get(retained as u64).is_some(),
        "retained handle must be present"
    );
    assert!(
        handles::get(handle as u64).is_some(),
        "original handle must remain present"
    );

    nyrt_handle_release_h(retained);
    assert!(
        handles::get(retained as u64).is_none(),
        "released handle must be dropped"
    );
    assert!(
        handles::get(handle as u64).is_some(),
        "release_h must not drop unrelated handles"
    );
}

#[test]
fn handle_lifecycle_legacy_release_alias_drops_handle() {
    let object: Arc<dyn NyashBox> = Arc::new(StringBox::new("legacy".to_string()));
    let handle = handles::to_handle_arc(object) as i64;
    assert!(handles::get(handle as u64).is_some());

    ny_release_strong(handle);
    assert!(handles::get(handle as u64).is_none());
}

#[test]
fn handle_abi_borrowed_owned_conformance() {
    // SSOT: args borrowed / return owned
    // - borrowed arg remains valid during callee execution
    // - return value must be independently releasable by caller
    let object: Arc<dyn NyashBox> = Arc::new(StringBox::new("borrowed-owned".to_string()));
    let borrowed_handle = handles::to_handle_arc(object) as i64;
    assert!(borrowed_handle > 0);
    assert!(handles::get(borrowed_handle as u64).is_some());

    // Simulate "callee returns borrowed arg as owned" by retain on escape.
    let returned_owned_handle = nyrt_handle_retain_h(borrowed_handle);
    assert!(returned_owned_handle > 0);
    assert_ne!(
        borrowed_handle, returned_owned_handle,
        "retain_h must allocate an independent caller-owned handle"
    );
    assert!(handles::get(returned_owned_handle as u64).is_some());

    // Caller may release borrowed argument path (e.g., overwrite old binding)
    // while returned-owned value must stay alive.
    nyrt_handle_release_h(borrowed_handle);
    assert!(
        handles::get(borrowed_handle as u64).is_none(),
        "borrowed handle should be released"
    );
    assert!(
        handles::get(returned_owned_handle as u64).is_some(),
        "owned return must survive borrowed release"
    );

    // Caller is responsible for releasing owned return.
    nyrt_handle_release_h(returned_owned_handle);
    assert!(
        handles::get(returned_owned_handle as u64).is_none(),
        "owned return handle should be released by caller"
    );
}

#[test]
fn handle_abi_borrowed_owned_multi_escape_conformance() {
    // Matrix case: returned-owned handle can be re-borrowed/re-escaped and released independently.
    let object: Arc<dyn NyashBox> = Arc::new(StringBox::new("borrowed-owned-chain".to_string()));
    let borrowed_handle = handles::to_handle_arc(object) as i64;
    assert!(borrowed_handle > 0);

    let owned_handle_1 = nyrt_handle_retain_h(borrowed_handle);
    let owned_handle_2 = nyrt_handle_retain_h(owned_handle_1);
    assert!(owned_handle_1 > 0);
    assert!(owned_handle_2 > 0);
    assert_ne!(borrowed_handle, owned_handle_1);
    assert_ne!(owned_handle_1, owned_handle_2);

    nyrt_handle_release_h(borrowed_handle);
    assert!(handles::get(borrowed_handle as u64).is_none());
    assert!(handles::get(owned_handle_1 as u64).is_some());
    assert!(handles::get(owned_handle_2 as u64).is_some());

    nyrt_handle_release_h(owned_handle_1);
    assert!(handles::get(owned_handle_1 as u64).is_none());
    assert!(handles::get(owned_handle_2 as u64).is_some());

    nyrt_handle_release_h(owned_handle_2);
    assert!(handles::get(owned_handle_2 as u64).is_none());
}

#[test]
fn handle_abi_borrowed_owned_invalid_handles_are_noop() {
    // Matrix case: invalid handle inputs never produce owned escapes and must not drop unrelated handles.
    let anchor: Arc<dyn NyashBox> = Arc::new(StringBox::new("borrowed-owned-anchor".to_string()));
    let anchor_handle = handles::to_handle_arc(anchor) as i64;
    assert!(anchor_handle > 0);
    assert!(handles::get(anchor_handle as u64).is_some());

    assert_eq!(nyrt_handle_retain_h(-1), 0, "negative handle must be no-op");
    assert_eq!(nyrt_handle_retain_h(0), 0, "zero handle must be no-op");
    assert_eq!(
        nyrt_handle_retain_h(i64::MAX),
        0,
        "unknown handle must not escape"
    );

    nyrt_handle_release_h(-1);
    nyrt_handle_release_h(0);
    nyrt_handle_release_h(i64::MAX);

    assert!(
        handles::get(anchor_handle as u64).is_some(),
        "invalid release path must not drop unrelated handles"
    );

    nyrt_handle_release_h(anchor_handle);
    assert!(handles::get(anchor_handle as u64).is_none());
}
