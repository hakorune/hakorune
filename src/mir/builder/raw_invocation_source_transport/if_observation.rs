//! If observations share the existing condition, branch and merge writers.
use super::*;
use crate::mir::builder::if_form::{IfConditionMaterializationV1, IfFormPhysicalObserverV1};
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathV1, SourcePathSegmentV1};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};

struct WriteGuardObserverV1 {
    ledger: Rc<OrdinaryNewClaimLedgerV1>,
    owner: FunctionOwnerIdV1,
    guard: SourceNodeSiteV1,
}
impl IfFormPhysicalObserverV1 for WriteGuardObserverV1 {
    fn before_branches(&mut self, function: &MirFunction, pending: Rc<IfConditionMaterializationV1>) -> Result<(), String> {
        self.ledger.prepare_borrowed_field_write_guard_condition_v1(self.owner, &self.guard, pending, function)
    }
    fn emitted_branch(&mut self, _function: &MirFunction, pending: &Rc<IfConditionMaterializationV1>,
        branch: (BasicBlockId, MirInstruction),
    ) -> Result<(), String> {
        self.ledger.seal_borrowed_field_write_guard_branch_v1(self.owner, &self.guard, pending, branch)
    }
}
impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn observe_write_guard_binary_v1(&self, builder: &mut MirBuilder,
        children: (ValueId, ValueId), completed: &crate::mir::builder::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<(), String> {
        let (Some(ledger), Some(owner)) = (&self.ordinary_new_claim_ledger, self.callable_owner_v1()) else { return Ok(()) };
        let Some(node) = self.current_source_site_v1() else { return Ok(()) };
        let site = OwnedExprSiteV1::new(owner, SourceExprSiteV1::from_node(node));
        if ledger.borrowed_field_write_guard_for_binary_v1(owner, &site)?.is_none() { return Ok(()) }
        let original = completed.comparison_original().ok_or("[freeze:contract][borrowed-write-guard/not-compare]")?;
        let MirInstruction::Compare { lhs, rhs, .. } = original.1 else {
            return Err("[freeze:contract][borrowed-write-guard/not-compare]".into());
        };
        let function = builder.function_state.current_function.as_ref().ok_or("[freeze:contract][borrowed-write-guard/no-function]")?;
        let index = &mut builder.function_state.local_ssa_correspondence;
        let operands = [index.loan(function, children.0, lhs).ok_or("[freeze:contract][borrowed-write-guard/left-relation]")?,
            index.loan(function, children.1, rhs).ok_or("[freeze:contract][borrowed-write-guard/right-relation]")?];
        for operand in &operands { index.require_member(operand)?; }
        ledger.record_borrowed_field_write_guard_compare_v1(owner, site, original.clone(), operands, function)
    }
    pub(super) fn write_guard_observer_v1(&self) -> Result<Option<Box<dyn IfFormPhysicalObserverV1>>, String> {
        let (Some(ledger), Some(owner)) = (&self.ordinary_new_claim_ledger, self.callable_owner_v1()) else { return Ok(None) };
        let Some(node) = self.current_source_site_v1() else { return Ok(None) };
        let condition = SourcePathV1::from_node(&node).child(SourcePathSegmentV1::IfCondition).expr();
        let site = OwnedExprSiteV1::new(owner, condition);
        let Some((guard, _, _)) = ledger.borrowed_field_write_guard_for_binary_v1(owner, &site)? else { return Ok(None) };
        Ok(Some(Box::new(WriteGuardObserverV1 { ledger: Rc::clone(ledger), owner, guard })))
    }
}

struct IfStateObserverV1 {
    state: Rc<std::cell::RefCell<crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState>>,
    slots: Vec<(crate::mir::resolved_semantics::BindingRefV1, String)>,
    baseline: Option<crate::mir::builder::normal_callable_semantic_lowering_state::MaterializedValuesV1>,
    arms: [Option<crate::mir::builder::normal_callable_semantic_lowering_state::MaterializedValuesV1>; 2],
}
impl IfFormPhysicalObserverV1 for IfStateObserverV1 {
    fn before_branches(&mut self, _: &MirFunction, _: Rc<IfConditionMaterializationV1>) -> Result<(), String> {
        self.baseline = Some(self.state.borrow().source_values_snapshot());
        Ok(())
    }
    fn emitted_branch(&mut self, _: &MirFunction, _: &Rc<IfConditionMaterializationV1>,
        _: (BasicBlockId, MirInstruction),
    ) -> Result<(), String> { Ok(()) }
    fn enter_branch(&mut self, _: crate::mir::builder::if_form::IfBranchKindV1) -> Result<(), String> {
        let baseline = self.baseline.as_ref().ok_or("If source value baseline missing")?.clone();
        self.state.borrow_mut().restore_source_values(baseline);
        Ok(())
    }
    fn finished_branch(&mut self, branch: crate::mir::builder::if_form::IfBranchKindV1) -> Result<(), String> {
        let index = match branch { crate::mir::builder::if_form::IfBranchKindV1::Then => 0,
            crate::mir::builder::if_form::IfBranchKindV1::Else => 1 };
        if self.arms[index].is_some() { return Err("If source value arm observed twice".into()); }
        self.arms[index] = Some(self.state.borrow().source_values_snapshot());
        Ok(())
    }
    fn completed_merge(&mut self, builder: &mut MirBuilder, exits: [Option<BasicBlockId>; 2]) -> Result<(), String> {
        let baseline = self.baseline.take().ok_or("If source value baseline missing")?;
        let [then_values, else_values] = &mut self.arms;
        let arms = [then_values.take().ok_or("If Then source values missing")?,
            else_values.take().ok_or("If Else source values missing")?];
        self.state.borrow_mut().restore_if_normal_values_v1(baseline, arms,
            exits.map(|exit| exit.is_some()), &self.slots, &builder.function_state.variable_ctx.variable_map)
    }
}
impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn if_state_observer_v1(&self, builder: &MirBuilder) -> Result<Option<Box<dyn IfFormPhysicalObserverV1>>, String> {
        let Some(state) = &self.callable_ledger else { return Ok(None) };
        let slots = state.borrow().source_if_value_slots_v1(&builder.function_state.variable_ctx.variable_map)?;
        Ok(Some(Box::new(IfStateObserverV1 { state: Rc::clone(state), slots, baseline: None, arms: [None, None] })))
    }
}
