use super::{BoxFactory, FactoryPolicy, FactoryType, RuntimeError};
use crate::box_trait::NyashBox;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Registry that manages all BoxFactory implementations
pub struct UnifiedBoxRegistry {
    /// Ordered list of factories with policy-based priority
    pub factories: Vec<Arc<dyn BoxFactory>>,

    /// Quick lookup cache for performance
    type_cache: RwLock<HashMap<String, usize>>, // maps type name to factory index

    /// Factory priority policy (Phase 15.5: Everything is Plugin)
    policy: FactoryPolicy,
}

impl UnifiedBoxRegistry {
    /// Create a new empty registry with default policy
    /// Phase 86: Default changed to StrictPluginFirst via with_env_policy()
    pub fn new() -> Self {
        Self::with_env_policy()
    }

    /// Create a new empty registry with specified policy
    pub fn with_policy(policy: FactoryPolicy) -> Self {
        Self {
            factories: Vec::new(),
            type_cache: RwLock::new(HashMap::new()),
            policy,
        }
    }

    /// Create registry with policy from environment variable (Phase 15.5 setup)
    pub fn with_env_policy() -> Self {
        let policy = match crate::config::env::box_factory_policy().as_deref() {
            Some("compat_plugin_first") => FactoryPolicy::CompatPluginFirst,
            Some("builtin_first") => FactoryPolicy::BuiltinFirst,
            Some("strict_plugin_first") | _ => FactoryPolicy::StrictPluginFirst,
        };

        if crate::config::env::cli_verbose_enabled() {
            let ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
            ring0.log.debug(&format!(
                "[UnifiedBoxRegistry] 🎯 Factory Policy: {:?} (Phase 15.5: Everything is Plugin!)",
                policy
            ));
        }
        Self::with_policy(policy)
    }

    /// Get current factory policy
    pub fn get_policy(&self) -> FactoryPolicy {
        self.policy
    }

    /// Set factory policy and rebuild cache to reflect new priorities
    pub fn set_policy(&mut self, policy: FactoryPolicy) {
        if self.policy != policy {
            self.policy = policy;
            self.rebuild_cache();
        }
    }

    /// Rebuild type cache based on current policy
    fn rebuild_cache(&mut self) {
        let mut cache = self.type_cache.write().unwrap();
        cache.clear();

        let factory_order = self.get_factory_order_by_policy();
        for &factory_index in factory_order.iter() {
            if let Some(factory) = self.factories.get(factory_index) {
                let types = factory.box_types();
                for type_name in types {
                    if is_reserved_type(type_name) && !factory.is_builtin_factory() {
                        let ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
                        ring0.log.error(&format!(
                            "[UnifiedBoxRegistry] ❌ Rejecting registration of reserved type '{}' by non-builtin factory #{}",
                            type_name, factory_index
                        ));
                        continue;
                    }

                    let entry = cache.entry(type_name.to_string());
                    use std::collections::hash_map::Entry;
                    match entry {
                        Entry::Occupied(existing) => {
                            if crate::config::env::cli_verbose_enabled() {
                                let ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
                                ring0.log.warn(&format!("[UnifiedBoxRegistry] ⚠️ Policy '{}': type '{}' kept by higher priority factory #{}, ignoring factory #{}",
                                          format!("{:?}", self.policy), existing.key(), existing.get(), factory_index));
                            }
                        }
                        Entry::Vacant(v) => {
                            v.insert(factory_index);
                        }
                    }
                }
            }
        }
    }

    /// Get factory indices ordered by current policy priority
    fn get_factory_order_by_policy(&self) -> Vec<usize> {
        let mut factory_indices: Vec<usize> = (0..self.factories.len()).collect();
        factory_indices.sort_by_key(|&index| {
            if let Some(factory) = self.factories.get(index) {
                let factory_type = factory.factory_type();
                match self.policy {
                    FactoryPolicy::StrictPluginFirst => match factory_type {
                        FactoryType::Plugin => 0,
                        FactoryType::User => 1,
                        FactoryType::Builtin => 2,
                    },
                    FactoryPolicy::CompatPluginFirst => match factory_type {
                        FactoryType::Plugin => 0,
                        FactoryType::Builtin => 1,
                        FactoryType::User => 2,
                    },
                    FactoryPolicy::BuiltinFirst => match factory_type {
                        FactoryType::Builtin => 0,
                        FactoryType::User => 1,
                        FactoryType::Plugin => 2,
                    },
                }
            } else {
                999
            }
        });
        factory_indices
    }

    /// Register a new factory (policy-aware)
    pub fn register(&mut self, factory: Arc<dyn BoxFactory>) {
        self.factories.push(factory);
        self.rebuild_cache();
    }

    /// Create a Box using the unified interface
    pub fn create_box(
        &self,
        name: &str,
        args: &[Box<dyn NyashBox>],
    ) -> Result<Box<dyn NyashBox>, RuntimeError> {
        let plugins_disabled = crate::config::env::disable_plugins();
        if !plugins_disabled && crate::config::env::use_plugin_builtins() {
            use crate::runtime::{get_global_registry, BoxProvider};
            let allow: Vec<String> =
                crate::config::env::plugin_override_types().unwrap_or_default();
            if allow.iter().any(|t| t == name) {
                let v2 = get_global_registry();
                if let Some(provider) = v2.get_provider(name) {
                    if let BoxProvider::Plugin(_lib) = provider {
                        return v2.create_box(name, args).map_err(|e| {
                            RuntimeError::InvalidOperation {
                                message: format!("Plugin Box creation failed: {}", e),
                            }
                        });
                    }
                }
            }
        }

        let cache = self.type_cache.read().unwrap();
        if let Some(&factory_index) = cache.get(name) {
            if let Some(factory) = self.factories.get(factory_index) {
                if factory.is_available() {
                    return factory.create_box(name, args);
                }
            }
        }
        drop(cache);

        let mut last_error: Option<RuntimeError> = None;
        let factory_order = self.get_factory_order_by_policy();
        for &factory_index in factory_order.iter() {
            if let Some(factory) = self.factories.get(factory_index) {
                if !factory.is_available() {
                    continue;
                }

                let box_types = factory.box_types();
                if !box_types.is_empty() && !box_types.contains(&name) {
                    continue;
                }

                if crate::config::env::debug_plugin() {
                    let ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
                    ring0.log.debug(&format!(
                        "[UnifiedBoxRegistry] try factory#{} {:?} for {}",
                        factory_index,
                        factory.factory_type(),
                        name
                    ));
                }

                match factory.create_box(name, args) {
                    Ok(boxed) => return Ok(boxed),
                    Err(e) => {
                        if name == "FileBox" {
                            if let Some(fallback_result) = self.try_filebox_fallback(name, args, &e)
                            {
                                return fallback_result;
                            }
                        }
                        last_error = Some(e);
                        continue;
                    }
                }
            }
        }

        {
            let v2 = crate::runtime::get_global_registry();
            if let Some(_prov) = v2.get_provider(name) {
                if let Ok(b) = v2.create_box(name, args) {
                    return Ok(b);
                }
            }
        }

        if let Some(err) = last_error {
            Err(err)
        } else {
            Err(RuntimeError::InvalidOperation {
                message: format!("Unknown Box type: {}", name),
            })
        }
    }

    /// Try FileBox fallback based on NYASH_FILEBOX_MODE
    /// Returns Some(result) if fallback is applicable, None if should continue trying other factories
    fn try_filebox_fallback(
        &self,
        name: &str,
        args: &[Box<dyn NyashBox>],
        original_error: &RuntimeError,
    ) -> Option<Result<Box<dyn NyashBox>, RuntimeError>> {
        use crate::runner::modes::common_util::provider_registry;

        let mode = provider_registry::read_filebox_mode_from_env();
        match mode {
            provider_registry::FileBoxMode::PluginOnly => {
                let ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
                ring0.log.error(&format!(
                    "[FileBox] Plugin creation failed in plugin-only mode: {}",
                    original_error
                ));
                Some(Err(RuntimeError::InvalidOperation {
                    message: format!(
                        "FileBox plugin creation failed (plugin-only mode): {}",
                        original_error
                    ),
                }))
            }
            provider_registry::FileBoxMode::Auto => {
                if crate::config::env::cli_verbose_enabled() {
                    let ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
                    ring0.log.debug(&format!(
                        "[FileBox] Plugin creation failed, falling back to builtin/core-ro: {}",
                        original_error
                    ));
                }

                for factory in &self.factories {
                    if factory.is_builtin_factory() && factory.box_types().contains(&name) {
                        match factory.create_box(name, args) {
                            Ok(boxed) => {
                                if crate::config::env::cli_verbose_enabled() {
                                    let ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
                                    ring0.log.debug(
                                        "[FileBox] Successfully created with builtin factory",
                                    );
                                }
                                return Some(Ok(boxed));
                            }
                            Err(e) => {
                                let ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
                                ring0.log.error(&format!(
                                    "[FileBox] Builtin factory also failed: {}",
                                    e
                                ));
                            }
                        }
                    }
                }
                None
            }
            provider_registry::FileBoxMode::CoreRo => {
                if crate::config::env::cli_verbose_enabled() {
                    let ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
                    ring0
                        .log
                        .debug("[FileBox] Using core-ro mode, trying builtin factory");
                }
                for factory in &self.factories {
                    if factory.is_builtin_factory() && factory.box_types().contains(&name) {
                        match factory.create_box(name, args) {
                            Ok(boxed) => return Some(Ok(boxed)),
                            Err(e) => {
                                return Some(Err(RuntimeError::InvalidOperation {
                                    message: format!("FileBox core-ro creation failed: {}", e),
                                }));
                            }
                        }
                    }
                }
                None
            }
        }
    }

    /// Check whether a type name is known to the registry
    pub fn has_type(&self, name: &str) -> bool {
        {
            let cache = self.type_cache.read().unwrap();
            if let Some(&idx) = cache.get(name) {
                if let Some(factory) = self.factories.get(idx) {
                    if factory.is_available() {
                        return true;
                    }
                }
            }
        }
        for factory in &self.factories {
            if !factory.is_available() {
                continue;
            }
            let types = factory.box_types();
            if !types.is_empty() && types.contains(&name) {
                return true;
            }
        }
        false
    }

    /// Get all available Box types
    pub fn available_types(&self) -> Vec<String> {
        let mut types = Vec::new();
        for factory in &self.factories {
            if factory.is_available() {
                for type_name in factory.box_types() {
                    types.push(type_name.to_string());
                }
            }
        }
        types.sort();
        types.dedup();
        types
    }
}

fn is_reserved_type(name: &str) -> bool {
    use crate::runtime::CoreBoxId;

    if crate::config::env::use_plugin_builtins() {
        if let Some(types) = crate::config::env::plugin_override_types() {
            if types.iter().any(|t| t == name) {
                return false;
            }
        }
    }

    CoreBoxId::from_name(name)
        .map(|id| id.is_core_required() || matches!(id, CoreBoxId::Result | CoreBoxId::Method))
        .unwrap_or(false)
}
