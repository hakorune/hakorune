//! Independent expression terminal cleanup.
use super::*;

impl ExpressionPhysicalProjectionRefV1<'_> {
    pub(in super::super::super) fn validate_paths_to_terminal(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
        residence: Option<&Binding>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_paths_with_policy(
            context,
            function,
            finishing,
            terminal,
            continuation,
            residence,
            inherited,
            false,
            None,
            None,
            None,
        )
    }
    pub(super) fn validate_paths_with_policy(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
        residence: Option<&Binding>,
        inherited: Option<&ExpressionCallTransientPrefixV1<'_>>,
        defer_owned_argument: bool,
        displaced: Option<&InvokeOperation>,
        purpose_acquisition: Option<&[Binding; 2]>,
        forwarded_dynamic_carrier: Option<ValueId>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        let (Some(terminal), Some(continuation)) = (terminal, continuation) else {
            if terminal.is_some()
                || continuation.is_some()
                || residence.is_some()
                || defer_owned_argument
                || displaced.is_some()
                || purpose_acquisition.is_some()
                || forwarded_dynamic_carrier.is_some()
            {
                return Err(freeze("expression/inline-delimiter-drift"));
            }
            return self.validate_original_scope(
                context,
                function,
                &std::collections::BTreeSet::new(),
                &None,
                terminal,
                continuation,
                &self.ordered_bindings_in_context(context)?,
                finishing,
            );
        };
        if self.inline_range.is_some()
            || !context
                .ledger()
                .expression_requires_fault_frame_v1(self.frame)
        {
            return Err(freeze("expression/inline-delimiter-drift"));
        }
        let ledger = context.ledger();
        use super::super::super::root_cleanup_graph::ordered_paths as paths;
        self.validate_source(context)?;
        self.validate_reference_dependencies(context, function, finishing)?;
        let mapped = |original: &Binding| match finishing {
            Some(projection) => projection.binding(original.0, &original.1)?.ok_or_else(|| {
                format!(
                    "{} owner={:?} source={:?} original={original:?}",
                    freeze("print-expression/required-producer-removed"),
                    self.frame.source().owner(),
                    self.frame.source().root().expression_site()
                )
            }),
            None => Ok(original.clone()),
        };
        for read in self.path_reads {
            for original in read.original_copy_dependencies_v1()? {
                let actual = match finishing {
                    Some(projection) => projection.relation_binding(function, original)?,
                    None => Some(original.clone()),
                };
                if let Some((block, instruction)) = actual {
                    if !super::super::super::physical_boundary::check_binding(
                        function,
                        None,
                        block,
                        &instruction,
                    )? {
                        return Err(freeze("print-expression/path-copy-missing"));
                    }
                }
            }
        }
        for (_, node) in self
            .frame
            .source()
            .nodes()
            .iter()
            .enumerate()
            .filter(|(node, _)| self.node_selected(*node))
        {
            for original in ledger.checked_field_copy_dependencies_v1(
                &OwnedExprSiteV1::new(self.frame.source().owner(), node.site().clone()),
                context,
            )? {
                let actual = match finishing {
                    Some(projection) => projection.relation_binding(function, &original)?,
                    None => Some(original),
                };
                if let Some((block, instruction)) = actual {
                    if !super::super::super::physical_boundary::check_binding(
                        function,
                        None,
                        block,
                        &instruction,
                    )? {
                        return Err(freeze("print-expression/field-copy-missing"));
                    }
                }
            }
        }
        let mut original_order = self.ordered_bindings_in_context(context)?;
        if let Some(pair) = purpose_acquisition {
            original_order.extend(pair.iter().cloned());
        }
        let (mut normal, fault_frame) = match &terminal.1 {
            MirInstruction::Invoke { fault_frame, .. } => {
                original_order.push(terminal.clone());
                let (_, current) = mapped(terminal)?;
                let MirInstruction::Invoke { normal_landing, .. } = current else {
                    return Err(freeze("expression/terminal-invoke"));
                };
                (normal_landing, *fault_frame)
            }
            MirInstruction::Jump {
                target,
                edge_args: None,
            } => {
                let frame = self
                    .original
                    .iter()
                    .find_map(|(_, row)| match row {
                        MirInstruction::FaultFrameEnter { dst, .. } => Some(*dst),
                        _ => None,
                    })
                    .ok_or_else(|| freeze("expression/local-fault-frame"))?;
                let normal = match finishing {
                    Some(projection) => projection
                        .destination(*target)
                        .ok_or_else(|| freeze("expression/local-normal-target"))?,
                    None => *target,
                };
                (normal, frame)
            }
            _ => return Err(freeze("expression/terminal-shape")),
        };
        if let Some(install) = residence {
            original_order.push(install.clone());
        }
        let mut ordered = Vec::new();
        for original in &original_order {
            let current = match finishing {
                Some(boundary)
                    if matches!(
                        original.1,
                        MirInstruction::Jump {
                            edge_args: None,
                            ..
                        }
                    ) =>
                {
                    boundary.structural_jump_binding(
                        function,
                        original,
                        matches!(context, CallValidationContextV1::Finalized(_)),
                    )?
                }
                Some(boundary) if original.1.dst_value().is_some() => {
                    boundary.relation_binding(function, original)?
                }
                _ => Some(mapped(original)?),
            };
            if let Some(current) = current {
                ordered.push(current);
            }
        }
        let frame = self.require_unique(
            |row| matches!(row, MirInstruction::FaultFrameEnter { dst, .. } if *dst == fault_frame),
        )?;
        let expected_mode = if ledger.root_owner() == Some(self.frame.source().owner()) {
            crate::mir::instruction::FaultFrameMode::RootOwned
        } else {
            crate::mir::instruction::FaultFrameMode::Borrowed
        };
        if !matches!(frame.1, MirInstruction::FaultFrameEnter { mode, .. } if mode == expected_mode)
        {
            return Err(freeze("print-expression/frame-mode"));
        }
        let mut homes = inherited
            .map(|prefix| prefix.for_argument(self.frame))
            .transpose()?
            .unwrap_or_default();
        match self.prior_home_unwind {
            Some(unwind) => homes.extend_from_slice(unwind),
            None => {
                homes.extend(ledger.prior_home_unwind_for(self.frame.prior_homes())?);
                ledger.with_loop_call_text_ends_for_frame_v2(self.frame, &mut homes)?;
            }
        }
        let (mut live, mut covered) = self.validate_acquisition_paths_v1(
            context,
            function,
            finishing,
            &ordered,
            &homes,
            fault_frame,
            inherited,
        )?;
        if let Some(root_fault) = self.root_fault {
            if !ledger.expression_root_cleanup_v1(self.frame)?.is_some()
                || !live.is_empty()
                || residence.is_some()
                || defer_owned_argument
                || displaced.is_some()
                || purpose_acquisition.is_some()
                || forwarded_dynamic_carrier.is_some()
            {
                return Err(freeze("expression/residence-original-pair"));
            }
            let start = match finishing {
                Some(boundary) => boundary
                    .destination(root_fault)
                    .ok_or_else(|| freeze("expression/residence-original-pair"))?,
                None => root_fault,
            };
            let borrowed = paths::pending_operations(function, start, &homes, Some(fault_frame))?;
            covered.retain(|block| !borrowed.contains(block));
        }
        if let Some(carrier) = forwarded_dynamic_carrier {
            if live.pop() != Some(ExpressionOwnedTemporaryV1::DynamicCarrier(carrier)) {
                return Err(freeze("dynamic-expression/local-handoff-order"));
            }
        }
        if defer_owned_argument {
            let original_root = self
                .values
                .first()
                .and_then(Option::as_ref)
                .map(|value| value.as_operand())
                .transpose()?;
            let root = purpose_acquisition
                .and_then(|pair| pair[1].1.dst_value())
                .or_else(|| match original_root {
                    Some(Operand::OwnedString(value)) => Some(*value),
                    _ => None,
                })
                .ok_or_else(|| freeze("argument-expression/deferred-root"))?;
            if residence.is_some() || (purpose_acquisition.is_none() && self.frame.owned_text_root_node_v1().is_none())
                || !live.last().is_some_and(|temporary| temporary.is_string(root))
                || !matches!(self.frame.source().root(),
                    crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { .. }
                    | crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::NewArgument { .. }
                    | crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Assignment { .. }) {
                return Err(freeze("argument-expression/deferred-source-acquisition"));
            }
        }
        let installed = if let Some(original) = residence {
            let MirInstruction::StringResidenceInstall {
                value,
                replaces,
                returning: false,
            } = original.1
            else {
                return Err(freeze("expression/residence-install-shape"));
            };
            let expected_previous = match displaced {
                Some(InvokeOperation::String(
                    crate::mir::instruction::StringInvokeOperation::End { value }
                    | crate::mir::instruction::StringInvokeOperation::EndIfOwned { value },
                )) => Some(*value),
                Some(_) => return Err(freeze("expression/residence-replacement-kind")),
                None => None,
            };
            let original_root = self
                .values
                .first()
                .and_then(Option::as_ref)
                .map(|value| value.as_operand().cloned())
                .transpose()?;
            let effective = purpose_acquisition
                .and_then(|pair| pair[1].1.dst_value())
                .map(Operand::OwnedString)
                .or(original_root);
            if replaces != expected_previous
                || effective != Some(Operand::OwnedString(value))
                || live.pop() != Some(ExpressionOwnedTemporaryV1::String(value))
            {
                return Err(freeze("expression/residence-original-root"));
            }
            let actual = mapped(original)?;
            if !super::super::super::physical_boundary::check_binding(
                function, None, actual.0, &actual.1,
            )? {
                return Err(freeze("expression/residence-install-missing"));
            }
            Some(value)
        } else {
            None
        };
        let mut full: Vec<_> = live
            .iter()
            .rev()
            .copied()
            .map(ExpressionOwnedTemporaryV1::end_operation)
            .chain(installed.map(|value| ExpressionOwnedTemporaryV1::String(value).end_operation()))
            .chain(homes.iter().cloned())
            .collect();
        let mut normal_indices: Vec<_> = if defer_owned_argument {
            Vec::new()
        } else {
            (0..live.len()).collect()
        };
        let mut displaced_after = None;
        if let Some(old) = displaced {
            let index = full
                .iter()
                .position(|operation| operation == old)
                .ok_or_else(|| freeze("rebind-expression/old-home-missing"))?;
            if residence.is_none()
                || full.iter().filter(|operation| *operation == old).count() != 1
                || normal_indices.contains(&index)
            {
                return Err(freeze("rebind-expression/old-home-duplicate"));
            }
            if normal_indices.is_empty() {
                normal_indices.push(index);
            } else {
                // The installed replacement owes the displaced End before any
                // temporary's End, on Normal and on Fault alike.
                full.remove(index);
                full.insert(0, old.clone());
                normal_indices = (0..=live.len()).collect();
                let position = (index + 1)
                    .checked_sub(live.len() + 2)
                    .ok_or_else(|| freeze("rebind-expression/old-home-order"))?;
                displaced_after = Some(
                    super::super::super::displaced_replacement_fault_sequences_v2(
                        &mut full,
                        live.len() + 1,
                        position,
                    )?,
                );
            }
        }
        let stop = match finishing {
            Some(projection) => projection
                .destination(continuation.0)
                .ok_or_else(|| freeze("print-expression/continuation-removed"))?,
            None => continuation.0,
        };
        for (step, index) in normal_indices.iter().enumerate() {
            normal = paths::skip_until(function, normal, Some(stop))?;
            let (next, fault) = paths::release(function, normal, &full[*index], Some(fault_frame))?;
            covered.insert(normal);
            let residual = match displaced_after.as_ref() {
                Some(after) => after[step].clone(),
                None => super::super::super::fault_indices_after_normal_step_v2(
                    full.len(),
                    &normal_indices,
                    step,
                )?,
            }
            .into_iter()
            .map(|index| full[index].clone())
            .collect::<Vec<_>>();
            covered.extend(paths::pending_operations(
                function,
                fault,
                &residual,
                Some(fault_frame),
            )?);
            normal = next;
        }
        if paths::skip_until(function, normal, Some(stop))? != stop {
            return Err(freeze("print-expression/continue-path"));
        }
        // No recorded effect can hide behind a valid named source operation.
        for original in self.original {
            if matches!(
                original.1,
                MirInstruction::Invoke {
                    operation: InvokeOperation::String(_)
                        | InvokeOperation::Call { .. }
                        | InvokeOperation::FieldGet { .. }
                        | InvokeOperation::Map(
                            crate::mir::instruction::MapInvokeOperation::FieldHas { .. }
                        ),
                    ..
                }
            ) {
                let actual = mapped(original)?;
                if !covered.contains(&actual.0) {
                    return Err(freeze("print-expression/unowned-operation"));
                }
            }
        }
        let actual_frame = mapped(frame)?;
        if !super::super::super::physical_boundary::check_binding(
            function,
            None,
            actual_frame.0,
            &actual_frame.1,
        )? {
            return Err(freeze("print-expression/frame-definition"));
        }
        normal_order(function, &ordered)?;
        Ok(covered)
    }
}
