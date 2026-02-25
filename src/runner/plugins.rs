/*!
 * Runner plugin/registry initialization (extracted)
 */

use super::*;

impl NyashRunner {
    /// Initialize global runtime registry and load configured plugins.
    ///
    /// Behavior (Phase 21.4: Unified ENV Policy):
    /// - Always initializes the unified type/box registry.
    /// - Loads native BID plugins unless `NYASH_DISABLE_PLUGINS=1`.
    /// - When `--load-ny-plugins` or `NYASH_LOAD_NY_PLUGINS=1` is set, best‑effort
    ///   loads Nyash scripts listed under `nyash.toml`'s `ny_plugins`.
    ///
    /// ENV Policy:
    /// - NYASH_DISABLE_PLUGINS=1: Skip all plugin initialization
    /// - NYASH_BOX_FACTORY_POLICY: Control factory priority (builtin_first|strict_plugin_first|compat_plugin_first)
    /// - NYASH_FILEBOX_MODE: FileBox provider selection (auto|core-ro|plugin-only)
    ///
    /// Deprecated ENV (removed auto-setting):
    /// - NYASH_USE_PLUGIN_BUILTINS: No longer auto-set (use policy instead)
    /// - NYASH_PLUGIN_OVERRIDE_TYPES: No longer auto-set (use policy instead)
    pub(crate) fn init_runtime_and_plugins(&self, groups: &crate::cli::CliGroups) {
        // Check if plugins are disabled (SSOT)
        let plugins_disabled = crate::config::env::disable_plugins();

        // Unified registry (always initialize)
        runtime::init_global_unified_registry();

        // Plugins (guarded by NYASH_DISABLE_PLUGINS)
        if !plugins_disabled {
            runner_plugin_init::init_bid_plugins();
            crate::runner::box_index::refresh_box_index();
        } else {
            // Gate/CI often runs with plugins disabled for hermeticity; avoid noise unless asked.
            if crate::config::env::debug_plugin() || crate::config::env::cli_verbose_enabled() {
                crate::runtime::get_global_ring0()
                    .log
                    .warn("[plugins] Skipping plugin initialization (NYASH_DISABLE_PLUGINS=1)");
            }
        }

        // Deprecation warnings for old ENV variables
        if std::env::var("NYASH_USE_PLUGIN_BUILTINS").is_ok() {
            crate::runtime::get_global_ring0().log.warn(
                "[warn] NYASH_USE_PLUGIN_BUILTINS is deprecated. Use NYASH_BOX_FACTORY_POLICY instead.",
            );
        }
        if std::env::var("NYASH_PLUGIN_OVERRIDE_TYPES").is_ok() {
            crate::runtime::get_global_ring0().log.warn(
                "[warn] NYASH_PLUGIN_OVERRIDE_TYPES is deprecated. Use NYASH_BOX_FACTORY_POLICY instead.",
            );
        }

        // Optional Ny script plugins loader (best-effort)
        if groups.load_ny_plugins
            || std::env::var("NYASH_LOAD_NY_PLUGINS").ok().as_deref() == Some("1")
        {
            if let Ok(text) = std::fs::read_to_string("nyash.toml") {
                if let Ok(doc) = toml::from_str::<toml::Value>(&text) {
                    if let Some(np) = doc.get("ny_plugins") {
                        let mut list: Vec<String> = Vec::new();
                        if let Some(arr) = np.as_array() {
                            for v in arr {
                                if let Some(s) = v.as_str() {
                                    list.push(s.to_string());
                                }
                            }
                        } else if let Some(tbl) = np.as_table() {
                            for (_k, v) in tbl {
                                if let Some(s) = v.as_str() {
                                    list.push(s.to_string());
                                } else if let Some(arr) = v.as_array() {
                                    for e in arr {
                                        if let Some(s) = e.as_str() {
                                            list.push(s.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        if !list.is_empty() {
                            let list_only =
                                std::env::var("NYASH_NY_PLUGINS_LIST_ONLY").ok().as_deref()
                                    == Some("1");
                            println!("🧩 Ny script plugins ({}):", list.len());
                            for p in list {
                                if list_only {
                                    println!("  • {}", p);
                                    continue;
                                }
                                match std::fs::read_to_string(&p) {
                                    Ok(_code) => {
                                        // Legacy interpreter removed - ny_plugins execution disabled
                                        println!(
                                            "[ny_plugins] {}: SKIP (legacy interpreter removed)",
                                            p
                                        );
                                    }
                                    Err(e) => println!("[ny_plugins] {}: FAIL (read: {})", p, e),
                                }
                            }
                        }
                    }
                }
            }
        }

        // Provider verify (受け口): env で warn/strict のみ動作（未設定時は無処理）
        match crate::runtime::provider_verify::verify_from_env() {
            Ok(()) => {}
            Err(e) => {
                eprintln!("❌ {}", e);
                std::process::exit(1);
            }
        }

        // Provider Lock — lock after registry and plugins are initialized (受け口)
        // Default: no-op behavior change. Exposed for future verify→lock sequencing.
        crate::runtime::provider_lock::lock_providers();
    }
}
