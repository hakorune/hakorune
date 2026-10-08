//! Full pure-owned source ancestry co-seal inside the completed-index owner.
//! This receipt precedes Normal projection and independent physical validation.
use super::super::{result_class_claim, OrdinaryNewClaimLedgerV1};
use super::return_acquisition::VerifiedObjectReturnSourceAcquisitionV1;
use super::return_leaf::VerifiedObjectReturnLeafV1;
use crate::mir::normal_callable_semantic_package::{
    model::OwnedCallableParameterContractDeclarationV1,
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    TerminalReturnedSourceV1, TerminalValueReturnV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceStmtSiteV1};
use result_class_claim::ResultWitnessStepV1;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[derive(Debug)]
pub(super) struct VerifiedObjectReturnHandoffV1 {
    acquisition: VerifiedObjectReturnSourceAcquisitionV1,
    alternatives: Box<[VerifiedObjectReturnAlternativeV1]>,
    teardown: ObjectReturnTeardownAvailabilityV1,
}
#[derive(Debug)]
pub(super) enum VerifiedObjectReturnAlternativeV1 {
    Leaf(VerifiedObjectReturnLeafV1),
    Call(Rc<VerifiedObjectReturnHandoffV1>),
}
#[path = "ordinary_new_return_teardown.rs"]
mod teardown;
pub(super) use teardown::ObjectReturnTeardownAvailabilityV1;

impl VerifiedObjectReturnHandoffV1 {
    pub(super) fn teardown(&self) -> &ObjectReturnTeardownAvailabilityV1 {
        &self.teardown
    }

    pub(super) fn acquisition(&self) -> &VerifiedObjectReturnSourceAcquisitionV1 {
        &self.acquisition
    }
    pub(super) fn alternatives(&self) -> &[VerifiedObjectReturnAlternativeV1] {
        &self.alternatives
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ObjectReturnHandoffUnavailableV1 {
    Acquisition,
    BorrowedOrigin,
    CalleeCompletion,
    Leaf,
    RecursivePath,
}
#[derive(Debug, Clone)]
pub(super) enum ObjectReturnHandoffAvailabilityV1 {
    Verified(Rc<VerifiedObjectReturnHandoffV1>),
    Unavailable(ObjectReturnHandoffUnavailableV1),
}

/// Valid only during one read-only seal pass; never persisted or shared across edits.
#[derive(Default)]
pub(super) struct ObjectReturnHandoffMemoV1 {
    completed: BTreeMap<
        (FunctionOwnerIdV1, SourceStmtSiteV1),
        (TerminalValueReturnV1, ObjectReturnHandoffAvailabilityV1),
    >,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn checked_object_return_handoff_v1(
        &self,
        value: &TerminalValueReturnV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<ObjectReturnHandoffAvailabilityV1, String> {
        self.checked_object_return_handoff_with_memo_v1(
            value,
            selected,
            contracts,
            signatures,
            &mut ObjectReturnHandoffMemoV1::default(),
        )
    }

    pub(super) fn checked_object_return_handoff_with_memo_v1(
        &self,
        value: &TerminalValueReturnV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        memo: &mut ObjectReturnHandoffMemoV1,
    ) -> Result<ObjectReturnHandoffAvailabilityV1, String> {
        self.checked_object_return_handoff_path_v1(
            value,
            selected,
            contracts,
            signatures,
            &mut BTreeSet::new(),
            memo,
        )
    }

    fn checked_object_return_handoff_path_v1(
        &self,
        value: &TerminalValueReturnV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        path: &mut BTreeSet<(FunctionOwnerIdV1, SourceStmtSiteV1)>,
        memo: &mut ObjectReturnHandoffMemoV1,
    ) -> Result<ObjectReturnHandoffAvailabilityV1, String> {
        use ObjectReturnHandoffAvailabilityV1::{Unavailable, Verified};
        let identity = (value.owner(), value.return_site().clone());
        if !path.insert(identity.clone()) {
            return Ok(Unavailable(ObjectReturnHandoffUnavailableV1::RecursivePath));
        }
        let result = (|| {
            let Some(acquisition) = self
                .checked_completed_object_acquisition_v1(value, selected, contracts, signatures)?
            else {
                return Ok(Unavailable(ObjectReturnHandoffUnavailableV1::Acquisition));
            };
            let loan = acquisition.original().qualification();
            let Some(source) = &self.borrowed_formal_source else {
                return Ok(Unavailable(ObjectReturnHandoffUnavailableV1::Acquisition));
            };
            let source = source.as_ref().map_err(Clone::clone)?;
            let Some(target) = source.object_source_target_at_v1(loan.call())? else {
                return Ok(Unavailable(ObjectReturnHandoffUnavailableV1::Acquisition));
            };
            let Some(callee) = self.checked_completed_object_callee_exits_v1(target, loan)? else {
                return Ok(Unavailable(
                    ObjectReturnHandoffUnavailableV1::CalleeCompletion,
                ));
            };
            if let Some((original, proof)) = memo.completed.get(&identity) {
                if original != value {
                    return Err(freeze("handoff-memo-terminal-identity"));
                }
                return Ok(proof.clone());
            }
            let mut alternatives = Vec::new();
            let mut missing = None;
            for (witness, terminal) in callee.iter() {
                match witness.step() {
                    ResultWitnessStepV1::FreshConstruction | ResultWitnessStepV1::NullLiteral => {
                        match self.checked_object_return_leaf_v1(loan.key(), witness)? {
                            Some(leaf) => {
                                alternatives.push(VerifiedObjectReturnAlternativeV1::Leaf(leaf))
                            }
                            None => {
                                missing.get_or_insert(ObjectReturnHandoffUnavailableV1::Leaf);
                            }
                        }
                    }
                    ResultWitnessStepV1::Call { .. } => {
                        let TerminalReturnedSourceV1::OwnedCall(original) = terminal.returned()
                        else {
                            return Err(freeze("handoff-call-terminal"));
                        };
                        if !original
                            .qualification()
                            .witnesses()
                            .iter()
                            .any(|candidate| Rc::ptr_eq(candidate, witness))
                        {
                            return Err(freeze("handoff-call-membership"));
                        }
                        match self.checked_object_return_handoff_path_v1(
                            terminal, selected, contracts, signatures, path, memo,
                        )? {
                            Verified(child) => {
                                alternatives.push(VerifiedObjectReturnAlternativeV1::Call(child))
                            }
                            Unavailable(reason) => {
                                missing.get_or_insert(reason);
                            }
                        }
                    }
                    ResultWitnessStepV1::Formal { .. } => {
                        missing.get_or_insert(ObjectReturnHandoffUnavailableV1::BorrowedOrigin);
                    }
                }
            }
            if let Some(reason) = missing {
                return Ok(Unavailable(reason));
            }
            let teardown = teardown::reduce(&alternatives)?;
            Ok(Verified(Rc::new(VerifiedObjectReturnHandoffV1 {
                teardown,
                acquisition,
                alternatives: alternatives.into_boxed_slice(),
            })))
        })();
        path.remove(&identity);
        if let Ok(availability) = &result {
            memo.completed
                .insert(identity, (value.clone(), availability.clone()));
        }
        result
    }
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/object-return/{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_return_handoff_tests.rs"]
mod tests;
