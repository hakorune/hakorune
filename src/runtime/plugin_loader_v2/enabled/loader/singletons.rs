use super::PluginLoaderV2;
use crate::bid::{BidError, BidResult};
use crate::runtime::get_global_ring0;
use crate::runtime::plugin_loader_v2::enabled::{host_bridge, types};

pub(super) fn prebirth_singletons(loader: &PluginLoaderV2) -> BidResult<()> {
    let config = loader.config.as_ref().ok_or(BidError::PluginError)?;
    let toml_value = loader.config_toml.as_ref().ok_or(BidError::PluginError)?;
    for (lib_name, lib_def) in &config.libraries {
        for box_name in &lib_def.boxes {
            if let Some(bc) = config.get_box_config(lib_name, box_name, toml_value) {
                if bc.singleton {
                    let _ = ensure_singleton_handle(loader, lib_name, box_name);
                }
            }
        }
    }
    Ok(())
}

pub(super) fn ensure_singleton_handle(
    loader: &PluginLoaderV2,
    lib_name: &str,
    box_type: &str,
) -> BidResult<()> {
    let singleton_key = (lib_name.to_string(), box_type.to_string());
    if loader
        .singletons
        .read()
        .map_err(|_| BidError::PluginError)?
        .contains_key(&singleton_key)
    {
        return Ok(());
    }
    let plugins = loader.plugins.read().map_err(|_| BidError::PluginError)?;
    let _plugin = plugins.get(lib_name).ok_or(BidError::PluginError)?;

    let birth_contract =
        super::super::route_resolver::resolve_birth_contract_for_lib(loader, lib_name, box_type)?;
    let type_id = birth_contract.type_id;
    let fini_id = birth_contract.fini_id;

    let tlv_args = crate::runtime::plugin_ffi_common::encode_empty_args();
    let route = super::super::route_resolver::resolve_invoke_route_contract(loader, type_id);
    let (status, _, out_vec) = host_bridge::invoke_alloc_with_route(
        route.invoke_box_fn,
        route.invoke_shim_fn,
        type_id,
        0,
        0,
        &tlv_args,
    );
    if status != 0 || out_vec.len() < 4 {
        if super::util::dbg_on() {
            get_global_ring0().log.debug(&format!(
                "[plugin/route:singleton_fail] lib={} box={} type_id={} status={} out_len={}",
                lib_name,
                box_type,
                type_id,
                status,
                out_vec.len()
            ));
        }
        return Err(BidError::PluginError);
    }
    let instance_id = u32::from_le_bytes([out_vec[0], out_vec[1], out_vec[2], out_vec[3]]);
    let handle = Arc::new(types::PluginHandleInner {
        type_id,
        invoke_fn: route.invoke_shim_fn,
        invoke_box_fn: route.invoke_box_fn,
        instance_id,
        fini_method_id: fini_id,
        finalized: std::sync::atomic::AtomicBool::new(false),
    });
    loader
        .singletons
        .write()
        .map_err(|_| BidError::PluginError)?
        .insert(singleton_key, handle);
    crate::runtime::cache_versions::bump_version(&format!("BoxRef:{}", box_type));
    Ok(())
}

use std::sync::Arc;
