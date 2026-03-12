//! Phase 136 follow-up (Step 5/7): VariableContext extraction
//!
//! Consolidates variable name → ValueId mapping management:
//! - variable_map: String -> ValueId mapping (SSA conversion tracking)
//! - Used extensively by JoinIR lowering for carrier tracking
//! - Critical for PHI node generation in if/loop route handling
//!
//! ## Relationship with other contexts:
//! - **BindingContext**: String -> BindingId (binding identity, parallel to variable_map)
//! - **TypeContext**: ValueId -> MirType (type information for ValueIds)
//! - **ScopeContext**: Manages lexical scope frames with variable restoration
//! - **CoreContext**: Allocates ValueId via next_value()
//!
//! ## Design:
//! - variable_map tracks current SSA values for named variables
//! - Used by JoinIR CarrierInfo::from_variable_map() for loop carrier tracking
//! - PHI nodes in if/loop route handling update variable_map with merged values
//! - NYASH_TRACE_VARMAP debug feature visualizes variable_map changes
//!
//! ## JoinIR Integration:
//! - CarrierInfo::from_variable_map(): Extracts loop carriers from variable_map
//! - ExitLine contract: Ensures carriers are present in variable_map
//! - LoopBreak / IfPhiJoin / LoopContinueOnly: Track carrier variables across loop iterations
//!
//! Phase 25.1: HashMap → BTreeMap for deterministic PHI generation
//! Phase 136 Step 5/7: Extraction into dedicated context

use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Phase 136 Step 5/7: Variable context for variable name → ValueId mapping
///
/// Manages the mapping from variable names to their current SSA ValueId.
/// This is the core data structure for SSA conversion and variable tracking.
///
/// ## Key responsibilities:
/// - Maintain current variable_map (String -> ValueId)
/// - Provide lookup/insertion/removal operations
/// - Support JoinIR carrier tracking via CarrierInfo::from_variable_map()
/// - Enable NYASH_TRACE_VARMAP debugging of variable mappings
///
/// ## Implementation note:
/// - Uses BTreeMap for deterministic iteration (Phase 25.1 - PHI generation consistency)
/// - ValueId allocation is delegated to CoreContext.next_value()
/// - Parallel to BindingContext (which tracks BindingId instead of ValueId)
#[derive(Debug, Clone)]
pub struct VariableContext {
    /// Variable name to ValueId mapping (for SSA conversion)
    ///
    /// ## Usage patterns:
    /// - Variable assignment: `variable_map["x"] = new_value_id`
    /// - Variable access: `let value_id = variable_map["x"]`
    /// - PHI merging: Update variable_map with merged PHI ValueId
    /// - JoinIR carriers: Extract via CarrierInfo::from_variable_map(&variable_map)
    ///
    /// ## Examples:
    /// ```text
    /// // Simple variable tracking:
    /// variable_map["x"] = ValueId(5)
    /// variable_map["sum"] = ValueId(10)
    ///
    /// // After PHI merge in if-statement:
    /// variable_map["result"] = ValueId(42)  // PHI(%then_val, %else_val)
    ///
    /// // Loop carrier tracking (LoopBreak / IfPhiJoin / LoopContinueOnly):
    /// variable_map["i"] = ValueId(7)       // Loop variable
    /// variable_map["acc"] = ValueId(11)    // Accumulator (carrier)
    /// ```
    ///
    /// Phase 25.1: HashMap → BTreeMap for deterministic PHI generation
    pub(super) variable_map: BTreeMap<String, ValueId>,
}

impl Default for VariableContext {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl VariableContext {
    /// Create a new VariableContext with empty variable_map
    pub fn new() -> Self {
        Self {
            variable_map: BTreeMap::new(),
        }
    }

    /// Lookup a variable's current ValueId
    ///
    /// Returns None if the variable is not in scope or not yet assigned.
    pub fn lookup(&self, name: &str) -> Option<ValueId> {
        self.variable_map.get(name).copied()
    }

    /// Require a variable's ValueId (fail-fast variant of lookup)
    ///
    /// Returns the ValueId if the variable exists, otherwise returns Err.
    ///
    /// ## Use cases:
    /// - Scan/Split/Predicate routes: Variable extraction with clear error messages
    /// - JoinIR lowering: Carrier/invariant resolution
    ///
    /// ## Example:
    /// ```ignore
    /// let i_init = builder.variable_ctx.require(&parts.i_var, "split_scan")?;
    /// let s_host = builder.variable_ctx.require(&parts.s_var, "split_scan")?;
    /// ```
    ///
    /// Phase 272 P0.2 Refactoring: Eliminate variable_map.get().ok_or() boilerplate
    pub fn require(&self, name: &str, context: &str) -> Result<ValueId, String> {
        self.variable_map.get(name).copied().ok_or_else(|| {
            format!(
                "[{}] Variable '{}' not found in variable_map",
                context, name
            )
        })
    }

    /// Insert or update a variable's ValueId
    ///
    /// ## Important notes:
    /// - **__pin$ temporaries**: NEVER insert __pin$ prefixed names
    ///   (Step 5-5-F/G: __pin$ are transient compiler temporaries, not real variables)
    /// - **SSA renaming**: Each assignment creates a new ValueId
    /// - **PHI merging**: Update with merged PHI ValueId after if/loop
    pub fn insert(&mut self, name: String, value_id: ValueId) {
        self.variable_map.insert(name, value_id);
    }

    /// Remove a variable from the map
    ///
    /// Returns the previous ValueId if the variable existed.
    /// Used for scope restoration and cleanup.
    pub fn remove(&mut self, name: &str) -> Option<ValueId> {
        self.variable_map.remove(name)
    }

    /// Get immutable reference to the variable_map
    ///
    /// ## Use cases:
    /// - JoinIR: `CarrierInfo::from_variable_map(&variable_map)`
    /// - PHI generation: Iterate over variables to detect changes
    /// - Debugging: NYASH_TRACE_VARMAP visualization
    pub fn variable_map(&self) -> &BTreeMap<String, ValueId> {
        &self.variable_map
    }

    /// Get mutable reference to the variable_map
    ///
    /// ## Use cases:
    /// - Bulk operations (clone, swap, replace)
    /// - Legacy code migration (temporary during Phase 136)
    pub fn variable_map_mut(&mut self) -> &mut BTreeMap<String, ValueId> {
        &mut self.variable_map
    }

    /// Check if a variable is currently mapped
    pub fn contains(&self, name: &str) -> bool {
        self.variable_map.contains_key(name)
    }

    /// Get the number of variables in the map
    pub fn len(&self) -> usize {
        self.variable_map.len()
    }

    /// Check if there are no variables in the map
    pub fn is_empty(&self) -> bool {
        self.variable_map.is_empty()
    }

    /// Clone the current variable_map (for snapshot/restore patterns)
    ///
    /// ## Use cases:
    /// - Before if-statement: Save pre-if variable_map
    /// - Before loop: Save pre-loop variable_map
    /// - PHI generation: Compare pre/post variable_map to detect changes
    pub fn snapshot(&self) -> BTreeMap<String, ValueId> {
        self.variable_map.clone()
    }

    /// Restore variable_map from a snapshot
    ///
    /// ## Use cases:
    /// - After if-then branch: Restore pre-if state before else branch
    /// - Scope exit: Restore outer scope's variable_map
    pub fn restore(&mut self, snapshot: BTreeMap<String, ValueId>) {
        self.variable_map = snapshot;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_context_basic() {
        let mut ctx = VariableContext::new();
        assert!(ctx.is_empty());
        assert_eq!(ctx.len(), 0);

        let vid = ValueId::new(42);
        ctx.insert("x".to_string(), vid);
        assert_eq!(ctx.lookup("x"), Some(vid));
        assert_eq!(ctx.len(), 1);
        assert!(!ctx.is_empty());

        ctx.remove("x");
        assert_eq!(ctx.lookup("x"), None);
        assert!(ctx.is_empty());
    }

    #[test]
    fn test_variable_context_contains() {
        let mut ctx = VariableContext::new();
        assert!(!ctx.contains("x"));

        ctx.insert("x".to_string(), ValueId::new(1));
        assert!(ctx.contains("x"));
    }

    #[test]
    fn test_variable_map_access() {
        let mut ctx = VariableContext::new();
        ctx.insert("a".to_string(), ValueId::new(10));
        ctx.insert("b".to_string(), ValueId::new(20));

        let map = ctx.variable_map();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("a"), Some(&ValueId::new(10)));
        assert_eq!(map.get("b"), Some(&ValueId::new(20)));
    }

    #[test]
    fn test_snapshot_restore() {
        let mut ctx = VariableContext::new();
        ctx.insert("x".to_string(), ValueId::new(1));
        ctx.insert("y".to_string(), ValueId::new(2));

        // Take snapshot
        let snapshot = ctx.snapshot();
        assert_eq!(snapshot.len(), 2);

        // Modify context
        ctx.insert("z".to_string(), ValueId::new(3));
        assert_eq!(ctx.len(), 3);

        // Restore snapshot
        ctx.restore(snapshot);
        assert_eq!(ctx.len(), 2);
        assert!(ctx.contains("x"));
        assert!(ctx.contains("y"));
        assert!(!ctx.contains("z"));
    }

    #[test]
    fn test_btree_deterministic_iteration() {
        let mut ctx = VariableContext::new();
        ctx.insert("z".to_string(), ValueId::new(3));
        ctx.insert("a".to_string(), ValueId::new(1));
        ctx.insert("m".to_string(), ValueId::new(2));

        // BTreeMap should maintain sorted order
        let keys: Vec<_> = ctx.variable_map().keys().cloned().collect();
        assert_eq!(keys, vec!["a", "m", "z"]);
    }

    #[test]
    fn test_ssa_renaming_pattern() {
        let mut ctx = VariableContext::new();

        // Initial assignment: x = 1
        ctx.insert("x".to_string(), ValueId::new(1));
        assert_eq!(ctx.lookup("x"), Some(ValueId::new(1)));

        // SSA renaming: x = 2 (new ValueId)
        ctx.insert("x".to_string(), ValueId::new(2));
        assert_eq!(ctx.lookup("x"), Some(ValueId::new(2)));

        // PHI merge: x = PHI(ValueId(2), ValueId(3))
        ctx.insert("x".to_string(), ValueId::new(4));
        assert_eq!(ctx.lookup("x"), Some(ValueId::new(4)));
    }
}
