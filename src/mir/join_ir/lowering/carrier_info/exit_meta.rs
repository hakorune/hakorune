use super::types::{ExitMeta, JoinFragmentMeta};
use crate::mir::ValueId;
use std::collections::BTreeSet;

impl JoinFragmentMeta {
    /// Create JoinFragmentMeta for expression-result shape
    ///
    /// Use when the loop returns a value (like `return loop(...)`).
    pub fn with_expr_result(expr_result: ValueId, exit_meta: ExitMeta) -> Self {
        Self {
            expr_result: Some(expr_result),
            exit_meta,
            continuation_funcs: BTreeSet::new(),
        }
    }

    /// Create JoinFragmentMeta for carrier-only shape
    ///
    /// Use when the loop only updates carriers (like the trim route).
    pub fn carrier_only(exit_meta: ExitMeta) -> Self {
        Self {
            expr_result: None,
            exit_meta,
            continuation_funcs: BTreeSet::new(),
        }
    }

    /// Create empty JoinFragmentMeta (no expr result, no carriers)
    pub fn empty() -> Self {
        Self {
            expr_result: None,
            exit_meta: ExitMeta::empty(),
            continuation_funcs: BTreeSet::new(),
        }
    }

    /// Check if this fragment has an expression result
    pub fn has_expr_result(&self) -> bool {
        self.expr_result.is_some()
    }

    /// Phase 33-14: Backward compatibility - convert to ExitMeta
    ///
    /// During migration, some code may still expect ExitMeta.
    /// This extracts just the carrier bindings.
    #[deprecated(since = "33-14", note = "Use exit_meta directly for carrier access")]
    pub fn to_exit_meta(&self) -> ExitMeta {
        self.exit_meta.clone()
    }
}

impl ExitMeta {
    /// Create new ExitMeta with no exit values
    pub fn empty() -> Self {
        Self {
            exit_values: vec![],
        }
    }

    /// Create ExitMeta with a single exit value
    pub fn single(carrier_name: String, join_value: ValueId) -> Self {
        Self {
            exit_values: vec![(carrier_name, join_value)],
        }
    }

    /// Create ExitMeta with multiple exit values
    pub fn multiple(exit_values: Vec<(String, ValueId)>) -> Self {
        Self { exit_values }
    }

    /// Phase 193-2: Get the count of exit bindings
    ///
    /// Useful for checking if this ExitMeta has any exit values.
    pub fn binding_count(&self) -> usize {
        self.exit_values.len()
    }

    /// Phase 193-2: Check if this has any exit values
    pub fn is_empty(&self) -> bool {
        self.exit_values.is_empty()
    }

    /// Phase 193-2: Find a binding by carrier name
    ///
    /// Lookup a specific exit value by carrier name.
    pub fn find_binding(&self, carrier_name: &str) -> Option<ValueId> {
        self.exit_values
            .iter()
            .find(|(name, _)| name == carrier_name)
            .map(|(_, value_id)| *value_id)
    }

    /// Phase 193-2: Add a binding to ExitMeta
    ///
    /// Convenient way to build ExitMeta incrementally.
    pub fn with_binding(mut self, carrier_name: String, join_value: ValueId) -> Self {
        self.exit_values.push((carrier_name, join_value));
        self
    }
}
