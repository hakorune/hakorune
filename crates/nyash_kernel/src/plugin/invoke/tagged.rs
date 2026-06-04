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
    let Some(recv) = invoke_core::resolve_receiver_for_a0(a0) else {
        return 0;
    };
    let instance_id = recv.instance_id;
    let real_type_id = recv.real_type_id;
    let invoke = recv.invoke;

    let nargs = argc.max(0) as usize;
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    let mut enc = |val: i64, tag: i64| {
        crate::plugin::encode_tagged_value(
            &mut buf,
            val,
            tag,
            crate::plugin::MissingHandleEncoding::Zero,
        )
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

    invoke_core::invoke_receiver_to_i64(invoke, real_type_id, method_id as u32, instance_id, &buf)
        .unwrap_or(0)
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
    let Some(recv) = invoke_core::resolve_receiver_for_a0(recv_h) else {
        return 0;
    };
    let instance_id = recv.instance_id;
    let real_type_id = recv.real_type_id;
    let invoke = recv.invoke;

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
        crate::plugin::encode_tagged_value(
            &mut buf,
            vals[i],
            tags[i],
            crate::plugin::MissingHandleEncoding::Zero,
        );
    }

    invoke_core::invoke_receiver_to_i64(invoke, real_type_id, method_id as u32, instance_id, &buf)
        .unwrap_or(0)
}
