use super::error_reporter::{report_and_fail, PluginErrorContext};
use super::specs;
use super::util::dbg_on;
use super::PluginLoaderV2;
use crate::bid::{BidError, BidResult};
use crate::config::env::env_bool;
use crate::config::nyash_toml_v2::LibraryDefinition;
use crate::runtime::get_global_ring0;
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub(super) fn load_all_plugins(loader: &PluginLoaderV2) -> BidResult<()> {
    let config = loader.config.as_ref().ok_or(BidError::PluginError)?;
    // Strict mode policy (SSOT): reuse JoinIR strict flag to avoid env-var sprawl.
    // - strict=0: Phase 134 best-effort load (continue on failure)
    // - strict=1: Fail-Fast on first plugin/library load error
    let strict = env_bool("HAKO_JOINIR_STRICT");

    // Phase 134 P0: Best-effort loading
    // Failures don't stop the entire load process
    let mut loaded_count = 0;
    let mut failed_count = 0;

    // Load libraries in deterministic order (sorted by name)
    let mut lib_items: Vec<_> = config.libraries.iter().collect();
    lib_items.sort_by_key(|(name, _)| *name);

    for (lib_name, lib_def) in lib_items {
        match load_plugin(loader, lib_name, lib_def) {
            Ok(()) => loaded_count += 1,
            Err(e) => {
                failed_count += 1;
                if strict {
                    return Err(e);
                }
                // Log already printed by load_plugin, continue
            }
        }
    }

    // Load plugins in deterministic order (sorted by name)
    let mut plugin_items: Vec<_> = config.plugins.iter().collect();
    plugin_items.sort_by_key(|(name, _)| *name);

    for (plugin_name, root) in plugin_items {
        match load_plugin_from_root(loader, plugin_name, root) {
            Ok(()) => loaded_count += 1,
            Err(e) => {
                failed_count += 1;
                if strict {
                    return Err(e);
                }
                // Log already printed by load_plugin_from_root, continue
            }
        }
    }

    // Phase 134 P0: Log summary
    if failed_count > 0 {
        get_global_ring0().log.warn(&format!(
            "[plugin/init] loaded {} plugins, {} failed",
            loaded_count, failed_count
        ));
    }

    // Continue with singleton prebirth even if some plugins failed
    // This follows "fail gracefully" principle: partially working state is better than complete failure
    super::singletons::prebirth_singletons(loader)?;
    Ok(())
}

pub(super) fn load_plugin(
    loader: &PluginLoaderV2,
    lib_name: &str,
    lib_def: &LibraryDefinition,
) -> BidResult<()> {
    let base = Path::new(&lib_def.path);
    let candidates = candidate_paths(base);
    let mut lib_path = candidates.iter().find(|p| p.exists()).cloned();
    if lib_path.is_none() {
        if let Some(cfg) = &loader.config {
            for candidate in &candidates {
                if let Some(fname) = candidate.file_name().and_then(|s| s.to_str()) {
                    if let Some(resolved) = cfg.resolve_plugin_path(fname) {
                        let pb = PathBuf::from(resolved);
                        if pb.exists() {
                            lib_path = Some(pb);
                            break;
                        }
                    }
                }
            }
        }
    }
    let lib_path = match lib_path {
        Some(path) => path,
        None => {
            // Phase 97: Use structured error reporter
            let ctx = PluginErrorContext::missing_library(
                lib_name,
                &base.display().to_string(),
                candidates,
            );
            return Err(report_and_fail(ctx));
        }
    };
    if dbg_on() {
        get_global_ring0().log.debug(&format!(
            "[PluginLoaderV2] load_plugin: lib='{}' path='{}'",
            lib_name,
            lib_path.display()
        ));
    }
    let lib = unsafe { Library::new(&lib_path) }.map_err(|e| {
        // Phase 97: Use structured error reporter
        let ctx = PluginErrorContext::load_failed(
            lib_name,
            &lib_path.display().to_string(),
            &e.to_string(),
        );
        report_and_fail(ctx)
    })?;
    let lib_arc = Arc::new(lib);

    unsafe {
        if let Ok(init_sym) =
            lib_arc.get::<Symbol<unsafe extern "C" fn() -> i32>>(b"nyash_plugin_init\0")
        {
            let _ = init_sym();
        }
    }

    let loaded = super::super::types::LoadedPluginV2 {
        _lib: lib_arc.clone(),
        box_types: lib_def.boxes.clone(),
        typeboxes: HashMap::new(),
        init_fn: None,
    };
    loader
        .plugins
        .write()
        .map_err(|_| BidError::PluginError)?
        .insert(lib_name.to_string(), Arc::new(loaded));

    for box_type in &lib_def.boxes {
        let sym_name = format!("nyash_typebox_{}\0", box_type);
        unsafe {
            if let Ok(tb_sym) =
                lib_arc.get::<Symbol<&super::super::types::NyashTypeBoxFfi>>(sym_name.as_bytes())
            {
                specs::record_typebox_spec(loader, lib_name, box_type, &*tb_sym)?;
            } else if dbg_on() {
                get_global_ring0().log.debug(&format!(
                    "[PluginLoaderV2] NOTE: TypeBox symbol not found for {}.{} (symbol='{}'). Migrate plugin to Nyash ABI v2 to enable per-Box dispatch.",
                    lib_name,
                    box_type,
                    sym_name.trim_end_matches('\0')
                ));
            }
        }
    }

    Ok(())
}

pub(super) fn load_plugin_from_root(
    _loader: &PluginLoaderV2,
    _plugin_name: &str,
    _root: &str,
) -> BidResult<()> {
    Ok(())
}

fn candidate_paths(base: &Path) -> Vec<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if cfg!(target_os = "windows") {
        candidates.push(base.with_extension("dll"));
        if let Some(file) = base.file_name().and_then(|s| s.to_str()) {
            if file.starts_with("lib") {
                let mut alt = base.to_path_buf();
                let alt_file = file.trim_start_matches("lib");
                alt.set_file_name(alt_file);
                candidates.push(alt.with_extension("dll"));
            }
        }
    } else if cfg!(target_os = "macos") {
        candidates.push(base.with_extension("dylib"));
    } else {
        candidates.push(base.with_extension("so"));
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::nyash_toml_v2::{NyashConfigV2, PluginPaths};
    use crate::tests::helpers::joinir_env::with_joinir_env_lock;
    use std::env;

    struct EnvGuard {
        key: &'static str,
        original: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let original = env::var(key).ok();
            env::set_var(key, value);
            Self { key, original }
        }

        fn unset(key: &'static str) -> Self {
            let original = env::var(key).ok();
            env::remove_var(key);
            Self { key, original }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(val) = &self.original {
                env::set_var(self.key, val);
            } else {
                env::remove_var(self.key);
            }
        }
    }

    fn ensure_ring0_initialized() {
        use crate::runtime::ring0::{default_ring0, GLOBAL_RING0};
        use std::sync::Arc;

        if GLOBAL_RING0.get().is_none() {
            let _ = GLOBAL_RING0.set(Arc::new(default_ring0()));
        }
    }

    fn loader_with_missing_library(path: &str) -> PluginLoaderV2 {
        let mut libraries = HashMap::new();
        libraries.insert(
            "missing_lib".to_string(),
            LibraryDefinition {
                boxes: vec!["FileBox".to_string()],
                path: path.to_string(),
            },
        );
        PluginLoaderV2 {
            config: Some(NyashConfigV2 {
                libraries,
                plugin_paths: PluginPaths::default(),
                plugins: HashMap::new(),
                box_types: HashMap::new(),
            }),
            ..PluginLoaderV2::new()
        }
    }

    #[test]
    fn load_all_plugins_strict_fails_on_missing_library() {
        with_joinir_env_lock(|| {
            ensure_ring0_initialized();
            let _guard = EnvGuard::set("HAKO_JOINIR_STRICT", "1");
            let loader = loader_with_missing_library("/nonexistent/libnyash_filebox_plugin");

            let result = load_all_plugins(&loader);
            assert!(
                result.is_err(),
                "strict mode must fail when library is missing"
            );
        });
    }

    #[test]
    fn load_all_plugins_best_effort_continues_on_missing_library() {
        with_joinir_env_lock(|| {
            ensure_ring0_initialized();
            let _guard = EnvGuard::unset("HAKO_JOINIR_STRICT");
            let loader = loader_with_missing_library("/nonexistent/libnyash_filebox_plugin");

            let result = load_all_plugins(&loader);
            assert!(
                result.is_ok(),
                "non-strict mode should continue even when a library is missing"
            );
        });
    }
}
