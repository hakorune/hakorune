//! Unified Plugin Host facade
//!
//! Thin wrapper over v2 loader to provide a stable facade
//! with minimal, friendly API for runtime/runner and future transports.

use once_cell::sync::Lazy;
use std::cell::Cell;
use std::sync::{Arc, RwLock};

use crate::bid::{BidError, BidResult};
use crate::config::nyash_toml_v2::NyashConfigV2;
use crate::runtime::get_global_ring0;
use crate::runtime::plugin_loader_v2::PluginLoaderV2;

/// Opaque library handle (by name for now)
#[derive(Clone, Debug)]
pub struct PluginLibraryHandle {
    pub name: String,
}

/// Box type descriptor
#[derive(Clone, Debug)]
pub struct PluginBoxType {
    pub lib: String,
    pub name: String,
    pub type_id: u32,
}

/// Resolved method handle
#[derive(Clone, Debug)]
pub struct MethodHandle {
    pub lib: String,
    pub box_type: String,
    pub type_id: u32,
    pub method_id: u32,
    pub returns_result: bool,
}

/// Unified facade
pub struct PluginHost {
    loader: Arc<RwLock<PluginLoaderV2>>, // delegate
    config: Option<NyashConfigV2>,       // cached config for resolution
    config_toml: Option<toml::Value>,
    config_path: Option<String>,
}

impl PluginHost {
    pub fn new(loader: Arc<RwLock<PluginLoaderV2>>) -> Self {
        Self {
            loader,
            config: None,
            config_toml: None,
            config_path: None,
        }
    }

    /// Load config and dynamic libraries, keeping a local config cache.
    pub fn load_libraries(&mut self, config_path: &str) -> BidResult<()> {
        {
            let mut l = self.loader.write().unwrap();
            l.load_config(config_path)?;
        }

        // Keep our own copy for quick lookups
        let canonical = std::fs::canonicalize(config_path)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| config_path.to_string());
        let content = std::fs::read_to_string(&canonical).map_err(|_| BidError::PluginError)?;
        self.config = Some(NyashConfigV2::from_str(&content).map_err(|_| BidError::PluginError)?);
        self.config_toml =
            Some(toml::from_str::<toml::Value>(&content).map_err(|_| BidError::PluginError)?);
        self.config_path = Some(canonical);

        // Delegate actual library loads + pre-birth singletons to v2
        let l = self.loader.read().unwrap();
        l.load_all_plugins()
    }

    /// Register built-ins or user-defined boxes if needed (no-op for now).
    pub fn register_boxes(&self) -> BidResult<()> {
        Ok(())
    }

    /// Expose read-only view of loaded config for callers migrating from v2 paths.
    pub fn config_ref(&self) -> Option<&NyashConfigV2> {
        self.config.as_ref()
    }

    /// Load a single library directly from path for `using kind="dylib"` autoload.
    /// Boxes list is best-effort (may be empty). When empty, TypeBox FFI is used to resolve metadata.
    pub fn load_library_direct(
        &self,
        lib_name: &str,
        path: &str,
        boxes: &[String],
    ) -> BidResult<()> {
        // If caller didn't provide box names, try to infer from nyash_box.toml
        let inferred_boxes: Vec<String> = if boxes.is_empty() {
            let nyb = std::path::Path::new(path)
                .parent()
                .unwrap_or(std::path::Path::new("."))
                .join("nyash_box.toml");
            infer_box_names_from_nyash_box(&nyb)
        } else {
            Vec::new()
        };
        let effective_boxes: Vec<String> = if boxes.is_empty() {
            inferred_boxes
        } else {
            boxes.to_vec()
        };
        let def = crate::config::nyash_toml_v2::LibraryDefinition {
            boxes: effective_boxes.clone(),
            path: path.to_string(),
        };
        // Ensure loader has a minimal config so find_library_for_box works
        {
            let mut l = self.loader.write().unwrap();
            if l.config.is_none() {
                let mut cfg = NyashConfigV2 {
                    libraries: std::collections::HashMap::new(),
                    plugin_paths: crate::config::nyash_toml_v2::PluginPaths {
                        search_paths: vec![],
                    },
                    plugins: std::collections::HashMap::new(),
                    box_types: std::collections::HashMap::new(),
                };
                cfg.libraries.insert(
                    lib_name.to_string(),
                    crate::config::nyash_toml_v2::LibraryDefinition {
                        boxes: def.boxes.clone(),
                        path: def.path.clone(),
                    },
                );
                l.config = Some(cfg);
                // No dedicated config file; keep config_path None and rely on box_specs fallback
            } else if let Some(cfg) = l.config.as_mut() {
                cfg.libraries.insert(
                    lib_name.to_string(),
                    crate::config::nyash_toml_v2::LibraryDefinition {
                        boxes: def.boxes.clone(),
                        path: def.path.clone(),
                    },
                );
            }
            // Load the library now
            l.load_plugin_direct(lib_name, &def)?;
            // Ingest nyash_box.toml (if present) to populate box_specs: type_id/method ids
            let nyb_path = std::path::Path::new(path)
                .parent()
                .unwrap_or(std::path::Path::new("."))
                .join("nyash_box.toml");
            l.ingest_box_specs_from_nyash_box(lib_name, &def.boxes, &nyb_path);
            // Also register providers in the v2 BoxFactoryRegistry so `new BoxType()` works
            let registry = crate::runtime::get_global_registry();
            for bx in &def.boxes {
                registry.apply_plugin_config(&crate::runtime::PluginConfig {
                    plugins: [(bx.clone(), lib_name.to_string())].into(),
                });
            }
        }
        Ok(())
    }

    /// Resolve a method handle for a given plugin box type and method name.
    pub fn resolve_method(&self, box_type: &str, method_name: &str) -> BidResult<MethodHandle> {
        let cfg = self.config.as_ref().ok_or(BidError::PluginError)?;
        let toml_value = self.config_toml.as_ref().ok_or(BidError::PluginError)?;

        // Path A: library-backed box (dynamic plugin)
        if let Some((lib_name, _lib_def)) = cfg.find_library_for_box(box_type) {
            if let Some(box_conf) = cfg.get_box_config(lib_name, box_type, toml_value) {
                // Prefer config mapping; fallback to loader's TypeBox resolve(name)
                let (method_id, returns_result) = if let Some(m) = box_conf.methods.get(method_name)
                {
                    (m.method_id, m.returns_result)
                } else {
                    // No implicit resolver fallback in fail-fast mode.
                    // Keep legacy fallback only when explicitly relaxed.
                    if crate::config::env::fail_fast() {
                        return Err(BidError::InvalidMethod);
                    }
                    let l = self.loader.read().unwrap();
                    let mid = l
                        .resolve_method_id(box_type, method_name)
                        .map_err(|_| BidError::InvalidMethod)?;
                    (mid, false)
                };
                return Ok(MethodHandle {
                    lib: lib_name.to_string(),
                    box_type: box_type.to_string(),
                    type_id: box_conf.type_id,
                    method_id,
                    returns_result,
                });
            }
        }

        // Path B: builtin/core boxes via central config (no library/path required)
        // Require: [box_types] BoxName = <id> and [box_methods.BoxName.methods] entries
        if let Some(type_id) = cfg.box_types.get(box_type).copied() {
            if let Some(bm) = toml_value
                .get("box_methods")
                .and_then(|v| v.get(box_type))
                .and_then(|v| v.get("methods"))
                .and_then(|v| v.as_table())
            {
                if let Some(entry) = bm.get(method_name) {
                    // Support both { method_id = N } and bare integer in the future
                    let (method_id, returns_result) = if let Some(mid) = entry.get("method_id") {
                        (
                            mid.as_integer().unwrap_or(0) as u32,
                            entry
                                .get("returns_result")
                                .and_then(|b| b.as_bool())
                                .unwrap_or(false),
                        )
                    } else if let Some(mid) = entry.as_integer() {
                        (mid as u32, false)
                    } else {
                        return Err(BidError::InvalidMethod);
                    };
                    return Ok(MethodHandle {
                        lib: "builtin".to_string(),
                        box_type: box_type.to_string(),
                        type_id,
                        method_id,
                        returns_result,
                    });
                }
            }
        }

        // Fallback: delegate to loader (TypeBox, file-based, etc.)
        if crate::config::env::fail_fast() {
            return Err(BidError::InvalidMethod);
        }
        let l = self.loader.read().unwrap();
        let mid = l
            .resolve_method_id(box_type, method_name)
            .map_err(|_| BidError::InvalidMethod)?;
        let type_id = *cfg.box_types.get(box_type).unwrap_or(&0);
        Ok(MethodHandle {
            lib: "builtin".to_string(),
            box_type: box_type.to_string(),
            type_id,
            method_id: mid,
            returns_result: false,
        })
    }

    // --- v2 adapter layer: allow gradual migration of callers ---
    pub fn create_box(
        &self,
        box_type: &str,
        args: &[Box<dyn crate::box_trait::NyashBox>],
    ) -> BidResult<Box<dyn crate::box_trait::NyashBox>> {
        let l = self.loader.read().unwrap();
        l.create_box(box_type, args)
    }

    pub fn invoke_instance_method(
        &self,
        box_type: &str,
        method_name: &str,
        instance_id: u32,
        args: &[Box<dyn crate::box_trait::NyashBox>],
    ) -> BidResult<Option<Box<dyn crate::box_trait::NyashBox>>> {
        thread_local! { static HOST_REENTRANT: Cell<bool> = Cell::new(false); }
        let recursed = HOST_REENTRANT.with(|f| f.get());
        if recursed {
            // Break potential host<->loader recursion: return None (void) to keep VM running
            return Ok(None);
        }
        let out = HOST_REENTRANT.with(|f| {
            f.set(true);
            let res = {
                let l = self.loader.read().unwrap();
                l.invoke_instance_method(box_type, method_name, instance_id, args)
            };
            f.set(false);
            res
        });
        out
    }

    /// Check if a method returns Result (Ok/Err) per plugin spec or central config.
    pub fn method_returns_result(&self, box_type: &str, method_name: &str) -> bool {
        // Prefer central config when available (works for builtin boxes)
        if let (Some(cfg), Some(toml_value)) = (self.config.as_ref(), self.config_toml.as_ref()) {
            if let Some(bm) = toml_value
                .get("box_methods")
                .and_then(|v| v.get(box_type))
                .and_then(|v| v.get("methods"))
                .and_then(|v| v.as_table())
            {
                if let Some(entry) = bm.get(method_name) {
                    return entry
                        .get("returns_result")
                        .and_then(|b| b.as_bool())
                        .unwrap_or(false);
                }
            }
            // Library-backed path
            if let Some((lib_name, _)) = cfg.find_library_for_box(box_type) {
                if let Some(box_conf) = cfg.get_box_config(lib_name, box_type, toml_value) {
                    if let Some(m) = box_conf.methods.get(method_name) {
                        return m.returns_result;
                    }
                }
            }
        }
        let l = self.loader.read().unwrap();
        l.method_returns_result(box_type, method_name)
    }

    pub fn extern_call(
        &self,
        iface_name: &str,
        method_name: &str,
        args: &[Box<dyn crate::box_trait::NyashBox>],
    ) -> BidResult<Option<Box<dyn crate::box_trait::NyashBox>>> {
        // Special-case env.future.await to avoid holding loader RwLock while polling scheduler
        if iface_name == "env.future" && method_name == "await" {
            use crate::boxes::result::NyashResultBox;
            if let Some(arg0) = args.get(0) {
                if let Some(fut) = arg0
                    .as_any()
                    .downcast_ref::<crate::boxes::future::FutureBox>()
                {
                    let max_ms: u64 = crate::config::env::await_max_ms();
                    // Phase 90-C/D: time/thread 系移行
                    let ring0 = crate::runtime::ring0::get_global_ring0();
                    let start = ring0
                        .time
                        .monotonic_now()
                        .map_err(|_e| crate::bid::error::BidError::PluginError)?;
                    let mut spins = 0usize;
                    while !fut.ready() {
                        crate::runtime::global_hooks::safepoint_and_poll();
                        std::thread::yield_now();
                        spins += 1;
                        if spins % 1024 == 0 {
                            ring0.thread.sleep(std::time::Duration::from_millis(1));
                        }
                        if ring0.time.elapsed(start) >= std::time::Duration::from_millis(max_ms) {
                            let err = crate::box_trait::StringBox::new("Timeout");
                            return Ok(Some(Box::new(NyashResultBox::new_err(Box::new(err)))));
                        }
                    }
                    return Ok(fut.wait_and_get().ok().map(|v| {
                        Box::new(NyashResultBox::new_ok(v)) as Box<dyn crate::box_trait::NyashBox>
                    }));
                } else {
                    return Ok(Some(Box::new(NyashResultBox::new_ok(arg0.clone_box()))));
                }
            }
            return Ok(Some(Box::new(NyashResultBox::new_err(Box::new(
                crate::box_trait::StringBox::new("InvalidArgs"),
            )))));
        }
        let l = self.loader.read().unwrap();
        l.extern_call(iface_name, method_name, args)
    }
}

/// Best-effort extraction of box names from a nyash_box.toml file.
/// Priority:
///  1) [provides].boxes = ["BoxA", "BoxB"]
///  2) Top-level tables that look like box sections (have `type_id` or `methods`/`lifecycle`)
fn infer_box_names_from_nyash_box(nyb_path: &std::path::Path) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if !nyb_path.exists() {
        return out;
    }
    let Ok(text) = std::fs::read_to_string(nyb_path) else {
        return out;
    };
    let Ok(doc) = toml::from_str::<toml::Value>(&text) else {
        return out;
    };
    // 1) explicit provides
    if let Some(arr) = doc
        .get("provides")
        .and_then(|v| v.get("boxes"))
        .and_then(|v| v.as_array())
    {
        for v in arr {
            if let Some(s) = v.as_str() {
                out.push(s.to_string());
            }
        }
        out.sort();
        out.dedup();
        if !out.is_empty() {
            return out;
        }
    }
    // 2) heuristic: tables with type_id or lifecycle/methods
    if let Some(tbl) = doc.as_table() {
        for (k, v) in tbl.iter() {
            if k == "box" || k == "implementation" || k == "artifacts" || k == "provides" {
                continue;
            }
            if let Some(t) = v.as_table() {
                let looks_like_box = t.get("type_id").is_some()
                    || t.get("methods").is_some()
                    || t.get("lifecycle").is_some();
                if looks_like_box {
                    out.push(k.clone());
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

// Global singleton
static GLOBAL_HOST: Lazy<Arc<RwLock<PluginHost>>> = Lazy::new(|| {
    let loader = crate::runtime::plugin_loader_v2::get_global_loader_v2();
    Arc::new(RwLock::new(PluginHost::new(loader)))
});

pub fn get_global_plugin_host() -> Arc<RwLock<PluginHost>> {
    GLOBAL_HOST.clone()
}

pub fn init_global_plugin_host(config_path: &str) -> BidResult<()> {
    let host = get_global_plugin_host();
    {
        let mut h = host.write().unwrap();
        let disabled = crate::config::env::disable_plugins();
        if disabled {
            get_global_ring0()
                .log
                .warn("[plugin/init] plugins disabled by NYASH_DISABLE_PLUGINS=1");
            return Err(BidError::PluginError);
        }

        if !std::path::Path::new(config_path).exists() {
            get_global_ring0().log.warn(&format!(
                "[plugin/init] plugins disabled (config={}): config file not found",
                config_path
            ));
            return Err(BidError::PluginError);
        }

        h.load_libraries(config_path).map_err(|e| {
            get_global_ring0().log.error(&format!(
                "[plugin/init] load_libraries({}) failed: {}",
                config_path, e
            ));
            BidError::PluginError
        })?;
        h.register_boxes().map_err(|e| {
            get_global_ring0().log.error(&format!(
                "[plugin/init] register_boxes({}) failed: {}",
                config_path, e
            ));
            BidError::PluginError
        })?;
    }
    Ok(())
}
