use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;

/// Thin shared helpers for plugin invoke shims (i64/f64)
///
/// Goal: centralize receiver resolution and the dynamic buffer call loop,
/// keeping extern functions in invoke.rs small and consistent.

pub struct Receiver {
    pub instance_id: u32,
    pub real_type_id: u32,
    pub invoke: InvokeFn,
}

pub struct NamedReceiver {
    pub instance_id: u32,
    pub real_type_id: u32,
    pub box_type: String,
    pub invoke: InvokeFn,
}

#[inline]
fn plugin_shim_invoke() -> InvokeFn {
    nyash_rust::runtime::plugin_loader_v2::nyash_plugin_invoke_v2_shim
}

pub type InvokeFn = unsafe extern "C" fn(
    u32,
    u32,
    u32,
    *const u8,
    usize,
    *mut u8,
    *mut usize,
) -> i32;

/// Resolve route for a plugin object returned as TLV handle(tag=8).
///
/// Mainline (`NYASH_FAIL_FAST=1`):
/// - metadata missing -> None
/// - metadata present but no box route -> None
///
/// Compat (`NYASH_FAIL_FAST=0`):
/// - metadata missing -> fallback invoke + generic box type
pub fn resolve_invoke_route_for_type(
    type_id: u32,
    fallback_invoke: InvokeFn,
) -> Option<(String, InvokeFn, Option<u32>)> {
    let meta_opt = nyash_rust::runtime::plugin_loader_v2::metadata_for_type_id(type_id);
    if let Some(meta) = meta_opt {
        if meta.invoke_box_fn.is_none() && nyash_rust::config::env::fail_fast() {
            return None;
        }
        return Some((meta.box_type, plugin_shim_invoke(), meta.fini_method_id));
    }
    if nyash_rust::config::env::fail_fast() {
        return None;
    }
    super::compat_invoke_core::resolve_generic_fallback_route(fallback_invoke)
}

/// Resolve receiver from a0: handle registry only (legacy VM receiver fallback removed).
pub fn resolve_receiver_for_a0(a0: i64) -> Option<Receiver> {
    // 1) Handle registry (preferred)
    if a0 > 0 {
        if let Some(obj) = nyash_rust::runtime::host_handles::get(a0 as u64) {
            if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
                return Some(Receiver {
                    instance_id: p.instance_id(),
                    real_type_id: p.inner.type_id,
                    invoke: p.inner.invoke_fn,
                });
            }
        }
    }
    // ✂️ REMOVED: Legacy VM argument receiver resolution
    // Plugin-First architecture requires explicit handle-based receiver resolution only
    None
}

/// Resolve a plugin receiver for name-based invoke shims.
/// This includes the concrete box_type string used by method resolution.
pub fn resolve_named_receiver_for_handle(recv_handle: i64) -> Option<NamedReceiver> {
    if recv_handle <= 0 {
        return None;
    }
    let obj = nyash_rust::runtime::host_handles::get(recv_handle as u64)?;
    let p = obj.as_any().downcast_ref::<PluginBoxV2>()?;
    Some(NamedReceiver {
        instance_id: p.instance_id(),
        real_type_id: p.inner.type_id,
        box_type: p.box_type.clone(),
        invoke: p.inner.invoke_fn,
    })
}

/// Resolve method id by name for a plugin receiver route.
pub fn resolve_method_id_for_named_receiver(
    receiver: &NamedReceiver,
    method: &str,
) -> Option<u32> {
    let host = nyash_rust::runtime::plugin_loader_unified::get_global_plugin_host();
    let guard = host.read().ok()?;
    let handle = guard.resolve_method(&receiver.box_type, method).ok()?;
    Some(handle.method_id as u32)
}

/// Call plugin invoke with dynamic buffer growth, returning first TLV entry on success.
pub fn plugin_invoke_call(
    invoke: InvokeFn,
    type_id: u32,
    method_id: u32,
    instance_id: u32,
    tlv_args: &[u8],
) -> Option<(u8, usize, Vec<u8>)> {
    let mut cap: usize = 256;
    let mut tag_ret: u8 = 0;
    let mut sz_ret: usize = 0;
    let mut payload_ret: Vec<u8> = Vec::new();
    loop {
        let mut out = vec![0u8; cap];
        let mut out_len: usize = out.len();
        let rc = unsafe {
            invoke(
                type_id,
                method_id,
                instance_id,
                tlv_args.as_ptr(),
                tlv_args.len(),
                out.as_mut_ptr(),
                &mut out_len,
            )
        };
        if rc != 0 {
            // Retry on short buffer hint (-1) or when plugin wrote beyond capacity (len > cap)
            if rc == -1 || out_len > cap {
                cap = cap.saturating_mul(2).max(out_len + 16);
                if cap > 1 << 20 {
                    break;
                }
                continue;
            }
            return None;
        }
        let slice = &out[..out_len];
        if let Some((t, s, p)) = nyash_rust::runtime::plugin_ffi_common::decode::tlv_first(slice) {
            tag_ret = t;
            sz_ret = s;
            payload_ret = p.to_vec();
        }
        break;
    }
    if payload_ret.is_empty() {
        return None;
    }
    Some((tag_ret, sz_ret, payload_ret))
}

/// Decode a single TLV entry to i64 with side-effects (handle registration) when applicable.
pub fn decode_entry_to_i64(
    tag: u8,
    sz: usize,
    payload: &[u8],
    fallback_invoke: InvokeFn,
) -> Option<i64> {
    match tag {
        2 => nyash_rust::runtime::plugin_ffi_common::decode::i32(payload).map(|v| v as i64),
        3 => {
            if let Some(v) = nyash_rust::runtime::plugin_ffi_common::decode::i32(payload) {
                return Some(v as i64);
            }
            if payload.len() == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload);
                return Some(i64::from_le_bytes(b));
            }
            None
        }
        6 | 7 => {
            use nyash_rust::box_trait::{NyashBox, StringBox};
            let s = nyash_rust::runtime::plugin_ffi_common::decode::string(payload);
            let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(StringBox::new(s));
            let h = nyash_rust::runtime::host_handles::to_handle_arc(arc) as u64;
            Some(h as i64)
        }
        8 => {
            if sz == 8 {
                let mut t = [0u8; 4];
                t.copy_from_slice(&payload[0..4]);
                let mut i = [0u8; 4];
                i.copy_from_slice(&payload[4..8]);
                let r_type = u32::from_le_bytes(t);
                let r_inst = u32::from_le_bytes(i);
                let (box_type_name, invoke_ptr, _fini_id) =
                    resolve_invoke_route_for_type(r_type, fallback_invoke)?;
                let pb = nyash_rust::runtime::plugin_loader_v2::make_plugin_box_v2(
                    box_type_name,
                    r_type,
                    r_inst,
                    invoke_ptr,
                );
                let arc: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> =
                    std::sync::Arc::new(pb);
                let h = nyash_rust::runtime::host_handles::to_handle_arc(arc) as u64;
                return Some(h as i64);
            }
            None
        }
        1 => {
            nyash_rust::runtime::plugin_ffi_common::decode::bool(payload)
                .map(|b| if b { 1 } else { 0 })
        }
        5 => {
            if std::env::var("NYASH_JIT_NATIVE_F64").ok().as_deref() == Some("1") && sz == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload);
                let f = f64::from_le_bytes(b);
                return Some(f as i64);
            }
            None
        }
        _ => None,
    }
}

/// Decode a single TLV entry to f64 when possible.
pub fn decode_entry_to_f64(tag: u8, sz: usize, payload: &[u8]) -> Option<f64> {
    match tag {
        5 => {
            if sz == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload);
                Some(f64::from_le_bytes(b))
            } else {
                None
            }
        }
        3 => {
            if let Some(v) = nyash_rust::runtime::plugin_ffi_common::decode::i32(payload) {
                return Some(v as f64);
            }
            if payload.len() == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload);
                return Some((i64::from_le_bytes(b)) as f64);
            }
            None
        }
        1 => nyash_rust::runtime::plugin_ffi_common::decode::bool(payload).map(|b| {
            if b {
                1.0
            } else {
                0.0
            }
        }),
        _ => None,
    }
}

#[inline]
pub fn jit_args_handle_only_enabled() -> bool {
    std::env::var("NYASH_JIT_ARGS_HANDLE_ONLY").ok().as_deref() == Some("1")
}

#[inline]
pub fn encode_legacy_vm_args_range(dst: &mut Vec<u8>, start_pos: usize, end_pos_inclusive: usize) {
    if nyash_rust::config::env::fail_fast()
        || start_pos > end_pos_inclusive
        || jit_args_handle_only_enabled()
    {
        return;
    }
    super::compat_invoke_core::encode_legacy_vm_args_range(dst, start_pos, end_pos_inclusive);
}

#[inline]
pub fn encode_legacy_args_with_failfast_policy(
    dst: &mut Vec<u8>,
    start_pos: usize,
    end_pos_inclusive: usize,
) -> bool {
    if start_pos > end_pos_inclusive {
        return true;
    }
    if nyash_rust::config::env::fail_fast() {
        return false;
    }
    encode_legacy_vm_args_range(dst, start_pos, end_pos_inclusive);
    true
}
