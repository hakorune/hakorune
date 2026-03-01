//! Compat/dev-only helpers for ffi_bridge.
//!
//! Mainline invoke flow should stay small; optional tracing/probing paths live here.

use crate::box_trait::NyashBox;
use crate::runtime::get_global_ring0;
use std::env;

pub(super) fn maybe_probe_c_wrap(box_type: &str, method_name: &str) {
    if env::var("HAKO_PLUGIN_LOADER_C_WRAP").ok().as_deref() == Some("1") {
        if should_trace_cwrap(box_type, method_name) {
            get_global_ring0()
                .log
                .debug(&format!("[cwrap:invoke:{}.{}]", box_type, method_name));
        }
    }
}

pub(super) fn maybe_probe_c_core(
    box_type: &str,
    method_name: &str,
    _args: &[Box<dyn NyashBox>],
    _type_id: u32,
    _instance_id: u32,
) {
    if env::var("HAKO_C_CORE_ENABLE").ok().as_deref() != Some("1") {
        return;
    }
    if !should_route_ccore(box_type, method_name) {
        return;
    }

    get_global_ring0()
        .log
        .debug(&format!("[c-core:invoke:{}.{}]", box_type, method_name));
    #[cfg(feature = "c-core")]
    {
        let args = _args;
        let type_id = _type_id;
        let instance_id = _instance_id;
        if box_type == "MapBox" && method_name == "set" {
            let key = args
                .first()
                .map(|b| b.to_string_box().value)
                .unwrap_or_default();
            let val = args
                .get(1)
                .map(|b| b.to_string_box().value)
                .unwrap_or_default();
            let _ = nyash_c_core::core_map_set(type_id as i32, instance_id, &key, &val);
        } else if box_type == "ArrayBox" && method_name == "push" {
            let _ = nyash_c_core::core_array_push(type_id as i32, instance_id, 0);
        } else if box_type == "ArrayBox" && method_name == "get" {
            let _ = nyash_c_core::core_array_get(type_id as i32, instance_id, 0);
        } else if box_type == "ArrayBox"
            && (method_name == "size" || method_name == "len" || method_name == "length")
        {
            let _ = nyash_c_core::core_array_len(type_id as i32, instance_id);
        } else {
            let _ = nyash_c_core::core_probe_invoke(box_type, method_name, args.len() as i32);
        }
    }
}

pub(super) fn maybe_trace_call(box_type: &str, method_name: &str) {
    if env::var("HAKO_CALL_TRACE").ok().as_deref() != Some("1") {
        return;
    }
    if should_trace_call(box_type, method_name) {
        get_global_ring0()
            .log
            .debug(&format!("[call:{}.{}]", box_type, method_name));
    }
}

pub(super) fn maybe_trace_tlv_shim(box_type: &str, method_name: &str, argc: usize) {
    if env::var("HAKO_TLV_SHIM_TRACE").ok().as_deref() != Some("1") {
        return;
    }
    if env::var("HAKO_TLV_SHIM").ok().as_deref() != Some("1") {
        return;
    }
    if !should_trace_tlv_shim(box_type, method_name) {
        return;
    }

    get_global_ring0()
        .log
        .debug(&format!("[tlv/shim:{}.{}]", box_type, method_name));
    if env::var("HAKO_TLV_SHIM_TRACE_DETAIL").ok().as_deref() == Some("1") {
        get_global_ring0()
            .log
            .debug(&format!("[tlv/shim:detail argc={}]", argc));
    }
}

fn should_trace_tlv_shim(box_type: &str, method: &str) -> bool {
    if let Ok(flt) = env::var("HAKO_TLV_SHIM_FILTER") {
        let key = format!("{}.{}", box_type, method);
        for pat in flt.split(',') {
            let p = pat.trim();
            if p.is_empty() {
                continue;
            }
            if p == method || p == key {
                return true;
            }
        }
        return false;
    }
    box_type == "MapBox" && method == "set"
}

fn should_trace_cwrap(box_type: &str, method: &str) -> bool {
    if let Ok(flt) = env::var("HAKO_PLUGIN_LOADER_C_WRAP_FILTER") {
        let key = format!("{}.{}", box_type, method);
        for pat in flt.split(',') {
            let p = pat.trim();
            if p.is_empty() {
                continue;
            }
            if p == method || p == key {
                return true;
            }
        }
        return false;
    }
    box_type == "MapBox" && method == "set"
}

fn should_trace_call(target: &str, method: &str) -> bool {
    if let Ok(flt) = env::var("HAKO_CALL_TRACE_FILTER") {
        let key = format!("{}.{}", target, method);
        for pat in flt.split(',') {
            let p = pat.trim();
            if p.is_empty() {
                continue;
            }
            if p == method || p == key {
                return true;
            }
        }
        return false;
    }
    true
}

fn should_route_ccore(box_type: &str, method: &str) -> bool {
    if let Ok(flt) = env::var("HAKO_C_CORE_TARGETS") {
        let key = format!("{}.{}", box_type, method);
        for pat in flt.split(',') {
            let p = pat.trim();
            if p.is_empty() {
                continue;
            }
            if p == method || p == key {
                return true;
            }
        }
        return false;
    }
    box_type == "MapBox" && method == "set"
}
