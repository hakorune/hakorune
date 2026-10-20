//! Complete selected Static incoming identity before executable actuals.

use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use std::rc::Rc;

impl PreparedBorrowedFormalIngressV1 {
    /// Original scalar protocol membership, before any actual or Completion proof.
    /// Share this selection with entry staging so a newer input family cannot
    /// take an existing scalar owner's source uses into a different protocol.
    pub(in crate::mir::normal_callable_semantic_package) fn is_scalar_only_static_input_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> bool {
        self.source_only_definitions.get(&owner).is_some_and(|draft| {
            let mul = draft.uses.iter().any(|row| matches!(row.kind,
                BorrowedFormalUseDraftKindV1::MulOperand { .. }));
            let forwards = draft.uses.iter().filter(|row| matches!(row.kind,
                BorrowedFormalUseDraftKindV1::UnresolvedArgument { ordinal: 0, .. })).count();
            mul || (forwards == 1 && draft.uses.iter().all(|row| matches!(row.kind,
                BorrowedFormalUseDraftKindV1::Copy { .. }
                    | BorrowedFormalUseDraftKindV1::UnresolvedArgument { ordinal: 0, .. })))
                || self.is_compare_only_static_input_v1(owner)
        })
    }

    /// Candidate source family only; executable entry still needs the whole
    /// original input cohort, signature, result and Completion.
    pub(in crate::mir::normal_callable_semantic_package) fn is_compare_only_static_input_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> bool {
        self.source_only_definitions.get(&owner).is_some_and(|draft| {
            draft.uses.iter().any(|row| matches!(row.kind,
                BorrowedFormalUseDraftKindV1::CompareOperand { .. }))
                && draft.uses.iter().all(|row| {
                    matches!(row.kind, BorrowedFormalUseDraftKindV1::Copy { .. }
                        | BorrowedFormalUseDraftKindV1::CompareOperand { .. })
                        && draft.origins.get(&row.binding) == Some(&row.formal)
                        && self.checked_static_input(row.formal)
                })
        })
    }
    /// The selected-caller inventory's whole-callee projection retains vetoes.
    /// This receipt is source-only: no actual, carrier or call coordinate.
    pub(in crate::mir::normal_callable_semantic_package) fn static_incoming_cohort_v1(
        &self,
        selected: &Rc<StaticIncomingSourceV1>,
    ) -> Result<Box<[Rc<StaticIncomingSourceV1>]>, String> {
        let rows = self
            .source_incoming
            .project(&BTreeSet::from([selected.callee_owner()]))
            .map_err(|error| format!("{}: {error:?}", freeze("static-cohort/incoming-veto")))?;
        let mut sites = BTreeSet::new();
        let mut includes_selected = false;
        let mut original = Vec::with_capacity(rows.len());
        for row in rows {
            let BorrowedIncomingSourceV1::Static(source) = row.source else {
                return Err(freeze("static-cohort/nonstatic-incoming"));
            };
            if row.callee != selected.callee_owner()
                || source.callee_owner() != row.callee
                || source.call_site() != &row.call
                || source.target() != selected.target()
                || source.target_batch_slot() != selected.target_batch_slot()
                || source.argument_sites().len() != selected.target().arity() as usize
                || !sites.insert(row.call)
            {
                return Err(freeze("static-cohort/original-drift"));
            }
            includes_selected |= Rc::ptr_eq(&source, selected);
            original.push(source);
        }
        if !includes_selected {
            return Err(freeze("static-cohort/selected-missing"));
        }
        Ok(original.into_boxed_slice())
    }
}

impl PreparedBorrowedFormalIngressV1 {
    /// Dispatch can query membership without exposing the private source row.
    pub(in crate::mir::normal_callable_semantic_package) fn has_scalar_qualified_source_call_v1(
        &self,
        call: &OwnedExprSiteV1,
    ) -> Result<bool, String> {
        self.scalar_qualified_source_call_v1(call)
            .map(|row| row.is_some())
    }

    /// Select source argument observation for a Qualified sibling of the
    /// existing source-only scalar owner. This does not select entry or a
    /// physical packet; the whole input cohort still has to finish.
    pub(in crate::mir::normal_callable_semantic_package) fn scalar_qualified_source_call_v1(
        &self,
        call: &OwnedExprSiteV1,
    ) -> Result<Option<&BorrowedIncomingCallDraftV1>, String> {
        let mut originals = self
            .source_incoming
            .exact_rows()
            .filter(|row| &row.call == call);
        let Some(row) = originals.next() else {
            return Ok(None);
        };
        if originals.next().is_some() {
            return Err(freeze("static-scalar/source-call-duplicate"));
        }
        let BorrowedIncomingSourceV1::Static(original) = &row.source else {
            return Ok(None);
        };
        let compare_only = self.is_compare_only_static_input_v1(row.callee);
        if !original.is_qualified_i64_source_v1()
            || self.definitions.contains_key(&row.callee)
            || self.target_static.contains_key(&row.callee)
            || (!compare_only && !self
                .source_only_definitions
                .get(&row.callee)
                .is_some_and(|draft| {
                    draft.uses.iter().any(|use_| {
                        matches!(use_.kind, BorrowedFormalUseDraftKindV1::MulOperand { .. })
                    })
                }))
            || original.parameters().len() != 1
            || !original.parameters()[0].kind.is_ordinary_borrowed_handle()
            || (original.required_i64_arguments() != [0]
                && !(compare_only && original.required_i64_arguments().is_empty()))
        {
            return Ok(None);
        }
        let cohort = self.static_incoming_cohort_v1(original)?;
        if !cohort
            .iter()
            .any(|member| member.is_current_owner_i64_source_v1())
            && !compare_only
        {
            return Ok(None);
        }
        Ok(Some(row))
    }
}
