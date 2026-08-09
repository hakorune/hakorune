//! Source-bound Dynamic member target admission.
//!
//! This module owns message/source identity only. It does not issue a result
//! class, effect envelope, Home ABI, provider, Recipe item, or MIR value.

use crate::mir::builder::{
    issue_catalog_callable_owner_link_v1, issue_source_backed_dynamic_callable_v1,
    CanonicalSameModuleCallableKeyV1, CatalogCallableOwnerLinkIssueV1,
    VerifiedCatalogCallableOwnerLinkV1, VerifiedNormalCallableSemanticSourceV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1,
    SourceExprSiteV1,
};

use super::{VerifiedSourceCallTargetCatalogV1, VerifiedSourceCallTargetV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DynamicMemberSourceUnresolvedV1 {
    DynamicOriginEvidenceUnavailable(Box<str>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DynamicMemberSourceRejectV1 {
    ForeignCatalogCallable(CanonicalSameModuleCallableKeyV1),
    CallerOwnerMismatch {
        expected: FunctionOwnerIdV1,
        actual: FunctionOwnerIdV1,
    },
    ReceiverBindingOwnerMismatch(BindingRefV1),
    ArgumentResultRelationMismatch(SourceExprSiteV1),
    DuplicateOrCollidingTarget {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    CatalogCallableOwnerLink(CatalogCallableOwnerLinkIssueV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DynamicMemberSourceIssueV1 {
    Unresolved(DynamicMemberSourceUnresolvedV1),
    Rejected(DynamicMemberSourceRejectV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DynamicMemberDispatchKeyV1 {
    selector: Box<str>,
    arity: u32,
}

impl DynamicMemberDispatchKeyV1 {
    pub(crate) fn selector(&self) -> &str {
        &self.selector
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DynamicMemberArgumentSourceV1 {
    ordinal: u32,
    site: SourceExprSiteV1,
}

impl DynamicMemberArgumentSourceV1 {
    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }
}

/// Exact Dynamic member message/source relation for one call site.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedSourceBoundDynamicMemberCallV1 {
    owner: FunctionOwnerIdV1,
    call_site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver_binding: BindingRefV1,
    dynamic_origin: BindingRefV1,
    arguments: Box<[DynamicMemberArgumentSourceV1]>,
    result_site: SourceExprSiteV1,
    dispatch: DynamicMemberDispatchKeyV1,
}

impl VerifiedSourceBoundDynamicMemberCallV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(crate) const fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) const fn receiver_binding(&self) -> BindingRefV1 {
        self.receiver_binding
    }

    pub(crate) const fn dynamic_origin(&self) -> BindingRefV1 {
        self.dynamic_origin
    }

    pub(crate) fn arguments(&self) -> &[DynamicMemberArgumentSourceV1] {
        &self.arguments
    }

    pub(crate) const fn result_site(&self) -> &SourceExprSiteV1 {
        &self.result_site
    }

    pub(crate) const fn dispatch(&self) -> &DynamicMemberDispatchKeyV1 {
        &self.dispatch
    }
}

impl<'catalog> VerifiedSourceCallTargetCatalogV1<'catalog> {
    /// Extends the production catalog from every cataloged callable in one
    /// complete semantic-source batch. Top-level functions are deliberately
    /// outside the same-module callable catalog and remain untouched.
    pub(crate) fn extend_complete_dynamic_sources(
        mut self,
        source: &VerifiedNormalCallableSemanticSourceV1<'_>,
    ) -> Result<Self, DynamicMemberSourceIssueV1> {
        for key in source.keys() {
            let loan = source.loan(key).map_err(|error| {
                DynamicMemberSourceIssueV1::Unresolved(
                    DynamicMemberSourceUnresolvedV1::DynamicOriginEvidenceUnavailable(error.into()),
                )
            })?;
            let link = match issue_catalog_callable_owner_link_v1(loan, self.declarations) {
                Ok(link) => link,
                Err(CatalogCallableOwnerLinkIssueV1::CatalogedCallableRequired) => continue,
                Err(error) => {
                    return Err(DynamicMemberSourceIssueV1::Rejected(
                        DynamicMemberSourceRejectV1::CatalogCallableOwnerLink(error),
                    ))
                }
            };
            self = self.extend_dynamic_members(link)?;
        }
        Ok(self)
    }

    /// Atomically extends the one route-neutral catalog with Dynamic rows.
    ///
    /// The caller identity comes only from `link`; selector, arity, receiver,
    /// argument order, and result site come only from the resolver-owned call
    /// rows. Any failure drops this unpublished catalog value.
    pub(crate) fn extend_dynamic_members(
        mut self,
        link: VerifiedCatalogCallableOwnerLinkV1<'_>,
    ) -> Result<Self, DynamicMemberSourceIssueV1> {
        let (caller, source) = link.into_parts();
        if self.declarations.declaration(&caller).is_none() {
            return Err(DynamicMemberSourceIssueV1::Rejected(
                DynamicMemberSourceRejectV1::ForeignCatalogCallable(caller),
            ));
        }
        let owner = source.owner();
        let dynamic = issue_source_backed_dynamic_callable_v1(source.input()).map_err(|error| {
            DynamicMemberSourceIssueV1::Unresolved(
                DynamicMemberSourceUnresolvedV1::DynamicOriginEvidenceUnavailable(error.into()),
            )
        })?;
        let ledger = source.ledger();

        let mut calls = ledger
            .method_calls()
            .map(|(_, call)| call)
            .collect::<Vec<_>>();
        calls.sort_by(|left, right| left.site().cmp(right.site()));

        for call in calls {
            if call.owner() != owner {
                return Err(owner_mismatch(owner, call.owner()));
            }
            if call.result_site() != call.site()
                || call.arguments().len() != call.arity() as usize
                || call
                    .arguments()
                    .iter()
                    .enumerate()
                    .any(|(ordinal, row)| row.ordinal() as usize != ordinal)
            {
                return Err(DynamicMemberSourceIssueV1::Rejected(
                    DynamicMemberSourceRejectV1::ArgumentResultRelationMismatch(
                        call.site().clone(),
                    ),
                ));
            }
            let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(
                receiver_binding,
            )) = call.receiver()
            else {
                // A completely observed non-local receiver belongs to an
                // existing static/declared-instance route, not this one.
                continue;
            };
            if receiver_binding.owner() != owner {
                return Err(DynamicMemberSourceIssueV1::Rejected(
                    DynamicMemberSourceRejectV1::ReceiverBindingOwnerMismatch(receiver_binding),
                ));
            }
            let Some(dynamic_origin) = dynamic.origin_for_binding(receiver_binding) else {
                // A typed or otherwise non-Dynamic local is a valid,
                // completely observed non-candidate for this admission.
                continue;
            };
            let row_key = (caller.clone(), call.site().clone());
            if self.rows.contains_key(&row_key) {
                return Err(DynamicMemberSourceIssueV1::Rejected(
                    DynamicMemberSourceRejectV1::DuplicateOrCollidingTarget {
                        caller,
                        site: call.site().clone(),
                    },
                ));
            }
            let arguments = call
                .arguments()
                .iter()
                .map(|row| DynamicMemberArgumentSourceV1 {
                    ordinal: row.ordinal(),
                    site: row.site().clone(),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice();
            self.rows.insert(
                row_key,
                VerifiedSourceCallTargetV1::DynamicMember(VerifiedSourceBoundDynamicMemberCallV1 {
                    owner,
                    call_site: call.site().clone(),
                    receiver_site: call.receiver_site().clone(),
                    receiver_binding,
                    dynamic_origin,
                    arguments,
                    result_site: call.result_site().clone(),
                    dispatch: DynamicMemberDispatchKeyV1 {
                        selector: call.selector().into(),
                        arity: call.arity(),
                    },
                }),
            );
        }
        Ok(self)
    }
}

fn owner_mismatch(
    expected: FunctionOwnerIdV1,
    actual: FunctionOwnerIdV1,
) -> DynamicMemberSourceIssueV1 {
    DynamicMemberSourceIssueV1::Rejected(DynamicMemberSourceRejectV1::CallerOwnerMismatch {
        expected,
        actual,
    })
}
