//! Phase 244: Unified Condition Lowering Interface
//!
//! This module provides a trait-based abstraction for condition lowering,
//! allowing different implementations (ExprLowerer, legacy lowerers) to
//! be used interchangeably.
//!
//! ## Design Philosophy
//!
//! **Box-First**: ConditionLoweringBox is a "box" trait that encapsulates
//! all condition lowering logic with a single, unified interface.
//!
//! **Single Responsibility**: Implementations ONLY perform AST → ValueId lowering.
//! They do NOT manage scopes, extract variables, or handle PHI generation.
//!
//! **Fail-Safe**: Implementations return explicit errors for unsupported route shapes,
//! allowing callers to fall back to alternative paths.

use super::scope_manager::ScopeManager;
use crate::ast::ASTNode;
use crate::mir::ValueId;

/// Phase 244: Context for condition lowering
///
/// This struct encapsulates all the necessary context for lowering a condition
/// expression to JoinIR, including loop variable information and scope access.
///
/// # Fields
///
/// * `loop_var_name` - Name of the loop variable (e.g., "i")
/// * `loop_var_id` - ValueId of the loop variable in JoinIR space
/// * `scope` - Reference to ScopeManager for variable lookup
/// * `alloc_value` - ValueId allocator function
/// * `current_static_box_name` - Phase 252: Name of the static box being lowered (for this.method)
///
/// # Example
///
/// ```ignore
/// let context = ConditionContext {
///     loop_var_name: "i".to_string(),
///     loop_var_id: ValueId(1),
///     scope: &scope_manager,
///     alloc_value: &mut alloc_fn,
///     current_static_box_name: Some("StringUtils".to_string()), // Phase 252
/// };
///
/// let value_id = lowerer.lower_condition(&ast, &context)?;
/// ```
pub struct ConditionContext<'a, S: ScopeManager> {
    /// Name of the loop variable (e.g., "i", "pos")
    pub loop_var_name: String,

    /// ValueId of the loop variable in JoinIR space
    pub loop_var_id: ValueId,

    /// Scope manager for variable resolution
    pub scope: &'a S,

    /// ValueId allocator function
    pub alloc_value: &'a mut dyn FnMut() -> ValueId,

    /// Phase 252: Name of the static box being lowered (for this.method(...) support)
    ///
    /// When lowering a static box method (e.g., `StringUtils.trim_end/1`),
    /// this field contains the box name ("StringUtils"). This allows
    /// `this.is_whitespace(...)` to be resolved to `StringUtils.is_whitespace(...)`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Function: StringUtils.trim_end/1
    /// // Condition: not this.is_whitespace(s.substring(i, i + 1))
    /// // current_static_box_name: Some("StringUtils")
    /// // → Resolves to: StringUtils.is_whitespace(...)
    /// ```
    ///
    /// Set to `None` for non-static-box contexts (e.g., `Main.main()` loops).
    pub current_static_box_name: Option<String>,
}

/// Phase 244: Unified condition lowering interface
///
/// This trait provides a common interface for all condition lowering implementations.
/// It allows loop_break / if_phi_join / loop_continue_only to use any lowering
/// strategy (ExprLowerer, legacy, etc.)
/// without coupling to specific implementation details.
///
/// # Design Principles
///
/// 1. **Single Method**: `lower_condition()` is the only required method
/// 2. **Context-Based**: All necessary information passed via ConditionContext
/// 3. **Fail-Fast**: Errors returned immediately (no silent fallbacks)
/// 4. **Stateless**: Implementations should be reusable across multiple calls
///
/// # Example Implementation
///
/// ```ignore
/// impl<S: ScopeManager> ConditionLoweringBox<S> for ExprLowerer<'_, S> {
///     fn lower_condition(
///         &mut self,
///         condition: &ASTNode,
///         context: &ConditionContext<S>,
///     ) -> Result<ValueId, String> {
///         // Delegate to existing ExprLowerer::lower() method
///         self.lower(condition)
///     }
///
///     fn supports(&self, condition: &ASTNode) -> bool {
///         Self::is_supported_condition(condition)
///     }
/// }
/// ```
pub trait ConditionLoweringBox<S: ScopeManager> {
    /// Lower condition AST to ValueId
    ///
    /// This method translates an AST condition expression (e.g., `i < 10`,
    /// `digit_pos < 0`) into a JoinIR ValueId representing the boolean result.
    ///
    /// # Arguments
    ///
    /// * `condition` - AST node representing the boolean condition
    /// * `context` - Lowering context (loop var, scope, allocator)
    ///
    /// # Returns
    ///
    /// * `Ok(ValueId)` - Boolean result ValueId
    /// * `Err(String)` - Lowering error (unsupported route shape, variable not found, etc.)
    ///
    /// # Fail-Fast Principle
    ///
    /// Implementations MUST return `Err` immediately for unsupported route shapes.
    /// Callers can then decide whether to fall back to alternative lowering paths.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // LoopBreak: break condition lowering
    /// let cond_value = lowerer.lower_condition(&break_cond_ast, &context)?;
    ///
    /// // Use cond_value in conditional Jump
    /// let break_jump = JoinInst::Jump {
    ///     target: k_exit,
    ///     args: vec![loop_var_id],
    ///     condition: Some(cond_value),
    /// };
    /// ```
    fn lower_condition(
        &mut self,
        condition: &ASTNode,
        context: &mut ConditionContext<S>,
    ) -> Result<ValueId, String>;

    /// Check if this lowerer supports the given condition pattern
    ///
    /// This method allows callers to check support BEFORE attempting lowering,
    /// enabling early fallback to alternative strategies.
    ///
    /// # Arguments
    ///
    /// * `condition` - AST node to check
    ///
    /// # Returns
    ///
    /// * `true` - This lowerer can handle this pattern
    /// * `false` - This lowerer cannot handle this pattern (caller should use alternative)
    ///
    /// # Example
    ///
    /// ```ignore
    /// if lowerer.supports(&break_cond_ast) {
    ///     let value = lowerer.lower_condition(&break_cond_ast, &context)?;
    /// } else {
    ///     // Fall back to legacy lowering
    ///     let value = legacy_lower_condition(&break_cond_ast, env)?;
    /// }
    /// ```
    fn supports(&self, condition: &ASTNode) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::MirBuilder;
    use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
    use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
    use crate::mir::join_ir::lowering::expr_lowerer::{ExprContext, ExprLowerer};
    use crate::mir::join_ir::lowering::scope_manager::LoopBreakScopeManager;

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn lit_i(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn bin(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
            span: span(),
        }
    }

    #[test]
    fn test_condition_lowering_box_trait_exists() {
        // This test verifies that the ConditionLoweringBox trait can be used
        // with ExprLowerer (implementation added in Step 2).

        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![],
            trim_helper: None,
            promoted_body_locals: vec![],
        };

        let scope = LoopBreakScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: None,
            captured_env: None,
            carrier_info: &carrier_info,
        };

        let mut builder = MirBuilder::new();
        let mut alloc_counter = 1000u32;
        let mut alloc_fn = || {
            let id = ValueId(alloc_counter);
            alloc_counter += 1;
            id
        };

        let mut context = ConditionContext {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            scope: &scope,
            alloc_value: &mut alloc_fn,
            current_static_box_name: None, // Phase 252: No static box in test
        };

        // AST: i < 10
        let ast = bin(BinaryOperator::Less, var("i"), lit_i(10));

        // ExprLowerer implements ConditionLoweringBox (Step 2)
        let mut expr_lowerer = ExprLowerer::new(&scope, ExprContext::Condition, &mut builder);

        // Check support
        assert!(
            expr_lowerer.supports(&ast),
            "ExprLowerer should support i < 10"
        );

        // Lower condition via ConditionLoweringBox trait (Step 2 implemented)
        let result = expr_lowerer.lower_condition(&ast, &mut context);
        assert!(result.is_ok(), "i < 10 should lower successfully via trait");
    }

    #[test]
    fn test_condition_context_structure() {
        // Verify ConditionContext can be constructed and fields are accessible

        let mut condition_env = ConditionEnv::new();
        condition_env.insert("i".to_string(), ValueId(100));

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            carriers: vec![],
            trim_helper: None,
            promoted_body_locals: vec![],
        };

        let scope = LoopBreakScopeManager {
            condition_env: &condition_env,
            loop_body_local_env: None,
            captured_env: None,
            carrier_info: &carrier_info,
        };

        let mut alloc_counter = 1000u32;
        let mut alloc_fn = || {
            let id = ValueId(alloc_counter);
            alloc_counter += 1;
            id
        };

        let context = ConditionContext {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(1),
            scope: &scope,
            alloc_value: &mut alloc_fn,
            current_static_box_name: None, // Phase 252: No static box in test
        };

        assert_eq!(context.loop_var_name, "i");
        assert_eq!(context.loop_var_id, ValueId(1));

        // Verify allocator works
        let vid = (context.alloc_value)();
        assert_eq!(vid, ValueId(1000));
    }
}
