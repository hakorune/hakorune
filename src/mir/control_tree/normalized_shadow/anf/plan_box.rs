//! Phase 145 P1: ANF Plan Box (AST pattern detection with whitelist)
//!
//! ## Responsibility
//!
//! Walk AST expression to detect impure subexpressions (Call/MethodCall) and build AnfPlan.
//! Does NOT perform transformation (that's execute_box's job).
//!
//! ## Contract
//!
//! - Returns `Ok(Some(plan))` if expression is in scope (pure or impure)
//! - Returns `Ok(None)` if expression is out-of-scope (unknown AST node type)
//! - Returns `Err(reason)` if expression is explicitly out-of-scope (e.g., nested impure)
//!
//! ## Phase Scope
//!
//! - **P0**: Detect Call/MethodCall presence (basic impure detection)
//! - **P1**: Add whitelist check (String.length() only), BinaryOp pattern detection
//! - **P2**: Add recursive compound expression detection

use super::contract::{AnfHoistTarget, AnfOutOfScopeReason, AnfParentKind, AnfPlan, HoistPosition};
use crate::ast::ASTNode;
use crate::mir::control_tree::normalized_shadow::common::expr_lowering_contract::KnownIntrinsic;
use crate::mir::control_tree::normalized_shadow::common::known_intrinsics::KnownIntrinsicRegistryBox;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Phase 145 P1: Whitelist of intrinsics allowed for ANF transformation
///
/// Initially: String.length() only (KnownIntrinsic::Length0).
/// P2+: Will expand to more intrinsics.
const P1_WHITELIST: &[KnownIntrinsic] = &[KnownIntrinsic::Length0];

/// Box-First: ANF plan builder (AST pattern detection)
pub struct AnfPlanBox;

impl AnfPlanBox {
    /// Phase 145 P1: Check if intrinsic is whitelisted for ANF transformation
    fn is_whitelisted(intrinsic: KnownIntrinsic) -> bool {
        P1_WHITELIST.contains(&intrinsic)
    }

    /// Phase 145 P1: Try to match MethodCall to a whitelisted intrinsic
    ///
    /// Returns Some(intrinsic) if the MethodCall matches a whitelisted known intrinsic.
    fn try_match_whitelisted_method_call(
        object: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
        _env: &BTreeMap<String, ValueId>,
    ) -> Option<KnownIntrinsic> {
        // Match using KnownIntrinsicRegistryBox
        let arity = arguments.len();
        let intrinsic = KnownIntrinsicRegistryBox::lookup(method, arity)?;

        // Check whitelist
        if !Self::is_whitelisted(intrinsic) {
            return None;
        }

        // Additional validation: object must be a pure expression (variable or literal)
        // P1: Only support simple receivers (not chained calls)
        match object {
            ASTNode::Variable { .. } | ASTNode::Literal { .. } => Some(intrinsic),
            _ => None, // Nested MethodCall (e.g., s.trim().length()) is P2+
        }
    }

    /// Phase 146 P1: Check if BinaryOperator is a comparison operator
    fn is_compare_operator(op: &crate::ast::BinaryOperator) -> bool {
        use crate::ast::BinaryOperator;
        matches!(
            op,
            BinaryOperator::Equal
                | BinaryOperator::NotEqual
                | BinaryOperator::Less
                | BinaryOperator::Greater
                | BinaryOperator::LessEqual
                | BinaryOperator::GreaterEqual
        )
    }

    /// Plan ANF transformation for an expression
    ///
    /// Walks AST to detect impure subexpressions (Call/MethodCall) and builds AnfPlan.
    ///
    /// ## Returns
    ///
    /// - `Ok(Some(plan))`: Expression is in scope (plan.requires_anf indicates if ANF needed)
    /// - `Ok(None)`: Expression is out-of-scope (unknown AST node type, route decline)
    /// - `Err(reason)`: Expression is explicitly out-of-scope (e.g., nested impure)
    ///
    /// ## Phase Scope
    ///
    /// - **P0**: Detect Call/MethodCall presence
    /// - **P1+**: Whitelist check, BinaryOp pattern detection
    pub fn plan_expr(
        ast: &ASTNode,
        _env: &BTreeMap<String, ValueId>, // P0: unused, P1+ for intrinsic detection
    ) -> Result<Option<AnfPlan>, AnfOutOfScopeReason> {
        // Baseline impure detection (Call/MethodCall presence)
        match ast {
            // Pure expressions (no ANF transformation needed)
            ASTNode::Variable { .. } => Ok(Some(AnfPlan::pure())),
            ASTNode::Literal { .. } => Ok(Some(AnfPlan::pure())),

            // Unary: Check operand recursively
            ASTNode::UnaryOp { operand, .. } => {
                match Self::plan_expr(operand, _env)? {
                    Some(operand_plan) => Ok(Some(AnfPlan {
                        requires_anf: operand_plan.requires_anf,
                        impure_count: operand_plan.impure_count,
                        hoist_targets: vec![],
                        parent_kind: AnfParentKind::UnaryOp,
                    })),
                    None => Ok(None), // Operand out-of-scope → propagate
                }
            }

            // Binary: Check left and right recursively
            // Phase 146 P1: Handle both arithmetic and comparison operators
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                // Phase 145 P1: Detect whitelisted MethodCall in operands
                let mut hoist_targets = vec![];

                // Check left operand for whitelisted MethodCall
                if let ASTNode::MethodCall {
                    object,
                    method,
                    arguments,
                    ..
                } = left.as_ref()
                {
                    if let Some(intrinsic) =
                        Self::try_match_whitelisted_method_call(object, method, arguments, _env)
                    {
                        hoist_targets.push(AnfHoistTarget {
                            intrinsic,
                            ast_node: left.as_ref().clone(),
                            position: HoistPosition::Left,
                        });
                    }
                }

                // Check right operand for whitelisted MethodCall
                if let ASTNode::MethodCall {
                    object,
                    method,
                    arguments,
                    ..
                } = right.as_ref()
                {
                    if let Some(intrinsic) =
                        Self::try_match_whitelisted_method_call(object, method, arguments, _env)
                    {
                        hoist_targets.push(AnfHoistTarget {
                            intrinsic,
                            ast_node: right.as_ref().clone(),
                            position: HoistPosition::Right,
                        });
                    }
                }

                // Phase 146 P1: Determine parent kind (Compare vs BinaryOp)
                let parent_kind = if Self::is_compare_operator(operator) {
                    AnfParentKind::Compare
                } else {
                    AnfParentKind::BinaryOp
                };

                // If we found whitelisted MethodCalls, return a plan with hoist targets
                if !hoist_targets.is_empty() {
                    return Ok(Some(AnfPlan::with_hoists(hoist_targets, parent_kind)));
                }

                // Recursively check operands for pure/impure.
                let left_plan = match Self::plan_expr(left, _env)? {
                    Some(p) => p,
                    None => return Ok(None), // Left out-of-scope → propagate
                };
                let right_plan = match Self::plan_expr(right, _env)? {
                    Some(p) => p,
                    None => return Ok(None), // Right out-of-scope → propagate
                };

                // Combine: ANF needed if either operand requires it
                let combined_impure_count = left_plan.impure_count + right_plan.impure_count;
                let requires_anf = left_plan.requires_anf || right_plan.requires_anf;

                Ok(Some(AnfPlan {
                    requires_anf,
                    impure_count: combined_impure_count,
                    hoist_targets: vec![],
                    parent_kind,
                }))
            }

            // Impure expressions (ANF transformation candidates)
            ASTNode::FunctionCall { .. } | ASTNode::Call { .. } => {
                // Direct Call remains out-of-scope for the current ANF route.
                Err(AnfOutOfScopeReason::ContainsCall)
            }

            ASTNode::MethodCall { .. } => {
                // Standalone MethodCall remains out-of-scope unless a parent plan hoists it.
                Err(AnfOutOfScopeReason::ContainsMethodCall)
            }

            // Out-of-scope (unknown AST node types)
            _ => {
                // Unknown expression type -> route decline.
                Ok(None)
            }
        }
    }

    /// Check if expression is pure (no impure subexpressions)
    ///
    /// Helper function for quick pure/impure discrimination.
    ///
    /// ## Returns
    ///
    /// - `true`: Expression is pure (no Call/MethodCall)
    /// - `false`: Expression contains impure subexpressions
    ///
    /// ## Phase Scope
    ///
    /// - **P0**: Basic Call/MethodCall detection
    /// - **P1+**: Consider whitelist (e.g., String.length() may be treated as pure)
    pub fn is_pure(ast: &ASTNode, env: &BTreeMap<String, ValueId>) -> bool {
        match Self::plan_expr(ast, env) {
            Ok(Some(plan)) => !plan.requires_anf,
            Ok(None) => true, // Unknown -> no impure evidence for this query
            Err(_) => false,  // Contains Call/MethodCall → impure
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
    use std::collections::BTreeMap;

    fn span() -> crate::ast::Span {
        crate::ast::Span::unknown()
    }

    #[test]
    fn test_plan_pure_variable() {
        let ast = ASTNode::Variable {
            name: "x".to_string(),
            span: span(),
        };
        let env = BTreeMap::new();
        let plan = AnfPlanBox::plan_expr(&ast, &env).unwrap().unwrap();
        assert!(!plan.requires_anf);
        assert_eq!(plan.impure_count, 0);
    }

    #[test]
    fn test_plan_pure_literal() {
        let ast = ASTNode::Literal {
            value: LiteralValue::Integer(42),
            span: span(),
        };
        let env = BTreeMap::new();
        let plan = AnfPlanBox::plan_expr(&ast, &env).unwrap().unwrap();
        assert!(!plan.requires_anf);
        assert_eq!(plan.impure_count, 0);
    }

    #[test]
    fn test_plan_pure_binop() {
        // x + 2 (pure)
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: span(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(2),
                span: span(),
            }),
            span: span(),
        };
        let env = BTreeMap::new();
        let plan = AnfPlanBox::plan_expr(&ast, &env).unwrap().unwrap();
        assert!(!plan.requires_anf);
        assert_eq!(plan.impure_count, 0);
    }

    #[test]
    fn test_plan_call_out_of_scope() {
        // f() → P0 out-of-scope
        let ast = ASTNode::FunctionCall {
            name: "f".to_string(),
            arguments: vec![],
            span: span(),
        };
        let env = BTreeMap::new();
        let result = AnfPlanBox::plan_expr(&ast, &env);
        assert!(matches!(result, Err(AnfOutOfScopeReason::ContainsCall)));
    }

    #[test]
    fn test_plan_method_call_out_of_scope() {
        // obj.method() → P0 out-of-scope
        let ast = ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "obj".to_string(),
                span: span(),
            }),
            method: "method".to_string(),
            arguments: vec![],
            span: span(),
        };
        let env = BTreeMap::new();
        let result = AnfPlanBox::plan_expr(&ast, &env);
        assert!(matches!(
            result,
            Err(AnfOutOfScopeReason::ContainsMethodCall)
        ));
    }

    #[test]
    fn test_is_pure_variable() {
        let ast = ASTNode::Variable {
            name: "x".to_string(),
            span: span(),
        };
        let env = BTreeMap::new();
        assert!(AnfPlanBox::is_pure(&ast, &env));
    }

    #[test]
    fn test_is_pure_call_false() {
        let ast = ASTNode::FunctionCall {
            name: "f".to_string(),
            arguments: vec![],
            span: span(),
        };
        let env = BTreeMap::new();
        assert!(!AnfPlanBox::is_pure(&ast, &env));
    }

    // P0: 4 plan tests (pure_variable, pure_literal, pure_binop, call_out_of_scope)

    // ========== Phase 145 P1 Tests ==========

    #[test]
    fn test_p1_whitelist_length0() {
        // String.length() is whitelisted in P1
        use crate::mir::control_tree::normalized_shadow::common::expr_lowering_contract::KnownIntrinsic;
        assert!(AnfPlanBox::is_whitelisted(KnownIntrinsic::Length0));
    }

    #[test]
    fn test_p1_binop_with_whitelisted_method_call_right() {
        // x + s.length() → hoist target detected
        let mut env = BTreeMap::new();
        env.insert("x".to_string(), ValueId(100));
        env.insert("s".to_string(), ValueId(200));

        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: span(),
            }),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "s".to_string(),
                    span: span(),
                }),
                method: "length".to_string(),
                arguments: vec![],
                span: span(),
            }),
            span: span(),
        };

        let plan = AnfPlanBox::plan_expr(&ast, &env).unwrap().unwrap();
        assert!(plan.requires_anf, "Should require ANF transformation");
        assert_eq!(plan.hoist_targets.len(), 1, "Should have 1 hoist target");
        assert_eq!(plan.impure_count, 1);
        assert_eq!(plan.parent_kind, AnfParentKind::BinaryOp);
        assert_eq!(plan.hoist_targets[0].position, HoistPosition::Right);
    }

    #[test]
    fn test_p1_binop_with_whitelisted_method_call_left() {
        // s.length() + x → hoist target detected (left position)
        let mut env = BTreeMap::new();
        env.insert("x".to_string(), ValueId(100));
        env.insert("s".to_string(), ValueId(200));

        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "s".to_string(),
                    span: span(),
                }),
                method: "length".to_string(),
                arguments: vec![],
                span: span(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: span(),
            }),
            span: span(),
        };

        let plan = AnfPlanBox::plan_expr(&ast, &env).unwrap().unwrap();
        assert!(plan.requires_anf);
        assert_eq!(plan.hoist_targets.len(), 1);
        assert_eq!(plan.hoist_targets[0].position, HoistPosition::Left);
    }

    #[test]
    fn test_p1_binop_with_pure_operands() {
        // x + y (pure) → no hoist targets
        let mut env = BTreeMap::new();
        env.insert("x".to_string(), ValueId(100));
        env.insert("y".to_string(), ValueId(200));

        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: span(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "y".to_string(),
                span: span(),
            }),
            span: span(),
        };

        let plan = AnfPlanBox::plan_expr(&ast, &env).unwrap().unwrap();
        assert!(!plan.requires_anf);
        assert_eq!(plan.hoist_targets.len(), 0);
    }

    #[test]
    fn test_p1_method_call_not_whitelisted() {
        // s.unknown() is not whitelisted → no hoist (falls back to ContainsMethodCall)
        let mut env = BTreeMap::new();
        env.insert("s".to_string(), ValueId(200));

        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: "s".to_string(),
                span: span(),
            }),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "s".to_string(),
                    span: span(),
                }),
                method: "unknown".to_string(),
                arguments: vec![],
                span: span(),
            }),
            span: span(),
        };

        // Should fall back to recursive plan which detects MethodCall as out-of-scope
        let result = AnfPlanBox::plan_expr(&ast, &env);
        // MethodCall not whitelisted → falls back to recursive check → Err(ContainsMethodCall)
        assert!(matches!(
            result,
            Err(AnfOutOfScopeReason::ContainsMethodCall)
        ));
    }

    #[test]
    fn test_p1_binop_with_two_whitelisted_method_calls() {
        // s1.length() + s2.length() → 2 hoist targets (P1 supports this!)
        let mut env = BTreeMap::new();
        env.insert("s1".to_string(), ValueId(100));
        env.insert("s2".to_string(), ValueId(200));

        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "s1".to_string(),
                    span: span(),
                }),
                method: "length".to_string(),
                arguments: vec![],
                span: span(),
            }),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "s2".to_string(),
                    span: span(),
                }),
                method: "length".to_string(),
                arguments: vec![],
                span: span(),
            }),
            span: span(),
        };

        let plan = AnfPlanBox::plan_expr(&ast, &env).unwrap().unwrap();
        assert!(plan.requires_anf);
        assert_eq!(plan.hoist_targets.len(), 2, "Should have 2 hoist targets");
        assert_eq!(plan.impure_count, 2);
    }

    // P1: 6 new tests (whitelist, binop_right, binop_left, pure, not_whitelisted, two_method_calls)
}
