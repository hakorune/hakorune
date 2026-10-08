//! The same affine Object row retains its checked input/result join.
//! Snapshots detect drift; only the existing full-slot finish issues this seal.
use super::super::borrowed_formal_actuals::{
    PreparedBorrowedCallActualsV1, PreparedBorrowedFormalActualV1,
};
use super::super::borrowed_formal_uses::{BorrowedIncomingCallDraftV1, BorrowedIncomingSourceV1};
use super::*;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct ObjectPacketSealV1
{
    result: InvokeCallResultKind,
    completion: Rc<VerifiedFunctionCompletionV1>,
    terminals: Rc<BTreeMap<crate::mir::resolved_semantics::SourceStmtSiteV1, TerminalRelationV1>>,
    opaque: bool,
    actuals: Box<[(BorrowedIncomingCallDraftV1, PreparedBorrowedCallActualsV1)]>,
}

impl ObjectPacketSealV1 {
    pub(super) fn retain(
        ledger: &OrdinaryNewClaimLedgerV1,
        target: &LexicalInstanceCallSourceTargetV1,
        result: InvokeCallResultKind,
    ) -> Result<Self, String> {
        let ingress = ledger
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("object-packet/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let opaque = ingress.definitions.contains_key(&target.callee_owner());
        let calls = if opaque {
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
        Ok(Self {
            result,
            completion: Rc::clone(completion),
            terminals: Rc::clone(terminals),
            opaque,
            actuals: actuals.into_boxed_slice(),
        })
    }
}

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
        let ingress = ledger
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("object-packet/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let calls = if seal.opaque {
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
            if ingress.definitions.contains_key(&self.callee_owner()) {
                return Err(freeze("object-packet/input-family-drift"));
            }
            ingress
                .source_incoming
                .project(&[self.callee_owner()].into_iter().collect())
                .map_err(|issue| {
                    format!("{}: {issue:?}", freeze("object-packet/incoming-coverage"))
                })?
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
        if self.result() != Some(seal.result) {
            return Err(freeze("borrowed-call/result-mismatch"));
        }
        own.ok_or_else(|| freeze("object-packet/incoming-missing"))
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
        if !row.source_target().has_object_source_requirement()
            || (receiver_only && !row.source_target().is_self_receiver())
        {
            return Err(freeze("receiver-object/source-requirement"));
        }
        row.checked_object_packet_inputs_v1(self)
            .map(|inputs| inputs.1)
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
    let source = match (&a.source, &b.source) {
        (BorrowedIncomingSourceV1::Instance(a), BorrowedIncomingSourceV1::Instance(b)) => a == b,
        (
            BorrowedIncomingSourceV1::QualifiedStatic(a),
            BorrowedIncomingSourceV1::QualifiedStatic(b),
        ) => Rc::ptr_eq(a, b),
        _ => false,
    };
    source && a.call == b.call && a.callee == b.callee && a.arguments == b.arguments
}

#[cfg(test)]
#[path = "ordinary_new_object_packet_seal_tests.rs"]
mod tests;
