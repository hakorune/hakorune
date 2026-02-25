//! CompareEmissionBox — 比較命令発行の薄いヘルパ（仕様不変）

use crate::mir::builder::MirBuilder;
use crate::mir::{CompareOp, MirInstruction, MirType, ValueId};

#[inline]
pub fn emit_to(
    b: &mut MirBuilder,
    dst: ValueId,
    op: CompareOp,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<(), String> {
    if let (Some(func), Some(cur_bb)) = (b.scope_ctx.current_function.as_mut(), b.current_block) {
        crate::mir::ssot::cf_common::emit_compare_func(func, cur_bb, dst, op, lhs, rhs);
    } else {
        b.emit_instruction(MirInstruction::Compare { dst, op, lhs, rhs })?;
    }
    // 比較結果は Bool 型（既存実装と同じ振る舞い）
    b.type_ctx.value_types.insert(dst, MirType::Bool);
    Ok(())
}

// Convenience wrappers (明示関数名が読みやすい箇所用)
#[inline]
#[allow(dead_code)]
pub fn emit_eq_to(
    b: &mut MirBuilder,
    dst: ValueId,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<(), String> {
    emit_to(b, dst, CompareOp::Eq, lhs, rhs)
}

#[inline]
#[allow(dead_code)]
pub fn emit_ne_to(
    b: &mut MirBuilder,
    dst: ValueId,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<(), String> {
    emit_to(b, dst, CompareOp::Ne, lhs, rhs)
}
