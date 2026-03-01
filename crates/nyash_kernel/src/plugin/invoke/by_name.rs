use crate::encode::nyrt_encode_arg;
use crate::plugin::invoke_core;
use crate::plugin::invoke::instance_fields::{handle_instance_get_field, handle_instance_set_field};

#[no_mangle]
pub extern "C" fn nyash_plugin_invoke_name_getattr_i64(argc: i64, a0: i64, a1: i64, a2: i64) -> i64 {
    nyash_plugin_invoke_name_common_i64("getattr", argc, a0, a1, a2)
}

#[no_mangle]
pub extern "C" fn nyash_plugin_invoke_name_call_i64(argc: i64, a0: i64, a1: i64, a2: i64) -> i64 {
    nyash_plugin_invoke_name_common_i64("call", argc, a0, a1, a2)
}

fn nyash_plugin_invoke_name_common_i64(method: &str, argc: i64, a0: i64, _a1: i64, _a2: i64) -> i64 {
    let Some(receiver) = invoke_core::resolve_named_receiver_for_handle(a0) else {
        return 0;
    };
    let Some(method_id) = invoke_core::resolve_method_id_for_named_receiver(&receiver, method)
    else {
        return 0;
    };

    let nargs = argc.saturating_sub(1).max(0) as usize;
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    // This shim has no explicit a1/a2 payload path; compat mode recovers
    // arguments from legacy VM slots (fail_fast=0 only).
    if nargs > 0
        && !invoke_core::encode_legacy_args_with_failfast_policy(&mut buf, 1, nargs)
    {
        return 0;
    }

    let Some((tag, sz, payload)) =
        invoke_core::plugin_invoke_call(
            receiver.invoke,
            receiver.real_type_id,
            method_id,
            receiver.instance_id,
            &buf,
        )
    else {
        return 0;
    };
    invoke_core::decode_entry_to_i64(tag, sz, payload.as_slice(), receiver.invoke).unwrap_or(0)
}

#[export_name = "nyash.plugin.invoke_by_name_i64"]
pub extern "C" fn nyash_plugin_invoke_by_name_i64(
    recv_handle: i64,
    method: *const i8,
    argc: i64,
    a1: i64,
    a2: i64,
) -> i64 {
    if method.is_null() {
        return 0;
    }
    if let Some(v) = crate::hako_forward_bridge::call_plugin_invoke_by_name(
        recv_handle,
        method,
        argc,
        a1,
        a2,
    ) {
        return v;
    }
    if !crate::hako_forward_bridge::rust_fallback_allowed() {
        return crate::hako_forward_bridge::hook_miss_return_zero("plugin.invoke_by_name");
    }
    let mname = unsafe { std::ffi::CStr::from_ptr(method) };
    let Ok(method_str) = mname.to_str() else {
        return 0;
    };

    if let Some(result) =
        crate::plugin::module_string_dispatch::try_dispatch(recv_handle, method_str, argc, a1, a2)
    {
        return result;
    }

    use nyash_rust::instance_v2::InstanceBox;
    if recv_handle > 0 {
        if let Some(obj) = nyash_rust::runtime::host_handles::get(recv_handle as u64) {
            if let Some(inst) = obj.as_any().downcast_ref::<InstanceBox>() {
                return match method_str {
                    "getField" => handle_instance_get_field(inst, a1),
                    "setField" => handle_instance_set_field(inst, a1, a2),
                    _ => 0,
                };
            }
        }
    }
    let Some(receiver) = invoke_core::resolve_named_receiver_for_handle(recv_handle) else {
        return 0;
    };
    let Some(method_id) = invoke_core::resolve_method_id_for_named_receiver(&receiver, method_str)
    else {
        return 0;
    };

    let nargs = argc.max(0) as usize;
    if nargs > 2 && nyash_rust::config::env::fail_fast() {
        return 0;
    }
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    nyrt_encode_arg(&mut buf, a1);
    if nargs >= 2 {
        nyrt_encode_arg(&mut buf, a2);
    }
    if nargs > 2 {
        if !invoke_core::encode_legacy_args_with_failfast_policy(&mut buf, 3, nargs) {
            return 0;
        }
    }
    let Some((tag, sz, payload)) =
        invoke_core::plugin_invoke_call(
            receiver.invoke,
            receiver.real_type_id,
            method_id,
            receiver.instance_id,
            &buf,
        )
    else {
        return 0;
    };
    invoke_core::decode_entry_to_i64(tag, sz, payload.as_slice(), receiver.invoke).unwrap_or(0)
}
