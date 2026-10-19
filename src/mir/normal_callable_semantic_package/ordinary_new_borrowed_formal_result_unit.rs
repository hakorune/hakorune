//! Original Unit statement source through the existing Home argument callback.
//! This does not stage executable actuals, issue Completion or select a receiver.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1;

pub(in crate::mir::normal_callable_semantic_package) fn source_unit_discard_v1(
    targets: &super::super::PreparedLexicalInstanceCallSourceTargetsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
) -> Result<Option<BorrowedCallArgumentsV1>, String> {
    let mut matching = Vec::new();
    for target in targets.as_ref().map_err(Clone::clone)? {
        if let Some(target) = target.as_ref().map_err(Clone::clone)? {
            if target.call_site() == site { matching.push(target); }
        }
    }
    let ingress = match source {
        Ok(ingress) => ingress,
        Err(_) if matching.len() == 1 && !matching[0].is_unit_discard_source_v1() => return Ok(None),
        Err(issue) => return Err(issue.clone()),
    };
    let originals: Vec<_> = ingress.source_incoming.exact_rows().filter(|row| &row.call == site).collect();
    let selected = originals.iter().any(|row| row.source.unit_discard_source_v1().is_some())
        || matching.iter().any(|target| target.is_unit_discard_source_v1());
    if !selected { return Ok(None); }
    let [original] = originals.as_slice() else { return Err(freeze("unit-source/original-missing-or-duplicate")); };
    let loan = original.source.unit_discard_source_v1().ok_or_else(|| freeze("unit-source/original-loan-missing"))?;
    // Pending stored membership cannot disappear into an old scalar probe.
    let target = original.source.require_instance().map_err(|_| freeze("unit-source/stored-receiver-pending"))?;
    let [prepared] = matching.as_slice() else { return Err(freeze("unit-source/prepared-missing-or-duplicate")); };
    let LexicalCallSourceResultRequirementV1::UnitDiscardSource(retained) = &target.result_requirement
        else { return Err(freeze("unit-source/requirement-drift")); };
    if *prepared != target || retained != loan || !loan.is_unit_candidate_v1()
        || !loan.matches(&super::super::source::CallTargetReferenceV1::from_target(target))
        || target.call_site() != site || target.callee_owner() != original.callee
    { return Err(freeze("unit-source/original-discard-drift")); }
    check_original_argument_contract_v1(contracts, target, original)?;
    let arguments = super::super::borrowed_formal_actuals::project_pending_instance_arguments_by_phase_v1(
        source, pending, contracts, target, None)?;
    Ok(Some(BorrowedCallArgumentsV1::SourceUnitDiscard { source: target.clone(), arguments }))
}

/// The incoming opaque-formal subset and full ordered actual list have different
/// cardinalities. Corroborate both against the same original declaration.
fn check_original_argument_contract_v1(
    contracts: &[OwnedCallableParameterContractDeclarationV1], target: &LexicalInstanceCallSourceTargetV1,
    incoming: &super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
) -> Result<(), String> {
    let declarations: Vec<_> = contracts.iter().filter(|row| row.owner == incoming.callee).collect();
    let [declaration] = declarations.as_slice() else { return Err(freeze("unit-source/declaration-missing-or-duplicate")); };
    if target.callee_owner() != incoming.callee || target.call_site() != &incoming.call
        || incoming.source.require_instance()? != target || declaration.batch_slot != target.target_batch_slot()
        || declaration.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
        || declaration.parameters.len() != target.target().arity() as usize
        || target.argument_sites().len() != declaration.parameters.len()
        || declaration.parameters.iter().enumerate().any(|(ordinal, row)| row.ordinal as usize != ordinal
            || row.binding.owner() != incoming.callee)
    { return Err(freeze("unit-source/original-argument-contract")); }
    let opaque: Vec<_> = declaration.parameters.iter().filter(|row| row.kind.is_ordinary_borrowed_handle()).collect();
    if opaque.len() != incoming.arguments.len() || opaque.iter().zip(&incoming.arguments).any(|(formal, (ordinal, site, binding))|
        formal.ordinal != *ordinal || formal.binding != *binding
            || target.argument_sites().get(*ordinal as usize) != Some(site)) {
        return Err(freeze("unit-source/original-opaque-arguments"));
    }
    Ok(())
}

impl PreparedBorrowedFormalIngressV1 {
    /// Original Unit return demand requires the full source Home walk.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn requires_unit_return_walk_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> bool {
        self.source_incoming.exact_rows().any(|row|
            row.callee == owner && row.source.unit_discard_source_v1().is_some())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Lend the canonical completion selected by original Unit call sources.
    /// The caller Home flow retains sources after affine call-row consumption.
    pub(in crate::mir) fn unit_completion_source_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Option<Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>>, String> {
        let Some(ingress) = &self.borrowed_formal_source else { return Ok(None); };
        let ingress = ingress.as_ref().map_err(Clone::clone)?;
        let mut selected = None;
        for incoming in ingress.source_incoming.exact_rows() {
            let Some(source) = incoming.source.instance() else { continue; };
            if source.callee_owner() != owner || !source.is_unit_discard_source_v1() { continue; }
            let completion = self.original_unit_completion_v1(source)?;
            if selected.as_ref().is_some_and(|previous| !Rc::ptr_eq(previous, &completion)) {
                return Err(freeze("unit-call/completion-cohort-drift"));
            }
            selected = Some(completion);
        }
        Ok(selected)
    }

    pub(in crate::mir) fn implicit_unit_completion_source_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Option<Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>>, String> {
        let completion = match self.unit_completion_source_v1(owner)? {
            Some(completion) => completion,
            None => {
                // Source-owned field appends require their canonical body end
                // even when no caller happens to invoke this declared method.
                // This selects no Call and manufactures no Unit-call source.
                let Some(completion) = self.completion_source_for_owner_v1(owner) else { return Ok(None); };
                if completion.returns_value() || !completion.explicit_sites().is_empty()
                    || completion.implicit_body_end().is_none() { return Ok(None); }
                let Some(flow) = completion.cleanup().root_flow() else { return Ok(None); };
                let mut proofs = flow.field_array_loop_sources().peekable();
                if proofs.peek().is_none() { return Ok(None); }
                let mut sites = std::collections::BTreeSet::new();
                for proof in proofs {
                    if proof.owner() != owner || proof.appends().is_empty() || !sites.insert(proof.site())
                        || !self.no_exit_array_home_source_v1(owner, proof.site())
                            .is_some_and(|original| Rc::ptr_eq(original, proof)) {
                        return Err(freeze("implicit-field-array/source-membership"));
                    }
                }
                Rc::clone(completion)
            }
        };
        if completion.implicit_body_end().is_none() { return Ok(None); }
        self.implicit_root_home_source_v1(owner, &completion)?;
        Ok(Some(completion))
    }

    fn original_unit_completion_v1(
        &self, source: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>, String> {
        let LexicalCallSourceResultRequirementV1::UnitDiscardSource(original) = &source.result_requirement
            else { return Err(freeze("unit-call/original-source-missing")); };
        let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("unit-call/incoming-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let mut originals = ingress.source_incoming.exact_rows().filter(|row| &row.call == source.call_site());
        let incoming = originals.next().ok_or_else(|| freeze("unit-call/original-missing"))?;
        if originals.next().is_some() || incoming.callee != source.callee_owner()
            || incoming.source.require_instance()? != source || incoming.source.unit_discard_source_v1() != Some(original)
            || !original.matches(&super::super::source::CallTargetReferenceV1::from_target(source)) {
            return Err(freeze("unit-call/original-incoming-drift"));
        }
        let contracts = self.original_parameter_contracts.as_deref().ok_or_else(|| freeze("unit-call/original-contract-missing"))?;
        check_original_argument_contract_v1(contracts, source, incoming)?;
        if incoming.source.constructor_unit_source_v1(source)?.is_none() {
            let observation = self.unit_discard_source_v1(source.call_site())
                .ok_or_else(|| freeze("unit-call/discard-observation-missing"))?;
            let arguments = super::super::borrowed_formal_actuals::project_pending_instance_arguments_by_phase_v1(
                self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("unit-call/incoming-missing"))?,
                &self.borrowed_formal_actuals, contracts, source, Some(self))?;
            if observation.unit_source() != Some(source) || observation.local_binding().is_some()
                || observation.is_expression_value() || observation.arguments() != arguments.as_ref()
                || observation.borrowed_scalar().is_some() || observation.borrowed_nullable().is_some() {
                return Err(freeze("unit-call/original-discard-drift"));
            }
        }
        let completion = self.completion_index.get(&source.callee_owner()).and_then(|row| row.as_ref().ok())
            .ok_or_else(|| freeze("unit-call/completion-unavailable"))?;
        if !original.matches_unit_completion_v1(completion) {
            return Err(freeze("unit-call/completion-source-drift"));
        }
        Ok(Rc::clone(completion))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn issue_unit_discard_v1(
        &self, source: LexicalInstanceCallSourceTargetV1,
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let completion = self.original_unit_completion_v1(&source)?;
        let contract = results.row(source.target_batch_slot()).ok_or_else(|| freeze("unit-call/result-contract-missing"))?;
        if contract.owner() != source.callee_owner() || contract.result().is_some()
            || !std::ptr::eq(contract.borrow().completion(), completion.as_ref()) {
            return Err(freeze("unit-call/result-completion-drift"));
        }
        let site = source.call_site().clone();
        let row = LexicalInstanceCallDispositionRowV1 { source, result: Some(InvokeCallResultKind::Unit),
            scalar_result: None, unit_completion: Some(completion), borrowed_completion: None, object_packet: None, mixed_result: None };
        self.corroborate_unit_discard_v1(&row)?;
        if self.lexical_instance_calls.borrow_mut().insert(site.clone(), LexicalInstanceCallDispositionSlotV1::Ready(row)).is_some() {
            return Err(freeze("unit-call/duplicate"));
        }
        self.record_lifecycle_local_call_site(site.owner(), site);
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn constructor_unit_step_ready_v1(
        &self,
        source: &Rc<crate::mir::normal_callable_semantic_package::instance_construction::ConstructionSourceStructureV1>,
        site: &OwnedExprSiteV1,
    ) -> Result<bool, String> {
        let slots = self.lexical_instance_calls.borrow();
        let Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) = slots.get(site) else { return Ok(false); };
        if row.result() != Some(InvokeCallResultKind::Unit) { return Ok(false); }
        let Some(loan) = self.constructor_unit_call_source_v1(row)? else { return Ok(false); };
        if !Rc::ptr_eq(source, loan.source()) || source.child_call_source_v1(site)?.as_ref() != Some(&loan) {
            return Err(freeze("constructor-unit/plan-source-drift"));
        }
        let completion = row.unit_completion.as_ref().ok_or_else(|| freeze("constructor-unit/completion-missing"))?;
        self.root_home_flow_ready_v1(completion)
    }

    /// Same original constructor-step loan; selection and affine row stay owned
    /// by the existing ordinary call owner, never reconstructed from MIR.
    pub(crate) fn constructor_unit_call_source_v1(
        &self, row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::instance_construction::ConstructionChildCallSourceLoanV1>, String> {
        self.corroborate_unit_discard_v1(row)?;
        let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("unit-call/incoming-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let mut originals = ingress.source_incoming.exact_rows().filter(|incoming| &incoming.call == row.call_site());
        let incoming = originals.next().ok_or_else(|| freeze("unit-call/original-missing"))?;
        if originals.next().is_some() { return Err(freeze("unit-call/original-duplicate")); }
        Ok(incoming.source.constructor_unit_source_v1(row.source_target())?.cloned())
    }

    pub(crate) fn corroborate_unit_discard_v1(&self, row: &LexicalInstanceCallDispositionRowV1) -> Result<(), String> {
        let completion = self.original_unit_completion_v1(row.source_target())?;
        if row.result() != Some(InvokeCallResultKind::Unit) || row.scalar_result.is_some() || row.object_packet.is_some()
            || row.unit_completion.as_ref().is_none_or(|retained| !Rc::ptr_eq(retained, &completion)) {
            return Err(freeze("unit-call/retained-completion-drift"));
        }
        Ok(())
    }
}
