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
    claims: Option<Rc<OrdinaryNewClaimLedgerV1>>,
    sources: Vec<Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>>,
    pending: Option<Rc<IfConditionMaterializationV1>>,
    branch: Option<(BasicBlockId, MirInstruction)>,
    slots: Vec<(crate::mir::resolved_semantics::BindingRefV1, String)>,
    baseline: Option<crate::mir::builder::normal_callable_semantic_lowering_state::MaterializedValuesV1>,
    arms: [Option<crate::mir::builder::normal_callable_semantic_lowering_state::MaterializedValuesV1>; 2],
}
impl IfFormPhysicalObserverV1 for IfStateObserverV1 {
    fn before_branches(&mut self, _: &MirFunction, pending: Rc<IfConditionMaterializationV1>) -> Result<(), String> {
        if self.pending.replace(pending).is_some() { return Err("If condition observed twice".into()); }
        self.baseline = Some(self.state.borrow().source_values_snapshot());
        Ok(())
    }
    fn emitted_branch(&mut self, _: &MirFunction, pending: &Rc<IfConditionMaterializationV1>,
        branch: (BasicBlockId, MirInstruction),
    ) -> Result<(), String> {
        if self.pending.as_ref().is_none_or(|original| !Rc::ptr_eq(original, pending))
            || self.branch.replace(branch).is_some() { return Err("If branch observation mismatch".into()); }
        Ok(())
    }
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
        let mut observed = Vec::new();
        for source in &self.sources {
            if !self.slots.iter().any(|(binding, _)| *binding == source.binding()) {
                return Err("If scalar source has no original value slot".into());
            }
            let claims = self.claims.as_ref().ok_or("If scalar source claims missing")?;
            let inputs = [self.state.borrow().observe_snapshot_binding_v1(&arms[0], source.binding()).map_err(|error| error.to_string())?,
                self.state.borrow().observe_snapshot_binding_v1(&arms[1], source.binding()).map_err(|error| error.to_string())?];
            let name = &self.slots.iter().find(|(binding, _)| *binding == source.binding()).ok_or("If scalar slot missing")?.1;
            let value = *builder.function_state.variable_ctx.variable_map.get(name).ok_or("If scalar merged value missing")?;
            let function = builder.function_state.current_function.as_ref().ok_or("If scalar function missing")?;
            let index = &mut builder.function_state.local_ssa_correspondence;
            let relation = index.loan(function, value, value).ok_or("If scalar Phi relation missing")?;
            index.require_member(&relation)?;
            observed.push(crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1::prepare_branch_scalar_phi_v1(
                Rc::clone(source), Rc::clone(self.pending.as_ref().ok_or("If scalar condition missing")?),
                self.branch.clone().ok_or("If scalar branch missing")?, exits, inputs, relation,
                crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(claims), function)?);
        }
        let mut state = self.state.borrow_mut();
        state.restore_if_normal_values_v1(baseline, arms,
            exits.map(|exit| exit.is_some()), &self.slots, &builder.function_state.variable_ctx.variable_map)?;
        for point in observed { state.attach_branch_scalar_phi_v1(point)?; }
        Ok(())
    }
}
impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn if_state_observer_v1(&self, builder: &MirBuilder) -> Result<Option<Box<dyn IfFormPhysicalObserverV1>>, String> {
        let Some(state) = &self.callable_ledger else { return Ok(None) };
        let slots = state.borrow().source_if_value_slots_v1(&builder.function_state.variable_ctx.variable_map)?;
        let mut sources: Vec<Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>> = Vec::new();
        if let (Some(claims), Some(owner), Some(node)) = (&self.ordinary_new_claim_ledger, self.callable_owner_v1(), self.current_source_site_v1()) {
            for source in claims.branch_scalar_sources_v1(owner, &crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(node)) {
                claims.require_branch_scalar_source_v1(&source)?;
                if !sources.iter().any(|old| old.as_ref() == source.as_ref()) { sources.push(source); }
            }
        }
        Ok(Some(Box::new(IfStateObserverV1 { state: Rc::clone(state), slots, sources,
            claims: self.ordinary_new_claim_ledger.clone(), pending: None, branch: None, baseline: None, arms: [None, None] })))
    }
}
