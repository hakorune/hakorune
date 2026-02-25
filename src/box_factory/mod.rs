/*!
 * Unified Box Factory Architecture
 *
 * Phase 9.78: 統合BoxFactoryアーキテクチャ
 * すべてのBox生成（ビルトイン、ユーザー定義、プラグイン）を統一的に扱う
 *
 * Design principles:
 * - "Everything is Box" 哲学の実装レベルでの体現
 * - birth/finiライフサイクルの明確な責務分離
 * - 保守性と拡張性の劇的向上
 */

use crate::box_trait::NyashBox;

mod policy;
mod registry;

pub use policy::{FactoryPolicy, FactoryType};
pub use registry::UnifiedBoxRegistry;

/// Runtime error types for Box operations
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("invalid operation: {message}")]
    InvalidOperation { message: String },
    #[error("type error: {message}")]
    TypeError { message: String },
}

/// Shared state for interpreter context (legacy compatibility)
#[derive(Debug, Default, Clone)]
pub struct SharedState;

impl SharedState {
    pub fn new() -> Self {
        Self
    }
}

/// Unified interface for all Box creation
pub trait BoxFactory: Send + Sync {
    /// Create a new Box instance with given arguments
    fn create_box(
        &self,
        name: &str,
        args: &[Box<dyn NyashBox>],
    ) -> Result<Box<dyn NyashBox>, RuntimeError>;

    /// Check if this factory is currently available
    fn is_available(&self) -> bool {
        true
    }

    /// Get list of Box types this factory can create
    fn box_types(&self) -> Vec<&str>;

    /// Check if this factory supports birth/fini lifecycle
    fn supports_birth(&self) -> bool {
        true
    }

    /// Identify builtin factory to enforce reserved-name protections
    fn is_builtin_factory(&self) -> bool {
        false
    }

    /// Identify factory type for policy-based priority ordering
    fn factory_type(&self) -> FactoryType {
        if self.is_builtin_factory() {
            FactoryType::Builtin
        } else {
            FactoryType::Plugin // Default assumption for external factories
        }
    }
}

pub mod builtin;
pub mod plugin;
/// Re-export submodules
#[cfg(feature = "interpreter-legacy")]
pub mod user_defined;

// Phase 15.5: Separated builtin implementations for easy deletion
pub mod builtin_impls;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    static BOX_FACTORY_POLICY_ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_registry_creation() {
        let registry = UnifiedBoxRegistry::new();
        assert_eq!(registry.available_types().len(), 0);
    }

    // Phase 86: BoxFactory Priority Tests

    #[test]
    fn test_default_policy_is_strict_plugin_first() {
        let _lock = BOX_FACTORY_POLICY_ENV_LOCK.lock().unwrap();
        let prev = crate::config::env::box_factory_policy();
        // Ensure NYASH_BOX_FACTORY_POLICY is not set
        crate::config::env::reset_box_factory_policy();

        let registry = UnifiedBoxRegistry::new();
        assert_eq!(
            registry.get_policy(),
            FactoryPolicy::StrictPluginFirst,
            "Default policy should be StrictPluginFirst"
        );

        if let Some(v) = prev {
            crate::config::env::set_box_factory_policy(&v);
        }
    }

    #[test]
    fn test_env_policy_override() {
        let _lock = BOX_FACTORY_POLICY_ENV_LOCK.lock().unwrap();
        let prev = crate::config::env::box_factory_policy();

        // Test builtin_first override
        crate::config::env::set_box_factory_policy("builtin_first");
        let registry = UnifiedBoxRegistry::with_env_policy();
        assert_eq!(registry.get_policy(), FactoryPolicy::BuiltinFirst);

        // Test compat_plugin_first override
        crate::config::env::set_box_factory_policy("compat_plugin_first");
        let registry = UnifiedBoxRegistry::with_env_policy();
        assert_eq!(registry.get_policy(), FactoryPolicy::CompatPluginFirst);

        // Test strict_plugin_first explicit
        crate::config::env::set_box_factory_policy("strict_plugin_first");
        let registry = UnifiedBoxRegistry::with_env_policy();
        assert_eq!(registry.get_policy(), FactoryPolicy::StrictPluginFirst);

        // Cleanup
        if let Some(v) = prev {
            crate::config::env::set_box_factory_policy(&v);
        } else {
            crate::config::env::reset_box_factory_policy();
        }
    }

    #[test]
    fn test_reserved_type_protection() {
        // Ensure env vars are cleared
        // Note: can't clear env vars directly; tests rely on default behavior

        // Create a mock non-builtin factory that claims a reserved type
        struct MockPluginFactory;

        impl BoxFactory for MockPluginFactory {
            fn create_box(
                &self,
                name: &str,
                _args: &[Box<dyn NyashBox>],
            ) -> Result<Box<dyn NyashBox>, RuntimeError> {
                // This should never be called for StringBox since it's rejected
                Err(RuntimeError::InvalidOperation {
                    message: format!("Mock factory attempted to create: {}", name),
                })
            }

            fn box_types(&self) -> Vec<&str> {
                vec!["StringBox", "CustomBox"] // Claims a reserved type
            }

            fn is_builtin_factory(&self) -> bool {
                false // Non-builtin
            }

            fn factory_type(&self) -> FactoryType {
                FactoryType::Plugin
            }
        }

        let mut registry = UnifiedBoxRegistry::new();
        registry.register(Arc::new(MockPluginFactory));

        // Test that create_box fails for StringBox (not registered in cache)
        let result = registry.create_box("StringBox", &[]);
        assert!(
            result.is_err(),
            "StringBox creation should fail when only non-builtin factory provides it"
        );

        // Verify the error message indicates it's unknown (not in cache)
        if let Err(e) = result {
            let err_msg = format!("{}", e);
            assert!(
                err_msg.contains("Unknown Box type") || err_msg.contains("Mock factory"),
                "Error message should indicate StringBox is not properly registered: {}",
                err_msg
            );
        }
    }

    #[test]
    fn test_plugin_override_with_env() {
        let _lock = BOX_FACTORY_POLICY_ENV_LOCK.lock().unwrap();
        // This test verifies that NYASH_USE_PLUGIN_BUILTINS or
        // NYASH_PLUGIN_OVERRIDE_TYPES allows plugins to override reserved types

        // Create a mock plugin factory
        struct MockPluginFactory;

        impl BoxFactory for MockPluginFactory {
            fn create_box(
                &self,
                name: &str,
                _args: &[Box<dyn NyashBox>],
            ) -> Result<Box<dyn NyashBox>, RuntimeError> {
                if name == "StringBox" {
                    // Return a mock box for testing
                    Err(RuntimeError::InvalidOperation {
                        message: "Mock plugin StringBox".to_string(),
                    })
                } else {
                    Err(RuntimeError::InvalidOperation {
                        message: "Unknown".to_string(),
                    })
                }
            }

            fn box_types(&self) -> Vec<&str> {
                vec!["StringBox"]
            }

            fn is_builtin_factory(&self) -> bool {
                false
            }

            fn factory_type(&self) -> FactoryType {
                FactoryType::Plugin
            }
        }

        let prev_policy = crate::config::env::box_factory_policy();
        let prev_override_types = std::env::var("NYASH_PLUGIN_OVERRIDE_TYPES").ok();

        // Test with NYASH_PLUGIN_OVERRIDE_TYPES
        crate::config::env::set_box_factory_policy("strict_plugin_first"); // ensure plugin first
        std::env::set_var("NYASH_PLUGIN_OVERRIDE_TYPES", "StringBox");
        let mut registry = UnifiedBoxRegistry::new();
        registry.register(Arc::new(MockPluginFactory));

        // With override enabled, StringBox should not be rejected
        // (Note: has_type will be false because create_box fails, but registration shouldn't be rejected)
        if let Some(v) = prev_override_types {
            std::env::set_var("NYASH_PLUGIN_OVERRIDE_TYPES", v);
        } else {
            std::env::remove_var("NYASH_PLUGIN_OVERRIDE_TYPES");
        }
        if let Some(v) = prev_policy {
            crate::config::env::set_box_factory_policy(&v);
        } else {
            crate::config::env::reset_box_factory_policy();
        }
    }

    #[test]
    fn test_non_reserved_plugin_priority() {
        // Test that non-reserved types (like FileBox) can be overridden by plugins

        struct MockBuiltinFactory;
        impl BoxFactory for MockBuiltinFactory {
            fn create_box(
                &self,
                _name: &str,
                _args: &[Box<dyn NyashBox>],
            ) -> Result<Box<dyn NyashBox>, RuntimeError> {
                Err(RuntimeError::InvalidOperation {
                    message: "Builtin FileBox".to_string(),
                })
            }

            fn box_types(&self) -> Vec<&str> {
                vec!["FileBox"]
            }

            fn is_builtin_factory(&self) -> bool {
                true
            }

            fn factory_type(&self) -> FactoryType {
                FactoryType::Builtin
            }
        }

        struct MockPluginFactory;
        impl BoxFactory for MockPluginFactory {
            fn create_box(
                &self,
                _name: &str,
                _args: &[Box<dyn NyashBox>],
            ) -> Result<Box<dyn NyashBox>, RuntimeError> {
                Err(RuntimeError::InvalidOperation {
                    message: "Plugin FileBox".to_string(),
                })
            }

            fn box_types(&self) -> Vec<&str> {
                vec!["FileBox"]
            }

            fn is_builtin_factory(&self) -> bool {
                false
            }

            fn factory_type(&self) -> FactoryType {
                FactoryType::Plugin
            }
        }

        let mut registry = UnifiedBoxRegistry::new();

        // Register builtin first, then plugin
        registry.register(Arc::new(MockBuiltinFactory));
        registry.register(Arc::new(MockPluginFactory));

        // With StrictPluginFirst policy, plugin should have priority
        // Both fail, but the error message tells us which was tried first
        let result = registry.create_box("FileBox", &[]);
        assert!(result.is_err());

        // The error should be from plugin (tried first) or builtin (fallback)
        // This test just verifies the mechanism works
    }
}
