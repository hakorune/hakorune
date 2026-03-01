//! Method resolution system for plugin loader v2
//!
//! This module handles all method ID resolution, method handle resolution,
//! and metadata queries for plugin methods.

use crate::bid::{BidError, BidResult};
use crate::runtime::plugin_loader_v2::enabled::PluginLoaderV2;

impl PluginLoaderV2 {
    /// Resolve a method ID for a given box type and method name
    pub(crate) fn resolve_method_id(&self, box_type: &str, method_name: &str) -> BidResult<u32> {
        if let Ok((lib_name, _type_id)) = super::route_resolver::resolve_type_info(self, box_type) {
            return super::route_resolver::resolve_method_id_for_lib(
                self,
                &lib_name,
                box_type,
                method_name,
            );
        }
        // Legacy file-based fallback is compat-only.
        if !crate::config::env::fail_fast() {
            if crate::config::env::dev_provider_trace() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[provider/trace] compat legacy file fallback box_type={} method={}",
                    box_type, method_name
                ));
            }
            return super::compat_method_resolver::resolve_method_id_from_file(box_type, method_name);
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

    /// Check if a method returns a Result type
    pub fn method_returns_result(&self, box_type: &str, method_name: &str) -> bool {
        if let Ok((lib_name, _type_id)) = super::route_resolver::resolve_type_info(self, box_type) {
            if let Ok(returns_result) = super::route_resolver::resolve_method_returns_result_for_lib(
                self,
                &lib_name,
                box_type,
                method_name,
            ) {
                return returns_result;
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
        let contract =
            super::route_resolver::resolve_method_contract(self, box_type, method_name)?;
        Ok((contract.type_id, contract.method_id, contract.returns_result))
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
