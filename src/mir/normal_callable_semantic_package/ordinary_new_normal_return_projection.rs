//! Read-only Normal cleanup projection from the original sealed return proof.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::{freeze, NormalReturnDispositionV1, OrdinaryNewClaimLedgerV1};
use crate::mir::resolved_semantics::home_new_prefix::{
    ObjectReturnAcquisitionV1, RootHomeExitV1, TerminalObjectReturnObligationV1,
    TerminalRelationV1, TerminalReturnedSourceV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceStmtSiteV1,
};
use std::borrow::Cow;

pub(in crate::mir::normal_callable_semantic_package) struct NormalExitProjectionV1<'a> {
    original: &'a RootHomeExitV1,
    homes: Cow<'a, [LiveHomeV1]>,
}
impl NormalExitProjectionV1<'_> {
    pub(in crate::mir::normal_callable_semantic_package) fn homes(&self) -> &[LiveHomeV1] {
        &self.homes
    }
    pub(in crate::mir::normal_callable_semantic_package) fn fault_homes(&self) -> &[LiveHomeV1] {
        self.original.homes()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn covered_calls(
        &self,
    ) -> &[OwnedExprSiteV1] {
        self.original.covered_calls()
    }
}

fn project_normal_homes<'a>(
    obligation: &TerminalObjectReturnObligationV1,
    homes: &'a [LiveHomeV1],
) -> Result<Cow<'a, [LiveHomeV1]>, String> {
    if obligation.exit_homes() != homes {
        return Err(freeze("projection-exit-homes"));
    }
    match obligation.acquisition() {
        ObjectReturnAcquisitionV1::Direct { fault_homes, .. } => {
            if fault_homes.as_ref() != homes {
                return Err(freeze("projection-fault-homes"));
            }
            Ok(Cow::Borrowed(homes))
        }
        ObjectReturnAcquisitionV1::Received(call) => {
            let (_, destination) = call
                .local_binding()
                .ok_or_else(|| freeze("projection-destination"))?;
            received_normal_homes(destination, homes).map(Cow::Owned)
        }
    }
}

fn received_normal_homes(
    destination: BindingRefV1,
    homes: &[LiveHomeV1],
) -> Result<Vec<LiveHomeV1>, String> {
    if homes.iter().filter(|home| home.binding() == destination).count() != 1 {
        return Err(freeze("projection-destination-count"));
    }
    Ok(homes
        .iter()
        .cloned()
        .filter(|home| home.binding() != destination)
        .collect())
}

impl OrdinaryNewClaimLedgerV1 {
    /// Borrow the original non-owning LOCAL return relation. This source view
    /// never issues a result value, Home, transfer, or executable Call packet.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_borrowed_nullable_return_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
    ) -> Result<
        Option<&crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1>,
        String,
    > {
        let Some(TerminalRelationV1::Value(value)) =
            self.terminal_relation_for_owner_at(owner, exit)
        else {
            return Ok(None);
        };
        let TerminalReturnedSourceV1::BorrowedNullableLocal(loan) = value.returned() else {
            return Ok(None);
        };
        let Some(completion) = self.completion_index.get(&owner) else {
            return Ok(None);
        };
        let completion = completion
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("borrowed-local-completion")))?;
        let Some(original) = completion
            .cleanup()
            .root_flow()
            .and_then(|flow| flow.exit_row(exit))
        else {
            return Ok(None);
        };
        let original =
            original.map_err(|issue| format!("{}: {issue:?}", freeze("borrowed-local-exit")))?;
        let observation = self
            .local_call_for_owner(owner, loan.call().site())
            .ok_or_else(|| freeze("borrowed-local-observation-missing"))?;
        if completion.owner() != owner || !completion.explicit_sites().contains(exit)
            || value.owner() != owner || value.return_site() != exit
            || value.value_site().node() != &crate::mir::resolved_semantics::SourcePathV1::from_node(exit.node()).child(
                crate::mir::resolved_semantics::SourcePathSegmentV1::Value).node()
            || loan.call().owner() != owner || loan.root().is_none_or(|root| root.owner() != owner)
            || loan.destination().owner() != owner || Some(loan.destination()) == loan.root()
            || observation.site() != loan.call()
            || observation.borrowed_nullable() != Some(loan.as_ref())
            || observation.local_binding() != Some((loan.declaration(), loan.destination()))
            || observation.result() != crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::BorrowedNullable
            || !original.covered_calls().contains(loan.call())
            || original.homes().iter().any(|home| home.binding() == loan.destination())
            || self.normal_return_dispositions.as_ref().is_some_and(|rows|
                rows.contains_key(&(owner, exit.clone()))) {
            return Err(freeze("borrowed-local-source-identity"));
        }
        let roots = self
            .callable_result_classes
            .checked_original_source_roots_v1(loan.key(), loan.callee_owner())
            .map_err(freeze)?;
        self.callable_result_classes
            .check_borrowed_nullable_roots_v1(loan, &roots)
            .map_err(freeze)?;
        Ok(Some(loan))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn normal_exit_projection_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
    ) -> Result<Option<NormalExitProjectionV1<'_>>, String> {
        let Some(dispositions) = &self.normal_return_dispositions else {
            return Ok(None);
        };
        let Some(completion) = self.completion_index.get(&owner) else {
            return Ok(None);
        };
        let completion = completion
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("projection-completion")))?;
        if completion.owner() != owner || !completion.explicit_sites().contains(exit) {
            return Err(freeze("projection-exit-identity"));
        }
        let Some(row) = completion
            .cleanup()
            .root_flow()
            .and_then(|flow| flow.exit_row(exit))
        else {
            return Ok(None);
        };
        let original = row.map_err(|issue| {
            if std::env::var_os("HAKO_DEBUG_FIELD_TEXT_APPEND").is_some() {
                eprintln!("[field-text-append] projection owner={owner:?} exit={exit:?} issue={issue:?}");
            }
            format!("{}: {issue:?}", freeze("projection-exit"))
        })?;
        let Some(relation) = self.terminal_relation_for_owner_at(owner, exit) else {
            return Ok(None);
        };
        if relation.owner() != owner || relation.return_site() != exit {
            return Err(freeze("projection-terminal-owner"));
        }
        let disposition = dispositions.get(&(owner, exit.clone()));
        let homes = match relation {
            TerminalRelationV1::Value(value)
                if matches!(value.returned(), TerminalReturnedSourceV1::TextResidence { .. }) =>
            {
                let TerminalReturnedSourceV1::TextResidence { binding, acquisition } = value.returned() else {
                    unreachable!()
                };
                let returned = LiveHomeV1::Text(acquisition.clone());
                if disposition.is_some() || binding.owner() != owner
                    || acquisition.binding().owner() != owner || acquisition.source().owner() != owner
                    || !acquisition.owns_root() || !acquisition.is_text_residence()
                    || value.value_site().node() != &crate::mir::resolved_semantics::SourcePathV1::from_node(exit.node())
                        .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Value).node()
                    || original.homes().iter().filter(|home| home.binding() == acquisition.binding()).count() != 1
                    || !original.homes().contains(&returned)
                {
                    return Err(freeze("projection-proof-identity"));
                }
                // Transfer commits at Return after every Normal cleanup has
                // succeeded. The SAME source residence remains in the original
                // Fault list so a sibling cleanup failure still discharges it.
                Cow::Owned(original.homes().iter().filter(|home| *home != &returned).cloned().collect())
            }
            TerminalRelationV1::Value(value)
                if matches!(value.returned(), TerminalReturnedSourceV1::OwnedCall(_)) =>
            {
                let TerminalReturnedSourceV1::OwnedCall(obligation) = value.returned() else {
                    unreachable!()
                };
                let Some(NormalReturnDispositionV1::Verified { proof }) = disposition else {
                    return Ok(None);
                };
                if proof.acquisition().original() != obligation.as_ref()
                    || value.return_site() != exit
                    || obligation.return_site() != exit
                    || obligation.qualification().value()
                        != &OwnedExprSiteV1::new(owner, value.value_site().clone())
                {
                    return Err(freeze("projection-proof-identity"));
                }
                if !self.recheck_object_return_field_supports_v1(proof)? {
                    return Ok(None);
                }
                project_normal_homes(obligation, original.homes())?
            }
            TerminalRelationV1::Value(value)
                if matches!(
                    value.returned(),
                    TerminalReturnedSourceV1::BorrowedNullableLocal(_)
                ) =>
            {
                self.checked_borrowed_nullable_return_source_v1(owner, exit)?
                    .ok_or_else(|| freeze("borrowed-local-source-missing"))?;
                // The result aliases the original formal. It was never a
                // Home, so neither it nor any other Home is transferred here.
                Cow::Borrowed(original.homes())
            }
            _ => {
                if disposition.is_some() {
                    return Err(freeze("projection-spurious-disposition"));
                }
                Cow::Borrowed(original.homes())
            }
        };
        Ok(Some(NormalExitProjectionV1 { original, homes }))
    }
}

#[cfg(test)]
#[path = "ordinary_new_normal_return_projection_tests.rs"]
mod tests;
