//! Package-owned logical-to-physical callable lane mapping.
//!
//! This product is deliberately pre-Builder: it contains source identity,
//! declaration mode, bindings, and lane roles, but no `ValueId`, target
//! address, runtime lease, or Completion.  The package issuer is the only
//! constructor; install/Port only lend its rows.

use std::collections::BTreeSet;

use crate::mir::builder::{SameModuleCallableCatalogBrandV1, SelectedCallableConsumptionRoleV1};
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::callable_semantic_batch::{
    ResolvedCallableDeclarationModeV1, ResolvedCallableSemanticBatchLoanErrorV1,
    VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, SourceBindingSiteV1,
};
use crate::parser::CallableDeclarationIdentityV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PhysicalCallableLaneRoleV1 {
    InstanceReceiver,
    OrdinaryScalar,
    ExactTextSlot,
    ExactTextGeneration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PhysicalCallableLaneV1 {
    index: u32,
    role: PhysicalCallableLaneRoleV1,
    logical_ordinal: Option<u32>,
    binding: BindingRefV1,
}

impl PhysicalCallableLaneV1 {
    pub(crate) const fn index(self) -> u32 {
        self.index
    }

    pub(crate) const fn role(self) -> PhysicalCallableLaneRoleV1 {
        self.role
    }

    pub(crate) const fn logical_ordinal(self) -> Option<u32> {
        self.logical_ordinal
    }

    pub(crate) const fn binding(self) -> BindingRefV1 {
        self.binding
    }
}

#[derive(Debug)]
pub(super) struct VerifiedCallablePhysicalSignatureRowV1 {
    batch_slot: u32,
    identity: CallableDeclarationIdentityV1,
    owner: FunctionOwnerIdV1,
    mode: CallableParameterDeclarationModeV1,
    role: SelectedCallableConsumptionRoleV1,
    receiver: Option<BindingRefV1>,
    source_logical_arity: u32,
    receiver_lane_count: u32,
    physical_formal_lane_count: u32,
    physical_callable_lane_count: u32,
    lanes: Box<[PhysicalCallableLaneV1]>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PhysicalCallableSignatureRowRefV1<'a> {
    row: &'a VerifiedCallablePhysicalSignatureRowV1,
}

impl<'a> PhysicalCallableSignatureRowRefV1<'a> {
    pub(crate) const fn batch_slot(self) -> u32 {
        self.row.batch_slot
    }

    pub(crate) fn identity(self) -> &'a CallableDeclarationIdentityV1 {
        &self.row.identity
    }

    pub(crate) const fn owner(self) -> FunctionOwnerIdV1 {
        self.row.owner
    }

    pub(crate) const fn mode(self) -> CallableParameterDeclarationModeV1 {
        self.row.mode
    }

    pub(crate) const fn role(self) -> SelectedCallableConsumptionRoleV1 {
        self.row.role
    }

    pub(crate) const fn receiver(self) -> Option<BindingRefV1> {
        self.row.receiver
    }

    pub(crate) const fn source_logical_arity(self) -> u32 {
        self.row.source_logical_arity
    }

    pub(crate) const fn receiver_lane_count(self) -> u32 {
        self.row.receiver_lane_count
    }

    pub(crate) const fn physical_formal_lane_count(self) -> u32 {
        self.row.physical_formal_lane_count
    }

    pub(crate) const fn physical_callable_lane_count(self) -> u32 {
        self.row.physical_callable_lane_count
    }

    pub(crate) fn lanes(self) -> &'a [PhysicalCallableLaneV1] {
        &self.row.lanes
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedCallablePhysicalSignatureCohortV1 {
    brand: SameModuleCallableCatalogBrandV1,
    rows: Box<[VerifiedCallablePhysicalSignatureRowV1]>,
}

impl VerifiedCallablePhysicalSignatureCohortV1 {
    pub(crate) fn brand(&self) -> &SameModuleCallableCatalogBrandV1 {
        &self.brand
    }

    pub(crate) fn rows(
        &self,
    ) -> impl ExactSizeIterator<Item = PhysicalCallableSignatureRowRefV1<'_>> {
        self.rows
            .iter()
            .map(|row| PhysicalCallableSignatureRowRefV1 { row })
    }

    pub(crate) fn row(&self, batch_slot: u32) -> Option<PhysicalCallableSignatureRowRefV1<'_>> {
        self.rows
            .binary_search_by_key(&batch_slot, |row| row.batch_slot)
            .ok()
            .map(|index| PhysicalCallableSignatureRowRefV1 {
                row: &self.rows[index],
            })
    }
}

#[derive(Debug)]
pub(super) enum CallablePhysicalSignatureIssueV1 {
    BatchLoan(ResolvedCallableSemanticBatchLoanErrorV1),
    MissingSelectedIdentity,
    MissingParameterContract,
    DuplicateParameterContract,
    ParameterOwnerMismatch,
    ParameterOrdinalMismatch,
    ReceiverMissing,
    ReceiverUnexpected,
    ReceiverOwnerMismatch,
    ReceiverRecordMismatch,
    LaneOverflow,
}

pub(super) fn issue_callable_physical_signature_v1(
    brand: SameModuleCallableCatalogBrandV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    parameter_contracts: &[OwnedCallableParameterContractDeclarationV1],
) -> Result<VerifiedCallablePhysicalSignatureCohortV1, CallablePhysicalSignatureIssueV1> {
    let mut selected_slots = BTreeSet::new();
    for key in selected.keys() {
        let Some(batch_slot) = selected.batch_slot(key) else {
            return Err(CallablePhysicalSignatureIssueV1::MissingSelectedIdentity);
        };
        if !selected_slots.insert(batch_slot) {
            return Err(CallablePhysicalSignatureIssueV1::MissingSelectedIdentity);
        }
    }

    let rows = batch
        .with_declaration_semantics(|view| {
            let mut rows = Vec::new();
            for batch_slot in selected_slots {
                let Some(declaration) = view
                    .declarations()
                    .iter()
                    .find(|row| row.batch_slot() == batch_slot)
                else {
                    return Err(CallablePhysicalSignatureIssueV1::MissingSelectedIdentity);
                };
                if matches!(
                    declaration.mode(),
                    ResolvedCallableDeclarationModeV1::TopLevel
                ) {
                    // Top-level rows remain in the complete semantic batch,
                    // but the current package physical-signature row is for
                    // selected direct Box methods only.  Their physical
                    // owner is a later root lane, never inferred here.
                    continue;
                }
                let Some(selected_identity) = selected.identity_for_batch_slot(batch_slot) else {
                    return Err(CallablePhysicalSignatureIssueV1::MissingSelectedIdentity);
                };
                if !declaration.identity().same_as(selected_identity) {
                    return Err(CallablePhysicalSignatureIssueV1::MissingSelectedIdentity);
                }
                let mut contracts = parameter_contracts
                    .iter()
                    .filter(|row| row.batch_slot == batch_slot);
                let Some(contract) = contracts.next() else {
                    return Err(CallablePhysicalSignatureIssueV1::MissingParameterContract);
                };
                if contracts.next().is_some() {
                    return Err(CallablePhysicalSignatureIssueV1::DuplicateParameterContract);
                }
                if contract.owner != declaration.owner() {
                    return Err(CallablePhysicalSignatureIssueV1::ParameterOwnerMismatch);
                }

                let (mode, receiver) = match declaration.mode() {
                    ResolvedCallableDeclarationModeV1::StaticBoxMethod => {
                        if declaration
                            .function()
                            .declaration_binding(&SourceBindingSiteV1::Receiver)
                            .is_some()
                        {
                            return Err(CallablePhysicalSignatureIssueV1::ReceiverUnexpected);
                        }
                        (CallableParameterDeclarationModeV1::StaticBoxMethod, None)
                    }
                    ResolvedCallableDeclarationModeV1::InstanceBoxMethod => {
                        let binding = declaration
                            .function()
                            .declaration_binding(&SourceBindingSiteV1::Receiver)
                            .ok_or(CallablePhysicalSignatureIssueV1::ReceiverMissing)?;
                        if binding.owner() != declaration.owner() {
                            return Err(CallablePhysicalSignatureIssueV1::ReceiverOwnerMismatch);
                        }
                        let record = declaration
                            .function()
                            .binding(binding)
                            .ok_or(CallablePhysicalSignatureIssueV1::ReceiverRecordMismatch)?;
                        if record.kind() != BindingKindV1::Receiver
                            || record.origin()
                                != &BindingOriginV1::Source(SourceBindingSiteV1::Receiver)
                        {
                            return Err(CallablePhysicalSignatureIssueV1::ReceiverRecordMismatch);
                        }
                        (
                            CallableParameterDeclarationModeV1::InstanceBoxMethod,
                            Some(binding),
                        )
                    }
                    ResolvedCallableDeclarationModeV1::TopLevel => unreachable!(),
                };
                if contract.mode != mode {
                    return Err(CallablePhysicalSignatureIssueV1::ParameterOwnerMismatch);
                }
                if receiver.is_some_and(|receiver| {
                    contract
                        .parameters
                        .iter()
                        .any(|parameter| parameter.binding == receiver)
                }) {
                    return Err(CallablePhysicalSignatureIssueV1::ReceiverRecordMismatch);
                }

                let source_logical_arity = u32::try_from(contract.parameters.len())
                    .map_err(|_| CallablePhysicalSignatureIssueV1::LaneOverflow)?;
                let mut lanes = Vec::new();
                let mut next_index = 0u32;
                if let Some(binding) = receiver {
                    lanes.push(PhysicalCallableLaneV1 {
                        index: next_index,
                        role: PhysicalCallableLaneRoleV1::InstanceReceiver,
                        logical_ordinal: None,
                        binding,
                    });
                    next_index = next_index
                        .checked_add(1)
                        .ok_or(CallablePhysicalSignatureIssueV1::LaneOverflow)?;
                }
                for (expected_ordinal, parameter) in contract.parameters.iter().enumerate() {
                    let expected_ordinal = u32::try_from(expected_ordinal)
                        .map_err(|_| CallablePhysicalSignatureIssueV1::LaneOverflow)?;
                    if parameter.ordinal != expected_ordinal {
                        return Err(CallablePhysicalSignatureIssueV1::ParameterOrdinalMismatch);
                    }
                    if parameter.binding.owner() != declaration.owner() {
                        return Err(CallablePhysicalSignatureIssueV1::ParameterOwnerMismatch);
                    }
                    let role = match parameter.kind {
                        CallableParameterContractKindV1::ExactText(_) => {
                            lanes.push(PhysicalCallableLaneV1 {
                                index: next_index,
                                role: PhysicalCallableLaneRoleV1::ExactTextSlot,
                                logical_ordinal: Some(parameter.ordinal),
                                binding: parameter.binding,
                            });
                            next_index = next_index
                                .checked_add(1)
                                .ok_or(CallablePhysicalSignatureIssueV1::LaneOverflow)?;
                            PhysicalCallableLaneRoleV1::ExactTextGeneration
                        }
                        CallableParameterContractKindV1::OpaqueHandle
                        | CallableParameterContractKindV1::ExactTrivial(_) => {
                            PhysicalCallableLaneRoleV1::OrdinaryScalar
                        }
                    };
                    lanes.push(PhysicalCallableLaneV1 {
                        index: next_index,
                        role,
                        logical_ordinal: Some(parameter.ordinal),
                        binding: parameter.binding,
                    });
                    next_index = next_index
                        .checked_add(1)
                        .ok_or(CallablePhysicalSignatureIssueV1::LaneOverflow)?;
                }
                let receiver_lane_count = u32::from(receiver.is_some());
                let physical_callable_lane_count = next_index;
                let physical_formal_lane_count = physical_callable_lane_count
                    .checked_sub(receiver_lane_count)
                    .ok_or(CallablePhysicalSignatureIssueV1::LaneOverflow)?;
                let role = selected
                    .role_for_batch_slot(batch_slot)
                    .ok_or(CallablePhysicalSignatureIssueV1::MissingSelectedIdentity)?;
                rows.push(VerifiedCallablePhysicalSignatureRowV1 {
                    batch_slot,
                    identity: declaration.identity().clone(),
                    owner: declaration.owner(),
                    mode,
                    role,
                    receiver,
                    source_logical_arity,
                    receiver_lane_count,
                    physical_formal_lane_count,
                    physical_callable_lane_count,
                    lanes: lanes.into_boxed_slice(),
                });
            }
            rows.sort_by_key(|row| row.batch_slot);
            Ok(rows.into_boxed_slice())
        })
        .map_err(CallablePhysicalSignatureIssueV1::BatchLoan)??;
    Ok(VerifiedCallablePhysicalSignatureCohortV1 { brand, rows })
}
