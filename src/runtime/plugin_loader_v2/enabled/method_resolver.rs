//! Method resolution system for plugin loader v2
//!
//! This module handles all method ID resolution, method handle resolution,
//! and metadata queries for plugin methods.

use crate::bid::BidResult;
use crate::runtime::plugin_loader_v2::enabled::PluginLoaderV2;

impl PluginLoaderV2 {
    /// Resolve a method ID for a given box type and method name
    pub(crate) fn resolve_method_id(&self, box_type: &str, method_name: &str) -> BidResult<u32> {
        if let Ok(plan) =
            super::method_route_plan::resolve_method_call_plan(self, box_type, method_name, 0)
        {
            return Ok(plan.method_id);
        }
        super::compat_method_resolver::resolve_method_id_with_compat_policy(box_type, method_name)
    }

    /// Check if a method returns a Result type
    pub fn method_returns_result(&self, box_type: &str, method_name: &str) -> bool {
        super::method_route_plan::resolve_method_call_plan(self, box_type, method_name, 0)
            .map(|plan| plan.returns_result)
            .unwrap_or(false)
    }

    /// Resolve (type_id, method_id, returns_result) for a box_type.method
    pub fn resolve_method_handle(
        &self,
        box_type: &str,
        method_name: &str,
    ) -> BidResult<(u32, u32, bool)> {
        let plan =
            super::method_route_plan::resolve_method_call_plan(self, box_type, method_name, 0)?;
        Ok((plan.type_id, plan.method_id, plan.returns_result))
    }
}
