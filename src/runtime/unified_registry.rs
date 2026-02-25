/*!
 * Global Unified Box Registry
 *
 * Manages the global instance of UnifiedBoxRegistry
 * Integrates all Box creation sources (builtin, user-defined, plugin)
 */

use crate::box_factory::builtin::BuiltinBoxFactory;
#[cfg(feature = "plugins")]
use crate::box_factory::plugin::PluginBoxFactory;
use crate::box_factory::UnifiedBoxRegistry;
use std::sync::{Arc, Mutex, OnceLock};

/// Global registry instance
static GLOBAL_REGISTRY: OnceLock<Arc<Mutex<UnifiedBoxRegistry>>> = OnceLock::new();

/// Initialize the global unified registry
pub fn init_global_unified_registry() {
    GLOBAL_REGISTRY.get_or_init(|| {
        // Phase 15.5: Use environment variable policy (StrictPluginFirst for "Everything is Plugin")
        let mut registry = UnifiedBoxRegistry::with_env_policy();
        // Default: enable builtins unless building with feature "plugins-only"
        #[cfg(not(feature = "plugins-only"))]
        {
            registry.register(std::sync::Arc::new(BuiltinBoxFactory::new()));
        }

        // Register plugin Box factory (primary)
        #[cfg(feature = "plugins")]
        {
            registry.register(Arc::new(PluginBoxFactory::new()));
        }

        // TODO: User-defined Box factory will be registered by interpreter

        // Phase 15.5: FactoryPolicy determines actual priority order
        // StrictPluginFirst: plugins > user > builtin (SOLVES StringBox/IntegerBox issue)
        // BuiltinFirst: builtin > user > plugin (legacy default)

        Arc::new(Mutex::new(registry))
    });
}

/// Get the global unified registry
pub fn get_global_unified_registry() -> Arc<Mutex<UnifiedBoxRegistry>> {
    init_global_unified_registry();
    GLOBAL_REGISTRY.get().unwrap().clone()
}

/// Register a user-defined Box factory (called by interpreter)
pub fn register_user_defined_factory(factory: Arc<dyn crate::box_factory::BoxFactory>) {
    let registry = get_global_unified_registry();
    let mut registry_lock = registry.lock().unwrap();

    // Phase 25.1b: delegate to policy-aware register() so that
    // type_cache is rebuilt and user-defined Box types (HakoCli など)
    // are correctly advertised to the registry. Priorityは
    // FactoryPolicy + factory_type に従って決まる。
    registry_lock.register(factory);
}
