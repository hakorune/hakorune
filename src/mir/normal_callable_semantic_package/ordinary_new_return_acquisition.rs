//! Original acquisition joins completed inputs; Fault and source snapshots stay immutable.
use super::super::{OrdinaryNewClaimLedgerV1, OrdinaryNewResultClassV1};
use crate::mir::normal_callable_semantic_package::{
    model::OwnedCallableParameterContractDeclarationV1,
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallArgumentV1, LocalCallResultClassV1, ObjectCallSourceSupportV1,
    ObjectReturnAcquisitionV1, SelectedNewArgumentKindV1, TerminalObjectReturnObligationV1,
    TerminalRelationV1, TerminalReturnedSourceV1, TerminalValueReturnV1,
};
use crate::mir::resolved_semantics::OwnedExprSiteV1;

#[derive(Debug)]
pub(super) struct VerifiedObjectReturnSourceAcquisitionV1 {
    original: TerminalObjectReturnObligationV1,
    arguments: Box<[LocalCallArgumentV1]>,
}
impl VerifiedObjectReturnSourceAcquisitionV1 {
    pub(super) fn original(&self) -> &TerminalObjectReturnObligationV1 {
        &self.original
    }
    pub(super) fn arguments(&self) -> &[LocalCallArgumentV1] {
        &self.arguments
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn checked_completed_object_acquisition_v1(
        &self,
        value: &TerminalValueReturnV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    ) -> Result<Option<VerifiedObjectReturnSourceAcquisitionV1>, String> {
        let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
            return Ok(None);
        };
        let loan = original.qualification();
        if self
            .callable_result_classes
            .object_return_qualification(loan.value())
            .as_ref()
            != Some(loan)
            || loan.value() != &OwnedExprSiteV1::new(value.owner(), value.value_site().clone())
            || loan.call().owner() != value.owner()
            || original.return_site() != value.return_site()
        {
            return Err(freeze("qualification"));
        }
        let Some(completion) = self.completion_index.get(&value.owner()) else {
            return Ok(None);
        };
        let completion = completion
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("completion")))?;
        if completion.owner() != value.owner()
            || !completion.explicit_sites().contains(value.return_site())
            || !matches!(self.terminal_relation_for_owner_at(value.owner(), value.return_site()), Some(TerminalRelationV1::Value(current)) if current == value)
        {
            return Err(freeze("exit-identity"));
        }
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(None);
        };
        let Some(exit) = flow.exit_row(value.return_site()) else {
            return Ok(None);
        };
        let exit = exit
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("exit")))?;
        if exit.homes() != original.exit_homes() {
            return Err(freeze("exit-homes"));
        }
        let Some(source) = &self.borrowed_formal_source else {
            return Ok(None);
        };
        let source = source.as_ref().map_err(Clone::clone)?;
        let Some(target) = source.object_source_target_at_v1(loan.call())? else {
            return Ok(None);
        };
        let mut rows = contracts
            .iter()
            .filter(|row| row.owner == target.callee_owner());
        let contract = rows.next().ok_or_else(|| freeze("contract-missing"))?;
        if rows.next().is_some() {
            return Err(freeze("contract-duplicate"));
        }
        let typed = contract.parameters.iter().all(|row| matches!(row.kind,
            crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactTrivial(abi)
            if abi == crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64));
        let arguments = if typed {
            self.checked_completed_typed_object_arguments_v1(
                target, loan, selected, contracts, signatures,
            )?
        } else {
            self.checked_completed_opaque_object_arguments_v1(target, loan, contract)?
        };
        let Some(arguments) = arguments else {
            return Ok(None);
        };
        match original.arguments() {
            ObjectCallSourceSupportV1::Unavailable => return Ok(None),
            ObjectCallSourceSupportV1::Observed(snapshot)
            | ObjectCallSourceSupportV1::SourceOnly(snapshot) => {
                if snapshot.as_ref() != arguments {
                    return Err(freeze("argument-snapshot-drift"));
                }
            }
        }
        match original.acquisition() {
            ObjectReturnAcquisitionV1::Direct {
                argument_sites,
                fault_homes,
            } => {
                if loan.call() != loan.value()
                    || fault_homes.as_ref() != exit.homes()
                    || argument_sites.len() != target.argument_sites().len()
                    || argument_sites
                        .iter()
                        .zip(target.argument_sites())
                        .any(|(original, site)| {
                            original.owner() != value.owner() || original.site() != site
                        })
                {
                    return Err(freeze("direct-identity"));
                }
            }
            ObjectReturnAcquisitionV1::Received(acquisition) => {
                let Some((_, binding)) = acquisition.local_binding() else {
                    return Err(freeze("received-destination"));
                };
                let mut calls = flow
                    .local_calls()
                    .iter()
                    .filter(|call| call.site() == loan.call());
                if loan.call() == loan.value()
                    || acquisition.owner() != value.owner()
                    || acquisition.site() != loan.call()
                    || binding.owner() != value.owner()
                    || exit.homes().iter().filter(|home| **home == binding).count() != 1
                    || !exit.covered_calls().contains(loan.call())
                    || calls.next() != Some(acquisition)
                    || calls.next().is_some()
                    || !matches!(
                        (loan.class(), acquisition.result()),
                        (
                            OrdinaryNewResultClassV1::Object(_),
                            LocalCallResultClassV1::Handle
                        ) | (
                            OrdinaryNewResultClassV1::NullableObject(_),
                            LocalCallResultClassV1::Nullable
                        )
                    )
                {
                    return Err(freeze("received-identity"));
                }
                if target.is_self_receiver() {
                    let Some(receiver) = self.receiver_call_observation(loan.call()) else {
                        return Ok(None);
                    };
                    if receiver.callee() != loan.key()
                        || receiver.class() != loan.class()
                        || receiver.destination() != binding
                        || receiver.arguments().len() != arguments.len()
                        || !acquisition.arguments().is_empty()
                    {
                        return Err(freeze("receiver-identity"));
                    }
                    for (ordinal, (row, argument)) in
                        receiver.arguments().iter().zip(arguments).enumerate()
                    {
                        if row.ordinal() as usize != ordinal
                            || target.argument_sites().get(ordinal) != Some(row.site())
                        {
                            return Err(freeze("receiver-arguments"));
                        }
                        let equal = match (row.kind(), argument) {
                            (
                                SelectedNewArgumentKindV1::Integer(a),
                                LocalCallArgumentV1::Integer(b),
                            ) => a == b,
                            (SelectedNewArgumentKindV1::Bool(a), LocalCallArgumentV1::Bool(b)) => {
                                a == b
                            }
                            (
                                SelectedNewArgumentKindV1::Local { binding },
                                LocalCallArgumentV1::Scalar(actual),
                            ) => binding == actual,
                            (
                                _,
                                LocalCallArgumentV1::BorrowedActual {
                                    ordinal: actual_ordinal,
                                    site,
                                },
                            ) => {
                                *actual_ordinal as usize == ordinal
                                    && site == row.site()
                                    && self.checked_completed_opaque_receiver_argument_v1(
                                        loan.call(),
                                        *actual_ordinal,
                                        row.kind(),
                                    )?
                            }
                            _ => false,
                        };
                        if !equal {
                            return Err(freeze("receiver-arguments"));
                        }
                    }
                } else if acquisition.arguments() != arguments {
                    return Err(freeze("received-arguments"));
                }
            }
        }
        Ok(Some(VerifiedObjectReturnSourceAcquisitionV1 {
            original: original.as_ref().clone(),
            arguments: arguments.into(),
        }))
    }
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/object-return/acquisition-{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_return_acquisition_tests.rs"]
mod tests;
