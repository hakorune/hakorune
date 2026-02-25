#![allow(unexpected_cfgs)]
/*!
 * Host reverse-call API for plugins (Phase 12 / A-1)
 *
 * - Provides C ABI functions that plugins can call to operate on HostHandle (user/builtin boxes) via TLV.
 * - Minimal supported methods: InstanceBox.getField/setField, ArrayBox.get/set
 * - GC correctness: setField/Array.set triggers write barrier using current VM's runtime (TLS-bound during plugin calls).
 */

mod common;
mod host_array_ops;
mod host_box_ops;
mod host_string_ops;

// ===== TLS: current VM pointer during plugin invoke =====
// VM-legacy feature removed - provide stubs only
pub fn set_current_vm(_ptr: *mut ()) {}
pub fn clear_current_vm() {}

#[cfg_attr(all(not(test), feature = "c-abi-export"), no_mangle)]
pub extern "C" fn nyrt_host_call_name(
    handle: u64,
    method_ptr: *const u8,
    method_len: usize,
    args_ptr: *const u8,
    args_len: usize,
    out_ptr: *mut u8,
    out_len: *mut usize,
) -> i32 {
    // Resolve receiver
    let recv_arc = match crate::runtime::host_handles::get(handle) {
        Some(a) => a,
        None => return -1,
    };
    let method = unsafe {
        std::str::from_utf8(common::slice_from_raw(method_ptr, method_len)).unwrap_or("")
    }
    .to_string();
    let argv = common::parse_tlv_args(args_ptr, args_len);

    if let Some(code) =
        host_box_ops::dispatch_call_name(&recv_arc, &method, &argv, out_ptr, out_len)
    {
        return code;
    }
    if let Some(code) =
        host_array_ops::dispatch_call_name(&recv_arc, &method, &argv, out_ptr, out_len)
    {
        return code;
    }

    -10
}

// ---- by-slot variant (selector_id: u64) ----
#[cfg_attr(all(not(test), feature = "c-abi-export"), no_mangle)]
pub extern "C" fn nyrt_host_call_slot(
    handle: u64,
    selector_id: u64,
    args_ptr: *const u8,
    args_len: usize,
    out_ptr: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let recv_arc = match crate::runtime::host_handles::get(handle) {
        Some(a) => a,
        None => return -1,
    };
    let argv = common::parse_tlv_args(args_ptr, args_len);
    if let Some(code) =
        host_box_ops::dispatch_call_slot(&recv_arc, selector_id, &argv, out_ptr, out_len)
    {
        return code;
    }
    if let Some(code) =
        host_array_ops::dispatch_call_slot(&recv_arc, selector_id, &argv, out_ptr, out_len)
    {
        return code;
    }
    if let Some(code) =
        host_string_ops::dispatch_call_slot(&recv_arc, selector_id, &argv, out_ptr, out_len)
    {
        return code;
    }
    -10
}
