use crate::encode::nyrt_encode_arg_or_legacy;
use crate::plugin::invoke_core;

#[no_mangle]
pub extern "C" fn nyash_plugin_invoke3_i64(
    type_id: i64,
    method_id: i64,
    argc: i64,
    a0: i64,
    a1: i64,
    a2: i64,
) -> i64 {
    let Some(recv) = invoke_core::resolve_receiver_for_a0(a0) else {
        return 0;
    };
    let nargs = argc.max(0) as usize;
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    if nargs >= 1 {
        nyrt_encode_arg_or_legacy(&mut buf, a1, 1);
    }
    if nargs >= 2 {
        nyrt_encode_arg_or_legacy(&mut buf, a2, 2);
    }
    if nargs > 2 {
        invoke_core::encode_legacy_vm_args_range(&mut buf, 3, nargs);
    }
    let Some((tag, sz, payload)) = invoke_core::plugin_invoke_call(
        recv.invoke,
        type_id as u32,
        method_id as u32,
        recv.instance_id,
        &buf,
    ) else {
        return 0;
    };
    invoke_core::decode_entry_to_i64(tag, sz, payload.as_slice(), recv.invoke).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn nyash_plugin_invoke3_f64(
    type_id: i64,
    method_id: i64,
    argc: i64,
    a0: i64,
    a1: i64,
    a2: i64,
) -> f64 {
    let Some(recv) = invoke_core::resolve_receiver_for_a0(a0) else {
        return 0.0;
    };
    let nargs = argc.max(0) as usize;
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    if nargs >= 1 {
        nyrt_encode_arg_or_legacy(&mut buf, a1, 1);
    }
    if nargs >= 2 {
        nyrt_encode_arg_or_legacy(&mut buf, a2, 2);
    }
    if nargs > 2 {
        invoke_core::encode_legacy_vm_args_range(&mut buf, 3, nargs);
    }
    let Some((tag, sz, payload)) = invoke_core::plugin_invoke_call(
        recv.invoke,
        type_id as u32,
        method_id as u32,
        recv.instance_id,
        &buf,
    ) else {
        return 0.0;
    };
    invoke_core::decode_entry_to_f64(tag, sz, payload.as_slice()).unwrap_or(0.0)
}
