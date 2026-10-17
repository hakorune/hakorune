//! Same original field-Array append and canonical Home participation.
//! This immutable source loan does not issue an opcode or runtime representation.
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{NamedArrayFieldResidenceRequirementV1, NamedArrayRequirementV1,
    VerifiedResolverCoreMethodCallableContractV1, CoreMethodHomeResultRelationV1,
    CoreMethodHomeParameterRelationV1, CoreMethodHomeSchemaV1, ResolvedLoopPlacementV1, OwnedExprSiteV1};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct FieldArrayAppendSourceLoanV1 {
    proof: Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>,
    index: usize,
    receiver_site: crate::mir::resolved_semantics::SourceExprSiteV1,
    completion: Rc<VerifiedFunctionCompletionV1>,
}
impl FieldArrayAppendSourceLoanV1 {
    pub(crate) fn proof(&self) -> &Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1> { &self.proof }
    pub(crate) fn receiver_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 { &self.receiver_site }
    pub(crate) fn index(&self) -> usize { self.index }
    pub(crate) fn requirement(&self) -> &Rc<NamedArrayFieldResidenceRequirementV1> { self.proof.appends()[self.index].requirement() }
    pub(crate) fn argument(&self) -> &crate::mir::builder::VerifiedNoExitIntegerExpressionV1 { self.proof.appends()[self.index].argument() }
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 { self.proof.appends()[self.index].fault().call() }
    pub(crate) fn matches_original(&self, proof: &Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>, index: usize) -> bool {
        Rc::ptr_eq(&self.proof, proof) && self.index == index
    }
    pub(crate) fn validate_source(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        let owner = self.site().owner();
        if self.completion.owner() != owner || self.proof.owner() != owner
            || !ledger.completion_source_for_owner_v1(owner).is_some_and(|original| Rc::ptr_eq(original, &self.completion))
            || !ledger.no_exit_array_home_source_v1(owner, self.proof.site()).is_some_and(|original| Rc::ptr_eq(original, &self.proof))
        { return Err(fault("source-owner")); }
        let flow = self.completion.cleanup().root_flow().ok_or_else(|| fault("home-flow-missing"))?;
        if flow.field_array_loop_sources().filter(|original| Rc::ptr_eq(original, &self.proof)).count() != 1
            || self.proof.appends().get(self.index).is_none_or(|append|
                append.requirement().owner() != owner || append.fault().call() != self.site()
                    || append.fault().loop_site() != self.proof.site())
        { return Err(fault("home-source-membership")); }
        Ok(())
    }
}
impl OrdinaryNewClaimLedgerV1 {
    /// Selected SourcePublication FieldResidence calls demand this loan. Missing
    /// source or unavailable Home meaning is rejection, never raw-route selection.
    pub(crate) fn field_array_append_source_v1(
        &self, contract: &VerifiedResolverCoreMethodCallableContractV1,
    ) -> Result<FieldArrayAppendSourceLoanV1, String> {
        let Some(NamedArrayRequirementV1::FieldResidence(requirement)) = contract.named_array_requirement() else {
            return Err(fault("field-residence-required"));
        };
        let target = contract.target();
        if target.row().row().op != crate::mir::core_method_op::CoreMethodOp::ArrayPush
            || target.row().arity() != 1 || target.schema() != CoreMethodHomeSchemaV1::ArrayIntegerAppend
            || target.result() != CoreMethodHomeResultRelationV1::NoValue
            || target.parameters() != [CoreMethodHomeParameterRelationV1::I64Parameter]
            || contract.placement() != ResolvedLoopPlacementV1::Body
            || contract.call_site() != requirement.call() || contract.arguments().len() != 1
            || contract.arguments()[0].site() != requirement.argument()
            || contract.owner() != requirement.owner()
        { return Err(fault("original-call")); }
        let proof = self.no_exit_array_home_source_v1(contract.owner(), contract.loop_site())
            .ok_or_else(|| fault("loop-source-missing"))?;
        if !contract.frame().matches(proof.frame()) { return Err(fault("loop-frame")); }
        let mut originals = proof.appends().iter().enumerate().filter(|(_, append)| Rc::ptr_eq(append.requirement(), requirement));
        let (index, _) = originals.next().ok_or_else(|| fault("append-source-missing"))?;
        if originals.next().is_some() { return Err(fault("append-source-duplicate")); }
        let completion = self.completion_source_for_owner_v1(contract.owner()).ok_or_else(|| fault("completion-missing"))?;
        let source = FieldArrayAppendSourceLoanV1 { proof: Rc::clone(proof), index, receiver_site: contract.receiver_site().clone(), completion: Rc::clone(completion) };
        source.validate_source(self)?;
        Ok(source)
    }
}
fn fault(reason: &str) -> String { format!("[freeze:contract][field-array-append/{reason}]") }
