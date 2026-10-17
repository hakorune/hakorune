//! The same affine Object row retains its checked input/result join.
//! Snapshots detect drift; only the existing full-slot finish issues this seal.
use super::super::super::completion_index::{
    ObjectReturnTeardownAvailabilityV1, ObjectReturnTeardownDescriptorV1,
};
use super::super::borrowed_formal_actuals::{
    PreparedBorrowedCallActualsV1, PreparedBorrowedFormalActualV1,
};
use super::super::borrowed_formal_uses::{BorrowedIncomingCallDraftV1, BorrowedIncomingSourceV1};
use super::*;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ObjectPacketInputFamilyV1 {
    QualifiedOriginal,
    OriginalSourceInstance,
    GlobalOpaque,
    OriginalNonGlobal,
}

fn packet_input_family_v1(
    ledger: &OrdinaryNewClaimLedgerV1,
    ingress: &super::super::borrowed_formal_source::PreparedBorrowedFormalIngressV1,
    target: &LexicalInstanceCallSourceTargetV1,
) -> Result<ObjectPacketInputFamilyV1, String> {
    let catalog = ledger
        .original_parameter_contracts
        .as_deref()
        .ok_or_else(|| freeze("object-packet/original-catalog-missing"))?;
    let mut contracts = catalog
        .iter()
        .filter(|row| row.owner == target.callee_owner());
    let contract = contracts
        .next()
        .ok_or_else(|| freeze("object-packet/original-contract-missing"))?;
    if contracts.next().is_some()
        || contract.batch_slot != target.target_batch_slot()
        || contract.parameters.len() != target.argument_sites().len()
    {
        return Err(freeze("object-packet/original-contract-drift"));
    }
    ingress.recheck_field_producer_source_v1(target, &ledger.callable_result_classes, &ledger.field_write_claims)?;
    let source_input = ledger.has_original_object_source_input_v1(ingress, target)?;
    if ledger.selected_original_object_input_owner_v1(ingress, target.callee_owner())? {
        return Ok(ObjectPacketInputFamilyV1::QualifiedOriginal);
    }
    if source_input {
        return Ok(ObjectPacketInputFamilyV1::OriginalSourceInstance);
    }
    Ok(
        if ingress.definitions.contains_key(&target.callee_owner()) {
            ObjectPacketInputFamilyV1::GlobalOpaque
        } else {
            ObjectPacketInputFamilyV1::OriginalNonGlobal
        },
    )
}

fn packet_input_ready_v1(
    ledger: &OrdinaryNewClaimLedgerV1,
    ingress: &super::super::borrowed_formal_source::PreparedBorrowedFormalIngressV1,
    target: &LexicalInstanceCallSourceTargetV1,
    family: ObjectPacketInputFamilyV1,
) -> Result<bool, String> {
    match family {
        ObjectPacketInputFamilyV1::QualifiedOriginal | ObjectPacketInputFamilyV1::OriginalSourceInstance => {
            let Some((selected, signatures)) = &ledger.original_instance_input_issuance else {
                return Ok(false);
            };
            let catalog = ledger
                .original_parameter_contracts
                .as_deref()
                .ok_or_else(|| freeze("object-packet/original-catalog-missing"))?;
            let arguments = if family == ObjectPacketInputFamilyV1::OriginalSourceInstance {
                ledger.checked_completed_original_instance_target_arguments_v1(
                    target, selected, catalog, signatures,
                )?
            } else {
                ledger.checked_completed_typed_object_target_arguments_v1(
                    target, selected, catalog, signatures,
                )?
            };
            Ok(arguments.is_some())
        }
        ObjectPacketInputFamilyV1::GlobalOpaque => ledger
            .checked_borrowed_entry_incoming(ingress, target.callee_owner())
            .map(|_| true),
        ObjectPacketInputFamilyV1::OriginalNonGlobal => Ok(true),
    }
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct ObjectPacketSealV1
{
    result: InvokeCallResultKind,
    teardown: ObjectReturnTeardownAvailabilityV1,
    field_supports: Box<[super::super::super::completion_index::ObjectCallFieldSupportRequirementV1]>,
    completion: Rc<VerifiedFunctionCompletionV1>,
    terminals: Rc<BTreeMap<crate::mir::resolved_semantics::SourceStmtSiteV1, TerminalRelationV1>>,
    input_family: ObjectPacketInputFamilyV1,
    actuals: Box<[(BorrowedIncomingCallDraftV1, PreparedBorrowedCallActualsV1)]>,
}

impl ObjectPacketSealV1 {
    pub(super) fn retain(
        ledger: &OrdinaryNewClaimLedgerV1,
        target: &LexicalInstanceCallSourceTargetV1,
        result: InvokeCallResultKind,
        teardown: ObjectReturnTeardownAvailabilityV1,
        field_supports: Box<[super::super::super::completion_index::ObjectCallFieldSupportRequirementV1]>,
    ) -> Result<Option<Self>, String> {
        let ingress = ledger
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("object-packet/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let input_family = packet_input_family_v1(ledger, ingress, target)?;
        if !packet_input_ready_v1(ledger, ingress, target, input_family)? {
            return Ok(None);
        }
        let calls = if input_family == ObjectPacketInputFamilyV1::GlobalOpaque {
            ingress.incoming.clone()
        } else {
            ingress
                .source_incoming
                .project(&[target.callee_owner()].into_iter().collect())
                .map_err(|issue| {
                    format!("{}: {issue:?}", freeze("object-packet/incoming-coverage"))
                })?
        };
        if !calls.iter().any(|call| call.call == *target.call_site()) {
            return Err(freeze("object-packet/incoming-missing"));
        }
        let mut actuals = Vec::new();
        for call in &calls {
            let rows = ledger
                .borrowed_formal_actuals
                .get(&call.call)
                .ok_or_else(|| freeze("object-packet/actuals-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            rows.ordered_arguments_for_v1(call)?;
            actuals.push((call.clone(), rows.clone()));
        }
        let completion = ledger
            .completion_index
            .get(&target.callee_owner())
            .ok_or_else(|| freeze("object-packet/completion-missing"))?
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("object-packet/completion")))?;
        let terminals = ledger
            .terminal_relation_index
            .get(&target.callee_owner())
            .ok_or_else(|| freeze("object-packet/terminals-missing"))?;
        Ok(Some(Self {
            result,
            teardown,
            field_supports,
            completion: Rc::clone(completion),
            terminals: Rc::clone(terminals),
            input_family,
            actuals: actuals.into_boxed_slice(),
        }))
    }
}

#[path = "ordinary_new_object_packet_field_support.rs"]
mod field_support;

impl LexicalInstanceCallDispositionRowV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_object_packet_actuals_v1<
        'a,
    >(
        &self,
        ledger: &'a OrdinaryNewClaimLedgerV1,
    ) -> Result<&'a [PreparedBorrowedFormalActualV1], String> {
        self.checked_object_packet_inputs_v1(ledger)
            .map(|inputs| inputs.0)
    }

    fn checked_object_packet_inputs_v1<'a>(
        &self,
        ledger: &'a OrdinaryNewClaimLedgerV1,
    ) -> Result<
        (
            &'a [PreparedBorrowedFormalActualV1],
            &'a [crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1],
        ),
        String,
    > {
        let seal = self
            .object_packet
            .as_ref()
            .ok_or_else(|| freeze("object-packet/seal-missing"))?;
        let target = self.source_target();
        if !ledger.requires_object_packet_v1(target)? {
            return Err(freeze("receiver-object/source-requirement"));
        }
        let ingress = ledger
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("object-packet/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let family = packet_input_family_v1(ledger, ingress, target)?;
        if family != seal.input_family {
            return Err(freeze("object-packet/input-family-drift"));
        }
        let calls = if family == ObjectPacketInputFamilyV1::GlobalOpaque {
            // Retain the original checker and its whole-final-incoming scope.
            ledger.checked_borrowed_entry_incoming(ingress, self.callee_owner())?;
            let original = ingress
                .source_incoming
                .project(&[self.callee_owner()].into_iter().collect())
                .map_err(|issue| {
                    format!("{}: {issue:?}", freeze("object-packet/incoming-coverage"))
                })?;
            let sealed: Vec<_> = seal
                .actuals
                .iter()
                .filter(|(call, _)| call.callee == self.callee_owner())
                .collect();
            if original.len() != sealed.len()
                || original
                    .iter()
                    .any(|call| !sealed.iter().any(|(saved, _)| same_incoming(call, saved)))
            {
                return Err(freeze("object-packet/incoming-domain-drift"));
            }
            ingress.incoming.clone()
        } else {
            let original = ingress
                .source_incoming
                .project(&[self.callee_owner()].into_iter().collect())
                .map_err(|issue| {
                    format!("{}: {issue:?}", freeze("object-packet/incoming-coverage"))
                })?;
            let mut seen = std::collections::BTreeSet::new();
            for projected in ingress.incoming.iter().filter(|call| call.callee == self.callee_owner()) {
                if !seen.insert(projected.call.clone())
                    || !original.iter().any(|call| same_incoming(call, projected))
                { return Err(freeze("object-packet/incoming-identity")); }
            }
            original
        };
        if calls.len() != seal.actuals.len() {
            return Err(freeze("object-packet/incoming-domain-drift"));
        }
        let mut own = None;
        for (call, (original, saved)) in calls.iter().zip(seal.actuals.iter()) {
            let site = &original.call;
            if !same_incoming(call, original) {
                return Err(freeze("object-packet/incoming-domain-drift"));
            }
            let current = ledger
                .borrowed_formal_actuals
                .get(site)
                .ok_or_else(|| freeze("object-packet/actuals-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            current.ordered_arguments_for_v1(call)?;
            if current != saved {
                return Err(freeze("object-packet/actuals-drift"));
            }
            if site == self.call_site() {
                if own.is_some()
                    || call.callee != self.callee_owner()
                    || call.source.require_instance()? != target
                {
                    return Err(freeze("object-packet/target-drift"));
                }
                own = Some((
                    current.opaque_actuals.as_ref(),
                    current.ordered_arguments_for_v1(call)?,
                ));
            }
        }
        let completion = ledger
            .completion_index
            .get(&self.callee_owner())
            .and_then(|row| row.as_ref().ok());
        if completion.is_none_or(|row| !Rc::ptr_eq(row, &seal.completion))
            || ledger
                .terminal_relation_index
                .get(&self.callee_owner())
                .is_none_or(|row| !Rc::ptr_eq(row, &seal.terminals))
        {
            return Err(freeze("object-packet/result-source-drift"));
        }
        if !packet_input_ready_v1(ledger, ingress, target, family)? {
            return Err(freeze("object-packet/original-input-not-ready"));
        }
        ledger.recheck_object_call_field_support_requirements_v1(target, &seal.field_supports)?;
        if self.result() != Some(seal.result) {
            return Err(freeze("borrowed-call/result-mismatch"));
        }
        let own = own.ok_or_else(|| freeze("object-packet/incoming-missing"))?;
        super::super::super::completion_index::ObjectCallFieldSupportRequirementV1::visit_actuals_v1(
            &seal.field_supports, target, own.0, |_, _| Ok(()))?;
        Ok(own)
    }
}

/// Borrow the exact existing affine packet as a whole. This view owns no
/// destination, acquisition, receipt or new source inventory. Local and
/// expression consumers keep their different transfer/cleanup obligations.
#[derive(Debug)]
pub(crate) struct BorrowedObjectCallPacketV1<'ledger, 'packet> {
    arguments: &'ledger [crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1],
    row: &'packet LexicalInstanceCallDispositionRowV1,
    teardown: &'packet ObjectReturnTeardownDescriptorV1,
}
impl BorrowedObjectCallPacketV1<'_, '_> {
    pub(crate) fn arguments(&self) -> &[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1] {
        self.arguments
    }
    pub(crate) fn result(&self) -> InvokeCallResultKind {
        self.row.result().expect("checked original Object packet result")
    }
    /// Complete cancellation operations for this Taken packet's projected
    /// candidate. The expression consumer must corroborate that real Normal
    /// producer and own the pending transfer/Fault path; no LOCAL is created.
    pub(crate) fn cancellation_operations_v1(&self, value: crate::mir::ValueId) -> Vec<crate::mir::instruction::InvokeOperation> {
        super::super::super::object_end::operations_v1(
            self.teardown.object(), self.result() == InvokeCallResultKind::NullableHandle,
            self.teardown.owned_children().unwrap_or(&[]), value,
        ).map(|(_, operation)| operation).collect()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn teardown(&self) -> &ObjectReturnTeardownDescriptorV1 {
        self.teardown
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// One Taken Object packet lends its original executable ordered arguments.
    /// Typed and zero-argument calls owe the same completed incoming proof.
    pub(crate) fn object_packet_arguments_v1<'a>(
        &'a self,
        row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<&'a [crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1], String>
    {
        self.object_packet_arguments_for_v1(row, false)
    }

    fn object_packet_arguments_for_v1<'a>(
        &'a self,
        row: &LexicalInstanceCallDispositionRowV1,
        receiver_only: bool,
    ) -> Result<&'a [crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1], String>
    {
        if !matches!(
            self.lexical_instance_calls.borrow().get(row.call_site()),
            Some(LexicalInstanceCallDispositionSlotV1::Taken)
        ) {
            return Err(freeze("borrowed-call/disposition-not-owned-and-taken"));
        }
        if !self.requires_object_packet_v1(row.source_target())?
            || (receiver_only && !row.source_target().is_self_receiver())
        {
            return Err(freeze("receiver-object/source-requirement"));
        }
        row.checked_object_packet_inputs_v1(self)
            .map(|inputs| inputs.1)
    }

    /// Inputs and complete End come from one Taken packet borrow. A nested
    /// expression uses this same boundary without manufacturing a local Home.
    pub(crate) fn borrowed_object_call_packet_v1<'ledger, 'packet>(
        &'ledger self,
        row: &'packet LexicalInstanceCallDispositionRowV1,
    ) -> Result<BorrowedObjectCallPacketV1<'ledger, 'packet>, String> {
        let arguments = self.object_packet_arguments_v1(row)?;
        let seal = row.object_packet.as_ref()
            .ok_or_else(|| freeze("object-packet/seal-missing"))?;
        let (teardown, nullable) = match &seal.teardown {
            ObjectReturnTeardownAvailabilityV1::Verified { descriptor, nullable } => (descriptor, *nullable),
            ObjectReturnTeardownAvailabilityV1::AnchoredFields { descriptor, nullable } => {
                self.object_packet_field_support_parents_v1(row)?
                    .ok_or_else(|| freeze("object-packet/field-support-missing"))?;
                (descriptor, *nullable)
            }
            ObjectReturnTeardownAvailabilityV1::Unavailable { .. } => return Err(freeze("object-packet/teardown-unavailable")),
        };
        if nullable != (row.result() == Some(InvokeCallResultKind::NullableHandle)) {
            return Err(freeze("object-packet/teardown-kind-drift"));
        }
        Ok(BorrowedObjectCallPacketV1 { arguments, row, teardown })
    }

    /// Entry-receiver flow rows intentionally carry no argument authority.
    pub(crate) fn receiver_object_packet_arguments_v1<'a>(
        &'a self,
        row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<&'a [crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1], String>
    {
        self.object_packet_arguments_for_v1(row, true)
    }
}

fn same_incoming(a: &BorrowedIncomingCallDraftV1, b: &BorrowedIncomingCallDraftV1) -> bool {
    let source = a.source.same_source(&b.source);
    source && a.call == b.call && a.callee == b.callee && a.arguments == b.arguments
}

#[cfg(test)]
#[path = "ordinary_new_object_packet_seal_tests.rs"]
mod tests;
