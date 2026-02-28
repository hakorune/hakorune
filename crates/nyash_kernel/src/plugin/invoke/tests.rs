use super::*;

#[test]
fn invoke_i64_invalid_receiver_returns_zero() {
    assert_eq!(nyash_plugin_invoke3_i64(0, 0, 0, 0, 0, 0), 0);
}

#[test]
fn invoke_f64_invalid_receiver_returns_zero() {
    assert_eq!(nyash_plugin_invoke3_f64(0, 0, 0, 0, 0, 0), 0.0);
}

#[test]
fn invoke_by_name_invalid_inputs_return_zero() {
    assert_eq!(nyash_plugin_invoke_name_getattr_i64(0, 0, 0, 0), 0);
    assert_eq!(nyash_plugin_invoke_name_call_i64(0, 0, 0, 0), 0);
    assert_eq!(nyash_plugin_invoke_by_name_i64(0, std::ptr::null(), 0, 0, 0), 0);
}

#[test]
fn invoke_tagged_invalid_receiver_returns_zero() {
    assert_eq!(
        nyash_plugin_invoke3_tagged_i64(0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 3),
        0
    );
    assert_eq!(
        nyash_plugin_invoke_tagged_v_i64(0, 0, 0, 0, std::ptr::null(), std::ptr::null()),
        0
    );
}
