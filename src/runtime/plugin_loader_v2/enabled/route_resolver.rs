//! Shared route resolver for plugin loader v2.
//!
//! Keeps config/spec/compat resolution policy in one place so bridge and
//! instance manager do not duplicate route logic.

use super::loader::PluginLoaderV2;
use super::host_bridge::{BoxInvokeFn, InvokeFn};
use crate::bid::{BidError, BidResult};

#[derive(Clone, Debug)]
pub(super) struct MethodRouteContract {
    pub lib_name: String,
    pub type_id: u32,
    pub method_id: u32,
    pub returns_result: bool,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct BirthRouteContract {
    pub type_id: u32,
    pub birth_id: u32,
    pub fini_id: Option<u32>,
}

#[derive(Clone, Copy)]
pub(super) struct InvokeRouteContract {
    pub invoke_box_fn: Option<BoxInvokeFn>,
    pub invoke_shim_fn: InvokeFn,
}

pub(super) fn resolve_lib_box_for_type_id(
    loader: &PluginLoaderV2,
    type_id: u32,
) -> Option<(String, String)> {
    if let (Some(config), Some(toml_value)) = (loader.config.as_ref(), loader.config_toml.as_ref()) {
        for (lib_name, lib_def) in &config.libraries {
            for box_name in &lib_def.boxes {
                if let Some(box_conf) = config.get_box_config(lib_name, box_name, toml_value) {
                    if box_conf.type_id == type_id {
                        return Some((lib_name.to_string(), box_name.to_string()));
                    }
                }
            }
        }
    }

    if crate::config::env::fail_fast() {
        return None;
    }

    // Compat-only fallback when config is missing:
    // choose deterministic lexical (lib, box) for this type_id.
    let map = loader.box_specs.read().ok()?;
    let mut cands: Vec<(String, String)> = map
        .iter()
        .filter_map(|((lib, bt), spec)| {
            if spec.type_id == Some(type_id) {
                Some((lib.clone(), bt.clone()))
            } else {
                None
            }
        })
        .collect();
    if cands.is_empty() {
        return None;
    }
    cands.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    cands.into_iter().next()
}

fn selected_library_name(loader: &PluginLoaderV2, box_type: &str) -> Option<String> {
    let cfg = loader.config.as_ref()?;
    let (_lib_name, _lib_def) = cfg.find_library_for_box(box_type)?;
    Some(_lib_name.to_string())
}

fn type_id_from_selected_lib(
    loader: &PluginLoaderV2,
    lib_name: &str,
    box_type: &str,
) -> BidResult<Option<u32>> {
    if let (Some(cfg), Some(toml_value)) = (loader.config.as_ref(), loader.config_toml.as_ref()) {
        if let Some(box_conf) = cfg.get_box_config(lib_name, box_type, toml_value) {
            return Ok(Some(box_conf.type_id));
        }
    }
    let map = loader.box_specs.read().map_err(|_| BidError::PluginError)?;
    Ok(map
        .get(&(lib_name.to_string(), box_type.to_string()))
        .and_then(|spec| spec.type_id))
}

pub(super) fn resolve_type_info(
    loader: &PluginLoaderV2,
    box_type: &str,
) -> BidResult<(String, u32)> {
    if let Some(lib_name) = selected_library_name(loader, box_type) {
        let type_id =
            type_id_from_selected_lib(loader, &lib_name, box_type)?.ok_or(BidError::InvalidType)?;
        return Ok((lib_name, type_id));
    }

    if crate::config::env::fail_fast() {
        return Err(BidError::InvalidType);
    }

    // Compat-only fallback when config is missing:
    // choose a deterministic lexical library for this box_type.
    let map = loader.box_specs.read().map_err(|_| BidError::PluginError)?;
    let mut cands: Vec<(String, u32)> = map
        .iter()
        .filter(|((_, bt), _)| bt == box_type)
        .filter_map(|((lib, _), spec)| spec.type_id.map(|tid| (lib.clone(), tid)))
        .collect();
    if cands.is_empty() {
        return Err(BidError::InvalidType);
    }
    cands.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(cands[0].clone())
}

pub(super) fn resolve_method_id_for_lib(
    loader: &PluginLoaderV2,
    lib_name: &str,
    box_type: &str,
    method_name: &str,
) -> BidResult<u32> {
    if let (Some(cfg), Some(toml_value)) = (loader.config.as_ref(), loader.config_toml.as_ref()) {
        if let Some(bc) = cfg.get_box_config(lib_name, box_type, toml_value) {
            if let Some(method_spec) = bc.methods.get(method_name) {
                return Ok(method_spec.method_id);
            }
        }
    }

    let map = loader.box_specs.read().map_err(|_| BidError::PluginError)?;
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

    Err(BidError::InvalidMethod)
}

pub(super) fn resolve_method_returns_result_for_lib(
    loader: &PluginLoaderV2,
    lib_name: &str,
    box_type: &str,
    method_name: &str,
) -> BidResult<bool> {
    if let (Some(cfg), Some(toml_value)) = (loader.config.as_ref(), loader.config_toml.as_ref()) {
        if let Some(bc) = cfg.get_box_config(lib_name, box_type, toml_value) {
            if let Some(method_spec) = bc.methods.get(method_name) {
                return Ok(method_spec.returns_result);
            }
        }
    }

    let map = loader.box_specs.read().map_err(|_| BidError::PluginError)?;
    let key = (lib_name.to_string(), box_type.to_string());
    if let Some(spec) = map.get(&key) {
        if let Some(ms) = spec.methods.get(method_name) {
            return Ok(ms.returns_result);
        }
    }

    Err(BidError::InvalidMethod)
}

pub(super) fn resolve_birth_and_fini_for_lib(
    loader: &PluginLoaderV2,
    lib_name: &str,
    box_type: &str,
) -> BidResult<(u32, Option<u32>)> {
    let mut birth_id = None;
    let mut fini_id = None;

    if let (Some(cfg), Some(toml_value)) = (loader.config.as_ref(), loader.config_toml.as_ref()) {
        if let Some(box_conf) = cfg.get_box_config(lib_name, box_type, toml_value) {
            birth_id = box_conf.methods.get("birth").map(|m| m.method_id);
            fini_id = box_conf.methods.get("fini").map(|m| m.method_id);
        }
    }

    if birth_id.is_none() || fini_id.is_none() {
        let map = loader.box_specs.read().map_err(|_| BidError::PluginError)?;
        let key = (lib_name.to_string(), box_type.to_string());
        if let Some(spec) = map.get(&key) {
            if birth_id.is_none() {
                if let Some(ms) = spec.methods.get("birth") {
                    birth_id = Some(ms.method_id);
                } else if let Some(res_fn) = spec.resolve_fn {
                    if let Ok(cstr) = std::ffi::CString::new("birth") {
                        let mid = res_fn(cstr.as_ptr());
                        if mid != 0 {
                            birth_id = Some(mid);
                        }
                    }
                }
            }
            if fini_id.is_none() {
                fini_id = spec.fini_method_id;
            }
        }
    }

    let birth_id = birth_id.ok_or(BidError::InvalidMethod)?;
    Ok((birth_id, fini_id))
}

pub(super) fn resolve_type_and_fini_for_lib(
    loader: &PluginLoaderV2,
    lib_name: &str,
    box_type: &str,
    fallback_type_id: u32,
) -> BidResult<(u32, Option<u32>)> {
    let mut resolved_type = type_id_from_selected_lib(loader, lib_name, box_type)?
        .unwrap_or(fallback_type_id);
    let mut fini_id = None;

    if let (Some(cfg), Some(toml_value)) = (loader.config.as_ref(), loader.config_toml.as_ref()) {
        if let Some(box_conf) = cfg.get_box_config(lib_name, box_type, toml_value) {
            if resolved_type == fallback_type_id {
                resolved_type = box_conf.type_id;
            }
            fini_id = box_conf.methods.get("fini").map(|m| m.method_id);
        }
    }

    if fini_id.is_none() {
        let map = loader.box_specs.read().map_err(|_| BidError::PluginError)?;
        let key = (lib_name.to_string(), box_type.to_string());
        if let Some(spec) = map.get(&key) {
            if resolved_type == fallback_type_id {
                if let Some(tid) = spec.type_id {
                    resolved_type = tid;
                }
            }
            fini_id = spec.fini_method_id;
        }
    }

    Ok((resolved_type, fini_id))
}

pub(super) fn resolve_method_contract(
    loader: &PluginLoaderV2,
    box_type: &str,
    method_name: &str,
) -> BidResult<MethodRouteContract> {
    let (lib_name, type_id) = resolve_type_info(loader, box_type)?;
    let method_id = resolve_method_id_for_lib(loader, &lib_name, box_type, method_name)?;
    let returns_result =
        resolve_method_returns_result_for_lib(loader, &lib_name, box_type, method_name)
            .unwrap_or(false);
    Ok(MethodRouteContract {
        lib_name,
        type_id,
        method_id,
        returns_result,
    })
}

pub(super) fn resolve_birth_contract(
    loader: &PluginLoaderV2,
    box_type: &str,
) -> BidResult<BirthRouteContract> {
    let (lib_name, type_id) = resolve_type_info(loader, box_type)?;
    let (birth_id, fini_id) = resolve_birth_and_fini_for_lib(loader, &lib_name, box_type)?;
    Ok(BirthRouteContract {
        type_id,
        birth_id,
        fini_id,
    })
}

pub(super) fn resolve_invoke_route_contract(
    loader: &PluginLoaderV2,
    type_id: u32,
) -> InvokeRouteContract {
    InvokeRouteContract {
        invoke_box_fn: loader.box_invoke_fn_for_type_id(type_id),
        invoke_shim_fn: super::super::nyash_plugin_invoke_v2_shim,
    }
}
