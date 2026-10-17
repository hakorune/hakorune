//! Shared source-call identity and witness composition; no Home or pending policy.
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::AppMainSourceRefV1;
use super::{
    parameter_contract, ResultActualSourceV1, ResultExitOriginV1, ResultFormalSubstitutionV1,
    ResultOriginWitnessV1, ResultValueOriginV1, ResultWitnessStepV1,
};
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::rc::Rc;

/// The original local initializer edge, shared by nullable borrowing and
/// construction-child provenance. This does not classify ownership or issue Home.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OriginalLocalCallSourceV1 {
    pub(super) declaration: crate::mir::resolved_semantics::SourceBindingSiteV1,
    pub(super) receiver: Option<crate::mir::resolved_semantics::SourceExprSiteV1>,
    pub(super) key: CanonicalSameModuleCallableKeyV1,
    pub(super) callee_owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    pub(super) caller_slot: u32,
    pub(super) actuals: Box<[ResultActualSourceV1]>,
    pub(super) call: OwnedExprSiteV1,
}

impl OriginalLocalCallSourceV1 {
    pub(super) fn formal_actual_v1(
        &self,
        batch: &super::VerifiedResolvedCallableSemanticBatchV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        ordinal: u32,
    ) -> Result<(super::BindingRefV1, u32), &'static str> {
        let root = self
            .actuals
            .get(ordinal as usize)
            .and_then(|row| row.binding)
            .ok_or("borrowed-nullable/actual-root")?;
        let mut contracts = parameters
            .iter()
            .filter(|row| row.batch_slot == self.caller_slot);
        let contract = contracts
            .next()
            .ok_or("borrowed-nullable/caller-contract")?;
        if contracts.next().is_some() || contract.owner != self.call.owner() {
            return Err("borrowed-nullable/caller-contract");
        }
        let mut matching = contract.parameters.iter().filter(|row| row.binding == root);
        let parameter = matching.next().ok_or("borrowed-nullable/formal-root")?;
        if matching.next().is_some() {
            return Err("borrowed-nullable/formal-root");
        }
        if contract.parameters.iter().filter(|row| row.ordinal == parameter.ordinal).count() != 1 {
            return Err("borrowed-nullable/formal-root");
        }
        let exact = batch
            .with_lowering_input(self.caller_slot, |input| {
                root.owner() == self.call.owner()
                    && matches!(input.function().binding(root).map(|row| row.kind()),
                    Some(super::BindingKindV1::Parameter { index }) if index == parameter.ordinal)
                    && matches!(
                        parameter.kind,
                        CallableParameterContractKindV1::DeclaredObject(_)
                            | CallableParameterContractKindV1::OpaqueHandle
                    )
            })
            .map_err(|_| "borrowed-nullable/caller-input")?;
        if !exact {
            return Err("borrowed-nullable/formal-root");
        }
        Ok((root, parameter.ordinal))
    }

    /// Preserve every original root, including formal-derived Null, without
    /// projecting a mixed result into the legacy owning/borrowed class enum.
    pub(super) fn compose_at_v1(
        &self,
        output: &OwnedExprSiteV1,
        facts: &super::OrdinaryNewResultClassClaimsV1,
        batch: &super::VerifiedResolvedCallableSemanticBatchV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
    ) -> Result<Vec<Rc<ResultOriginWitnessV1>>, &'static str> {
        let roots = facts.checked_original_source_roots_v1(&self.key, self.callee_owner)?;
        if roots.is_empty() {
            return Err("borrowed-nullable/callee-roots-empty");
        }
        for root in roots {
            if let Some(ordinal) = root.formal_ordinal() {
                self.formal_actual_v1(batch, parameters, ordinal)?;
            }
        }
        CallWitnessSourceV1::verify(output, &self.call, &self.key, &self.actuals)
            .and_then(|source| {
                source.compose(facts.outcomes(&self.key)?, parameters, self.caller_slot)
            })
            .ok_or("borrowed-nullable/substitution")
    }
}

impl super::OrdinaryNewResultClassClaimsV1 {
    pub(super) fn original_local_call_source_v1(
        &self,
        batch: &super::VerifiedResolvedCallableSemanticBatchV1,
        selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        call: &OwnedExprSiteV1,
        destination: super::BindingRefV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        app_main: Option<AppMainSourceRefV1<'_>>,
    ) -> Result<Option<OriginalLocalCallSourceV1>, &'static str> {
        if destination.owner() != call.owner() {
            return Err("borrowed-nullable/destination-owner");
        }
        let mut declarations = batch
            .declarations()
            .filter(|row| row.owner() == call.owner());
        let declaration = declarations
            .next()
            .ok_or("borrowed-nullable/caller-missing")?;
        if declarations.next().is_some() {
            return Err("borrowed-nullable/caller-not-unique");
        }
        let slot = declaration.batch_slot();
        let caller_key = match selected.key_for_batch_slot(slot) {
            Some(super::SelectedNormalCallableKeyV1::Cataloged(key)) => key,
            None => app_main.ok_or("borrowed-nullable/caller-key")?
                .checked_caller_key(batch, parameters, call.owner(), slot)?,
            Some(_) => return Err("borrowed-nullable/caller-key"),
        };
        batch
            .with_lowering_input(slot, |input| {
                let function = input.function();
                let declaration = original_local_declaration_v1(input, call, destination)?;
                let shape = input.body_shape().ok_or("borrowed-nullable/body-shape")?;
                let Some((key, actuals)) = super::resolve_call_key(
                    call.site(),
                    function,
                    shape,
                    caller_key,
                    batch,
                    selected,
                    fields,
                ) else {
                    return Ok(None);
                };
                // Missing source outcomes select no result edge. Demand a
                // cataloged callee only after the source owner has a product.
                if self.outcomes(&key).is_none() {
                    return Ok(None);
                }
                let receiver = function
                    .method_call(call.site())
                    .map(|method| method.receiver_site().clone());
                let callee_slot = selected
                    .batch_slot(&super::SelectedNormalCallableKeyV1::Cataloged(key.clone()))
                    .ok_or("borrowed-nullable/callee-key")?;
                let mut callees = batch
                    .declarations()
                    .filter(|row| row.batch_slot() == callee_slot);
                let callee_owner = callees
                    .next()
                    .ok_or("borrowed-nullable/callee-owner")?
                    .owner();
                if callees.next().is_some()
                    || callee_owner.compilation_brand() != call.owner().compilation_brand()
                {
                    return Err("borrowed-nullable/callee-owner");
                }
                Ok(Some(OriginalLocalCallSourceV1 {
                    declaration,
                    receiver,
                    key,
                    callee_owner,
                    caller_slot: slot,
                    actuals,
                    call: call.clone(),
                }))
            })
            .map_err(|_| "borrowed-nullable/caller-input")?
    }

    /// Consume the canonical preparation's original target coordinates. This
    /// lends an input demand only; receiver eligibility is proved downstream.
    pub(super) fn original_prepared_local_call_source_v1(
        &self,
        batch: &super::VerifiedResolvedCallableSemanticBatchV1,
        selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        reference: &super::super::lexical_instance_call::CallTargetReferenceV1,
        destination: super::BindingRefV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        app_main: Option<AppMainSourceRefV1<'_>>,
    ) -> Result<OriginalLocalCallSourceV1, &'static str> {
        let call = &reference.call_site;
        let mut callers = batch.declarations().filter(|row| row.owner() == call.owner());
        let caller = callers.next().ok_or("borrowed-nullable/caller-missing")?;
        if callers.next().is_some() { return Err("borrowed-nullable/caller-not-unique"); }
        let slot = caller.batch_slot();
        match selected.key_for_batch_slot(slot) {
            Some(super::SelectedNormalCallableKeyV1::Cataloged(_)) => {},
            None => { app_main.ok_or("borrowed-nullable/caller-key")?
                .checked_caller_key(batch, parameters, call.owner(), slot)?; },
            Some(_) => return Err("borrowed-nullable/caller-key"),
        }
        let mut callees = batch.declarations().filter(|row| row.batch_slot() == reference.target_batch_slot);
        let callee = callees.next().ok_or("borrowed-nullable/callee-owner")?;
        if callees.next().is_some() || callee.owner() != reference.callee_owner
            || callee.owner().compilation_brand() != call.owner().compilation_brand()
            || !matches!(selected.key_for_batch_slot(reference.target_batch_slot),
                Some(super::SelectedNormalCallableKeyV1::Cataloged(key)) if key == &reference.target)
        { return Err("borrowed-nullable/callee-owner"); }
        batch.with_lowering_input(slot, |input| {
            let declaration = original_local_declaration_v1(input, call, destination)?;
            let method = input.function().method_call(call.site())
                .ok_or("borrowed-nullable/original-call")?;
            if !reference.matches_original_call_v1(input, method) {
                return Err("borrowed-nullable/original-call");
            }
            Ok(OriginalLocalCallSourceV1 {
                declaration, receiver: Some(reference.receiver_site.clone()),
                key: reference.target.clone(), callee_owner: reference.callee_owner,
                caller_slot: slot,
                actuals: super::call_actual_bindings(&reference.argument_sites, input.function()),
                call: call.clone(),
            })
        }).map_err(|_| "borrowed-nullable/caller-input")?
    }

    /// Borrow the already checked raw target without resolving its key again.
    /// This input source grants no result, Home or completed wrapper support.
    pub(super) fn original_local_object_target_source_v1(
        &self,
        batch: &super::VerifiedResolvedCallableSemanticBatchV1,
        selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        target: &super::super::lexical_instance_call::LexicalInstanceCallSourceTargetV1,
        destination: super::BindingRefV1,
        loan: &super::ObjectCallSourceLoanV1,
    ) -> Result<OriginalLocalCallSourceV1, &'static str> {
        self.recheck_object_call_source_v1(batch, selected, target, loan)?;
        let call = target.call_site();
        let mut callers = batch.declarations().filter(|row| row.owner() == call.owner());
        let caller = callers.next().ok_or("returned-field/caller-missing")?;
        if callers.next().is_some() { return Err("returned-field/caller-ambiguous"); }
        batch.with_lowering_input(caller.batch_slot(), |input| {
            let declaration = original_local_declaration_v1(input, call, destination)?;
            if input.forest().semantic_owners().any(|(_, function)|
                function.assignment_targets().any(|(_, target)| matches!(target,
                    super::ResolvedAssignmentTargetV1::UpvarRebind(upvar) if upvar.source() == destination))) {
                return Err("returned-field/local-upvar-rebind");
            }
            Ok(OriginalLocalCallSourceV1 {
                declaration, receiver: Some(target.receiver_site().clone()),
                key: target.target().clone(), callee_owner: target.callee_owner(),
                caller_slot: caller.batch_slot(),
                actuals: super::call_actual_bindings(target.argument_sites(), input.function()),
                call: call.clone(),
            })
        }).map_err(|_| "returned-field/caller-input")?
    }

}

/// Shared LOCAL spelling law. The owning returned-field DAG also rejects
/// descendant UpvarRebinds before lending its immutable acquisition.
pub(super) fn original_local_declaration_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    call: &OwnedExprSiteV1,
    destination: super::BindingRefV1,
) -> Result<crate::mir::resolved_semantics::SourceBindingSiteV1, &'static str> {
    let function = input.function();
    if destination.owner() != input.owner() || call.owner() != input.owner()
        || !matches!(function.binding(destination).map(|row| row.kind()), Some(super::BindingKindV1::Local { .. }))
        || super::sole_initializer_site(function, destination).as_ref() != Some(call.site()) {
        return Err("borrowed-nullable/local-initializer");
    }
    let initializer = function.expression_source().initializers().find(|row| row.binding() == destination)
        .ok_or("borrowed-nullable/local-declaration")?;
    if function.declaration_binding(initializer.declaration_site()) != Some(destination) {
        return Err("borrowed-nullable/local-declaration");
    }
    Ok(initializer.declaration_site().clone())
}

/// Ephemeral corroboration of existing source edges, not an admission receipt.
pub(super) struct CallWitnessSourceV1<'a> {
    output_site: &'a OwnedExprSiteV1,
    call_site: &'a OwnedExprSiteV1,
    key: &'a CanonicalSameModuleCallableKeyV1,
    actuals: &'a [ResultActualSourceV1],
}
impl<'a> CallWitnessSourceV1<'a> {
    pub(super) fn verify(
        output_site: &'a OwnedExprSiteV1,
        call_site: &'a OwnedExprSiteV1,
        key: &'a CanonicalSameModuleCallableKeyV1,
        actuals: &'a [ResultActualSourceV1],
    ) -> Option<Self> {
        if call_site.owner() != output_site.owner() || actuals.len() != key.arity() as usize {
            return None;
        }
        for (ordinal, actual) in actuals.iter().enumerate() {
            let mut expected = call_site.site().node().segments().to_vec();
            expected.push(
                crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(ordinal as u32),
            );
            if actual.site.owner() != call_site.owner()
                || actual.site.site().node().segments() != expected
            {
                return None;
            }
        }
        Some(Self {
            output_site,
            call_site,
            key,
            actuals,
        })
    }

    pub(super) fn compose(
        self,
        callee_rows: &[ResultExitOriginV1],
        parameter_contracts: &[OwnedCallableParameterContractDeclarationV1],
        batch_slot: u32,
    ) -> Option<Vec<Rc<ResultOriginWitnessV1>>> {
        let Self {
            output_site,
            call_site,
            key,
            actuals,
        } = self;
        let mut witnesses = Vec::new();
        for callee in callee_rows.iter().flat_map(|row| row.witnesses()) {
            let substitution = if let Some(ordinal) = callee.formal_ordinal() {
                let Some(actual) = actuals.get(ordinal as usize) else {
                    return None;
                };
                let Some(binding) = actual.binding else {
                    return None;
                };
                let Some(parameter) = parameter_contract(parameter_contracts, batch_slot, binding)
                else {
                    return None;
                };
                if !matches!(
                    parameter.kind,
                    CallableParameterContractKindV1::DeclaredObject(_)
                        | CallableParameterContractKindV1::OpaqueHandle
                ) {
                    return None;
                }
                Some(ResultFormalSubstitutionV1 {
                    argument_site: actual.site.clone(),
                    binding,
                    callee_ordinal: ordinal,
                    caller_ordinal: parameter.ordinal,
                })
            } else {
                None
            };
            let origin = match (callee.origin(), &substitution) {
                (ResultValueOriginV1::ForwardFormal { .. }, Some(row)) => {
                    ResultValueOriginV1::ForwardFormal {
                        ordinal: row.caller_ordinal,
                    }
                }
                (ResultValueOriginV1::ForwardFormal { .. }, None) => return None,
                (other, _) => other.clone(),
            };
            witnesses.push(std::rc::Rc::new(ResultOriginWitnessV1 {
                site: output_site.clone(),
                origin,
                step: ResultWitnessStepV1::Call {
                    site: call_site.clone(),
                    key: key.clone(),
                    callee: std::rc::Rc::clone(callee),
                    substitution,
                },
            }));
        }
        Some(witnesses)
    }
}
