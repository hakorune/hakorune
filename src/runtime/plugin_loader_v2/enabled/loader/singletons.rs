use super::PluginLoaderV2;
use crate::bid::{BidError, BidResult};
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

    let (resolved_lib, type_id) = super::super::route_resolver::resolve_type_info(loader, box_type)?;
    if resolved_lib != lib_name {
        return Err(BidError::InvalidType);
    }
    let (_birth_id, fini_id) =
        super::super::route_resolver::resolve_birth_and_fini_for_lib(loader, lib_name, box_type)?;

    let tlv_args = crate::runtime::plugin_ffi_common::encode_empty_args();
    let invoke_box = loader.box_invoke_fn_for_type_id(type_id);
    let (status, _, out_vec) = host_bridge::invoke_alloc_with_route(
        invoke_box,
        super::super::nyash_plugin_invoke_v2_shim,
        type_id,
        0,
        0,
        &tlv_args,
    );
    if status != 0 || out_vec.len() < 4 {
        return Err(BidError::PluginError);
    }
    let instance_id = u32::from_le_bytes([out_vec[0], out_vec[1], out_vec[2], out_vec[3]]);
    let handle = Arc::new(types::PluginHandleInner {
        type_id,
        invoke_fn: super::super::nyash_plugin_invoke_v2_shim,
        invoke_box_fn: loader.box_invoke_fn_for_type_id(type_id),
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
