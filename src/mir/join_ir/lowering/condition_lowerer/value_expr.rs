use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::join_ir::{BinOpKind, ConstValue, JoinInst, MirLikeInst};
use crate::mir::ValueId;

use super::super::condition_env::ConditionEnv;
use super::super::loop_body_local_env::LoopBodyLocalEnv; // Phase 92 P2-2: Body-local support
use super::super::method_call_lowerer::MethodCallLowerer;

/// Lower a literal value (e.g., `10`, `true`, `"text"`)
pub(super) fn lower_literal(
    value: &LiteralValue,
    alloc_value: &mut dyn FnMut() -> ValueId,
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    let dst = alloc_value();
    let const_value = match value {
        LiteralValue::Integer(n) => ConstValue::Integer(*n),
        LiteralValue::String(s) => ConstValue::String(s.clone()),
        LiteralValue::Bool(b) => ConstValue::Bool(*b),
        LiteralValue::Float(_) => {
            return Err("Float literals not supported in JoinIR conditions yet".to_string());
        }
        _ => {
            return Err(format!(
                "Unsupported literal type in condition: {:?}",
                value
            ));
        }
    };

    instructions.push(JoinInst::Compute(MirLikeInst::Const {
        dst,
        value: const_value,
    }));

    Ok(dst)
}

/// Lower a value expression (for comparison operands, etc.)
///
/// This handles the common case where we need to evaluate a simple value
/// (variable or literal) as part of a comparison.
///
/// # Phase 92 P2-2
///
/// Added `body_local_env` parameter to support body-local variable resolution
/// (e.g., `ch` in `ch == '\\'`).
///
/// # Phase 252
///
/// Added `current_static_box_name` parameter to support `this.method(...)` calls
/// in argument expressions.
pub fn lower_value_expression(
    expr: &ASTNode,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    body_local_env: Option<&LoopBodyLocalEnv>, // Phase 92 P2-2
    current_static_box_name: Option<&str>,     // Phase 252
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    match expr {
        // Phase 92 P2-2: Variables - resolve from ConditionEnv or LoopBodyLocalEnv
        ASTNode::Variable { name, .. } => {
            // Priority 1: ConditionEnv (loop parameters, captured variables)
            if let Some(value_id) = env.get(name) {
                return Ok(value_id);
            }
            // Priority 2: LoopBodyLocalEnv (body-local variables like `ch`)
            if let Some(body_env) = body_local_env {
                if let Some(value_id) = body_env.get(name) {
                    return Ok(value_id);
                }
            }
            Err(format!(
                "Variable '{}' not found in ConditionEnv or LoopBodyLocalEnv",
                name
            ))
        }

        // Literals - emit as constants
        ASTNode::Literal { value, .. } => lower_literal(value, alloc_value, instructions),

        // Binary operations (for arithmetic in conditions like i + 1 < n)
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => lower_arithmetic_binop(
            operator,
            left,
            right,
            alloc_value,
            env,
            body_local_env,
            current_static_box_name,
            instructions,
        ),

        // Phase 224-C: MethodCall support with arguments (e.g., s.length(), s.indexOf(ch))
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => {
            // 1. Lower receiver (object) to ValueId
            let recv_val = lower_value_expression(
                object,
                alloc_value,
                env,
                body_local_env,
                current_static_box_name,
                instructions,
            )?;

            // 2. Lower method call using MethodCallLowerer
            // Phase 256.7: Use lower_for_init (more permissive whitelist) for value expressions
            // Value expressions like s.substring(i, i+1) should be allowed even in condition arguments
            let empty_body_local = LoopBodyLocalEnv::new();
            let body_env = body_local_env.unwrap_or(&empty_body_local);
            MethodCallLowerer::lower_for_init(
                recv_val,
                method,
                arguments,
                alloc_value,
                env,
                body_env,
                instructions,
            )
        }

        _ => Err(format!(
            "Unsupported expression in value context: {:?}",
            expr
        )),
    }
}

/// Lower an arithmetic binary operation (e.g., `i + 1`)
fn lower_arithmetic_binop(
    operator: &BinaryOperator,
    left: &ASTNode,
    right: &ASTNode,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    body_local_env: Option<&LoopBodyLocalEnv>, // Phase 92 P2-2
    current_static_box_name: Option<&str>,     // Phase 252
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    let lhs = lower_value_expression(
        left,
        alloc_value,
        env,
        body_local_env,
        current_static_box_name,
        instructions,
    )?;
    let rhs = lower_value_expression(
        right,
        alloc_value,
        env,
        body_local_env,
        current_static_box_name,
        instructions,
    )?;
    let dst = alloc_value();

    let bin_op = match operator {
        BinaryOperator::Add => BinOpKind::Add,
        BinaryOperator::Subtract => BinOpKind::Sub,
        BinaryOperator::Multiply => BinOpKind::Mul,
        BinaryOperator::Divide => BinOpKind::Div,
        BinaryOperator::Modulo => BinOpKind::Mod,
        _ => {
            return Err(format!(
                "Unsupported binary operator in expression: {:?}",
                operator
            ));
        }
    };

    instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst,
        op: bin_op,
        lhs,
        rhs,
    }));

    Ok(dst)
}
