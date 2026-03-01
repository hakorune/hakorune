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

#[inline]
fn compat_fallback_allowed() -> bool {
    // Runtime/plugin compat fallback must follow the same route policy gate.
    crate::config::env::vm_compat_fallback_allowed()
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

    if crate::config::env::fail_fast() || !compat_fallback_allowed() {
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

    if crate::config::env::fail_fast() || !compat_fallback_allowed() {
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

pub(super) fn resolve_birth_contract_for_lib(
    loader: &PluginLoaderV2,
    lib_name: &str,
    box_type: &str,
) -> BidResult<BirthRouteContract> {
    let type_id =
        type_id_from_selected_lib(loader, lib_name, box_type)?.ok_or(BidError::InvalidType)?;
    let (birth_id, fini_id) = resolve_birth_and_fini_for_lib(loader, lib_name, box_type)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::nyash_toml_v2::NyashConfigV2;
    use crate::runtime::plugin_loader_v2::enabled::PluginLoaderV2;

    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn with_env_vars<F: FnOnce()>(pairs: &[(&str, &str)], f: F) {
        let _guard = ENV_LOCK.lock().expect("env lock");
        let prev: Vec<(String, Option<String>)> = pairs
            .iter()
            .map(|(k, _)| ((*k).to_string(), std::env::var(k).ok()))
            .collect();
        for (k, v) in pairs {
            std::env::set_var(k, v);
        }
        f();
        for (k, prev_v) in prev {
            if let Some(v) = prev_v {
                std::env::set_var(&k, v);
            } else {
                std::env::remove_var(&k);
            }
        }
    }

    fn seed_loader_with_spec() -> PluginLoaderV2 {
        let mut loader = PluginLoaderV2::new();
        let toml_str = r#"
[libraries]
[libraries.demo]
boxes = ["DemoBox"]
path = "./libdemo.so"

[libraries.demo.DemoBox]
type_id = 42

[libraries.demo.DemoBox.methods]
birth = { method_id = 1 }
fini = { method_id = 999 }
run = { method_id = 7, returns_result = true }
"#;
        loader.config = Some(NyashConfigV2::from_str(toml_str).expect("parse config"));
        loader.config_toml = Some(toml::from_str::<toml::Value>(toml_str).expect("parse raw toml"));
        loader
    }

    #[test]
    fn resolve_method_contract_from_specs() {
        let loader = seed_loader_with_spec();
        let got = resolve_method_contract(&loader, "DemoBox", "run").expect("method contract");
        assert_eq!(got.lib_name, "demo");
        assert_eq!(got.type_id, 42);
        assert_eq!(got.method_id, 7);
        assert!(got.returns_result);
    }

    #[test]
    fn resolve_birth_contract_from_specs() {
        let loader = seed_loader_with_spec();
        let got = resolve_birth_contract(&loader, "DemoBox").expect("birth contract");
        assert_eq!(got.type_id, 42);
        assert_eq!(got.birth_id, 1);
        assert_eq!(got.fini_id, Some(999));
    }

    #[test]
    fn resolve_birth_contract_for_lib_from_specs() {
        let loader = seed_loader_with_spec();
        let got =
            resolve_birth_contract_for_lib(&loader, "demo", "DemoBox").expect("birth contract");
        assert_eq!(got.type_id, 42);
        assert_eq!(got.birth_id, 1);
        assert_eq!(got.fini_id, Some(999));
    }

    #[test]
    fn resolve_invoke_route_contract_returns_shim_when_invoke_box_missing() {
        let loader = seed_loader_with_spec();
        let got = resolve_invoke_route_contract(&loader, 42);
        assert!(got.invoke_box_fn.is_none());
        // With no per-box invoke function, shim returns E_PLUGIN (-5).
        let mut out = [0u8; 8];
        let mut out_len: usize = out.len();
        let code = unsafe {
            (got.invoke_shim_fn)(42, 7, 1, std::ptr::null(), 0, out.as_mut_ptr(), &mut out_len)
        };
        assert_eq!(code, -5);
    }

    #[test]
    fn resolve_type_info_compat_fallback_respects_vm_fallback_policy() {
        let loader = PluginLoaderV2::new();
        let mut spec_path = std::env::temp_dir();
        spec_path.push(format!(
            "phase29cc_route_resolver_{}_{}.toml",
            std::process::id(),
            "compat_fallback"
        ));
        std::fs::write(&spec_path, "[DemoBox]\ntype_id = 77\n").expect("write spec");
        loader.ingest_box_specs_from_nyash_box(
            "demo",
            &["DemoBox".to_string()],
            spec_path.as_path(),
        );
        let _ = std::fs::remove_file(&spec_path);

        with_env_vars(
            &[("NYASH_FAIL_FAST", "0"), ("NYASH_VM_USE_FALLBACK", "1")],
            || {
                let got = resolve_type_info(&loader, "DemoBox").expect("compat fallback route");
                assert_eq!(got.0, "demo");
                assert_eq!(got.1, 77);
            },
        );
        with_env_vars(
            &[("NYASH_FAIL_FAST", "0"), ("NYASH_VM_USE_FALLBACK", "0")],
            || {
                let got = resolve_type_info(&loader, "DemoBox");
                assert!(matches!(got, Err(BidError::InvalidType)));
            },
        );
    }
}
