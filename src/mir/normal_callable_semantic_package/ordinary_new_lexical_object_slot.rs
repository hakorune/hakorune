//! Source-pending lexical Object slots close atomically over original evidence.
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
            if let Some((kind, route)) = self
                .checked_object_lexical_slot_v1(source, selected, contracts, signatures, results)?
            {
                let packet = ObjectPacketSealV1::retain(self, source, kind)?;
                upgrades.push((site.clone(), source.clone(), kind, route, packet));
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
                    object_packet: Some(packet),
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
    ) -> Result<Option<(InvokeCallResultKind, bool)>, String> {
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
            if !ingress.definitions.contains_key(&contract.owner) {
                return Ok(None);
            }
        }
        let result = self.checked_object_callee_result_v1(source, results)?;
        missing |= result.is_none();
        let observation = self.checked_object_slot_observation_v1(source, result)?;
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
            result.zip(observation)
        })
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Membership comes from the original incoming source requirement, even
    /// after the affine Ready row has been consumed. ExistingBorrowedResult
    /// keeps its older receiver protocol; an Object failure never retries it.
    pub(crate) fn take_receiver_object_packet_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    ) -> Result<Option<LexicalInstanceCallDispositionRowV1>, String> {
        let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
        let object_slot = match self.lexical_instance_calls.borrow().get(&owned) {
            Some(LexicalInstanceCallDispositionSlotV1::SourcePending(source)) => {
                source.has_object_source_requirement()
            }
            Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) => {
                row.source_target().has_object_source_requirement()
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
        let original = incoming
            .and_then(|call| call.source.instance())
            .filter(|source| source.has_object_source_requirement());
        let Some(original) = original else {
            let slots = self.lexical_instance_calls.borrow();
            return match slots.get(&owned) {
                Some(LexicalInstanceCallDispositionSlotV1::SourcePending(source))
                    if source.has_object_source_requirement() =>
                {
                    Err(freeze("receiver-object/source-missing"))
                }
                Some(LexicalInstanceCallDispositionSlotV1::Ready(row))
                    if row.source_target().has_object_source_requirement() =>
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
