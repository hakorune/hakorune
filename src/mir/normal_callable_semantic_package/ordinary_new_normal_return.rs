//! Atomic source handoff dispositions in the original completed-index owner.
//! Original cleanup and artifact gates remain unchanged.
use super::super::OrdinaryNewClaimLedgerV1;
use super::return_handoff::{
    ObjectReturnHandoffAvailabilityV1, ObjectReturnHandoffMemoV1, ObjectReturnHandoffUnavailableV1,
    VerifiedObjectReturnHandoffV1,
};
use crate::mir::normal_callable_semantic_package::{
    model::OwnedCallableParameterContractDeclarationV1,
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    TerminalRelationV1, TerminalReturnedSourceV1, TerminalValueReturnV1,
};
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) enum NormalReturnDispositionV1
{
    Verified {
        proof: Rc<VerifiedObjectReturnHandoffV1>,
    },
    Unavailable(ObjectReturnHandoffUnavailableV1),
}

impl OrdinaryNewClaimLedgerV1 {
    /// Shared exact exit-domain law for retained Normal dispositions.
    pub(in crate::mir::normal_callable_semantic_package) fn validate_object_return_disposition_exits_v1(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        exits: &[crate::mir::resolved_semantics::SourceStmtSiteV1],
    ) -> Result<(), String> {
        if self
            .normal_return_dispositions
            .as_ref()
            .is_some_and(|rows| {
                rows.keys()
                    .any(|(actual, exit)| *actual == owner && !exits.contains(exit))
            })
        {
            return Err(
                "[freeze:contract][ordinary-new/object-return/construction-disposition-exit]"
                    .into(),
            );
        }
        Ok(())
    }

    /// All metadata is borrowed from this signature-ready issuance. No partial install.
    pub(in crate::mir::normal_callable_semantic_package) fn seal_object_return_dispositions_v1(
        &mut self,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<(), String> {
        if self.normal_return_dispositions.is_some() {
            return Err(freeze("duplicate-seal"));
        }
        let mut values: BTreeMap<_, TerminalValueReturnV1> = BTreeMap::new();
        for relation in self.terminal_relation.values().chain(
            self.terminal_relation_index
                .values()
                .flat_map(|rows| rows.values()),
        ) {
            let TerminalRelationV1::Value(value) = relation else {
                continue;
            };
            if !matches!(value.returned(), TerminalReturnedSourceV1::OwnedCall(_)) {
                continue;
            }
            let identity = (value.owner(), value.return_site().clone());
            if let Some(prior) = values.insert(identity, value.clone()) {
                if prior != *value {
                    return Err(freeze("exit-identity"));
                }
            }
        }
        let mut planned = BTreeMap::new();
        let mut memo = ObjectReturnHandoffMemoV1::default();
        for (identity, value) in values {
            let disposition = match self.checked_object_return_handoff_with_memo_v1(
                &value, selected, contracts, signatures, &mut memo,
            )? {
                ObjectReturnHandoffAvailabilityV1::Verified(proof) => {
                    NormalReturnDispositionV1::Verified { proof }
                }
                ObjectReturnHandoffAvailabilityV1::Unavailable(reason) => {
                    NormalReturnDispositionV1::Unavailable(reason)
                }
            };
            planned.insert(identity, disposition);
        }
        self.normal_return_dispositions = Some(planned);
        Ok(())
    }
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/object-return/normal-{reason}]")
}
#[cfg(test)]
#[path = "ordinary_new_normal_return_tests.rs"]
mod tests;

#[path = "ordinary_new_normal_return_projection.rs"]
mod projection;
