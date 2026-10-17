//! Original expression-to-physical correspondence in the existing prefix pool.
//! This packet never issues fields, calls, input permissions or runtime owners.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use crate::mir::instruction::{StringInvokeOperand as Operand, StringInvokeOperation as Operation};
use crate::mir::resolved_semantics::home_new_prefix::CheckedPrintExpressionFrameV1;
use crate::mir::resolved_semantics::ResolvedLiteralSourceV1 as Literal;
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(crate) struct EmittedPrintExpressionProjectionV1 {
    frame: Rc<CheckedPrintExpressionFrameV1>,
    values: Box<[Option<Operand>]>,
    calls: Box<[Rc<EmittedLexicalCallProjectionV1>]>,
    path_reads: Box<[crate::mir::builder::ExactLexicalReadV1]>,
    entry: Binding,
    print: Binding,
    continuation: Binding,
    original: Vec<Binding>,
}

#[cfg(test)]
#[path = "print_expression_projection_tests.rs"]
mod tests;

impl EmittedPrintExpressionProjectionV1 {
    pub(crate) fn frame(&self) -> &Rc<CheckedPrintExpressionFrameV1> {
        &self.frame
    }
    pub(crate) fn calls(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        self.calls.iter().map(Rc::as_ref)
    }

    /// Observations must come from the sole writer. Even a literal Print owes
    /// its exact checked Print and a real continuing edge, never a fake Call.
    pub(crate) fn new(
        frame: Rc<CheckedPrintExpressionFrameV1>,
        values: Vec<Option<Operand>>,
        calls: Vec<Rc<EmittedLexicalCallProjectionV1>>,
        path_reads: Vec<crate::mir::builder::ExactLexicalReadV1>,
        entry: Binding,
        print: Binding,
        continuation: Binding,
        original: Vec<Binding>,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
    ) -> Result<Self, String> {
        let packet = Self {
            frame,
            values: values.into_boxed_slice(),
            calls: calls.into_boxed_slice(),
            path_reads: path_reads.into_boxed_slice(),
            entry,
            print,
            continuation,
            original,
        };
        packet.validate_source(CallValidationContextV1::Lowering(ledger))?;
        packet.validate_recorded(&packet.original, None)?;
        let covered = packet.validate_paths(ledger, function, None)?;
        packet.validate_original_scope(ledger, function, &covered)?;
        Ok(packet)
    }

    fn validate_source(&self, context: CallValidationContextV1<'_>) -> Result<(), String> {
        let ledger = context.ledger();
        let source = self.frame.source();
        if self.values.len() != source.nodes().len() || self.calls.len() != self.frame.calls().len()
        {
            return Err(freeze("print-expression/source-coverage"));
        }
        let mut roots = std::collections::BTreeMap::new();
        for node in source.nodes() {
            let site = OwnedExprSiteV1::new(source.owner(), node.site().clone());
            if let Some(path) = ledger.statement_field_path_v1(&site, context)? {
                let root_site = OwnedExprSiteV1::new(source.owner(), path.root_site().clone());
                let expected = (path.root(), OwnedExprSiteV1::new(source.owner(), path.first_site().clone()));
                if roots.insert(root_site, expected.clone()).is_some_and(|prior| prior != expected) {
                    return Err(freeze("print-expression/path-root-source"));
                }
            }
        }
        if roots.len() != self.path_reads.len() { return Err(freeze("print-expression/path-root-census")); }
        let mut seen = std::collections::BTreeSet::new();
        for read in &self.path_reads {
            let (site, _, _) = read.source_value();
            let (binding, first) = roots.get(&site).ok_or_else(|| freeze("print-expression/path-root-source"))?;
            if !seen.insert(site.clone()) { return Err(freeze("print-expression/path-root-duplicate")); }
            let value = read.value_for(source.owner(), site.site().node(), *binding).map_err(|error| error.to_string())?;
            ledger.validate_statement_field_root_read_v1(first, read, context)?;
            let emitted = ledger.expression_field_read_emission_v1(first, source, context)?;
            if !matches!(emitted.1, MirInstruction::ObjectFieldGet { base, .. } if base == value) {
                return Err(freeze("print-expression/path-root-value"));
            }
        }
        for (observed, packet) in self.frame.calls().iter().zip(self.calls()) {
            if packet.call_site() != observed.site()
                || ledger.lexical_i64_call_source(packet.call_site()) != Some(observed)
                || observed.prior_homes() != self.frame.prior_homes()
            {
                return Err(freeze("print-expression/call-source"));
            }
            let index = source
                .nodes()
                .iter()
                .position(|node| node.site() == observed.site().site())
                .ok_or_else(|| freeze("print-expression/call-node"))?;
            let value = packet.value_in_context_v1(source.owner(), observed.arguments(), context)?;
            if self.values[index] != Some(Operand::NormalInteger(value)) {
                return Err(freeze("print-expression/call-value"));
            }
        }
        let find_value = |site| {
            source
                .nodes()
                .iter()
                .position(|node| node.site() == site)
                .and_then(|index| self.values[index].as_ref())
                .cloned()
                .ok_or_else(|| freeze("print-expression/operand-missing"))
        };
        for &index in self.frame.ordered_nodes() {
            let node = &source.nodes()[index];
            let value = self.values[index].as_ref();
            if self.frame.transfers_node(index) {
                if value.is_some() {
                    return Err(freeze("print-expression/transferred-value"));
                }
                continue;
            }
            match (node.source(), value) {
                (Node::Literal(Literal::String(text)), Some(Operand::StaticText { utf8 }))
                    if utf8.as_str() == text.as_ref() => {}
                (Node::Literal(Literal::Integer(expected)), Some(Operand::NormalInteger(dst))) => {
                    self.require_unique(|row| matches!(row, MirInstruction::Const { dst: actual, value: crate::mir::ConstValue::Integer(value) }
                        if actual == dst && value == expected))?;
                }
                (Node::MethodCall, Some(Operand::NormalInteger(_)))
                    if self
                        .calls()
                        .any(|packet| packet.call_site().site() == node.site()) => {}
                (
                    Node::Shape(
                        crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess {
                            ..
                        },
                    ),
                    Some(Operand::NormalInteger(dst)),
                ) => {
                    let emitted = ledger.expression_field_read_bindings_v1(
                        &OwnedExprSiteV1::new(source.owner(), node.site().clone()),
                        source, context,
                    )?;
                    if !matches!(emitted.last().map(|binding| &binding.1), Some(MirInstruction::ObjectFieldGet { dst: actual, .. }
                        | MirInstruction::InvokeNormalResult { dst: actual, .. }) if actual == dst)
                        || emitted.iter().any(|binding| !self.original.contains(binding))
                    {
                        return Err(freeze("print-expression/field-source-value"));
                    }
                }
                (Node::Binary(binary), Some(value)) => {
                    let left = find_value(binary.lhs())?;
                    let right = find_value(binary.rhs())?;
                    match (value, left, right) {
                        (
                            Operand::NormalInteger(dst),
                            Operand::NormalInteger(lhs),
                            Operand::NormalInteger(rhs),
                        ) => {
                            self.require_unique(|row| matches!(row, MirInstruction::BinOp { dst: actual, op: crate::mir::BinaryOp::Add, lhs: a, rhs: b }
                                if actual == dst && *a == lhs && *b == rhs))?;
                        }
                        (Operand::OwnedString(dst), left, right)
                            if self.frame.owned_result_nodes().contains(&index) =>
                        {
                            let projection = self.require_unique(|row| matches!(row, MirInstruction::InvokeNormalResult { dst: actual, .. } if actual == dst))?;
                            let MirInstruction::InvokeNormalResult { invoke_block, .. } =
                                projection.1
                            else {
                                unreachable!()
                            };
                            let invoke = self.original.iter().find(|(block, row)| block == &invoke_block
                                && matches!(row, MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Add { left: a, right: b }), normal_landing, .. }
                                    if a == &left && b == &right && *normal_landing == projection.0))
                                .ok_or_else(|| freeze("print-expression/add-producer"))?;
                            if !self.frame.operations().iter().any(|operation| {
                                operation.node() == index && Rc::ptr_eq(operation.source(), source)
                            }) || invoke.0 == projection.0
                            {
                                return Err(freeze("print-expression/add-source"));
                            }
                        }
                        _ => return Err(freeze("print-expression/add-values")),
                    }
                }
                // Receiver identity is owned by its field/call source relation.
                (Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }), None)
                    if self.frame.node_is_identity(index) => {
                    let site = OwnedExprSiteV1::new(source.owner(), node.site().clone());
                    if !ledger.statement_field_path_v1(&site, context)?.is_some_and(|path| path.is_intermediate()) {
                        return Err(freeze("print-expression/identity-field-source"));
                    }
                    let emitted = ledger.expression_field_read_bindings_v1(&site, source, context)?;
                    if !matches!(emitted.last().map(|binding| &binding.1), Some(MirInstruction::ObjectFieldGet { .. }))
                        || emitted.iter().any(|binding| !self.original.contains(binding)) {
                        return Err(freeze("print-expression/identity-field-binding"));
                    }
                },
                (_, None) if self.frame.node_is_identity(index) => {}
                _ => return Err(freeze("print-expression/node-value")),
            }
        }
        if !matches!((&self.print.1, self.values.first()),
            (MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Print { value }), .. }, Some(Some(root))) if value == root)
            || !matches!(self.continuation.1, MirInstruction::Jump { .. })
        {
            return Err(freeze("print-expression/print-or-continuation"));
        }
        Ok(())
    }

    fn require_unique(
        &self,
        matches: impl Fn(&MirInstruction) -> bool,
    ) -> Result<&Binding, String> {
        let mut rows = self.original.iter().filter(|(_, row)| matches(row));
        let result = rows
            .next()
            .ok_or_else(|| freeze("print-expression/producer-missing"))?;
        if rows.next().is_some() {
            return Err(freeze("print-expression/producer-duplicate"));
        }
        Ok(result)
    }

    fn ordered_bindings(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<Vec<Binding>, String> {
        self.ordered_bindings_in_context(CallValidationContextV1::Lowering(ledger))
    }

    fn ordered_bindings_in_context(&self, context: CallValidationContextV1<'_>) -> Result<Vec<Binding>, String> {
        let mut ordered = Vec::new();
        let source = self.frame.source();
        for &index in self.frame.ordered_nodes() {
            if self.frame.transfers_node(index) {
                continue;
            }
            if self.frame.node_is_identity(index) && matches!(source.nodes()[index].source(), Node::Shape(
                crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })) {
                ordered.extend(context.ledger().expression_field_read_bindings_v1(
                    &OwnedExprSiteV1::new(source.owner(), source.nodes()[index].site().clone()), source, context)?);
                continue;
            }
            match self.values[index].as_ref() {
                Some(Operand::NormalInteger(value)) => {
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
                            source.owner(), call.arguments(), context)?);
                    } else if matches!(source.nodes()[index].source(), Node::Shape(
                        crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. })) {
                        ordered.extend(context.ledger().expression_field_read_bindings_v1(
                            &OwnedExprSiteV1::new(source.owner(), source.nodes()[index].site().clone()), source, context)?);
                    } else {
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
                _ => {}
            }
        }
        ordered.push(self.print.clone());
        Ok(ordered)
    }

    /// A genuine entry Jump delimits evaluation from preceding statements.
    /// Check the complete original scope before capture. Final contraction and
    /// insertion detection then belong to the existing PhysicalBoundary's
    /// complete sequence comparison, including these same structural Jumps.
    fn validate_original_scope(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        covered: &std::collections::BTreeSet<BasicBlockId>,
    ) -> Result<(), String> {
        use std::collections::BTreeSet;
        let MirInstruction::Jump {
            target: entry,
            edge_args: None,
        } = self.entry.1
        else {
            return Err(freeze("print-expression/entry-shape"));
        };
        if entry == self.entry.0 || self.entry.0 == self.continuation.0 {
            return Err(freeze("print-expression/entry-boundary"));
        }
        if !super::super::physical_boundary::check_binding(
            function,
            None,
            self.entry.0,
            &self.entry.1,
        )? {
            return Err(freeze("print-expression/entry-missing"));
        }
        let ordered = self.ordered_bindings(ledger)?;
        let mut calls = Vec::new();
        let mut copies = Vec::new();
        for (observation, packet) in self.frame.calls().iter().zip(self.calls()) {
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
                            .filter(|binding| node.has_producer_at(node.call_site(), binding))
                            .cloned(),
                    );
                    Ok(())
                },
            )?;
        }
        for node in self.frame.source().nodes() {
            copies.extend(ledger.checked_field_copy_dependencies_v1(
                &OwnedExprSiteV1::new(self.frame.source().owner(), node.site().clone()),
                CallValidationContextV1::Lowering(ledger))?);
        }
        for read in &self.path_reads { copies.extend_from_slice(read.original_copy_dependencies_v1()?); }
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
            if id == self.continuation.0 {
                if terminal != &self.continuation.1 || !block.instructions.is_empty() {
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
        if !scope.contains(&self.print.0)
            || !scope.contains(&self.continuation.0)
            || ordered.iter().any(|(id, _)| !scope.contains(id))
            || covered.iter().any(|id| !scope.contains(id))
        {
            return Err(freeze("print-expression/scope-disconnected"));
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
                    return Err(freeze("print-expression/scope-foreign-entry"));
                }
            }
        }
        Ok(())
    }

    pub(super) fn validate_paths(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        self.validate_paths_in_context(CallValidationContextV1::Lowering(ledger), function, finishing)
    }

    pub(super) fn validate_paths_in_context(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction, finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<std::collections::BTreeSet<BasicBlockId>, String> {
        let ledger = context.ledger();
        use super::super::root_cleanup_graph::ordered_paths as paths;
        use std::collections::BTreeSet;
        self.validate_source(context)?;
        let mapped = |original: &Binding| match finishing {
            Some(projection) => projection
                .binding(original.0, &original.1)?
                .ok_or_else(|| freeze("print-expression/required-producer-removed")),
            None => Ok(original.clone()),
        };
        for read in &self.path_reads {
            for original in read.original_copy_dependencies_v1()? {
                let actual = match finishing {
                    Some(projection) => projection.relation_binding(function, original)?,
                    None => Some(original.clone()),
                };
                if let Some((block, instruction)) = actual {
                    if !super::super::physical_boundary::check_binding(function, None, block, &instruction)? {
                        return Err(freeze("print-expression/path-copy-missing"));
                    }
                }
            }
        }
        for node in self.frame.source().nodes() {
            for original in ledger.checked_field_copy_dependencies_v1(
                &OwnedExprSiteV1::new(self.frame.source().owner(), node.site().clone()), context)? {
                let actual = match finishing {
                    Some(projection) => projection.relation_binding(function, &original)?,
                    None => Some(original),
                };
                if let Some((block, instruction)) = actual {
                    if !super::super::physical_boundary::check_binding(function, None, block, &instruction)? {
                        return Err(freeze("print-expression/field-copy-missing"));
                    }
                }
            }
        }
        let ordered = self
            .ordered_bindings_in_context(context)?
            .iter()
            .map(mapped)
            .collect::<Result<Vec<_>, _>>()?;
        let (_, print) = mapped(&self.print)?;
        let MirInstruction::Invoke {
            fault_frame,
            normal_landing: mut normal,
            ..
        } = print
        else {
            return Err(freeze("print-expression/print-invoke"));
        };
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
        let homes = ledger.prior_home_unwind_for(self.frame.prior_homes())?;
        let mut live = Vec::new();
        let mut covered = BTreeSet::new();
        let end_sequence = |live: &[ValueId]| {
            live.iter()
                .rev()
                .map(|value| InvokeOperation::String(Operation::End { value: *value }))
                .chain(homes.iter().cloned())
                .collect::<Vec<_>>()
        };
        for (id, instruction) in &ordered {
            let MirInstruction::Invoke {
                operation,
                fault_frame: actual,
                fault_landing,
                ..
            } = instruction
            else {
                continue;
            };
            if *actual != fault_frame {
                return Err(freeze("print-expression/frame-drift"));
            }
            covered.insert(*id);
            covered.extend(paths::pending_operations(
                function,
                *fault_landing,
                &end_sequence(&live),
                Some(fault_frame),
            )?);
            if matches!(operation, InvokeOperation::String(Operation::Add { .. })) {
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
                live.push(value);
            }
        }
        let stop = match finishing {
            Some(projection) => projection
                .destination(self.continuation.0)
                .ok_or_else(|| freeze("print-expression/continuation-removed"))?,
            None => self.continuation.0,
        };
        while let Some(value) = live.pop() {
            normal = paths::skip_until(function, normal, Some(stop))?;
            let (next, fault) = paths::release(
                function,
                normal,
                &InvokeOperation::String(Operation::End { value }),
                Some(fault_frame),
            )?;
            covered.insert(normal);
            covered.extend(paths::pending_operations(
                function,
                fault,
                &end_sequence(&live),
                Some(fault_frame),
            )?);
            normal = next;
        }
        if paths::skip_until(function, normal, Some(stop))? != stop {
            return Err(freeze("print-expression/continue-path"));
        }
        // No recorded effect can hide behind a valid named source operation.
        for original in &self.original {
            if matches!(
                original.1,
                MirInstruction::Invoke {
                    operation: InvokeOperation::String(_) | InvokeOperation::Call { .. } | InvokeOperation::FieldGet { .. },
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
        if !super::super::physical_boundary::check_binding(
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

    pub(super) fn validate_recorded(
        &self,
        bindings: &[Binding],
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        for original in &self.original {
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
        if !self.original.contains(&self.entry)
            || !self.original.contains(&self.print)
            || !self.original.contains(&self.continuation)
        {
            return Err(freeze("print-expression/terminal-membership"));
        }
        Ok(())
    }
}

/// Check actual instruction order along the single Normal expression path.
/// This does not infer evaluation order from block allocation or SSA numbers.
fn normal_order(function: &MirFunction, ordered: &[Binding]) -> Result<(), String> {
    let mut previous = None;
    let mut closed = std::collections::BTreeSet::new();
    for (id, instruction) in ordered {
        let block = function
            .blocks
            .get(id)
            .ok_or_else(|| freeze("print-expression/ordered-block"))?;
        let mut positions = block
            .all_instructions()
            .enumerate()
            .filter(|(_, row)| *row == instruction);
        let index = positions
            .next()
            .ok_or_else(|| freeze("print-expression/ordered-instruction"))?
            .0;
        if positions.next().is_some() {
            return Err(freeze("print-expression/ordered-duplicate"));
        }
        if let Some((mut before, previous_index, previous_was_invoke)) = previous {
            if before == *id {
                if previous_index >= index {
                    return Err(freeze("print-expression/source-order"));
                }
            } else {
                let initial = before;
                if closed.contains(id) {
                    return Err(freeze("print-expression/normal-cycle"));
                }
                while before != *id {
                    if !closed.insert(before) {
                        return Err(freeze("print-expression/normal-cycle"));
                    }
                    before = match super::super::root_cleanup_graph::ordered_paths::terminal(
                        function, before,
                    )? {
                        MirInstruction::Jump {
                            target,
                            edge_args: None,
                        } => *target,
                        MirInstruction::Invoke { normal_landing, .. }
                            if before == initial && previous_was_invoke =>
                        {
                            *normal_landing
                        }
                        _ => return Err(freeze("print-expression/source-order")),
                    };
                }
            }
        }
        previous = Some((
            *id,
            index,
            matches!(instruction, MirInstruction::Invoke { .. }),
        ));
    }
    Ok(())
}
