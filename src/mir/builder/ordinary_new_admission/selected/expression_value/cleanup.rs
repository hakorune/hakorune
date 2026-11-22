//! Exact cleanup stack construction for the one typed acquisition ledger.
use super::*;

pub(in crate::mir::builder::ordinary_new_admission::selected) fn bool_condition_temporary_operations_v1(
    frame: &CheckedExpressionEvaluationV1,
    comparison: usize,
    body: &EmittedExpressionBodyV1,
) -> Result<Vec<InvokeOperation>, String> {
    let order = frame
        .scalar_invoke_input_order_v1(comparison)
        .ok_or_else(|| freeze("bool-condition/comparison-input-order"))?;
    let expected: std::collections::BTreeSet<_> = order
        .iter()
        .copied()
        .filter(|node| frame.owned_result_nodes().contains(node))
        .collect();
    let actual: std::collections::BTreeSet<_> =
        body.temporaries.iter().map(|(node, _)| *node).collect();
    if body.temporaries.len() != expected.len() || actual != expected {
        return Err(freeze("bool-condition/temporary-order-coverage"));
    }
    for &(node, temporary) in &body.temporaries {
        if !temporary.matches_value_v1(body.values.get(node).and_then(Option::as_ref)) {
            return Err(freeze(match temporary {
                ExpressionOwnedTemporaryV1::String(_) => "bool-condition/string-temporary-coverage",
                _ => "bool-condition/dynamic-core-temporary-coverage",
            }));
        }
    }
    let mut dynamic_call_nodes = std::collections::BTreeSet::new();
    for packet in &body.dynamic_calls {
        let call = packet.source();
        let node = frame
            .source()
            .nodes()
            .iter()
            .position(|row| row.site() == call.site().site())
            .ok_or_else(|| freeze("bool-condition/dynamic-call-node"))?;
        if !expected.contains(&node)
            || !dynamic_call_nodes.insert(node)
            || call.dynamic_invocation_result_lifecycle_v1().is_none()
            || !frame.calls().iter().any(|original| original == call)
            || !body.temporaries.contains(&(
                node,
                ExpressionOwnedTemporaryV1::DynamicCarrier(packet.value()),
            ))
            || body.values.get(node).and_then(Option::as_ref)
                != Some(&Operand::DynamicCarrier(packet.value()).into())
        {
            return Err(freeze("bool-condition/dynamic-temporary-coverage"));
        }
    }
    for &(node, temporary) in &body.temporaries {
        let ExpressionOwnedTemporaryV1::DynamicCarrier(value) = temporary else {
            continue;
        };
        if body.dynamic_calls.iter().any(|packet| {
            packet.value() == value
                && frame.source().nodes()[node].site() == packet.source().site().site()
        }) {
            continue;
        }
        if body.calls.iter().any(|packet| {
            packet.call_site().site() == frame.source().nodes()[node].site()
                && matches!(packet.original_source().result(), Some(
                    crate::mir::instruction::InvokeCallResultKind::NullableText
                        | crate::mir::instruction::InvokeCallResultKind::OwnedDynamicCarrier
                ))
                && packet.outer_bindings().is_ok_and(|(_, normal)| {
                    matches!(normal.1, MirInstruction::InvokeNormalResult { dst, .. } if dst == value)
                })
        }) {
            continue;
        }
        let source_producer = if let Some(core) = frame.cores().get(&node) {
            matches!(
                crate::mir::normal_callable_semantic_package::checked_expression_core_operation_v1(
                    core,
                    &body.values
                )?,
                std::ops::ControlFlow::Break(InvokeOperation::DynamicArrayGet { .. })
            )
        } else if let Some(operation) = frame
            .operations()
            .iter()
            .find(|operation| operation.node() == node)
        {
            operation.normal_result_v1() == Some(ResultKind::SelfContainedNonAliasingDynamicCarrier)
                && operation
                    .single_envelope()
                    .is_some_and(|envelope| envelope.domain().family() == OperatorFamily::Sub)
        } else {
            false
        };
        if !source_producer {
            return Err(freeze("bool-condition/dynamic-core-temporary-coverage"));
        }
    }
    FaultCleanupPlanV1::owed_operations(&body.temporaries, &[])
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_dynamic_initializer_continuation(
    builder: &mut MirBuilder,
    expression: &mut EmittedExpressionValueV1,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    let Operand::DynamicCarrier(value) = expression.value.as_operand()? else {
        return Err(freeze("initializer-expression/dynamic-carrier-result"));
    };
    let projection = expression
        .body
        .bindings
        .iter()
        .find(|(_, row)| {
            matches!(row, MirInstruction::InvokeNormalResult { dst, .. } if *dst == *value)
        })
        .cloned()
        .ok_or_else(|| freeze("initializer-expression/dynamic-carrier-projection"))?;
    if expression
        .body
        .bindings
        .iter()
        .filter(|(_, row)| {
            matches!(row, MirInstruction::InvokeNormalResult { dst, .. } if *dst == *value)
        })
        .count()
        != 1
    {
        return Err(freeze("initializer-expression/dynamic-carrier-projection"));
    }
    if !matches!(expression.body.temporaries.last(), Some((0, ExpressionOwnedTemporaryV1::DynamicCarrier(carrier))) if carrier == value)
        || expression
            .body
            .temporaries
            .iter()
            .filter(|(_, temporary)| *temporary == ExpressionOwnedTemporaryV1::DynamicCarrier(*value))
            .count()
            != 1
    {
        return Err(freeze("initializer-expression/dynamic-carrier-order"));
    }
    let MirInstruction::InvokeNormalResult { invoke_block, .. } = projection.1 else {
        return Err(freeze("initializer-expression/dynamic-carrier-projection"));
    };
    // Transport does not select dispatch. Ordinary nullable Text calls lend
    // their exact checked Call/Normal pair, just as dynamic producers do.
    let carrier_invoke = |binding: &Binding| {
        if binding.0 != invoke_block {
            return false;
        }
        let MirInstruction::Invoke {
            operation,
            normal_landing,
            ..
        } = &binding.1
        else {
            return false;
        };
        if *normal_landing != projection.0 {
            return false;
        }
        match operation {
            InvokeOperation::DynamicInvocation(_)
            | InvokeOperation::DynamicArrayGet { .. }
            | InvokeOperation::DynamicCarrierSubI64 { .. } => true,
            InvokeOperation::Call {
                result: result @ (crate::mir::instruction::InvokeCallResultKind::NullableText | crate::mir::instruction::InvokeCallResultKind::OwnedDynamicCarrier),
                ..
            } => {
                let mut packets = expression.body.calls.iter().filter(|packet| {
                    packet.original_source().result()
                        == Some(*result)
                        && packet.outer_bindings().is_ok_and(|(invoke, normal)| {
                            invoke == binding && normal == &projection
                        })
                });
                packets.next().is_some() && packets.next().is_none()
            }
            _ => false,
        }
    };
    if expression
        .body
        .bindings
        .iter()
        .filter(|binding| carrier_invoke(binding))
        .count()
        != 1
    {
        return Err(freeze("initializer-expression/dynamic-carrier-invoke"));
    }
    if expression.body.temporaries.pop() != Some((0, ExpressionOwnedTemporaryV1::DynamicCarrier(*value))) {
        return Err(freeze("initializer-expression/dynamic-carrier-order"));
    }
    finish_body_continuation(builder, &mut expression.body, None, None, false)
}

/// I64 purpose completion retains the scalar result and ends only its child
/// Text acquisitions. Prior Homes remain live on Normal and unwind on Fault.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_scalar_continuation(
    builder: &mut MirBuilder,
    expression: &mut EmittedExpressionValueV1,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    finish_continuation(builder, expression, None, None, false)
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_residence_with_displaced_continuation(
    builder: &mut MirBuilder,
    expression: &mut EmittedExpressionValueV1,
    displaced: Option<InvokeOperation>,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    let Operand::OwnedString(root) = expression.value.as_operand()?.clone() else {
        return Err(freeze("expression/residence-root"));
    };
    let materialized = expression
        .purpose_acquisition
        .as_ref()
        .and_then(|pair| pair[1].1.dst_value());
    if materialized != Some(root)
        && expression
            .body
            .string_acquisitions_v1()
            .last()
            .is_none_or(|(_, value)| *value != root)
    {
        return Err(freeze("expression/residence-acquisition-order"));
    }
    finish_continuation(builder, expression, Some(root), displaced, false)
}

/// An owned argument lends its entire remaining stack to the enclosing Call.
/// The newest root prevents earlier children from ending before that Call.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_deferred_argument_continuation(
    builder: &mut MirBuilder,
    expression: &mut EmittedExpressionValueV1,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    let Operand::OwnedString(root) = expression.value.as_operand()?.clone() else {
        return Err(freeze("argument-expression/deferred-root"));
    };
    let materialized = expression
        .purpose_acquisition
        .as_ref()
        .and_then(|pair| pair[1].1.dst_value());
    if materialized != Some(root) && expression.body.string_acquisitions_v1().last() != Some(&(0, root)) {
        return Err(freeze("argument-expression/deferred-acquisition-order"));
    }
    let mut writer = RawRootGraphWriterV1;
    let next = writer.reserve_block(builder)?;
    let clean = writer.reserve_block(builder)?;
    let continuation = (
        clean,
        MirInstruction::Jump {
            target: next,
            edge_args: None,
        },
    );
    writer.append_terminated(
        builder,
        clean,
        continuation.1.clone(),
        &mut expression.body.bindings,
    )?;
    let terminal = writer.emit_current(
        builder,
        MirInstruction::Jump {
            target: clean,
            edge_args: None,
        },
    )?;
    expression.body.bindings.push(terminal.clone());
    builder.start_new_block(next)?;
    Ok((Some(terminal), Some(continuation)))
}

/// Materialize after the original source acquisition-prefix validation. The
/// original operand table and node stack remain untouched for both final scans.
/// Shared purpose acquisition; preserve the original evaluation operands.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn materialize_text_root(
    builder: &mut MirBuilder,
    expression: &mut EmittedExpressionValueV1,
    required: bool,
    source: &CheckedExpressionEvaluationV1,
) -> Result<(), String> {
    if !required {
        return Ok(());
    }
    let operand = expression.value.as_operand()?;
    let snapshot = matches!(operand, Operand::DynamicCarrier(_))
        && source.text_snapshot_root_v1().is_some_and(|node| {
            expression.body.values.get(node).and_then(Option::as_ref) == Some(&expression.value)
        });
    if !(operand.is_text() || snapshot) || expression.purpose_acquisition.is_some() {
        return Err(freeze("rebind-expression/materialize-domain"));
    }
    let pending = FaultCleanupPlanV1::pending(
        &expression.body.temporaries,
        &expression.body.homes,
    )?;
    let (value, _, pair) = emit_scalar_operation_with_pending_v1(
        builder,
        &mut RawRootGraphWriterV1,
        InvokeOperation::String(Operation::Materialize {
            value: expression.value.as_operand()?.clone(),
        }),
        expression.body.required_fault_frame_v1()?,
        &pending,
        &mut expression.body.bindings,
    )?;
    expression.purpose_acquisition = Some(pair);
    expression.value = Operand::OwnedString(value).into();
    Ok(())
}

/// Acquire the original, still-live tagged borrow before the Return cleanup.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn materialize_carrier_return_v1(
    builder: &mut MirBuilder,
    expression: &mut EmittedExpressionValueV1,
) -> Result<(), String> {
    let Operand::Dynamic(tagged) = expression.value.as_operand()? else {
        return Err(freeze("expression/return-carrier-origin"));
    };
    if expression.purpose_acquisition.is_some() {
        return Err(freeze("expression/return-carrier-origin"));
    }
    let pending = FaultCleanupPlanV1::pending(&expression.body.temporaries, &expression.body.homes)?;
    let (value, _, pair) = emit_scalar_operation_with_pending_v1(
        builder,
        &mut RawRootGraphWriterV1,
        InvokeOperation::DynamicMaterializeBorrowed { tagged: *tagged },
        expression.body.required_fault_frame_v1()?,
        &pending,
        &mut expression.body.bindings,
    )?;
    expression.body.temporaries.push((0, ExpressionOwnedTemporaryV1::DynamicCarrier(value)));
    expression.purpose_acquisition = Some(pair);
    expression.value = Operand::DynamicCarrier(value).into();
    Ok(())
}

/// A typed destination checks the original Dynamic result without consuming
/// the carrier. The common cleanup still owns its End on both edges.
pub(in crate::mir::builder::ordinary_new_admission::selected) fn check_i64_destination_v1(
    builder: &mut MirBuilder,
    expression: &mut EmittedExpressionValueV1,
) -> Result<(), String> {
    let Operand::DynamicCarrier(carrier) = expression.value.as_operand()? else {
        return Err(freeze("expression/scalar-continuation-result"));
    };
    if expression.purpose_acquisition.is_some() {
        return Err(freeze("expression/scalar-continuation-result"));
    }
    let pending = FaultCleanupPlanV1::pending(&expression.body.temporaries, &expression.body.homes)?;
    let (value, result, pair) = emit_scalar_operation_with_pending_v1(
        builder,
        &mut RawRootGraphWriterV1,
        InvokeOperation::DynamicCarrierInteger { carrier: *carrier },
        expression.body.required_fault_frame_v1()?,
        &pending,
        &mut expression.body.bindings,
    )?;
    if result != crate::mir::instruction::InvokeNormalResultKind::I64 {
        return Err(freeze("expression/scalar-continuation-result"));
    }
    expression.purpose_acquisition = Some(pair);
    expression.value = Operand::NormalInteger(value).into();
    Ok(())
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_rebind_continuation(
    builder: &mut MirBuilder,
    expression: &mut EmittedExpressionValueV1,
    frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1,
    displaced: Option<InvokeOperation>,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    if frame.declared_read_v1().is_some() {
        if !matches!(
            expression.value,
            EvaluatedExpressionValueV1::BoxReference(_)
        ) {
            return Err(freeze("expression/scalar-continuation-result"));
        }
        return finish_body_continuation(builder, &mut expression.body, None, None, false);
    }
    if !frame.is_text_residence() {
        return finish_continuation(
            builder,
            expression,
            None,
            None,
            frame.is_dynamic_loop_carrier_v1(),
        );
    }
    let Operand::OwnedString(root) = expression.value.as_operand()?.clone() else {
        return Err(freeze("expression/residence-root"));
    };
    if expression.purpose_acquisition.is_some() != frame.materializes_root()
        || !frame.materializes_root() && expression.body.string_acquisitions_v1().last() != Some(&(0, root))
    {
        return Err(freeze("expression/residence-acquisition-order"));
    }
    finish_continuation(builder, expression, Some(root), displaced, false)
}

fn finish_continuation(
    builder: &mut MirBuilder,
    expression: &mut EmittedExpressionValueV1,
    residence: Option<ValueId>,
    displaced: Option<InvokeOperation>,
    dynamic_loop_carrier: bool,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    let scalar_result = matches!(
        expression.value,
        EvaluatedExpressionValueV1::Operand(Operand::NormalInteger(_))
            | EvaluatedExpressionValueV1::Bool(_)
    ) || dynamic_loop_carrier
        && matches!(
            expression.value,
            EvaluatedExpressionValueV1::Operand(Operand::Dynamic(_) | Operand::DynamicCarrier(_))
        );
    if residence.is_none() && !scalar_result {
        return Err(freeze("expression/scalar-continuation-result"));
    }
    finish_body_continuation(
        builder,
        &mut expression.body,
        residence,
        displaced,
        residence.is_some() && expression.purpose_acquisition.is_some(),
    )
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_value_continuation_v1(
    builder: &mut MirBuilder,
    body: &mut EmittedExpressionBodyV1,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    finish_body_continuation(builder, body, None, None, false)
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_bool_condition_continuation_v1(
    builder: &mut MirBuilder,
    body: &mut EmittedExpressionBodyV1,
    normal: Vec<InvokeOperation>,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    let fault = normal
        .iter()
        .cloned()
        .chain(body.homes.iter().cloned())
        .collect();
    finish_body_cleanup(builder, body, normal, fault, None)
}

/// Transfer the exact Normal Dynamic carrier result into its Local receipt.
/// The carrier is intentionally left live here; its Local scope owner must
/// later discharge it with DynamicCarrierEnd on every exit path.
fn finish_body_continuation(
    builder: &mut MirBuilder,
    body: &mut EmittedExpressionBodyV1,
    residence: Option<ValueId>,
    displaced: Option<InvokeOperation>,
    materialized: bool,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    if let Some(value) = residence {
        let block = builder
            .function_state
            .current_block
            .ok_or_else(|| freeze("no-block"))?;
        let replaces = match displaced.as_ref() {
            Some(InvokeOperation::String(
                Operation::End { value } | Operation::EndIfOwned { value },
            )) => Some(*value),
            Some(_) => return Err(freeze("expression/residence-replacement-kind")),
            None => None,
        };
        let install = MirInstruction::StringResidenceInstall {
            value,
            replaces,
            returning: false,
        };
        builder.emit_instruction(install.clone())?;
        body.bindings.push((block, install));
        if !materialized {
            if body.temporaries.pop() != Some((0, ExpressionOwnedTemporaryV1::String(value))) {
                return Err(freeze("expression/residence-acquisition-order"));
            }
        }
    }
    let normal = FaultCleanupPlanV1::owed_operations(
        &body.temporaries,
        &[],
    )?;
    let mut homes = body.homes.clone();
    if let Some(value) = residence {
        homes.insert(0, InvokeOperation::String(Operation::End { value }));
    }
    let fault = FaultCleanupPlanV1::owed_operations(
        &body.temporaries,
        &homes,
    )?;
    finish_body_cleanup(builder, body, normal, fault, displaced)
}

pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_carrier_return(
    builder: &mut MirBuilder,
    body: &mut EmittedExpressionBodyV1,
    carrier: ValueId,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    if body.temporaries.last() != Some(&(0, ExpressionOwnedTemporaryV1::DynamicCarrier(carrier))) {
        return Err(freeze("expression/return-carrier-origin"));
    }
    let fault = FaultCleanupPlanV1::owed_operations(&body.temporaries, &body.homes)?;
    body.temporaries.pop();
    let normal = FaultCleanupPlanV1::owed_operations(&body.temporaries, &[])?;
    finish_body_cleanup(builder, body, normal, fault, None)
}

fn finish_body_cleanup(
    builder: &mut MirBuilder,
    body: &mut EmittedExpressionBodyV1,
    mut normal: Vec<InvokeOperation>,
    fault: Vec<InvokeOperation>,
    displaced: Option<InvokeOperation>,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    if (body.fault_frame.is_none() || body.root_cleanup.is_some())
        && (!normal.is_empty() || displaced.is_some())
    {
        return Err(freeze("expression/fault-frame-required"));
    }
    let Some(fault_frame) = body.fault_frame else {
        return Ok((None, None));
    };
    if let Some((continuation, _)) = &body.root_cleanup {
        if fault != body.homes {
            return Err(freeze("expression/fault-frame-required"));
        }
        let terminal = RawRootGraphWriterV1.emit_current(
            builder,
            MirInstruction::Jump {
                target: continuation.0,
                edge_args: None,
            },
        )?;
        body.bindings.push(terminal.clone());
        return Ok((Some(terminal), Some(continuation.clone())));
    }
    emit_body_continuation(
        builder,
        body,
        fault_frame,
        |_| FaultCleanupPlanV1::body(normal, fault, displaced),
        None,
    )
}

impl EmittedExpressionValueV1 {
    pub(in crate::mir::builder::ordinary_new_admission::selected) fn finish_statement(
        &mut self,
        builder: &mut MirBuilder,
        frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1,
    ) -> Result<(Option<Binding>, Option<Binding>), String> {
        if let Some(fault_frame) = self.body.fault_frame {
            emit_body_continuation(
                builder,
                &mut self.body,
                fault_frame,
                |body| {
                    FaultCleanupPlanV1::temporaries_then_homes(
                        &body.temporaries,
                        &body.homes,
                        !frame.is_discard(),
                    )
                },
                Some((frame, &self.value)),
            )
        } else {
            if !frame.is_discard() {
                return Err(freeze("expression/fault-frame-required"));
            }
            let continuation = finish_value_continuation_v1(builder, &mut self.body)?;
            deduplicate_body_bindings(&mut self.body);
            Ok(continuation)
        }
    }
}

fn emit_body_continuation(
    builder: &mut MirBuilder,
    body: &mut EmittedExpressionBodyV1,
    fault_frame: ValueId,
    make_plan: impl FnOnce(&EmittedExpressionBodyV1) -> Result<FaultCleanupPlanV1, String>,
    statement: Option<(
        &crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1,
        &EvaluatedExpressionValueV1,
    )>,
) -> Result<(Option<Binding>, Option<Binding>), String> {
    let mut writer = RawRootGraphWriterV1;
    let next = writer.reserve_block(builder)?;
    let clean = writer.reserve_block(builder)?;
    let continuation = (
        clean,
        MirInstruction::Jump {
            target: next,
            edge_args: None,
        },
    );
    writer.append_terminated(builder, clean, continuation.1.clone(), &mut body.bindings)?;
    let plan = make_plan(body)?;
    let instruction = if statement.is_some() || !plan.normal().is_empty() {
        let graph = root_graph::emit_cleanup_suffixes_with_writer(
            &mut writer,
            builder,
            fault_frame,
            clean,
            &plan,
            &mut body.bindings,
        )?;
        match statement {
            Some((frame, value)) if !frame.is_discard() => MirInstruction::Invoke {
                operation: InvokeOperation::String(Operation::Print {
                    value: value.as_operand()?.clone(),
                }),
                fault_frame,
                normal_landing: graph.clean,
                fault_landing: graph.acquisition_fault,
            },
            _ => MirInstruction::Jump {
                target: graph.clean,
                edge_args: None,
            },
        }
    } else {
        MirInstruction::Jump {
            target: clean,
            edge_args: None,
        }
    };
    let terminal = writer.emit_current(builder, instruction)?;
    body.bindings.push(terminal.clone());
    builder.start_new_block(next)?;
    deduplicate_body_bindings(body);
    Ok((Some(terminal), Some(continuation)))
}

fn deduplicate_body_bindings(body: &mut EmittedExpressionBodyV1) {
    let mut unique = Vec::new();
    for binding in std::mem::take(&mut body.bindings) {
        if !unique.contains(&binding) {
            unique.push(binding);
        }
    }
    body.bindings = unique;
}

#[path = "inline.rs"]
mod inline;
pub(super) use inline::{begin_inline_scope_v1, finish_inline_scope_v1};
