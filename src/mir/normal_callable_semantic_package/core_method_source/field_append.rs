//! Same original field-Array append and canonical Home participation.
//! This immutable source loan does not issue an opcode or runtime representation.
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{NamedArrayFieldResidenceRequirementV1, NamedArrayRequirementV1,
    VerifiedResolverCoreMethodCallableContractV1, CoreMethodHomeResultRelationV1,
    CoreMethodHomeParameterRelationV1, CoreMethodHomeSchemaV1, ResolvedLoopPlacementV1, OwnedExprSiteV1};
use std::rc::Rc;

#[derive(Debug, Clone)]
enum FieldArrayAppendOriginalV1 {
    LoopInteger { proof: Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>, index: usize },
    StatementText(Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedFieldArrayTextAppendV1>),
}

#[derive(Debug, Clone)]
pub(crate) struct FieldArrayAppendSourceLoanV1 {
    original: FieldArrayAppendOriginalV1,
    receiver_site: crate::mir::resolved_semantics::SourceExprSiteV1,
    completion: Rc<VerifiedFunctionCompletionV1>,
}
impl FieldArrayAppendSourceLoanV1 {
    pub(crate) fn text_frame(&self) -> Option<&Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedFieldArrayTextAppendV1>> {
        match &self.original { FieldArrayAppendOriginalV1::StatementText(frame) => Some(frame), _ => None }
    }
    pub(crate) fn same_source(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.completion, &other.completion) && match &other.original {
            FieldArrayAppendOriginalV1::LoopInteger { proof, index } => self.matches_original(proof, *index),
            FieldArrayAppendOriginalV1::StatementText(frame) => self.matches_text_frame(frame),
        }
    }
    pub(crate) fn receiver_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 { &self.receiver_site }
    pub(crate) fn integer_requirement(&self) -> Result<&Rc<NamedArrayFieldResidenceRequirementV1>, String> {
        match &self.original {
            FieldArrayAppendOriginalV1::LoopInteger { proof, index } => Ok(proof.appends()[*index].requirement()),
            FieldArrayAppendOriginalV1::StatementText(_) => Err(fault("original-call")),
        }
    }
    pub(crate) fn integer_argument(&self) -> Result<&crate::mir::builder::VerifiedNoExitIntegerExpressionV1, String> {
        match &self.original {
            FieldArrayAppendOriginalV1::LoopInteger { proof, index } => Ok(proof.appends()[*index].argument()),
            FieldArrayAppendOriginalV1::StatementText(_) => Err(fault("original-call")),
        }
    }
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        match &self.original {
            FieldArrayAppendOriginalV1::LoopInteger { proof, index } => proof.appends()[*index].fault().call(),
            FieldArrayAppendOriginalV1::StatementText(frame) => frame.source().call(),
        }
    }
    pub(crate) fn prior_homes(&self) -> &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1] {
        match &self.original {
            FieldArrayAppendOriginalV1::LoopInteger { proof, .. } => proof.prior_homes(),
            FieldArrayAppendOriginalV1::StatementText(frame) => frame.evaluation().prior_homes(),
        }
    }
    pub(crate) fn matches_original(&self, proof: &Rc<crate::mir::builder::VerifiedNoExitArrayHomeAfterV1>, index: usize) -> bool {
        matches!(&self.original, FieldArrayAppendOriginalV1::LoopInteger { proof: original, index: ordinal }
            if Rc::ptr_eq(original, proof) && *ordinal == index)
    }
    pub(crate) fn matches_text_frame(&self, frame: &Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedFieldArrayTextAppendV1>) -> bool {
        matches!(&self.original, FieldArrayAppendOriginalV1::StatementText(original) if Rc::ptr_eq(original, frame))
    }
    pub(crate) fn validate_source(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        let owner = self.site().owner();
        if self.completion.owner() != owner
            || !ledger.completion_source_for_owner_v1(owner).is_some_and(|original| Rc::ptr_eq(original, &self.completion))
        { return Err(fault("source-owner")); }
        let flow = self.completion.cleanup().root_flow().ok_or_else(|| fault("home-flow-missing"))?;
        match &self.original {
            FieldArrayAppendOriginalV1::LoopInteger { proof, index } => {
                if proof.owner() != owner
                    || !ledger.no_exit_array_home_source_v1(owner, proof.site()).is_some_and(|original| Rc::ptr_eq(original, proof))
                    || flow.field_array_loop_sources().filter(|original| Rc::ptr_eq(original, proof)).count() != 1
                    || proof.appends().get(*index).is_none_or(|append|
                        append.requirement().owner() != owner || append.fault().call() != self.site()
                            || append.fault().loop_site() != proof.site())
                { return Err(fault("home-source-membership")); }
            }
            FieldArrayAppendOriginalV1::StatementText(frame) => {
                use crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1;
                use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1;
                let evaluation = frame.evaluation();
                let Some(call) = frame.source().append_method() else { return Err(fault("original-call")) };
                if flow.named_arrays().iter().filter(|row| matches!(row, NamedArrayHomeObservationV1::FieldTextAppend(original)
                    if Rc::ptr_eq(original, frame))).count() != 1
                    || flow.named_arrays().iter().filter(|row| matches!(row, NamedArrayHomeObservationV1::FieldTextAppend(original)
                        if original.source().call() == self.site())).count() != 1
                    || frame.receiver_read() != self.receiver_site() || frame.receiver().owner() != owner
                    || evaluation.source().owner() != owner || !evaluation.reference_coverage_is_exact()
                    || !matches!(evaluation.source().root(), ExpressionPlanRootV1::CallArgument { statement, call: original, ordinal: 0 }
                        if statement == frame.statement() && std::sync::Arc::ptr_eq(original, call))
                    || call.owner() != owner || call.site() != self.site().site()
                    || call.arguments().first().is_none_or(|argument| evaluation.source().nodes().first()
                        .is_none_or(|node| argument.site() != node.site()))
                    || evaluation.root_class() != crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Value(
                        crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1::String)
                    || !evaluation.calls().is_empty() || !evaluation.cores().is_empty()
                    || !evaluation.operations().is_empty() || !evaluation.owned_result_nodes().is_empty()
                { return Err(fault("home-source-membership")); }
                frame.source().recheck_census(owner, &ledger.array_i64_fields)?;
            }
        }
        Ok(())
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn field_array_text_append_at_v1(&self, site: &OwnedExprSiteV1)
        -> Result<Option<FieldArrayAppendSourceLoanV1>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1;
        let Some(completion) = self.completion_source_for_owner_v1(site.owner()) else { return Ok(None) };
        let Some(flow) = completion.cleanup().root_flow() else { return Ok(None) };
        if !flow.named_arrays().iter().any(|row| matches!(row,
            NamedArrayHomeObservationV1::FieldTextAppend(frame) if frame.source().call() == site)) { return Ok(None); }
        self.field_array_text_append_source_v1(site).map(Some)
    }
    /// Borrow the same original statement frame; never fabricate a Loop
    /// contract, Integer expression, Home transfer or physical admission.
    pub(crate) fn field_array_text_append_source_v1(&self, site: &OwnedExprSiteV1) -> Result<FieldArrayAppendSourceLoanV1, String> {
        use crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1;
        let completion = self.completion_source_for_owner_v1(site.owner()).ok_or_else(|| fault("completion-missing"))?;
        let flow = completion.cleanup().root_flow().ok_or_else(|| fault("home-flow-missing"))?;
        let mut frames = flow.named_arrays().iter().filter_map(|row| match row {
            NamedArrayHomeObservationV1::FieldTextAppend(frame) if frame.source().call() == site => Some(frame), _ => None,
        });
        let frame = frames.next().ok_or_else(|| fault("append-source-missing"))?;
        if frames.next().is_some() { return Err(fault("append-source-duplicate")); }
        let source = FieldArrayAppendSourceLoanV1 { original: FieldArrayAppendOriginalV1::StatementText(Rc::clone(frame)),
            receiver_site: frame.receiver_read().clone(), completion: Rc::clone(completion) };
        source.validate_source(self)?;
        Ok(source)
    }
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
        let source = FieldArrayAppendSourceLoanV1 { original: FieldArrayAppendOriginalV1::LoopInteger { proof: Rc::clone(proof), index }, receiver_site: contract.receiver_site().clone(), completion: Rc::clone(completion) };
        source.validate_source(self)?;
        Ok(source)
    }
}
fn fault(reason: &str) -> String { format!("[freeze:contract][field-array-append/{reason}]") }
