// Spawn a plugin instance method asynchronously and return a Future handle (i64).

fn build_spawn_method_tlv(argc: i64, vals: *const i64, tags: *const i64) -> Vec<u8> {
    use nyash_rust::box_trait::{IntegerBox, StringBox};
    use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;

    let nargs = argc.saturating_sub(1).max(0) as usize;
    let mut buf = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs as u16);
    let vals_slice = if !vals.is_null() && nargs > 0 {
        // SAFETY: vals/tags are C ABI pointers paired with argc from caller.
        unsafe { std::slice::from_raw_parts(vals, nargs) }
    } else {
        &[]
    };
    let tags_slice = if !tags.is_null() && nargs > 0 {
        // SAFETY: vals/tags are C ABI pointers paired with argc from caller.
        unsafe { std::slice::from_raw_parts(tags, nargs) }
    } else {
        &[]
    };
    for i in 0..nargs {
        let v = vals_slice.get(i).copied().unwrap_or(0);
        let t = tags_slice.get(i).copied().unwrap_or(3); // default i64
        match t {
            3 => nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, v),
            5 => {
                let bits = v as u64;
                let f = f64::from_bits(bits);
                nyash_rust::runtime::plugin_ffi_common::encode::f64(&mut buf, f);
            }
            8 => {
                if v > 0 {
                    if let Some(obj) = nyash_rust::runtime::host_handles::get(v as u64) {
                        if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
                            // Try common coercions: String/Integer to TLV primitives.
                            let host = nyash_rust::runtime::get_global_plugin_host();
                            if let Ok(hg) = host.read() {
                                if p.box_type == "StringBox" {
                                    if let Ok(Some(sb)) =
                                        hg.invoke_instance_method("StringBox", "toUtf8", p.instance_id(), &[])
                                    {
                                        if let Some(s) = sb.as_any().downcast_ref::<StringBox>() {
                                            nyash_rust::runtime::plugin_ffi_common::encode::string(
                                                &mut buf, &s.value,
                                            );
                                            continue;
                                        }
                                    }
                                } else if p.box_type == "IntegerBox" {
                                    if let Ok(Some(ibx)) =
                                        hg.invoke_instance_method("IntegerBox", "get", p.instance_id(), &[])
                                    {
                                        if let Some(i) = ibx.as_any().downcast_ref::<IntegerBox>() {
                                            nyash_rust::runtime::plugin_ffi_common::encode::i64(
                                                &mut buf, i.value,
                                            );
                                            continue;
                                        }
                                    }
                                }
                            }
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
                        nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, v);
                    }
                } else {
                    nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, 0);
                }
            }
            _ => nyash_rust::runtime::plugin_ffi_common::encode::i64(&mut buf, v),
        }
    }
    buf
}

fn resolve_method_name_from_arg(a1: i64) -> Option<String> {
    use nyash_rust::box_trait::StringBox;
    use nyash_rust::runtime::plugin_loader_v2::PluginBoxV2;

    if a1 <= 0 {
        return None;
    }
    if let Some(obj) = nyash_rust::runtime::host_handles::get(a1 as u64) {
        if let Some(p) = obj.as_any().downcast_ref::<PluginBoxV2>() {
            if p.box_type == "StringBox" {
                if let Ok(hg) = nyash_rust::runtime::get_global_plugin_host().read() {
                    if let Ok(Some(sb)) =
                        hg.invoke_instance_method("StringBox", "toUtf8", p.instance_id(), &[])
                    {
                        if let Some(s) = sb.as_any().downcast_ref::<StringBox>() {
                            return Some(s.value.clone());
                        }
                    }
                }
            }
        }
    }
    let cptr = a1 as *const i8;
    if cptr.is_null() {
        return None;
    }
    // SAFETY: caller passes C string pointer in LLVM path.
    unsafe { std::ffi::CStr::from_ptr(cptr).to_str().ok().map(|s| s.to_string()) }
}

fn append_spawn_instance_payload_tlv(buf: &mut Vec<u8>, nargs_payload: usize, a2: i64) -> bool {
    if nargs_payload >= 1 {
        crate::encode::nyrt_encode_arg(buf, a2);
    }
    if nargs_payload > 1 {
        if !super::invoke_core::encode_legacy_args_with_failfast_policy(buf, 3, nargs_payload) {
            return false;
        }
    }
    true
}

fn parse_plugin_handle_payload(payload: &[u8]) -> Option<(u32, u32)> {
    if payload.len() != 8 {
        return None;
    }
    let mut t = [0u8; 4];
    t.copy_from_slice(&payload[0..4]);
    let mut i = [0u8; 4];
    i.copy_from_slice(&payload[4..8]);
    Some((u32::from_le_bytes(t), u32::from_le_bytes(i)))
}

fn set_future_result_from_tlv<F>(
    fut_box: &std::sync::Arc<nyash_rust::boxes::future::FutureBox>,
    slice: &[u8],
    resolve_plugin_handle: F,
) -> bool
where
    F: Fn(u32, u32) -> Option<Box<dyn nyash_rust::box_trait::NyashBox>>,
{
    use nyash_rust::box_trait::{BoolBox, IntegerBox, StringBox, VoidBox};

    let Some((tag, _sz, payload)) = nyash_rust::runtime::plugin_ffi_common::decode::tlv_first(slice)
    else {
        return false;
    };
    match tag {
        2 => {
            if let Some(v) = nyash_rust::runtime::plugin_ffi_common::decode::i32(payload) {
                fut_box.set_result(Box::new(IntegerBox::new(v as i64)));
                return true;
            }
        }
        3 => {
            if payload.len() == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload);
                let n = i64::from_le_bytes(b);
                fut_box.set_result(Box::new(IntegerBox::new(n)));
                return true;
            }
        }
        1 => {
            let v = nyash_rust::runtime::plugin_ffi_common::decode::bool(payload).unwrap_or(false);
            fut_box.set_result(Box::new(BoolBox::new(v)));
            return true;
        }
        5 => {
            if payload.len() == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload);
                let f = f64::from_le_bytes(b);
                fut_box.set_result(Box::new(nyash_rust::boxes::math_box::FloatBox::new(f)));
                return true;
            }
        }
        6 | 7 => {
            let s = nyash_rust::runtime::plugin_ffi_common::decode::string(payload);
            fut_box.set_result(Box::new(StringBox::new(s)));
            return true;
        }
        8 => {
            if let Some((r_type, r_inst)) = parse_plugin_handle_payload(payload) {
                if let Some(pb) = resolve_plugin_handle(r_type, r_inst) {
                    fut_box.set_result(pb);
                    return true;
                }
            }
        }
        9 => {
            fut_box.set_result(Box::new(VoidBox::new()));
            return true;
        }
        _ => {}
    }
    false
}

// Exported as: nyash.future.spawn_method_h(type_id, method_id, argc, recv_h, vals*, tags*) -> i64 (FutureBox handle)
#[export_name = "nyash.future.spawn_method_h"]
pub extern "C" fn nyash_future_spawn_method_h(
    _type_id: i64,
    method_id: i64,
    argc: i64,
    recv_h: i64,
    vals: *const i64,
    tags: *const i64,
) -> i64 {
    use nyash_rust::box_trait::{NyashBox, StringBox};

    if recv_h <= 0 {
        return 0;
    }
    let Some(receiver) = super::invoke_core::resolve_receiver_for_a0(recv_h) else {
        return 0;
    };
    let tlv = build_spawn_method_tlv(argc, vals, tags);

    let fut_box = std::sync::Arc::new(nyash_rust::boxes::future::FutureBox::new());
    let handle =
        nyash_rust::runtime::host_handles::to_handle_arc(fut_box.clone() as std::sync::Arc<dyn NyashBox>);
    let invoke = receiver.invoke;
    nyash_rust::runtime::global_hooks::spawn_task(
        "nyash.future.spawn_method_h",
        Box::new(move || {
            let mut out = vec![0u8; 512];
            let mut out_len: usize = out.len();
            let rc = unsafe {
                invoke(
                    receiver.real_type_id,
                    method_id as u32,
                    receiver.instance_id,
                    tlv.as_ptr(),
                    tlv.len(),
                    out.as_mut_ptr(),
                    &mut out_len,
                )
            };
            if rc != 0 {
                fut_box.set_result(Box::new(StringBox::new(format!("invoke_failed rc={}", rc))));
                return;
            }
            let slice = &out[..out_len];
            if set_future_result_from_tlv(&fut_box, slice, |r_type, r_inst| {
                let (box_type_name, invoke_ptr, fini_id) =
                    super::invoke_core::resolve_invoke_route_for_type(r_type, receiver.invoke)?;
                let pb = nyash_rust::runtime::plugin_loader_v2::construct_plugin_box(
                    box_type_name,
                    r_type,
                    invoke_ptr,
                    r_inst,
                    fini_id,
                );
                Some(Box::new(pb))
            }) {
                return;
            }
            fut_box.set_result(Box::new(StringBox::new("<unknown>")));
        }),
    );
    handle as i64
}

// Simpler spawn shim for JIT: pass argc(total explicit args incl. method_name),
// receiver handle (a0), method name (a1), and first payload (a2). Extra args
// are read from legacy VM args, same as plugin_invoke3_*.
// Returns a handle (i64) to FutureBox.
#[export_name = "nyash.future.spawn_instance3_i64"]
pub extern "C" fn nyash_future_spawn_instance3_i64(a0: i64, a1: i64, a2: i64, argc: i64) -> i64 {
    use nyash_rust::box_trait::{NyashBox, StringBox};

    if let Some(v) = crate::hako_forward_bridge::call_future_spawn_instance(a0, a1, a2, argc) {
        return v;
    }
    if a0 <= 0 {
        return 0;
    }
    let Some(receiver) = super::invoke_core::resolve_named_receiver_for_handle(a0) else {
        return 0;
    };
    let box_type_name = receiver.box_type.clone();
    let Some(method_name) = resolve_method_name_from_arg(a1) else {
        return 0;
    };
    let method_id =
        super::invoke_core::resolve_method_id_for_named_receiver(&receiver, &method_name).unwrap_or(0);
    if method_id == 0 {
        // Dynamic plugins may use 0 for birth; disallow in spawn_instance path.
        return 0;
    }

    let nargs_total = argc.max(0) as usize; // includes method_name
    let nargs_payload = nargs_total.saturating_sub(1);
    let mut tlv = nyash_rust::runtime::plugin_ffi_common::encode_tlv_header(nargs_payload as u16);
    if !append_spawn_instance_payload_tlv(&mut tlv, nargs_payload, a2) {
        return 0;
    }

    let fut_box = std::sync::Arc::new(nyash_rust::boxes::future::FutureBox::new());
    let handle =
        nyash_rust::runtime::host_handles::to_handle_arc(fut_box.clone() as std::sync::Arc<dyn NyashBox>);
    nyash_rust::runtime::global_hooks::spawn_task(
        "nyash.future.spawn_instance3_i64",
        Box::new(move || {
            let mut cap: usize = 512;
            loop {
                let mut out = vec![0u8; cap];
                let mut out_len: usize = out.len();
                let rc = unsafe {
                    (receiver.invoke)(
                        receiver.real_type_id,
                        method_id as u32,
                        receiver.instance_id,
                        tlv.as_ptr(),
                        tlv.len(),
                        out.as_mut_ptr(),
                        &mut out_len,
                    )
                };
                if rc == -1 || out_len > cap {
                    cap = cap.saturating_mul(2).max(out_len + 16);
                    if cap > 1 << 20 {
                        break;
                    }
                    continue;
                }
                if rc != 0 {
                    fut_box.set_result(Box::new(StringBox::new(format!("invoke_failed rc={}", rc))));
                    return;
                }
                let slice = &out[..out_len];
                if set_future_result_from_tlv(&fut_box, slice, |r_type, r_inst| {
                    let pb = nyash_rust::runtime::plugin_loader_v2::construct_plugin_box(
                        box_type_name.clone(),
                        r_type,
                        receiver.invoke,
                        r_inst,
                        None,
                    );
                    Some(Box::new(pb))
                }) {
                    return;
                }
                fut_box.set_result(Box::new(StringBox::new("<unknown>")));
                return;
            }
        }),
    );
    handle as i64
}

// LLVM harness currently resolves ExternCall(iface="env.future", method="spawn_instance")
// to the raw symbol name `env.future.spawn_instance`.
// Provide a stable alias that forwards to the existing nyash.* export.
#[export_name = "env.future.spawn_instance"]
pub extern "C" fn env_future_spawn_instance(a0: i64, a1: i64, a2: i64, argc: i64) -> i64 {
    nyash_future_spawn_instance3_i64(a0, a1, a2, argc)
}

#[export_name = "env.future.new"]
pub extern "C" fn env_future_new(value: i64) -> i64 {
    use nyash_rust::box_trait::{IntegerBox, NyashBox};
    use nyash_rust::runtime::host_handles;

    let fut_box = nyash_rust::boxes::future::FutureBox::new();
    let handle =
        host_handles::to_handle_arc(std::sync::Arc::new(fut_box.clone()) as std::sync::Arc<dyn NyashBox>)
            as i64;

    let boxed: Box<dyn NyashBox> = match host_handles::get(value as u64) {
        Some(obj) => obj.clone_box(),
        None => Box::new(IntegerBox::new(value)),
    };
    fut_box.set_result(boxed);
    handle
}

#[export_name = "env.future.set"]
pub extern "C" fn env_future_set(fut_handle: i64, value: i64) -> i64 {
    use nyash_rust::box_trait::{IntegerBox, NyashBox};
    use nyash_rust::runtime::host_handles;

    let boxed: Box<dyn NyashBox> = match host_handles::get(value as u64) {
        Some(obj) => obj.clone_box(),
        None => Box::new(IntegerBox::new(value)),
    };

    if let Some(obj) = host_handles::get(fut_handle as u64) {
        if let Some(fut) = obj.as_any().downcast_ref::<nyash_rust::boxes::future::FutureBox>() {
            fut.set_result(boxed);
        }
    }
    0
}

#[export_name = "env.future.await"]
pub extern "C" fn env_future_await(fut_handle: i64) -> i64 {
    use nyash_rust::box_trait::{BoolBox, IntegerBox, NyashBox, VoidBox};
    use nyash_rust::runtime::host_handles;

    let Some(obj) = host_handles::get(fut_handle as u64) else {
        return 0;
    };
    let Some(fut) = obj.as_any().downcast_ref::<nyash_rust::boxes::future::FutureBox>() else {
        return 0;
    };

    let v = fut.get();
    if let Some(ib) = v.as_any().downcast_ref::<IntegerBox>() {
        return ib.value;
    }
    if let Some(bb) = v.as_any().downcast_ref::<BoolBox>() {
        return if bb.value { 1 } else { 0 };
    }
    if v.as_any().downcast_ref::<VoidBox>().is_some() {
        return 0;
    }
    let arc: std::sync::Arc<dyn NyashBox> = v.into();
    host_handles::to_handle_arc(arc) as i64
}

// env.future.delay(ms) -> Future
//
// Lowered by the LLVM harness to `nyash.future.delay_i64` (see extern_normalize.py).
#[export_name = "nyash.future.delay_i64"]
pub extern "C" fn nyash_future_delay_i64(ms: i64) -> i64 {
    use nyash_rust::box_trait::{NyashBox, VoidBox};
    use std::time::Duration;

    let fut_box = nyash_rust::boxes::future::FutureBox::new();
    let handle = nyash_rust::runtime::host_handles::to_handle_arc(
        std::sync::Arc::new(fut_box.clone()) as std::sync::Arc<dyn NyashBox>
    );
    let fut = fut_box.clone();
    let ms_u64 = ms.max(0) as u64;
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(ms_u64));
        fut.set_result(Box::new(VoidBox::new()));
    });
    handle as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn future_spawn_method_invalid_receiver_returns_zero() {
        assert_eq!(
            nyash_future_spawn_method_h(0, 0, 0, 0, std::ptr::null(), std::ptr::null()),
            0
        );
    }

    #[test]
    fn future_spawn_instance3_invalid_receiver_returns_zero() {
        assert_eq!(nyash_future_spawn_instance3_i64(0, 0, 0, 0), 0);
        assert_eq!(nyash_future_spawn_instance3_i64(-1, 0, 0, 1), 0);
    }

    #[test]
    fn env_future_invalid_handle_paths_return_zero() {
        assert_eq!(env_future_set(0, 1), 0);
        assert_eq!(env_future_await(0), 0);
    }

    #[test]
    fn future_spawn_instance_prefers_hako_forward_hook_when_registered() {
        extern "C" fn future_hook(_a0: i64, _a1: i64, _a2: i64, _argc: i64) -> i64 {
            777
        }

        crate::hako_forward_bridge::with_test_reset(|| {
            assert_eq!(
                crate::hako_forward_bridge::register_future_spawn_instance(Some(future_hook)),
                1
            );
            assert_eq!(nyash_future_spawn_instance3_i64(0, 0, 0, 0), 777);
        });
    }
}
