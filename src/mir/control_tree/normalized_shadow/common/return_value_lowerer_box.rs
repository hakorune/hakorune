//! ReturnValueLowererBox: Return syntax lowering (Phase 140 P0)
//!
//! ## Responsibility
//!
//! Normalize return value syntax (`return` vs `return <expr>`) for Normalized shadow paths.
//!
//! Expression lowering is delegated to `NormalizedExprLowererBox` (SSOT).
//!
//! ## Fallback
//!
//! Out-of-scope patterns return `Ok(None)` for legacy routing
//!
//! ## Usage
//!
//! - Phase 138 P0: loop_true_break_once.rs
//! - Phase 139 P0: post_if_post_k.rs

use super::expr_lowerer_box::NormalizedExprLowererBox;
use super::expr_lowering_contract::{ExprLoweringScope, ImpurePolicy};
use crate::mir::control_tree::step_tree::AstNodeHandle;
use crate::mir::join_ir::JoinInst;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Box-First: Return value lowering for Normalized shadow
pub struct ReturnValueLowererBox;

impl ReturnValueLowererBox {
    /// Lower return value to ValueId
    ///
    /// Returns:
    /// - Ok(Some(vid)): Successfully lowered to ValueId
    /// - Ok(None): Out of scope (fallback to legacy routing)
    /// - Err(_): Internal error (should not happen for valid AST)
    ///
    /// Note: Does NOT return Err for unsupported patterns - returns Ok(None) instead
    pub fn lower_to_value_id(
        value_ast: &Option<AstNodeHandle>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
        env: &BTreeMap<String, ValueId>,
    ) -> Result<Option<ValueId>, String> {
        match value_ast {
            None => {
                // void return
                Ok(Some(ValueId(0))) // Dummy - caller handles void separately
            }
            Some(ast_handle) => {
                // Phase 141 P1: allow a small allowlist of known intrinsic method calls.
                NormalizedExprLowererBox::lower_expr_with_scope(
                    ExprLoweringScope::WithImpure(ImpurePolicy::KnownIntrinsicOnly),
                    ast_handle.0.as_ref(),
                    env,
                    body,
                    next_value_id,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::control_tree::step_tree::AstNodeHandle;
    use crate::mir::join_ir::JoinInst;

    fn make_span() -> Span {
        Span::unknown()
    }

    #[test]
    fn test_void_return_dummy_value_id() {
        let mut body = vec![];
        let mut next_value_id = 100;

        let result = ReturnValueLowererBox::lower_to_value_id(
            &None,
            &mut body,
            &mut next_value_id,
            &BTreeMap::new(),
        )
        .unwrap();

        assert_eq!(result, Some(ValueId(0)));
        assert!(body.is_empty());
        assert_eq!(next_value_id, 100);
    }

    #[test]
    fn test_delegates_integer_literal() {
        let mut body = vec![];
        let mut next_value_id = 100;
        let env = BTreeMap::new();

        let int_ast = AstNodeHandle(Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(7),
            span: make_span(),
        }));

        let result = ReturnValueLowererBox::lower_to_value_id(
            &Some(int_ast),
            &mut body,
            &mut next_value_id,
            &env,
        )
        .unwrap();

        assert_eq!(result, Some(ValueId(100)));
        assert_eq!(body.len(), 1); // Const instruction emitted
        assert_eq!(next_value_id, 101);
    }

    #[test]
    fn test_delegates_add_var_plus_int() {
        let mut body = vec![];
        let mut next_value_id = 100;
        let mut env = BTreeMap::new();
        env.insert("x".to_string(), ValueId(1));

        let add_ast = AstNodeHandle(Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: make_span(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(2),
                span: make_span(),
            }),
            span: make_span(),
        }));

        let result = ReturnValueLowererBox::lower_to_value_id(
            &Some(add_ast),
            &mut body,
            &mut next_value_id,
            &env,
        )
        .unwrap();

        assert_eq!(result, Some(ValueId(101))); // BinOp result
        assert_eq!(body.len(), 2); // Const(2) + BinOp
        assert_eq!(next_value_id, 102);
    }

    #[test]
    fn test_delegates_known_intrinsic_method_call_length0() {
        let mut body = vec![];
        let mut next_value_id = 100;
        let mut env = BTreeMap::new();
        env.insert("s".to_string(), ValueId(1));

        let length_ast = AstNodeHandle(Box::new(ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "s".to_string(),
                span: make_span(),
            }),
            method: "length".to_string(),
            arguments: vec![],
            span: make_span(),
        }));

        let result = ReturnValueLowererBox::lower_to_value_id(
            &Some(length_ast),
            &mut body,
            &mut next_value_id,
            &env,
        )
        .unwrap();

        assert_eq!(result, Some(ValueId(100)));
        assert_eq!(next_value_id, 101);
        assert!(matches!(
            body.as_slice(),
            [JoinInst::MethodCall { method, args, .. }]
                if method == "length" && args.is_empty()
        ));
    }
}
