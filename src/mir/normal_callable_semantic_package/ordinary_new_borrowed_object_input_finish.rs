//! Typed inputs close over the original inventory, Completion and signature.
//! This regenerates actuals, never a borrowed entry or a Normal return seal.
use super::*;
use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::{PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1},
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
    OrdinaryNewClaimLedgerV1,
};
use std::collections::BTreeSet;

#[cfg(test)]
#[path = "ordinary_new_borrowed_object_input_finish_tests.rs"]
mod tests;

impl OrdinaryNewClaimLedgerV1 {
    /// The same source input family serves producer and received-field users.
    pub(in crate::mir::normal_callable_semantic_package) fn has_original_object_source_input_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<bool, String> {
        let producer = source.original_object_producer_callee_v1(target)?;
        let nullable = self.has_original_owning_object_input_v1(source, target)?;
        Ok(producer || nullable)
    }
    /// Classify the complete original census without selecting execution.
    /// No entry, actual phase, Completion or result qualification is consulted.
    pub(in crate::mir::normal_callable_semantic_package) fn original_instance_input_kind_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<OriginalInstanceInputKindV1>, String> {
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("object-input/original-catalog-missing"))?;
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let contract = contracts
            .next()
            .ok_or_else(|| freeze("object-input/original-contract-missing"))?;
        if contracts.next().is_some() {
            return Err(freeze("object-input/original-contract-duplicate"));
        }
        if contract.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod {
            return Ok(None);
        }
        let mut kind = None;
        let calls = if Self::original_finite_instance_input_request_v1(source, catalog, owner)? {
            Self::original_instance_input_calls_v1(source, catalog, owner)?
        } else {
            source
                .source_incoming
                .exact_rows()
                .filter(|call| call.callee == owner)
                .cloned()
                .collect::<Vec<_>>()
                .into_boxed_slice()
        };
        for call in &calls {
            let target = call.source.require_instance()?;
            if target.call_site() != &call.call
                || target.callee_owner() != owner
                || target.target_batch_slot() != contract.batch_slot
                || target.argument_sites().len() != contract.parameters.len()
            {
                return Err(freeze("object-input/original-contract-drift"));
            }
            let current = if source.source_incoming.has_object_input_callee_v1(target) {
                OriginalInstanceInputKindV1::QualifiedObject
            } else {
                OriginalInstanceInputKindV1::SourceInstance
            };
            if kind.is_some_and(|kind| kind != current) {
                return Err(freeze("instance-input/original-family-drift"));
            }
            kind = Some(current);
        }
        Ok(kind)
    }

    /// Retain original read/scalar/Mixed input demand and stronger Object selection.
    pub(in crate::mir::normal_callable_semantic_package) fn selected_original_instance_input_kind_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<OriginalInstanceInputKindV1>, String> {
        let Some(kind) = self.original_instance_input_kind_v1(source, owner)? else {
            return Ok(None);
        };
        let contract = self
            .original_parameter_contracts
            .as_deref()
            .and_then(|catalog| catalog.iter().find(|row| row.owner == owner))
            .ok_or_else(|| freeze("object-input/original-contract-missing"))?;
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("object-input/original-catalog-missing"))?;
        let finite_input = Self::original_finite_instance_input_request_v1(source, catalog, owner)?;
        let mut source_result_request = source
            .original_returned_field_participant_request_v1(owner)?
            .is_some()
            || self.borrowed_i64_results.original_return_source_v1(&owner)?
                .is_some_and(|original| original.is_text_return_v1(source));
        // Owning selection checks the whole callee, including every target and
        // root; share that result only within this immutable selection scan.
        let mut owning_request = None;
        for call in source
            .source_incoming
            .exact_rows()
            .filter(|call| call.callee == owner)
        {
            let target = call.source.require_instance()?;
            source_result_request |= source.original_object_producer_source_v1(target)?.is_some()
                || source.original_mixed_input_source_v1(target)?.is_some()
                || match owning_request {
                    Some(owning) => owning,
                    None => {
                        let owning = self.has_original_owning_object_input_v1(source, target)?;
                        owning_request = Some(owning);
                        owning
                    }
                };
        }
        if kind == OriginalInstanceInputKindV1::SourceInstance
            && !source_result_request
            && !finite_input
            && !self.callable_scalar_result_participant_v1(owner)?
        {
            return Ok(None);
        }
        Ok(self
            .selected_object_input_contract_v1(source, contract)?
            .then_some(kind))
    }

    /// One input protocol for finishing, entry and completed-target demand.
    /// Source Instance qualification reuses the existing original entry laws;
    /// Object qualification keeps its stronger result/source requirements.
    pub(in crate::mir::normal_callable_semantic_package) fn selected_instance_input_protocol_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<OriginalInstanceInputKindV1>, String> {
        match self.original_instance_input_kind_v1(source, owner)? {
            Some(OriginalInstanceInputKindV1::QualifiedObject) => {
                self.selected_original_instance_input_kind_v1(source, owner)
            }
            Some(OriginalInstanceInputKindV1::SourceInstance) => {
                if self
                    .selected_original_instance_input_kind_v1(source, owner)?
                    .is_some()
                {
                    return Ok(Some(OriginalInstanceInputKindV1::SourceInstance));
                }
                if !self.selected_source_instance_input_owner_v1(source, owner)? {
                    return Ok(None);
                }
                Ok(self
                    .original_instance_input_requires_reconstruction_v1(source, owner)?
                    .then_some(OriginalInstanceInputKindV1::SourceInstance))
            }
            None => Ok(None),
        }
    }

    /// Original source membership selects reconstruction, never actual phase.
    pub(in crate::mir::normal_callable_semantic_package) fn original_instance_input_requires_reconstruction_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("object-input/original-catalog-missing"))?;
        if Self::original_finite_instance_input_request_v1(source, catalog, owner)? {
            Self::original_instance_input_calls_v1(source, catalog, owner)?;
            return Ok(true);
        }
        // exact_rows alone omits retained scanner errors and unresolved callers.
        source
            .source_incoming
            .project_declaration_v1(owner)
            .map_err(|issue| {
                format!(
                    "{}: {issue:?}",
                    freeze("instance-input/original-incoming-coverage")
                )
            })?;
        let originals: Vec<_> = source
            .source_incoming
            .exact_rows()
            .filter(|call| call.callee == owner)
            .collect();
        let transport: Vec<_> = source
            .incoming
            .iter()
            .filter(|call| call.callee == owner)
            .collect();
        let complete_transport = source.definitions.contains_key(&owner)
            && originals.len() == transport.len()
            && originals.iter().all(|original| {
                transport
                    .iter()
                    .filter(|call| {
                        call.call == original.call
                            && call.callee == original.callee
                            && call.arguments == original.arguments
                            && call.source.instance() == original.source.instance()
                    })
                    .count()
                    == 1
            });
        // The original source-only forward issuer owns these inputs even if
        // a transport row also retains the same target. Do not inspect its
        // current SourceInstance/ExecutableSourceInstance phase to select it.
        let source_forward = originals.iter().any(|call|
            source.source_only_definitions.get(&call.call.owner()).is_some_and(|draft|
                call.arguments.iter().any(|(ordinal, site, _)| draft.uses.iter().any(|row|
                    row.site.site() == site && draft.origins.get(&row.binding) == Some(&row.formal)
                        && matches!(&row.kind, super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::UnresolvedArgument { call: original, ordinal: original_ordinal }
                            if original == &call.call && original_ordinal == ordinal)))));
        // A complete legacy transport census does not mean a computed Core
        // argument has completed its original expression/temporary protocol.
        // Select reconstruction from that original demand, never current phase.
        let mut core_argument = false;
        for call in &originals {
            core_argument |= original_core_argument_input_request_v1(source, catalog, &call.call)?;
        }
        Ok(!complete_transport
            || source_forward
            || core_argument
            || Self::original_finite_instance_input_request_v1(source, catalog, owner)?)
    }

    /// Object protocols retain their original, stronger result/source qualification.
    pub(in crate::mir::normal_callable_semantic_package) fn selected_original_object_input_owner_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        // A source Instance cannot become an Object input by validating an
        // unrelated scalar-result graph. Select this query's original family first.
        if self.original_instance_input_kind_v1(source, owner)?
            != Some(OriginalInstanceInputKindV1::QualifiedObject)
        {
            return Ok(false);
        }
        Ok(
            self.selected_original_instance_input_kind_v1(source, owner)?
                == Some(OriginalInstanceInputKindV1::QualifiedObject),
        )
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_typed_object_input_actuals_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<(), String> {
        let Some(Ok(source)) = &self.borrowed_formal_source else {
            // No successful inventory to select. Preserve the original failure.
            return Ok(());
        };
        let owners: BTreeSet<_> = source
            .source_incoming
            .exact_rows()
            .filter(|row| {
                row.source
                    .instance()
                    .is_some_and(|target| target.has_object_source_requirement())
            })
            .map(|row| row.callee)
            .collect();
        for owner in owners {
            let mut matching = contracts.iter().filter(|row| row.owner == owner);
            let contract = matching
                .next()
                .ok_or_else(|| freeze("object-input/contract-missing"))?;
            if matching.next().is_some() {
                return Err(freeze("object-input/contract-duplicate"));
            }
            if !self.selected_object_input_contract_v1(source, contract)? {
                continue;
            }
            match self.prepare_original_instance_input_owner_v1(
                source,
                selected,
                contract,
                signatures,
                OriginalInstanceInputKindV1::QualifiedObject,
            )? {
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
                    for (site, actuals) in rows {
                        self.borrowed_formal_actuals.insert(site, Ok(actuals));
                    }
                }
            }
        }
        Ok(())
    }

    /// Demand-time recheck of the original completed input cohort.
    /// Returns original argument storage only; no result or Normal authority.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_typed_object_arguments_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        loan: &crate::mir::normal_callable_semantic_package::ObjectReturnCallQualificationV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<Option<&[LocalCallArgumentV1]>, String> {
        if loan.call() != target.call_site()
            || loan.key() != target.target()
            || self
                .callable_result_classes
                .object_return_qualification(loan.value())
                .as_ref()
                != Some(loan)
            || !target
                .object_return_sources()
                .is_some_and(|rows| rows.contains(loan))
        {
            return Err(freeze("object-input/return-qualification"));
        }
        self.checked_completed_typed_object_target_arguments_v1(
            target, selected, contracts, signatures,
        )
    }

    /// Borrows input evidence for the exact original target without granting result authority.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_typed_object_target_arguments_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<Option<&[LocalCallArgumentV1]>, String> {
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("object-input/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        if !self.selected_original_object_input_owner_v1(source, target.callee_owner())? {
            return Ok(None);
        }
        self.checked_completed_original_instance_target_arguments_v1(
            target, selected, contracts, signatures,
        )
    }

    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_original_instance_target_arguments_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<Option<&[LocalCallArgumentV1]>, String> {
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("object-input/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let mut matching = contracts
            .iter()
            .filter(|row| row.owner == target.callee_owner());
        let contract = matching
            .next()
            .ok_or_else(|| freeze("object-input/contract-missing"))?;
        if matching.next().is_some() {
            return Err(freeze("object-input/contract-duplicate"));
        }
        let Some(kind) = self.selected_instance_input_protocol_v1(source, contract.owner)? else {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[instance-input] owner={:?} no selected input protocol", contract.owner);
            }
            return Ok(None);
        };
        match self.prepare_original_instance_input_owner_v1(
            source, selected, contract, signatures, kind,
        )? {
            TypedInputClosureV1::Pending => {
                if crate::config::env::builder_debug_enabled() {
                    eprintln!("[instance-input] owner={:?} closure pending", contract.owner);
                }
                return Ok(None);
            }
            TypedInputClosureV1::Refused(issue) => return Err(issue),
            TypedInputClosureV1::Ready(rows) if !rows.is_empty() => {
                if crate::config::env::builder_debug_enabled() {
                    eprintln!("[instance-input] owner={:?} has {} unreplaced source actuals", contract.owner, rows.len());
                }
                return Ok(None);
            }
            TypedInputClosureV1::Ready(_) => {}
        }
        let call = Self::original_instance_input_call_v1(source, contracts, target.call_site())?
            .ok_or_else(|| freeze("object-input/original-row-missing"))?;
        if call.source.require_instance()? != target || call.callee != target.callee_owner() {
            return Err(freeze("object-input/original-target"));
        }
        let Some(actuals) = self.borrowed_formal_actuals.get(target.call_site()) else {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[instance-input] owner={:?} call={:?} has no staged actuals", contract.owner, target.call_site());
            }
            return Ok(None);
        };
        let actuals = actuals.as_ref().map_err(Clone::clone)?;
        actuals.ordered_arguments_for_v1(&call).map(Some)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn selected_object_input_contract_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> Result<bool, String> {
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("object-input/original-catalog-missing"))?;
        let mut originals = catalog.iter().filter(|row| row.owner == contract.owner);
        let original = originals
            .next()
            .ok_or_else(|| freeze("object-input/original-contract-missing"))?;
        if originals.next().is_some()
            || original.batch_slot != contract.batch_slot
            || original.mode != contract.mode
            || original.parameters.len() != contract.parameters.len()
            || original
                .parameters
                .iter()
                .zip(&contract.parameters)
                .any(|(source, observed)| {
                    source.ordinal != observed.ordinal
                        || source.binding != observed.binding
                        || source.kind != observed.kind
                })
        {
            return Err(freeze("object-input/original-contract-drift"));
        }
        if contract.parameters.iter().all(|formal| {
            matches!(formal.kind,
            CallableParameterContractKindV1::ExactTrivial(abi)
                if abi == crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64)
        }) {
            return Ok(true);
        }
        // Original result/read requests borrow the common parameter frame.
        // Mixed/Field stay SourceInstance; only the full input verifier can
        // close every incoming/Completion/signature and permit reconstruction.
        if Self::original_finite_instance_input_request_v1(source, catalog, contract.owner)? {
            if self.checked_input_use_source_v1(contract)?.is_some() {
                return Ok(true);
            }
            return self.source_signed_i64_parameter_frame_v1(source, contract);
        }
        let mut parameter_frame_request = source
            .original_returned_field_participant_request_v1(contract.owner)?
            .is_some();
        for call in source
            .source_incoming
            .exact_rows()
            .filter(|call| call.callee == contract.owner)
        {
            if let Some(target) = call.source.instance() {
                parameter_frame_request |= source.original_mixed_input_source_v1(target)?.is_some()
                    || self.has_original_owning_object_input_v1(source, target)?;
            }
        }
        if parameter_frame_request
            || self.original_instance_input_kind_v1(source, contract.owner)?
                == Some(OriginalInstanceInputKindV1::QualifiedObject)
        {
            if self.original_object_input_parameter_frame_v1(source, contract.owner)? {
                return Ok(true);
            }
            return self.source_signed_i64_parameter_frame_v1(source, contract);
        }
        if contract.parameters.is_empty()
            || contract
                .parameters
                .iter()
                .any(|formal| formal.kind != CallableParameterContractKindV1::OpaqueHandle)
        {
            return Ok(false);
        }
        // A globally admitted original Instance frame is already a source
        // qualification for the SAME full input owner. It needs no unrelated
        // caller's comparison guard to select its own opaque input cohort.
        if let Some(definition) = source.definitions.get(&contract.owner) {
            // Qualification reads the original frame directly. Asking a body
            // entry consumer here would recursively reselect its input family.
            if source.source_only_definitions.contains_key(&contract.owner)
                || contract.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod
                || definition
                    .origins
                    .values()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    != contract
                        .parameters
                        .iter()
                        .map(|formal| formal.binding)
                        .collect()
                || contract
                    .parameters
                    .iter()
                    .any(|formal| definition.origins.get(&formal.binding) != Some(&formal.binding))
            {
                return Err(freeze("object-input/original-frame-identity"));
            }
            return Ok(true);
        }
        for call in source
            .source_incoming
            .exact_rows()
            .filter(|row| row.callee == contract.owner)
        {
            if self
                .checked_guarded_instance_definition_v1(source, call.call.owner())?
                .is_some()
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// An opaque declaration can still have a complete scalar input frame when
    /// the retained whole-source SUM proves every original formal is signed I64.
    /// This lends source input qualification only; execution stays on the
    /// existing completed Instance/Object cohort.
    fn source_signed_i64_parameter_frame_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> Result<bool, String> {
        let Some(retained) = self.borrowed_formal_source.as_ref() else {
            return Ok(false);
        };
        let retained = retained.as_ref().map_err(Clone::clone)?;
        if !std::ptr::eq(retained, source) {
            return Err(freeze("object-input/source-sum-ledger-drift"));
        }
        if contract.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod
            || contract.parameters.is_empty()
            || contract.parameters.iter().any(|formal| {
                formal.binding.owner() != contract.owner
                    || !matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
                    || !source.lend_source_value_sum_v1(formal.binding)
                        .is_some_and(|sum| sum.signed_only())
            })
        {
            return Ok(false);
        }
        Ok(true)
    }

    /// Diagnose the same prerequisite gate used by preparation, without a retained receipt.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn typed_object_input_completion_diagnostic_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<String, String> {
        Ok(
            match self.checked_typed_object_input_completion_v1(owner)? {
                ObjectInputCompletionGateV1::Pending(reason) => {
                    format!("callee={owner:?}, pending={reason}")
                }
                ObjectInputCompletionGateV1::Refused(issue) => {
                    format!("callee={owner:?}, refused={issue}")
                }
                ObjectInputCompletionGateV1::Ready => {
                    format!("callee={owner:?}, completion-ready; input cohort still uncompleted")
                }
            },
        )
    }

    pub(super) fn checked_typed_object_input_completion_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<ObjectInputCompletionGateV1, String> {
        let Some(completion) = self.completion_index.get(&owner) else {
            return Ok(ObjectInputCompletionGateV1::Pending(
                "completion-missing".into(),
            ));
        };
        let completion = match completion {
            Ok(row) => row,
            Err(issue) => {
                return Ok(ObjectInputCompletionGateV1::Refused(format!(
                    "{}: {issue:?}",
                    freeze("object-input/callee-completion")
                )))
            }
        };
        if completion.owner() != owner {
            return Err(freeze("object-input/completion-owner"));
        }
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(ObjectInputCompletionGateV1::Pending(
                "root-flow-missing".into(),
            ));
        };
        let mut implicit_ready = false;
        if completion.implicit_body_end().is_some() {
            let ingress = self
                .borrowed_formal_source
                .as_ref()
                .ok_or_else(|| freeze("object-input/source-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            let catalog = self
                .original_parameter_contracts
                .as_deref()
                .ok_or_else(|| freeze("object-input/original-catalog-missing"))?;
            if Self::original_finite_instance_input_request_v1(ingress, catalog, owner)? {
                let incoming = Self::original_instance_input_calls_v1(ingress, catalog, owner)?;
                if incoming.is_empty()
                    || completion.returns_value()
                    || incoming.iter().any(|row| {
                        row.source
                            .unit_discard_source_v1()
                            .is_none_or(|original| !original.matches_unit_completion_v1(completion))
                    })
                {
                    return Err(freeze("unit-call/completion-source-drift"));
                }
                match flow.implicit_exit() {
                    None => {
                        return Ok(ObjectInputCompletionGateV1::Pending(
                            "implicit-exit-missing".into(),
                        ))
                    }
                    Some(Err(issue)) => {
                        return Ok(ObjectInputCompletionGateV1::Refused(format!(
                            "{}: callee={owner:?}, implicit-exit: {issue:?}",
                            freeze("object-input/callee-exit")
                        )))
                    }
                    Some(Ok(_)) => {
                        self.implicit_root_home_source_v1(owner, completion)?;
                        implicit_ready = true;
                    }
                }
            }
        }
        if completion.explicit_sites().is_empty() && !implicit_ready {
            return Ok(ObjectInputCompletionGateV1::Pending(
                "explicit-exits-empty".into(),
            ));
        }
        for exit in completion.explicit_sites() {
            match flow.exit_row(exit) {
                Some(Err(issue)) => {
                    return Ok(ObjectInputCompletionGateV1::Refused(format!(
                        "{}: callee={owner:?}, exit={exit:?}: {issue:?}",
                        freeze("object-input/callee-exit")
                    )))
                }
                None => {
                    return Ok(ObjectInputCompletionGateV1::Pending(format!(
                        "exit-row-missing: {exit:?}"
                    )))
                }
                Some(Ok(_)) => {}
            }
            if self.terminal_relation_for_owner_at(owner, exit).is_none() {
                return Ok(ObjectInputCompletionGateV1::Pending(format!(
                    "terminal-relation-missing: {exit:?}"
                )));
            }
        }
        Ok(ObjectInputCompletionGateV1::Ready)
    }

    pub(super) fn prepare_original_instance_input_owner_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contract: &OwnedCallableParameterContractDeclarationV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        kind: OriginalInstanceInputKindV1,
    ) -> Result<TypedInputClosureV1, String> {
        let owner = contract.owner;
        let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) =
            selected.key_for_batch_slot(contract.batch_slot)
        else {
            return Err(freeze("object-input/selected-key"));
        };
        if contract.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod
            || key.namespace()
                != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || key.arity() as usize != contract.parameters.len()
            || contract
                .parameters
                .iter()
                .enumerate()
                .any(|(ordinal, formal)| {
                    formal.ordinal as usize != ordinal || formal.binding.owner() != owner
                })
        {
            return Err(freeze("object-input/contract-identity"));
        }
        let catalog = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("object-input/original-catalog-missing"))?;
        let calls = match Self::original_instance_input_calls_v1(source, catalog, owner) {
            Ok(rows) => rows,
            Err(issue) => {
                return Ok(TypedInputClosureV1::Refused(format!(
                    "{issue}, target={key:?}"
                )))
            }
        };
        match self.checked_typed_object_input_completion_v1(owner)? {
            ObjectInputCompletionGateV1::Pending(_) => {
                return Ok(TypedInputClosureV1::Pending);
            }
            ObjectInputCompletionGateV1::Refused(issue) => {
                return Ok(TypedInputClosureV1::Refused(issue))
            }
            ObjectInputCompletionGateV1::Ready => {}
        }
        let Some(signature) = signatures.row(contract.batch_slot) else {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[instance-input] owner={owner:?} has no physical signature row");
            }
            return Ok(TypedInputClosureV1::Pending);
        };
        let identity = selected
            .identity_for_batch_slot(contract.batch_slot)
            .ok_or_else(|| freeze("object-input/signature-identity"))?;
        let lanes = signature.lanes();
        if !signature.identity().same_as(identity)
            || signature.owner() != owner
            || signature.mode() != contract.mode
            || signature.source_logical_arity() as usize != contract.parameters.len()
            || signature.receiver_lane_count() != 1
            || signature.physical_formal_lane_count() as usize != contract.parameters.len()
            || signature.physical_callable_lane_count() as usize != contract.parameters.len() + 1
            || lanes.len() != contract.parameters.len() + 1
            || lanes[0].role() != PhysicalCallableLaneRoleV1::InstanceReceiver
            || lanes[0].index() != 0
            || lanes[0].logical_ordinal().is_some()
            || lanes[0].binding().owner() != owner
            || lanes[1..]
                .iter()
                .zip(&contract.parameters)
                .any(|(lane, formal)| {
                    lane.role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
                        || lane.index() != formal.ordinal + 1
                        || lane.logical_ordinal() != Some(formal.ordinal)
                        || lane.binding() != formal.binding
                })
        {
            return Err(freeze("object-input/signature-identity"));
        }
        let mut missing = false;
        let mut replacements = Vec::new();
        for call in &calls {
            let target = call.source.require_instance()?;
            if call.callee != owner
                || target.callee_owner() != owner
                || target.target() != key
                || target.target_batch_slot() != contract.batch_slot
                || target.call_site() != &call.call
                || source.source_incoming.has_object_input_callee_v1(target)
                    != (kind == OriginalInstanceInputKindV1::QualifiedObject)
            {
                return Err(freeze("object-input/original-target"));
            }
            let Some(actuals) = self.borrowed_formal_actuals.get(&call.call) else {
                if crate::config::env::builder_debug_enabled() {
                    eprintln!("[instance-input] owner={owner:?} call={:?} has no staged actuals", call.call);
                }
                missing = true;
                continue;
            };
            let actuals = match actuals {
                Ok(rows) => rows,
                Err(issue) => return Ok(TypedInputClosureV1::Refused(issue.clone())),
            };
            match &actuals.phase {
                BorrowedCallActualEvidencePhaseV1::SourceObject(original)
                    if kind == OriginalInstanceInputKindV1::QualifiedObject =>
                {
                    if original.source != *target
                        || original.incoming_arguments != call.arguments
                        || !actuals.opaque_actuals.is_empty()
                    {
                        return Err(freeze("object-input/original-actuals"));
                    }
                    let catalog = self
                        .original_parameter_contracts
                        .as_deref()
                        .ok_or_else(|| freeze("object-input/original-catalog-missing"))?;
                    let Some(regenerated) = object_source::reconstruct_object_source_actuals_v1(
                        source,
                        catalog,
                        call,
                        original,
                        Some(self),
                    )?
                    else {
                        return Ok(TypedInputClosureV1::Pending);
                    };
                    if regenerated.ordered_arguments != actuals.ordered_arguments {
                        return Err(freeze("object-input/argument-snapshot-drift"));
                    }
                    replacements.push((call.call.clone(), regenerated));
                }
                BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(original)
                    if kind == OriginalInstanceInputKindV1::QualifiedObject =>
                {
                    let catalog = self
                        .original_parameter_contracts
                        .as_deref()
                        .ok_or_else(|| freeze("object-input/original-catalog-missing"))?;
                    object_source::recheck_object_source_actuals_v1(
                        source,
                        catalog,
                        call,
                        actuals,
                        original,
                        Some(self),
                    )?;
                    actuals.ordered_arguments_for_v1(call)?;
                }
                BorrowedCallActualEvidencePhaseV1::SourceInstance(original)
                    if kind == OriginalInstanceInputKindV1::SourceInstance =>
                {
                    let catalog = self
                        .original_parameter_contracts
                        .as_deref()
                        .ok_or_else(|| freeze("instance-input/original-catalog-missing"))?;
                    if !original.array_loans.is_empty() {
                        match self.prepare_original_array_producer_inputs_v1(
                            source, selected, signatures, original,
                        )? {
                            TypedInputClosureV1::Pending => {
                                return Ok(TypedInputClosureV1::Pending)
                            }
                            TypedInputClosureV1::Refused(issue) => {
                                return Ok(TypedInputClosureV1::Refused(issue))
                            }
                            TypedInputClosureV1::Ready(_) => {}
                        }
                    }
                    let Some(regenerated) =
                        instance_source::reconstruct_instance_source_actuals_v1(
                            source,
                            catalog,
                            call,
                            original,
                            Some(self),
                        )?
                    else {
                        if crate::config::env::builder_debug_enabled() {
                            eprintln!("[instance-input] owner={owner:?} call={:?} source actuals not reconstructed", call.call);
                        }
                        return Ok(TypedInputClosureV1::Pending);
                    };
                    if !actuals.opaque_actuals.is_empty()
                        || regenerated.ordered_arguments != actuals.ordered_arguments
                    {
                        return Err(freeze("instance-input/argument-snapshot-drift"));
                    }
                    replacements.push((call.call.clone(), regenerated));
                }
                BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(original)
                    if kind == OriginalInstanceInputKindV1::SourceInstance =>
                {
                    let catalog = self
                        .original_parameter_contracts
                        .as_deref()
                        .ok_or_else(|| freeze("instance-input/original-catalog-missing"))?;
                    instance_source::recheck_instance_source_actuals_v1(
                        source,
                        catalog,
                        call,
                        actuals,
                        original,
                        Some(self),
                    )?;
                    actuals.ordered_arguments_for_v1(call)?;
                }
                BorrowedCallActualEvidencePhaseV1::Executable => {
                    actuals.ordered_arguments_for_v1(call)?;
                }
                _ => return Err(freeze("object-input/actual-phase")),
            }
        }
        Ok(if missing {
            TypedInputClosureV1::Pending
        } else {
            TypedInputClosureV1::Ready(replacements)
        })
    }
}

/// An ephemeral preparation outcome, never a source/ABI/completion receipt.
pub(super) enum ObjectInputCompletionGateV1 {
    Ready,
    Pending(String),
    Refused(String),
}

/// Original result/source qualification is preserved while sharing input gates.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) enum OriginalInstanceInputKindV1 {
    QualifiedObject,
    SourceInstance,
}
