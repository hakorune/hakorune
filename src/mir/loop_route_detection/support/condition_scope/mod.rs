//! Phase 170-D: Loop Condition Scope Analysis Box
//!
//! Analyzes which variables appear in loop conditions (header, break, continue)
//! and classifies them by their scope:
//! - LoopParam: The loop parameter itself (e.g., 'i' in loop(i < 10))
//! - OuterLocal: Variables from outer scope (pre-existing before loop)
//! - `LoopBodyLocal`: body-local variables defined inside the loop body
//!
//! This Box enables `loop_break` / `loop_continue_only` routes to determine if they can handle a given loop's
//! condition expressions, providing clear Fail-Fast when conditions reference
//! unsupported loop-body variables.

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use std::collections::HashSet;

mod var_analyzer;

/// Scope classification for a condition variable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CondVarScope {
    /// The loop parameter itself (e.g., 'i' in loop(i < 10))
    LoopParam,
    /// A variable from outer scope, defined before the loop
    OuterLocal,
    /// A body-local variable defined inside the loop body
    LoopBodyLocal,
}

/// Information about a single condition variable
#[derive(Debug, Clone)]
pub struct CondVarInfo {
    pub name: String,
    pub scope: CondVarScope,
}

/// Analysis result for all variables in loop conditions
#[derive(Debug, Clone)]
pub struct LoopConditionScope {
    pub vars: Vec<CondVarInfo>,
}

impl LoopConditionScope {
    /// Create a new empty condition scope
    pub fn new() -> Self {
        LoopConditionScope { vars: Vec::new() }
    }

    /// Check if this scope contains any body-local variables
    pub fn has_loop_body_local(&self) -> bool {
        self.vars
            .iter()
            .any(|v| v.scope == CondVarScope::LoopBodyLocal)
    }

    /// Check if all variables in this scope are in the allowed set
    pub fn all_in(&self, allowed: &[CondVarScope]) -> bool {
        self.vars.iter().all(|v| allowed.contains(&v.scope))
    }

    /// Get all variable names as a set
    pub fn var_names(&self) -> HashSet<String> {
        self.vars.iter().map(|v| v.name.clone()).collect()
    }

    /// Add a variable to this scope (avoiding duplicates by name)
    ///
    /// # Phase 170-ultrathink: Scope Priority
    ///
    /// If a variable already exists with a different scope, keep the more restrictive one:
    /// - LoopParam > OuterLocal > `LoopBodyLocal` (in terms of priority)
    ///
    /// This ensures that if a variable is classified in multiple ways due to
    /// different analysis passes or ambiguous AST structures, we prefer the
    /// most specific/restrictive classification.
    ///
    /// Example: If 'i' is detected as both LoopParam and OuterLocal,
    /// we keep LoopParam (more specific).
    pub fn add_var(&mut self, name: String, scope: CondVarScope) {
        // Check if variable already exists
        if let Some(existing) = self.vars.iter_mut().find(|v| v.name == name) {
            // Phase 170-ultrathink: Keep more restrictive scope
            // Priority: LoopParam (highest) > OuterLocal > `LoopBodyLocal` (lowest)
            let new_is_more_restrictive = match (existing.scope, scope) {
                // Same scope → no change
                (CondVarScope::LoopParam, CondVarScope::LoopParam) => false,
                (CondVarScope::OuterLocal, CondVarScope::OuterLocal) => false,
                (CondVarScope::LoopBodyLocal, CondVarScope::LoopBodyLocal) => false,
                // LoopParam is always most restrictive
                (_, CondVarScope::LoopParam) => true,
                (CondVarScope::LoopParam, _) => false,
                // OuterLocal is more restrictive than LoopBodyLocal
                (CondVarScope::LoopBodyLocal, CondVarScope::OuterLocal) => true,
                (CondVarScope::OuterLocal, CondVarScope::LoopBodyLocal) => false,
            };

            if new_is_more_restrictive {
                existing.scope = scope;
            }
        } else {
            // Variable doesn't exist yet → add it
            self.vars.push(CondVarInfo { name, scope });
        }
    }
}

impl Default for LoopConditionScope {
    fn default() -> Self {
        Self::new()
    }
}

/// Collect body-local variable names from condition scope analysis.
pub fn extract_loop_body_local_names(vars: &[CondVarInfo]) -> Vec<&String> {
    vars.iter()
        .filter(|v| v.scope == CondVarScope::LoopBodyLocal)
        .map(|v| &v.name)
        .collect()
}

/// Phase 170-D: Loop Condition Scope Analysis Box
///
/// This is the main analyzer that determines variable scopes for condition expressions.
pub struct LoopConditionScopeBox;

impl LoopConditionScopeBox {
    /// Analyze condition variable scopes for a loop
    ///
    /// # Arguments
    ///
    /// * `loop_param_name` - Name of the loop parameter (e.g., "i" in loop(i < 10))
    /// * `condition_nodes` - Array of AST nodes containing conditions to analyze
    /// * `scope` - LoopScopeShape with information about variable definitions
    ///
    /// # Returns
    ///
    /// LoopConditionScope with classified variables
    ///
    /// # Algorithm
    ///
    /// 1. Extract all variables from condition AST nodes using the local variable analyzer
    /// 2. Classify each variable:
    ///    - If matches loop_param_name → LoopParam
    ///    - Else if in outer scope (via the local variable analyzer) → OuterLocal
    ///    - Else → `LoopBodyLocal` (conservative body-local default)
    #[allow(dead_code)]
    pub(crate) fn analyze(
        loop_param_name: &str,
        condition_nodes: &[&ASTNode],
        scope: Option<&LoopScopeShape>,
    ) -> LoopConditionScope {
        let mut result = LoopConditionScope::new();
        let mut found_vars = HashSet::new();

        // Phase 170-D-impl-2: Use condition variable analyzer for extraction
        for node in condition_nodes {
            let vars = var_analyzer::extract_all_variables(node);
            found_vars.extend(vars);
        }

        // Classify each variable
        for var_name in found_vars {
            let var_scope = if var_name == loop_param_name {
                CondVarScope::LoopParam
            } else if var_analyzer::is_outer_scope_variable(&var_name, scope) {
                CondVarScope::OuterLocal
            } else {
                // Default: assume it is body-local if not identified as outer
                CondVarScope::LoopBodyLocal
            };

            result.add_var(var_name, var_scope);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_scope_new() {
        let scope = LoopConditionScope::new();
        assert!(scope.vars.is_empty());
        assert!(!scope.has_loop_body_local());
    }

    #[test]
    fn test_add_var() {
        let mut scope = LoopConditionScope::new();
        scope.add_var("i".to_string(), CondVarScope::LoopParam);
        scope.add_var("end".to_string(), CondVarScope::OuterLocal);

        assert_eq!(scope.vars.len(), 2);
        assert!(!scope.has_loop_body_local());
    }

    #[test]
    fn test_has_loop_body_local() {
        let mut scope = LoopConditionScope::new();
        scope.add_var("i".to_string(), CondVarScope::LoopParam);
        scope.add_var("ch".to_string(), CondVarScope::LoopBodyLocal);

        assert!(scope.has_loop_body_local());
    }

    #[test]
    fn test_all_in() {
        let mut scope = LoopConditionScope::new();
        scope.add_var("i".to_string(), CondVarScope::LoopParam);
        scope.add_var("end".to_string(), CondVarScope::OuterLocal);

        assert!(scope.all_in(&[CondVarScope::LoopParam, CondVarScope::OuterLocal]));
        assert!(!scope.all_in(&[CondVarScope::LoopParam]));
    }

    #[test]
    fn test_var_names() {
        let mut scope = LoopConditionScope::new();
        scope.add_var("i".to_string(), CondVarScope::LoopParam);
        scope.add_var("end".to_string(), CondVarScope::OuterLocal);

        let names = scope.var_names();
        assert!(names.contains("i"));
        assert!(names.contains("end"));
        assert_eq!(names.len(), 2);
    }
}
