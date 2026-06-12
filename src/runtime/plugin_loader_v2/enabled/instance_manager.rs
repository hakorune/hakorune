//! Instance management for plugin boxes

use crate::bid::{BidError, BidResult};
use crate::box_trait::NyashBox;
use crate::runtime::get_global_ring0;
use crate::runtime::plugin_loader_v2::enabled::{
    types::{PluginBoxV2, PluginHandleInner},
    PluginLoaderV2,
};
use std::sync::Arc;

use super::lifecycle_route_plan::{
    invoke_plugin_lifecycle, resolve_newbox_lifecycle_plan, PluginNewBoxExecutionPlan,
};

fn dbg_on() -> bool {
    std::env::var("PLUGIN_DEBUG").is_ok()
}

impl PluginLoaderV2 {
    /// Create a new plugin box instance
    pub fn create_box(
        &self,
        box_type: &str,
        _args: &[Box<dyn NyashBox>],
    ) -> BidResult<Box<dyn NyashBox>> {
        // Non-recursive: registry lifecycle target -> route plan -> invoke birth.
        let plan = resolve_newbox_lifecycle_plan(self, box_type)?;
        let instance_id = invoke_birth_and_decode_instance_id(box_type, plan)?;
        let bx = build_plugin_box_handle(box_type, plan, instance_id);

        // Get loaded plugin invoke
        let _plugins = self.plugins.read().map_err(|_| BidError::PluginError)?;

        // Diagnostics: register for leak tracking (optional)
        crate::runtime::leak_tracker::register_plugin(box_type, instance_id);
        Ok(Box::new(bx))
    }

    /// Shutdown singletons: finalize and clear all singleton handles
    pub fn shutdown_singletons(&self) -> BidResult<()> {
        let mut map = self.singletons.write().map_err(|_| BidError::PluginError)?;
        for (_, handle) in map.drain() {
            if let Ok(inner) = Arc::try_unwrap(handle) {
                inner.finalize_now();
            }
        }
        Ok(())
    }
}

/// Invoke plugin birth and decode returned instance id from first 4 bytes (little-endian).
fn invoke_birth_and_decode_instance_id(
    box_type: &str,
    plan: PluginNewBoxExecutionPlan,
) -> BidResult<u32> {
    if dbg_on() {
        get_global_ring0().log.debug(&format!(
            "[PluginLoaderV2] invoking birth: box_type={} type_id={} birth_id={}",
            box_type, plan.type_id, plan.birth_id
        ));
    }

    let tlv = crate::runtime::plugin_ffi_common::encode_empty_args();
    let (code, out_len, out_buf) =
        invoke_plugin_lifecycle(plan.invoke_route, plan.type_id, plan.birth_id, 0, &tlv);

    if dbg_on() {
        get_global_ring0().log.debug(&format!(
            "[PluginLoaderV2] create_box: box_type={} type_id={} birth_id={} code={} out_len={}",
            box_type, plan.type_id, plan.birth_id, code, out_len
        ));
        if out_len > 0 {
            get_global_ring0().log.debug(&format!(
                "[PluginLoaderV2] create_box: out[0..min(8)]={:02x?}",
                &out_buf[..out_len.min(8)]
            ));
        }
    }

    if code != 0 || out_len < 4 {
        return Err(BidError::PluginError);
    }

    Ok(u32::from_le_bytes([
        out_buf[0], out_buf[1], out_buf[2], out_buf[3],
    ]))
}

/// Build a PluginBoxV2 handle from resolved birth contract and created instance id.
fn build_plugin_box_handle(
    box_type: &str,
    plan: PluginNewBoxExecutionPlan,
    instance_id: u32,
) -> PluginBoxV2 {
    PluginBoxV2 {
        box_type: box_type.to_string(),
        inner: Arc::new(PluginHandleInner {
            type_id: plan.type_id,
            invoke_fn: plan.invoke_route.invoke_shim_fn,
            invoke_box_fn: plan.invoke_route.invoke_box_fn,
            allow_compat_shim: plan.invoke_route.allow_compat_shim,
            instance_id,
            fini_method_id: plan.fini_id,
            finalized: std::sync::atomic::AtomicBool::new(false),
        }),
    }
}
