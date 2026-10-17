//! Finish the original non-Object Instance input cohort, without result authority.
use super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1;
use super::object_input_finish::OriginalInstanceInputKindV1;
use super::*;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1, OrdinaryNewClaimLedgerV1,
};

use super::super::borrowed_formal_source::{ArrayElementSourceLoanV1, ArrayHolderLifecycleRecipeV1};
use super::super::super::completion_index::ObjectReturnTeardownDescriptorV1;

enum ArrayProducerReadinessV1 {
    Pending,
    Refused(String),
    Ready(ObjectReturnTeardownDescriptorV1),
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn retain_original_instance_input_issuance_v1(
        &mut self,
        selected: std::rc::Rc<VerifiedSelectedCallableBatchMapV1>,
        signatures: std::rc::Rc<VerifiedCallablePhysicalSignatureCohortV1>,
    ) -> Result<(), String> {
        if self.original_instance_input_issuance.is_some() {
            return Err(freeze("instance-input/duplicate-original-issuance"));
        }
        self.original_instance_input_issuance = Some((selected, signatures));
        Ok(())
    }

    /// Demand all original producers through the existing whole-input owner.
    /// Proposed rows are not installed and lend no Array execution permission.
    pub(super) fn prepare_original_array_producer_inputs_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        identity: &instance_source::InstanceSourceActualIdentityV1,
    ) -> Result<TypedInputClosureV1, String> {
        let elements: Vec<_> = identity.array_loans.iter().map(|(_, element)| element).collect();
        self.prepare_array_holder_producer_inputs_v1(source, selected, signatures, &elements)
    }

    fn prepare_array_holder_producer_inputs_v1(
        &self, source: &PreparedBorrowedFormalIngressV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        elements: &[&ArrayElementSourceLoanV1],
    ) -> Result<TypedInputClosureV1, String> {
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("array-producer-input/original-catalog-missing"))?;
        let mut owners = std::collections::BTreeSet::new();
        let mut pending = false;
        let mut proposed = Vec::new();
        for element in elements {
            let inputs = element.original_producer_inputs_v1(source, catalog)?;
            for (producer, (original, contract)) in element.producer_sources_v1().zip(inputs) {
                let target = producer.source();
                self.callable_result_classes
                    .recheck_object_call_roots_v1(target, producer)
                    .map_err(|issue| freeze(&format!("array-producer-input/{issue}")))?;
                // Validate each exact target before sharing its full callee cohort.
                if !owners.insert(original.callee) {
                    continue;
                }
                for call in source
                    .source_incoming
                    .exact_rows()
                    .filter(|call| call.callee == original.callee)
                {
                    if let Some(Ok(actuals)) = self.borrowed_formal_actuals.get(&call.call) {
                        if matches!(&actuals.phase, BorrowedCallActualEvidencePhaseV1::SourceInstance(row)
                            | BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(row) if !row.array_loans.is_empty())
                        {
                            return Err(freeze(
                                "array-producer-input/nested-array-input-unconnected",
                            ));
                        }
                    }
                }
                let kind = if source.source_incoming.has_object_input_callee_v1(target) {
                    OriginalInstanceInputKindV1::QualifiedObject
                } else {
                    OriginalInstanceInputKindV1::SourceInstance
                };
                match self.prepare_original_instance_input_owner_v1(
                    source, selected, contract, signatures, kind,
                )? {
                    TypedInputClosureV1::Pending => pending = true,
                    TypedInputClosureV1::Refused(issue) => {
                        return Ok(TypedInputClosureV1::Refused(issue))
                    }
                    TypedInputClosureV1::Ready(rows) => proposed.extend(rows),
                }
            }
        }
        Ok(if pending {
            TypedInputClosureV1::Pending
        } else {
            TypedInputClosureV1::Ready(proposed)
        })
    }

    /// Before the sole Normal seal this remains Pending. Afterwards EVERY
    /// original producer must have the same completed source-issued End.
    fn prepare_original_array_producer_results_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        identity: &instance_source::InstanceSourceActualIdentityV1,
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) -> Result<TypedInputClosureV1, String> {
        match self
            .prepare_original_array_producer_inputs_v1(source, selected, signatures, identity)?
        {
            TypedInputClosureV1::Ready(rows) if rows.is_empty() => {}
            // Proposed producer rows are not installed by this demand.
            TypedInputClosureV1::Ready(_) | TypedInputClosureV1::Pending => {
                return Ok(TypedInputClosureV1::Pending)
            }
            TypedInputClosureV1::Refused(issue) => return Ok(TypedInputClosureV1::Refused(issue)),
        }
        let (original_selected, original_signatures) = self
            .original_instance_input_issuance
            .as_ref()
            .ok_or_else(|| freeze("array-producer-plan/original-issuance-missing"))?;
        if !std::ptr::eq(selected, original_selected.as_ref())
            || !std::ptr::eq(signatures, original_signatures.as_ref())
        {
            return Err(freeze("array-producer-plan/original-issuance-drift"));
        }
        let mut plans = Vec::new();
        let mut pending = false;
        let mut seen: Vec<&ArrayElementSourceLoanV1> = Vec::new();
        for (_, element) in &identity.array_loans {
            if seen.iter().any(|previous| element.same_holder_v1(previous)) { continue; }
            seen.push(element);
            match self.checked_array_holder_producers_v1(source, selected, signatures, element, results)? {
                ArrayProducerReadinessV1::Pending => pending = true,
                ArrayProducerReadinessV1::Refused(issue) => return Ok(TypedInputClosureV1::Refused(issue)),
                ArrayProducerReadinessV1::Ready(descriptor) => plans.push((element, descriptor)),
            }
        }
        if pending { return Ok(TypedInputClosureV1::Pending); }
        for (element, descriptor) in &plans {
            element.check_producer_plan_v1(descriptor, original_selected, original_signatures)?;
        }
        for (element, descriptor) in &plans {
            element.retain_checked_producer_plan_v1(descriptor.clone(), original_selected, original_signatures)?;
        }
        let mut recipes = Vec::new();
        for (element, _) in plans {
            let Some(completion) = self.completion_index.get(&element.read().owner()) else {
                return Ok(TypedInputClosureV1::Pending);
            };
            let completion = completion.as_ref().map_err(|issue| {
                format!("{}: {issue:?}", freeze("array-home/completion"))
            })?;
            let caller = self.array_holder_caller_v1(element.read().owner(), original_selected)?;
            let Some(recipe) = ArrayHolderLifecycleRecipeV1::issue(
                element, source, completion, caller, original_selected, original_signatures)?
                else { return Ok(TypedInputClosureV1::Pending); };
            recipes.push(recipe);
        }
        // Preflight the complete cohort before publishing any source obligation.
        let mut retained = self.array_holder_lifecycles.borrow_mut();
        for recipe in &recipes {
            if retained.get(&recipe.holder()).is_some_and(|old| !old.same_source(recipe)) {
                return Err(freeze("array-lifecycle/original-recipe-drift"));
            }
        }
        for recipe in recipes { retained.entry(recipe.holder()).or_insert(recipe); }
        Ok(TypedInputClosureV1::Ready(Vec::new()))
    }

    /// Pure whole-producer validation, shared by preparation and later demand.
    /// It never installs a recipe or lends an executable actual.
    fn checked_array_holder_producers_v1(
        &self, source: &PreparedBorrowedFormalIngressV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        element: &ArrayElementSourceLoanV1,
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) -> Result<ArrayProducerReadinessV1, String> {
        match self.prepare_array_holder_producer_inputs_v1(source, selected, signatures, &[element])? {
            TypedInputClosureV1::Ready(rows) if rows.is_empty() => {},
            TypedInputClosureV1::Ready(_) | TypedInputClosureV1::Pending => return Ok(ArrayProducerReadinessV1::Pending),
            TypedInputClosureV1::Refused(issue) => return Ok(ArrayProducerReadinessV1::Refused(issue)),
        }
        let current = ArrayElementSourceLoanV1::issue(source, element.read())?
            .ok_or_else(|| freeze("array-producer-plan/current-source-missing"))?;
        if &current != element { return Err(freeze("array-producer-plan/current-source-drift")); }
        let mut descriptor = None;
        let mut pending = false;
        for producer in element.producer_sources_v1() {
            let Some(current) = self.checked_array_producer_teardown_v1(producer, results)? else {
                pending = true;
                continue;
            };
            if descriptor.as_ref().is_some_and(|previous| previous != &current) {
                return Err(freeze("array-producer-result/teardown-disagreement"));
            }
            descriptor = Some(current);
        }
        if pending { return Ok(ArrayProducerReadinessV1::Pending); }
        Ok(ArrayProducerReadinessV1::Ready(descriptor
            .ok_or_else(|| freeze("array-producer-plan/empty-original-producers"))?))
    }

    /// AppMain is not a selected child result row. Lend its SAME source key
    /// for root holder retention; this issues no completed result contract.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_root_array_holder_caller_v1(
        &self, owner: FunctionOwnerIdV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
    ) -> Result<Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1>, String> {
        let source = self.borrowed_formal_source.as_ref()
            .ok_or_else(|| freeze("array-lifecycle/original-ingress-missing"))?
            .as_ref().map_err(Clone::clone)?;
        if source.app_main_owner != Some(owner) { return Ok(None); }
        let (original_selected, _) = self.original_instance_input_issuance.as_ref()
            .ok_or_else(|| freeze("array-lifecycle/original-issuance-missing"))?;
        let original_contracts = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("array-lifecycle/original-catalog-missing"))?;
        if !std::ptr::eq(selected, original_selected.as_ref())
            || !std::ptr::eq(contracts, original_contracts) {
            return Err(freeze("array-lifecycle/original-issuance-drift"));
        }
        self.array_holder_caller_v1(owner, original_selected).map(Some)
    }

    fn array_holder_caller_v1<'a>(
        &'a self, owner: FunctionOwnerIdV1, selected: &'a VerifiedSelectedCallableBatchMapV1,
    ) -> Result<&'a hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, String> {
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("array-lifecycle/original-catalog-missing"))?;
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let contract = contracts.next().ok_or_else(|| freeze("array-lifecycle/caller-missing"))?;
        if contracts.next().is_some() { return Err(freeze("array-lifecycle/caller-duplicate")); }
        match selected.key_for_batch_slot(contract.batch_slot) {
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) => Ok(key),
            None => {
                let source = self.borrowed_formal_source.as_ref()
                    .ok_or_else(|| freeze("array-lifecycle/original-ingress-missing"))?
                    .as_ref().map_err(Clone::clone)?;
                let root = self.root_completion.as_ref()
                    .ok_or_else(|| freeze("array-lifecycle/root-completion-missing"))?
                    .as_ref().map_err(|issue| format!("{}: {issue:?}", freeze("array-lifecycle/root-completion")))?;
                let indexed = self.completion_index.get(&owner)
                    .ok_or_else(|| freeze("array-lifecycle/root-index-missing"))?
                    .as_ref().map_err(|issue| format!("{}: {issue:?}", freeze("array-lifecycle/root-index")))?;
                let key = self.app_main_catalog_key.as_ref()
                    .ok_or_else(|| freeze("array-lifecycle/caller-key-unavailable"))?;
                if source.app_main_owner != Some(owner) || root.owner() != owner
                    || !std::rc::Rc::ptr_eq(root, indexed) || self.app_main_identity.is_none()
                    || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod
                    || key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod
                    || key.arity() as usize != contract.parameters.len()
                {
                    return Err(freeze("array-lifecycle/root-caller-identity"));
                }
                Ok(key)
            }
            _ => Err(freeze("array-lifecycle/caller-key-unavailable")),
        }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn owned_array_lifecycle_recipes_v1(
        &self,
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) -> Result<Vec<std::rc::Rc<ArrayHolderLifecycleRecipeV1>>, String> {
        let recipes: Vec<_> = self.array_holder_lifecycles.borrow().values().cloned().collect();
        if recipes.is_empty() { return Ok(recipes); }
        let source = self.borrowed_formal_source.as_ref()
            .ok_or_else(|| freeze("array-lifecycle/original-ingress-missing"))?.as_ref().map_err(Clone::clone)?;
        let (selected, signatures) = self.original_instance_input_issuance.as_ref()
            .ok_or_else(|| freeze("array-lifecycle/original-issuance-missing"))?;
        for recipe in &recipes {
            let completion = self.completion_index.get(&recipe.owner())
                .ok_or_else(|| freeze("array-lifecycle/current-completion-missing"))?.as_ref()
                .map_err(|issue| format!("{}: {issue:?}", freeze("array-lifecycle/current-completion")))?;
            recipe.recheck(source, completion, selected, signatures)?;
            if recipe.caller() != self.array_holder_caller_v1(recipe.owner(), selected)? {
                return Err(freeze("array-lifecycle/current-caller-drift"));
            }
            match self.checked_array_holder_producers_v1(source, selected, signatures, recipe.source(), results)? {
                ArrayProducerReadinessV1::Ready(descriptor) => recipe.source().check_producer_plan_v1(&descriptor, selected, signatures)?,
                ArrayProducerReadinessV1::Pending => return Err(freeze("array-lifecycle/producer-unfinished")),
                ArrayProducerReadinessV1::Refused(issue) => return Err(issue),
            }
        }
        Ok(recipes)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_original_instance_input_actuals_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = &self.borrowed_formal_source else {
            return Ok(());
        };
        // Original Instance membership is stable when an actual becomes executable.
        // Rechecking a completed owner yields Ready(empty), never new input permission.
        let owners: std::collections::BTreeSet<_> = source.source_incoming.exact_rows()
            .filter_map(|call| call.source.instance().filter(|target|
                !source.source_incoming.has_object_input_callee_v1(target)).map(|_| call.callee))
            .collect();
        for owner in owners {
            // The original global/guarded/mixed Instance frame selects input only.
            // It lends no body, field, result, cleanup or publication permission.
            if self.selected_instance_input_protocol_v1(source, owner)?
                != Some(OriginalInstanceInputKindV1::SourceInstance)
            {
                continue;
            }
            let mut matching = contracts.iter().filter(|row| row.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("instance-input/contract-missing"))?;
            if matching.next().is_some() {
                return Err(freeze("instance-input/contract-duplicate"));
            }
            // Finish non-Array inputs before Normal; keep Array rows source-only
            // until all original producer inputs AND the sole Normal seal agree.
            let mut array_pending = false;
            for row in self.borrowed_formal_actuals.values() {
                let Ok(PreparedBorrowedCallActualsV1 {
                    phase: BorrowedCallActualEvidencePhaseV1::SourceInstance(identity),
                    ..
                }) = row
                else {
                    continue;
                };
                if identity.source.callee_owner() != owner || identity.array_loans.is_empty() {
                    continue;
                }
                match self.prepare_original_array_producer_results_v1(
                    source, selected, signatures, identity, results,
                )? {
                    TypedInputClosureV1::Pending => array_pending = true,
                    TypedInputClosureV1::Refused(issue) => return Err(issue),
                    TypedInputClosureV1::Ready(_) => {}
                }
            }
            if array_pending {
                continue;
            }
            let closure = self.prepare_original_instance_input_owner_v1(
                source,
                selected,
                contract,
                signatures,
                OriginalInstanceInputKindV1::SourceInstance,
            )?;
            match closure {
                TypedInputClosureV1::Pending => {}
                TypedInputClosureV1::Refused(issue) => {
                    for call in source
                        .source_incoming
                        .exact_rows()
                        .filter(|row| row.callee == owner)
                    {
                        if let Some(row) = self.borrowed_formal_actuals.get_mut(&call.call) {
                            if row.is_ok() {
                                *row = Err(issue.clone());
                            }
                        }
                    }
                }
                TypedInputClosureV1::Ready(rows) => {
                    // The shared prerequisite gate and every original row have
                    // succeeded before any row of this cohort is installed.
                    for (site, actuals) in rows {
                        self.borrowed_formal_actuals.insert(site, Ok(actuals));
                    }
                }
            }
        }
        Ok(())
    }
}

/// Demand uses the same immutable mapping/signature and full preparation gates.
/// A ready row beside a pending sibling never lends an executable input.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn recheck_finished_instance_input_v1(
    ledger: &OrdinaryNewClaimLedgerV1,
    source: &PreparedBorrowedFormalIngressV1,
    catalog: &[OwnedCallableParameterContractDeclarationV1],
    original: &BorrowedIncomingCallDraftV1,
    actuals: &PreparedBorrowedCallActualsV1,
) -> Result<(), String> {
    let owner = original.callee;
    let mut contracts = catalog.iter().filter(|row| row.owner == owner);
    let contract = contracts
        .next()
        .ok_or_else(|| freeze("instance-input/contract-missing"))?;
    if contracts.next().is_some()
        || ledger.selected_instance_input_protocol_v1(source, owner)?
            != Some(OriginalInstanceInputKindV1::SourceInstance)
    {
        return Err(freeze("instance-input/contract-identity"));
    }
    let (selected, signatures) = ledger
        .original_instance_input_issuance
        .as_ref()
        .ok_or_else(|| freeze("instance-input/original-issuance-missing"))?;
    match ledger.prepare_original_instance_input_owner_v1(
        source,
        selected,
        contract,
        signatures,
        OriginalInstanceInputKindV1::SourceInstance,
    )? {
        TypedInputClosureV1::Ready(rows) if rows.is_empty() => {}
        TypedInputClosureV1::Refused(issue) => return Err(issue),
        _ => return Err(freeze("instance-input/cohort-unfinished")),
    }
    let mut matching = source
        .source_incoming
        .exact_rows()
        .filter(|row| row.call == original.call);
    if !matching.next().is_some_and(|row| {
        row.callee == original.callee
            && row.arguments == original.arguments
            && row
                .source
                .instance()
                .is_some_and(|target| Some(target) == original.source.instance())
    }) || matching.next().is_some()
        || ledger
            .borrowed_formal_actuals
            .get(&original.call)
            .and_then(|row| row.as_ref().ok())
            != Some(actuals)
    {
        return Err(freeze("instance-input/selected-snapshot"));
    }
    let BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity) = &actuals.phase
    else {
        return Err(freeze("instance-input/source-phase"));
    };
    instance_source::recheck_instance_source_actuals_v1(
        source, catalog, original, actuals, identity, Some(ledger),
    )
}

/// A finished original Instance input lends only its exact guarded reverse edge.
pub(in crate::mir::normal_callable_semantic_package) fn revalidate_guarded_instance_incoming_v1(
    ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    source: &PreparedBorrowedFormalIngressV1,
    catalog: &[OwnedCallableParameterContractDeclarationV1],
    call: &BorrowedIncomingCallDraftV1,
    rows: &PreparedBorrowedCallActualsV1,
) -> Result<bool, String> {
    if !matches!(
        rows.phase,
        BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(_)
    ) {
        return Ok(false);
    }
    recheck_finished_instance_input_v1(ledger, source, catalog, call, rows)?;
    Ok(rows.opaque_actuals.iter().any(|actual| {
        matches!(actual.source,
        BorrowedFormalActualSourceV1::Forwarded { binding, formal }
            if source.source_only_definitions.get(&call.call.owner())
                .is_some_and(|draft| draft.origins.get(&binding) == Some(&formal))
                && source.guarded_actuals.contains_key(&(call.call.clone(), actual.ordinal)))
    }))
}
