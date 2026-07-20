//! ConstantEmissionBox — Const 命令の発行を集約（仕様不変）
//!
//! ✅ Phase 25.1b Fix: All constant emission now uses function-local ID generator
//! when inside a function context to ensure proper SSA verification.

use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, MirInstruction, ValueId};

use super::constant_type::PreparedCanonicalConstTypeV1;

fn emit_exact_const(b: &mut MirBuilder, value: ConstValue) -> Result<ValueId, String> {
    let dst = b.next_value_id();
    let prepared =
        PreparedCanonicalConstTypeV1::prepare(&value, b.function_state.type_ctx.get_type(dst))
            .map_err(|error| error.to_string())?;
    b.emit_instruction(MirInstruction::Const { dst, value })?;
    prepared.commit(dst, &mut b.function_state.type_ctx);
    Ok(dst)
}

#[inline]
pub fn emit_integer(b: &mut MirBuilder, val: i64) -> Result<ValueId, String> {
    emit_exact_const(b, ConstValue::Integer(val))
}

#[inline]
pub fn emit_bool(b: &mut MirBuilder, val: bool) -> Result<ValueId, String> {
    emit_exact_const(b, ConstValue::Bool(val))
}

#[inline]
pub fn emit_float(b: &mut MirBuilder, val: f64) -> Result<ValueId, String> {
    emit_exact_const(b, ConstValue::Float(val))
}

#[inline]
pub fn emit_string<S: Into<String>>(b: &mut MirBuilder, s: S) -> Result<ValueId, String> {
    let s = s.into();
    let dst = emit_exact_const(b, ConstValue::String(s.clone()))?;
    // 137x-H1: string constants are value-world text. Runtime method dispatch may
    // still route through StringBox, but const emission must not create object origin.
    b.function_state.type_ctx.string_literals.insert(dst, s);
    Ok(dst)
}

#[inline]
pub fn emit_null(b: &mut MirBuilder) -> Result<ValueId, String> {
    // Null is syntactic sugar for the exact Void representation.
    emit_exact_const(b, ConstValue::Null)
}

#[inline]
pub fn emit_void(b: &mut MirBuilder) -> Result<ValueId, String> {
    emit_exact_const(b, ConstValue::Void)
}

#[cfg(test)]
mod tests {
    use super::{emit_bool, emit_float, emit_integer, emit_null, emit_string, emit_void};
    use crate::mir::builder::MirBuilder;
    use crate::mir::{ConstValue, MirInstruction, MirType};

    fn instruction_count(builder: &MirBuilder) -> usize {
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .map(|block| block.instructions.len())
            .sum()
    }

    #[test]
    fn every_canonical_const_emits_before_its_exact_transient_fact() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("constant_emission_success/0".to_string());

        let integer = emit_integer(&mut builder, 7).unwrap();
        let boolean = emit_bool(&mut builder, true).unwrap();
        let float = emit_float(&mut builder, 1.5).unwrap();
        let string = emit_string(&mut builder, "text").unwrap();
        let null = emit_null(&mut builder).unwrap();
        let void = emit_void(&mut builder).unwrap();

        let type_ctx = &builder.function_state.type_ctx;
        assert_eq!(type_ctx.get_type(integer), Some(&MirType::Integer));
        assert_eq!(type_ctx.get_type(boolean), Some(&MirType::Bool));
        assert_eq!(type_ctx.get_type(float), Some(&MirType::Float));
        assert_eq!(type_ctx.get_type(string), Some(&MirType::String));
        assert_eq!(type_ctx.get_type(null), Some(&MirType::Void));
        assert_eq!(type_ctx.get_type(void), Some(&MirType::Void));
        assert_eq!(
            type_ctx.string_literals.get(&string),
            Some(&"text".to_string())
        );

        let constants = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter_map(|instruction| match instruction {
                MirInstruction::Const { value, .. } => Some(value),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            constants,
            vec![
                &ConstValue::Integer(7),
                &ConstValue::Bool(true),
                &ConstValue::Float(1.5),
                &ConstValue::String("text".to_string()),
                &ConstValue::Null,
                &ConstValue::Void,
            ]
        );
    }

    #[test]
    fn missing_current_block_publishes_no_const_type_or_string_fact() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("constant_emission_failure/0".to_string());
        builder.function_state.current_block = None;

        let before_instruction_count = instruction_count(&builder);
        assert!(emit_integer(&mut builder, 7).is_err());
        assert!(emit_bool(&mut builder, true).is_err());
        assert!(emit_float(&mut builder, 1.5).is_err());
        assert!(emit_string(&mut builder, "text").is_err());
        assert!(emit_null(&mut builder).is_err());
        assert!(emit_void(&mut builder).is_err());

        assert_eq!(instruction_count(&builder), before_instruction_count);
        assert!(builder.function_state.type_ctx.value_types.is_empty());
        assert!(builder.function_state.type_ctx.string_literals.is_empty());
        assert!(builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .metadata
            .value_types
            .is_empty());
    }
}
