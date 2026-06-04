use crate::c_string::c_string_text;
use crate::plugin::invoke_core;
use std::sync::OnceLock;

#[no_mangle]
pub extern "C" fn nyash_plugin_invoke_name_getattr_i64(
    argc: i64,
    a0: i64,
    a1: i64,
    a2: i64,
) -> i64 {
    nyash_plugin_invoke_name_common_i64("getattr", argc, a0, a1, a2)
}

#[no_mangle]
pub extern "C" fn nyash_plugin_invoke_name_call_i64(argc: i64, a0: i64, a1: i64, a2: i64) -> i64 {
    nyash_plugin_invoke_name_common_i64("call", argc, a0, a1, a2)
}

fn nyash_plugin_invoke_name_common_i64(method: &str, argc: i64, a0: i64, a1: i64, a2: i64) -> i64 {
    let Some((receiver, method_id)) = invoke_core::resolve_named_method_for_handle(a0, method)
    else {
        return 0;
    };
    let Some(buf) = invoke_core::build_two_payload_tlv(argc.saturating_sub(1), a1, a2) else {
        return 0;
    };
    invoke_core::invoke_receiver_to_i64(
        receiver.invoke,
        receiver.real_type_id,
        method_id,
        receiver.instance_id,
        &buf,
    )
    .unwrap_or(0)
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
    if !nyash_rust::config::env::vm_compat_fallback_allowed() {
        return crate::hako_forward_bridge::hook_miss_freeze_handle("plugin.invoke_by_name");
    }
    let Some(method_str) = c_string_text(method) else {
        return 0;
    };

    static TRACE_ENABLED: OnceLock<bool> = OnceLock::new();
    let trace = crate::env_flags::flag_any_on_cached(
        &TRACE_ENABLED,
        &["HAKO_STAGE1_MODULE_DISPATCH_TRACE", "STAGE1_CLI_DEBUG"],
    );

    // Probe compiled-stage1 dispatch before the main host/plugin route.
    if let Some(result) =
        crate::plugin::module_string_dispatch::try_dispatch(recv_handle, method_str, argc, a1, a2)
    {
        if trace {
            eprintln!(
                "[stage1/plugin_invoke] route result method={} argc={} result_handle={}",
                method_str, argc, result
            );
        }
        return result;
    }

    let Some((receiver, method_id)) =
        invoke_core::resolve_named_method_for_handle(recv_handle, method_str)
    else {
        return 0;
    };

    let Some(buf) = invoke_core::build_two_payload_tlv(argc, a1, a2) else {
        return 0;
    };
    invoke_core::invoke_receiver_to_i64(
        receiver.invoke,
        receiver.real_type_id,
        method_id,
        receiver.instance_id,
        &buf,
    )
    .unwrap_or(0)
}
