//! Original input-family classification and contract qualification queries.
//! These issue no actuals, Completion, or execution permission.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
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
            || original.parameters != contract.parameters
        {
            return Err(freeze("object-input/original-contract-drift"));
        }
        if contract.has_exact_i64_parameters_v1() {
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
                    || target.borrowed_nullable_local_source().is_some_and(|loan| loan.is_declared_read_v1())
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
}
