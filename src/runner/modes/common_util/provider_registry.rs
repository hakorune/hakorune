//! Provider registry: selects concrete providers for core resources (e.g. FileBox).
//! SSOT (Single Source of Truth) for provider selection via ProviderFactory registration.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::boxes::file::core_ro::CoreRoFileIo;
use crate::boxes::file::provider::FileIo;
use crate::config::provider_env::{self, ProviderPolicy};
use crate::runner::modes::common_util::diag;

// Policy/Mode are provided by config::provider_env (centralized)
pub use crate::config::provider_env::FileBoxMode;

/// Factory for creating providers for a specific Box type.
/// Note: Currently specialized for FileBox via `FileIo`.
/// The registry is structured as BoxName → [Factory], enabling future
/// extension to other Box kinds without changing the selection policy surface.
pub trait ProviderFactory: Send + Sync {
    fn box_name(&self) -> &str;
    fn create_provider(&self) -> Arc<dyn FileIo>;
    fn is_available(&self) -> bool;
    fn priority(&self) -> i32 {
        0 // Default priority (higher = preferred)
    }
}

/// Global registry of provider factories, grouped by Box name
static PROVIDER_FACTORIES: OnceLock<Mutex<HashMap<String, Vec<Arc<dyn ProviderFactory>>>>> =
    OnceLock::new();

/// Register a provider factory (called by builtin/dynamic loaders)
pub fn register_provider_factory(factory: Arc<dyn ProviderFactory>) {
    let registry = PROVIDER_FACTORIES.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = registry.lock().unwrap();
    let key = factory.box_name().to_string();
    guard.entry(key).or_default().push(factory);
}

/// Built‑in ring‑1 FileBox provider (core‑ro) — always available, lowest priority
struct CoreRoFileProviderFactory;

impl ProviderFactory for CoreRoFileProviderFactory {
    fn box_name(&self) -> &str {
        "FileBox"
    }
    fn create_provider(&self) -> Arc<dyn FileIo> {
        Arc::new(CoreRoFileIo::new())
    }
    fn is_available(&self) -> bool {
        true
    }
    fn priority(&self) -> i32 {
        -100
    } // ring‑1: lower than any plugin/provider
}

/// Ensure ring‑1 (core‑ro) provider is present in the registry
fn ensure_builtin_file_provider_registered() {
    let reg = PROVIDER_FACTORIES.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = reg.lock().unwrap();
    let list = guard.entry("FileBox".to_string()).or_default();
    // keep ring‑1 present for safety; avoid duplicates by checking any core‑ro present by priority
    let has_core_ro = list.iter().any(|f| f.priority() <= -100);
    if !has_core_ro {
        list.push(Arc::new(CoreRoFileProviderFactory));
    }
}

/// Backward-compat public readers for existing callers (if any)
#[allow(dead_code)]
pub fn read_filebox_mode_from_env() -> FileBoxMode {
    provider_env::filebox_mode_from_env()
}

/// Select provider based on mode and registered factories (SSOT)
#[allow(dead_code)]
pub fn select_file_provider(mode: FileBoxMode) -> Arc<dyn FileIo> {
    let quiet_pipe = crate::config::env::env_bool("NYASH_JSON_ONLY");
    // Always ensure ring‑1 (core‑ro) exists before inspecting registry
    ensure_builtin_file_provider_registered();
    let registry = PROVIDER_FACTORIES.get();

    match mode {
        FileBoxMode::Auto => {
            // Selection by global policy
            let policy = provider_env::provider_policy_from_env();
            if let Some(reg) = registry {
                let mut factories: Vec<_> = reg
                    .lock()
                    .unwrap()
                    .get("FileBox")
                    .map(|v| v.iter().filter(|f| f.is_available()).cloned().collect())
                    .unwrap_or_else(|| Vec::new());

                // Sort by priority (descending); plugin providers should rank higher than ring-1 (priority -100)
                factories.sort_by(|a, b| b.priority().cmp(&a.priority()));

                // Try policy-driven choice first
                match policy {
                    ProviderPolicy::StrictPluginFirst => {
                        if let Some(factory) = factories.first() {
                            if diag::provider_log_enabled(quiet_pipe) {
                                diag::provider_log_info(&format!(
                                    "FileBox: using registered provider (priority={})",
                                    factory.priority()
                                ));
                                diag::provider_log_select("FileBox", "plugin", "dynamic", None);
                            }
                            return factory.create_provider();
                        }
                    }
                    ProviderPolicy::SafeCoreFirst | ProviderPolicy::StaticPreferred => {
                        // Prefer ring-1 (priority <= -100)
                        if let Some(core_ro) = factories.iter().find(|f| f.priority() <= -100) {
                            if diag::provider_log_enabled(quiet_pipe) {
                                diag::provider_log_info("FileBox: using core-ro (policy)");
                                diag::provider_log_select("FileBox", "1", "static", Some("[read]"));
                            }
                            return core_ro.create_provider();
                        }
                        // Fallback to first available (plugin)
                        if let Some(factory) = factories.first() {
                            if diag::provider_log_enabled(quiet_pipe) {
                                diag::provider_log_info(&format!(
                                    "FileBox: using registered provider (priority={})",
                                    factory.priority()
                                ));
                                diag::provider_log_select("FileBox", "plugin", "dynamic", None);
                            }
                            return factory.create_provider();
                        }
                    }
                }
            }

            // Fallback policy
            // Allow a narrow, explicit carve‑out:
            // - When JSON‑only pipeline is active (quiet structured I/O), or
            // - When NYASH_FILEBOX_ALLOW_FALLBACK=1 is set,
            // always use core‑ro provider even if Fail‑Fast is ON.
            let allow_fb_override = provider_env::allow_filebox_fallback_override(quiet_pipe);

            if crate::config::env::fail_fast() && !allow_fb_override {
                diag::failfast_provider("filebox:auto-fallback-blocked");
                panic!("Fail-Fast: FileBox provider fallback is disabled (NYASH_FAIL_FAST=0 or NYASH_FILEBOX_ALLOW_FALLBACK=1 to override)");
            } else {
                if diag::provider_log_enabled(quiet_pipe) {
                    diag::provider_log_info(&format!(
                        "FileBox: using core-ro fallback{}",
                        if allow_fb_override { " (override)" } else { "" }
                    ));
                    diag::provider_log_select("FileBox", "1", "static", Some("[read]"));
                }
                Arc::new(CoreRoFileIo::new())
            }
        }
        FileBoxMode::PluginOnly => {
            // Try only registered providers, Fail-Fast if none available
            if let Some(reg) = registry {
                let mut factories: Vec<_> = reg
                    .lock()
                    .unwrap()
                    .get("FileBox")
                    .map(|v| v.iter().filter(|f| f.is_available()).cloned().collect())
                    .unwrap_or_else(|| Vec::new());

                factories.sort_by(|a, b| b.priority().cmp(&a.priority()));

                if let Some(factory) = factories.first() {
                    if diag::provider_log_enabled(quiet_pipe) {
                        diag::provider_log_info(&format!(
                            "FileBox: using plugin-only provider (priority={})",
                            factory.priority()
                        ));
                        diag::provider_log_select("FileBox", "plugin", "dynamic", None);
                    }
                    return factory.create_provider();
                }
            }

            panic!("FileBox plugin-only mode: no provider registered. Set NYASH_FILEBOX_MODE=auto or NYASH_FILEBOX_MODE=core-ro to use fallback.");
        }
        FileBoxMode::CoreRo => {
            // Always use core-ro, ignore registry
            if diag::provider_log_enabled(quiet_pipe) {
                diag::provider_log_info("FileBox: using core-ro (forced)");
                diag::provider_log_select("FileBox", "1", "static", Some("[read]"));
            }
            Arc::new(CoreRoFileIo::new())
        }
    }
}
/// Provider descriptor (ring/source/capabilities). Currently informational.
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ProviderDescriptor {
    box_name: &'static str,
    ring: &'static str,                    // "0" | "1" | "plugin"
    source: &'static str,                  // "static" | "dynamic"
    capabilities: &'static [&'static str], // e.g., ["read"]
    priority: i32,
}
