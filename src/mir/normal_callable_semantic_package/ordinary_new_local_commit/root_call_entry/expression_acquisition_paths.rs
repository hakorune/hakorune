//! Independent expression acquisition paths.
use super::*;

impl ExpressionPhysicalProjectionRefV1<'_> {
    /// Shared physical operation/Fault correspondence. The purpose retains its
    /// terminal and Normal lifetime policy; this walk issues no source rights.
    pub(super) fn validate_acquisition_paths_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        ordered: &[Binding],
        homes: &[InvokeOperation],
        fault_frame: ValueId,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<
        (
            Vec<ExpressionOwnedTemporaryV1>,
            std::collections::BTreeSet<BasicBlockId>,
        ),
        String,
    > {
        use super::super::super::root_cleanup_graph::ordered_paths as paths;
        use std::collections::BTreeSet;
        let mut live = Vec::<ExpressionOwnedTemporaryV1>::new();
        let mut covered = BTreeSet::new();
        let inherited_homes = inherited
            .map(|prefix| prefix.for_argument(self.frame))
            .transpose()?
            .unwrap_or_default();
        let project_homes = |current: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1]| -> Result<Vec<InvokeOperation>, String> {
            let mut unwind = inherited_homes.clone();
            unwind.extend(context.ledger().expression_home_unwind_at_v1(self.frame, current, self.prior_home_unwind)?);
            Ok(unwind)
        };
        let entry_site = self
            .frame
            .ownership_v1()
            .first_site()
            .cloned()
            .unwrap_or_else(|| {
                OwnedExprSiteV1::new(
                    self.frame.source().owner(),
                    self.frame.source().nodes()[0].site().clone(),
                )
            });
        let entry = inherited
            .map(|prefix| prefix.homes_before_site_v1(&entry_site, self.frame.prior_homes()))
            .transpose()?
            .unwrap_or(self.frame.prior_homes());
        let mut current_homes = project_homes(entry)?;
        let end_sequence = |live: &[ExpressionOwnedTemporaryV1], homes: &[InvokeOperation]| {
            live.iter()
                .rev()
                .copied()
                .map(ExpressionOwnedTemporaryV1::end_operation)
                .collect::<Result<Vec<_>, _>>().map(|mut operations| { operations.extend_from_slice(homes); operations })
        };
        let calls = self
            .source_calls()
            .zip(self.calls())
            .map(|(source, call)| {
                let effects = call
                    .ordered_call_bindings_in_context_v1(
                        self.frame.source().owner(),
                        source.arguments(),
                        context,
                    )?
                    .into_iter()
                    .filter(|(_, instruction)| matches!(instruction, MirInstruction::Invoke { .. }))
                    .map(|binding| match finishing {
                        Some(projection) => projection
                            .binding(binding.0, &binding.1)?
                            .ok_or_else(|| freeze("argument-expression/enclosing-invoke-removed")),
                        None => Ok(binding),
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                Ok((effects, call))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut validated_calls = BTreeSet::new();
        let dynamic_packets = self
            .dynamic_calls()
            .map(|packet| {
                let original = packet.invoke_binding();
                let current = match finishing {
                    Some(projection) => projection
                        .binding(original.0, &original.1)?
                        .ok_or_else(|| freeze("dynamic-expression/invoke-removed"))?,
                    None => original.clone(),
                };
                let mut arguments = packet.argument_bindings_in_context_v1(context)?.0;
                arguments.push(original.clone());
                let effects = arguments.into_iter()
                    .filter(|(_, instruction)| matches!(instruction, MirInstruction::Invoke { .. }))
                    .map(|binding| match finishing {
                        Some(projection) => projection.binding(binding.0, &binding.1)?
                            .ok_or_else(|| freeze("dynamic-expression/argument-effect-removed")),
                        None => Ok(binding),
                    }).collect::<Result<Vec<_>, String>>()?;
                Ok((current, effects, packet))
            })
            .collect::<Result<Vec<_>, String>>()?;
        for (id, instruction) in ordered {
            let MirInstruction::Invoke {
                operation,
                fault_frame: actual,
                normal_landing,
                fault_landing,
                ..
            } = instruction
            else {
                continue;
            };
            if *actual != fault_frame {
                return Err(freeze("print-expression/frame-drift"));
            }
            if let Some((effects, call)) = calls.iter().find(|(effects, _)| {
                effects
                    .iter()
                    .any(|binding| binding == &(*id, instruction.clone()))
            }) {
                if validated_calls.insert(call.invoke_binding().0) {
                    covered.extend(
                        call.validate_taken_argument_scopes_v1(context, function, finishing)?,
                    );
                    let prefix = ExpressionCallTransientPrefixV1 {
                        origin: TransientPrefixOriginV1::Emitted {
                            frame: self.frame,
                            call,
                            live: &live,
                            prior_homes: self.frame.prior_homes(),
                        },
                        parent: inherited,
                    };
                    covered.extend(call.validate_expression_arguments_with_prefix_v1(
                        context,
                        function,
                        finishing,
                        self.original,
                        None,
                        Some(&prefix),
                    )?);
                }
                if !effects.iter().all(|binding| covered.contains(&binding.0)) {
                    return Err(freeze("argument-expression/call-effect-uncovered"));
                }
                let actual_call = match finishing {
                    Some(boundary) => boundary
                        .binding(call.invoke_binding().0, &call.invoke_binding().1)?
                        .ok_or_else(|| freeze("argument-expression/enclosing-invoke-removed"))?,
                    None => call.invoke_binding().clone(),
                };
                if actual_call == (*id, instruction.clone()) {
                    let current = self
                        .frame
                        .ownership_v1()
                        .after_site(call.call_site())
                        .ok_or_else(|| freeze("expression/argument-owner-unavailable"))?;
                    let current = inherited
                        .map(|prefix| prefix.homes_after_site_v1(call.call_site(), current))
                        .transpose()?
                        .unwrap_or(current);
                    current_homes = project_homes(current)?;
                }
                if matches!(
                    operation.normal_result_kind(),
                    Some(
                        crate::mir::instruction::InvokeNormalResultKind::OwnedString
                            | crate::mir::instruction::InvokeNormalResultKind::DynamicCarrier
                    )
                ) {
                    let original = call.invoke_binding();
                    let main = match finishing {
                        Some(projection) => projection
                            .binding(original.0, &original.1)?
                            .ok_or_else(|| {
                                freeze("argument-expression/enclosing-invoke-removed")
                            })?,
                        None => original.clone(),
                    };
                    if main == (*id, instruction.clone()) {
                        let (_, projection) = call.outer_bindings()?;
                        let MirInstruction::InvokeNormalResult { dst, .. } = projection.1 else {
                            return Err(freeze("print-expression/call-value"));
                        };
                        let temporary = match operation.normal_result_kind() {
                            Some(crate::mir::instruction::InvokeNormalResultKind::OwnedString) => {
                                ExpressionOwnedTemporaryV1::String(dst)
                            }
                            Some(
                                crate::mir::instruction::InvokeNormalResultKind::DynamicCarrier,
                            ) => ExpressionOwnedTemporaryV1::DynamicCarrier(dst),
                            _ => unreachable!("checked owned call result"),
                        };
                        if live.contains(&temporary) {
                            return Err(freeze("dynamic-expression/result-lifecycle"));
                        }
                        live.push(temporary);
                    }
                }
                continue;
            }
            if let Some((entry, _, packet)) = dynamic_packets.iter()
                .find(|(_, effects, _)| effects.contains(&(*id, instruction.clone())))
            {
                let source = self.frame.calls().iter().find(|call| call.site() == packet.source().site())
                    .ok_or_else(|| freeze("dynamic-expression/source-loan"))?;
                if validated_calls.insert(entry.0) {
                    let prefix = ExpressionCallTransientPrefixV1::from_emitted_dynamic_v1(
                        self.frame, source, &live, inherited,
                    );
                    for (ordinal, argument) in packet.argument_projections().iter().enumerate() {
                        let argument_prefix = ExpressionCallTransientPrefixV1::for_prepared_call_argument(
                            ExpressionCallArgumentSourceV1::from_local(source), ordinal,
                            &packet.argument_projections()[..ordinal], context, Some(&prefix),
                        )?;
                        covered.extend(argument.validate_expression_physical_with_prefix_v1(
                            context, function, finishing, self.original, None, Some(&argument_prefix),
                        )?);
                    }
                }
                if entry != &(*id, instruction.clone()) {
                    if !covered.contains(id) { return Err(freeze("dynamic-expression/argument-effect-uncovered")); }
                    continue;
                }
                let ends = LexicalCallArgumentProjectionV1::deferred_ends_for_arguments_v1(
                    packet.argument_projections(), context,
                )?;
                let suffix = end_sequence(&live, &current_homes)?;
                covered.extend(validate_dynamic_invoke_cleanup_v1(
                    function, finishing, packet, *id, operation, *normal_landing,
                    *fault_landing, fault_frame, &ends, &suffix,
                )?);
                let value = match finishing {
                    Some(boundary) => boundary.binding(packet.projection_binding().0, &packet.projection_binding().1)?
                        .and_then(|row| row.1.dst_value()).ok_or_else(|| freeze("dynamic-expression/result-removed"))?,
                    None => packet.value(),
                };
                let temporary = ExpressionOwnedTemporaryV1::DynamicCarrier(value);
                if live.contains(&temporary) { return Err(freeze("dynamic-expression/result-lifecycle")); }
                live.push(temporary);
                continue;
            }
            // A previously validated Call includes its Normal End sequence.
            if covered.contains(id) { continue; }
            let dynamic_result = match operation {
                InvokeOperation::DynamicArrayGet { .. } => {
                    let mut values = self.frame.cores().iter().filter_map(|(node, core)| {
                        let expected = crate::mir::normal_callable_semantic_package::checked_expression_core_operation_v1(
                            core, self.values,
                        ).ok()?;
                        if expected != std::ops::ControlFlow::Break(operation.clone()) { return None; }
                        self.values.get(*node).and_then(Option::as_ref)
                            .and_then(|value| value.as_operand().ok())
                            .and_then(|value| match value {
                                crate::mir::instruction::StringInvokeOperand::DynamicCarrier(value) => Some(*value),
                                _ => None,
                            })
                    });
                    let value = values.next();
                    if values.next().is_some() {
                        return Err(freeze("dynamic-expression/core-source-ambiguous"));
                    }
                    value
                }
                InvokeOperation::DynamicCarrierSubI64 { carrier, rhs } => {
                    let mut values = self.frame.operations().iter().filter_map(|loan| {
                        let node = loan.node();
                        let value = self
                            .values
                            .get(node)
                            .and_then(Option::as_ref)
                            .and_then(|value| value.as_operand().ok())?;
                        let crate::mir::instruction::StringInvokeOperand::DynamicCarrier(result) =
                            value
                        else {
                            return None;
                        };
                        self.dynamic_sub_original_v1(node, *result, *carrier, *rhs)
                            .ok()
                            .map(|_| *result)
                    });
                    let value = values.next();
                    if values.next().is_some() {
                        return Err(freeze("dynamic-expression/sub-source-ambiguous"));
                    }
                    value
                }
                _ => None,
            };
            if matches!(
                operation,
                InvokeOperation::DynamicArrayGet { .. }
                    | InvokeOperation::DynamicCarrierSubI64 { .. }
            ) && dynamic_result.is_none()
            {
                return Err(freeze("dynamic-expression/owned-source-missing"));
            }
            if let Some(value) = dynamic_result {
                let mut projections = ordered.iter().filter(|(_, row)| {
                    matches!(row,
                    MirInstruction::InvokeNormalResult { invoke_block, dst }
                        if *invoke_block == *id && *dst == value)
                });
                let Some((normal_block, _)) = projections.next() else {
                    return Err(freeze("dynamic-expression/core-result-projection"));
                };
                if projections.next().is_some()
                    || *normal_block != *normal_landing
                    || live.contains(&ExpressionOwnedTemporaryV1::DynamicCarrier(value))
                {
                    return Err(freeze("dynamic-expression/core-result-lifecycle"));
                }
                covered.insert(*id);
                covered.extend(paths::pending_operations(
                    function,
                    *fault_landing,
                    &end_sequence(&live, &current_homes)?,
                    Some(fault_frame),
                )?);
                live.push(ExpressionOwnedTemporaryV1::DynamicCarrier(value));
                continue;
            }
            covered.insert(*id);
            covered.extend(paths::pending_operations(
                function,
                *fault_landing,
                &end_sequence(&live, &current_homes)?,
                Some(fault_frame),
            )?);
            if matches!(operation, InvokeOperation::String(operation) if operation.owns_normal_result())
            {
                let value = ordered
                    .iter()
                    .find_map(|(_, row)| match row {
                        MirInstruction::InvokeNormalResult { invoke_block, dst }
                            if invoke_block == id =>
                        {
                            Some(*dst)
                        }
                        _ => None,
                    })
                    .ok_or_else(|| freeze("print-expression/add-result"))?;
                if live.contains(&ExpressionOwnedTemporaryV1::String(value)) || homes.iter().any(|operation| matches!(operation,
                    InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value: prior }) if *prior == value)) {
                    return Err(freeze("argument-expression/prefix-duplicate-acquisition"));
                }
                live.push(ExpressionOwnedTemporaryV1::String(value));
            }
        }
        Ok((live, covered))
    }
}

/// Independent physical validation against the common argument End sequence.
fn validate_dynamic_invoke_cleanup_v1(
    function: &MirFunction,
    finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
    packet: &EmittedDynamicInvocationCallProjectionV1,
    id: BasicBlockId,
    operation: &InvokeOperation,
    normal_landing: BasicBlockId,
    fault_landing: BasicBlockId,
    fault_frame: ValueId,
    ends: &[InvokeOperation],
    suffix: &[InvokeOperation],
) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
    use super::super::super::root_cleanup_graph::ordered_paths as paths;
    if !matches!(operation, InvokeOperation::DynamicInvocation(actual) if actual == packet.operation()) {
        return Err(freeze("dynamic-expression/invoke-operation"));
    }
    let projected = match finishing {
        Some(projection) => projection.binding(packet.projection_binding().0, &packet.projection_binding().1)?
            .ok_or_else(|| freeze("dynamic-expression/result-removed"))?,
        None => packet.projection_binding().clone(),
    };
    let MirInstruction::InvokeNormalResult { invoke_block, dst } = projected.1 else {
        return Err(freeze("dynamic-expression/result-projection"));
    };
    if invoke_block != id { return Err(freeze("dynamic-expression/result-projection")); }

    let all = ends.iter().chain(suffix).cloned().collect::<Vec<_>>();
    let mut covered = paths::pending_operations(function, fault_landing, &all, Some(fault_frame))?;
    covered.insert(id);
    let Some(continuation) = packet.cleanup_continuation() else {
        return if ends.is_empty() { Ok(covered) } else { Err(freeze("dynamic-expression/cleanup-continuation")) };
    };
    let stop = match finishing {
        Some(projection) => projection.destination(continuation)
            .ok_or_else(|| freeze("dynamic-expression/cleanup-continuation"))?,
        None => continuation,
    };
    let residuals = (1..=ends.len())
        .map(|next| {
            ends[next..].iter().cloned()
                .chain([InvokeOperation::DynamicCarrierEnd { carrier: dst }])
                .chain(suffix.iter().cloned())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    covered.extend(paths::validate_to_continuation_v2(
        function, normal_landing, stop, ends, &residuals,
        Some(fault_frame), &mut |_, operation| Ok(operation.clone()),
    )?);
    Ok(covered)
}
