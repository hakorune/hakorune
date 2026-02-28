//! Instance management for plugin boxes

use crate::bid::{BidError, BidResult};
use crate::box_trait::NyashBox;
use crate::runtime::get_global_ring0;
use crate::runtime::plugin_loader_v2::enabled::{
    types::{PluginBoxV2, PluginHandleInner},
    PluginLoaderV2,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
struct ResolvedBirthContract {
    type_id: u32,
    birth_id: u32,
    fini_id: Option<u32>,
}

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
        // Non-recursive: resolve birth contract -> invoke birth -> construct PluginBoxV2
        let contract = resolve_instance_birth_contract(self, box_type)?;
        let instance_id = invoke_birth_and_decode_instance_id(box_type, contract)?;
        let bx = build_plugin_box_handle(box_type, contract, instance_id);

        // Get loaded plugin invoke
        let _plugins = self.plugins.read().map_err(|_| BidError::PluginError)?;

        // Diagnostics: register for leak tracking (optional)
        crate::runtime::leak_tracker::register_plugin(box_type, instance_id);
        Ok(Box::new(bx))
    }

    /// Shutdown singletons: finalize and clear all singleton handles
    pub fn shutdown_singletons(&self) -> BidResult<()> {
        let mut map = self
            .singletons
            .write()
            .map_err(|_| BidError::PluginError)?;
        for (_, handle) in map.drain() {
            if let Ok(inner) = Arc::try_unwrap(handle) {
                inner.finalize_now();
            }
        }
        Ok(())
    }
}

/// Resolve birth contract (type_id, birth_id, fini_id) from configuration or specs.
fn resolve_instance_birth_contract(
    loader: &PluginLoaderV2,
    box_type: &str,
) -> BidResult<ResolvedBirthContract> {
    let (lib_name, type_id) = super::route_resolver::resolve_type_info(loader, box_type)?;
    let (birth_id, fini_id) =
        super::route_resolver::resolve_birth_and_fini_for_lib(loader, &lib_name, box_type)?;

    Ok(ResolvedBirthContract {
        type_id,
        birth_id,
        fini_id,
    })
}

/// Invoke plugin birth and decode returned instance id from first 4 bytes (little-endian).
fn invoke_birth_and_decode_instance_id(
    box_type: &str,
    contract: ResolvedBirthContract,
) -> BidResult<u32> {
    if dbg_on() {
        get_global_ring0().log.debug(&format!(
            "[PluginLoaderV2] invoking birth: box_type={} type_id={} birth_id={}",
            box_type, contract.type_id, contract.birth_id
        ));
    }

    let tlv = crate::runtime::plugin_ffi_common::encode_empty_args();
    let invoke_box = super::super::box_invoke_for_type_id(contract.type_id);
    let (code, out_len, out_buf) = super::host_bridge::invoke_alloc_with_route(
        invoke_box,
        super::super::nyash_plugin_invoke_v2_shim,
        contract.type_id,
        contract.birth_id,
        0,
        &tlv,
    );

    if dbg_on() {
        get_global_ring0().log.debug(&format!(
            "[PluginLoaderV2] create_box: box_type={} type_id={} birth_id={} code={} out_len={}",
            box_type, contract.type_id, contract.birth_id, code, out_len
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
    contract: ResolvedBirthContract,
    instance_id: u32,
) -> PluginBoxV2 {
    PluginBoxV2 {
        box_type: box_type.to_string(),
        inner: Arc::new(PluginHandleInner {
            type_id: contract.type_id,
            invoke_fn: super::super::nyash_plugin_invoke_v2_shim,
            invoke_box_fn: super::super::box_invoke_for_type_id(contract.type_id),
            instance_id,
            fini_method_id: contract.fini_id,
            finalized: std::sync::atomic::AtomicBool::new(false),
        }),
    }
}
