//! Normal-path ordering, complete scope and recorded finishing correspondence.
use super::*;

impl<'a> ExpressionPhysicalProjectionRefV1<'a> {
    /// Validate an inline source interval, including read copies and empty
    /// evaluations; final rewriting still uses PhysicalBoundary's whole block.
    pub(super) fn validate_inline_scope_v1(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        let ledger = context.ledger();
        if ledger.expression_requires_fault_frame_v1(self.frame) {
            return Err(freeze("expression/fault-frame-required"));
        }
        self.validate_source(context)?;
        self.validate_reference_dependencies(context, function, finishing)?;
        let ordered = self.ordered_bindings_in_context(context)?;
        let mut copies = self.scalar_copy_dependencies_v1(context)?;
        for read in self.path_reads {
            copies.extend_from_slice(read.original_copy_dependencies_v1()?);
        }
        for read in self.reference_reads.values() {
            copies.extend(read.dependencies()?);
        }
        if self.original.iter().any(|row| !ordered.contains(row) && !copies.contains(row)) {
            return Err(freeze("print-expression/unowned-scope-instruction"));
        }
        let (block, start, end, had_entry_frame) = self.inline_range
            .ok_or_else(|| freeze("expression/inline-range-missing"))?;
        let mut actual_order = Vec::new();
        for original in &ordered {
            match finishing {
                Some(boundary) => {
                    if let Some(actual) = boundary.binding(original.0, &original.1)? {
                        actual_order.push(actual);
                    }
                }
                None => actual_order.push(original.clone()),
            }
        }
        normal_order(function, &actual_order)?;
        if let Some(boundary) = finishing {
            if matches!(context, CallValidationContextV1::Finalized(_)) {
                boundary.validate_finished_sequences_v1(function)?;
            }
        }
        if finishing.is_none() {
            let body = function.blocks.get(&block)
                .ok_or_else(|| freeze("print-expression/scope-block"))?;
            // Only the shared function owner may insert its one frame at
            // the canonical entry position after this interval was captured.
            // This relocates coordinates; source laws above decide effects.
            let inserted_frame = if block == function.entry_block {
                let expected_mode = if ledger.root_owner() == Some(self.frame.source().owner()) {
                    crate::mir::instruction::FaultFrameMode::RootOwned
                } else { crate::mir::instruction::FaultFrameMode::Borrowed };
                let frames: Vec<_> = function.blocks.iter().flat_map(|(id, block)|
                    block.instructions.iter().enumerate().filter_map(move |(index, row)|
                        matches!(row, MirInstruction::FaultFrameEnter { .. }).then_some((*id, index, row))))
                    .collect();
                let has_frame = match frames.as_slice() {
                    [] => false,
                    [(id, index, MirInstruction::FaultFrameEnter { mode, .. })]
                        if *id == block && *mode == expected_mode
                            && body.instructions[..*index].iter().all(|row| matches!(row, MirInstruction::Phi { .. })) => true,
                    _ => return Err(freeze("print-expression/frame-mode")),
                };
                if had_entry_frame && !has_frame { return Err(freeze("expression/inline-range-drift")); }
                usize::from(has_frame && !had_entry_frame)
            } else { 0 };
            let scope = body.instructions.get(start + inserted_frame..end + inserted_frame)
                .ok_or_else(|| freeze("expression/inline-range-drift"))?;
            for instruction in scope {
                let row = (block, instruction.clone());
                if !self.original.contains(&row) && !copies.contains(&row) {
                    return Err(freeze("print-expression/unowned-scope-instruction"));
                }
            }
            if self.original.iter().any(|(id, instruction)| *id != block
                || scope.iter().filter(|actual| *actual == instruction).count() != 1) {
                return Err(freeze("print-expression/scope-disconnected"));
            }
        }
        let actual_block = match finishing {
            Some(boundary) => boundary.destination(block)
                .ok_or_else(|| freeze("print-expression/scope-disconnected"))?,
            None => block,
        };
        Ok(std::collections::BTreeSet::from([actual_block]))
    }

    pub(in crate::mir) fn ordered_bindings_in_context(
        &self,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<Binding>, String> {
        let mut ordered = Vec::new();
        let source = self.frame.source();
        for &index in self
            .frame
            .ordered_nodes()
            .iter()
            .filter(|node| self.node_selected(**node))
        {
            if self.frame.transfers_node(index) && !self.frame.direct_array_receiver_v1(index)
                .is_some_and(|core| self.node_selected(core.node())) {
                continue;
            }
            if self.frame.node_is_identity(index)
                && matches!(
                    source.nodes()[index].source(),
                    Node::Shape(
                        crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }
                    )
                )
            {
                ordered.extend(context.ledger().expression_field_read_bindings_v1(
                    &OwnedExprSiteV1::new(source.owner(), source.nodes()[index].site().clone()),
                    source,
                    context,
                )?);
                continue;
            }
            if self.frame.cores().contains_key(&index) {
                let value = self.values[index]
                    .as_ref()
                    .and_then(EvaluatedExpressionValueV1::value)
                    .ok_or_else(|| freeze("expression-core/result-missing"))?;
                let result = self.require_unique(|row| row.dst_value() == Some(value))?;
                if self.frame.cores()[&index].array_count_receiver_v1(self.values)?.is_some() {
                    ordered.push(result.clone());
                    continue;
                }
                let MirInstruction::InvokeNormalResult { invoke_block, .. } = result.1 else {
                    return Err(freeze("expression-core/projection"));
                };
                let invoke = self
                    .original
                    .iter()
                    .find(|(block, row)| {
                        *block == invoke_block && matches!(row, MirInstruction::Invoke { .. })
                    })
                    .ok_or_else(|| freeze("expression-core/invoke"))?;
                ordered.extend([invoke.clone(), result.clone()]);
                continue;
            }
            if source.transparent_tail_node_v1(index).is_some() {
                continue;
            }
            if matches!(source.nodes()[index].source(), Node::Reference(_)) {
                continue;
            }
            if let Some(packet) = self
                .calls()
                .find(|packet| packet.call_site().site() == source.nodes()[index].site())
            {
                let call = self
                    .frame
                    .calls()
                    .iter()
                    .find(|call| call.site() == packet.call_site())
                    .ok_or_else(|| freeze("print-expression/call-source"))?;
                ordered.extend(packet.ordered_call_bindings_in_context_v1(
                    source.owner(),
                    call.arguments(),
                    context,
                )?);
                continue;
            }
            if let Some(packet) = self
                .dynamic_calls()
                .find(|packet| packet.source().site().site() == source.nodes()[index].site())
            {
                ordered.extend(
                    packet
                        .scope_bindings_in_context_v1(context)?
                        .0,
                );
                continue;
            }
            if let Some(EvaluatedExpressionValueV1::Bool(value)) = self.values[index].as_ref() {
                if source.nodes()[index].binary().is_some() {
                    ordered.extend(scalar_condition_bindings_v1(self.frame, index, self.values, None, *value, self.original)?);
                } else {
                    ordered.push(self.require_unique(|row| row.dst_value() == Some(*value))?.clone());
                }
                continue;
            }
            match self.values[index]
                .as_ref()
                .map(|value| value.as_operand())
                .transpose()?
            {
                Some(Operand::NormalInteger(value)) => {
                    if matches!(
                        source.nodes()[index].source(),
                        Node::Shape(
                            crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }
                        )
                    ) {
                        ordered.extend(context.ledger().expression_field_read_bindings_v1(
                            &OwnedExprSiteV1::new(
                                source.owner(),
                                source.nodes()[index].site().clone(),
                            ),
                            source,
                            context,
                        )?);
                    } else {
                        let scalar = self
                            .frame
                            .operations()
                            .iter()
                            .find(|loan| loan.node() == index)
                            .and_then(|loan| loan.scalar_source_v1());
                        if let Some(scalar) = scalar {
                            let original = context.ledger().scalar_operation_original_v1(scalar)?;
                            for relation in original.operand_relations() {
                                relation.append_materialization_chain_v1(&mut ordered);
                            }
                        }
                        ordered.push(
                            self.require_unique(|row| row.dst_value() == Some(*value))?
                                .clone(),
                        );
                    }
                }
                Some(Operand::OwnedString(value)) => {
                    let result = self.require_unique(|row| row.dst_value() == Some(*value))?;
                    let MirInstruction::InvokeNormalResult { invoke_block, .. } = result.1 else {
                        return Err(freeze("print-expression/add-result"));
                    };
                    let invoke = self
                        .original
                        .iter()
                        .find(|(id, row)| {
                            *id == invoke_block && matches!(row, MirInstruction::Invoke { .. })
                        })
                        .ok_or_else(|| freeze("print-expression/add-invoke"))?;
                    ordered.extend([invoke.clone(), result.clone()]);
                }
                Some(Operand::Dynamic(value)) => {
                    ordered.push(self.dynamic_add_original_v1(index, *value)?);
                }
                Some(Operand::DynamicCarrier(value)) => {
                    let node = self.frame.source().nodes().get(index)
                        .ok_or_else(|| freeze("print-expression/dynamic-sub-node"))?;
                    let Node::Binary(binary) = node.source() else {
                        return Err(freeze("print-expression/dynamic-sub-node"));
                    };
                    let operand = |site| -> Result<Operand, String> {
                        let value = self.frame.source().nodes().iter()
                            .position(|node| node.site() == site)
                            .and_then(|node| self.values[node].as_ref())
                            .ok_or_else(|| freeze("print-expression/dynamic-sub-operand"))?;
                        Ok(value.as_operand()?.clone())
                    };
                    let (Operand::Dynamic(carrier), Operand::NormalInteger(rhs)) =
                        (operand(binary.lhs())?, operand(binary.rhs())?)
                    else { return Err(freeze("print-expression/dynamic-sub-operands")); };
                    ordered.extend(self.dynamic_sub_original_v1(index, *value, carrier, rhs)?);
                }
                _ => {}
            }
        }
        if let Some(pair) = self.comparison_pair()? {
            ordered.extend(pair);
        }
        Ok(ordered)
    }

    /// A genuine entry Jump delimits evaluation from preceding statements.
    /// Check the complete original scope before capture. Final contraction and
    /// insertion detection then belong to the existing PhysicalBoundary's
    /// complete sequence comparison, including these same structural Jumps.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_original_scope(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        covered: &std::collections::BTreeSet<BasicBlockId>,
        entry_binding: &Option<Binding>,
        terminal: &Option<Binding>,
        continuation: &Option<Binding>,
        ordered: &[Binding],
    ) -> Result<(), String> {
        use std::collections::BTreeSet;
        let (Some(entry_binding), Some(terminal), Some(continuation)) =
            (entry_binding, terminal, continuation) else {
                if entry_binding.is_some() || terminal.is_some() || continuation.is_some() {
                    return Err(freeze("expression/inline-delimiter-drift"));
                }
                self.validate_inline_scope_v1(CallValidationContextV1::Lowering(ledger), function, None)?;
                return Ok(());
            };
        if self.inline_range.is_some() || !ledger.expression_requires_fault_frame_v1(self.frame) {
            return Err(freeze("expression/inline-delimiter-drift"));
        }
        let MirInstruction::Jump {
            target: entry,
            edge_args: None,
        } = entry_binding.1
        else {
            return Err(freeze("print-expression/entry-shape"));
        };
        if entry == entry_binding.0 || entry_binding.0 == continuation.0 {
            return Err(freeze("print-expression/entry-boundary"));
        }
        if !super::super::super::super::physical_boundary::check_binding(
            function,
            None,
            entry_binding.0,
            &entry_binding.1,
        )? {
            return Err(freeze("print-expression/entry-missing"));
        }
        let mut calls = Vec::new();
        let mut copies = Vec::new();
        for (observation, packet) in self.source_calls().zip(self.calls()) {
            copies.extend(
                packet
                    .copy_dependencies(ledger)?
                    .into_iter()
                    .map(|(_, binding)| binding),
            );
            packet.visit_original_nodes_v1(
                self.frame.source().owner(),
                observation.arguments(),
                ledger,
                &mut |node, _| {
                    calls.extend(
                        self.original
                            .iter()
                            .filter(|binding| {
                                node.has_original_scope_binding_at(node.call_site(), binding)
                            })
                            .cloned(),
                    );
                    Ok(())
                },
            )?;
        }
        for (_, node) in self
            .frame
            .source()
            .nodes()
            .iter()
            .enumerate()
            .filter(|(node, _)| self.node_selected(*node))
        {
            copies.extend(ledger.checked_field_copy_dependencies_v1(
                &OwnedExprSiteV1::new(self.frame.source().owner(), node.site().clone()),
                CallValidationContextV1::Lowering(ledger),
            )?);
        }
        copies.extend(self.scalar_copy_dependencies_v1(CallValidationContextV1::Lowering(ledger))?);
        for packet in self.dynamic_calls() {
            copies.extend(
                packet
                    .scope_bindings_in_context_v1(CallValidationContextV1::Lowering(ledger))?
                    .1,
            );
        }
        for read in self.path_reads {
            copies.extend_from_slice(read.original_copy_dependencies_v1()?);
        }
        for read in self.reference_reads.values() {
            copies.extend(read.dependencies()?);
        }
        let mut scope = BTreeSet::new();
        let mut pending = vec![entry];
        while let Some(id) = pending.pop() {
            if !scope.insert(id) {
                continue;
            }
            let block = function
                .blocks
                .get(&id)
                .ok_or_else(|| freeze("print-expression/scope-block"))?;
            for instruction in block.all_instructions() {
                let binding = (id, instruction.clone());
                let structural = matches!(
                    instruction,
                    MirInstruction::Jump {
                        edge_args: None,
                        ..
                    } | MirInstruction::ReturnFault { .. }
                );
                let approved = structural
                    || ordered.contains(&binding)
                    || copies.contains(&binding)
                    || calls.contains(&binding)
                    || covered.contains(&id)
                        && matches!(instruction, MirInstruction::Invoke { .. });
                if !approved || !self.original.contains(&binding) && !copies.contains(&binding) {
                    return Err(freeze("print-expression/unowned-scope-instruction"));
                }
            }
            let terminal = block
                .terminator
                .as_ref()
                .ok_or_else(|| freeze("print-expression/scope-terminal"))?;
            if id == continuation.0 {
                if terminal != &continuation.1 || !block.instructions.is_empty() {
                    return Err(freeze("print-expression/continuation-body"));
                }
                continue;
            }
            match terminal {
                MirInstruction::Jump {
                    target,
                    edge_args: None,
                } => pending.push(*target),
                MirInstruction::Invoke {
                    normal_landing,
                    fault_landing,
                    ..
                } => pending.extend([*normal_landing, *fault_landing]),
                MirInstruction::ReturnFault { .. } => {}
                _ => return Err(freeze("print-expression/scope-escape")),
            }
        }
        if !scope.contains(&terminal.0)
            || !scope.contains(&continuation.0)
            || ordered.iter().any(|(id, _)| !scope.contains(id))
            || covered.iter().any(|id| !scope.contains(id))
        {
            return Err(freeze("print-expression/scope-disconnected"));
        }
        if let Some(pair) = self.comparison_pair()? {
            let MirInstruction::Invoke { fault_frame, .. } = pair[0].1 else {
                unreachable!()
            };
            let frame = self.require_unique(|row| matches!(row, MirInstruction::FaultFrameEnter { dst, .. } if *dst == fault_frame))?;
            if self
                .original
                .iter()
                .any(|row| !scope.contains(&row.0) && row != entry_binding && row != frame)
            {
                return Err(freeze("print-expression/unowned-scope-instruction"));
            }
        }
        for (id, block) in &function.blocks {
            if scope.contains(id) {
                continue;
            }
            for edge in block.out_edges() {
                if scope.contains(&edge.target)
                    && !(*id == entry_binding.0
                        && edge.target == entry
                        && block.terminator.as_ref() == Some(&entry_binding.1))
                {
                    return Err(freeze("print-expression/scope-foreign-entry"));
                }
            }
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_recorded(
        &self,
        bindings: &[Binding],
        finishing: Option<&super::super::super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        for original in self.original {
            let mapped = match finishing {
                Some(projection) => projection.binding(original.0, &original.1)?,
                None => Some(original.clone()),
            };
            // Constants may be removed only by the existing captured boundary.
            if let Some(mapped) = mapped {
                if bindings
                    .iter()
                    .filter(|binding| **binding == mapped)
                    .count()
                    != 1
                {
                    return Err(freeze("print-expression/binding-membership"));
                }
            } else if !matches!(
                original.1,
                MirInstruction::Const { .. }
                    | MirInstruction::Jump {
                        edge_args: None,
                        ..
                    }
            ) {
                return Err(freeze("print-expression/required-binding-removed"));
            }
        }
        for packet in self.calls() {
            packet.validate_recorded_projected(bindings, finishing)?;
        }
        for packet in self.dynamic_calls() {
            packet.validate_recorded_projected_v1(bindings, finishing)?;
        }
        Ok(())
    }
}
