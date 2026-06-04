// Host-side runtime glue for plugin invoke shims.
// Keep receiver resolution / TLV decode policy here; do not let semantic ownership flow back in.

use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;

/// Thin shared host-service helpers for plugin invoke shims (i64/f64)
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

pub type InvokeFn =
    unsafe extern "C" fn(u32, u32, u32, *const u8, usize, *mut u8, *mut usize) -> i32;

/// Resolve route for a plugin object returned as TLV handle(tag=8).
///
/// Mainline (`NYASH_FAIL_FAST=1`):
/// - metadata missing -> None
/// - metadata present but no box route -> None
///
/// Compat (`NYASH_FAIL_FAST=0` and `NYASH_VM_USE_FALLBACK!=0`):
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
        return Some((
            meta.box_type,
            nyash_rust::runtime::plugin_loader_v2::nyash_plugin_invoke_v2_shim,
            meta.fini_method_id,
        ));
    }
    if nyash_rust::config::env::fail_fast()
        || !nyash_rust::config::env::vm_compat_fallback_allowed()
    {
        return None;
    }
    Some(("PluginBox".to_string(), fallback_invoke, None))
}

/// Resolve receiver from a0 via the handle registry only.
pub fn resolve_receiver_for_a0(a0: i64) -> Option<Receiver> {
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
    None
}

/// Resolve receiver + method id together for name-based invoke routes.
#[inline]
pub fn resolve_named_method_for_handle(
    recv_handle: i64,
    method: &str,
) -> Option<(NamedReceiver, u32)> {
    if recv_handle <= 0 {
        return None;
    }
    let obj = nyash_rust::runtime::host_handles::get(recv_handle as u64)?;
    let p = obj.as_any().downcast_ref::<PluginBoxV2>()?;
    let receiver = NamedReceiver {
        instance_id: p.instance_id(),
        real_type_id: p.inner.type_id,
        box_type: p.box_type.clone(),
        invoke: p.inner.invoke_fn,
    };
    let host = nyash_rust::runtime::plugin_loader_unified::get_global_plugin_host();
    let guard = host.read().ok()?;
    let handle = guard.resolve_method(&receiver.box_type, method).ok()?;
    Some((receiver, handle.method_id as u32))
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

/// Build the common two-payload TLV used by by-id / by-name invoke shims.
#[inline]
pub fn build_two_payload_tlv(argc: i64, a1: i64, a2: i64) -> Option<Vec<u8>> {
    let nargs = argc.max(0) as usize;
    if nargs > 2 && nyash_rust::config::env::fail_fast() {
        return None;
    }
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    if nargs >= 1 {
        crate::encode::nyrt_encode_arg(&mut buf, a1);
    }
    if nargs >= 2 {
        crate::encode::nyrt_encode_arg(&mut buf, a2);
    }
    Some(buf)
}

/// Invoke a resolved receiver and decode first TLV entry to i64.
#[inline]
pub fn invoke_receiver_to_i64(
    invoke: InvokeFn,
    type_id: u32,
    method_id: u32,
    instance_id: u32,
    tlv_args: &[u8],
) -> Option<i64> {
    let (tag, sz, payload) = plugin_invoke_call(invoke, type_id, method_id, instance_id, tlv_args)?;
    match tag {
        2 => nyash_rust::runtime::plugin_ffi_common::decode::i32(payload.as_slice())
            .map(|v| v as i64),
        3 => {
            if let Some(v) = nyash_rust::runtime::plugin_ffi_common::decode::i32(payload.as_slice())
            {
                return Some(v as i64);
            }
            if payload.len() == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload.as_slice());
                return Some(i64::from_le_bytes(b));
            }
            None
        }
        6 | 7 => {
            use nyash_rust::box_trait::{NyashBox, StringBox};
            let s = nyash_rust::runtime::plugin_ffi_common::decode::string(payload.as_slice());
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
                    resolve_invoke_route_for_type(r_type, invoke)?;
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
        1 => nyash_rust::runtime::plugin_ffi_common::decode::bool(payload.as_slice()).map(|b| {
            if b {
                1
            } else {
                0
            }
        }),
        5 => {
            if crate::env_flags::flag_on("NYASH_JIT_NATIVE_F64") && sz == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload.as_slice());
                let f = f64::from_le_bytes(b);
                return Some(f as i64);
            }
            None
        }
        _ => None,
    }
}

/// Invoke a resolved receiver and decode first TLV entry to f64.
#[inline]
pub fn invoke_receiver_to_f64(
    invoke: InvokeFn,
    type_id: u32,
    method_id: u32,
    instance_id: u32,
    tlv_args: &[u8],
) -> Option<f64> {
    let (tag, sz, payload) = plugin_invoke_call(invoke, type_id, method_id, instance_id, tlv_args)?;
    match tag {
        5 => {
            if sz == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload.as_slice());
                Some(f64::from_le_bytes(b))
            } else {
                None
            }
        }
        3 => {
            if let Some(v) = nyash_rust::runtime::plugin_ffi_common::decode::i32(payload.as_slice())
            {
                return Some(v as f64);
            }
            if payload.len() == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload.as_slice());
                return Some((i64::from_le_bytes(b)) as f64);
            }
            None
        }
        1 => nyash_rust::runtime::plugin_ffi_common::decode::bool(payload.as_slice()).map(|b| {
            if b {
                1.0
            } else {
                0.0
            }
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::with_env_vars;

    unsafe extern "C" fn fallback_stub(
        _type_id: u32,
        _method_id: u32,
        _instance_id: u32,
        _args: *const u8,
        _args_len: usize,
        _result: *mut u8,
        _result_len: *mut usize,
    ) -> i32 {
        0
    }

    #[test]
    fn resolve_invoke_route_allows_compat_when_enabled() {
        with_env_vars(
            &[("NYASH_FAIL_FAST", "0"), ("NYASH_VM_USE_FALLBACK", "1")],
            || {
                let route = resolve_invoke_route_for_type(u32::MAX, fallback_stub);
                assert!(route.is_some());
            },
        );
    }

    #[test]
    fn resolve_invoke_route_blocks_compat_when_fallback_off() {
        with_env_vars(
            &[("NYASH_FAIL_FAST", "0"), ("NYASH_VM_USE_FALLBACK", "0")],
            || {
                let route = resolve_invoke_route_for_type(u32::MAX, fallback_stub);
                assert!(route.is_none());
            },
        );
    }
}
