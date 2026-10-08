//! Exact original result cohort joined to the completed-index source proof.
use super::super::{
    lexical_instance_call::LexicalInstanceCallSourceTargetV1,
    result_class_claim::ResultWitnessStepV1, OrdinaryNewClaimLedgerV1, OrdinaryNewResultClassV1,
};
use super::normal_return::NormalReturnDispositionV1;
use super::return_leaf::VerifiedObjectReturnLeafV1;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1;
use crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1;
use crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1;
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn checked_object_callee_result_v1(
        &self,
        source: &LexicalInstanceCallSourceTargetV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<Option<InvokeCallResultKind>, String> {
        let row = results.row(source.target_batch_slot());
        if row.is_some_and(|row| row.owner() != source.callee_owner()) {
            return Err(freeze("cohort-owner"));
        }
        let Some(completion) = self.completion_index.get(&source.callee_owner()) else {
            return Ok(None);
        };
        let completion = completion
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("completion")))?;
        let Some(row) = row else {
            return Ok(None);
        };
        let contract = row.borrow();
        if completion.owner() != source.callee_owner()
            || !std::ptr::eq(contract.completion(), completion.as_ref())
        {
            return Err(freeze("cohort-completion-identity"));
        }
        let Some(relations) = self.terminal_relation_index.get(&source.callee_owner()) else {
            return Ok(None);
        };
        if !std::ptr::eq(contract.terminal_relations(), relations.as_ref()) {
            return Err(freeze("cohort-terminal-identity"));
        }
        if row.result().is_some()
            || completion.function_exit_contract().declared_result()
                != &DeclaredFunctionResultContractV1::Unannotated
        {
            return Err(freeze("cohort-result-contract"));
        }
        let Some(kind) = self
            .callable_result_classes
            .get(source.target())
            .and_then(|class| match class {
                OrdinaryNewResultClassV1::Object(_) => Some(InvokeCallResultKind::Handle),
                OrdinaryNewResultClassV1::NullableObject(_) => {
                    Some(InvokeCallResultKind::NullableHandle)
                }
                _ => None,
            })
        else {
            return Ok(None);
        };
        let original_dependencies = self
            .callable_result_classes
            .object_return_dependencies(source.target(), source.callee_owner());
        let mut missing = false;
        if let Some(dependencies) = source.object_producer_dependencies() {
            if original_dependencies.as_deref() != Some(dependencies) {
                return Err(freeze("producer-dependency-identity"));
            }
        } else if source.object_return_sources().is_none() && original_dependencies.is_some() {
            // Original producer membership is not yet retained on this target.
            missing = true;
        }
        if let Some(qualifications) = source.object_return_sources() {
            let original = self
                .callable_result_classes
                .qualifications_for_call(source.call_site(), source.target());
            if qualifications.is_empty() || qualifications != original.as_ref() {
                return Err(freeze("caller-qualification-identity"));
            }
            // Demand all original caller edges through the same verifier.
            for loan in qualifications {
                missing |= self
                    .checked_completed_object_callee_exits_v1(source, loan)?
                    .is_none();
            }
        }
        let Some(exits) = self.checked_completed_object_callee_target_exits_v1(source)? else {
            return Ok(None);
        };
        for (witness, value) in exits.iter() {
            match witness.step() {
                ResultWitnessStepV1::FreshConstruction | ResultWitnessStepV1::NullLiteral => {
                    match self.checked_object_return_leaf_v1(source.target(), witness)? {
                        None => missing = true,
                        Some(VerifiedObjectReturnLeafV1::Null)
                            if kind != InvokeCallResultKind::NullableHandle =>
                        {
                            return Err(freeze("null-result-kind"))
                        }
                        Some(_) => {}
                    }
                }
                ResultWitnessStepV1::Call { .. } => {
                    let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
                        return Err(freeze("call-terminal-role"));
                    };
                    if !original
                        .qualification()
                        .witnesses()
                        .iter()
                        .any(|candidate| Rc::ptr_eq(candidate, witness))
                    {
                        return Err(freeze("call-witness-membership"));
                    }
                    match self
                        .normal_return_dispositions
                        .as_ref()
                        .and_then(|map| map.get(&(value.owner(), value.return_site().clone())))
                    {
                        Some(NormalReturnDispositionV1::Verified { proof })
                            if proof.acquisition().original() == original.as_ref() => {}
                        Some(NormalReturnDispositionV1::Verified { .. }) => {
                            return Err(freeze("call-disposition-identity"))
                        }
                        _ => missing = true,
                    }
                }
                ResultWitnessStepV1::Formal { .. } => missing = true,
            }
            missing |= self
                .normal_exit_projection_v1(value.owner(), value.return_site())?
                .is_none();
        }
        Ok((!missing).then_some(kind))
    }
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/object-return/result-{reason}]")
}
#[cfg(test)]
#[path = "ordinary_new_return_result_tests.rs"]
mod tests;
