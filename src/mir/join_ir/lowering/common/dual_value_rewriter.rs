//! Phase 246-EX / 247-EX: Dual-value rewrite helpers (Box)
//!
//! Purpose: isolate name-based rewrite rules for promoted condition carriers
//! and loop-local derived carriers, so loop_break lowering remains structural.
//!
//! This module is intentionally narrow and fail-fast-ish:
//! - It only performs rewrites when it can prove the required body-local source exists.
//! - Otherwise it leaves the original instructions/behavior unchanged.

use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::{CompareOp, ConstValue, JoinInst, MirLikeInst, UnaryOp};
use crate::mir::ValueId;

/// Rewrite a lowered break-condition instruction stream to use fresh body-local values
/// instead of stale promoted carrier parameters.
///
/// Current supported rewrite:
/// - `!<is_*>` where operand matches a carrier Join ValueId, and body-local provides `<name>`
///   derived from `carrier.name.strip_prefix("is_")`.
pub fn rewrite_break_condition_insts(
    insts: Vec<JoinInst>,
    carrier_info: &CarrierInfo,
    body_local_env: Option<&LoopBodyLocalEnv>,
    alloc_value: &mut dyn FnMut() -> ValueId,
) -> Vec<JoinInst> {
    let mut out = Vec::with_capacity(insts.len());

    for inst in insts.into_iter() {
        match inst {
            JoinInst::Compute(MirLikeInst::UnaryOp {
                op: UnaryOp::Not,
                operand,
                dst,
            }) => {
                let mut operand_value = operand;

                // Check if operand is a promoted carrier (e.g., is_digit)
                for carrier in &carrier_info.carriers {
                    if carrier.join_id == Some(operand_value) {
                        if let Some(stripped) = carrier.name.strip_prefix("is_") {
                            let source_name = stripped.to_string();
                            if let Some(src_val) =
                                body_local_env.and_then(|env| env.get(&source_name))
                            {
                                // Emit fresh comparison: is_* = (source >= 0)
                                let zero = alloc_value();
                                out.push(JoinInst::Compute(MirLikeInst::Const {
                                    dst: zero,
                                    value: ConstValue::Integer(0),
                                }));

                                let fresh_bool = alloc_value();
                                out.push(JoinInst::Compute(MirLikeInst::Compare {
                                    dst: fresh_bool,
                                    op: CompareOp::Ge,
                                    lhs: src_val,
                                    rhs: zero,
                                }));

                                operand_value = fresh_bool;
                            }
                        }
                    }
                }

                out.push(JoinInst::Compute(MirLikeInst::UnaryOp {
                    dst,
                    op: UnaryOp::Not,
                    operand: operand_value,
                }));
            }
            other => out.push(other),
        }
    }

    out
}

/// Try to derive an updated value for a loop-local derived carrier (e.g., `<x>_value`)
/// from a body-local `<x>_pos` value.
pub fn try_derive_looplocal_from_bodylocal_pos(
    carrier_name: &str,
    body_local_env: Option<&LoopBodyLocalEnv>,
) -> Option<ValueId> {
    let stripped = carrier_name.strip_suffix("_value")?;
    let source_name = format!("{}_pos", stripped);
    body_local_env.and_then(|env| env.get(&source_name))
}

/// Try to derive a condition-only boolean carrier (e.g., `is_<x>`) from a body-local `<x>_pos`.
///
/// Emits: `cmp = (<x>_pos >= 0)` and returns `cmp` ValueId.
pub fn try_derive_conditiononly_is_from_bodylocal_pos(
    carrier_name: &str,
    body_local_env: Option<&LoopBodyLocalEnv>,
    alloc_value: &mut dyn FnMut() -> ValueId,
    out: &mut Vec<JoinInst>,
) -> Option<ValueId> {
    let stripped = carrier_name.strip_prefix("is_")?;
    let source_name = format!("{}_pos", stripped);
    let src_val = body_local_env.and_then(|env| env.get(&source_name))?;

    let zero = alloc_value();
    out.push(JoinInst::Compute(MirLikeInst::Const {
        dst: zero,
        value: ConstValue::Integer(0),
    }));

    let cmp = alloc_value();
    out.push(JoinInst::Compute(MirLikeInst::Compare {
        dst: cmp,
        op: CompareOp::Ge,
        lhs: src_val,
        rhs: zero,
    }));

    Some(cmp)
}
