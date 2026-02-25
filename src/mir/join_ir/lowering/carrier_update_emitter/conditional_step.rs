use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::carrier_info::CarrierVar;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::condition_lowerer::lower_condition_to_joinir_no_body_locals;
use crate::mir::join_ir::{BinOpKind, ConstValue, JoinInst, MirLikeInst, VarId};
use crate::mir::MirType;
use crate::mir::ValueId;

// Phase 92 P0-3: ConditionalStep Support
// ============================================================================

/// Emit JoinIR instructions for conditional step update (Phase 92 P0-3)
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
/// * `carrier` - Carrier variable information (name, ValueId)
/// * `cond_ast` - AST node for the condition expression (e.g., `ch == '\\'`)
/// * `then_delta` - Delta to add when condition is true
/// * `else_delta` - Delta to add when condition is false
/// * `alloc_value` - ValueId allocator closure
/// * `env` - ConditionEnv for variable resolution
/// * `instructions` - Output vector to append instructions to
///
/// # Returns
///
/// ValueId of the computed update result (the dst of Select)
#[allow(dead_code)]
pub fn emit_conditional_step_update(
    carrier: &CarrierVar,
    cond_ast: &ASTNode,
    then_delta: i64,
    else_delta: i64,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    // Step 1: Lower the condition expression
    // Phase 92 P2-2: No body-local support in legacy emitter (use common/conditional_step_emitter instead)
    let (cond_id, cond_insts) = lower_condition_to_joinir_no_body_locals(cond_ast, alloc_value, env)?;
    instructions.extend(cond_insts);

    // Step 2: Get carrier parameter ValueId from env
    let carrier_param = env
        .get(&carrier.name)
        .ok_or_else(|| format!("Carrier '{}' not found in ConditionEnv", carrier.name))?;

    // Step 3: Compute then_result = carrier + then_delta
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

    // Step 4: Compute else_result = carrier + else_delta
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

    // Step 5: Emit Select instruction
    let carrier_new: VarId = alloc_value();
    instructions.push(JoinInst::Select {
        dst: carrier_new,
        cond: cond_id,
        then_val: then_result,
        else_val: else_result,
        type_hint: Some(MirType::Integer),  // Carrier is always Integer
    });

    Ok(carrier_new)
}
