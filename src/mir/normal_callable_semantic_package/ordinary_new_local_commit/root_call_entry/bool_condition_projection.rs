//! One original Bool condition and its real source-ordered physical producers.
//! A fixed prefix may be lent during branch descent; final users require the
//! same affine outer-Branch seal. No semantic permission is inferred from CFG.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use crate::mir::builder::{IfConditionMaterializationV1, ExactLexicalReadV1, LocalSsaPhysicalRelationV1};
use crate::mir::builder::ops::CompletedLogicalShortCircuitV1;
use crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1;
use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1;
use std::cell::RefCell;
use std::rc::Rc;
type Binding = (BasicBlockId, MirInstruction);
type Finished = super::super::super::physical_boundary::FinishedBindings;

#[path = "bool_condition_source.rs"]
mod source;
#[path = "bool_condition_control.rs"]
mod control;

/// Cursors are observations at the existing recursive descent boundary.
/// They never select source children or issue a value/operation contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct BoolConditionCursorV1 {
    pub(in crate::mir) block: BasicBlockId,
    pub(in crate::mir) instruction: usize,
}
#[derive(Debug)]
pub(in crate::mir) enum BoolConditionValueProofV1 {
    Literal(Binding),
    Reference { read: ExactLexicalReadV1, producer: Option<Binding> },
    Field(Binding),
    Call { packet: Rc<EmittedLexicalCallProjectionV1>, bindings: Box<[Binding]> },
    Binary { append: Binding, operands: [Rc<LocalSsaPhysicalRelationV1>; 2] },
    Logical { completed: CompletedLogicalShortCircuitV1, conditions: [Rc<LocalSsaPhysicalRelationV1>; 2] },
}
#[derive(Debug)]
pub(in crate::mir) struct BoolConditionNodeProjectionV1 {
    pub(in crate::mir) node: usize,
    pub(in crate::mir) entry: BoolConditionCursorV1,
    pub(in crate::mir) exit: BoolConditionCursorV1,
    pub(in crate::mir) value: ValueId,
    pub(in crate::mir) proof: BoolConditionValueProofV1,
}
#[derive(Debug)]
pub(in crate::mir) struct EmittedBoolConditionProjectionV1 {
    frame: Rc<CheckedBoolConditionFrameV1>,
    nodes: Box<[BoolConditionNodeProjectionV1]>,
    entry: Binding,
    prefix: Vec<Binding>,
    materialization: Rc<IfConditionMaterializationV1>,
    outer: RefCell<Option<Binding>>,
}

/// Sole writer lease. Dropping it without sealing leaves an unpublishable
/// Pending packet; no final consumer can turn absence into completion.
pub(in crate::mir) struct BoolConditionSealV1 {
    packet: Rc<EmittedBoolConditionProjectionV1>,
}
impl BoolConditionSealV1 {
    pub(in crate::mir) fn seal(self, function: &MirFunction,
        materialization: &Rc<IfConditionMaterializationV1>, branch: Binding,
    ) -> Result<(), String> {
        if !Rc::ptr_eq(materialization, &self.packet.materialization)
            || self.packet.outer.borrow().is_some()
        { return Err(freeze("bool-condition/outer-seal-identity")); }
        self.packet.verify_outer(function, &branch, None)?;
        *self.packet.outer.borrow_mut() = Some(branch);
        Ok(())
    }
}
impl EmittedBoolConditionProjectionV1 {
    pub(in crate::mir) fn prepare(
        frame: Rc<CheckedBoolConditionFrameV1>, nodes: Vec<BoolConditionNodeProjectionV1>,
        entry: Binding, prefix: Vec<Binding>, materialization: Rc<IfConditionMaterializationV1>,
        ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
    ) -> Result<(Rc<Self>, BoolConditionSealV1), String> {
        let packet = Rc::new(Self { frame, nodes: nodes.into_boxed_slice(), entry, prefix,
            materialization, outer: RefCell::new(None) });
        packet.validate_source(CallValidationContextV1::Lowering(ledger), function, None)?;
        packet.validate_prefix_recorded(&packet.prefix)?;
        packet.validate_original_scope(ledger, function)?;
        let lease = BoolConditionSealV1 { packet: Rc::clone(&packet) };
        Ok((packet, lease))
    }
    pub(in crate::mir) fn frame(&self) -> &Rc<CheckedBoolConditionFrameV1> { &self.frame }
    pub(in crate::mir) fn prefix(&self) -> &[Binding] { &self.prefix }
    pub(in crate::mir) fn require_sealed(&self) -> Result<(), String> {
        if self.outer.borrow().is_none() { return Err(freeze("bool-condition/outer-branch-pending")); }
        Ok(())
    }
    pub(in crate::mir) fn calls(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        self.nodes.iter().filter_map(|node| match &node.proof {
            BoolConditionValueProofV1::Call { packet, .. } => Some(packet.as_ref()), _ => None,
        })
    }
    fn node(&self, index: usize) -> Result<&BoolConditionNodeProjectionV1, String> {
        let mut nodes = self.nodes.iter().filter(|node| node.node == index);
        let node = nodes.next().ok_or_else(|| freeze("bool-condition/node-missing"))?;
        if nodes.next().is_some() { return Err(freeze("bool-condition/node-duplicate")); }
        Ok(node)
    }
    pub(super) fn validate_prefix_recorded(&self, bindings: &[Binding]) -> Result<(), String> {
        if self.prefix.is_empty() || !self.prefix.contains(&self.entry) {
            return Err(freeze("bool-condition/entry-membership"));
        }
        for binding in &self.prefix {
            if bindings.iter().filter(|actual| *actual == binding).count() != 1 {
                return Err(freeze("bool-condition/prefix-membership"));
            }
        }
        for packet in self.calls() { packet.validate_recorded(bindings)?; }
        Ok(())
    }
    pub(super) fn validate_recorded(&self, bindings: &[Binding], context: CallValidationContextV1<'_>,
        function: &MirFunction, coordinates: Option<&Finished>, finishing: Option<&Finished>,
    ) -> Result<(), String> {
        self.require_sealed()?;
        let ledger = context.ledger();
        self.validate_source(context, function, finishing)?;
        for original in &self.prefix {
            match mapped(original, coordinates)? {
                Some(actual) if bindings.iter().filter(|row| **row == actual).count() == 1 => {},
                None if matches!(original.1, MirInstruction::Const { .. } | MirInstruction::Copy { .. }
                    | MirInstruction::Jump { edge_args: None, .. }) => {},
                _ => return Err(freeze("bool-condition/final-prefix-membership")),
            }
        }
        let outer = self.outer.borrow();
        self.verify_outer(function, outer.as_ref().expect("sealed above"), finishing)?;
        for packet in self.calls() { packet.validate_recorded_projected(bindings, coordinates)?; }
        if finishing.is_none() { self.validate_original_scope(ledger, function)?; }
        Ok(())
    }
    fn verify_outer(&self, function: &MirFunction, original: &Binding,
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        let (then_bb, else_bb) = self.materialization.arms();
        if original.0 != self.materialization.block()
            || !matches!(&original.1, MirInstruction::Branch { condition, then_bb: actual_then, else_bb: actual_else,
                then_edge_args: Some(a), else_edge_args: Some(b) }
                if *condition == self.materialization.values().1 && (*actual_then, *actual_else) == (then_bb, else_bb)
                    && a.layout == crate::mir::edge_args::JumpArgsLayout::CarriersOnly && a.values.is_empty()
                    && b.layout == crate::mir::edge_args::JumpArgsLayout::CarriersOnly && b.values.is_empty())
        { return Err(freeze("bool-condition/outer-branch-source")); }
        require_binding(function, original, finishing)
    }
}
fn mapped(original: &Binding, finishing: Option<&Finished>) -> Result<Option<Binding>, String> {
    match finishing { Some(view) => view.binding(original.0, &original.1), None => Ok(Some(original.clone())) }
}
fn require_binding(function: &MirFunction, original: &Binding, finishing: Option<&Finished>) -> Result<(), String> {
    let actual = mapped(original, finishing)?.ok_or_else(|| freeze("bool-condition/required-producer-removed"))?;
    if !super::super::super::physical_boundary::check_binding(function, None, actual.0, &actual.1)? {
        return Err(format!("{} original={original:?} actual={actual:?} finished={}",
            freeze("bool-condition/producer-missing"), finishing.is_some()));
    }
    Ok(())
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn bool_condition_frame_v1(&self, owner: FunctionOwnerIdV1, statement: &SourceStmtSiteV1)
        -> Result<Option<Rc<CheckedBoolConditionFrameV1>>, String> {
        let Some(flow) = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow())
            else { return Ok(None) };
        let mut frames = flow.expression_plans().iter().filter_map(|plan| plan.condition_frame())
            .filter(|frame| frame.source().root().statement() == statement);
        let frame = frames.next().cloned();
        if frames.next().is_some() { return Err(freeze("bool-condition/duplicate-source-frame")); }
        Ok(frame)
    }
    pub(in crate::mir) fn validate_bool_condition_source_v1(&self, frame: &Rc<CheckedBoolConditionFrameV1>) -> Result<(), String> {
        let owner = frame.source().owner();
        let sources = self.expected_local_call_binding_sources(owner)?;
        let groups = self.root_local_call_bindings.borrow();
        let count = groups.get(&owner).map_or(0, Vec::len);
        if !matches!(sources.get(count), Some(LifecyclePrefixSourceV1::BoolCondition { frame: expected, .. })
            if Rc::ptr_eq(frame, expected)) { return Err(freeze("bool-condition/source-prefix-order")); }
        Ok(())
    }
    pub(in crate::mir) fn prepare_bool_condition_bindings_v1(&self, packet: Rc<EmittedBoolConditionProjectionV1>)
        -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        self.prepare_local_call_binding_group(packet.frame().source().owner(),
            RootLifecyclePrefixBindingGroupV1::new_bool_pending(packet)?)
    }
    /// Lend the same source-owned Phi tuples to draft repair. A symbol is
    /// only the existing root/child physical locator, never source authority.
    pub(in crate::mir) fn original_source_phis_for_repair_v1(
        &self, function: &MirFunction, root_key: &str,
    ) -> Result<Vec<Binding>, String> {
        let owner = if function.signature.name == root_key {
            self.root_owner()
        } else {
            let children = self.child_physical_validation.borrow();
            let mut owners = children.iter().filter_map(|(owner, state)| {
                let symbol = match state {
                    super::super::super::ChildPhysicalValidation::Checked { symbol, .. }
                    | super::super::super::ChildPhysicalValidation::FinishingChecked { symbol, .. } => symbol,
                };
                (symbol == &function.signature.name).then_some(*owner)
            });
            let owner = owners.next();
            if owners.next().is_some() { return Err(freeze("bool-condition/repair-owner-duplicate")); }
            owner
        };
        let Some(owner) = owner else { return Ok(Vec::new()) };
        let mut bindings = self.original_condition_bindings_v1(owner)?;
        bindings.extend(self.original_field_array_append_bindings_v1(owner)?);
        bindings.extend(self.original_borrowed_field_write_guard_bindings_v1(owner)?);
        bindings.extend(self.original_borrowed_array_set_bindings_v1(owner)?);
        bindings.extend(self.original_borrowed_field_write_bindings_v1(owner, false)?);
        Ok(bindings.into_iter()
            .filter(|row| matches!(row.1, MirInstruction::Phi { .. })).collect())
    }

    /// Shared source lender for draft repair and required finishing liveness.
    /// The prefix stays in its original group; no physical reclassification.
    pub(in crate::mir) fn original_condition_bindings_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Binding>, String> {
        let groups = self.root_local_call_bindings.borrow();
        let mut bindings = Vec::new();
        for group in groups.get(&owner).into_iter().flatten() {
            let Some(packet) = group.bool_condition() else { continue };
            packet.require_sealed()?;
            let frame = self.bool_condition_frame_v1(owner, packet.frame.source().root().statement())?
                .ok_or_else(|| freeze("bool-condition/repair-source-missing"))?;
            if !Rc::ptr_eq(&frame, &packet.frame) { return Err(freeze("bool-condition/repair-source-foreign")); }
            for row in &packet.prefix {
                if !bindings.contains(row) { bindings.push(row.clone()); }
            }
        }
        Ok(bindings)
    }

    pub(in crate::mir) fn require_sealed_condition_groups_v1(&self) -> Result<(), String> {
        for group in self.root_local_call_bindings.borrow().values().flatten() {
            group.require_sealed_condition_v1()?;
        }
        Ok(())
    }
}
