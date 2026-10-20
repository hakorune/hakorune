//! Source-pending lexical Object slots close atomically over original evidence.
use super::super::completion_index::ObjectReturnTeardownAvailabilityV1;
use super::*;
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::normal_callable_semantic_package::{
    model::OwnedCallableParameterContractDeclarationV1,
    result_contract::VerifiedCallableResultContractCohortV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    TerminalRelationV1, TerminalReturnedSourceV1,
};

#[path = "ordinary_new_lexical_object_observation.rs"]
mod observation;

#[path = "ordinary_new_object_packet_seal.rs"]
mod packet_seal;
pub(super) use packet_seal::ObjectPacketSealV1;

impl OrdinaryNewClaimLedgerV1 {
    /// Retained full-slot membership survives consumption; demand still rechecks
    /// the original source and inputs, never retrying the older protocol.
    pub(crate) fn uses_object_packet_protocol_v1(
        &self,
        row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<bool, String> {
        Ok(row.object_packet.is_some() || self.requires_object_packet_v1(row.source_target())?)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn has_original_owning_object_input_v1(
        &self, ingress: &PreparedBorrowedFormalIngressV1, target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<bool, String> {
        let loans = ingress.original_owning_object_input_callee_sources_v1(target)?;
        for loan in &loans {
            self.callable_result_classes.recheck_owning_object_local_input_roots_v1(loan).map_err(freeze)?;
        }
        Ok(!loans.is_empty())
    }

    /// Select the existing packet protocol from original source obligations.
    /// Guarded sibling input does not import another caller's result qualification;
    /// full-slot result/Completion/teardown and actual regeneration stay mandatory.
    pub(crate) fn requires_object_packet_v1(
        &self,
        target: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<bool, String> {
        if target.borrowed_nullable_local_source().is_some() || target.mixed_local_source().is_some() {
            return Ok(false);
        }
        if target.has_object_source_requirement() {
            return Ok(true);
        }
        // A checked numeric formal forward belongs to its original guarded
        // input owner. Losing that support cannot select nullable acquisition.
        // Text/typed forwards do not acquire this numeric guard requirement.
        if let Some(Ok(source)) = &self.borrowed_formal_source {
            let caller = target.call_site().owner();
            if target.is_self_receiver()
                && borrowed_formal_actuals::selected_source_only_object_forward_v1(source, target) {
                let numeric_forward = source.source_only_definitions.get(&caller).is_some_and(|draft| {
                    draft.uses.iter().any(|usage| {
                        if usage.site.owner() != caller
                            || draft.origins.get(&usage.binding) != Some(&usage.formal) { return false; }
                        let borrowed_formal_uses::BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal }
                            = &usage.kind else { return false; };
                        call == target.call_site()
                            && target.argument_sites().get(*ordinal as usize) == Some(usage.site.site())
                            && self.original_parameter_contracts.as_deref().is_some_and(|catalog|
                                catalog.iter().filter(|row| row.owner == caller)
                                    .flat_map(|row| &row.parameters).any(|parameter|
                                        parameter.binding == usage.formal
                                            && parameter.kind == CallableParameterContractKindV1::OpaqueHandle))
                            && draft.uses.iter().any(|guard| guard.formal == usage.formal
                                && matches!(guard.kind,
                                    borrowed_formal_uses::BorrowedFormalUseDraftKindV1::CompareOperand { .. }))
                    })
                });
                if numeric_forward {
                    return has_original_forwarding_receiver_input_source_v1(source,
                        self.original_parameter_contracts.as_deref(), caller, &mut |original| original == target);
                }
            }
        }
        if self.has_received_field_packet_source_v1(target)? { return Ok(true); }
        if self.has_original_receiver_nullable_packet_source_v1(target)? {
            return Ok(true);
        }
        let Some(Ok(source)) = &self.borrowed_formal_source else {
            return Ok(false);
        };
        if self.has_original_owning_object_input_v1(source, target)? { return Ok(true); }
        if source.original_object_producer_source_v1(target)?.is_some() {
            source.recheck_field_producer_source_v1(target, &self.callable_result_classes, &self.field_write_claims)?;
            return Ok(true);
        }
        if source.recheck_field_producer_callee_v1(target, &self.callable_result_classes, &self.field_write_claims)? {
            if let Some(observation) = self.handle_call_source(target.call_site()) {
                if observation.owner() != target.call_site().owner()
                    || source.object_source_target_at_v1(target.call_site())? != Some(target)
                    || observation.local_binding().is_none_or(|(_, destination)| destination.owner() != target.call_site().owner()) {
                    return Err(freeze("field-local/original-observation-drift"));
                }
                return Ok(true);
            }
        }
        if !target.is_self_receiver() {
            return Ok(false);
        }
        has_original_forwarding_receiver_input_source_v1(
            source,
            self.original_parameter_contracts.as_deref(),
            target.call_site().owner(),
            &mut |original| original == target,
        )
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish_object_lexical_slots_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let pending: Vec<_> = self
            .lexical_instance_calls
            .borrow()
            .iter()
            .filter_map(|(site, slot)| match slot {
                LexicalInstanceCallDispositionSlotV1::SourcePending(source) => {
                    Some((site.clone(), source.clone()))
                }
                _ => None,
            })
            .collect();
        let mut upgrades = Vec::new();
        for (site, source) in &pending {
            if site != source.call_site() {
                return Err(freeze("lexical-object/slot-source"));
            }
            if source.borrowed_nullable_local_source().is_some() || source.mixed_local_source().is_some() { continue; }
            if let Some((kind, route, teardown, field_supports)) = self
                .checked_object_lexical_slot_v1(source, selected, contracts, signatures, results)?
            {
                if let Some(packet) = ObjectPacketSealV1::retain(self, source, kind, teardown, field_supports)? {
                    upgrades.push((site.clone(), source.clone(), kind, route, packet));
                }
            }
        }
        // Recheck every original cell before changing any cell or route inventory.
        let mut slots = self.lexical_instance_calls.borrow_mut();
        for (site, original) in &pending {
            if !matches!(slots.get(site), Some(LexicalInstanceCallDispositionSlotV1::SourcePending(current)) if current == original)
            {
                return Err(freeze("lexical-object/slot-drift"));
            }
        }
        let mut routes = Vec::new();
        for (site, source, kind, route, packet) in upgrades {
            if route {
                routes.push(site.clone());
            }
            slots.insert(
                site,
                LexicalInstanceCallDispositionSlotV1::Ready(LexicalInstanceCallDispositionRowV1 {
                    source,
                    result: Some(kind),
                    scalar_result: None,
                    unit_completion: None,
                    borrowed_completion: None,
                    object_packet: Some(packet), mixed_result: None,
                }),
            );
        }
        drop(slots);
        for site in routes {
            self.record_lifecycle_local_call_site(site.owner(), site);
        }
        Ok(())
    }

    fn checked_object_lexical_slot_v1(
        &self,
        source: &LexicalInstanceCallSourceTargetV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<
        Option<(
            InvokeCallResultKind,
            bool,
            ObjectReturnTeardownAvailabilityV1,
            Box<[super::super::completion_index::ObjectCallFieldSupportRequirementV1]>,
        )>,
        String,
    > {
        if source.borrowed_receiver_source_v1().is_some() { return Ok(None); }
        if let Some(qualifications) = source.object_return_sources() {
            let original = self
                .callable_result_classes
                .qualifications_for_call(source.call_site(), source.target());
            if qualifications.is_empty() || qualifications != original.as_ref() {
                return Err(freeze("lexical-object/caller-qualification-identity"));
            }
        }
        if let Some(dependencies) = source.object_producer_dependencies() {
            if dependencies.is_empty()
                || self
                    .callable_result_classes
                    .object_return_dependencies(source.target(), source.callee_owner())
                    .as_deref()
                    != Some(dependencies)
            {
                return Err(freeze("lexical-object/producer-dependency-identity"));
            }
        }
        let mut matching = contracts
            .iter()
            .filter(|row| row.owner == source.callee_owner());
        let contract = matching
            .next()
            .ok_or_else(|| freeze("lexical-object/contract-missing"))?;
        if matching.next().is_some()
            || contract.batch_slot != source.target_batch_slot()
            || contract.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod
            || !matches!(selected.key_for_batch_slot(contract.batch_slot), Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == source.target())
            || contract.parameters.len() != source.argument_sites().len()
            || contract.parameters.len() != source.target().arity() as usize
        {
            return Err(freeze("lexical-object/contract-identity"));
        }
        let supported = contract.parameters.iter().all(|formal| {
            matches!(
                formal.kind,
                CallableParameterContractKindV1::ExactTrivial(
                    crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64
                )
            ) || formal.kind.is_ordinary_borrowed_handle()
        });
        let mut missing = false;
        if let Some(signature) = signatures.row(contract.batch_slot) {
            let identity = selected
                .identity_for_batch_slot(contract.batch_slot)
                .ok_or_else(|| freeze("lexical-object/signature-identity"))?;
            let lanes = signature.lanes();
            if !signature.identity().same_as(identity)
                || signature.owner() != contract.owner
                || signature.mode() != contract.mode
                || signature.source_logical_arity() as usize != contract.parameters.len()
                || signature.receiver_lane_count() != 1
                || lanes.first().is_none_or(|lane| {
                    lane.index() != 0
                        || lane.role() != PhysicalCallableLaneRoleV1::InstanceReceiver
                        || lane.logical_ordinal().is_some()
                        || lane.binding().owner() != contract.owner
                })
            {
                return Err(freeze("lexical-object/signature-identity"));
            }
            // This source input family admits single scalar lanes only.
            if contract.parameters.iter().all(|formal| {
                matches!(
                    formal.kind,
                    CallableParameterContractKindV1::ExactTrivial(
                        crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64
                    )
                ) || formal.kind.is_ordinary_borrowed_handle()
            }) {
                if signature.physical_formal_lane_count() as usize != contract.parameters.len()
                    || signature.physical_callable_lane_count() as usize
                        != contract.parameters.len() + 1
                    || lanes.len() != contract.parameters.len() + 1
                    || lanes[1..].iter().zip(&contract.parameters).enumerate().any(
                        |(ordinal, (lane, formal))| {
                            formal.ordinal as usize != ordinal
                                || formal.binding.owner() != contract.owner
                                || lane.index() != formal.ordinal + 1
                                || lane.role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
                                || lane.logical_ordinal() != Some(formal.ordinal)
                                || lane.binding() != formal.binding
                        },
                    )
                {
                    return Err(freeze("lexical-object/signature-geometry"));
                }
            } else {
                missing = true;
            }
        } else {
            missing = true;
        }
        // Excluded input families retain source requirements only. They do not
        // select executable Completion/exit demands for this bounded input owner.
        if !supported {
            return Ok(None);
        }
        if contract
            .parameters
            .iter()
            .any(|formal| formal.kind.is_ordinary_borrowed_handle())
        {
            let Some(ingress) = &self.borrowed_formal_source else {
                return Ok(None);
            };
            let ingress = ingress.as_ref().map_err(Clone::clone)?;
            if !ingress.definitions.contains_key(&contract.owner)
                && !(self.has_original_object_source_input_v1(ingress, source)?
                    && self.selected_original_instance_input_kind_v1(ingress, contract.owner)?.is_some()) {
                return Ok(None);
            }
        }
        let result = self.checked_object_callee_source_v1(source, results)?;
        missing |= result.is_none();
        let observation = self
            .checked_object_slot_observation_v1(source, result.as_ref().map(|source| source.kind))?;
        missing |= observation.is_none();
        if let Some(qualifications) = source.object_return_sources() {
            for loan in qualifications {
                let relations = self.terminal_relations_for_owner(loan.value().owner());
                let mut matching = relations.iter().filter_map(|relation| match relation {
                    TerminalRelationV1::Value(value)
                        if value.value_site() == loan.value().site() =>
                    {
                        Some(value)
                    }
                    _ => None,
                });
                let Some(value) = matching.next() else {
                    missing = true;
                    continue;
                };
                if matching.next().is_some() {
                    return Err(freeze("lexical-object/caller-terminal-duplicate"));
                }
                if !matches!(value.returned(), TerminalReturnedSourceV1::OwnedCall(original) if original.qualification() == loan)
                {
                    return Err(freeze("lexical-object/caller-terminal-qualification"));
                }
                missing |= self
                    .normal_exit_projection_v1(value.owner(), value.return_site())?
                    .is_none();
            }
        }
        let arguments = if contract
            .parameters
            .iter()
            .any(|formal| formal.kind.is_ordinary_borrowed_handle())
        {
            self.checked_completed_opaque_object_target_arguments_v1(source, contract)?
        } else if {
            let ingress = self.borrowed_formal_source.as_ref()
                .ok_or_else(|| freeze("object-input/original-ingress-missing"))?
                .as_ref().map_err(Clone::clone)?;
            self.has_original_object_source_input_v1(ingress, source)?
        } {
            self.checked_completed_original_instance_target_arguments_v1(
                source, selected, contracts, signatures,
            )?
        } else {
            self.checked_completed_typed_object_target_arguments_v1(
                source, selected, contracts, signatures,
            )?
        };
        missing |= arguments.is_none();
        if let Some(arguments) = arguments {
            if arguments.len() != source.argument_sites().len() {
                return Err(freeze("lexical-object/argument-cardinality"));
            }
        }
        Ok(if missing {
            None
        } else {
            result
                .zip(observation)
                .map(|(source, route)| (source.kind, route, source.teardown, source.field_supports))
        })
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Membership comes from the original incoming source requirement, even
    /// after the affine Ready row has been consumed. The original guarded/mixed
    /// forwarding frame joins this protocol; an Object failure never retries it.
    pub(crate) fn take_receiver_object_packet_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    ) -> Result<Option<LexicalInstanceCallDispositionRowV1>, String> {
        let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
        // A selected borrower cannot retry the owning receiver protocol or raw
        // emission. The shared affine take preserves its exact pending source.
        let borrowed_pending = matches!(self.lexical_instance_calls.borrow().get(&owned),
            Some(LexicalInstanceCallDispositionSlotV1::SourcePending(source))
                if source.borrowed_nullable_local_source().is_some());
        if borrowed_pending { return self.take_lexical_instance_call(owner, site); }
        let object_slot = match self.lexical_instance_calls.borrow().get(&owned) {
            Some(LexicalInstanceCallDispositionSlotV1::SourcePending(_)) => true,
            Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) => {
                row.object_packet.is_some()
                    || self.requires_object_packet_v1(row.source_target())?
            }
            Some(LexicalInstanceCallDispositionSlotV1::Taken) => true,
            None => false,
        };
        let ingress = match &self.borrowed_formal_source {
            Some(Ok(ingress)) => ingress,
            Some(Err(error)) if object_slot => return Err(error.clone()),
            None if object_slot => return Err(freeze("receiver-object/source-missing")),
            _ => return Ok(None),
        };
        let mut matching = ingress
            .source_incoming
            .exact_rows()
            .filter(|call| call.call == owned);
        let incoming = matching.next();
        if matching.next().is_some() {
            return Err(freeze("receiver-object/source-duplicate"));
        }
        let original = incoming.and_then(|call| call.source.instance());
        let original = match original {
            Some(source) if self.requires_object_packet_v1(source)? => Some(source),
            _ => None,
        };
        let Some(original) = original else {
            let slots = self.lexical_instance_calls.borrow();
            return match slots.get(&owned) {
                Some(LexicalInstanceCallDispositionSlotV1::SourcePending(_)) => {
                    Err(freeze("receiver-object/source-missing"))
                }
                Some(LexicalInstanceCallDispositionSlotV1::Ready(row))
                    if row.object_packet.is_some()
                        || self.requires_object_packet_v1(row.source_target())? =>
                {
                    Err(freeze("receiver-object/source-missing"))
                }
                Some(LexicalInstanceCallDispositionSlotV1::Taken) => {
                    Err(freeze("receiver-object/already-taken"))
                }
                _ => Ok(None),
            };
        };
        if !original.is_self_receiver()
            || original.call_site() != &owned
            || original.target() != key
        {
            return Err(freeze("receiver-object/source-identity"));
        }
        // A terminal Direct producer is not a received local acquisition.
        // Its exit owner, including unsupported teardown, keeps that boundary.
        if self
            .handle_call_source(&owned)
            .or_else(|| self.nullable_call_source(&owned))
            .is_none()
            || self.receiver_call_observation(&owned).is_none()
        {
            return Err(freeze("receiver-object/local-source-missing"));
        }
        let row = self
            .take_lexical_instance_call(owner, site)?
            .ok_or_else(|| freeze("receiver-object/packet-unavailable"))?;
        if row.source_target() != original {
            return Err(freeze("receiver-object/packet-source-drift"));
        }
        Ok(Some(row))
    }
}

#[cfg(test)]
#[path = "ordinary_new_lexical_object_slot_tests.rs"]
mod tests;

/// Query the original qualified self-input obligations before Completion or packet
/// issuance. The callback selects a source target, never executable authority.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn has_original_forwarding_receiver_input_source_v1(
    source: &PreparedBorrowedFormalIngressV1,
    catalog: Option<&[OwnedCallableParameterContractDeclarationV1]>,
    owner: FunctionOwnerIdV1,
    select: &mut impl FnMut(&LexicalInstanceCallSourceTargetV1) -> bool,
) -> Result<bool, String> {
    let Some(draft) =
        borrowed_formal_entry::checked_forwarding_instance_source_v1(source, catalog, owner)?
    else {
        return Ok(false);
    };
    for usage in &draft.uses {
        let borrowed_formal_uses::BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal } =
            &usage.kind
        else {
            continue;
        };
        let mut rows = source
            .source_incoming
            .exact_rows()
            .filter(|row| row.call == *call);
        let original = rows
            .next()
            .ok_or_else(|| freeze("receiver-object/guarded-source-missing"))?;
        if rows.next().is_some() || original.call.owner() != owner
            || original.source.argument_sites().get(*ordinal as usize) != Some(usage.site.site())
            || original.arguments.iter().filter(|(index, site, formal)|
                index == ordinal && site == usage.site.site() && formal.owner() == original.callee).count() != 1
        {
            return Err(freeze("receiver-object/guarded-source-identity"));
        }
        let Some(target) = original.source.instance() else { continue; };
        if target.is_self_receiver()
            && source.source_incoming.has_object_input_callee_v1(target) && select(target) {
            return Ok(true);
        }
    }
    Ok(false)
}
