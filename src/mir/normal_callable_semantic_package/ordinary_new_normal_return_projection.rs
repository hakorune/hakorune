//! Read-only Normal cleanup projection from the original sealed return proof.
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
    homes: Cow<'a, [BindingRefV1]>,
}
impl NormalExitProjectionV1<'_> {
    pub(in crate::mir::normal_callable_semantic_package) fn homes(&self) -> &[BindingRefV1] {
        &self.homes
    }
    pub(in crate::mir::normal_callable_semantic_package) fn fault_homes(&self) -> &[BindingRefV1] {
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
    homes: &'a [BindingRefV1],
) -> Result<Cow<'a, [BindingRefV1]>, String> {
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
    homes: &[BindingRefV1],
) -> Result<Vec<BindingRefV1>, String> {
    if homes.iter().filter(|home| **home == destination).count() != 1 {
        return Err(freeze("projection-destination-count"));
    }
    Ok(homes
        .iter()
        .copied()
        .filter(|home| *home != destination)
        .collect())
}

impl OrdinaryNewClaimLedgerV1 {
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
        let original = row.map_err(|issue| format!("{}: {issue:?}", freeze("projection-exit")))?;
        let Some(relation) = self.terminal_relation_for_owner_at(owner, exit) else {
            return Ok(None);
        };
        if relation.owner() != owner || relation.return_site() != exit {
            return Err(freeze("projection-terminal-owner"));
        }
        let disposition = dispositions.get(&(owner, exit.clone()));
        let homes = match relation {
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
                project_normal_homes(obligation, original.homes())?
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
