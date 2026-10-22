//! Invocation-scoped observations of the existing recursive expression writers.
//! No alternate evaluator, source tree, Call registry or cleanup algorithm.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    BoolConditionCursorV1 as Cursor, BoolConditionNodeProjectionV1 as Node,
    BoolConditionValueProofV1 as Proof, BoolConditionSealV1, EmittedBoolConditionProjectionV1,
};
use crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::builder::if_form::{IfConditionMaterializationV1, IfFormPhysicalObserverV1};
use std::collections::BTreeMap;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);

pub(in crate::mir::builder) struct BoolConditionEmissionV1 {
    pub(in crate::mir::builder) frame: Rc<CheckedBoolConditionFrameV1>,
    pub(in crate::mir::builder) nodes: Vec<Node>,
    pub(in crate::mir::builder) binary: BTreeMap<SourceExprSiteV1, Binding>,
    pub(in crate::mir::builder) logical: BTreeMap<SourceExprSiteV1, crate::mir::builder::ops::CompletedLogicalShortCircuitV1>,
    entry: Binding,
}
impl BoolConditionEmissionV1 {
    pub(in crate::mir::builder) fn begin(builder: &mut MirBuilder, frame: Rc<CheckedBoolConditionFrameV1>) -> Result<Self, String> {
        let from = builder.function_state.current_block.ok_or_else(|| freeze("bool-condition/no-entry"))?;
        let entry = builder.next_block_id();
        crate::mir::builder::emission::branch::emit_jump(builder, entry)?;
        let binding = (from, builder.function_state.current_function.as_ref().and_then(|function|
            function.blocks.get(&from)).and_then(|block| block.terminator.clone())
            .ok_or_else(|| freeze("bool-condition/entry-not-emitted"))?);
        builder.start_new_block(entry)?;
        Ok(Self { frame, nodes: Vec::new(), binary: BTreeMap::new(), logical: BTreeMap::new(), entry: binding })
    }
    pub(in crate::mir::builder) fn value_at(&self, site: &SourceExprSiteV1) -> Result<ValueId, String> {
        let index = self.frame.source().nodes().iter().position(|node| node.site() == site)
            .ok_or_else(|| freeze("bool-condition/child-source"))?;
        self.nodes.iter().find(|node| node.node == index).map(|node| node.value)
            .ok_or_else(|| freeze("bool-condition/child-not-emitted"))
    }
    pub(in crate::mir::builder) fn finish(self, value: ValueId, ledger: Rc<OrdinaryNewClaimLedgerV1>) -> Result<LoweredBoolConditionV1, String> {
        if !self.binary.is_empty() || !self.logical.is_empty() { return Err(freeze("bool-condition/unconsumed-writer-observation")); }
        Ok(LoweredBoolConditionV1 { value, ledger, frame: self.frame,
            nodes: Some(self.nodes), entry: self.entry, seal: None })
    }
}

pub(in crate::mir::builder) struct LoweredBoolConditionV1 {
    value: ValueId,
    ledger: Rc<OrdinaryNewClaimLedgerV1>,
    frame: Rc<CheckedBoolConditionFrameV1>,
    nodes: Option<Vec<Node>>,
    entry: Binding,
    seal: Option<BoolConditionSealV1>,
}
impl LoweredBoolConditionV1 {
    pub(in crate::mir::builder) fn value(&self) -> ValueId { self.value }
    pub(in crate::mir::builder) fn observer(&mut self) -> &mut dyn IfFormPhysicalObserverV1 { self }
}
impl LoweredBoolConditionV1 {
    fn take_projection_v2(&mut self, function: &MirFunction,
        physical: &crate::mir::builder::ConditionMaterializationV1,
        expected_product: Option<&Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
    ) -> Result<(Vec<Node>, Vec<Binding>), String> {
        let nodes = self.nodes.take().ok_or_else(|| freeze("bool-condition/duplicate-condition-materialization"))?;
        let mut prefix = vec![self.entry.clone()];
        for node in &nodes {
            match &node.proof {
                Proof::Literal(binding) => prefix.push(binding.clone()),
                Proof::Field(binding) => {
                    let source = self.frame.source().nodes().get(node.node)
                        .ok_or_else(|| freeze("bool-condition/field-node"))?;
                    let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                        self.frame.source().owner(), source.site().clone());
                    if let Some(pair) = self.ledger.checked_field_pair_v1(&site,
                        crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(&self.ledger))? {
                        let rows = self.ledger.checked_field_normal_prefix_v1(&site)?.into_iter()
                            .chain(pair)
                            .chain(self.ledger.checked_field_cleanup_bindings_v1(&site)?);
                        for row in rows { if !prefix.contains(&row) { prefix.push(row); } }
                    } else {
                        prefix.push(binding.clone());
                    }
                }
                Proof::Reference { producer, .. } => prefix.extend(producer.iter().cloned()),
                // Retain the shared producer/Copy dependency census without reissuing it.
                Proof::OriginalReference(read) => prefix.extend(read.dependencies()?),
                Proof::Call { packet, bindings } => {
                    prefix.extend(bindings.iter().cloned());
                    let MirInstruction::Invoke { fault_frame, .. } = packet.invoke_binding().1
                        else { return Err(freeze("bool-condition/call-frame")) };
                    prefix.push(definition(function, fault_frame)?.ok_or_else(|| freeze("bool-condition/frame-definition"))?);
                },
                // The enclosing source Loop emitted this CallSlot before the
                // condition frame. Keep the packet as a value proof without
                // importing its effect bindings into the condition prefix.
                Proof::ScheduledSourceLoopCall { .. } => {},
                Proof::Binary { append, operands } => {
                    prefix.push(append.clone());
                    for relation in operands { relation.append_observed_bindings_v1(&mut prefix); }
                },
                Proof::ScalarInvocation { bindings, .. } => prefix.extend_from_slice(bindings),
                Proof::Logical { completed, conditions } => {
                    prefix.extend_from_slice(completed.bindings());
                    for relation in conditions { relation.append_observed_bindings_v1(&mut prefix); }
                },
            }
        }
        prefix.extend_from_slice(physical.bindings());
        physical.relation().append_observed_bindings_v1(&mut prefix);
        let borrowed_reads: Vec<_> = nodes.iter().flat_map(|node| match &node.proof {
            Proof::OriginalReference(read) => vec![read],
            Proof::ScalarInvocation { input_reads, .. } => input_reads.values().collect(),
            _ => Vec::new(),
        }).collect();
        let borrowed_loop_phis = crate::mir::normal_callable_semantic_package::borrowed_source_loop_phi_relations_v2(
            &self.frame, &nodes, &self.ledger, expected_product)?;
        let prefix = crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1::without_borrowed_phis_v2(
            prefix, borrowed_reads.into_iter())?;
        let prefix = crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1::without_borrowed_source_loop_phis_v2(
            prefix, &borrowed_loop_phis)?;
        let mut unique = Vec::new();
        for binding in prefix { if !unique.contains(&binding) { unique.push(binding); } }
        Ok((nodes, unique))
    }
    fn publish_projection_v2(&mut self, packet: Rc<EmittedBoolConditionProjectionV1>, seal: BoolConditionSealV1) -> Result<(), String> {
        self.ledger.prepare_bool_condition_bindings_v1(packet)?.commit()?;
        self.seal = Some(seal);
        Ok(())
    }
    pub(in crate::mir::builder) fn prepare_source_loop_branch_v2(&mut self, function: &MirFunction,
        product: Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>, index: usize,
        materialization: Rc<crate::mir::builder::ConditionMaterializationV1>, targets: (BasicBlockId, BasicBlockId),
    ) -> Result<(), String> {
        let (nodes, prefix) = self.take_projection_v2(function, &materialization, Some(&product))?;
        let (packet, seal) = EmittedBoolConditionProjectionV1::prepare_source_loop_v2(Rc::clone(&self.frame), nodes,
            self.entry.clone(), prefix, materialization, product, index, targets, &self.ledger, function)?;
        self.publish_projection_v2(packet, seal)
    }
    pub(in crate::mir::builder) fn seal_source_loop_branch_v2(&mut self, function: &MirFunction,
        materialization: &Rc<crate::mir::builder::ConditionMaterializationV1>, branch: Binding,
    ) -> Result<(), String> {
        self.seal.take().ok_or_else(|| freeze("bool-condition/outer-writer-lease-missing"))?
            .seal_source_loop_v2(function, materialization, branch)
    }
}
impl IfFormPhysicalObserverV1 for LoweredBoolConditionV1 {
    fn before_branches(&mut self, function: &MirFunction, pending: Rc<IfConditionMaterializationV1>) -> Result<(), String> {
        let (nodes, prefix) = self.take_projection_v2(function, pending.materialization(), None)?;
        let (packet, seal) = EmittedBoolConditionProjectionV1::prepare(Rc::clone(&self.frame), nodes,
            self.entry.clone(), prefix, pending, &self.ledger, function)?;
        self.publish_projection_v2(packet, seal)
    }
    fn emitted_branch(&mut self, function: &MirFunction, pending: &Rc<IfConditionMaterializationV1>, branch: Binding) -> Result<(), String> {
        self.seal.take().ok_or_else(|| freeze("bool-condition/outer-writer-lease-missing"))?
            .seal(function, pending, branch)
    }
}

pub(in crate::mir::builder) fn cursor(builder: &MirBuilder) -> Result<Cursor, String> {
    let block = builder.function_state.current_block.ok_or_else(|| freeze("bool-condition/no-block"))?;
    let body = builder.function_state.current_function.as_ref().and_then(|function| function.blocks.get(&block))
        .ok_or_else(|| freeze("bool-condition/no-function"))?;
    if body.terminator.is_some() { return Err(freeze("bool-condition/closed-expression-block")); }
    Ok(Cursor { block, instruction: body.instructions.len() })
}

/// The SAME scoped scalar terminal writer serves Text comparison and field
/// presence. Source reach determines when inputs run; cleanup remains shared.
pub(in crate::mir::builder) fn emit_scalar_condition_v1(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, frame: &CheckedBoolConditionFrameV1, node: usize,
) -> Result<(ValueId, Proof), String> {
    use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
    let receiver = if let Some(field) = frame.evaluation().cores().get(&node).and_then(|core| core.field_map()) {
        let read = state.take_exact_lexical_read(frame.source().owner(), field.receiver_site().node(), field.receiver())
            .map_err(|error| error.to_string())?;
        field.require_receiver_v1(CallValidationContextV1::Lowering(ledger),
            builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?, &read)?;
        Some(read)
    } else { None };
    let mut body = super::expression_value::emit_comparison_inputs_v1(builder, state, ledger, frame.evaluation(), node)?;
    if !body.calls.is_empty() || !body.path_reads.is_empty() { return Err(freeze("bool-condition/text-operand-order")); }
    let operation = crate::mir::normal_callable_semantic_package::checked_scalar_invoke_operation_v1(
        frame.evaluation(), node, &body.values, receiver.as_ref())?;
    let pending = super::expression_value::pending_operations(&body.acquired, &body.homes);
    let (value, kind, _) = super::expression_value::emit_scalar_operation_with_pending_v1(builder,
        &mut RawRootGraphWriterV1, operation, body.fault_frame, &pending, &mut body.bindings)?;
    if kind != crate::mir::instruction::InvokeNormalResultKind::Bool { return Err(freeze("bool-condition/text-result")); }
    if let Some(receiver) = &receiver { body.bindings.extend(receiver.original_copy_dependencies_v1()?.iter().cloned()); }
    let (terminal, continuation) = super::expression_value::finish_value_continuation_v1(builder, &mut body)?;
    Ok((value, Proof::ScalarInvocation { receiver,
        input_values: body.values.into_boxed_slice(), input_reads: body.reference_reads, entry: body.entry,
        terminal, continuation, bindings: body.bindings.into_boxed_slice() }))
}
pub(in crate::mir::builder) fn definition(function: &MirFunction, value: ValueId) -> Result<Option<Binding>, String> {
    let mut definitions = function.blocks.iter().flat_map(|(id, block)| block.all_instructions()
        .filter(move |row| row.dst_value() == Some(value)).map(move |row| (*id, row.clone())));
    let definition = definitions.next();
    if definitions.next().is_some() { return Err(freeze("bool-condition/duplicate-value-definition")); }
    Ok(definition)
}
