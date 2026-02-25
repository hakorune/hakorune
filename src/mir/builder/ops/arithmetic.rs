//! # Arithmetic Binary Operations Module
//!
//! ## Purpose
//! Handles arithmetic binary operations (Add, Sub, Mul, Div, Mod, Shl, Shr, BitAnd, BitOr, BitXor)
//! with Type facts integration and operator Box routing.
//!
//! ## Responsibilities
//! - **Operator Box routing**: Route to AddOperator, SubOperator, etc. under NYASH_BUILDER_OPERATOR_BOX flags
//! - **Type facts classification**: Classify String vs Integer for Add operation
//! - **Guard detection**: Prevent infinite recursion (in_add_op, in_sub_op, etc.)
//! - **Core-13 pure expansion**: Use ssot::binop_lower for pure MIR emission
//! - **ALL_CALL flag support**: Enable all operator boxes via NYASH_BUILDER_OPERATOR_BOX_ALL_CALL
//!
//! ## Type Facts Integration
//! The Add operation uses TypeFactsBox to classify operand types:
//! - String + String → StringBox result
//! - Integer + Integer → Integer result
//! - Mixed types → Unknown (use-site coercion in LLVM backend)
//!
//! ## Environment Variables
//! - `NYASH_BUILDER_OPERATOR_BOX_ALL_CALL=1`: Enable all operator boxes
//! - `NYASH_BUILDER_OPERATOR_BOX_ADD_CALL=1`: Enable AddOperator only
//!
//! ## Phase History
//! - Phase Dev: Lower '+' to operator box calls (default OFF)
//! - Phase 196: TypeFacts SSOT - type annotation based on operands
//! - Phase 131-11-E: TypeFacts - classify operand types
//! - Phase 136: Use TypeFactsBox for type inference

use super::super::{MirInstruction, MirType, ValueId};

/// Build an arithmetic binary operation instruction.
///
/// This function handles all arithmetic operations (Add, Sub, Mul, Div, Mod, Shl, Shr,
/// BitAnd, BitOr, BitXor) with optional operator Box routing based on environment flags.
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
/// - Both String → StringBox result
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

    let all_call = crate::config::env::builder_operator_box_all_call();

    // Phase Dev: Lower '+' を演算子ボックス呼び出しに置換（既定OFF）
    let in_add_op = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.starts_with("AddOperator.apply/"))
        .unwrap_or(false);
    if matches!(op, crate::mir::BinaryOp::Add)
        && !in_add_op
        && (all_call || crate::config::env::builder_operator_box_add_call())
    {
        // AddOperator.apply/2(lhs, rhs)
        let name = "AddOperator.apply/2".to_string();
        builder.emit_legacy_call(
            Some(dst),
            super::super::builder_calls::CallTarget::Global(name),
            vec![lhs, rhs],
        )?;
        // Phase 196: TypeFacts SSOT - AddOperator call type annotation
        // Phase 131-11-E: TypeFacts - classify operand types (Phase 136: use TypeFactsBox)
        let type_facts = super::super::type_facts::TypeFactsBox::new(
            &builder.type_ctx.value_types,
            &builder.type_ctx.value_origin_newbox,
        );
        let lhs_type = type_facts.classify_operand_type(lhs);
        let rhs_type = type_facts.classify_operand_type(rhs);

        use super::super::type_facts::OperandTypeClass::*;
        match (lhs_type, rhs_type) {
            (String, String) => {
                // BOTH are strings: result is string
                builder
                    .type_ctx
                    .value_types
                    .insert(dst, MirType::Box("StringBox".to_string()));
                builder
                    .type_ctx
                    .value_origin_newbox
                    .insert(dst, "StringBox".to_string());
            }
            (Integer, Integer) | (Integer, Unknown) | (Unknown, Integer) => {
                // TypeFact: Integer + anything non-String = Integer
                builder.type_ctx.value_types.insert(dst, MirType::Integer);
            }
            (String, Integer) | (Integer, String) => {
                // Mixed types: leave as Unknown for use-site coercion
            }
            (Unknown, Unknown) => {
                // Both Unknown: cannot infer
            }
            (String, Unknown) | (Unknown, String) => {
                // One side is String, other is Unknown: cannot infer safely
            }
        }
    } else if all_call {
        // Lower other arithmetic ops to operator boxes under ALL flag
        let (name, guard_prefix) = match op {
            crate::mir::BinaryOp::Sub => ("SubOperator.apply/2", "SubOperator.apply/"),
            crate::mir::BinaryOp::Mul => ("MulOperator.apply/2", "MulOperator.apply/"),
            crate::mir::BinaryOp::Div => ("DivOperator.apply/2", "DivOperator.apply/"),
            crate::mir::BinaryOp::Mod => ("ModOperator.apply/2", "ModOperator.apply/"),
            crate::mir::BinaryOp::Shl => ("ShlOperator.apply/2", "ShlOperator.apply/"),
            crate::mir::BinaryOp::Shr => ("ShrOperator.apply/2", "ShrOperator.apply/"),
            crate::mir::BinaryOp::BitAnd => {
                ("BitAndOperator.apply/2", "BitAndOperator.apply/")
            }
            crate::mir::BinaryOp::BitOr => {
                ("BitOrOperator.apply/2", "BitOrOperator.apply/")
            }
            crate::mir::BinaryOp::BitXor => {
                ("BitXorOperator.apply/2", "BitXorOperator.apply/")
            }
            _ => ("", ""),
        };
        if !name.is_empty() {
            let in_guard = builder
                .scope_ctx
                .current_function
                .as_ref()
                .map(|f| f.signature.name.starts_with(guard_prefix))
                .unwrap_or(false);
            if !in_guard {
                builder.emit_legacy_call(
                    Some(dst),
                    super::super::builder_calls::CallTarget::Global(name.to_string()),
                    vec![lhs, rhs],
                )?;
                // 型注釈: 算術はおおむね整数（Addは上で注釈済み）
                builder.type_ctx.value_types.insert(dst, MirType::Integer);
            } else {
                // guard中は従来のBinOp
                if let (Some(func), Some(cur_bb)) =
                    (builder.scope_ctx.current_function.as_mut(), builder.current_block)
                {
                    crate::mir::ssot::binop_lower::emit_binop_to_dst(
                        func, cur_bb, dst, op, lhs, rhs,
                    );
                } else {
                    builder.emit_instruction(MirInstruction::BinOp { dst, op, lhs, rhs })?;
                }
                builder.type_ctx.value_types.insert(dst, MirType::Integer);
            }
        } else {
            // 既存の算術経路
            if let (Some(func), Some(cur_bb)) =
                (builder.scope_ctx.current_function.as_mut(), builder.current_block)
            {
                crate::mir::ssot::binop_lower::emit_binop_to_dst(
                    func, cur_bb, dst, op, lhs, rhs,
                );
            } else {
                builder.emit_instruction(MirInstruction::BinOp { dst, op, lhs, rhs })?;
            }
            // Phase 196: TypeFacts SSOT - BinOp type is determined by operands only
            // String concatenation is handled at use-site in LLVM lowering
            if matches!(op, crate::mir::BinaryOp::Add) {
                // Phase 131-11-E: TypeFacts - classify operand types (Phase 136: use TypeFactsBox)
                let type_facts = super::super::type_facts::TypeFactsBox::new(
                    &builder.type_ctx.value_types,
                    &builder.type_ctx.value_origin_newbox,
                );
                let lhs_type = type_facts.classify_operand_type(lhs);
                let rhs_type = type_facts.classify_operand_type(rhs);

                use super::super::type_facts::OperandTypeClass::*;
                match (lhs_type, rhs_type) {
                    (String, String) => {
                        // BOTH are strings: result is definitely a string
                        builder
                            .type_ctx
                            .value_types
                            .insert(dst, MirType::Box("StringBox".to_string()));
                        builder
                            .type_ctx
                            .value_origin_newbox
                            .insert(dst, "StringBox".to_string());
                    }
                    (Integer, Integer) | (Integer, Unknown) | (Unknown, Integer) => {
                        // TypeFact: Integer + anything non-String = Integer
                        // This handles `counter + 1` where counter might be Unknown
                        builder.type_ctx.value_types.insert(dst, MirType::Integer);
                    }
                    (String, Integer) | (Integer, String) => {
                        // Mixed types: leave as Unknown for use-site coercion
                        // LLVM backend will handle string concatenation
                    }
                    (Unknown, Unknown) => {
                        // Both Unknown: cannot infer, leave as Unknown
                    }
                    (String, Unknown) | (Unknown, String) => {
                        // One side is String, other is Unknown: cannot infer safely
                        // Leave as Unknown
                    }
                }
            } else {
                builder.type_ctx.value_types.insert(dst, MirType::Integer);
            }
        }
    } else {
        // 既存の算術経路
        if let (Some(func), Some(cur_bb)) =
            (builder.scope_ctx.current_function.as_mut(), builder.current_block)
        {
            crate::mir::ssot::binop_lower::emit_binop_to_dst(
                func, cur_bb, dst, op, lhs, rhs,
            );
        } else {
            builder.emit_instruction(MirInstruction::BinOp { dst, op, lhs, rhs })?;
        }
        // Phase 196: TypeFacts SSOT - BinOp type is determined by operands only
        // String concatenation is handled at use-site in LLVM lowering
        if matches!(op, crate::mir::BinaryOp::Add) {
            // Check if BOTH operands are known to be strings (TypeFacts)
            let lhs_is_str = match builder.type_ctx.value_types.get(&lhs) {
                Some(MirType::String) => true,
                Some(MirType::Box(bt)) if bt == "StringBox" => true,
                _ => builder
                    .type_ctx
                    .value_origin_newbox
                    .get(&lhs)
                    .map(|s| s == "StringBox")
                    .unwrap_or(false),
            };
            let rhs_is_str = match builder.type_ctx.value_types.get(&rhs) {
                Some(MirType::String) => true,
                Some(MirType::Box(bt)) if bt == "StringBox" => true,
                _ => builder
                    .type_ctx
                    .value_origin_newbox
                    .get(&rhs)
                    .map(|s| s == "StringBox")
                    .unwrap_or(false),
            };
            if lhs_is_str && rhs_is_str {
                // BOTH are strings: result is definitely a string
                builder
                    .type_ctx
                    .value_types
                    .insert(dst, MirType::Box("StringBox".to_string()));
                builder
                    .type_ctx
                    .value_origin_newbox
                    .insert(dst, "StringBox".to_string());
            } else if !lhs_is_str && !rhs_is_str {
                // NEITHER is a string: numeric addition
                builder.type_ctx.value_types.insert(dst, MirType::Integer);
            }
            // else: Mixed types (string + int or int + string)
            // Leave dst type as Unknown - LLVM will handle coercion at use-site
        } else {
            builder.type_ctx.value_types.insert(dst, MirType::Integer);
        }
    }

    // Fail-fast: Verify BinOp Add's operands are defined (strict/dev+planner_required only)
    // The dst will be defined by this instruction, but operands must be defined upstream
    if crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled()
    {
        // Check only for Add operation (our target: %229's Add generation point)
        if matches!(op, crate::mir::BinaryOp::Add) {
            if let Some(func) = builder.scope_ctx.current_function.as_ref() {
                let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);

                // Check both lhs and rhs operands are defined
                let check_operand = |name: &str, v: ValueId| -> Result<(), String> {
                    if !def_blocks.contains_key(&v) {
                        let span = builder.metadata_ctx.current_span();
                        let file = builder.metadata_ctx.current_source_file().unwrap_or_else(|| "unknown".to_string());

                        Err(format!(
                            "[freeze:contract][ops/binop_add:operand_not_defined] fn={} bb={:?} operand={} v=%{} span={} span_start={} span_end={} file={}",
                            func.signature.name,
                            builder.current_block,
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
