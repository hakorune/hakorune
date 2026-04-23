//! ConstantEmissionBox — Const 命令の発行を集約（仕様不変）
//!
//! ✅ Phase 25.1b Fix: All constant emission now uses function-local ID generator
//! when inside a function context to ensure proper SSA verification.

use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, MirInstruction, ValueId};

#[inline]
pub fn emit_integer(b: &mut MirBuilder, val: i64) -> Result<ValueId, String> {
    let dst = b.next_value_id();
    b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Integer(val),
    })?;
    // Phase 84-1: Integer constant type annotation
    b.type_ctx
        .value_types
        .insert(dst, crate::mir::MirType::Integer);
    Ok(dst)
}

#[inline]
pub fn emit_bool(b: &mut MirBuilder, val: bool) -> Result<ValueId, String> {
    let dst = b.next_value_id();
    b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Bool(val),
    })?;
    // Phase 84-1: Bool constant type annotation
    b.type_ctx
        .value_types
        .insert(dst, crate::mir::MirType::Bool);
    Ok(dst)
}

#[inline]
pub fn emit_float(b: &mut MirBuilder, val: f64) -> Result<ValueId, String> {
    let dst = b.next_value_id();
    b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Float(val),
    })?;
    // Phase 84-1: Float constant type annotation
    b.type_ctx
        .value_types
        .insert(dst, crate::mir::MirType::Float);
    Ok(dst)
}

#[inline]
pub fn emit_string<S: Into<String>>(b: &mut MirBuilder, s: S) -> Result<ValueId, String> {
    let s = s.into();
    let dst = b.next_value_id();
    b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::String(s.clone()),
    })?;
    // 137x-H1: string constants are value-world text. Runtime method dispatch may
    // still route through StringBox, but const emission must not create object origin.
    b.type_ctx
        .value_types
        .insert(dst, crate::mir::MirType::String);
    b.type_ctx.string_literals.insert(dst, s);
    Ok(dst)
}

#[inline]
pub fn emit_null(b: &mut MirBuilder) -> Result<ValueId, String> {
    let dst = b.next_value_id();
    b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Null,
    })?;
    // Phase 285A1.1: Null constant type annotation
    // Null is syntactic sugar for Void (SSOT: lifecycle.md)
    b.type_ctx
        .value_types
        .insert(dst, crate::mir::MirType::Void);
    Ok(dst)
}

#[inline]
pub fn emit_void(b: &mut MirBuilder) -> Result<ValueId, String> {
    let dst = b.next_value_id();
    b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Void,
    })?;
    // Phase 84-1: Void constant type annotation
    b.type_ctx
        .value_types
        .insert(dst, crate::mir::MirType::Void);
    Ok(dst)
}
