// C‑ABI shim (PoC). These functions are no‑ops for 20.36/20.37.
// Kept tiny and isolated. Linkage names match include/nyrt.h.
//
// Export policy:
// - `no_mangle` is enabled only with `c-core` feature.
// - Default builds keep this shim internal to avoid accidental production linkage.

#[cfg_attr(all(not(test), feature = "c-core"), no_mangle)]
pub extern "C" fn nyrt_init() -> i32 {
    0
}

#[cfg_attr(all(not(test), feature = "c-core"), no_mangle)]
pub extern "C" fn nyrt_teardown() {}

#[cfg_attr(all(not(test), feature = "c-core"), no_mangle)]
pub extern "C" fn nyrt_load_mir_json(_json_text: *const ::std::os::raw::c_char) -> u64 {
    1
}

#[cfg_attr(all(not(test), feature = "c-core"), no_mangle)]
pub extern "C" fn nyrt_exec_main(_module_handle: u64) -> i32 {
    0
}

#[cfg_attr(all(not(test), feature = "c-core"), no_mangle)]
pub extern "C" fn nyrt_verify_mir_json(json_text: *const ::std::os::raw::c_char) -> i32 {
    if json_text.is_null() {
        return 1;
    }
    0
}

#[cfg_attr(all(not(test), feature = "c-core"), no_mangle)]
pub extern "C" fn nyrt_safety_check_mir_json(json_text: *const ::std::os::raw::c_char) -> i32 {
    if json_text.is_null() {
        return 1;
    }
    0
}

#[cfg_attr(all(not(test), feature = "c-core"), no_mangle)]
pub extern "C" fn nyrt_hostcall(
    _name: *const ::std::os::raw::c_char,
    _method: *const ::std::os::raw::c_char,
    _payload_json: *const ::std::os::raw::c_char,
    _out_buf: *mut ::std::os::raw::c_char,
    _out_buf_len: u32,
) -> i32 {
    0
}

#[cfg_attr(all(not(test), feature = "c-core"), no_mangle)]
pub extern "C" fn nyrt_handle_retain_h(handle: i64) -> i64 {
    handle
}

#[cfg_attr(all(not(test), feature = "c-core"), no_mangle)]
pub extern "C" fn nyrt_handle_release_h(_handle: i64) {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn load_and_exec_noop_returns_zero() {
        // init/teardown are no-ops but should stay callable
        assert_eq!(nyrt_init(), 0);

        let json = CString::new("{}").expect("CString");
        let handle = nyrt_load_mir_json(json.as_ptr());
        assert_eq!(handle, 1);

        assert_eq!(nyrt_exec_main(handle), 0);
        assert_eq!(nyrt_verify_mir_json(json.as_ptr()), 0);
        assert_eq!(nyrt_safety_check_mir_json(json.as_ptr()), 0);

        // ensure teardown does not panic even when called after exec
        nyrt_teardown();
    }

    #[test]
    fn verify_and_safety_reject_null_json_pointer() {
        assert_ne!(nyrt_verify_mir_json(std::ptr::null()), 0);
        assert_ne!(nyrt_safety_check_mir_json(std::ptr::null()), 0);
    }

    #[test]
    fn lifecycle_handle_zero_contract_is_stable() {
        assert_eq!(nyrt_handle_retain_h(0), 0);
        nyrt_handle_release_h(0);
    }

    #[test]
    fn lifecycle_handle_non_zero_passthrough() {
        assert_eq!(nyrt_handle_retain_h(42), 42);
        nyrt_handle_release_h(42);
    }
}
