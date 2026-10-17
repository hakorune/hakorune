//! Borrowed physical correspondence for one checked original expression body.
//! Purpose wrappers retain source identity, terminal effects and lifetime policy.
use super::*;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::instruction::{StringInvokeOperand as Operand, StringInvokeOperation as Operation};
use crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1;
use crate::mir::resolved_semantics::ResolvedLiteralSourceV1 as Literal;
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);
#[path = "expression_reference_read.rs"]
mod reference_read;
pub(in crate::mir) use reference_read::ExpressionReferenceReadV1;
#[path = "expression_terminal_paths.rs"]
mod terminal_paths;

/// A view of the purpose's original observations, never a second packet pool.
/// Successful correspondence grants no terminal, residence or cleanup rights.
pub(in crate::mir) struct ExpressionPhysicalProjectionRefV1<'a> {
    frame: &'a CheckedExpressionEvaluationV1,
    values: &'a [Option<Operand>],
    calls: &'a [Rc<EmittedLexicalCallProjectionV1>],
    path_reads: &'a [crate::mir::builder::ExactLexicalReadV1],
    reference_reads: &'a std::collections::BTreeMap<usize, ExpressionReferenceReadV1>,
    original: &'a [Binding],
}
impl<'a> ExpressionPhysicalProjectionRefV1<'a> {
    pub(in crate::mir) fn new(
        frame: &'a CheckedExpressionEvaluationV1,
        values: &'a [Option<Operand>],
        calls: &'a [Rc<EmittedLexicalCallProjectionV1>],
        path_reads: &'a [crate::mir::builder::ExactLexicalReadV1],
        reference_reads: &'a std::collections::BTreeMap<usize, ExpressionReferenceReadV1>,
        original: &'a [Binding],
    ) -> Self { Self { frame, values, calls, path_reads, reference_reads, original } }

    fn calls(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        self.calls.iter().map(Rc::as_ref)
    }

    pub(in crate::mir) fn validate_source(&self, context: CallValidationContextV1<'_>) -> Result<(), String> {
        let ledger = context.ledger();
        let source = self.frame.source();
        if !self.frame.reference_coverage_is_exact()
            || self.values.len() != source.nodes().len() || self.calls.len() != self.frame.calls().len()
        {
            return Err(freeze("print-expression/source-coverage"));
        }
        if self.reference_reads.len() != self.frame.references().len()
            || self.reference_reads.keys().any(|node| !self.frame.references().contains_key(node)) {
            return Err(freeze("expression/reference-read-census"));
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
        for read in self.path_reads {
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
                (Node::Reference(_), Some(Operand::StaticText { utf8 })) => {
                    let Some(crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::StaticText(original)) = self.frame.references().get(&index) else {
                        return Err(freeze("expression/reference-source-kind"));
                    };
                    let observed = self.reference_reads.get(&index).ok_or_else(|| freeze("expression/reference-read-missing"))?;
                    let read = observed.read();
                    let producer = observed.definition().ok_or_else(|| freeze("expression/reference-static-producer"))?;
                    read.value_for(source.owner(), node.site().node(), original.binding()).map_err(|error| error.to_string())?;
                    let (initializer, _) = read.loan_local_initializer_v1(original.definition_binding())?;
                    if utf8 != original.utf8() || !matches!(&producer.1,
                        MirInstruction::Const { dst, value: crate::mir::ConstValue::String(text) }
                            if *dst == initializer && text == original.utf8()) {
                        return Err(freeze("expression/reference-static-value"));
                    }
                }
                (Node::Reference(_), Some(Operand::NormalInteger(value))) => {
                    let Some(crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::Scalar(source)) = self.frame.references().get(&index) else {
                        return Err(freeze("expression/reference-scalar-source"));
                    };
                    let source = source.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?;
                    let observed = self.reference_reads.get(&index).ok_or_else(|| freeze("expression/reference-read-missing"))?;
                    if observed.validate_scalar_source(source, context)? != *value { return Err(freeze("expression/reference-scalar-value")); }
                }
                (Node::Literal(Literal::Integer(expected)), Some(Operand::NormalInteger(dst))) => {
                    self.require_unique(|row| matches!(row, MirInstruction::Const { dst: actual, value: crate::mir::ConstValue::Integer(value) }
                        if actual == dst && value == expected))?;
                }
                (Node::MethodCall, Some(value)) if self.frame.cores().contains_key(&index) => {
                    let core = &self.frame.cores()[&index];
                    if !core.matches_plan(source, index) { return Err(freeze("expression-core/source-identity")); }
                    let expected = checked_expression_core_operation_v1(core, self.values)?;
                    let dst = match (value, expected.normal_result_kind()) {
                        (Operand::OwnedString(dst), Some(crate::mir::instruction::InvokeNormalResultKind::OwnedString))
                            if self.frame.owned_result_nodes().contains(&index) => *dst,
                        (Operand::NormalInteger(dst), Some(crate::mir::instruction::InvokeNormalResultKind::I64))
                            if !self.frame.owned_result_nodes().contains(&index) => *dst,
                        _ => return Err(freeze("expression-core/result-domain")),
                    };
                    let result = self.require_unique(|row| matches!(row, MirInstruction::InvokeNormalResult { dst: actual, .. } if *actual == dst))?;
                    let MirInstruction::InvokeNormalResult { invoke_block, .. } = result.1 else { unreachable!() };
                    let invoke = self.require_unique(|row| matches!(row, MirInstruction::Invoke {
                        operation: InvokeOperation::String(actual), normal_landing, .. }
                        if actual == &expected && *normal_landing == result.0))?;
                    if invoke.0 != invoke_block || invoke.0 == result.0 { return Err(freeze("expression-core/producer")); }
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
        Ok(())
    }

    /// Initializer producers precede the expression entry. They are captured
    /// dependencies, never additional instructions in its evaluation order.
    #[cfg(test)]
    pub(in crate::mir) fn validate_original_reference_dependencies(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_reference_dependencies(context, function, None)
    }

    pub(super) fn validate_reference_dependencies(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_source(context)?;
        for (&node, observed) in self.reference_reads {
            if let Some(crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::Scalar(original)) = self.frame.references().get(&node) {
                observed.validate_scalar_layout(original.original_read().ok_or_else(|| freeze("expression/reference-scalar-obligation"))?, context, function)?;
            }
        }
        if let (Some(projection), CallValidationContextV1::Finalized(_)) = (finishing, context) {
            return self.validate_finished_reference_dependencies(function, projection);
        }
        for observed in self.reference_reads.values() {
            for original in observed.dependencies()? {
                let actual = match (finishing, context) {
                    (Some(_), CallValidationContextV1::Finalized(_)) => unreachable!("final dependencies handled above"),
                    (Some(projection), CallValidationContextV1::Lowering(_)) => {
                        // validate_complete seals omission permission later.
                        // This phase proves capture only, never final survival.
                        let dst = original.1.dst_value().ok_or_else(|| freeze("expression/reference-dependency-dst"))?;
                        if projection.captured_definition(dst)? != original { return Err(freeze("expression/reference-captured-dependency")); }
                        continue;
                    }
                    (None, _) => Some(original.clone()),
                };
                if let Some((block, instruction)) = actual {
                    if !super::super::physical_boundary::check_binding(function, None, block, &instruction)? {
                        return Err(freeze("expression/reference-dependency-missing"));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_finished_reference_dependencies(
        &self, function: &MirFunction, projection: &super::super::physical_boundary::FinishedBindings,
    ) -> Result<(), String> {
        for observed in self.reference_reads.values() {
            for original in observed.dependencies()? {
                // Only the existing sealed boundary can approve an omission.
                projection.relation_binding(function, original)?;
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub(in crate::mir) fn assert_reference_finishing_for_test(&self, function: &MirFunction) {
        if self.reference_reads.is_empty() { return; }
        // Close the common evaluation's open Normal block for this structural
        // boundary test; no terminal purpose or executable input is issued.
        let mut original = function.clone();
        for block in original.blocks.values_mut() {
            if block.terminator.is_none() { block.set_terminator(MirInstruction::Return { value: None }); }
        }
        let bindings = self.original.to_vec();
        let boundary = super::super::physical_boundary::PhysicalBoundary::capture(&original, &bindings).unwrap();
        let mut projection = boundary.project(&original).unwrap();
        boundary.validate_complete(&original, &mut projection, &bindings).unwrap();
        self.validate_finished_reference_dependencies(&original, &projection).unwrap();
        for observed in self.reference_reads.values() {
            for dependency in observed.dependencies().unwrap() {
                let mut changed = original.clone();
                changed.blocks.get_mut(&dependency.0).unwrap().instructions.retain(|row| row != &dependency.1);
                assert!(self.validate_finished_reference_dependencies(&changed, &projection).is_err(),
                    "unapproved dependency removal must fail the final consumer");
            }
        }
    }

    pub(super) fn require_unique(
        &self,
        matches: impl Fn(&MirInstruction) -> bool,
    ) -> Result<&'a Binding, String> {
        let mut rows = self.original.iter().filter(|(_, row)| matches(row));
        let result = rows
            .next()
            .ok_or_else(|| freeze("print-expression/producer-missing"))?;
        if rows.next().is_some() {
            return Err(freeze("print-expression/producer-duplicate"));
        }
        Ok(result)
    }

    pub(in crate::mir) fn ordered_bindings_in_context(&self, context: CallValidationContextV1<'_>) -> Result<Vec<Binding>, String> {
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
            if self.frame.cores().contains_key(&index) {
                let value = self.values[index].as_ref().and_then(Operand::value)
                    .ok_or_else(|| freeze("expression-core/result-missing"))?;
                let result = self.require_unique(|row| row.dst_value() == Some(value))?;
                let MirInstruction::InvokeNormalResult { invoke_block, .. } = result.1 else { return Err(freeze("expression-core/projection")); };
                let invoke = self.original.iter().find(|(block, row)| *block == invoke_block && matches!(row, MirInstruction::Invoke { .. }))
                    .ok_or_else(|| freeze("expression-core/invoke"))?;
                ordered.extend([invoke.clone(), result.clone()]);
                continue;
            }
            if matches!(source.nodes()[index].source(), Node::Reference(_)) { continue; }
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
        Ok(ordered)
    }

    /// A genuine entry Jump delimits evaluation from preceding statements.
    /// Check the complete original scope before capture. Final contraction and
    /// insertion detection then belong to the existing PhysicalBoundary's
    /// complete sequence comparison, including these same structural Jumps.
    pub(super) fn validate_original_scope(
        &self,
        ledger: &OrdinaryNewClaimLedgerV1,
        function: &MirFunction,
        covered: &std::collections::BTreeSet<BasicBlockId>,
        entry_binding: &Binding,
        terminal: &Binding,
        continuation: &Binding,
        ordered: &[Binding],
    ) -> Result<(), String> {
        use std::collections::BTreeSet;
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
        if !super::super::physical_boundary::check_binding(
            function,
            None,
            entry_binding.0,
            &entry_binding.1,
        )? {
            return Err(freeze("print-expression/entry-missing"));
        }
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
        for read in self.path_reads { copies.extend_from_slice(read.original_copy_dependencies_v1()?); }
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

    pub(super) fn validate_recorded(
        &self,
        bindings: &[Binding],
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
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
        Ok(())
    }}

/// Check actual instruction order along the single Normal expression path.
/// This does not infer evaluation order from block allocation or SSA numbers.
pub(super) fn normal_order(function: &MirFunction, ordered: &[Binding]) -> Result<(), String> {
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


/// The sole typed Core-to-String-operation projection, shared by the writer
/// and both independent source consumers. It does not issue source permission.
pub(crate) fn checked_expression_core_operation_v1(
    core: &crate::mir::normal_callable_semantic_package::ExpressionCoreSourceLoanV1,
    values: &[Option<Operand>],
) -> Result<Operation, String> {
    use crate::mir::core_method_op::CoreMethodOp as Core;
    let source = core.plan();
    if !core.matches_plan(source, core.node()) || values.len() != source.nodes().len() {
        return Err(freeze("expression-core/source-identity"));
    }
    let original = core.source().source();
    let operand = |site| source.nodes().iter().position(|node| node.site() == site)
        .and_then(|node| values[node].clone()).ok_or_else(|| freeze("expression-core/operand-missing"));
    let receiver = operand(original.receiver_site())?;
    let arguments = original.arguments().iter().map(|arg| operand(arg.site())).collect::<Result<Vec<_>, _>>()?;
    let operation = match (core.source().target().row().row().op, arguments.as_slice()) {
        (Core::StringLen, []) => Operation::Length { value: receiver },
        (Core::StringSubstring, [Operand::NormalInteger(start), Operand::NormalInteger(end)]) =>
            Operation::Substring { value: receiver, start: *start, end: *end },
        (Core::StringIndexOf, [needle]) => Operation::IndexOf { value: receiver, needle: needle.clone() },
        _ => return Err(freeze("expression-core/operand-domain")),
    };
    if !operation.valid_operand_domains() || operation.owns_normal_result() != core.owns_result() {
        return Err(freeze("expression-core/operand-domain"));
    }
    Ok(operation)
}
