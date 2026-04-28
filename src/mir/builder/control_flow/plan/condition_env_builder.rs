//! ConditionEnvBuilder - Unified ConditionEnv construction
//!
//! Phase 171-172: Issue 5
//!
//! Provides unified construction methods for ConditionEnv and ConditionBindings
//! used in loop_break route condition analysis.
//!
//! # Responsibility
//!
//! - Extract condition variables from AST nodes
//! - Allocate JoinIR-local ValueIds for condition-only variables
//! - Build ConditionEnv mapping (variable name → JoinIR ValueId)
//! - Create ConditionBindings for host↔JoinIR value mapping
//!
//! # Usage
//!
//! ```rust
//! // Standard break condition analysis (Phase 201+)
//! let mut space = JoinValueSpace::new();
//! let (env, bindings, loop_var_join_id) = ConditionEnvBuilder::build_for_break_condition_v2(
//!     break_condition,
//!     &loop_var_name,
//!     &variable_map,
//!     loop_var_id,
//!     &mut space,
//! )?;
//! ```

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::condition_env::{ConditionBinding, ConditionEnv};
use crate::mir::join_ir::lowering::condition_to_joinir::extract_condition_variables;
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::ValueId;
use std::collections::BTreeMap;

pub(crate) struct ConditionEnvBuilder;

impl ConditionEnvBuilder {
    /// Phase 201: Build ConditionEnv using JoinValueSpace (disjoint ValueId regions)
    ///
    /// This method uses JoinValueSpace to allocate ValueIds, ensuring that
    /// param IDs (100+) never collide with local IDs (1000+) used by JoinIR lowering.
    ///
    /// # Arguments
    ///
    /// * `break_condition` - AST node for the break condition
    /// * `loop_var_name` - Loop parameter name (excluded from condition-only variables)
    /// * `variable_map` - HOST function's variable_map (for looking up HOST ValueIds)
    /// * `loop_var_id` - HOST ValueId for the loop parameter
    /// * `space` - JoinValueSpace for unified ValueId allocation
    ///
    /// # Returns
    ///
    /// Tuple of:
    /// - ConditionEnv: Variable name → JoinIR ValueId mapping
    /// - Vec<ConditionBinding>: HOST↔JoinIR value mappings for merge
    /// - loop_var_join_id: The JoinIR ValueId allocated for the loop parameter
    pub fn build_for_break_condition_v2(
        break_condition: &ASTNode,
        loop_var_name: &str,
        variable_map: &BTreeMap<String, ValueId>,
        _loop_var_id: ValueId,
        space: &mut JoinValueSpace,
    ) -> Result<(ConditionEnv, Vec<ConditionBinding>, ValueId), String> {
        // Extract all variables used in the condition (excluding loop parameter)
        let condition_var_names =
            extract_condition_variables(break_condition, &[loop_var_name.to_string()]);

        let mut env = ConditionEnv::new();
        let mut bindings = Vec::new();

        // Phase 201: Allocate loop parameter ValueId from JoinValueSpace (Param region)
        let loop_var_join_id = space.alloc_param();
        env.insert(loop_var_name.to_string(), loop_var_join_id);

        // Phase 79-2: Register loop variable BindingId (dev-only)
        // NOTE: We don't have access to builder.binding_map here, so this registration
        // needs to happen at the call site (loop_break lowerer/orchestrator,
        // if_phi_join lowering path, etc.)
        // This comment serves as a reminder for future developers.

        // For each condition variable, allocate JoinIR-local ValueId and build binding
        for var_name in &condition_var_names {
            let host_id = variable_map.get(var_name).copied().ok_or_else(|| {
                format!(
                    "Condition variable '{}' not found in variable_map. \
                         Loop condition references undefined variable.",
                    var_name
                )
            })?;

            // Phase 201: Allocate from Param region to avoid collision with locals
            let join_id = space.alloc_param();

            env.insert(var_name.clone(), join_id);
            bindings.push(ConditionBinding {
                name: var_name.clone(),
                host_value: host_id,
                join_value: join_id,
            });
        }

        Ok((env, bindings, loop_var_join_id))
    }

    /// Phase 201: Build ConditionEnv with loop parameter only using JoinValueSpace
    ///
    /// Uses JoinValueSpace to allocate the loop parameter ValueId.
    pub fn build_loop_param_only_v2(
        loop_var_name: &str,
        space: &mut JoinValueSpace,
    ) -> (ConditionEnv, ValueId) {
        let mut env = ConditionEnv::new();
        let loop_var_join_id = space.alloc_param();
        env.insert(loop_var_name.to_string(), loop_var_join_id);
        (env, loop_var_join_id)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};

    #[test]
    fn test_build_for_break_condition_no_extra_vars() {
        // Condition: i < 10 (only uses loop parameter)
        let condition = ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(10),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(100));

        let mut space = JoinValueSpace::new();
        let (env, bindings, loop_var_join_id) = ConditionEnvBuilder::build_for_break_condition_v2(
            &condition,
            "i",
            &variable_map,
            ValueId(100),
            &mut space,
        )
        .unwrap();

        // Should have loop parameter in env
        assert!(env.get("i").is_some());

        // Loop var should be in param region (100+)
        assert!(loop_var_join_id.0 >= 100);

        // Should have no condition-only bindings
        assert_eq!(bindings.len(), 0);
    }

    #[test]
    fn test_build_for_break_condition_with_extra_var() {
        // Condition: i < max (uses loop parameter + extra variable)
        let condition = ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "max".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(100));
        variable_map.insert("max".to_string(), ValueId(200));

        let mut space = JoinValueSpace::new();
        let (env, bindings, loop_var_join_id) = ConditionEnvBuilder::build_for_break_condition_v2(
            &condition,
            "i",
            &variable_map,
            ValueId(100),
            &mut space,
        )
        .unwrap();

        // Should have loop parameter in env
        assert!(env.get("i").is_some());
        assert!(loop_var_join_id.0 >= 100);

        // Should have "max" in env with JoinIR-local ValueId
        assert!(env.get("max").is_some());

        // Should have one condition-only binding for "max"
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].name, "max");
        assert_eq!(bindings[0].host_value, ValueId(200));
        // Phase 201: Condition-only variables are allocated from Param region (100+), not Local region
        assert!(bindings[0].join_value.0 >= 100); // Should be in param region (100+)
    }

    #[test]
    fn test_build_for_break_condition_undefined_variable() {
        // Condition: i < undefined_var
        let condition = ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "undefined_var".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(100));
        // "undefined_var" is NOT in variable_map

        let mut space = JoinValueSpace::new();
        let result = ConditionEnvBuilder::build_for_break_condition_v2(
            &condition,
            "i",
            &variable_map,
            ValueId(100),
            &mut space,
        );

        // Should return error
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("undefined_var"));
    }

    /// Phase 201: Test that v2 API uses JoinValueSpace correctly
    #[test]
    fn test_build_for_break_condition_v2_uses_param_region() {
        use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;

        // Condition: i < max
        let condition = ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "max".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(100));
        variable_map.insert("max".to_string(), ValueId(200));

        let mut space = JoinValueSpace::new();
        let (env, bindings, loop_var_join_id) = ConditionEnvBuilder::build_for_break_condition_v2(
            &condition,
            "i",
            &variable_map,
            ValueId(100),
            &mut space,
        )
        .unwrap();

        // Phase 201: Loop param should be in Param region (100+)
        assert_eq!(loop_var_join_id, ValueId(100)); // First param allocation
        assert_eq!(env.get("i"), Some(ValueId(100)));

        // Phase 201: Condition variable should also be in Param region
        assert_eq!(env.get("max"), Some(ValueId(101))); // Second param allocation
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].join_value, ValueId(101));

        // Phase 201: Verify no collision with Local region (1000+)
        let local_id = space.alloc_local();
        assert_eq!(local_id, ValueId(1000)); // Locals start at 1000
        assert_ne!(local_id, loop_var_join_id);
        assert_ne!(local_id, bindings[0].join_value);
    }

    #[test]
    fn test_build_loop_param_only_v2() {
        use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;

        let mut space = JoinValueSpace::new();
        let (env, loop_var_join_id) =
            ConditionEnvBuilder::build_loop_param_only_v2("i", &mut space);

        // Phase 201: Should use Param region
        assert_eq!(loop_var_join_id, ValueId(100));
        assert_eq!(env.get("i"), Some(ValueId(100)));
    }
}
