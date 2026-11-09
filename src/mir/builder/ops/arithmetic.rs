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
    build_arithmetic_op_recorded(builder, op, lhs, rhs).map(|completed| completed.value)
}

/// The sole arithmetic append lends its original tuple to ordered Binary
/// completion. Existing value callers borrow this core through the adapter.
pub(in crate::mir::builder) fn build_arithmetic_op_recorded(
    builder: &mut super::super::MirBuilder,
    op: crate::mir::BinaryOp,
    lhs: ValueId,
    rhs: ValueId,
) -> Result<CompletedOrdinaryArithmeticV1, String> {
    let dst = builder.next_value_id();

    let original = if let (Some(func), Some(cur_bb)) = (
        builder.function_state.current_function.as_mut(),
        builder.function_state.current_block,
    ) {
        crate::mir::ssot::binop_lower::emit_binop_to_dst_recorded(func, cur_bb, dst, op, lhs, rhs)
    } else {
        crate::mir::ssot::binop_lower::emit_binop_with(dst, op, lhs, rhs, |instruction| {
            builder.emit_instruction(instruction)
        })?;
        builder.function_state.current_block.map(|block| {
            (
                block,
                crate::mir::ssot::binop_lower::emit_binop_with(dst, op, lhs, rhs, |instruction| {
                    instruction
                }),
            )
        })
    };

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

    Ok(CompletedOrdinaryArithmeticV1 {
        value: dst,
        original,
    })
}

/// Immutable append observation, with no source identity or semantic grant.
/// None means the existing emitter did not append into an existing block.
#[derive(Debug)]
pub(in crate::mir) struct CompletedOrdinaryArithmeticV1 {
    pub(super) value: ValueId,
    original: Option<(crate::mir::BasicBlockId, MirInstruction)>,
}

impl CompletedOrdinaryArithmeticV1 {
    pub(in crate::mir::builder) fn original(
        &self,
    ) -> Option<&(crate::mir::BasicBlockId, MirInstruction)> {
        self.original.as_ref()
    }
}

/// Select the physical arithmetic shape once from the existing checked source
/// law. Exact Integer remains raw; String/carrier operations retain their
/// existing Fault continuation and ordered acquisition prefix.
pub(in crate::mir::builder) fn emit_checked_binary_v1(
    builder: &mut super::super::MirBuilder,
    ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
    operation: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionOperationLoanV1,
    binary: &crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1,
    index: usize,
    left: crate::mir::instruction::StringInvokeOperand,
    right: crate::mir::instruction::StringInvokeOperand,
    writer: &mut crate::mir::builder::ordinary_new_admission::selected::root_graph::RawRootGraphWriterV1,
    fault_frame: ValueId,
    acquired: &mut Vec<(usize, ValueId)>,
    dynamic_carriers: &mut Vec<(usize, ValueId)>,
    temporary_order: &mut Vec<usize>,
    homes: &[crate::mir::instruction::InvokeOperation],
    bindings: &mut Vec<(crate::mir::BasicBlockId, MirInstruction)>,
) -> Result<crate::mir::instruction::StringInvokeOperand, String> {
    use crate::mir::builder::ordinary_new_admission::selected::{
        emit_scalar_operation_with_pending_v1, emit_string_value, freeze, FaultCleanupPlanV1,
    };
    use crate::mir::dynamic_operator_contract::{
        DynamicOperatorFamilyV1 as OperatorFamily, DynamicOperatorNormalResultV1 as ResultKind,
        DynamicOperatorValueClassV1 as OperatorClass,
    };
    use crate::mir::instruction::{
        InvokeOperation, StringInvokeOperand as Operand, StringInvokeOperation as Operation,
    };
    use std::rc::Rc;
    Ok(
        match operation
            .normal_result_v1()
            .ok_or_else(|| freeze("expression/finite-envelope-execution-pending"))?
        {
            ResultKind::NormalInteger => {
                let (Operand::NormalInteger(lhs), Operand::NormalInteger(rhs)) = (left, right)
                else {
                    return Err(freeze("print-expression/integer-operands"));
                };
                let completed = if let Some(scalar) = operation.scalar_source_v1() {
                    let operator = match scalar.integer_operator_v1() {
                        Some(crate::mir::BinaryOp::Add) => crate::ast::BinaryOperator::Add,
                        Some(crate::mir::BinaryOp::Sub) => crate::ast::BinaryOperator::Subtract,
                        Some(crate::mir::BinaryOp::Div) => crate::ast::BinaryOperator::Divide,
                        Some(crate::mir::BinaryOp::Mod) => crate::ast::BinaryOperator::Modulo,
                        _ => return Err(freeze("print-expression/operator-source-missing")),
                    };
                    let (completed, relations) =
                        builder.build_source_scalar_from_values_recorded(operator, lhs, rhs)?;
                    for relation in &relations {
                        relation.append_materialization_chain_v1(bindings);
                    }
                    let function = builder
                        .function_state
                        .current_function
                        .as_ref()
                        .ok_or_else(|| freeze("no-function"))?;
                    let record = ledger.record_scalar_operation_append_v1(
                        Rc::clone(scalar),
                        (lhs, rhs),
                        relations,
                        &completed,
                        builder.local_ssa_physical_correspondence_v1(),
                        function,
                    )?;
                    builder
                        .function_state
                        .local_ssa_correspondence
                        .install_scalar_original(function, record)?;
                    ledger.retain_scalar_operation_use_v1(scalar, false)?;
                    completed
                } else {
                    crate::mir::builder::ops::CompletedOrdinaryBinaryV1::Arithmetic(
                        crate::mir::builder::ops::arithmetic::build_arithmetic_op_recorded(
                            builder,
                            operation.integer_operator_v1().ok_or_else(|| {
                                freeze("print-expression/operator-source-missing")
                            })?,
                            lhs,
                            rhs,
                        )?,
                    )
                };
                let binding = completed
                    .arithmetic_original()
                    .ok_or_else(|| freeze("print-expression/integer-append"))?
                    .clone();
                let value = binding
                    .1
                    .dst_value()
                    .ok_or_else(|| freeze("print-expression/integer-result"))?;
                bindings.push(binding);
                Operand::NormalInteger(value)
            }
            ResultKind::OwnedStringValue => {
                let right = frame.string_add_right_operand_v1(binary.rhs(), right);
                emit_string_value(
                    builder,
                    writer,
                    Operation::Add { left, right },
                    index,
                    fault_frame,
                    acquired,
                    dynamic_carriers,
                    temporary_order,
                    homes,
                    bindings,
                )?
            }
            ResultKind::SelfContainedNonAliasingDynamicCarrier => {
                let envelope = operation
                    .single_envelope()
                    .ok_or_else(|| freeze("print-expression/operator-envelope"))?;
                let domain = envelope.domain();
                let (Operand::Dynamic(lhs), Operand::NormalInteger(rhs)) = (&left, &right) else {
                    return Err(freeze("print-expression/dynamic-add-operands"));
                };
                if domain.left() != OperatorClass::Dynamic || domain.right() != OperatorClass::I64 {
                    return Err(freeze("print-expression/dynamic-add-contract"));
                }
                match domain.family() {
                    OperatorFamily::Add if !frame.owned_result_nodes().contains(&index) => {
                        let block = builder
                            .function_state
                            .current_block
                            .ok_or_else(|| freeze("no-block"))?;
                        let value = builder.next_value_id();
                        let binding = (
                            block,
                            crate::mir::ssot::binop_lower::emit_binop_with(
                                value,
                                crate::mir::BinaryOp::Add,
                                *lhs,
                                *rhs,
                                |instruction| instruction,
                            ),
                        );
                        builder.emit_instruction(binding.1.clone())?;
                        bindings.push(binding);
                        Operand::Dynamic(value)
                    }
                    OperatorFamily::Sub if frame.owned_result_nodes().contains(&index) => {
                        let pending = FaultCleanupPlanV1::pending(
                            acquired,
                            dynamic_carriers,
                            temporary_order,
                            homes,
                        )?;
                        let (value, result, _) = emit_scalar_operation_with_pending_v1(
                            builder,
                            writer,
                            InvokeOperation::DynamicCarrierSubI64 {
                                carrier: *lhs,
                                rhs: *rhs,
                            },
                            fault_frame,
                            &pending,
                            bindings,
                        )?;
                        if result != crate::mir::instruction::InvokeNormalResultKind::DynamicCarrier
                        {
                            return Err(freeze("print-expression/dynamic-sub-result"));
                        }
                        dynamic_carriers.push((index, value));
                        temporary_order.push(index);
                        Operand::DynamicCarrier(value)
                    }
                    _ => return Err(freeze("print-expression/dynamic-operator-contract")),
                }
            }
            _ => return Err(freeze("print-expression/operator-result")),
        },
    )
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
