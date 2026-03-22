/*!
 * Runner plugin initialization (extracted from runner.rs)
 *
 * Purpose: Initialize v2 plugin system from nyash.toml and apply config
 * Behavior: Quiet by default; use NYASH_CLI_VERBOSE=1 or NYASH_DEBUG_PLUGIN=1 for logs
 */

use crate::runtime::get_global_ring0;
use crate::runtime::{
    get_global_plugin_host, get_global_registry, init_global_plugin_host, PluginConfig,
};

fn resolve_plugin_toml() -> String {
    // Prefer hakorune.toml, fallback to nyash.toml (check CWD, then NYASH_ROOT)
    let cwd_hako = std::path::Path::new("hakorune.toml");
    if cwd_hako.exists() {
        return "hakorune.toml".to_string();
    }
    let cwd_ny = std::path::Path::new("nyash.toml");
    if cwd_ny.exists() {
        return "nyash.toml".to_string();
    }
    if let Some(root) = crate::config::env::nyash_root() {
        let p = std::path::Path::new(&root).join("hakorune.toml");
        if p.exists() {
            return p.to_string_lossy().to_string();
        }
        let p2 = std::path::Path::new(&root).join("nyash.toml");
        if p2.exists() {
            return p2.to_string_lossy().to_string();
        }
    }
    "nyash.toml".to_string()
}

pub fn init_bid_plugins() {
    let cli_verbose = crate::config::env::cli_verbose_enabled();
    let plugin_debug = crate::config::env::debug_plugin();
    if plugin_debug {
        get_global_ring0()
            .log
            .debug("[plugin/init] Initializing v2 plugin system");
    }

    let cfg_path = resolve_plugin_toml();
    match init_global_plugin_host(&cfg_path) {
        Ok(()) => {
            if plugin_debug || cli_verbose {
                let plugin_exec_mode = crate::config::env::plugin_exec_mode();
                let box_factory_policy = crate::config::env::box_factory_policy()
                    .unwrap_or_else(|| "builtin_first(default)".to_string());
                get_global_ring0().log.info(&format!(
                    "[plugin/init] plugin host initialized from {}",
                    cfg_path
                ));
                get_global_ring0().log.info(&format!(
                    "[plugin-loader] backend={}",
                    crate::runtime::plugin_loader_v2::backend_kind()
                ));
                get_global_ring0().log.info(&format!(
                    "[runtime/exec-path] plugin_loader_backend={} plugin_exec_mode={:?} box_factory_policy={}",
                    crate::runtime::plugin_loader_v2::backend_kind(),
                    plugin_exec_mode,
                    box_factory_policy
                ));
            }
            let host = get_global_plugin_host();
            let host = host.read().unwrap();
            if let Some(config) = host.config_ref() {
                let registry = get_global_registry();
                for (lib_name, lib_def) in &config.libraries {
                    for box_name in &lib_def.boxes {
                        if plugin_debug {
                            get_global_ring0().log.debug(&format!(
                                "[plugin/init] Registering plugin provider for {}",
                                box_name
                            ));
                        }
                        registry.apply_plugin_config(&PluginConfig {
                            plugins: [(box_name.clone(), lib_name.clone())].into(),
                        });
                    }
                }
                if plugin_debug || cli_verbose {
                    get_global_ring0()
                        .log
                        .info("[plugin/init] plugin host fully configured");
                }
            }

            // Optional autoload for [using.*] kind="dylib" packages
            if crate::config::env::using_dylib_autoload() && !crate::config::env::disable_plugins()
            {
                if plugin_debug || cli_verbose {
                    get_global_ring0()
                        .log
                        .info("[using.dylib/autoload] scanning nyash.toml packages …");
                }
                let mut using_paths: Vec<String> = Vec::new();
                let mut pending_modules: std::vec::Vec<(String, String)> = Vec::new();
                let mut aliases: std::collections::HashMap<String, String> =
                    std::collections::HashMap::new();
                let mut packages: std::collections::HashMap<
                    String,
                    crate::using::spec::UsingPackage,
                > = std::collections::HashMap::new();
                let mut module_roots: Vec<(String, String)> = Vec::new();
                let _ = crate::using::resolver::populate_from_toml(
                    &mut using_paths,
                    &mut pending_modules,
                    &mut aliases,
                    &mut packages,
                    &mut module_roots,
                );
                for (name, pkg) in packages.iter() {
                    if let crate::using::spec::PackageKind::Dylib = pkg.kind {
                        // Build library name from file stem (best-effort)
                        let lib_name = std::path::Path::new(&pkg.path)
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or(name)
                            .to_string();
                        let host = get_global_plugin_host();
                        let res =
                            host.read()
                                .unwrap()
                                .load_library_direct(&lib_name, &pkg.path, &[]);
                        if let Err(e) = res {
                            if plugin_debug || cli_verbose {
                                get_global_ring0().log.warn(&format!(
                                    "[using.dylib/autoload] failed '{}': {}",
                                    lib_name, e
                                ));
                            }
                        } else if plugin_debug || cli_verbose {
                            get_global_ring0().log.info(&format!(
                                "[using.dylib/autoload] loaded '{}' from {}",
                                lib_name, pkg.path
                            ));
                        }
                    }
                }
            }
        }
        Err(e) => {
            if plugin_debug || cli_verbose {
                get_global_ring0().log.warn(&format!(
                    "[plugin/init] failed to initialize from {}: {}",
                    cfg_path, e
                ));
            } else {
                get_global_ring0().log.warn(&format!(
                    "[plugin/init] plugins disabled (config={}): {}",
                    cfg_path, e
                ));
            }
            return;
        }
    }
}
