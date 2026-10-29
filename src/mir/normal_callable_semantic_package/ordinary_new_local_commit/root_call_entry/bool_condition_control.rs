//! Source-recursive conditional reach, then an exact owned-instruction census.
//! Graph traversal checks coverage only after source/producer authority closes.
use super::*;
use crate::ast::BinaryOperator;
use std::collections::BTreeSet;

impl EmittedBoolConditionProjectionV1 {
    pub(super) fn validate_logical(
        &self,
        node: &BoolConditionNodeProjectionV1,
        binary: &crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1,
        completed: &CompletedLogicalShortCircuitV1,
        conditions: &[Rc<LocalSsaPhysicalRelationV1>; 2],
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        let is_and = match binary.operator() {
            ResolvedBinaryOperatorV1::And if completed.operator() == &BinaryOperator::And => true,
            ResolvedBinaryOperatorV1::Or if completed.operator() == &BinaryOperator::Or => false,
            _ => return Err(freeze("bool-condition/logical-operator")),
        };
        if !self.frame.logical_nodes().contains(&node.node)
            || completed.operands() != (self.value_at(binary.lhs())?, self.value_at(binary.rhs())?)
            || completed.value() != node.value
        {
            return Err(freeze("bool-condition/logical-source"));
        }
        let (lhs_branch, rhs_branch) = completed.branches();
        let (lhs_condition, rhs_condition) = completed.conditions();
        if conditions[0].values() != (completed.pinned_lhs(), lhs_condition)
            || conditions[1].values() != (completed.operands().1, rhs_condition)
            || !completed.bindings().iter().any(|(_, row)| matches!(row,
                MirInstruction::Copy { dst, src } if *dst == completed.pinned_lhs() && *src == completed.operands().0)) {
            return Err(freeze("bool-condition/logical-condition-origin"));
        }
        for relation in conditions {
            relation.verify_projected(function, &mut |binding| mapped(binding, finishing))?;
        }
        let [(skip, skip_value), (yes, yes_value), (no, no_value)] = *completed.paths();
        let lhs_targets = if is_and {
            (completed.rhs_entry(), skip)
        } else {
            (skip, completed.rhs_entry())
        };
        let find = |id, predicate: &dyn Fn(&MirInstruction) -> bool| -> Result<&Binding, String> {
            let mut rows = completed
                .bindings()
                .iter()
                .filter(|(block, row)| *block == id && predicate(row));
            let row = rows
                .next()
                .ok_or_else(|| freeze("bool-condition/control-producer"))?;
            if rows.next().is_some() {
                return Err(freeze("bool-condition/control-duplicate"));
            }
            Ok(row)
        };
        find(lhs_branch, &|row| {
            matches!(row, MirInstruction::Branch {
            condition, then_bb, else_bb, then_edge_args: None, else_edge_args: None,
        } if *condition == lhs_condition && (*then_bb, *else_bb) == lhs_targets)
        })?;
        find(rhs_branch, &|row| {
            matches!(row, MirInstruction::Branch {
            condition, then_bb, else_bb, then_edge_args: None, else_edge_args: None,
        } if *condition == rhs_condition && (*then_bb, *else_bb) == (yes, no))
        })?;
        for (block, value, expected) in [
            (skip, skip_value, !is_and),
            (yes, yes_value, true),
            (no, no_value, false),
        ] {
            find(block, &|row| {
                matches!(row, MirInstruction::Const { dst, value: crate::mir::ConstValue::Bool(actual) }
                if *dst == value && *actual == expected)
            })?;
            find(
                block,
                &|row| matches!(row, MirInstruction::Jump { target, edge_args: None } if *target == completed.merge()),
            )?;
        }
        find(completed.merge(), &|row| {
            matches!(row, MirInstruction::Phi { dst, inputs, .. }
            if *dst == node.value && inputs.iter().copied().eq(completed.paths().iter().copied()))
        })?;
        for row in completed.bindings() {
            if mapped(row, finishing)?.is_some() {
                require_binding(function, row, finishing)?;
            } else if !matches!(
                row.1,
                MirInstruction::Const { .. }
                    | MirInstruction::Copy { .. }
                    | MirInstruction::Jump {
                        edge_args: None,
                        ..
                    }
            ) {
                return Err(freeze("bool-condition/control-removed"));
            }
        }
        if finishing.is_none() {
            let child = |site| {
                self.frame
                    .source()
                    .nodes()
                    .iter()
                    .position(|row| row.site() == site)
                    .ok_or_else(|| freeze("bool-condition/control-child"))
            };
            let lhs = self.node(child(binary.lhs())?)?;
            let rhs = self.node(child(binary.rhs())?)?;
            if lhs.entry != node.entry
                || lhs.exit.block != lhs_branch
                || rhs.entry.block != completed.rhs_entry()
                || rhs.exit.block != rhs_branch
                || node.exit.block != completed.merge()
            {
                return Err(freeze("bool-condition/conditional-reach"));
            }
            let suffix = |start: BoolConditionCursorV1,
                          stop: Option<usize>,
                          terminal: bool|
             -> Result<Vec<Binding>, String> {
                let block = function
                    .blocks
                    .get(&start.block)
                    .ok_or_else(|| freeze("bool-condition/control-block"))?;
                let end = stop.unwrap_or(block.instructions.len());
                let instructions = block
                    .instructions
                    .get(start.instruction..end)
                    .ok_or_else(|| freeze("bool-condition/control-cursor"))?;
                Ok(instructions
                    .iter()
                    .chain(if terminal {
                        block.terminator.as_ref()
                    } else {
                        None
                    })
                    .cloned()
                    .map(|row| (start.block, row))
                    .collect())
            };
            for (actual, expected) in [
                (
                    suffix(lhs.exit, None, true)?,
                    completed
                        .bindings()
                        .iter()
                        .filter(|(id, _)| *id == lhs_branch)
                        .cloned()
                        .collect::<Vec<_>>(),
                ),
                (
                    suffix(rhs.exit, None, true)?,
                    completed
                        .bindings()
                        .iter()
                        .filter(|(id, row)| {
                            *id == rhs_branch
                                && (function.blocks[id]
                                    .instructions
                                    .iter()
                                    .skip(rhs.exit.instruction)
                                    .any(|actual| actual == row)
                                    || function.blocks[id].terminator.as_ref() == Some(row))
                        })
                        .cloned()
                        .collect(),
                ),
                (
                    suffix(
                        BoolConditionCursorV1 {
                            block: completed.merge(),
                            instruction: 0,
                        },
                        Some(node.exit.instruction),
                        false,
                    )?,
                    completed
                        .bindings()
                        .iter()
                        .filter(|(id, _)| *id == completed.merge())
                        .cloned()
                        .collect(),
                ),
                (
                    suffix(
                        BoolConditionCursorV1 {
                            block: completed.rhs_entry(),
                            instruction: 0,
                        },
                        Some(rhs.entry.instruction),
                        false,
                    )?,
                    completed
                        .bindings()
                        .iter()
                        .filter(|(id, row)| {
                            *id == completed.rhs_entry()
                                && function.blocks[id]
                                    .instructions
                                    .iter()
                                    .take(rhs.entry.instruction)
                                    .any(|actual| actual == row)
                        })
                        .cloned()
                        .collect(),
                ),
            ] {
                if actual != expected {
                    return Err(freeze("bool-condition/control-append-order"));
                }
            }
        }
        Ok(())
    }

    pub(super) fn validate_original_scope(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let MirInstruction::Jump {
            target: entry,
            edge_args: None,
        } = self.entry.1
        else {
            return Err(freeze("bool-condition/entry-shape"));
        };
        if entry == self.entry.0
            || self.node(0)?.entry
                != (BoolConditionCursorV1 {
                    block: entry,
                    instruction: 0,
                })
        {
            return Err(freeze("bool-condition/entry-source"));
        }
        require_binding(function, &self.entry, None)?;
        let wrapper_suffix =
            ledger.main_arguments_release_suffix_v1(self.frame.source().owner(), function)?;
        let mut allowed = self.expected_inventory(ledger, function, &wrapper_suffix)?;
        for required in &allowed {
            if !self.prefix.contains(required) {
                return Err(freeze("bool-condition/required-prefix-binding-missing"));
            }
        }
        let root_exit = self.node(0)?.exit;
        let block = function
            .blocks
            .get(&self.materialization.physical().block())
            .ok_or_else(|| freeze("bool-condition/materialization-block"))?;
        if root_exit.block != self.materialization.physical().block()
            || block
                .instructions
                .get(root_exit.instruction..)
                .is_none_or(|rows| {
                    rows.iter()
                        .cloned()
                        .map(|row| (root_exit.block, row))
                        .ne(self.materialization.physical().bindings().iter().cloned())
                })
        {
            return Err(freeze("bool-condition/root-materialization-order"));
        }
        if let Some(outer) = self.outer.borrow().as_ref() {
            allowed.push(outer.clone());
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
                .ok_or_else(|| freeze("bool-condition/scope-block"))?;
            for row in block.all_instructions() {
                if !allowed.contains(&(id, row.clone()))
                    && !wrapper_suffix.contains(&(id, row.clone()))
                {
                    return Err(format!(
                        "{} block={id:?} instruction={row:?}",
                        freeze("bool-condition/unowned-instruction")
                    ));
                }
            }
            if id == self.materialization.physical().block() {
                if let Some(outer) = self.outer.borrow().as_ref() {
                    if block.terminator.as_ref() != Some(&outer.1) {
                        return Err(freeze("bool-condition/outer-current"));
                    }
                } else if block.terminator.is_some() {
                    return Err(freeze("bool-condition/pending-terminal"));
                }
                continue;
            }
            match block
                .terminator
                .as_ref()
                .ok_or_else(|| freeze("bool-condition/scope-terminal"))?
            {
                MirInstruction::Jump { target, .. } => pending.push(*target),
                MirInstruction::Branch {
                    then_bb, else_bb, ..
                } => pending.extend([*then_bb, *else_bb]),
                MirInstruction::Invoke {
                    normal_landing,
                    fault_landing,
                    ..
                } => pending.extend([*normal_landing, *fault_landing]),
                MirInstruction::ReturnFault { .. } => {}
                _ => return Err(freeze("bool-condition/scope-escape")),
            }
        }
        if !scope.contains(&self.materialization.physical().block()) {
            return Err(freeze("bool-condition/scope-disconnected"));
        }
        for (id, block) in &function.blocks {
            if scope.contains(id) {
                continue;
            }
            for edge in block.out_edges() {
                if scope.contains(&edge.target)
                    && !(*id == self.entry.0
                        && edge.target == entry
                        && block.terminator.as_ref() == Some(&self.entry.1))
                {
                    return Err(freeze("bool-condition/foreign-entry"));
                }
            }
        }
        for node in &self.nodes {
            if !scope.contains(&node.entry.block) || !scope.contains(&node.exit.block) {
                return Err(freeze("bool-condition/node-scope"));
            }
        }
        Ok(())
    }
}

impl EmittedBoolConditionProjectionV1 {
    fn expected_inventory(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        wrapper_suffix: &[Binding],
    ) -> Result<Vec<Binding>, String> {
        use BoolConditionValueProofV1 as Proof;
        let mut allowed = vec![self.entry.clone()];
        for node in &self.nodes {
            match &node.proof {
                Proof::Literal(row) => allowed.push(row.clone()),
                Proof::Field(row) => {
                    let source = self
                        .frame
                        .source()
                        .nodes()
                        .get(node.node)
                        .ok_or_else(|| freeze("bool-condition/field-node"))?;
                    let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                        self.frame.source().owner(),
                        source.site().clone(),
                    );
                    if let Some(pair) = ledger.checked_field_pair_v1(&site,
                        crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger))? {
                        let rows = ledger.checked_field_normal_prefix_v1(&site)?.into_iter()
                            .chain(pair)
                            .chain(ledger.checked_field_cleanup_bindings_v1(&site)?);
                        for binding in rows { if !allowed.contains(&binding) { allowed.push(binding); } }
                    } else {
                        allowed.push(row.clone());
                    }
                }
                Proof::FieldArrayCount { field, read } => {
                    if !matches!(read.1, MirInstruction::ArrayRead { operation: crate::mir::instruction::ArrayReadOperationV1::Count, .. }) {
                        return Err(freeze("bool-condition/array-count-writer"));
                    }
                    allowed.push(field.clone());
                    allowed.push(read.clone());
                }
                Proof::Reference { producer, .. } => allowed.extend(producer.iter().cloned()),
                // Retain the shared producer/Copy dependency census without reissuing it.
                Proof::OriginalReference(read) => allowed.extend(read.dependencies()?),
                Proof::Binary { append, operands } => {
                    allowed.push(append.clone());
                    for relation in operands {
                        relation.append_observed_bindings_v1(&mut allowed);
                    }
                    let original = self.frame.source().nodes()[node.node]
                        .binary()
                        .ok_or_else(|| freeze("bool-condition/binary-source"))?;
                    let index = |site| {
                        self.frame
                            .source()
                            .nodes()
                            .iter()
                            .position(|node| node.site() == site)
                            .ok_or_else(|| freeze("bool-condition/binary-child"))
                    };
                    let lhs = self.node(index(original.lhs())?)?;
                    let rhs = self.node(index(original.rhs())?)?;
                    if node.entry != lhs.entry
                        || lhs.exit != rhs.entry
                        || rhs.exit.block != node.exit.block
                        || node.exit.instruction <= rhs.exit.instruction
                    {
                        return Err(freeze("bool-condition/binary-evaluation-order"));
                    }
                }
                Proof::Logical {
                    completed,
                    conditions,
                } => {
                    allowed.extend_from_slice(completed.bindings());
                    for relation in conditions {
                        relation.append_observed_bindings_v1(&mut allowed);
                    }
                }
                Proof::ScalarInvocation {
                    bindings,
                    entry,
                    terminal,
                    continuation,
                    ..
                } => {
                    let projection = self.comparison_inputs_v1(node)?;
                    let context = CallValidationContextV1::Lowering(ledger);
                    let covered = projection.validate_scalar_continuation_v1(
                        context,
                        function,
                        None,
                        entry,
                        terminal,
                        continuation,
                        None,
                    )?;
                    let mut ordered = projection.ordered_bindings_in_context(context)?;
                    ordered.push(terminal.clone());
                    projection.validate_original_scope(
                        ledger,
                        function,
                        &covered,
                        entry,
                        terminal,
                        continuation,
                        &ordered,
                    )?;
                    let MirInstruction::Jump {
                        target,
                        edge_args: None,
                    } = continuation.1
                    else {
                        return Err(freeze("bool-condition/text-evaluation-order"));
                    };
                    if node.entry.block != entry.0
                        || function
                            .blocks
                            .get(&entry.0)
                            .is_none_or(|block| node.entry.instruction != block.instructions.len())
                        || node.exit.block != target
                        || node.exit.instruction != 0
                    {
                        return Err(freeze("bool-condition/text-evaluation-order"));
                    }
                    allowed.extend_from_slice(bindings);
                }
                Proof::Call { packet, bindings } => {
                    let mut producers = Vec::new();
                    let observation = self
                        .frame
                        .calls()
                        .iter()
                        .find(|call| call.site() == packet.call_site())
                        .ok_or_else(|| freeze("bool-condition/call-observation"))?;
                    packet.visit_original_nodes_v1(
                        self.frame.source().owner(),
                        observation.arguments(),
                        ledger,
                        &mut |inner, _| {
                            // The original walk already checked read identity and
                            // its source-owned Fault chain. Keep those effects
                            // separate from the value-producer lender.
                            for (_, read) in inner.array_read_records_v1() {
                                producers.extend(read.bindings().cloned());
                            }
                            producers.extend(
                                bindings
                                    .iter()
                                    .filter(|binding| {
                                        inner.has_producer_at(inner.call_site(), binding)
                                    })
                                    .cloned(),
                            );
                            let MirInstruction::Invoke {
                                fault_frame,
                                fault_landing,
                                ..
                            } = &inner.invoke_binding().1
                            else {
                                return Err(freeze("bool-condition/call-invoke"));
                            };
                            producers.extend(self.expected_fault_inventory_v1(
                                ledger,
                                function,
                                fault_frame,
                                fault_landing,
                                wrapper_suffix,
                            )?);
                            Ok(())
                        },
                    )?;
                    producers.extend(
                        packet
                            .copy_dependencies(ledger)?
                            .into_iter()
                            .map(|(_, binding)| binding),
                    );
                    let mut current = node.entry.block;
                    let mut seen = BTreeSet::new();
                    let mut traversed = BTreeSet::new();
                    while current != node.exit.block {
                        if !seen.insert(current) {
                            return Err(freeze("bool-condition/call-normal-cycle"));
                        }
                        let terminal = function
                            .blocks
                            .get(&current)
                            .and_then(|block| block.terminator.as_ref())
                            .ok_or_else(|| freeze("bool-condition/call-normal-terminal"))?;
                        match terminal {
                            MirInstruction::Jump {
                                target,
                                edge_args: None,
                            } => {
                                producers.push((current, terminal.clone()));
                                current = *target;
                            }
                            MirInstruction::Invoke { normal_landing, .. }
                                if producers.contains(&(current, terminal.clone())) =>
                            {
                                traversed.insert(current);
                                current = *normal_landing;
                            }
                            _ => return Err(freeze("bool-condition/call-normal-escape")),
                        }
                    }
                    if !traversed.contains(&packet.invoke_binding().0) {
                        return Err(freeze("bool-condition/call-not-evaluated"));
                    }
                    for binding in bindings.iter() {
                        if !producers.contains(binding) {
                            return Err(freeze("bool-condition/unowned-call-binding"));
                        }
                    }
                    allowed.extend(producers);
                }
                Proof::ScheduledSourceLoopCall { packet, bindings } => {
                    let observation = self
                        .frame
                        .calls()
                        .iter()
                        .find(|call| call.site() == packet.call_site())
                        .ok_or_else(|| freeze("bool-condition/source-loop-call-observation"))?;
                    let effect = ledger
                        .source_loop_bool_call_effect_v2(&self.frame, observation)?
                        .ok_or_else(|| freeze("bool-condition/source-loop-call-effect"))?;
                    let original = effect
                        .lexical_rc_v1()
                        .ok_or_else(|| freeze("bool-condition/source-loop-call-packet"))?;
                    if !Rc::ptr_eq(packet, &original)
                        || effect.bindings() != bindings.as_ref()
                        || node.entry != node.exit
                    {
                        return Err(freeze("bool-condition/source-loop-call-scope"));
                    }
                }
            }
        }
        self.materialization
            .physical()
            .relation()
            .append_observed_bindings_v1(&mut allowed);
        allowed.extend_from_slice(self.materialization.physical().bindings());
        let borrowed_loop_phis =
            borrowed_source_loop_phi_relations_v2(&self.frame, &self.nodes,
                crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger), None)?;
        let allowed = crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1::without_borrowed_phis_v2(
            allowed, self.nodes.iter().filter_map(|node| match &node.proof { Proof::OriginalReference(read) => Some(read), _ => None }))?;
        let allowed = crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1::without_borrowed_source_loop_phis_v2(
            allowed, &borrowed_loop_phis)?;
        for binding in &self.prefix {
            if !allowed.contains(binding) {
                return Err(freeze("bool-condition/unowned-prefix-binding"));
            }
        }
        Ok(allowed)
    }
}

impl EmittedBoolConditionProjectionV1 {
    /// One source Home unwind inventory for both Call and String operations.
    fn expected_fault_inventory_v1(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        fault_frame: &ValueId,
        fault_landing: &BasicBlockId,
        wrapper_suffix: &[Binding],
    ) -> Result<Vec<Binding>, String> {
        let mut producers = Vec::new();
        let expected = ledger.prior_home_unwind_for(self.frame.prior_homes())?;
        let operations =
            super::super::super::super::root_cleanup_graph::ordered_paths::pending_operations(
                function,
                *fault_landing,
                &expected,
                Some(*fault_frame),
            )?;
        // Empty routing/ReturnFault nodes are structural; every
        // effectful Fault node must be in the source unwind.
        let mut pending = vec![*fault_landing];
        let mut seen = BTreeSet::new();
        while let Some(id) = pending.pop() {
            if !seen.insert(id) {
                continue;
            }
            let block = function
                .blocks
                .get(&id)
                .ok_or_else(|| freeze("bool-condition/fault-block"))?;
            if block.instructions.iter().collect::<Vec<_>>()
                != wrapper_suffix
                    .iter()
                    .filter(|(bound, _)| *bound == id)
                    .map(|(_, row)| row)
                    .collect::<Vec<_>>()
            {
                return Err(freeze("bool-condition/fault-instruction"));
            }
            let terminal = block
                .terminator
                .as_ref()
                .ok_or_else(|| freeze("bool-condition/fault-terminal"))?;
            match terminal {
                MirInstruction::Jump {
                    target,
                    edge_args: None,
                } => pending.push(*target),
                MirInstruction::Invoke {
                    normal_landing,
                    fault_landing,
                    ..
                } if operations.contains(&id) => pending.extend([*normal_landing, *fault_landing]),
                MirInstruction::ReturnFault {
                    fault_frame: actual,
                } if actual == fault_frame => {}
                _ => return Err(freeze("bool-condition/fault-source-escape")),
            }
            producers.push((id, terminal.clone()));
        }
        let mut frames = function.blocks.iter().flat_map(|(id, block)| {
            block.instructions.iter().filter_map(move |row| {
                matches!(row,
                MirInstruction::FaultFrameEnter { dst, .. } if dst == fault_frame)
                .then(|| (*id, row.clone()))
            })
        });
        let frame = frames
            .next()
            .ok_or_else(|| freeze("bool-condition/fault-frame"))?;
        if frames.next().is_some() {
            return Err(freeze("bool-condition/fault-frame-duplicate"));
        }
        let mode = if ledger.root_owner() == Some(self.frame.source().owner()) {
            crate::mir::instruction::FaultFrameMode::RootOwned
        } else {
            crate::mir::instruction::FaultFrameMode::Borrowed
        };
        if !matches!(frame.1, MirInstruction::FaultFrameEnter { mode: actual, .. } if actual == mode)
        {
            return Err(freeze("bool-condition/fault-frame-mode"));
        }
        producers.push(frame);
        Ok(producers)
    }
}
