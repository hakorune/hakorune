//! BindingMapProvider - Trait to abstract binding_map access
//!
//! Centralizes feature-gated binding map access, eliminating scattered
//! #[cfg] guards across promoters and patterns.
//!
//! # Design Philosophy
//!
//! This trait follows the **Single Point of Control** principle:
//! - **Before Phase 79**: `#[cfg(feature = "normalized_dev")]` guards scattered across 10+ locations
//! - **After Phase 79**: Feature gate centralized in 2 locations (trait + impl)
//!
//! # Benefits
//!
//! 1. **Maintainability**: Change feature gate logic in one place
//! 2. **Readability**: Request structs no longer need feature-gated fields
//! 3. **Testability**: Mock implementations for testing
//! 4. **Consistency**: Uniform access pattern across all promoters
//!
//! # Usage Example
//!
//! ```rust,ignore
//! // Before Phase 79: Scattered #[cfg] guards
//! #[cfg(feature = "normalized_dev")]
//! let binding_map = Some(&builder.binding_map);
//! #[cfg(not(feature = "normalized_dev"))]
//! let binding_map = None;
//!
//! // After Phase 79: Clean trait call
//! let binding_map = builder.get_binding_map();
//! ```

use crate::mir::binding_id::BindingId;
use std::collections::BTreeMap;

/// Trait to provide optional binding_map access (dev-only).
///
/// This trait abstracts the feature-gated access to binding_map,
/// allowing code to request binding information without knowing
/// whether the feature is enabled.
pub trait BindingMapProvider {
    /// Get binding map if available (dev-only).
    ///
    /// Returns Some(&BTreeMap) when `normalized_dev` feature is enabled,
    /// None otherwise.
    fn get_binding_map(&self) -> Option<&BTreeMap<String, BindingId>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementation for testing
    struct MockBuilder {
        #[cfg(feature = "normalized_dev")]
        binding_map: BTreeMap<String, BindingId>,
    }

    impl BindingMapProvider for MockBuilder {
        #[cfg(feature = "normalized_dev")]
        fn get_binding_map(&self) -> Option<&BTreeMap<String, BindingId>> {
            Some(&self.binding_map)
        }

        #[cfg(not(feature = "normalized_dev"))]
        fn get_binding_map(&self) -> Option<&BTreeMap<String, BindingId>> {
            None
        }
    }

    #[test]
    #[cfg(feature = "normalized_dev")]
    fn test_binding_map_provider_dev() {
        let mut map = BTreeMap::new();
        map.insert("x".to_string(), BindingId(1));
        map.insert("y".to_string(), BindingId(2));

        let builder = MockBuilder { binding_map: map };

        let result = builder.get_binding_map();
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    #[cfg(not(feature = "normalized_dev"))]
    fn test_binding_map_provider_non_dev() {
        let builder = MockBuilder {};

        let result = builder.get_binding_map();
        assert!(result.is_none());
    }

    #[test]
    fn test_trait_object_compatibility() {
        // Verify trait can be used as trait object
        #[cfg(feature = "normalized_dev")]
        let builder: Box<dyn BindingMapProvider> = Box::new(MockBuilder {
            binding_map: BTreeMap::new(),
        });

        #[cfg(not(feature = "normalized_dev"))]
        let builder: Box<dyn BindingMapProvider> = Box::new(MockBuilder {});

        let _result = builder.get_binding_map();
        // Test just verifies compilation and trait object usage
    }
}
