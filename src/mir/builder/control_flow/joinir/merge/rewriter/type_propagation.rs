//! Type propagation utilities for instruction rewriter
//!
//! Phase 260 P0.1 Step 4: Extracted from instruction_rewriter.rs (lines 29-99)
//! Propagates type information from JoinIR source to HOST MIR during merge.

use crate::mir::types::ConstValue;
use crate::mir::{MirFunction, MirInstruction, MirType};

/// Propagate value type information from source instruction to remapped instruction
///
/// This function ensures type information flows correctly from JoinIR→MIR merge:
/// 1. **SSOT from source**: Prefer type info from JoinIR source function metadata
/// 2. **Fallback inference**: Infer from instruction structure if no source hint
///
/// ## Why This Matters
///
/// The LLVM harness relies on `dst_type` hints to keep `+` as numeric add
/// instead of `concat_hh_mixed` when types are otherwise unknown.
///
/// ## Supported Instructions
///
/// - Const, BinOp, UnaryOp, Compare, Load, TypeOp
/// - Copy, NewBox, Phi
///
/// ## Example
///
/// ```ignore
/// // Before remapping
/// let old_inst = MirInstruction::Const {
///     dst: ValueId(42),  // JoinIR value
///     value: ConstValue::Integer(10),
/// };
///
/// // After remapping
/// let new_inst = MirInstruction::Const {
///     dst: ValueId(123), // HOST value
///     value: ConstValue::Integer(10),
/// };
///
/// // Propagate type: ValueId(123) → MirType::Integer
/// propagate_value_type_for_inst(builder, source_func, &old_inst, &new_inst);
/// ```
pub(in crate::mir::builder::control_flow::joinir::merge) fn propagate_value_type_for_inst(
    builder: &mut crate::mir::builder::MirBuilder,
    source_func: &MirFunction,
    old_inst: &MirInstruction,
    new_inst: &MirInstruction,
) {
    let Some(_) = builder.scope_ctx.current_function else {
        return;
    };

    let (old_dst, new_dst) = match (old_inst, new_inst) {
        (MirInstruction::Const { dst: o, .. }, MirInstruction::Const { dst: n, .. }) => (o, n),
        (MirInstruction::BinOp { dst: o, .. }, MirInstruction::BinOp { dst: n, .. }) => (o, n),
        (MirInstruction::UnaryOp { dst: o, .. }, MirInstruction::UnaryOp { dst: n, .. }) => (o, n),
        (MirInstruction::Compare { dst: o, .. }, MirInstruction::Compare { dst: n, .. }) => (o, n),
        (MirInstruction::Load { dst: o, .. }, MirInstruction::Load { dst: n, .. }) => (o, n),
        (MirInstruction::TypeOp { dst: o, .. }, MirInstruction::TypeOp { dst: n, .. }) => (o, n),
        (MirInstruction::Copy { dst: o, .. }, MirInstruction::Copy { dst: n, .. }) => (o, n),
        (MirInstruction::NewBox { dst: o, .. }, MirInstruction::NewBox { dst: n, .. }) => (o, n),
        (MirInstruction::Phi { dst: o, .. }, MirInstruction::Phi { dst: n, .. }) => (o, n),
        _ => return,
    };

    // Prefer SSOT from the source MIR (JoinIR→MIR bridge may have hints).
    if let Some(ty) = source_func.metadata.value_types.get(old_dst).cloned() {
        builder.type_ctx.value_types.insert(*new_dst, ty);
        return;
    }

    // Fallback inference from the rewritten instruction itself.
    //
    // This is important for the LLVM harness: it relies on `dst_type` hints to keep `+`
    // as numeric add instead of `concat_hh_mixed` when types are otherwise unknown.
    match new_inst {
        MirInstruction::Const { dst, value } if dst == new_dst => {
            let ty = match value {
                ConstValue::Integer(_) => Some(MirType::Integer),
                ConstValue::Float(_) => Some(MirType::Float),
                ConstValue::Bool(_) => Some(MirType::Bool),
                ConstValue::String(_) => Some(MirType::String),
                ConstValue::Void => Some(MirType::Void),
                _ => None,
            };
            if let Some(ty) = ty {
                builder.type_ctx.value_types.insert(*dst, ty);
            }
        }
        MirInstruction::Copy { dst, src } if dst == new_dst => {
            if let Some(src_ty) = builder.type_ctx.value_types.get(src).cloned() {
                builder.type_ctx.value_types.insert(*dst, src_ty);
            }
        }
        MirInstruction::BinOp { dst, lhs, rhs, .. } if dst == new_dst => {
            let lhs_ty = builder.type_ctx.value_types.get(lhs);
            let rhs_ty = builder.type_ctx.value_types.get(rhs);
            if matches!(lhs_ty, Some(MirType::Integer)) && matches!(rhs_ty, Some(MirType::Integer))
            {
                builder.type_ctx.value_types.insert(*dst, MirType::Integer);
            }
        }
        MirInstruction::Compare { dst, .. } if dst == new_dst => {
            builder.type_ctx.value_types.insert(*dst, MirType::Bool);
        }
        _ => {}
    }
}
