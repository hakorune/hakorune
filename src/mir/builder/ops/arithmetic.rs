//! # Arithmetic Binary Operations Module
//!
//! ## Purpose
//! Handles arithmetic binary operations (Add, Sub, Mul, Div, Mod, Shl, Shr, BitAnd, BitOr, BitXor)
//! with Type facts integration and direct MIR routing.
//!
//! ## Responsibilities
//! - **Type facts classification**: Classify String vs Integer for Add operation
//! - **Core-13 pure expansion**: Use ssot::binop_lower for pure MIR emission
//!
//! ## Type Facts Integration
//! The Add operation uses TypeFactsBox to classify operand types:
//! - String + String → String value result
//! - Integer + Integer → Integer result
//! - Mixed types → Unknown (use-site coercion in LLVM backend)
//!
//! ## Phase History
//! - Phase Dev: Lower '+' to operator box calls (default OFF)
//! - Phase 196: TypeFacts SSOT - type annotation based on operands
//! - Phase 131-11-E: TypeFacts - classify operand types
//! - Phase 136: Use TypeFactsBox for type inference

use super::super::{MirInstruction, MirType, ValueId};

fn classify_operand_type(
    builder: &super::super::MirBuilder,
    value: ValueId,
) -> super::super::type_facts::OperandTypeClass {
    use super::super::type_facts::OperandTypeClass;

    let type_facts = super::super::type_facts::TypeFactsBox::new(
        &builder.function_state.type_ctx.value_types,
        &builder.function_state.type_ctx.value_origin_newbox,
    );
    let class = type_facts.classify_operand_type(value);
    if class != OperandTypeClass::Unknown {
        return class;
    }

    let Some(function) = builder.function_state.current_function.as_ref() else {
        return OperandTypeClass::Unknown;
    };
    match function.metadata.value_types.get(&value) {
        Some(MirType::String) => OperandTypeClass::String,
        Some(MirType::Box(name)) if name == "StringBox" => OperandTypeClass::String,
        Some(MirType::Integer | MirType::Bool) => OperandTypeClass::Integer,
        _ => OperandTypeClass::Unknown,
    }
}

/// Build an arithmetic binary operation instruction.
///
/// This function handles all arithmetic operations (Add, Sub, Mul, Div, Mod, Shl, Shr,
/// BitAnd, BitOr, BitXor) through the direct MIR owner.
///
/// # Arguments
/// - `builder`: MIR builder context
/// - `op`: Binary operation type
/// - `lhs`: Left-hand side ValueId
/// - `rhs`: Right-hand side ValueId
///
/// # Returns
/// - `Ok(ValueId)`: Result value ID
/// - `Err(String)`: Error message
///
/// # Type Inference
/// For Add operations:
/// - Both String → String value result
/// - Both Integer → Integer result
/// - Mixed/Unknown → Unknown (use-site coercion)
///
/// For other arithmetic ops:
/// - Always Integer result type
pub(in crate::mir::builder) fn build_arithmetic_op(
    builder: &mut super::super::MirBuilder,
    op: crate::mir::BinaryOp,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<ValueId, String> {
    let dst = builder.next_value_id();

    if let (Some(func), Some(cur_bb)) = (
        builder.function_state.current_function.as_mut(),
        builder.function_state.current_block,
    ) {
        crate::mir::ssot::binop_lower::emit_binop_to_dst(func, cur_bb, dst, op, lhs, rhs);
    } else {
        builder.emit_instruction(MirInstruction::BinOp { dst, op, lhs, rhs })?;
    }

    // TypeFacts SSOT: direct BinOp emission records only source-backed types.
    if matches!(op, crate::mir::BinaryOp::Add) {
        use super::super::type_facts::OperandTypeClass::*;
        let lhs_type = classify_operand_type(builder, lhs);
        let rhs_type = classify_operand_type(builder, rhs);
        if lhs_type == String && rhs_type == String {
            builder
                .function_state
                .type_ctx
                .value_types
                .insert(dst, MirType::String);
        } else if lhs_type == Integer && rhs_type == Integer {
            builder
                .function_state
                .type_ctx
                .value_types
                .insert(dst, MirType::Integer);
        }
    } else {
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, MirType::Integer);
    }

    // Fail-fast: Verify BinOp Add's operands are defined (strict/dev+planner_required only)
    // The dst will be defined by this instruction, but operands must be defined upstream
    if crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled()
    {
        // Check only for Add operation (our target: %229's Add generation point)
        if matches!(op, crate::mir::BinaryOp::Add) {
            if let Some(func) = builder.function_state.current_function.as_ref() {
                let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);

                // Check both lhs and rhs operands are defined
                let check_operand = |name: &str, v: ValueId| -> Result<(), String> {
                    if !def_blocks.contains_key(&v) {
                        let span = builder.metadata_ctx.current_span();
                        let file = builder
                            .metadata_ctx
                            .current_source_file()
                            .unwrap_or_else(|| "unknown".to_string());

                        Err(format!(
                            "[freeze:contract][ops/binop_add:operand_not_defined] fn={} bb={:?} operand={} v=%{} span={} span_start={} span_end={} file={}",
                            func.signature.name,
                            builder.function_state.current_block,
                            name,
                            v.0,
                            span.location_string(),
                            span.start,
                            span.end,
                            file
                        ))
                    } else {
                        Ok(())
                    }
                };

                check_operand("lhs", lhs)?;
                check_operand("rhs", rhs)?;
            }
        }
    }

    Ok(dst)
}

#[cfg(test)]
mod tests {
    use super::build_arithmetic_op;
    use crate::mir::builder::MirBuilder;
    use crate::mir::{BinaryOp, MirType, ValueId};

    #[test]
    fn add_does_not_promote_unknown_plus_integer_to_integer() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("arithmetic_unknown_add/0".to_string());
        let lhs = ValueId::new(40);
        let rhs = ValueId::new(41);
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(rhs, MirType::Integer);

        let result = build_arithmetic_op(&mut builder, BinaryOp::Add, lhs, rhs).unwrap();

        assert_eq!(builder.function_state.type_ctx.get_type(result), None);
    }

    #[test]
    fn add_keeps_exact_integer_plus_integer_inference() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("arithmetic_integer_add/0".to_string());
        let lhs = ValueId::new(50);
        let rhs = ValueId::new(51);
        for value in [lhs, rhs] {
            builder
                .function_state
                .type_ctx
                .value_types
                .insert(value, MirType::Integer);
        }

        let result = build_arithmetic_op(&mut builder, BinaryOp::Add, lhs, rhs).unwrap();

        assert_eq!(
            builder.function_state.type_ctx.get_type(result),
            Some(&MirType::Integer)
        );
    }
}
