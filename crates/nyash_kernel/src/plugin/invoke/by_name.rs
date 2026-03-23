use crate::encode::nyrt_encode_arg;
use crate::plugin::invoke::instance_fields::{
    handle_instance_get_field, handle_instance_set_field,
};
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

fn encode_box_handle(boxed: Box<dyn nyash_rust::box_trait::NyashBox>) -> i64 {
    let arc: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> = boxed.into();
    nyash_rust::runtime::host_handles::to_handle_arc(arc) as i64
}

fn decode_handle_to_string_like(handle: i64) -> Option<String> {
    if handle <= 0 {
        return None;
    }
    let object = nyash_rust::runtime::host_handles::get(handle as u64)?;
    if let Some(string_box) = object
        .as_any()
        .downcast_ref::<nyash_rust::box_trait::StringBox>()
    {
        return Some(string_box.value.clone());
    }
    Some(object.to_string_box().value)
}

fn decode_handle_to_box_or_integer(handle: i64) -> Box<dyn nyash_rust::box_trait::NyashBox> {
    if handle > 0 {
        if let Some(object) = nyash_rust::runtime::host_handles::get(handle as u64) {
            return object.share_box();
        }
    }
    Box::new(nyash_rust::box_trait::IntegerBox::new(handle))
}

fn try_handle_builtin_file_box_by_name(
    recv_handle: i64,
    method: &str,
    argc: i64,
    a1: i64,
    a2: i64,
) -> Option<i64> {
    use nyash_rust::box_trait::{IntegerBox, StringBox};
    use nyash_rust::boxes::array::ArrayBox;
    use nyash_rust::boxes::file::FileBox;

    if recv_handle <= 0 {
        return None;
    }
    let object = nyash_rust::runtime::host_handles::get(recv_handle as u64)?;
    let file_box = object.as_any().downcast_ref::<FileBox>()?;

    match method {
        "open" => {
            if argc < 1 || argc > 2 {
                return Some(0);
            }
            let Some(path) = decode_handle_to_string_like(a1) else {
                return Some(0);
            };
            let mode = if argc >= 2 {
                match decode_handle_to_string_like(a2) {
                    Some(mode) => mode,
                    None => return Some(0),
                }
            } else {
                "r".to_string()
            };
            Some(if file_box.ny_open(&path, &mode).is_ok() {
                1
            } else {
                0
            })
        }
        "read" => {
            if argc != 0 {
                return Some(0);
            }
            file_box
                .ny_read_to_string()
                .ok()
                .map(|text| encode_box_handle(Box::new(StringBox::new(text))))
                .or(Some(0))
        }
        "readBytes" => {
            if argc != 0 {
                return Some(0);
            }
            match file_box.ny_read_bytes() {
                Ok(bytes) => {
                    let arr = ArrayBox::new();
                    for byte in bytes {
                        arr.push(Box::new(IntegerBox::new(byte as i64)));
                    }
                    Some(encode_box_handle(Box::new(arr)))
                }
                Err(_) => Some(0),
            }
        }
        "write" => {
            if argc != 1 {
                return Some(0);
            }
            let result = file_box.write(decode_handle_to_box_or_integer(a1));
            Some(encode_box_handle(result))
        }
        "writeBytes" => {
            if argc != 1 {
                return Some(0);
            }
            let result = file_box.writeBytes(decode_handle_to_box_or_integer(a1));
            Some(encode_box_handle(result))
        }
        "close" => {
            if argc != 0 {
                return Some(0);
            }
            let _ = file_box.ny_close();
            Some(0)
        }
        _ => None,
    }
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

    if let Some(result) =
        crate::plugin::module_string_dispatch::try_dispatch(recv_handle, method_str, argc, a1, a2)
    {
        if trace {
            eprintln!(
                "[stage1/plugin_invoke] module_dispatch result method={} argc={} result_handle={}",
                method_str, argc, result
            );
        }
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
    if let Some(result) = try_handle_builtin_file_box_by_name(recv_handle, method_str, argc, a1, a2)
    {
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
