/*!
 * Plugin Box Factory
 *
 * Handles creation of plugin-based Box types through BID/FFI system
 * Integrates with v2 plugin system (BoxFactoryRegistry)
 */

use super::BoxFactory;
use super::RuntimeError;
use crate::box_trait::NyashBox;
use crate::config::env;
use crate::runtime::get_global_registry;
use crate::runtime::get_global_ring0;

/// Factory for plugin-based Box types
pub struct PluginBoxFactory {
    // Uses the global BoxFactoryRegistry from v2 plugin system
}

impl PluginBoxFactory {
    pub fn new() -> Self {
        Self {}
    }
}

fn is_core_module_first_box(name: &str) -> bool {
    matches!(
        name,
        "ArrayBox" | "StringBox" | "MapBox" | "ConsoleBox" | "FileBox" | "PathBox"
    )
}

fn should_skip_dynamic_route(name: &str, mode: env::PluginExecMode) -> bool {
    match mode {
        env::PluginExecMode::ModuleFirst => is_core_module_first_box(name),
        env::PluginExecMode::DynamicOnly | env::PluginExecMode::DynamicFirst => false,
    }
}

impl BoxFactory for PluginBoxFactory {
    fn create_box(
        &self,
        name: &str,
        args: &[Box<dyn NyashBox>],
    ) -> Result<Box<dyn NyashBox>, RuntimeError> {
        // Check if plugins are disabled
        let plugins_disabled = env::disable_plugins();
        if env::debug_plugin() {
            get_global_ring0().log.debug(&format!(
                "[plugin/disable] disabled={} box={}",
                plugins_disabled, name
            ));
        }
        if plugins_disabled {
            return Err(RuntimeError::InvalidOperation {
                message: format!(
                    "Plugins disabled (NYASH_DISABLE_PLUGINS=1), cannot create {}",
                    name
                ),
            });
        }

        let exec_mode = env::plugin_exec_mode();
        if should_skip_dynamic_route(name, exec_mode) {
            if env::cli_verbose_enabled() || env::debug_plugin() {
                get_global_ring0().log.debug(&format!(
                    "[plugin/route] skip dynamic route box={} mode={:?} reason=core_module_first",
                    name, exec_mode
                ));
            }
            return Err(RuntimeError::InvalidOperation {
                message: format!(
                    "[freeze:contract][plugin/route] dynamic route skipped for {} in mode={:?}",
                    name, exec_mode
                ),
            });
        }

        // Use the existing v2 plugin system
        let registry = get_global_registry();

        if let Some(_provider) = registry.get_provider(name) {
            registry
                .create_box(name, args)
                .map_err(|e| RuntimeError::InvalidOperation {
                    message: format!("Plugin Box creation failed: {}", e),
                })
        } else {
            Err(RuntimeError::InvalidOperation {
                message: format!("No plugin provider for Box type: {}", name),
            })
        }
    }

    fn box_types(&self) -> Vec<&str> {
        // TODO: Get list from BoxFactoryRegistry
        // For now, return empty as registry doesn't expose this yet
        vec![]
    }

    fn is_available(&self) -> bool {
        // Check if any plugins are loaded
        let _registry = get_global_registry();
        // TODO: Add method to check if registry has any providers
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_module_first_box_set_contract() {
        assert!(is_core_module_first_box("ArrayBox"));
        assert!(is_core_module_first_box("StringBox"));
        assert!(is_core_module_first_box("MapBox"));
        assert!(is_core_module_first_box("ConsoleBox"));
        assert!(is_core_module_first_box("FileBox"));
        assert!(is_core_module_first_box("PathBox"));
        assert!(!is_core_module_first_box("MathBox"));
        assert!(!is_core_module_first_box("NetClientBox"));
    }

    #[test]
    fn should_skip_dynamic_route_core4_contract() {
        assert!(should_skip_dynamic_route(
            "StringBox",
            env::PluginExecMode::ModuleFirst
        ));
        assert!(should_skip_dynamic_route(
            "ArrayBox",
            env::PluginExecMode::ModuleFirst
        ));
        assert!(should_skip_dynamic_route(
            "MapBox",
            env::PluginExecMode::ModuleFirst
        ));
        assert!(should_skip_dynamic_route(
            "ConsoleBox",
            env::PluginExecMode::ModuleFirst
        ));
        assert!(!should_skip_dynamic_route(
            "StringBox",
            env::PluginExecMode::DynamicOnly
        ));
        assert!(!should_skip_dynamic_route(
            "StringBox",
            env::PluginExecMode::DynamicFirst
        ));
    }

    #[test]
    fn should_skip_dynamic_route_file_path_contract() {
        assert!(should_skip_dynamic_route(
            "FileBox",
            env::PluginExecMode::ModuleFirst
        ));
        assert!(should_skip_dynamic_route(
            "PathBox",
            env::PluginExecMode::ModuleFirst
        ));
    }

    #[test]
    fn should_keep_dynamic_route_math_net_compat_contract() {
        assert!(!should_skip_dynamic_route(
            "MathBox",
            env::PluginExecMode::ModuleFirst
        ));
        assert!(!should_skip_dynamic_route(
            "NetClientBox",
            env::PluginExecMode::ModuleFirst
        ));
    }
}
