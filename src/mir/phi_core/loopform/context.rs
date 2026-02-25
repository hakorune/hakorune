//! Phase 191: LoopForm Context - ValueId Management
//!
//! Responsibility:
//! - Track next available ValueId
//! - Parameter reservation
//! - Counter initialization
//!
//! Box-First principle: Make boundaries explicit for ValueId allocation,
//! enabling substitution and clear separation of concerns.

use crate::mir::ValueId;
use std::collections::BTreeMap;

/// LoopForm PHI generation context
///
/// Manages ValueId allocation for PHI construction, providing a clear boundary
/// for value numbering. This is the foundation for PHI building.
#[derive(Debug, Clone)]
pub struct LoopFormContext {
    /// Next ValueId to allocate
    pub next_value_id: u32,
    /// Parameter count (number of reserved ValueIds)
    pub param_count: usize,
}

impl LoopFormContext {
    /// Create a new context from parameter count and existing variables
    ///
    /// # Arguments
    /// * `param_count` - Number of function parameters
    /// * `existing_vars` - Existing variables (local variables, etc.)
    ///
    /// # Returns
    /// A new context with next_value_id set after parameters and existing variables
    pub fn new(param_count: usize, existing_vars: &BTreeMap<String, ValueId>) -> Self {
        // Reserve space for parameters
        let min_from_params = param_count as u32;
        // Find the highest existing ValueId
        let min_from_vars = existing_vars.values().map(|v| v.0 + 1).max().unwrap_or(0);

        Self {
            next_value_id: min_from_params.max(min_from_vars),
            param_count,
        }
    }

    /// Ensure next_value_id is after the given max_id
    ///
    /// Used to avoid conflicts with ValueIds allocated outside this context.
    pub fn ensure_after(&mut self, max_id: u32) {
        if self.next_value_id <= max_id {
            self.next_value_id = max_id + 1;
        }
    }

    /// Get next ValueId and increment counter
    pub fn next_value(&mut self) -> ValueId {
        let id = ValueId(self.next_value_id);
        self.next_value_id += 1;
        id
    }

    /// Allocate multiple ValueIds at once
    ///
    /// # Arguments
    /// * `count` - Number of ValueIds to allocate
    ///
    /// # Returns
    /// A vector of allocated ValueIds
    pub fn alloc_multiple(&mut self, count: usize) -> Vec<ValueId> {
        (0..count).map(|_| self.next_value()).collect()
    }

    /// Peek at the next ValueId without consuming it
    pub fn peek(&self) -> ValueId {
        ValueId(self.next_value_id)
    }

    /// Get the current counter value (raw u32)
    pub fn current_counter(&self) -> u32 {
        self.next_value_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_new_empty() {
        let ctx = LoopFormContext::new(0, &BTreeMap::new());
        assert_eq!(ctx.peek(), ValueId(0));
        assert_eq!(ctx.param_count, 0);
    }

    #[test]
    fn test_context_new_with_params() {
        let ctx = LoopFormContext::new(2, &BTreeMap::new());
        assert_eq!(ctx.peek(), ValueId(2));
        assert_eq!(ctx.param_count, 2);
    }

    #[test]
    fn test_context_new_with_existing_vars() {
        let mut vars = BTreeMap::new();
        vars.insert("x".to_string(), ValueId(3));
        vars.insert("y".to_string(), ValueId(5));

        let ctx = LoopFormContext::new(2, &vars);
        // Should start after the highest existing var (5 + 1 = 6)
        assert_eq!(ctx.peek(), ValueId(6));
    }

    #[test]
    fn test_context_next_value() {
        let mut ctx = LoopFormContext::new(0, &BTreeMap::new());
        assert_eq!(ctx.next_value(), ValueId(0));
        assert_eq!(ctx.next_value(), ValueId(1));
        assert_eq!(ctx.next_value(), ValueId(2));
    }

    #[test]
    fn test_context_alloc_multiple() {
        let mut ctx = LoopFormContext::new(0, &BTreeMap::new());
        let ids = ctx.alloc_multiple(3);
        assert_eq!(ids, vec![ValueId(0), ValueId(1), ValueId(2)]);
        assert_eq!(ctx.peek(), ValueId(3));
    }

    #[test]
    fn test_context_ensure_after() {
        let mut ctx = LoopFormContext::new(0, &BTreeMap::new());
        assert_eq!(ctx.peek(), ValueId(0));

        ctx.ensure_after(10);
        assert_eq!(ctx.peek(), ValueId(11));

        // Should not go backwards
        ctx.ensure_after(5);
        assert_eq!(ctx.peek(), ValueId(11));
    }

    #[test]
    fn test_context_current_counter() {
        let ctx = LoopFormContext::new(3, &BTreeMap::new());
        assert_eq!(ctx.current_counter(), 3);
    }
}
