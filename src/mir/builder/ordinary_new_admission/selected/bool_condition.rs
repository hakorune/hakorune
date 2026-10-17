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
impl IfFormPhysicalObserverV1 for LoweredBoolConditionV1 {
    fn before_branches(&mut self, function: &MirFunction, pending: Rc<IfConditionMaterializationV1>) -> Result<(), String> {
        let nodes = self.nodes.take().ok_or_else(|| freeze("bool-condition/duplicate-condition-materialization"))?;
        let mut prefix = vec![self.entry.clone()];
        for node in &nodes {
            match &node.proof {
                Proof::Literal(binding) | Proof::Field(binding) => prefix.push(binding.clone()),
                Proof::Reference { producer, .. } => prefix.extend(producer.iter().cloned()),
                Proof::Call { packet, bindings } => {
                    prefix.extend(bindings.iter().cloned());
                    let MirInstruction::Invoke { fault_frame, .. } = packet.invoke_binding().1
                        else { return Err(freeze("bool-condition/call-frame")) };
                    prefix.push(definition(function, fault_frame)?.ok_or_else(|| freeze("bool-condition/frame-definition"))?);
                },
                Proof::Binary { append, operands } => {
                    prefix.push(append.clone());
                    for relation in operands { relation.append_observed_bindings_v1(&mut prefix); }
                },
                Proof::Logical { completed, conditions } => {
                    prefix.extend_from_slice(completed.bindings());
                    for relation in conditions { relation.append_observed_bindings_v1(&mut prefix); }
                },
            }
        }
        prefix.extend_from_slice(pending.bindings());
        pending.relation().append_observed_bindings_v1(&mut prefix);
        let mut unique = Vec::new();
        for binding in prefix { if !unique.contains(&binding) { unique.push(binding); } }
        let (packet, seal) = EmittedBoolConditionProjectionV1::prepare(Rc::clone(&self.frame), nodes,
            self.entry.clone(), unique, pending, &self.ledger, function)?;
        self.ledger.prepare_bool_condition_bindings_v1(packet)?.commit()?;
        self.seal = Some(seal);
        Ok(())
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
pub(in crate::mir::builder) fn definition(function: &MirFunction, value: ValueId) -> Result<Option<Binding>, String> {
    let mut definitions = function.blocks.iter().flat_map(|(id, block)| block.all_instructions()
        .filter(move |row| row.dst_value() == Some(value)).map(move |row| (*id, row.clone())));
    let definition = definitions.next();
    if definitions.next().is_some() { return Err(freeze("bool-condition/duplicate-value-definition")); }
    Ok(definition)
}
