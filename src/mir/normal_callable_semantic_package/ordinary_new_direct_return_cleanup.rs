//! Retain the SAME checked Direct handoff without exposing a proof constructor.
use super::super::local_commit::RootHomeReleaseSubjectV1;
use super::return_handoff::VerifiedObjectReturnHandoffV1;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceStmtSiteV1};
use crate::mir::ValueId;
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct DirectRootCleanupSourceV1
{
    proof: DirectReturnProofV1,
    kind: InvokeCallResultKind,
}
#[derive(Debug)]
enum DirectReturnProofV1 {
    Owned(Rc<VerifiedObjectReturnHandoffV1>),
    Borrowed(Box<crate::mir::normal_callable_semantic_package::BorrowedCallSourceLoanV1>),
}
impl DirectRootCleanupSourceV1 {
    pub(super) fn from_checked_view(
        proof: Rc<VerifiedObjectReturnHandoffV1>,
        kind: InvokeCallResultKind,
    ) -> Self {
        Self { proof: DirectReturnProofV1::Owned(proof), kind }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn from_checked_borrowed(
        source: &crate::mir::normal_callable_semantic_package::BorrowedCallSourceLoanV1,
    ) -> Self {
        Self { proof: DirectReturnProofV1::Borrowed(Box::new(source.clone())), kind: source.result_kind_v1() }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn borrowed(&self)
        -> Option<&crate::mir::normal_callable_semantic_package::BorrowedCallSourceLoanV1> {
        match &self.proof { DirectReturnProofV1::Borrowed(source) => Some(source), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn call(
        &self,
    ) -> &OwnedExprSiteV1 {
        match &self.proof { DirectReturnProofV1::Owned(proof) => proof.acquisition().original().qualification().call(),
            DirectReturnProofV1::Borrowed(source) => source.call() }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn exit(
        &self,
    ) -> &SourceStmtSiteV1 {
        match &self.proof { DirectReturnProofV1::Owned(proof) => proof.acquisition().original().return_site(),
            DirectReturnProofV1::Borrowed(source) => source.return_statement().expect("checked direct Return") }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn kind(
        &self,
    ) -> InvokeCallResultKind {
        self.kind
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn end_plan(
        &self,
        value: ValueId,
    ) -> Result<Vec<(RootHomeReleaseSubjectV1, InvokeOperation)>, String> {
        let proof = match &self.proof {
            DirectReturnProofV1::Owned(proof) => proof,
            DirectReturnProofV1::Borrowed(source) => {
                if !source.is_declared_read_v1() || source.local().is_some()
                    || source.return_statement().is_none() || self.kind != source.result_kind_v1() {
                    return Err("[freeze:contract][ordinary-new/direct-result-kind]".into());
                }
                return Ok(Vec::new()); // A borrowed result acquires no Home.
            }
        };
        let (descriptor, nullable) = proof.teardown().descriptor().ok_or_else(|| {
            "[freeze:contract][ordinary-new/direct-result-teardown-unavailable]".to_string()
        })?;
        if nullable && self.kind != InvokeCallResultKind::NullableHandle {
            return Err("[freeze:contract][ordinary-new/direct-result-null-kind]".into());
        }
        let site = self.call();
        let nullable = match self.kind {
            InvokeCallResultKind::Handle => false,
            InvokeCallResultKind::NullableHandle => true,
            _ => return Err("[freeze:contract][ordinary-new/direct-result-kind]".into()),
        };
        Ok(super::super::object_end::operations_v1(
            descriptor.object(), nullable, descriptor.children(), value,
        ).map(|(field, operation)| (
            match field {
                Some(field) => RootHomeReleaseSubjectV1::DirectResultField { site: site.clone(), field },
                None => RootHomeReleaseSubjectV1::DirectResult { site: site.clone() },
            },
            operation,
        )).collect())
    }
}
