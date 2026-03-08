//! Phase 169: JoinIR Condition Lowering Orchestrator
//!
//! This module provides the high-level API for lowering AST conditions to JoinIR.
//! It re-exports functionality from specialized modules:
//!
//! - `condition_env`: Variable name → ValueId mapping
//! - `condition_lowerer`: AST → JoinIR lowering logic
//! - `condition_var_extractor`: Variable extraction from AST
//!
//! ## Design Philosophy
//!
//! **Orchestration Layer**: This module provides a unified API by composing
//! functionality from specialized modules. Each module has a single responsibility:
//!
//! - `condition_env.rs`: Environment management (80 lines)
//! - `condition_lowerer.rs`: Core lowering logic (330 lines)
//! - `condition_var_extractor.rs`: Variable extraction (90 lines)
//! - `condition_to_joinir.rs` (this file): API orchestration (100 lines)
//!
//! **Total: 600 lines → 500 lines (17% reduction)**
//!
//! ## Separation of Concerns
//!
//! - `condition_to_joinir`: AST → JoinIR (for loop lowerers)
//! - MIR-side condition lowering stays on the regular builder path
//!
//! Loop lowerers work in JoinIR space as a pure lowering step, while regular control flow
//! remains on the MIR builder path.

// Re-export public API from specialized modules
pub use super::condition_env::{ConditionBinding, ConditionEnv};
pub use super::condition_lowerer::{
    lower_condition_to_joinir, lower_condition_to_joinir_no_body_locals, lower_value_expression,
};
pub use super::condition_var_extractor::extract_condition_variables;

// Re-export JoinIR types for convenience
pub use crate::mir::join_ir::JoinInst;
pub use crate::mir::ValueId;

/// Module documentation test
///
/// This test verifies that the public API is accessible and works as expected.
#[cfg(test)]
mod api_tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    #[test]
    fn test_api_condition_env() {
        let mut env = ConditionEnv::new();
        env.insert("i".to_string(), ValueId(0));
        assert_eq!(env.get("i"), Some(ValueId(0)));
    }

    #[test]
    fn test_api_condition_binding() {
        let binding = ConditionBinding::new("start".to_string(), ValueId(33), ValueId(1));
        assert_eq!(binding.name, "start");
        assert_eq!(binding.host_value, ValueId(33));
        assert_eq!(binding.join_value, ValueId(1));
    }

    #[test]
    fn test_api_lower_condition() {
        let mut env = ConditionEnv::new();
        env.insert("i".to_string(), ValueId(0));

        let mut value_counter = 1u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };

        // AST: i < 10
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
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

        let result = lower_condition_to_joinir_no_body_locals(&ast, &mut alloc_value, &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_api_extract_variables() {
        // AST: start < end
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "start".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "end".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let vars = extract_condition_variables(&ast, &[]);
        assert_eq!(vars, vec!["end", "start"]); // Sorted
    }

    #[test]
    fn test_api_integration() {
        // Full integration: extract vars, create env, lower condition
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "end".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        // Step 1: Extract variables (excluding loop param 'i')
        let condition_vars = extract_condition_variables(&ast, &["i".to_string()]);
        assert_eq!(condition_vars, vec!["end"]);

        // Step 2: Create environment
        let mut env = ConditionEnv::new();
        env.insert("i".to_string(), ValueId(0)); // Loop parameter
        env.insert("end".to_string(), ValueId(1)); // Condition-only variable

        // Step 3: Lower condition
        let mut value_counter = 2u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };

        let result = lower_condition_to_joinir_no_body_locals(&ast, &mut alloc_value, &env);
        assert!(result.is_ok());

        let (_cond_value, instructions) = result.unwrap();
        assert_eq!(instructions.len(), 1); // Single Compare instruction
    }
}
