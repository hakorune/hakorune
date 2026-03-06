use crate::mir::join_ir::lowering::carrier_info::CarrierVar;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::loop_update_analyzer::{UpdateExpr, UpdateRhs};
use crate::mir::join_ir::{BinOpKind, ConstValue, JoinInst, MirLikeInst};
use crate::mir::ValueId;

/// Emit JoinIR instructions for a single carrier update (backward compatibility version)
///
/// This function is kept for backward compatibility with existing break/continue route code
/// that only needs ConditionEnv. New code should prefer `emit_carrier_update_with_env`.
///
/// # Arguments
///
/// * `carrier` - Carrier variable information (name, ValueId)
/// * `update` - Update expression (e.g., CounterLike, AccumulationLike)
/// * `alloc_value` - ValueId allocator closure
/// * `env` - ConditionEnv for variable resolution
/// * `instructions` - Output vector to append instructions to
///
/// # Returns
///
/// ValueId of the computed update result
///
/// # Example
///
/// ```ignore
/// // For "count = count + 1":
/// let count_next = emit_carrier_update(
///     &count_carrier,
///     &UpdateExpr::BinOp { lhs: "count", op: Add, rhs: Const(1) },
///     &mut alloc_value,
///     &env,
///     &mut instructions,
/// )?;
/// // Generates:
/// //   const_1 = Const(1)
/// //   count_next = BinOp(Add, count_param, const_1)
/// ```
pub fn emit_carrier_update(
    carrier: &CarrierVar,
    update: &UpdateExpr,
    alloc_value: &mut dyn FnMut() -> ValueId,
    env: &ConditionEnv,
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    match update {
        UpdateExpr::Const(step) => {
            // CounterLike: carrier = carrier + step
            // Allocate const ValueId
            let const_id = alloc_value();
            instructions.push(JoinInst::Compute(MirLikeInst::Const {
                dst: const_id,
                value: ConstValue::Integer(*step),
            }));

            // Get carrier parameter ValueId from env
            let carrier_param = env
                .get(&carrier.name)
                .ok_or_else(|| format!("Carrier '{}' not found in ConditionEnv", carrier.name))?;

            // Allocate result ValueId
            let result = alloc_value();
            instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: result,
                op: BinOpKind::Add,
                lhs: carrier_param,
                rhs: const_id,
            }));

            Ok(result)
        }

        UpdateExpr::BinOp { lhs, op, rhs } => {
            // General binary operation: carrier = carrier op rhs
            // Verify lhs matches carrier name
            if lhs != &carrier.name {
                return Err(format!(
                    "Update expression LHS '{}' doesn't match carrier '{}'",
                    lhs, carrier.name
                ));
            }

            // Get carrier parameter ValueId from env
            let carrier_param = env
                .get(&carrier.name)
                .ok_or_else(|| format!("Carrier '{}' not found in ConditionEnv", carrier.name))?;

            // Resolve RHS
            let rhs_id = match rhs {
                UpdateRhs::Const(n) => {
                    let const_id = alloc_value();
                    instructions.push(JoinInst::Compute(MirLikeInst::Const {
                        dst: const_id,
                        value: ConstValue::Integer(*n),
                    }));
                    const_id
                }
                UpdateRhs::Variable(var_name) => env.get(var_name).ok_or_else(|| {
                    format!(
                        "Update RHS variable '{}' not found in ConditionEnv",
                        var_name
                    )
                })?,
                // Phase 188: String updates now emit JoinIR BinOp
                // StringAppendLiteral: s = s + "literal"
                UpdateRhs::StringLiteral(s) => {
                    let const_id = alloc_value();
                    instructions.push(JoinInst::Compute(MirLikeInst::Const {
                        dst: const_id,
                        value: ConstValue::String(s.clone()),
                    }));
                    const_id
                }
                // Phase 190: Number accumulation pattern: result = result * base + digit
                // Emit as: tmp = carrier * base; result = tmp + digit
                UpdateRhs::NumberAccumulation { base, digit_var } => {
                    // Step 1: Emit const for base
                    let base_id = alloc_value();
                    instructions.push(JoinInst::Compute(MirLikeInst::Const {
                        dst: base_id,
                        value: ConstValue::Integer(*base),
                    }));

                    // Step 2: Emit multiplication: tmp = carrier * base
                    let tmp_id = alloc_value();
                    instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
                        dst: tmp_id,
                        op: BinOpKind::Mul,
                        lhs: carrier_param,
                        rhs: base_id,
                    }));

                    // Step 3: Resolve digit variable
                    let digit_id = env.get(digit_var).ok_or_else(|| {
                        format!(
                            "Number accumulation digit variable '{}' not found in ConditionEnv",
                            digit_var
                        )
                    })?;

                    // Step 4: Emit addition: result = tmp + digit
                    let result = alloc_value();
                    instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
                        dst: result,
                        op: *op, // Use the operation from outer UpdateExpr
                        lhs: tmp_id,
                        rhs: digit_id,
                    }));

                    // Return result directly - we've already emitted everything
                    return Ok(result);
                }
                // Phase 178/188: Complex updates (method calls) still rejected
                UpdateRhs::Other => {
                    return Err(format!(
                        "Carrier '{}' has complex update (UpdateRhs::Other) - should be rejected by can_lower()",
                        carrier.name
                    ));
                }
            };

            // Allocate result ValueId
            let result = alloc_value();
            instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
                dst: result,
                op: *op,
                lhs: carrier_param,
                rhs: rhs_id,
            }));

            Ok(result)
        }
    }
}

// ============================================================================
