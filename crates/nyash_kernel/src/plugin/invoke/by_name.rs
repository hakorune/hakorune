use crate::encode::nyrt_encode_arg;
use crate::plugin::invoke_core;

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

fn nyash_plugin_invoke_name_common_i64(
    method: &str,
    argc: i64,
    a0: i64,
    _a1: i64,
    _a2: i64,
) -> i64 {
    use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;

    let mut instance_id: u32 = 0;
    let mut type_id: u32 = 0;
    let mut box_type: Option<String> = None;
    let mut invoke: Option<invoke_core::InvokeFn> = None;
    if a0 > 0 {
        if let Some(obj) = nyash_rust::runtime::host_handles::get(a0 as u64) {
            if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
                instance_id = p.instance_id();
                type_id = p.inner.type_id;
                box_type = Some(p.box_type.clone());
                invoke = Some(p.inner.invoke_fn);
            }
        }
    }
    let Some(invoke) = invoke else {
        return 0;
    };
    let box_type = box_type.unwrap_or_default();
    let mh = if let Ok(host) =
        nyash_rust::runtime::plugin_loader_unified::get_global_plugin_host().read()
    {
        host.resolve_method(&box_type, method)
    } else {
        return 0;
    };
    let method_id = match mh {
        Ok(h) => h.method_id,
        Err(_) => return 0,
    } as u32;

    let nargs = argc.saturating_sub(1).max(0) as usize;
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    if nargs > 0 {
        if nyash_rust::config::env::fail_fast() {
            return 0;
        }
        // This shim has no explicit a1/a2 payload path; compat mode recovers
        // arguments from legacy VM slots.
        invoke_core::encode_legacy_vm_args_range(&mut buf, 1, nargs);
    }

    let Some((tag, sz, payload)) =
        invoke_core::plugin_invoke_call(invoke, type_id, method_id, instance_id, &buf)
    else {
        return 0;
    };
    invoke_core::decode_entry_to_i64(tag, sz, payload.as_slice(), invoke).unwrap_or(0)
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
    if !crate::hako_forward_bridge::rust_fallback_allowed() {
        return crate::hako_forward_bridge::hook_miss_freeze_handle("plugin.invoke_by_name");
    }
    let mname = unsafe { std::ffi::CStr::from_ptr(method) };
    let Ok(method_str) = mname.to_str() else {
        return 0;
    };

    let trace = std::env::var("HAKO_STAGE1_MODULE_DISPATCH_TRACE")
        .ok()
        .as_deref()
        == Some("1")
        || std::env::var("STAGE1_CLI_DEBUG").ok().as_deref() == Some("1");

    // Probe compiled-stage1 compat quarantine before the main host/plugin route.
    if let Some(result) =
        crate::plugin::module_string_dispatch::try_dispatch(recv_handle, method_str, argc, a1, a2)
    {
        if trace {
            eprintln!(
                "[stage1/plugin_invoke] compat_quarantine result method={} argc={} result_handle={}",
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
        if nyash_rust::config::env::fail_fast() {
            return 0;
        }
        invoke_core::encode_legacy_vm_args_range(&mut buf, 3, nargs);
    }
    invoke_core::invoke_named_receiver_to_i64(&receiver, method_id, &buf).unwrap_or(0)
}
