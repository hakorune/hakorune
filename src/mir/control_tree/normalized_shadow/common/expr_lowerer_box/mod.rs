//! NormalizedExprLowererBox: Pure expression lowering SSOT (Phase 140 P0)
//!
//! ## Responsibility
//!
//! Lower a *pure* AST expression into JoinIR `ValueId` + emitted `JoinInst`s.
//!
//! ## Scope (Phase 140 P0)
//!
//! Supported (pure only):
//! - Variable: env lookup → ValueId
//! - Literals: Integer / Bool
//! - Unary: `-` (Neg), `not` (Not)
//! - Binary arith (int-only): `+ - * /`
//! - Compare (int-only): `== != < <= > >=`
//!
//! Out of scope (returns `Ok(None)`):
//! - Call/MethodCall/FromCall/Array/Map/FieldAccess/Index/New/This/Me/…
//! - Literals other than Integer/Bool
//! - Operators outside the scope above
//!
//! ## Contract
//!
//! - Out-of-scope returns `Ok(None)` (caller routes to non-normalized lowering).
//! - `Err(_)` is reserved for internal invariants only (should be rare).

mod binary;
mod intrinsics;
mod lowering;
mod scope;

#[cfg(test)]
mod tests;

use super::expr_lowering_contract::ExprLoweringScope;
use crate::ast::ASTNode;
use crate::mir::join_ir::JoinInst;
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;
use std::collections::BTreeMap;

/// Box-First: Pure expression lowering for Normalized shadow paths
pub struct NormalizedExprLowererBox;

impl NormalizedExprLowererBox {
    pub fn lower_expr(
        ast: &ASTNode,
        env: &BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        Self::lower_expr_with_scope(ExprLoweringScope::PureOnly, ast, env, body, next_value_id)
    }

    pub fn lower_expr_with_scope(
        scope: ExprLoweringScope,
        ast: &ASTNode,
        env: &BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        // Phase 146 P1: ANF routing (dev-only, scope-aware)
        if crate::config::env::anf_dev_enabled() {
            // P1: Allow ANF for PureOnly if HAKO_ANF_ALLOW_PURE=1
            let should_try_anf = match scope {
                ExprLoweringScope::WithImpure(_) => true,
                ExprLoweringScope::PureOnly => crate::config::env::anf_allow_pure_enabled(),
            };

            if should_try_anf {
                use super::super::anf::{AnfExecuteBox, AnfPlanBox};
                match AnfPlanBox::plan_expr(ast, env) {
                    Ok(Some(plan)) => {
                        match AnfExecuteBox::try_execute(
                            &plan,
                            ast,
                            &mut env.clone(),
                            body,
                            next_value_id,
                        )? {
                            Some(vid) => return Ok(Some(vid)),
                            None => {
                                if crate::config::env::joinir_dev_enabled() {
                                    get_global_ring0().log.debug(
                                        "[phase145/debug] ANF plan found but execute returned None (P0 stub)",
                                    );
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        // Out-of-scope for ANF, continue with normalized lowering
                    }
                    Err(_reason) => {
                        // Explicitly out-of-scope (ContainsCall/ContainsMethodCall), continue
                    }
                }
            }
        }

        if Self::out_of_scope_reason(scope, ast, env).is_some() {
            return Ok(None);
        }

        match ast {
            ASTNode::Variable { name, .. } => Ok(env.get(name).copied()),
            ASTNode::Literal { value, .. } => Self::lower_literal(value, body, next_value_id),
            ASTNode::UnaryOp {
                operator, operand, ..
            } => Self::lower_unary(operator, operand, env, body, next_value_id),
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => Self::lower_binary(operator, left, right, env, body, next_value_id),
            ASTNode::MethodCall {
                object,
                method,
                arguments,
                ..
            } => match scope {
                ExprLoweringScope::PureOnly => Ok(None),
                ExprLoweringScope::WithImpure(policy) => {
                    let Some(intrinsic) = Self::match_known_intrinsic_method_call(
                        policy, object, method, arguments, env,
                    ) else {
                        return Ok(None);
                    };
                    Self::lower_known_intrinsic_method_call(
                        intrinsic,
                        object,
                        body,
                        next_value_id,
                        env,
                    )
                }
            },
            _ => Ok(None),
        }
    }
}
