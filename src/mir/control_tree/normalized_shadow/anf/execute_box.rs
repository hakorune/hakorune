//! Phase 145 P1: ANF Execute Box (BinaryOp + MethodCall hoist)
//!
//! ## Responsibility
//!
//! Execute ANF transformation for expressions that require it (per AnfPlan).
//! P1: Hoist whitelisted MethodCalls (String.length()) from BinaryOp operands.
//!
//! ## Contract
//!
//! - Returns `Ok(Some(vid))` if ANF transformation succeeded (P1+)
//! - Returns `Ok(None)` if transformation was not attempted or route-declined
//! - Returns `Err(msg)` only in strict mode for internal errors
//!
//! ## Phase Scope
//!
//! - **P1**: Implement String.length() hoist for BinaryOp (whitelist 1 intrinsic)
//! - **P2**: Implement recursive compound expression ANF

use super::contract::{AnfParentKind, AnfPlan};
use crate::ast::ASTNode;
use crate::mir::join_ir::{CompareOp, JoinInst, MirLikeInst};
use crate::mir::types::MirType;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Box-First: ANF transformation executor
pub struct AnfExecuteBox;

impl AnfExecuteBox {
    /// Try to execute ANF transformation for an expression
    ///
    /// ## Arguments
    ///
    /// - `plan`: ANF plan built by AnfPlanBox (indicates what transformation is needed)
    /// - `ast`: AST expression to transform
    /// - `env`: Current environment (variable → ValueId mapping)
    /// - `body`: JoinInst vector to emit transformed instructions
    /// - `next_value_id`: Mutable counter for allocating new ValueIds
    ///
    /// ## Returns
    ///
    /// - `Ok(Some(vid))`: ANF transformation succeeded, result is ValueId (P1+)
    /// - `Ok(None)`: Transformation not attempted or out-of-scope
    /// - `Err(msg)`: Internal error (strict mode only, P1+)
    ///
    /// ## Phase Scope
    ///
    /// - **Out-of-scope**: returns `Ok(None)` as route decline
    /// - **P1**: Implement String.length() hoist for BinaryOp
    /// - **P2**: Implement recursive ANF for compound expressions
    pub fn try_execute(
        plan: &AnfPlan,
        ast: &ASTNode,
        env: &mut BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        // DEBUG: Log attempt if HAKO_ANF_DEV=1
        if crate::config::env::anf_dev_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[phase145/debug] ANF execute called: requires_anf={}, targets={}",
                plan.requires_anf,
                plan.hoist_targets.len()
            ));
        }

        // P1: No hoist targets → out of scope for the ANF route
        if plan.hoist_targets.is_empty() {
            return Ok(None);
        }

        // Phase 146 P1: BinaryOp and Compare supported
        match plan.parent_kind {
            AnfParentKind::BinaryOp => {
                Self::execute_binary_op_hoist(plan, ast, env, body, next_value_id)
            }
            AnfParentKind::Compare => {
                // Phase 146 P1: Route Compare to execute_compare_hoist
                Self::execute_compare_hoist(plan, ast, env, body, next_value_id)
            }
            _ => Ok(None), // P2+: UnaryOp/MethodCall/Call
        }
    }

    /// Phase 145 P2: Execute ANF transformation for BinaryOp with recursive normalization
    ///
    /// Pattern: `x + s.length()` → `t = s.length(); result = x + t`
    /// Pattern: `s1.length() + s2.length()` → `t1 = s1.length(); t2 = s2.length(); result = t1 + t2`
    /// Pattern: `(x + s.length()) + z` → `t1 = s.length(); t2 = x + t1; result = t2 + z`
    ///
    /// This function recursively normalizes left and right operands (depth-first, left-to-right)
    /// and then generates a pure BinaryOp instruction.
    fn execute_binary_op_hoist(
        _plan: &AnfPlan,
        ast: &ASTNode,
        env: &mut BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        let ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } = ast
        else {
            return Err("ANF execute_binary_op_hoist: expected BinaryOp AST node".to_string());
        };

        // P2: Use recursive normalization instead of single-level hoist
        Self::execute_binary_op_recursive(left, right, operator, env, body, next_value_id)
    }

    /// Phase 145 P2: Recursively normalize BinaryOp operands (depth-first, left-to-right)
    ///
    /// This is the core recursive ANF transformation algorithm:
    /// 1. Normalize LEFT operand recursively (depth-first)
    /// 2. Normalize RIGHT operand recursively (left-to-right)
    /// 3. Generate pure BinaryOp instruction
    ///
    /// # Example
    ///
    /// Input: `x + s.length()`
    /// - Step 1: Normalize `x` → ValueId(1)
    /// - Step 2: Normalize `s.length()` → ValueId(2) (emits MethodCall)
    /// - Step 3: Emit BinaryOp(ValueId(1), +, ValueId(2)) → ValueId(3)
    ///
    /// Input: `s1.length() + s2.length()`
    /// - Step 1: Normalize `s1.length()` → ValueId(1) (emits MethodCall)
    /// - Step 2: Normalize `s2.length()` → ValueId(2) (emits MethodCall)
    /// - Step 3: Emit BinaryOp(ValueId(1), +, ValueId(2)) → ValueId(3)
    fn execute_binary_op_recursive(
        left: &ASTNode,
        right: &ASTNode,
        operator: &crate::ast::BinaryOperator,
        env: &mut BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        // Step 1: Recursively normalize LEFT (depth-first)
        let lhs_vid = Self::normalize_and_lower(left, env, body, next_value_id)?;

        // Step 2: Recursively normalize RIGHT (left-to-right)
        let rhs_vid = Self::normalize_and_lower(right, env, body, next_value_id)?;

        // Step 3: Generate pure BinOp instruction (wrapped in JoinInst::Compute)
        let dst = Self::alloc_value_id(next_value_id);

        // Convert AST BinaryOperator to JoinIR BinOpKind
        let joinir_op = Self::ast_binop_to_joinir(operator)?;

        body.push(JoinInst::Compute(MirLikeInst::BinOp {
            dst,
            op: joinir_op,
            lhs: lhs_vid,
            rhs: rhs_vid,
        }));

        if crate::config::env::anf_dev_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[phase145/p2] Emitted BinOp: ValueId({}) = ValueId({}) {:?} ValueId({})",
                dst.as_u32(),
                lhs_vid.as_u32(),
                joinir_op,
                rhs_vid.as_u32()
            ));
        }

        Ok(Some(dst))
    }

    /// Phase 145 P2: Normalize and lower an expression recursively
    ///
    /// This is the entry point for recursive ANF transformation.
    /// Handles:
    /// - MethodCall: Hoist to temporary
    /// - BinaryOp: Recursively normalize operands
    /// - Variable/Literal: Direct lowering
    ///
    /// # Returns
    ///
    /// - `Ok(ValueId)`: Normalized result ValueId
    /// - `Err(String)`: Normalization failed
    fn normalize_and_lower(
        ast: &ASTNode,
        env: &mut BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<ValueId, String> {
        match ast {
            // Base case: Variable (already in env)
            ASTNode::Variable { name, .. } => env
                .get(name)
                .copied()
                .ok_or_else(|| format!("normalize_and_lower: undefined variable '{}'", name)),

            // Base case: Literal (needs lowering)
            ASTNode::Literal { value, .. } => {
                let dst = Self::alloc_value_id(next_value_id);
                body.push(JoinInst::Compute(MirLikeInst::Const {
                    dst,
                    value: Self::literal_to_joinir_const(value)?,
                }));
                Ok(dst)
            }

            // Recursive case: MethodCall (hoist to temporary)
            ASTNode::MethodCall { .. } => Self::hoist_method_call(ast, env, body, next_value_id),

            // Recursive case: BinaryOp (normalize operands recursively)
            // Phase 146 P1: Handle both arithmetic and comparison operators
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                if Self::is_compare_operator(operator) {
                    // Phase 146 P1: Comparison operator → emit Compare instruction
                    let result_vid = Self::execute_compare_recursive(
                        left,
                        right,
                        operator,
                        env,
                        body,
                        next_value_id,
                    )?;
                    result_vid
                        .ok_or_else(|| "normalize_and_lower: Compare returned None".to_string())
                } else {
                    // Arithmetic operator → emit BinOp instruction
                    let result_vid = Self::execute_binary_op_recursive(
                        left,
                        right,
                        operator,
                        env,
                        body,
                        next_value_id,
                    )?;
                    result_vid
                        .ok_or_else(|| "normalize_and_lower: BinaryOp returned None".to_string())
                }
            }

            // TODO P3+: UnaryOp, Call, etc.
            _ => Err(format!(
                "normalize_and_lower: unsupported AST node type: {:?}",
                ast
            )),
        }
    }

    /// Convert AST BinaryOperator to JoinIR BinOpKind
    fn ast_binop_to_joinir(
        op: &crate::ast::BinaryOperator,
    ) -> Result<crate::mir::join_ir::BinOpKind, String> {
        use crate::ast::BinaryOperator as AstOp;
        use crate::mir::join_ir::BinOpKind;

        Ok(match op {
            AstOp::Add => BinOpKind::Add,
            AstOp::Subtract => BinOpKind::Sub,
            AstOp::Multiply => BinOpKind::Mul,
            AstOp::Divide => BinOpKind::Div,
            AstOp::Modulo => BinOpKind::Mod,
            _ => {
                return Err(format!(
                    "ast_binop_to_joinir: unsupported operator: {:?}",
                    op
                ))
            }
        })
    }

    /// Convert AST LiteralValue to JoinIR ConstValue
    fn literal_to_joinir_const(
        lit: &crate::ast::LiteralValue,
    ) -> Result<crate::mir::join_ir::ConstValue, String> {
        use crate::ast::LiteralValue as AstLit;
        use crate::mir::join_ir::ConstValue;

        Ok(match lit {
            AstLit::Integer(i) => ConstValue::Integer(*i),
            AstLit::String(s) => ConstValue::String(s.clone()),
            AstLit::Bool(b) => ConstValue::Bool(*b),
            AstLit::Void => ConstValue::Null, // JoinIR uses Null instead of Void
            AstLit::Null => ConstValue::Null,
            AstLit::Float(_) => {
                return Err("literal_to_joinir_const: Float not yet supported in P2".to_string())
            }
        })
    }

    /// Phase 145 P1: Hoist a MethodCall to a temporary variable
    ///
    /// Emits JoinInst::MethodCall and returns the result ValueId.
    fn hoist_method_call(
        ast: &ASTNode,
        env: &BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<ValueId, String> {
        let ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } = ast
        else {
            return Err("hoist_method_call: expected MethodCall AST node".to_string());
        };

        // Get receiver ValueId
        let receiver = match object.as_ref() {
            ASTNode::Variable { name, .. } => env
                .get(name)
                .copied()
                .ok_or_else(|| format!("hoist_method_call: undefined variable '{}'", name))?,
            _ => return Err("hoist_method_call: receiver is not a variable".to_string()),
        };

        // Validate arguments (P1: only 0-arity intrinsics supported)
        if !arguments.is_empty() {
            return Err("hoist_method_call: P1 only supports 0-arity intrinsics".to_string());
        }

        // Allocate result ValueId
        let dst = Self::alloc_value_id(next_value_id);

        // Emit MethodCall instruction
        body.push(JoinInst::MethodCall {
            dst,
            receiver,
            method: method.clone(),
            args: vec![],
            type_hint: Some(MirType::Integer), // P1: String.length() returns Integer
        });

        Ok(dst)
    }

    /// Phase 146 P1: Execute ANF transformation for Compare with recursive normalization
    ///
    /// Pattern: `s.length() == 3` → `t = s.length(); result = t == 3`
    /// Pattern: `s1.length() < s2.length()` → `t1 = s1.length(); t2 = s2.length(); result = t1 < t2`
    ///
    /// This function recursively normalizes left and right operands (depth-first, left-to-right)
    /// and then generates a pure Compare instruction.
    fn execute_compare_hoist(
        _plan: &AnfPlan,
        ast: &ASTNode,
        env: &mut BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        let ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } = ast
        else {
            return Err("ANF execute_compare_hoist: expected BinaryOp AST node".to_string());
        };

        // Verify it's a comparison operator
        if !Self::is_compare_operator(operator) {
            return Err(format!(
                "ANF execute_compare_hoist: expected comparison operator, got {:?}",
                operator
            ));
        }

        // Use recursive normalization (same pattern as BinaryOp)
        Self::execute_compare_recursive(left, right, operator, env, body, next_value_id)
    }

    /// Phase 146 P1: Recursively normalize Compare operands (depth-first, left-to-right)
    ///
    /// This is the core recursive ANF transformation for Compare:
    /// 1. Normalize LEFT operand recursively (depth-first)
    /// 2. Normalize RIGHT operand recursively (left-to-right)
    /// 3. Generate pure Compare instruction
    ///
    /// # Example
    ///
    /// Input: `s.length() == 3`
    /// - Step 1: Normalize `s.length()` → ValueId(1) (emits MethodCall)
    /// - Step 2: Normalize `3` → ValueId(2) (emits Const)
    /// - Step 3: Emit Compare(ValueId(1), ==, ValueId(2)) → ValueId(3)
    ///
    /// Input: `s1.length() < s2.length()`
    /// - Step 1: Normalize `s1.length()` → ValueId(1) (emits MethodCall)
    /// - Step 2: Normalize `s2.length()` → ValueId(2) (emits MethodCall)
    /// - Step 3: Emit Compare(ValueId(1), <, ValueId(2)) → ValueId(3)
    fn execute_compare_recursive(
        left: &ASTNode,
        right: &ASTNode,
        operator: &crate::ast::BinaryOperator,
        env: &mut BTreeMap<String, ValueId>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
    ) -> Result<Option<ValueId>, String> {
        // Step 1: Recursively normalize LEFT (depth-first)
        let lhs_vid = Self::normalize_and_lower(left, env, body, next_value_id)?;

        // Step 2: Recursively normalize RIGHT (left-to-right)
        let rhs_vid = Self::normalize_and_lower(right, env, body, next_value_id)?;

        // Step 3: Generate pure Compare instruction
        let dst = Self::alloc_value_id(next_value_id);

        // Convert AST BinaryOperator (comparison) to JoinIR CompareOp
        let joinir_op = Self::ast_compare_to_joinir(operator)?;

        body.push(JoinInst::Compute(MirLikeInst::Compare {
            dst,
            op: joinir_op,
            lhs: lhs_vid,
            rhs: rhs_vid,
        }));

        if crate::config::env::anf_dev_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[phase146/p1] Emitted Compare: ValueId({}) = ValueId({}) {:?} ValueId({})",
                dst.as_u32(),
                lhs_vid.as_u32(),
                joinir_op,
                rhs_vid.as_u32()
            ));
        }

        Ok(Some(dst))
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

    /// Phase 146 P1: Convert AST BinaryOperator (comparison) to JoinIR CompareOp
    fn ast_compare_to_joinir(op: &crate::ast::BinaryOperator) -> Result<CompareOp, String> {
        use crate::ast::BinaryOperator;
        Ok(match op {
            BinaryOperator::Equal => CompareOp::Eq,
            BinaryOperator::NotEqual => CompareOp::Ne,
            BinaryOperator::Less => CompareOp::Lt,
            BinaryOperator::LessEqual => CompareOp::Le,
            BinaryOperator::Greater => CompareOp::Gt,
            BinaryOperator::GreaterEqual => CompareOp::Ge,
            _ => {
                return Err(format!(
                    "ast_compare_to_joinir: not a comparison operator: {:?}",
                    op
                ))
            }
        })
    }

    /// Allocate a new ValueId
    fn alloc_value_id(next_value_id: &mut u32) -> ValueId {
        let id = *next_value_id;
        *next_value_id += 1;
        ValueId(id)
    }
}

#[cfg(test)]
mod tests {
    use super::super::contract::AnfPlan;
    use super::*;
    use crate::ast::{ASTNode, LiteralValue};
    use crate::mir::join_ir::JoinInst;
    use std::collections::BTreeMap;

    fn span() -> crate::ast::Span {
        crate::ast::Span::unknown()
    }

    #[test]
    fn test_execute_route_declines_without_targets() {
        // A pure/no-target plan is not an executor-owned ANF route.
        let plan = AnfPlan::pure();
        let ast = ASTNode::Literal {
            value: LiteralValue::Integer(42),
            span: span(),
        };
        let mut env = BTreeMap::new();
        let mut body = vec![];
        let mut next_value_id = 1000u32;

        let result =
            AnfExecuteBox::try_execute(&plan, &ast, &mut env, &mut body, &mut next_value_id);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_p2_normalize_variable() {
        // P2: normalize_and_lower should handle variables
        let mut env = BTreeMap::new();
        env.insert("x".to_string(), ValueId(100));
        let mut body = vec![];
        let mut next_value_id = 1000u32;

        let ast = ASTNode::Variable {
            name: "x".to_string(),
            span: span(),
        };

        let result =
            AnfExecuteBox::normalize_and_lower(&ast, &mut env, &mut body, &mut next_value_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ValueId(100));
        assert!(body.is_empty()); // Variable lookup doesn't emit instructions
    }

    #[test]
    fn test_p2_normalize_literal() {
        // P2: normalize_and_lower should emit Const for literals
        let mut env = BTreeMap::new();
        let mut body = vec![];
        let mut next_value_id = 1000u32;

        let ast = ASTNode::Literal {
            value: LiteralValue::Integer(42),
            span: span(),
        };

        let result =
            AnfExecuteBox::normalize_and_lower(&ast, &mut env, &mut body, &mut next_value_id);
        assert!(result.is_ok());
        let vid = result.unwrap();
        assert_eq!(vid, ValueId(1000));
        assert_eq!(body.len(), 1);

        // Check emitted Const instruction
        match &body[0] {
            JoinInst::Compute(MirLikeInst::Const { dst, value }) => {
                assert_eq!(*dst, ValueId(1000));
                match value {
                    crate::mir::join_ir::ConstValue::Integer(v) => assert_eq!(*v, 42),
                    _ => panic!("Expected Integer constant"),
                }
            }
            _ => panic!("Expected Compute(Const) instruction"),
        }
    }

    #[test]
    fn test_p2_nested_binop() {
        // P2: Nested BinaryOp should recursively normalize
        // Pattern: 10 + 5 + 3
        let mut env = BTreeMap::new();
        let mut body = vec![];
        let mut next_value_id = 1000u32;

        // Build AST: (10 + 5) + 3
        let ast = ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Add,
            left: Box::new(ASTNode::BinaryOp {
                operator: crate::ast::BinaryOperator::Add,
                left: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(10),
                    span: span(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(5),
                    span: span(),
                }),
                span: span(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: span(),
            }),
            span: span(),
        };

        let result =
            AnfExecuteBox::normalize_and_lower(&ast, &mut env, &mut body, &mut next_value_id);
        assert!(result.is_ok());

        // Should emit:
        // 1. Const(1000) = 10
        // 2. Const(1001) = 5
        // 3. BinOp(1002) = 1000 + 1001
        // 4. Const(1003) = 3
        // 5. BinOp(1004) = 1002 + 1003
        assert_eq!(body.len(), 5);
        assert_eq!(result.unwrap(), ValueId(1004));
    }

    #[test]
    fn test_p2_double_hoist_left_to_right() {
        // P2: Double MethodCall should hoist left-to-right
        // Pattern: s1.length() + s2.length()
        // This test verifies order preservation
        let mut env = BTreeMap::new();
        env.insert("s1".to_string(), ValueId(100));
        env.insert("s2".to_string(), ValueId(101));
        let mut body = vec![];
        let mut next_value_id = 1000u32;

        // Build AST: s1.length() + s2.length()
        let ast = ASTNode::BinaryOp {
            operator: crate::ast::BinaryOperator::Add,
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

        let result =
            AnfExecuteBox::normalize_and_lower(&ast, &mut env, &mut body, &mut next_value_id);
        assert!(result.is_ok());

        // Should emit:
        // 1. MethodCall(1000) = s1.length()  (LEFT first)
        // 2. MethodCall(1001) = s2.length()  (RIGHT second)
        // 3. BinOp(1002) = 1000 + 1001
        assert_eq!(body.len(), 3);
        assert_eq!(result.unwrap(), ValueId(1002));

        // Verify left-to-right order
        match &body[0] {
            JoinInst::MethodCall { receiver, .. } => {
                assert_eq!(*receiver, ValueId(100)); // s1 first
            }
            _ => panic!("Expected first instruction to be MethodCall for s1"),
        }

        match &body[1] {
            JoinInst::MethodCall { receiver, .. } => {
                assert_eq!(*receiver, ValueId(101)); // s2 second
            }
            _ => panic!("Expected second instruction to be MethodCall for s2"),
        }
    }

    // P2: 5 execute tests (normalize_variable, normalize_literal, nested_binop, double_hoist, left_to_right_order)
}
