//! FFI bridge for plugin method invocation and TLV encoding/decoding

use crate::bid::{BidError, BidResult};
use crate::box_trait::NyashBox;
use crate::runtime::get_global_ring0;
use crate::runtime::plugin_loader_v2::enabled::PluginLoaderV2;
use std::env;
use std::sync::Arc;

fn dbg_on() -> bool {
    std::env::var("PLUGIN_DEBUG").is_ok()
}

impl PluginLoaderV2 {
    /// Invoke a method on a plugin instance with TLV encoding/decoding
    pub fn invoke_instance_method(
        &self,
        box_type: &str,
        method_name: &str,
        instance_id: u32,
        args: &[Box<dyn NyashBox>],
    ) -> BidResult<Option<Box<dyn NyashBox>>> {
        // Resolve (lib_name, type_id) either from config or cached specs
        let (lib_name, type_id) = resolve_type_info(self, box_type)?;

        // Mainline path: resolve from selected library config/spec only.
        // This avoids route drift into legacy resolver fallback paths.
        let method_id = resolve_method_id_for_lib(self, &lib_name, box_type, method_name)?;

        // Get plugin handle
        let plugins = self.plugins.read().map_err(|_| BidError::PluginError)?;
        let _plugin = plugins.get(&lib_name).ok_or(BidError::PluginError)?;

        // Optional C wrapper (Phase 22.2: design insertion point; default OFF)
        if env::var("HAKO_PLUGIN_LOADER_C_WRAP").ok().as_deref() == Some("1") {
            if should_trace_cwrap(box_type, method_name) {
                get_global_ring0()
                    .log
                    .debug(&format!("[cwrap:invoke:{}.{}]", box_type, method_name));
            }
            // Future: route into a thin C shim here. For now, fall through to normal path.
        }

        // Optional C-core probe (design): emit tag and optionally call into c-core when enabled
        if env::var("HAKO_C_CORE_ENABLE").ok().as_deref() == Some("1")
            && should_route_ccore(box_type, method_name)
        {
            get_global_ring0()
                .log
                .debug(&format!("[c-core:invoke:{}.{}]", box_type, method_name));
            #[cfg(feature = "c-core")]
            {
                // MapBox.set: call C-core stub (no-op) with available info
                if box_type == "MapBox" && method_name == "set" {
                    let key = args
                        .get(0)
                        .map(|b| b.to_string_box().value)
                        .unwrap_or_default();
                    let val = args
                        .get(1)
                        .map(|b| b.to_string_box().value)
                        .unwrap_or_default();
                    let _ = nyash_c_core::core_map_set(type_id as i32, instance_id, &key, &val);
                } else if box_type == "ArrayBox" && method_name == "push" {
                    // For design stage, pass 0 (we don't rely on c-core result)
                    let _ = nyash_c_core::core_array_push(type_id as i32, instance_id, 0);
                } else if box_type == "ArrayBox" && method_name == "get" {
                    let _ = nyash_c_core::core_array_get(type_id as i32, instance_id, 0);
                } else if box_type == "ArrayBox"
                    && (method_name == "size" || method_name == "len" || method_name == "length")
                {
                    let _ = nyash_c_core::core_array_len(type_id as i32, instance_id);
                } else {
                    // Generic probe
                    let _ =
                        nyash_c_core::core_probe_invoke(box_type, method_name, args.len() as i32);
                }
            }
        }

        // Encode TLV args via shared helper (numeric→string→toString)
        let tlv = crate::runtime::plugin_ffi_common::encode_args(args);

        // Unified call trace (optional): plugin calls
        if env::var("HAKO_CALL_TRACE").ok().as_deref() == Some("1") {
            if should_trace_call(box_type, method_name) {
                get_global_ring0()
                    .log
                    .debug(&format!("[call:{}.{}]", box_type, method_name));
            }
        }

        // Optional trace for TLV shim path (debug only; default OFF)
        if env::var("HAKO_TLV_SHIM_TRACE").ok().as_deref() == Some("1")
            && env::var("HAKO_TLV_SHIM").ok().as_deref() == Some("1")
        {
            if should_trace_tlv_shim(box_type, method_name) {
                get_global_ring0()
                    .log
                    .debug(&format!("[tlv/shim:{}.{}]", box_type, method_name));
                if env::var("HAKO_TLV_SHIM_TRACE_DETAIL").ok().as_deref() == Some("1") {
                    get_global_ring0()
                        .log
                        .debug(&format!("[tlv/shim:detail argc={}]", args.len()));
                }
            }
        }

        if dbg_on() {
            get_global_ring0().log.debug(&format!(
                "[PluginLoaderV2] call {}.{}: type_id={} method_id={} instance_id={}",
                box_type, method_name, type_id, method_id, instance_id
            ));
        }

        let invoke_box = self.box_invoke_fn_for_type_id(type_id);
        let (_code, out_len, out) = super::host_bridge::invoke_alloc_with_route(
            invoke_box,
            super::super::nyash_plugin_invoke_v2_shim,
            type_id,
            method_id,
            instance_id,
            &tlv,
        );

        // Decode TLV (first entry) generically
        decode_tlv_result(box_type, &out[..out_len])
    }
}

fn resolve_method_id_for_lib(
    loader: &PluginLoaderV2,
    lib_name: &str,
    box_type: &str,
    method_name: &str,
) -> BidResult<u32> {
    // 1) Config mapping
    if let (Some(cfg), Some(toml_value)) = (loader.config.as_ref(), loader.config_toml.as_ref()) {
        if let Some(bc) = cfg.get_box_config(lib_name, box_type, toml_value) {
            if let Some(method_spec) = bc.methods.get(method_name) {
                return Ok(method_spec.method_id);
            }
        }
    }

    // 2) TypeBox spec mapping for the selected library
    if let Ok(map) = loader.box_specs.read() {
        let key = (lib_name.to_string(), box_type.to_string());
        if let Some(spec) = map.get(&key) {
            if let Some(ms) = spec.methods.get(method_name) {
                return Ok(ms.method_id);
            }
            if let Some(res_fn) = spec.resolve_fn {
                if let Ok(cstr) = std::ffi::CString::new(method_name) {
                    let mid = res_fn(cstr.as_ptr());
                    if mid != 0 {
                        return Ok(mid);
                    }
                }
            }
        }
    }

    if dbg_on() {
        get_global_ring0().log.debug(&format!(
            "[PluginLoaderV2] ERR: method resolve failed for {}.{} in lib={}",
            box_type, method_name, lib_name
        ));
    }
    Err(BidError::InvalidMethod)
}

fn should_trace_tlv_shim(box_type: &str, method: &str) -> bool {
    // Filter provided → honor it
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
    // Default (minimal noise): only trace MapBox.set to begin with
    box_type == "MapBox" && method == "set"
}

fn should_trace_cwrap(box_type: &str, method: &str) -> bool {
    // Filter provided → honor it
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
    // Default (minimal noise): only trace MapBox.set to begin with
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
    // Default minimal scope: MapBox.set only
    box_type == "MapBox" && method == "set"
}

/// Resolve type information for a box
fn resolve_type_info(loader: &PluginLoaderV2, box_type: &str) -> BidResult<(String, u32)> {
    if let (Some(cfg), Some(toml_value)) = (loader.config.as_ref(), loader.config_toml.as_ref()) {
        if let Some((lib_name, _)) = cfg.find_library_for_box(box_type) {
            if let Some(bc) = cfg.get_box_config(lib_name, box_type, &toml_value) {
                return Ok((lib_name.to_string(), bc.type_id));
            } else {
                let key = (lib_name.to_string(), box_type.to_string());
                let map = loader.box_specs.read().map_err(|_| BidError::PluginError)?;
                let tid = map
                    .get(&key)
                    .and_then(|s| s.type_id)
                    .ok_or(BidError::InvalidType)?;
                return Ok((lib_name.to_string(), tid));
            }
        }
    }

    // Compat-only fallback: if config is absent and fail-fast is relaxed, choose deterministic lib.
    if !crate::config::env::fail_fast() {
        let map = loader.box_specs.read().map_err(|_| BidError::PluginError)?;
        let mut cands: Vec<(String, u32)> = map
            .iter()
            .filter(|((_, bt), _)| bt == box_type)
            .filter_map(|((lib, _), spec)| spec.type_id.map(|tid| (lib.clone(), tid)))
            .collect();
        if !cands.is_empty() {
            cands.sort_by(|a, b| a.0.cmp(&b.0));
            return Ok(cands[0].clone());
        }
    }
    Err(BidError::InvalidType)
}

/// Decode TLV result into a NyashBox
fn decode_tlv_result(box_type: &str, data: &[u8]) -> BidResult<Option<Box<dyn NyashBox>>> {
    if let Some((tag, _sz, payload)) = crate::runtime::plugin_ffi_common::decode::tlv_first(data) {
        let bx: Box<dyn NyashBox> = match tag {
            1 => Box::new(crate::box_trait::BoolBox::new(
                crate::runtime::plugin_ffi_common::decode::bool(payload).unwrap_or(false),
            )),
            2 => Box::new(crate::box_trait::IntegerBox::new(
                crate::runtime::plugin_ffi_common::decode::i32(payload).unwrap_or(0) as i64,
            )),
            3 => {
                // i64 payload
                if payload.len() == 8 {
                    let mut b = [0u8; 8];
                    b.copy_from_slice(payload);
                    Box::new(crate::box_trait::IntegerBox::new(i64::from_le_bytes(b)))
                } else {
                    Box::new(crate::box_trait::IntegerBox::new(0))
                }
            }
            5 => {
                let x = crate::runtime::plugin_ffi_common::decode::f64(payload).unwrap_or(0.0);
                Box::new(crate::boxes::FloatBox::new(x))
            }
            6 | 7 => {
                let s = crate::runtime::plugin_ffi_common::decode::string(payload);
                Box::new(crate::box_trait::StringBox::new(s))
            }
            8 => {
                // Plugin handle (type_id, instance_id) → wrap into PluginBoxV2
                if let Some((ret_type, inst)) =
                    crate::runtime::plugin_ffi_common::decode::plugin_handle(payload)
                {
                    let (ret_box_type, fini_method_id) =
                        if let Some(meta) =
                            crate::runtime::plugin_loader_v2::enabled::metadata_for_type_id(
                                ret_type,
                            ) {
                            (meta.box_type, meta.fini_method_id)
                        } else {
                            (box_type.to_string(), None)
                        };
                    let handle = Arc::new(super::types::PluginHandleInner {
                        type_id: ret_type,
                        invoke_fn: super::super::nyash_plugin_invoke_v2_shim,
                        instance_id: inst,
                        fini_method_id,
                        finalized: std::sync::atomic::AtomicBool::new(false),
                    });
                    Box::new(super::types::PluginBoxV2 {
                        box_type: ret_box_type,
                        inner: handle,
                    })
                } else {
                    Box::new(crate::box_trait::VoidBox::new())
                }
            }
            9 => {
                // Host handle (u64) → try to map back to BoxRef, else void
                if let Some(u) = crate::runtime::plugin_ffi_common::decode::u64(payload) {
                    if let Some(arc) = crate::runtime::host_handles::get(u) {
                        return Ok(Some(arc.share_box()));
                    }
                }
                Box::new(crate::box_trait::VoidBox::new())
            }
            _ => Box::new(crate::box_trait::VoidBox::new()),
        };
        return Ok(Some(bx));
    }
    Ok(Some(Box::new(crate::box_trait::VoidBox::new())))
}
