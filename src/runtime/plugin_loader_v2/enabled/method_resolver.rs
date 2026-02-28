//! Method resolution system for plugin loader v2
//!
//! This module handles all method ID resolution, method handle resolution,
//! and metadata queries for plugin methods.

use crate::bid::{BidError, BidResult};
use crate::runtime::plugin_loader_v2::enabled::PluginLoaderV2;

impl PluginLoaderV2 {
    /// Resolve a method ID for a given box type and method name
    pub(crate) fn resolve_method_id(&self, box_type: &str, method_name: &str) -> BidResult<u32> {
        // First try config mapping
        if let (Some(cfg), Some(toml_value)) = (self.config.as_ref(), self.config_toml.as_ref()) {
            // Find library for box
            if let Some((lib_name, _)) = cfg.find_library_for_box(box_type) {
                if let Some(box_conf) = cfg.get_box_config(lib_name, box_type, &toml_value) {
                    if let Some(method_spec) = box_conf.methods.get(method_name) {
                        return Ok(method_spec.method_id);
                    }
                }
            }
        }

        // Fallback to TypeBox FFI spec (deterministic selection)
        if let Ok(map) = self.box_specs.read() {
            let mut candidates: Vec<(&str, u32, &str)> = Vec::new();

            for ((lib, bt), spec) in map.iter() {
                if bt != box_type {
                    continue;
                }
                if let Some(ms) = spec.methods.get(method_name) {
                    candidates.push((lib.as_str(), ms.method_id, "spec"));
                    continue;
                }
                if let Some(res_fn) = spec.resolve_fn {
                    if let Ok(cstr) = std::ffi::CString::new(method_name) {
                        let mid = res_fn(cstr.as_ptr());
                        if mid != 0 {
                            candidates.push((lib.as_str(), mid, "resolve_fn"));
                        }
                    }
                }
            }

            if !candidates.is_empty() {
                candidates.sort_by(|a, b| a.0.cmp(b.0));
                if crate::config::env::dev_provider_trace() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[provider/trace] request box_type={} method={} candidates={}",
                        box_type,
                        method_name,
                        candidates.len()
                    ));
                    for (lib, mid, src) in &candidates {
                        ring0.log.debug(&format!(
                            "[provider/trace] candidate lib={} method_id={} source={}",
                            lib, mid, src
                        ));
                    }
                    ring0.log.debug(&format!(
                        "[provider/trace] select lib={} method_id={} source={} rule=lex",
                        candidates[0].0, candidates[0].1, candidates[0].2
                    ));
                }
                return Ok(candidates[0].1);
            }
        }

        // Legacy file-based fallback is compat-only.
        // In fail-fast mode (default), unresolved methods must fail explicitly.
        if !crate::config::env::fail_fast() {
            return self.resolve_method_id_from_file(box_type, method_name);
        }

        if crate::config::env::dev_provider_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[provider/trace] reject legacy file fallback box_type={} method={} reason=fail_fast",
                box_type, method_name
            ));
        }
        Err(BidError::InvalidMethod)
    }

    /// Resolve method ID from file (legacy fallback)
    fn resolve_method_id_from_file(&self, box_type: &str, method_name: &str) -> BidResult<u32> {
        // Legacy file-based resolution (to be deprecated)
        match (box_type, method_name) {
            ("StringBox", "concat") => Ok(102),
            ("StringBox", "upper") => Ok(103),
            ("CounterBox", "inc") => Ok(102),
            ("CounterBox", "get") => Ok(103),
            _ => Err(BidError::InvalidMethod),
        }
    }

    /// Check if a method returns a Result type
    pub fn method_returns_result(&self, box_type: &str, method_name: &str) -> bool {
        if let (Some(cfg), Some(toml_value)) = (self.config.as_ref(), self.config_toml.as_ref()) {
            if let Some((lib_name, _)) = cfg.find_library_for_box(box_type) {
                if let Some(box_conf) = cfg.get_box_config(lib_name, box_type, &toml_value) {
                    if let Some(method_spec) = box_conf.methods.get(method_name) {
                        return method_spec.returns_result;
                    }
                }
            }
        }

        // Deterministic fallback: pick by library name if multiple specs exist
        if let Ok(map) = self.box_specs.read() {
            let mut candidates: Vec<(&str, bool)> = Vec::new();
            for ((lib, bt), spec) in map.iter() {
                if bt != box_type {
                    continue;
                }
                if let Some(method_spec) = spec.methods.get(method_name) {
                    candidates.push((lib.as_str(), method_spec.returns_result));
                }
            }
            if !candidates.is_empty() {
                candidates.sort_by(|a, b| a.0.cmp(b.0));
                if crate::config::env::dev_provider_trace() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[provider/trace] request box_type={} method={} candidates={}",
                        box_type,
                        method_name,
                        candidates.len()
                    ));
                    for (lib, rr) in &candidates {
                        ring0.log.debug(&format!(
                            "[provider/trace] candidate lib={} returns_result={}",
                            lib, rr
                        ));
                    }
                    ring0.log.debug(&format!(
                        "[provider/trace] select lib={} returns_result={} rule=lex",
                        candidates[0].0, candidates[0].1
                    ));
                }
                return candidates[0].1;
            }
        }

        // Default to false for unknown methods
        false
    }

    /// Resolve (type_id, method_id, returns_result) for a box_type.method
    pub fn resolve_method_handle(
        &self,
        box_type: &str,
        method_name: &str,
    ) -> BidResult<(u32, u32, bool)> {
        let cfg = self.config.as_ref().ok_or(BidError::PluginError)?;
        let toml_value = self.config_toml.as_ref().ok_or(BidError::PluginError)?;
        let (lib_name, _) = cfg
            .find_library_for_box(box_type)
            .ok_or(BidError::InvalidType)?;
        let bc = cfg
            .get_box_config(lib_name, box_type, &toml_value)
            .ok_or(BidError::InvalidType)?;
        let m = bc.methods.get(method_name).ok_or(BidError::InvalidMethod)?;
        Ok((bc.type_id, m.method_id, m.returns_result))
    }
}

/// Helper functions for method resolution
#[allow(dead_code)]
pub(super) fn is_special_method(method_name: &str) -> bool {
    matches!(method_name, "birth" | "fini" | "toString")
}

/// Get default method IDs for special methods
#[allow(dead_code)]
pub(super) fn get_special_method_id(method_name: &str) -> Option<u32> {
    match method_name {
        "birth" => Some(1),
        "toString" => Some(100),
        "fini" => Some(999),
        _ => None,
    }
}
