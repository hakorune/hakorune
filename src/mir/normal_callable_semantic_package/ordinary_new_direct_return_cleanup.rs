//! Retain the SAME checked Direct handoff without exposing a proof constructor.
use super::super::local_commit::RootHomeReleaseSubjectV1;
use super::super::OwnedFieldChildKindV1;
use super::return_handoff::VerifiedObjectReturnHandoffV1;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceStmtSiteV1};
use crate::mir::ValueId;
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct DirectRootCleanupSourceV1
{
    proof: Rc<VerifiedObjectReturnHandoffV1>,
    kind: InvokeCallResultKind,
}
impl DirectRootCleanupSourceV1 {
    pub(super) fn from_checked_view(
        proof: Rc<VerifiedObjectReturnHandoffV1>,
        kind: InvokeCallResultKind,
    ) -> Self {
        Self { proof, kind }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn call(
        &self,
    ) -> &OwnedExprSiteV1 {
        self.proof.acquisition().original().qualification().call()
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn exit(
        &self,
    ) -> &SourceStmtSiteV1 {
        self.proof.acquisition().original().return_site()
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
        let (descriptor, nullable) = self.proof.teardown().descriptor().ok_or_else(|| {
            "[freeze:contract][ordinary-new/direct-result-teardown-unavailable]".to_string()
        })?;
        if nullable && self.kind != InvokeCallResultKind::NullableHandle {
            return Err("[freeze:contract][ordinary-new/direct-result-null-kind]".into());
        }
        let site = self.call();
        let mut plan: Vec<_> = descriptor
            .children()
            .iter()
            .rev()
            .map(|child| {
                (
                    RootHomeReleaseSubjectV1::DirectResultField {
                        site: site.clone(),
                        field: child.field,
                    },
                    match child.kind {
                        OwnedFieldChildKindV1::Array => {
                            InvokeOperation::OwnedFieldResidenceRelease {
                                field: child.field,
                                base: value,
                            }
                        }
                        OwnedFieldChildKindV1::Object(object) => {
                            InvokeOperation::OwnedObjectFieldRelease {
                                field: child.field,
                                base: value,
                                child: object,
                            }
                        }
                    },
                )
            })
            .collect();
        let release = match self.kind {
            InvokeCallResultKind::Handle => InvokeOperation::HomeRelease {
                object: descriptor.object(),
                value,
            },
            InvokeCallResultKind::NullableHandle => InvokeOperation::HomeReleaseIfLive {
                object: descriptor.object(),
                value,
            },
            _ => return Err("[freeze:contract][ordinary-new/direct-result-kind]".into()),
        };
        plan.push((
            RootHomeReleaseSubjectV1::DirectResult { site: site.clone() },
            release,
        ));
        Ok(plan)
    }
}
