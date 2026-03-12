use crate::plugin::invoke_core;

#[export_name = "nyash_plugin_invoke3_tagged_i64"]
pub extern "C" fn nyash_plugin_invoke3_tagged_i64(
    type_id: i64,
    method_id: i64,
    argc: i64,
    a0: i64,
    a1: i64,
    tag1: i64,
    a2: i64,
    tag2: i64,
    a3: i64,
    tag3: i64,
    a4: i64,
    tag4: i64,
) -> i64 {
    use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;

    let mut instance_id: u32 = 0;
    let mut real_type_id: u32 = type_id as u32;
    let mut invoke: Option<invoke_core::InvokeFn> = None;
    if a0 > 0 {
        if let Some(obj) = nyash_rust::runtime::host_handles::get(a0 as u64) {
            if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
                instance_id = p.instance_id();
                real_type_id = p.inner.type_id;
                invoke = Some(p.inner.invoke_fn);
            }
        }
    }
    let Some(invoke) = invoke else {
        return 0;
    };

    let nargs = argc.max(0) as usize;
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    let mut enc = |val: i64, tag: i64| match tag {
        3 => nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, val),
        5 => {
            let bits = val as u64;
            nyash_rust::runtime::plugin_ffi_common::encode::f64(&mut buf, f64::from_bits(bits));
        }
        8 => {
            if val > 0 {
                if let Some(obj) = nyash_rust::runtime::host_handles::get(val as u64) {
                    if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
                        nyash_rust::runtime::plugin_ffi_common::encode::plugin_handle(
                            &mut buf,
                            p.inner.type_id,
                            p.instance_id(),
                        );
                    } else {
                        let s = obj.to_string_box().value;
                        nyash_rust::runtime::plugin_ffi_common::encode::string(&mut buf, &s);
                    }
                }
            } else {
                nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, 0);
            }
        }
        _ => nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, val),
    };
    if nargs >= 1 {
        enc(a1, tag1);
    }
    if nargs >= 2 {
        enc(a2, tag2);
    }
    if nargs >= 3 {
        enc(a3, tag3);
    }
    if nargs >= 4 {
        enc(a4, tag4);
    }

    let Some((tag, sz, payload)) =
        invoke_core::plugin_invoke_call(invoke, real_type_id, method_id as u32, instance_id, &buf)
    else {
        return 0;
    };
    invoke_core::decode_entry_to_i64(tag, sz, payload.as_slice(), invoke).unwrap_or(0)
}

#[export_name = "nyash.plugin.invoke_tagged_v_i64"]
pub extern "C" fn nyash_plugin_invoke_tagged_v_i64(
    _type_id: i64,
    method_id: i64,
    argc: i64,
    recv_h: i64,
    vals: *const i64,
    tags: *const i64,
) -> i64 {
    use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;
    if recv_h <= 0 {
        return 0;
    }

    let mut instance_id: u32 = 0;
    let mut real_type_id: u32 = 0;
    let mut invoke: Option<invoke_core::InvokeFn> = None;
    if let Some(obj) = nyash_rust::runtime::host_handles::get(recv_h as u64) {
        if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
            instance_id = p.instance_id();
            real_type_id = p.inner.type_id;
            invoke = Some(p.inner.invoke_fn);
        }
    }
    let Some(invoke) = invoke else {
        return 0;
    };

    let nargs = argc.saturating_sub(1).max(0) as usize;
    let (vals, tags) = if nargs > 0 && !vals.is_null() && !tags.is_null() {
        unsafe {
            (
                std::slice::from_raw_parts(vals, nargs),
                std::slice::from_raw_parts(tags, nargs),
            )
        }
    } else {
        (&[][..], &[][..])
    };

    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    for i in 0..nargs {
        match tags[i] {
            3 => nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, vals[i]),
            5 => {
                nyash_rust::runtime::plugin_ffi_common::encode::f64(
                    &mut buf,
                    f64::from_bits(vals[i] as u64),
                );
            }
            8 => {
                if let Some(obj) = nyash_rust::runtime::host_handles::get(vals[i] as u64) {
                    if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
                        nyash_rust::runtime::plugin_ffi_common::encode::plugin_handle(
                            &mut buf,
                            p.inner.type_id,
                            p.instance_id(),
                        );
                    } else {
                        let s = obj.to_string_box().value;
                        nyash_rust::runtime::plugin_ffi_common::encode::string(&mut buf, &s);
                    }
                } else {
                    nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, 0);
                }
            }
            _ => nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, vals[i]),
        }
    }

    let Some((tag, sz, payload)) =
        invoke_core::plugin_invoke_call(invoke, real_type_id, method_id as u32, instance_id, &buf)
    else {
        return 0;
    };
    invoke_core::decode_entry_to_i64(tag, sz, payload.as_slice(), invoke).unwrap_or(0)
}
