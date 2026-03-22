//! Phase 136 follow-up (Step 4/7): BindingContext extraction
//!
//! Consolidates variable binding management:
//! - binding_map: String -> BindingId mapping (parallel to variable_map)
//! - BindingId allocation (via CoreContext.next_binding())
//! - Scope restoration logic (stored in ScopeContext frames)
//!
//! ## Relationship with other contexts:
//! - **CoreContext**: Allocates BindingId via next_binding()
//! - **ScopeContext**: Manages lexical scope frames with restore_binding data
//! - **TypeContext**: Independent (type tracking vs binding tracking)
//!
//! ## Design:
//! - BindingId tracks variable binding identity (survives SSA renaming)
//! - Parallel to ValueId (variable_map), but for binding semantics
//! - Restored on scope exit (see LexicalScopeFrame.restore_binding)
//!
//! Phase 74: BindingId system introduction
//! Phase 136 Step 4/7: Extraction into dedicated context

use hakorune_mir_core::BindingId;
use std::collections::BTreeMap;

/// Phase 136 Step 4/7: Binding context for variable binding management
///
/// Manages the mapping from variable names to their BindingId.
/// Parallel to `variable_map` (String -> ValueId), but tracks binding identity.
///
/// ## Key responsibilities:
/// - Maintain current binding_map (String -> BindingId)
/// - Provide lookup/insertion/removal operations
/// - Work with ScopeContext for scope-based restoration
///
/// ## Implementation note:
/// - Uses BTreeMap for deterministic iteration (Phase 25.1 consistency)
/// - BindingId allocation is delegated to CoreContext.next_binding()
#[derive(Debug, Clone)]
pub struct BindingContext {
    /// Phase 74: BindingId mapping for lexical variable bindings
    /// Maps variable names to their current BindingId.
    /// Parallel to `variable_map` (String -> ValueId), but tracks binding identity.
    /// Restored on lexical scope exit (see ScopeContext restore_binding).
    pub(super) binding_map: BTreeMap<String, BindingId>,
}

impl Default for BindingContext {
    fn default() -> Self {
        Self::new()
    }
}

impl BindingContext {
    /// Create a new BindingContext
    pub fn new() -> Self {
        Self {
            binding_map: BTreeMap::new(),
        }
    }

    /// Lookup a variable's BindingId
    pub fn lookup(&self, name: &str) -> Option<BindingId> {
        self.binding_map.get(name).copied()
    }

    /// Insert a variable binding
    pub fn insert(&mut self, name: String, binding_id: BindingId) {
        self.binding_map.insert(name, binding_id);
    }

    /// Remove a variable binding
    pub fn remove(&mut self, name: &str) -> Option<BindingId> {
        self.binding_map.remove(name)
    }

    pub fn clear_for_function_entry(&mut self) {
        self.binding_map.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_context_basic() {
        let mut ctx = BindingContext::new();
        assert!(ctx.binding_map.is_empty());
        assert_eq!(ctx.binding_map.len(), 0);

        let bid = BindingId::new(0);
        ctx.insert("x".to_string(), bid);
        assert_eq!(ctx.lookup("x"), Some(bid));
        assert_eq!(ctx.binding_map.len(), 1);
        assert!(!ctx.binding_map.is_empty());

        ctx.remove("x");
        assert_eq!(ctx.lookup("x"), None);
        assert!(ctx.binding_map.is_empty());
    }

    #[test]
    fn test_binding_context_contains() {
        let mut ctx = BindingContext::new();
        assert!(!ctx.binding_map.contains_key("x"));

        ctx.insert("x".to_string(), BindingId::new(0));
        assert!(ctx.binding_map.contains_key("x"));
    }

    #[test]
    fn test_binding_map_access() {
        let mut ctx = BindingContext::new();
        ctx.insert("a".to_string(), BindingId::new(1));
        ctx.insert("b".to_string(), BindingId::new(2));

        assert_eq!(ctx.binding_map.len(), 2);
        assert_eq!(ctx.binding_map.get("a"), Some(&BindingId::new(1)));
        assert_eq!(ctx.binding_map.get("b"), Some(&BindingId::new(2)));
    }
}
