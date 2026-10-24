//! Exact original result cohort joined to the completed-index source proof.
use super::super::{
    lexical_instance_call::LexicalInstanceCallSourceTargetV1,
    result_class_claim::ResultWitnessStepV1, OrdinaryNewClaimLedgerV1, OrdinaryNewResultClassV1,
};
use super::normal_return::NormalReturnDispositionV1;
use super::return_handoff::{
    reduce_object_return_teardown_v1, ObjectReturnTeardownAvailabilityV1,
    VerifiedObjectReturnAlternativeV1,
};
use super::return_leaf::VerifiedObjectReturnLeafV1;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1;
use crate::mir::resolved_control_flow::DeclaredFunctionResultContractV1;
use crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1;
use std::rc::Rc;

/// One original result census occurrence, including Null/Fresh with no supports.
/// A Call occurrence borrows its whole conditional handoff requirements; the
/// root is an identity token, never permission to prune those requirements.
#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct ObjectCallFieldSupportRequirementV1 {
    root: Rc<super::super::result_class_claim::ResultOriginWitnessV1>,
    exit: crate::mir::resolved_semantics::SourceStmtSiteV1,
    supports: Box<[super::return_leaf::ObjectReturnFieldSupportV1]>,
}
impl ObjectCallFieldSupportRequirementV1 {
    /// Join requirements to this call's original actual storage, without
    /// changing its source kind or issuing an executable/lifetime permission.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn visit_actuals_v1(
        requirements: &[Self],
        target: &LexicalInstanceCallSourceTargetV1,
        actuals: &[super::super::lexical_instance_call::PreparedBorrowedFormalActualV1],
        mut visit: impl FnMut(
            &super::return_leaf::ObjectReturnFieldSupportV1,
            &super::super::lexical_instance_call::PreparedBorrowedFormalActualV1,
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        for requirement in requirements.iter().flat_map(|group| group.supports_v1()) {
            let mut matching = actuals.iter().filter(|actual| actual.ordinal == requirement.ordinal);
            let actual = matching.next().ok_or_else(|| freeze("object-packet/field-support-actual-missing"))?;
            if matching.next().is_some() || actual.formal != requirement.formal
                || requirement.formal.owner() != target.callee_owner()
                || target.argument_sites().get(requirement.ordinal as usize) != Some(&actual.site) {
                return Err(freeze("object-packet/field-support-actual-identity"));
            }
            visit(requirement, actual)?;
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn supports_v1(&self) -> &[super::return_leaf::ObjectReturnFieldSupportV1] {
        &self.supports
    }
}
impl PartialEq for ObjectCallFieldSupportRequirementV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.root, &other.root) && self.exit == other.exit && self.supports == other.supports
    }
}
impl Eq for ObjectCallFieldSupportRequirementV1 {}

/// Existing full-result proof with its original, ordered field support census.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct CheckedObjectCalleeSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) kind: InvokeCallResultKind,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) teardown: ObjectReturnTeardownAvailabilityV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) field_supports: Box<[ObjectCallFieldSupportRequirementV1]>,
}

impl OrdinaryNewClaimLedgerV1 {
    /// Array producer demand uses the same full result/Normal/End owner.
    /// The immutable source loan is rechecked before any capability is observed.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn checked_array_producer_teardown_v1(
        &self,
        producer: &super::super::result_class_claim::ObjectCallSourceLoanV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<Option<super::return_leaf::ObjectReturnTeardownDescriptorV1>, String> {
        let target = producer.source();
        self.callable_result_classes.recheck_object_call_roots_v1(target, producer)
            .map_err(|issue| freeze(issue))?;
        let Some((kind, teardown)) = self.checked_object_callee_result_with_teardown_v1(target, results)? else {
            return Ok(None);
        };
        let nullable = match producer.result_class() {
            OrdinaryNewResultClassV1::Object(_) => false,
            OrdinaryNewResultClassV1::NullableObject(_) => true,
            _ => return Err(freeze("array-producer-class")),
        };
        if kind != if nullable { InvokeCallResultKind::NullableHandle } else { InvokeCallResultKind::Handle } {
            return Err(freeze("array-producer-kind"));
        }
        let Some((descriptor, observed_nullable)) = teardown.descriptor() else {
            return Err(freeze("array-producer-end-unavailable"));
        };
        if observed_nullable != nullable {
            return Err(freeze("array-producer-nullability"));
        }
        Ok(Some(descriptor.clone()))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn checked_object_callee_result_v1(
        &self,
        source: &LexicalInstanceCallSourceTargetV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<Option<InvokeCallResultKind>, String> {
        self.checked_object_callee_result_with_teardown_v1(source, results)
            .map(|checked| checked.map(|(kind, _)| kind))
    }

    /// The same full-exit proof retains the exact checked teardown alternatives.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn checked_object_callee_result_with_teardown_v1(
        &self,
        source: &LexicalInstanceCallSourceTargetV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<Option<(InvokeCallResultKind, ObjectReturnTeardownAvailabilityV1)>, String> {
        self.checked_object_callee_source_v1(source, results)
            .map(|checked| checked.map(|source| (source.kind, source.teardown)))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn checked_object_callee_source_v1(
        &self,
        source: &LexicalInstanceCallSourceTargetV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<Option<CheckedObjectCalleeSourceV1>, String> {
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
        let ancestry = self.checked_object_callee_ancestry_v1(source)?;
        let Some((alternatives, field_supports)) = ancestry else {
            return Ok(None);
        };
        if missing {
            return Ok(None);
        }
        let teardown = reduce_object_return_teardown_v1(&alternatives)?;
        Ok(Some(CheckedObjectCalleeSourceV1 { kind, teardown, field_supports }))
    }

    fn checked_object_callee_ancestry_v1(
        &self, source: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<Option<(Vec<VerifiedObjectReturnAlternativeV1>, Box<[ObjectCallFieldSupportRequirementV1]>)>, String> {
        let Some(exits) = self.checked_completed_object_callee_target_exits_v1(source)? else {
            return Ok(None);
        };
        let mut alternatives = Vec::new();
        let mut field_supports = Vec::new();
        let mut missing = false;
        for (witness, value) in exits.iter() {
            let mut supports = Vec::new();
            match witness.step() {
                ResultWitnessStepV1::FreshConstruction | ResultWitnessStepV1::NullLiteral => {
                    match self.checked_object_return_leaf_v1(source.target(), witness)? {
                        None => {
                            missing = true;
                        }
                        Some(VerifiedObjectReturnLeafV1::Null)
                            if !matches!(self.callable_result_classes.get(source.target()),
                                Some(OrdinaryNewResultClassV1::NullableObject(_))) =>
                        {
                            return Err(freeze("null-result-kind"))
                        }
                        Some(leaf) => {
                            supports.extend(self.object_return_leaf_supports_v1(&leaf)?);
                            alternatives.push(VerifiedObjectReturnAlternativeV1::Leaf(leaf))
                        }
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
                            if proof.acquisition().original() == original.as_ref() =>
                        {
                            missing |= !self.recheck_object_return_field_supports_v1(proof)?;
                            supports.extend_from_slice(proof.field_supports_v1());
                            alternatives
                                .push(VerifiedObjectReturnAlternativeV1::Call(Rc::clone(proof)));
                        }
                        Some(NormalReturnDispositionV1::Verified { .. }) => {
                            return Err(freeze("call-disposition-identity"))
                        }
                        _ => {
                            missing = true;
                        }
                    }
                }
                ResultWitnessStepV1::Formal { .. } | ResultWitnessStepV1::MapStored { .. } => missing = true,
            }
            field_supports.push(ObjectCallFieldSupportRequirementV1 {
                root: Rc::clone(witness), exit: value.return_site().clone(), supports: supports.into_boxed_slice(),
            });
            let exit_ready = self.normal_exit_projection_v1(value.owner(), value.return_site())?.is_some();
            missing |= !exit_ready;
        }
        if missing {
            return Ok(None);
        }
        Ok(Some((alternatives, field_supports.into_boxed_slice())))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn recheck_object_call_field_support_requirements_v1(
        &self, source: &LexicalInstanceCallSourceTargetV1,
        saved: &[ObjectCallFieldSupportRequirementV1],
    ) -> Result<(), String> {
        let (_, current) = self.checked_object_callee_ancestry_v1(source)?
            .ok_or_else(|| freeze("field-support-source-unavailable"))?;
        if current.as_ref() != saved { return Err(freeze("field-support-source-drift")); }
        Ok(())
    }
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/object-return/result-{reason}]")
}
#[cfg(test)]
#[path = "ordinary_new_return_result_tests.rs"]
mod tests;
