//! Phase 92 P1-1: ConditionalStep Emitter Module
//!
//! Specialized emitter for conditional step updates in P5b (escape sequence) patterns.
//! Extracted from carrier_update_emitter.rs for improved modularity and single responsibility.
//!
//! # Design
//!
//! - **Single Responsibility**: Handles only ConditionalStep emission with Fail-Fast validation
//! - **Isolated Logic**: No side effects, pure JoinIR generation
//! - **Clean Interface**: Exports only `emit_conditional_step_update()`
//!
//! # Fail-Fast Invariants
//!
//! 1. `then_delta != else_delta` - ConditionalStep must have different deltas
//! 2. Condition must be pure expression (no side effects)

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::condition_lowerer::lower_condition_to_joinir;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv; // Phase 92 P2-2
use crate::mir::join_ir::{BinOpKind, ConstValue, JoinInst, MirLikeInst, VarId};
use crate::mir::{MirType, ValueId};

/// Emit JoinIR instructions for conditional step update (Phase 92 P1-1)
///
/// Handles the P5b escape sequence pattern where carrier update depends on a condition:
/// ```text
/// if escape_cond { carrier = carrier + then_delta }
/// else { carrier = carrier + else_delta }
/// ```
///
/// This generates:
/// 1. Lower condition expression to get cond_id
/// 2. Compute then_result = carrier + then_delta
/// 3. Compute else_result = carrier + else_delta
/// 4. JoinInst::Select { dst: carrier_new, cond: cond_id, then_val: then_result, else_val: else_result }
///
/// # Arguments
///
/// * `carrier_name` - Name of the carrier variable (e.g., "i", "pos")
/// * `carrier_param` - ValueId of the carrier parameter in JoinIR
/// * `cond_ast` - AST node for the condition expression (e.g., `ch == '\\'`)
/// * `then_delta` - Delta to add when condition is true
/// * `else_delta` - Delta to add when condition is false
/// * `alloc_value` - ValueId allocator closure
/// * `env` - ConditionEnv for variable resolution
/// * `body_local_env` - Phase 92 P2-2: Optional body-local variable environment
/// * `instructions` - Output vector to append instructions to
///
/// # Phase 92 P2-2: Body-Local Variable Support
///
/// When the condition references body-local variables (e.g., `ch == '\\'` in escape patterns),
/// the `body_local_env` provides name → ValueId mappings for variables defined in the loop body.
///
/// # Returns
///
/// ValueId of the computed update result (the dst of Select)
///
/// # Errors
///
/// Returns error if:
/// - `then_delta == else_delta` (Fail-Fast: invariant violation)
/// - Condition lowering fails (Fail-Fast: must be pure expression)
pub fn emit_conditional_step_update(
    carrier_name: &str,
    carrier_param: ValueId,
    cond_ast: &ASTNode,
    then_delta: i64,
    else_delta: i64,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    body_local_env: Option<&LoopBodyLocalEnv>, // Phase 92 P2-2
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    // Phase 92 P1-1: Fail-Fast check - then_delta must differ from else_delta
    if then_delta == else_delta {
        return Err(format!(
            "ConditionalStep invariant violated: then_delta ({}) must differ from else_delta ({}) for carrier '{}'",
            then_delta, else_delta, carrier_name
        ));
    }

    // Phase 92 P2-2: Lower the condition expression with body-local support
    let (cond_id, cond_insts) = lower_condition_to_joinir(cond_ast, alloc_value, env, body_local_env, None).map_err(|e| {
        format!(
            "ConditionalStep invariant violated: condition must be pure expression for carrier '{}': {}",
            carrier_name, e
        )
    })?;
    instructions.extend(cond_insts);

    // Step 2: Compute then_result = carrier + then_delta
    let then_const_id = alloc_value();
    instructions.push(JoinInst::Compute(MirLikeInst::Const {
        dst: then_const_id,
        value: ConstValue::Integer(then_delta),
    }));
    let then_result = alloc_value();
    instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: then_result,
        op: BinOpKind::Add,
        lhs: carrier_param,
        rhs: then_const_id,
    }));

    // Step 3: Compute else_result = carrier + else_delta
    let else_const_id = alloc_value();
    instructions.push(JoinInst::Compute(MirLikeInst::Const {
        dst: else_const_id,
        value: ConstValue::Integer(else_delta),
    }));
    let else_result = alloc_value();
    instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: else_result,
        op: BinOpKind::Add,
        lhs: carrier_param,
        rhs: else_const_id,
    }));

    // Step 4: Emit Select instruction
    let carrier_new: VarId = alloc_value();
    instructions.push(JoinInst::Select {
        dst: carrier_new,
        cond: cond_id,
        then_val: then_result,
        else_val: else_result,
        type_hint: Some(MirType::Integer), // Carrier is always Integer
    });

    Ok(carrier_new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};
    use crate::mir::join_ir::lowering::condition_env::ConditionEnv;

    fn test_env() -> ConditionEnv {
        let mut env = ConditionEnv::new();
        env.insert("ch".to_string(), ValueId(10));
        env.insert("i".to_string(), ValueId(20));
        env
    }

    fn ch_eq_backslash() -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(ASTNode::Variable {
                name: "ch".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::String("\\".to_string()),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_emit_conditional_step_basic() {
        // Test: if ch == "\\" { i = i + 2 } else { i = i + 1 }
        let env = test_env();
        let cond_ast = ch_eq_backslash();

        let mut value_counter = 100u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };

        let mut instructions = Vec::new();
        let result = emit_conditional_step_update(
            "i",
            ValueId(20), // carrier_param
            &cond_ast,
            2, // then_delta (escape: i + 2)
            1, // else_delta (normal: i + 1)
            &mut alloc_value,
            &env,
            None,
            &mut instructions,
        );

        assert!(result.is_ok());
        let result_id = result.unwrap();

        // Should generate:
        // 1. Condition lowering instructions (Compare)
        // 2. Const(2), BinOp(Add, i, 2) for then_result
        // 3. Const(1), BinOp(Add, i, 1) for else_result
        // 4. Select(cond, then_result, else_result)
        assert!(instructions.len() >= 6); // At least 6 instructions

        // Check Select instruction exists
        let select_found = instructions
            .iter()
            .any(|inst| matches!(inst, JoinInst::Select { .. }));
        assert!(select_found, "Select instruction should be emitted");

        assert!(result_id.0 >= 100);
    }

    #[test]
    fn test_fail_fast_equal_deltas() {
        // Phase 92 P1-1: Test Fail-Fast when then_delta == else_delta
        let env = test_env();
        let cond_ast = ch_eq_backslash();

        let mut value_counter = 200u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };

        let mut instructions = Vec::new();
        let result = emit_conditional_step_update(
            "i",
            ValueId(20),
            &cond_ast,
            2, // then_delta
            2, // else_delta (SAME! Should fail)
            &mut alloc_value,
            &env,
            None,
            &mut instructions,
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("ConditionalStep invariant violated"));
        assert!(err.contains("then_delta"));
        assert!(err.contains("must differ from else_delta"));
    }

    #[test]
    fn test_fail_fast_invalid_condition() {
        // Phase 92 P1-1: Test Fail-Fast when condition is not pure
        let env = ConditionEnv::new(); // Empty env - ch will not be found

        let cond_ast = ch_eq_backslash();

        let mut value_counter = 300u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };

        let mut instructions = Vec::new();
        let result = emit_conditional_step_update(
            "i",
            ValueId(20),
            &cond_ast,
            2, // then_delta
            1, // else_delta
            &mut alloc_value,
            &env,
            None,
            &mut instructions,
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("ConditionalStep invariant violated"));
        assert!(err.contains("condition must be pure expression"));
    }
}
