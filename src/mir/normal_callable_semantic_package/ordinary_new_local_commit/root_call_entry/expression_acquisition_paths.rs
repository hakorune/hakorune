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
        let end_sequence = |live: &[ExpressionOwnedTemporaryV1]| {
            live.iter()
                .rev()
                .copied()
                .map(ExpressionOwnedTemporaryV1::end_operation)
                .chain(homes.iter().cloned())
                .collect::<Vec<_>>()
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
                Ok((current, packet))
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
                    let prefix = ExpressionCallTransientPrefixV1 {
                        origin: TransientPrefixOriginV1::Emitted {
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
            if let Some((_, packet)) = dynamic_packets
                .iter()
                .find(|(binding, _)| binding == &(*id, instruction.clone()))
            {
                if !matches!(operation, InvokeOperation::DynamicInvocation(actual) if actual == packet.operation())
                {
                    return Err(freeze("dynamic-expression/invoke-operation"));
                }
                let projected = match finishing {
                    Some(projection) => projection
                        .binding(
                            packet.projection_binding().0,
                            &packet.projection_binding().1,
                        )?
                        .ok_or_else(|| freeze("dynamic-expression/result-removed"))?,
                    None => packet.projection_binding().clone(),
                };
                let MirInstruction::InvokeNormalResult { invoke_block, dst } = projected.1 else {
                    return Err(freeze("dynamic-expression/result-projection"));
                };
                if invoke_block != *id
                    || live.contains(&ExpressionOwnedTemporaryV1::DynamicCarrier(dst))
                {
                    return Err(freeze("dynamic-expression/result-lifecycle"));
                }
                covered.insert(*id);
                covered.extend(paths::pending_operations(
                    function,
                    *fault_landing,
                    &end_sequence(&live),
                    Some(fault_frame),
                )?);
                live.push(ExpressionOwnedTemporaryV1::DynamicCarrier(dst));
                continue;
            }
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
                    &end_sequence(&live),
                    Some(fault_frame),
                )?);
                live.push(ExpressionOwnedTemporaryV1::DynamicCarrier(value));
                continue;
            }
            covered.insert(*id);
            covered.extend(paths::pending_operations(
                function,
                *fault_landing,
                &end_sequence(&live),
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
