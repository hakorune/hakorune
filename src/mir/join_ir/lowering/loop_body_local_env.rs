//! Phase 184: Loop Body Local Variable Environment
//!
//! This module provides a storage box for body-local variables in loop routes.
//! It collects local variable declarations from loop body AST and maintains
//! name → JoinIR ValueId mappings.
//!
//! ## Design Philosophy
//!
//! **Single Responsibility**: This module ONLY handles body-local variable storage.
//! It does NOT:
//! - Resolve condition variables (that's ConditionEnv)
//! - Perform variable resolution priority logic (that's ScopeManager)
//! - Lower AST to JoinIR (that's JoinIrBuilder)
//!
//! ## Box-First Design
//!
//! Following 箱理論 (Box Theory) principles:
//! - **Single purpose**: Store body-local variable mappings
//! - **Clear boundaries**: Only body-scope variables, not condition variables
//! - **Deterministic**: BTreeMap ensures consistent ordering

use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Environment for loop body-local variables
///
/// Maps variable names to JoinIR-local ValueIds for variables declared
/// within the loop body (not in conditions).
///
/// # Example
///
/// ```nyash
/// loop(i < 5) {
///     local temp = i * 2  // Body-local: temp
///     sum = sum + temp
///     i = i + 1
/// }
/// ```
///
/// LoopBodyLocalEnv would contain: `{ "temp" → ValueId(5) }`
#[derive(Debug, Clone, Default)]
pub struct LoopBodyLocalEnv {
    /// Body-local variable name → JoinIR ValueId mapping
    ///
    /// BTreeMap ensures deterministic iteration order (important for PHI generation)
    locals: BTreeMap<String, ValueId>,
}

impl LoopBodyLocalEnv {
    /// Create a new empty environment
    pub fn new() -> Self {
        Self {
            locals: BTreeMap::new(),
        }
    }

    /// Create an environment from loop body AST nodes
    ///
    /// This method scans the loop body for local variable declarations
    /// and collects their JoinIR ValueIds.
    ///
    /// # Arguments
    ///
    /// * `body_locals` - List of (name, ValueId) pairs from body analysis
    ///
    /// # Example
    ///
    /// ```ignore
    /// let body_locals = vec![
    ///     ("temp".to_string(), ValueId(5)),
    ///     ("digit".to_string(), ValueId(6)),
    /// ];
    /// let env = LoopBodyLocalEnv::from_locals(body_locals);
    /// assert_eq!(env.get("temp"), Some(ValueId(5)));
    /// ```
    pub fn from_locals(body_locals: Vec<(String, ValueId)>) -> Self {
        let mut locals = BTreeMap::new();
        for (name, value_id) in body_locals {
            locals.insert(name, value_id);
        }
        Self { locals }
    }

    /// Insert a body-local variable binding
    ///
    /// # Arguments
    ///
    /// * `name` - Variable name (e.g., "temp", "digit")
    /// * `join_id` - JoinIR-local ValueId for this variable
    pub fn insert(&mut self, name: String, join_id: ValueId) {
        self.locals.insert(name, join_id);
    }

    /// Look up a body-local variable by name
    ///
    /// Returns `Some(ValueId)` if the variable exists in the environment,
    /// `None` otherwise.
    pub fn get(&self, name: &str) -> Option<ValueId> {
        self.locals.get(name).copied()
    }

    /// Check if a body-local variable exists in the environment
    pub fn contains(&self, name: &str) -> bool {
        self.locals.contains_key(name)
    }

    /// Get the number of body-local variables in the environment
    pub fn len(&self) -> usize {
        self.locals.len()
    }

    /// Check if the environment is empty
    pub fn is_empty(&self) -> bool {
        self.locals.is_empty()
    }

    /// Get an iterator over all (name, ValueId) pairs
    ///
    /// Iteration order is deterministic (sorted by name) due to BTreeMap.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &ValueId)> {
        self.locals.iter()
    }

    /// Get all variable names (sorted)
    pub fn names(&self) -> Vec<String> {
        self.locals.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_body_local_env() {
        let env = LoopBodyLocalEnv::new();
        assert!(env.is_empty());
        assert_eq!(env.len(), 0);
        assert_eq!(env.names(), Vec::<String>::new());
    }

    #[test]
    fn test_single_body_local() {
        let mut env = LoopBodyLocalEnv::new();
        env.insert("temp".to_string(), ValueId(5));

        assert!(!env.is_empty());
        assert_eq!(env.len(), 1);
        assert!(env.contains("temp"));
        assert_eq!(env.get("temp"), Some(ValueId(5)));
        assert_eq!(env.names(), vec!["temp".to_string()]);
    }

    #[test]
    fn test_multiple_body_locals() {
        let mut env = LoopBodyLocalEnv::new();
        env.insert("temp".to_string(), ValueId(5));
        env.insert("digit".to_string(), ValueId(6));
        env.insert("ch".to_string(), ValueId(7));

        assert_eq!(env.len(), 3);
        assert_eq!(env.get("temp"), Some(ValueId(5)));
        assert_eq!(env.get("digit"), Some(ValueId(6)));
        assert_eq!(env.get("ch"), Some(ValueId(7)));

        // BTreeMap ensures sorted keys
        let names = env.names();
        assert_eq!(names, vec!["ch", "digit", "temp"]);
    }

    #[test]
    fn test_get_nonexistent() {
        let env = LoopBodyLocalEnv::new();
        assert_eq!(env.get("nonexistent"), None);
        assert!(!env.contains("nonexistent"));
    }

    #[test]
    fn test_from_locals() {
        let body_locals = vec![
            ("temp".to_string(), ValueId(5)),
            ("digit".to_string(), ValueId(6)),
        ];
        let env = LoopBodyLocalEnv::from_locals(body_locals);

        assert_eq!(env.len(), 2);
        assert_eq!(env.get("temp"), Some(ValueId(5)));
        assert_eq!(env.get("digit"), Some(ValueId(6)));
    }

    #[test]
    fn test_iter_deterministic_order() {
        let mut env = LoopBodyLocalEnv::new();
        // Insert in non-alphabetical order
        env.insert("zebra".to_string(), ValueId(3));
        env.insert("apple".to_string(), ValueId(1));
        env.insert("mango".to_string(), ValueId(2));

        // Iteration should be sorted
        let names: Vec<_> = env.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names, vec!["apple", "mango", "zebra"]);
    }

    #[test]
    fn test_overwrite_existing() {
        let mut env = LoopBodyLocalEnv::new();
        env.insert("temp".to_string(), ValueId(5));
        env.insert("temp".to_string(), ValueId(10)); // Overwrite

        assert_eq!(env.len(), 1);
        assert_eq!(env.get("temp"), Some(ValueId(10)));
    }
}
