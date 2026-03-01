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
    use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;
    let mut instance_id: u32 = 0;
    let mut type_id: u32 = 0;
    let mut box_type: Option<String> = None;
    let mut invoke: Option<invoke_core::InvokeFn> = None;
    if recv_handle > 0 {
        if let Some(obj) = nyash_rust::runtime::host_handles::get(recv_handle as u64) {
            if let Some(inst) = obj.as_any().downcast_ref::<InstanceBox>() {
                return match method_str {
                    "getField" => handle_instance_get_field(inst, a1),
                    "setField" => handle_instance_set_field(inst, a1, a2),
                    _ => 0,
                };
            }
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
        host.resolve_method(&box_type, method_str)
    } else {
        return 0;
    };
    let method_id = match mh {
        Ok(h) => h.method_id,
        Err(_) => return 0,
    } as u32;

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
    let Some((tag, sz, payload)) =
        invoke_core::plugin_invoke_call(invoke, type_id, method_id, instance_id, &buf)
    else {
        return 0;
    };

    match tag {
        3 => {
            if payload.len() == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload.as_slice());
                return i64::from_le_bytes(b);
            }
        }
        1 => {
            return if nyash_rust::runtime::plugin_ffi_common::decode::bool(payload.as_slice())
                .unwrap_or(false)
            {
                1
            } else {
                0
            };
        }
        8 => {
            if payload.len() == 8 {
                let mut t = [0u8; 4];
                t.copy_from_slice(&payload[0..4]);
                let mut i = [0u8; 4];
                i.copy_from_slice(&payload[4..8]);
                let r_type = u32::from_le_bytes(t);
                let r_inst = u32::from_le_bytes(i);
                let Some((box_type_name, invoke_ptr, _)) =
                    invoke_core::resolve_invoke_route_for_type(r_type, invoke)
                else {
                    return 0;
                };
                let pb = nyash_rust::runtime::plugin_loader_v2::make_plugin_box_v2(
                    box_type_name,
                    r_type,
                    r_inst,
                    invoke_ptr,
                );
                let arc: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> =
                    std::sync::Arc::new(pb);
                let h = nyash_rust::runtime::host_handles::to_handle_arc(arc) as u64;
                return h as i64;
            }
        }
        5 => {
            if std::env::var("NYASH_JIT_NATIVE_F64").ok().as_deref() == Some("1")
                && payload.len() == 8
            {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload.as_slice());
                let f = f64::from_le_bytes(b);
                return f as i64;
            }
        }
        _ => {}
    }
    let _ = sz;
    0
}
