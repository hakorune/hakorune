#![allow(unused_mut, unused_variables)]
use crate::encode::{nyrt_encode_arg_or_legacy, nyrt_encode_from_legacy_at};
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
    // Resolve receiver via shared core helper
    let recv = match invoke_core::resolve_receiver_for_a0(a0) {
        Some(r) => r,
        None => return 0,
    };
    let instance_id: u32 = recv.instance_id;
    let _real_type_id: u32 = recv.real_type_id;
    let invoke = recv.invoke;
    // Build TLV args from a1/a2 if present. Prefer handles/StringBox/IntegerBox via runtime host.
    // ✂️ REMOVED: VMValue import - no longer needed in Plugin-First architecture
    // argc from LLVM lowering is explicit arg count (excludes receiver)
    let nargs = argc.max(0) as usize;
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    // Encode legacy VM arg at position into provided buffer (avoid capturing &mut buf)
    let mut encode_from_legacy_into =
        |dst: &mut Vec<u8>, arg_pos: usize| nyrt_encode_from_legacy_at(dst, arg_pos);
    // Encode argument value or fallback to legacy slot (avoid capturing &mut buf)
    let mut encode_arg_into =
        |dst: &mut Vec<u8>, val: i64, pos: usize| nyrt_encode_arg_or_legacy(dst, val, pos);
    if nargs >= 1 {
        encode_arg_into(&mut buf, a1, 1);
    }
    if nargs >= 2 {
        encode_arg_into(&mut buf, a2, 2);
    }
    // Extra args from legacy VM args (positions 3..nargs)
    if nargs > 2 && std::env::var("NYASH_JIT_ARGS_HANDLE_ONLY").ok().as_deref() != Some("1") {
        for pos in 3..=nargs {
            encode_from_legacy_into(&mut buf, pos);
        }
    }
    // Call invoke with dynamic buffer logic centralized
    let (tag_ret, sz_ret, payload_ret): (u8, usize, Vec<u8>) = match invoke_core::plugin_invoke_call(
        invoke,
        type_id as u32,
        method_id as u32,
        instance_id,
        &buf,
    ) {
        Some((t, s, p)) => (t, s, p),
        None => return 0,
    };
    if let Some((tag, sz, payload)) = Some((tag_ret, sz_ret, payload_ret.as_slice())) {
        if let Some(v) = invoke_core::decode_entry_to_i64(tag, sz, payload, invoke) {
            return v;
        }
    }
    0
}

// F64-typed shim: decode TLV first entry and return f64 when possible
#[no_mangle]
pub extern "C" fn nyash_plugin_invoke3_f64(
    type_id: i64,
    method_id: i64,
    argc: i64,
    a0: i64,
    a1: i64,
    a2: i64,
) -> f64 {
    use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;
    // Resolve receiver from legacy VM args or handle registry
    let mut instance_id: u32 = 0;
    let mut invoke: Option<
        unsafe extern "C" fn(u32, u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32,
    > = None;
    if a0 > 0 {
        if let Some(obj) = nyash_rust::runtime::host_handles::get(a0 as u64) {
            if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
                instance_id = p.instance_id();
                invoke = Some(p.inner.invoke_fn);
            }
        }
    }
    // ✂️ REMOVED: Legacy VM receiver resolution fallback
    // In Plugin-First architecture, receivers must be explicitly provided via handles
    // ✂️ REMOVED: Legacy VM fallback scan for PluginBoxV2
    // Plugin-First architecture requires explicit receiver handles
    if invoke.is_none() {
        return 0.0;
    }
    // Build TLV args from a1/a2 with String/Integer support
    // legacy helper imports not required in current path
    // argc from LLVM lowering is explicit arg count (excludes receiver)
    let nargs = argc.max(0) as usize;
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    // ✂️ REMOVED: Legacy VM argument encoding closure
    // Plugin-First architecture uses explicit handle-based argument encoding only
    let mut encode_from_legacy = |_arg_pos: usize| {
        // ✂️ REMOVED: Legacy VMValue processing - no fallback encoding
        nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, 0); // Placeholder
    };
    let mut encode_arg =
        |val: i64, pos: usize| crate::encode::nyrt_encode_arg_or_legacy(&mut buf, val, pos);
    if nargs >= 1 {
        encode_arg(a1, 1);
    }
    if nargs >= 2 {
        encode_arg(a2, 2);
    }
    if nargs > 2 && std::env::var("NYASH_JIT_ARGS_HANDLE_ONLY").ok().as_deref() != Some("1") {
        for pos in 3..=nargs {
            nyrt_encode_from_legacy_at(&mut buf, pos);
        }
    }
    // Invoke via shared helper
    let (mut tag_ret, mut sz_ret, mut payload_ret): (u8, usize, Vec<u8>) =
        match invoke_core::plugin_invoke_call(
            invoke.unwrap(),
            type_id as u32,
            method_id as u32,
            instance_id,
            &buf,
        ) {
            Some((t, s, p)) => (t, s, p),
            None => return 0.0,
        };
    if let Some((tag, sz, payload)) = Some((tag_ret, sz_ret, payload_ret.as_slice())) {
        if let Some(f) = invoke_core::decode_entry_to_f64(tag, sz, payload) {
            return f;
        }
    }
    0.0
}
// By-name shims for common method names (getattr/call)
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
    use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;
    // Resolve receiver
    let mut instance_id: u32 = 0;
    let mut type_id: u32 = 0;
    let mut box_type: Option<String> = None;
    let mut invoke: Option<
        unsafe extern "C" fn(u32, u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32,
    > = None;
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
    // ✂️ REMOVED: Legacy VM receiver resolution by index
    // Plugin-First architecture requires explicit handle-based receiver resolution
    // ✂️ REMOVED: Legacy VM argument scan fallback
    // Plugin-First architecture eliminates VM argument iteration
    if invoke.is_none() {
        return 0;
    }
    let box_type = box_type.unwrap_or_default();
    // Resolve method_id via PluginHost by name
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
    // Build TLV args from legacy VM args (skip receiver slot)
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(
        argc.saturating_sub(1).max(0) as u16,
    );
    // ✂️ REMOVED: Legacy VM argument addition closure
    // Plugin-First architecture handles arguments via explicit handles and primitives only
    let mut add_from_legacy = |_pos: usize| {
        // ✂️ REMOVED: Complete VMValue processing system
        nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, 0); // Default placeholder
    };
    if argc >= 2 {
        add_from_legacy(1);
    }
    if argc >= 3 {
        add_from_legacy(2);
    }
    if argc > 3 {
        for pos in 3..(argc as usize) {
            add_from_legacy(pos);
        }
    }
    let mut out = vec![0u8; 4096];
    let mut out_len: usize = out.len();
    let rc = unsafe {
        invoke.unwrap()(
            type_id as u32,
            method_id,
            instance_id,
            buf.as_ptr(),
            buf.len(),
            out.as_mut_ptr(),
            &mut out_len,
        )
    };
    if rc != 0 {
        return 0;
    }
    let out_slice = &out[..out_len];
    if let Some((tag, sz, payload)) =
        nyash_rust::runtime::plugin_ffi_common::decode::tlv_first(out_slice)
    {
        if let Some(v) = super::invoke_core::decode_entry_to_i64(tag, sz, payload, invoke.unwrap())
        {
            return v;
        }
    }
    0
}

// ========================================================================
// Phase 285LLVM-1.3: InstanceBox Field Access Helpers
// ========================================================================

/// Helper: handle → String デコード
///
/// Phase 285LLVM-1.3: Fail-Fast error logging
fn decode_handle_to_string(handle: i64) -> Result<String, String> {
    if handle <= 0 {
        return Err(format!("Invalid handle: {}", handle));
    }

    let obj = nyash_rust::runtime::host_handles::get(handle as u64)
        .ok_or_else(|| format!("Handle {} not found", handle))?;

    let sb = obj
        .as_any()
        .downcast_ref::<nyash_rust::box_trait::StringBox>()
        .ok_or_else(|| format!("Handle {} is not a StringBox", handle))?;

    Ok(sb.value.clone())
}

/// Helper: handle → NyashValue デコード（対応型のみ）
///
/// Phase 285LLVM-1.3: Integer/String/Bool のみ対応、未対応型は明示エラー
fn decode_handle_to_nyash_value(handle: i64) -> Result<nyash_rust::value::NyashValue, String> {
    use nyash_rust::box_trait::{BoolBox, IntegerBox, StringBox};
    use nyash_rust::value::NyashValue;

    if handle <= 0 {
        return Err(format!("Invalid handle: {}", handle));
    }

    let obj = nyash_rust::runtime::host_handles::get(handle as u64)
        .ok_or_else(|| format!("Handle {} not found", handle))?;

    // Integer
    if let Some(ib) = obj.as_any().downcast_ref::<IntegerBox>() {
        return Ok(NyashValue::Integer(ib.value));
    }

    // String
    if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
        return Ok(NyashValue::String(sb.value.clone()));
    }

    // Bool
    if let Some(bb) = obj.as_any().downcast_ref::<BoolBox>() {
        return Ok(NyashValue::Bool(bb.value));
    }

    // 未対応型: 明示的エラー（次フェーズで対応）
    Err(format!(
        "Unsupported Box type for handle {}: Phase 285LLVM-1.3 supports Integer/String/Bool only",
        handle
    ))
}

/// InstanceBox.getField(field_name) → i64 handle
///
/// Fail-Fast: エラーは明示的にログ出力して0返却
fn handle_instance_get_field(inst: &nyash_rust::instance_v2::InstanceBox, field_handle: i64) -> i64 {
    use nyash_rust::box_trait::{BoolBox, IntegerBox, StringBox};
    use nyash_rust::value::NyashValue;
    use std::sync::Arc;

    // 1. field_name デコード (既存ユーティリティ活用)
    let field_name = match decode_handle_to_string(field_handle) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "[llvm/invoke/getField] Failed to decode field_name handle {}: {}",
                field_handle, e
            );
            return 0;
        }
    };

    // 2. fields_ng から値を取得 (SSOT: get_field_ng のみ使用)
    let nv = match inst.get_field_ng(&field_name) {
        Some(v) => v,
        None => {
            eprintln!(
                "[llvm/invoke/getField] Field '{}' not found in InstanceBox",
                field_name
            );
            return 0;
        }
    };

    // 3. NyashValue → i64 handle 変換
    match nv {
        NyashValue::Integer(i) => {
            let arc: Arc<dyn nyash_rust::box_trait::NyashBox> = Arc::new(IntegerBox::new(i));
            let handle = nyash_rust::runtime::host_handles::to_handle_arc(arc) as i64;
            eprintln!("[llvm/invoke/getField] Returning Integer({}) as handle {}", i, handle);
            // Verify handle can be resolved back
            if let Some(obj) = nyash_rust::runtime::host_handles::get(handle as u64) {
                if let Some(ib) = obj.as_any().downcast_ref::<nyash_rust::box_trait::IntegerBox>() {
                    eprintln!("[llvm/invoke/getField] ✅ Verified: handle {} resolves to IntegerBox({})", handle, ib.value);
                } else {
                    eprintln!("[llvm/invoke/getField] ❌ ERROR: handle {} does not resolve to IntegerBox!", handle);
                }
            } else {
                eprintln!("[llvm/invoke/getField] ❌ ERROR: handle {} cannot be resolved!", handle);
            }
            handle
        }
        NyashValue::String(s) => {
            let arc: Arc<dyn nyash_rust::box_trait::NyashBox> = Arc::new(StringBox::new(s));
            nyash_rust::runtime::host_handles::to_handle_arc(arc) as i64
        }
        NyashValue::Bool(b) => {
            let arc: Arc<dyn nyash_rust::box_trait::NyashBox> = Arc::new(BoolBox::new(b));
            nyash_rust::runtime::host_handles::to_handle_arc(arc) as i64
        }
        NyashValue::Null | NyashValue::Void => 0,

        // 未対応型: 明示的エラー（次フェーズで対応）
        NyashValue::Float(_) => {
            eprintln!(
                "[llvm/invoke/getField] Unsupported type: Float (field: {})",
                field_name
            );
            0
        }
        NyashValue::Array(_) => {
            eprintln!(
                "[llvm/invoke/getField] Unsupported type: Array (field: {})",
                field_name
            );
            0
        }
        NyashValue::Map(_) => {
            eprintln!(
                "[llvm/invoke/getField] Unsupported type: Map (field: {})",
                field_name
            );
            0
        }
        NyashValue::Box(_) => {
            eprintln!(
                "[llvm/invoke/getField] Unsupported type: Box (field: {})",
                field_name
            );
            0
        }
        NyashValue::WeakBox(_) => {
            eprintln!(
                "[llvm/invoke/getField] Unsupported type: WeakBox (field: {})",
                field_name
            );
            0
        }
    }
}

/// InstanceBox.setField(field_name, value) → 1 (success) or 0 (failure)
///
/// Fail-Fast: エラーは明示的にログ出力して0返却
fn handle_instance_set_field(
    inst: &nyash_rust::instance_v2::InstanceBox,
    field_handle: i64,
    value_handle: i64,
) -> i64 {
    use nyash_rust::value::NyashValue;

    // 1. field_name デコード (既存ユーティリティ活用)
    let field_name = match decode_handle_to_string(field_handle) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "[llvm/invoke/setField] Failed to decode field_name handle {}: {}",
                field_handle, e
            );
            return 0;
        }
    };

    // 2. value handle → NyashValue 変換
    // LLVM backend では i64 値がそのまま渡される場合がある（handle ではなく生の値）
    let nv = if value_handle == 0 {
        NyashValue::Null
    } else {
        // まず handle として解決を試みる
        match decode_handle_to_nyash_value(value_handle) {
            Ok(v) => v,
            Err(_) => {
                // handle でない場合は、i64 値として直接扱う（LLVM backend の挙動）
                eprintln!(
                    "[llvm/invoke/setField] Handle {} not found for field '{}', treating as raw i64 value",
                    value_handle, field_name
                );
                NyashValue::Integer(value_handle)
            }
        }
    };

    // 3. fields_ng に設定 (SSOT: set_field_ng のみ使用)
    match inst.set_field_ng(field_name.clone(), nv) {
        Ok(_) => 1, // 成功
        Err(e) => {
            eprintln!(
                "[llvm/invoke/setField] Failed to set field '{}': {}",
                field_name, e
            );
            0
        }
    }
}

// General by-name invoke: (recv_handle, method_cstr, argc, a1, a2) -> i64
// Export name: nyash.plugin.invoke_by_name_i64
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

    use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;
    use nyash_rust::instance_v2::InstanceBox;
    let mut instance_id: u32 = 0;
    let mut type_id: u32 = 0;
    let mut box_type: Option<String> = None;
    let mut invoke: Option<
        unsafe extern "C" fn(u32, u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32,
    > = None;
    if recv_handle > 0 {
        if let Some(obj) = nyash_rust::runtime::host_handles::get(recv_handle as u64) {
            if let Some(inst) = obj.as_any().downcast_ref::<InstanceBox>() {
                match method_str {
                    "getField" => {
                        return handle_instance_get_field(inst, a1);
                    }
                    "setField" => {
                        return handle_instance_set_field(inst, a1, a2);
                    }
                    _ => {
                        return 0;
                    }
                }
            }
            if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
                instance_id = p.instance_id();
                type_id = p.inner.type_id;
                box_type = Some(p.box_type.clone());
                invoke = Some(p.inner.invoke_fn);
            }
        }
    }
    if invoke.is_none() {
        return 0;
    }
    let box_type = box_type.unwrap_or_default();
    // Resolve method_id via PluginHost by name
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
    // Build TLV args from a1/a2 (no legacy in LLVM path)
    // argc is the number of explicit arguments (receiver excluded)
    let nargs = argc.max(0) as usize;
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    nyrt_encode_arg_or_legacy(&mut buf, a1, 1);
    if nargs >= 2 {
        nyrt_encode_arg_or_legacy(&mut buf, a2, 2);
    }
    // Execute
    let mut out = vec![0u8; 512];
    let mut out_len: usize = out.len();
    let rc = unsafe {
        invoke.unwrap()(
            type_id as u32,
            method_id,
            instance_id,
            buf.as_ptr(),
            buf.len(),
            out.as_mut_ptr(),
            &mut out_len,
        )
    };
    if rc != 0 {
        return 0;
    }
    if let Some((tag, _sz, payload)) =
        nyash_rust::runtime::plugin_ffi_common::decode::tlv_first(&out[..out_len])
    {
        match tag {
            3 => {
                if payload.len() == 8 {
                    let mut b = [0u8; 8];
                    b.copy_from_slice(payload);
                    return i64::from_le_bytes(b);
                }
            }
            1 => {
                return if nyash_rust::runtime::plugin_ffi_common::decode::bool(payload)
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
                    let (box_type_name, invoke_ptr, _fini_id) =
                        match invoke_core::resolve_invoke_route_for_type(r_type, invoke.unwrap()) {
                            Some(v) => v,
                            None => return 0,
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
                if std::env::var("NYASH_JIT_NATIVE_F64").ok().as_deref() == Some("1") {
                    if payload.len() == 8 {
                        let mut b = [0u8; 8];
                        b.copy_from_slice(payload);
                        let f = f64::from_le_bytes(b);
                        return f as i64;
                    }
                }
            }
            _ => {}
        }
    }
    0
}

// Tagged by-id invoke (supports f64/int/handle for first two args)
// tag: 3=I64, 5=F64(bits), 8=Handle
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
    // Resolve receiver invoke and actual plugin type_id
    let mut instance_id: u32 = 0;
    let mut real_type_id: u32 = type_id as u32;
    let mut invoke: Option<
        unsafe extern "C" fn(u32, u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32,
    > = None;
    if a0 > 0 {
        if let Some(obj) = nyash_rust::runtime::host_handles::get(a0 as u64) {
            if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
                instance_id = p.instance_id();
                real_type_id = p.inner.type_id;
                invoke = Some(p.inner.invoke_fn);
            }
        }
    }
    if invoke.is_none() {
        return 0;
    }
    // Build TLV from tags
    // argc is the number of explicit arguments (receiver excluded)
    let nargs = argc.max(0) as usize;
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    let mut enc = |val: i64, tag: i64| match tag {
        3 => nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, val),
        5 => {
            let bits = val as u64;
            let f = f64::from_bits(bits);
            nyash_rust::runtime::plugin_ffi_common::encode::f64(&mut buf, f);
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
    // Invoke
    let mut out = vec![0u8; 512];
    let mut out_len: usize = out.len();
    let rc = unsafe {
        invoke.unwrap()(
            real_type_id,
            method_id as u32,
            instance_id,
            buf.as_ptr(),
            buf.len(),
            out.as_mut_ptr(),
            &mut out_len,
        )
    };
    if rc != 0 {
        return 0;
    }
    if let Some((tag, sz, payload)) =
        nyash_rust::runtime::plugin_ffi_common::decode::tlv_first(&out[..out_len])
    {
        if let Some(v) = invoke_core::decode_entry_to_i64(tag, sz, payload, invoke.unwrap()) {
            return v;
        }
    }
    0
}

// Variable-length tagged invoke by-id
// Exported as: nyash.plugin.invoke_tagged_v_i64(i64 type_id, i64 method_id, i64 argc, i64 recv_h, i64* vals, i64* tags) -> i64
#[export_name = "nyash.plugin.invoke_tagged_v_i64"]
pub extern "C" fn nyash_plugin_invoke_tagged_v_i64(
    type_id: i64,
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
    // Resolve receiver invoke
    let mut instance_id: u32 = 0;
    let mut real_type_id: u32 = 0;
    let mut invoke: Option<
        unsafe extern "C" fn(u32, u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32,
    > = None;
    if let Some(obj) = nyash_rust::runtime::host_handles::get(recv_h as u64) {
        if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
            instance_id = p.instance_id();
            real_type_id = p.inner.type_id;
            invoke = Some(p.inner.invoke_fn);
        }
    }
    if invoke.is_none() {
        return 0;
    }
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
                let f = f64::from_bits(vals[i] as u64);
                nyash_rust::runtime::plugin_ffi_common::encode::f64(&mut buf, f);
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
    let mut out = vec![0u8; 1024];
    let mut out_len: usize = out.len();
    let rc = unsafe {
        invoke.unwrap()(
            real_type_id,
            method_id as u32,
            instance_id,
            buf.as_ptr(),
            buf.len(),
            out.as_mut_ptr(),
            &mut out_len,
        )
    };
    if rc != 0 {
        return 0;
    }
    if let Some((tag, sz, payload)) =
        nyash_rust::runtime::plugin_ffi_common::decode::tlv_first(&out[..out_len])
    {
        if let Some(v) = invoke_core::decode_entry_to_i64(tag, sz, payload, invoke.unwrap()) {
            return v;
        }
    }
    0
}
