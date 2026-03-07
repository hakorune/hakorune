//! Phase 126 + Phase 141 P1.5: available_inputs collector (SSOT)
//!
//! ## Responsibility
//!
//! - Collect available_inputs from function params + prefix variables + CapturedEnv
//! - Returns BTreeMap<String, ValueId> with deterministic order
//! - No AST inference (SSOT sources only)
//!
//! ## Design
//!
//! - Input sources (priority order):
//!   1. Function params (from ScopeContext + VariableContext) - highest priority
//!   2. Prefix variables (from builder.variable_map) - medium priority (Phase 141 P1.5)
//!   3. CapturedEnv (pinned/captured variables from outer scope) - lowest priority
//! - Forbidden: AST-based capture inference (Phase 100 CapturedEnv is SSOT)

// Phase 126: Import contexts from MirBuilder (pub(in crate::mir) visibility)
// Since we're in crate::mir::control_tree::normalized_shadow, we can access these
use crate::mir::builder::MirBuilder;
use crate::mir::loop_route_detection::function_scope_capture::CapturedEnv;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Phase 126: Box-First available_inputs collector
pub struct AvailableInputsCollectorBox;

impl AvailableInputsCollectorBox {
    /// Collect available_inputs from SSOT sources (via MirBuilder)
    ///
    /// ## Contract (Phase 141 P1.5)
    ///
    /// - Sources (priority order):
    ///   1. Function params: builder.scope_ctx.function_param_names + builder.variable_ctx.variable_map
    ///   2. Prefix variables: prefix_variables (from builder.variable_map at call site)
    ///   3. CapturedEnv: captured_env.vars (pinned/captured from outer scope)
    /// - Output: BTreeMap<String, ValueId> (deterministic order)
    /// - No AST inference: only use pre-computed SSOT sources
    ///
    /// ## Implementation (Phase 141 P1.5)
    ///
    /// - Collect function params first (highest priority)
    /// - Collect prefix variables (medium priority, don't override params)
    /// - Collect CapturedEnv vars (lowest priority, don't override params or prefix)
    /// - Use BTreeMap for deterministic iteration
    pub fn collect(
        builder: &MirBuilder,
        captured_env: Option<&CapturedEnv>,
        prefix_variables: Option<&BTreeMap<String, ValueId>>,
    ) -> BTreeMap<String, ValueId> {
        let mut available_inputs = BTreeMap::new();

        // 1. Function params (SSOT: scope_ctx + variable_ctx) - highest priority
        for param_name in &builder.scope_ctx.function_param_names {
            if let Some(value_id) = builder.variable_ctx.lookup(param_name) {
                available_inputs.insert(param_name.clone(), value_id);
            }
        }

        // 2. Prefix variables (medium priority) - Phase 141 P1.5
        if let Some(prefix) = prefix_variables {
            for (name, value_id) in prefix {
                // Don't override function params (params have higher priority)
                if !available_inputs.contains_key(name) {
                    available_inputs.insert(name.clone(), *value_id);
                }
            }
        }

        // 3. CapturedEnv (SSOT: pinned/captured vars) - lowest priority
        if let Some(env) = captured_env {
            for var in &env.vars {
                // Don't override function params or prefix vars
                if !available_inputs.contains_key(&var.name) {
                    available_inputs.insert(var.name.clone(), var.host_id);
                }
            }
        }

        available_inputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_empty() {
        let builder = MirBuilder::new();
        let result = AvailableInputsCollectorBox::collect(&builder, None, None);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_collect_function_params() {
        let mut builder = MirBuilder::new();

        // Simulate function param: x -> ValueId(1)
        builder.scope_ctx.function_param_names.insert("x".to_string());
        builder.variable_ctx.insert("x".to_string(), ValueId(1));

        let result = AvailableInputsCollectorBox::collect(&builder, None, None);
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("x"), Some(&ValueId(1)));
    }

    #[test]
    fn test_collect_captured_env() {
        let builder = MirBuilder::new();

        let mut captured = CapturedEnv::new();
        captured.insert("outer_x".to_string(), ValueId(42));

        let result = AvailableInputsCollectorBox::collect(&builder, Some(&captured), None);
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("outer_x"), Some(&ValueId(42)));
    }

    #[test]
    fn test_collect_params_override_captured() {
        let mut builder = MirBuilder::new();

        // Function param: x -> ValueId(1)
        builder.scope_ctx.function_param_names.insert("x".to_string());
        builder.variable_ctx.insert("x".to_string(), ValueId(1));

        // Captured: x -> ValueId(42) (should be ignored)
        let mut captured = CapturedEnv::new();
        captured.insert("x".to_string(), ValueId(42));

        let result = AvailableInputsCollectorBox::collect(&builder, Some(&captured), None);
        assert_eq!(result.len(), 1);
        // Function param (ValueId(1)) should win over captured (ValueId(42))
        assert_eq!(result.get("x"), Some(&ValueId(1)));
    }

    #[test]
    fn test_collect_deterministic_order() {
        let mut builder = MirBuilder::new();

        // Add params in non-alphabetical order
        builder.scope_ctx.function_param_names.insert("z".to_string());
        builder.scope_ctx.function_param_names.insert("a".to_string());
        builder.scope_ctx.function_param_names.insert("m".to_string());
        builder.variable_ctx.insert("z".to_string(), ValueId(3));
        builder.variable_ctx.insert("a".to_string(), ValueId(1));
        builder.variable_ctx.insert("m".to_string(), ValueId(2));

        let result = AvailableInputsCollectorBox::collect(&builder, None, None);
        let keys: Vec<_> = result.keys().collect();

        // BTreeMap ensures alphabetical order
        assert_eq!(keys, vec![&"a".to_string(), &"m".to_string(), &"z".to_string()]);
    }

    // Phase 141 P1.5: New tests for prefix variables

    #[test]
    fn test_collect_with_prefix_variables() {
        let builder = MirBuilder::new();

        let mut prefix_vars = BTreeMap::new();
        prefix_vars.insert("s".to_string(), ValueId(42));
        prefix_vars.insert("flag".to_string(), ValueId(43));

        let result = AvailableInputsCollectorBox::collect(&builder, None, Some(&prefix_vars));
        assert_eq!(result.len(), 2);
        assert_eq!(result.get("s"), Some(&ValueId(42)));
        assert_eq!(result.get("flag"), Some(&ValueId(43)));
    }

    #[test]
    fn test_collect_params_override_prefix() {
        let mut builder = MirBuilder::new();

        // Function param: x -> ValueId(1)
        builder.scope_ctx.function_param_names.insert("x".to_string());
        builder.variable_ctx.insert("x".to_string(), ValueId(1));

        // Prefix: x -> ValueId(42) (should be ignored)
        let mut prefix_vars = BTreeMap::new();
        prefix_vars.insert("x".to_string(), ValueId(42));

        let result = AvailableInputsCollectorBox::collect(&builder, None, Some(&prefix_vars));
        assert_eq!(result.len(), 1);
        // Function param (ValueId(1)) should win over prefix (ValueId(42))
        assert_eq!(result.get("x"), Some(&ValueId(1)));
    }

    #[test]
    fn test_collect_priority_order() {
        let mut builder = MirBuilder::new();

        // Function param: x -> ValueId(1)
        builder.scope_ctx.function_param_names.insert("x".to_string());
        builder.variable_ctx.insert("x".to_string(), ValueId(1));

        // Prefix: x -> ValueId(2), y -> ValueId(3)
        let mut prefix_vars = BTreeMap::new();
        prefix_vars.insert("x".to_string(), ValueId(2));
        prefix_vars.insert("y".to_string(), ValueId(3));

        // Captured: x -> ValueId(4), y -> ValueId(5), z -> ValueId(6)
        let mut captured = CapturedEnv::new();
        captured.insert("x".to_string(), ValueId(4));
        captured.insert("y".to_string(), ValueId(5));
        captured.insert("z".to_string(), ValueId(6));

        let result = AvailableInputsCollectorBox::collect(&builder, Some(&captured), Some(&prefix_vars));
        assert_eq!(result.len(), 3);
        // x: param wins (ValueId(1))
        assert_eq!(result.get("x"), Some(&ValueId(1)));
        // y: prefix wins (ValueId(3))
        assert_eq!(result.get("y"), Some(&ValueId(3)));
        // z: captured wins (ValueId(6))
        assert_eq!(result.get("z"), Some(&ValueId(6)));
    }
}
