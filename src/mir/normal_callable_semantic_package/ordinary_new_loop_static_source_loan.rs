//! Source-only Static call evidence for a resolver-owned callable Loop.
//!
//! The package's existing claim issuer and original incoming Rc are joined
//! here. No actual value, entry carrier, Recipe key, or physical call is issued.

use std::rc::Rc;

use crate::ast::ASTNode;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::home_new_prefix::StaticI64CallClaimV1;
use crate::mir::resolved_semantics::{
    OwnedExprSiteV1, ResolvedLoopPlacementV1, ResolvedMethodCallReceiverSourceV1,
    SourceBindingSiteV1, SourceExprSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

use super::super::model::OwnedCallableParameterContractDeclarationV1;
use super::super::qualified_static_call_claim::{
    claim_for_source_site_v1, incoming_source::StaticIncomingSourceV1,
    QualifiedStaticCallClaimIndexV1,
};
use super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use super::lexical_instance_call::PreparedBorrowedFormalIngressV1;
use super::{OrdinaryNewClaimLedgerV1, OrdinaryNewCoSealIssueV1};

/// Same original call Rc plus the package's source-only I64 claim.
/// The absence of `Clone` prevents an independent call authority being made.
#[derive(Debug)]
pub(in crate::mir) struct LoopStaticSourceCallLoanV1 {
    loop_site: SourceStmtSiteV1,
    placement: ResolvedLoopPlacementV1,
    original: Rc<StaticIncomingSourceV1>,
    claim: StaticI64CallClaimV1,
}

/// A pre-loop local's I64 initializer, issued by the same Static claim owner.
/// This is a source class, never a live ValueId or executable call permission.
#[derive(Debug)]
pub(in crate::mir) struct LoopEntryStaticI64SourceLoanV1 {
    loop_site: SourceStmtSiteV1,
    declaration: SourceBindingSiteV1,
    original: Rc<StaticIncomingSourceV1>,
    claim: StaticI64CallClaimV1,
}

/// Original post-Loop return call. Its I64 result is source-only; no live
/// return value, Home relation or executable call is issued here.
#[derive(Debug)]
pub(in crate::mir) struct LoopTailStaticI64SourceLoanV1 {
    loop_site: SourceStmtSiteV1,
    return_site: SourceStmtSiteV1,
    original: Rc<StaticIncomingSourceV1>,
    claim: StaticI64CallClaimV1,
}

impl LoopTailStaticI64SourceLoanV1 {
    pub(in crate::mir) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }
    pub(in crate::mir) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }
    pub(in crate::mir) fn original(&self) -> &Rc<StaticIncomingSourceV1> {
        &self.original
    }
    pub(in crate::mir) fn call_site(&self) -> &OwnedExprSiteV1 {
        self.original.call_site()
    }
    pub(in crate::mir) fn claim(&self) -> &StaticI64CallClaimV1 {
        &self.claim
    }
}

impl LoopEntryStaticI64SourceLoanV1 {
    pub(in crate::mir) fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
    }
    pub(in crate::mir) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }
    pub(in crate::mir) fn original(&self) -> &Rc<StaticIncomingSourceV1> {
        &self.original
    }
    pub(in crate::mir) fn claim(&self) -> &StaticI64CallClaimV1 {
        &self.claim
    }
    pub(in crate::mir) fn call_site(&self) -> &OwnedExprSiteV1 {
        self.original.call_site()
    }
}

pub(super) struct StaticSourceSealV1 {
    original: Rc<StaticIncomingSourceV1>,
    claim: StaticI64CallClaimV1,
}

impl StaticSourceSealV1 {
    pub(super) fn original(&self) -> &Rc<StaticIncomingSourceV1> {
        &self.original
    }
}

impl LoopStaticSourceCallLoanV1 {
    pub(in crate::mir) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(in crate::mir) fn placement(&self) -> &ResolvedLoopPlacementV1 {
        &self.placement
    }

    pub(in crate::mir) fn original(&self) -> &Rc<StaticIncomingSourceV1> {
        &self.original
    }

    pub(in crate::mir) fn claim(&self) -> &StaticI64CallClaimV1 {
        &self.claim
    }
    pub(in crate::mir) fn call_site(&self) -> &OwnedExprSiteV1 {
        self.original.call_site()
    }
    pub(in crate::mir) fn argument_sites(
        &self,
    ) -> &[crate::mir::resolved_semantics::SourceExprSiteV1] {
        self.original.argument_sites()
    }
}

/// Co-seal one exact original CurrentOwner call within a selected Loop.
/// `None` means the site has no CurrentOwner I64 claim; a claimed site with
/// missing or divergent original source fails rather than gaining a new row.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn issue_loop_static_source_call_loan_v1(
    claims: &QualifiedStaticCallClaimIndexV1,
    incoming: &PreparedBorrowedFormalIngressV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    caller: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_site: &SourceStmtSiteV1,
    site: &OwnedExprSiteV1,
) -> Result<Option<LoopStaticSourceCallLoanV1>, String> {
    let reject = || "[freeze:contract][borrowed-static/loop-source-identity]".to_owned();
    let placement = input
        .function()
        .resolved_loop_placement(loop_site, site.site())
        .map_err(|_| reject())?
        .ok_or_else(reject)?;
    let Some(seal) =
        issue_static_source_seal_v1(claims, incoming, selected, contracts, caller, input, site)?
    else {
        return Ok(None);
    };
    Ok(Some(LoopStaticSourceCallLoanV1 {
        loop_site: loop_site.clone(),
        placement,
        original: seal.original,
        claim: seal.claim,
    }))
}

pub(super) fn issue_static_source_seal_v1(
    claims: &QualifiedStaticCallClaimIndexV1,
    incoming: &PreparedBorrowedFormalIngressV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    caller: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
) -> Result<Option<StaticSourceSealV1>, String> {
    let reject = || "[freeze:contract][borrowed-static/source-identity]".to_owned();
    if site.owner() != input.owner() || !claims.catalog_brand().is_same(selected.catalog_brand()) {
        return Err(reject());
    }
    let mut calls = input
        .function()
        .method_calls()
        .filter(|(observed, _)| *observed == site.site());
    let (_, call) = calls.next().ok_or_else(reject)?;
    if calls.next().is_some() {
        return Err(reject());
    }
    if call.receiver() != ResolvedMethodCallReceiverSourceV1::CurrentOwner {
        return Ok(None);
    }
    let Some(claim) = claim_for_source_site_v1(
        claims,
        Some(caller),
        &input,
        site,
        selected,
        contracts,
        None,
    )
    .map_err(|_| reject())?
    else {
        return Ok(None);
    };
    let original = incoming
        .static_observation_for_source_v1(site)
        .ok_or_else(reject)?
        .as_ref()
        .map_err(|_| reject())?;
    let indexed = claims
        .current_owner_source(caller, site.site())
        .ok_or_else(reject)?;
    if !original.catalog_brand().is_same(claims.catalog_brand())
        || original.caller() != caller
        || original.call_site() != site
        || original.current_owner_source() != Some(indexed)
        || original.target() != indexed.route().target()
        || original.argument_sites().len() != call.arguments().len()
        || original
            .argument_sites()
            .iter()
            .zip(call.arguments())
            .any(|(left, right)| left != right.site())
        || original.required_i64_arguments()
            != claim
                .current_owner_source_required_i64_arguments()
                .unwrap_or(&[])
        || !claim.corroborates_source(site, call.receiver(), call.arity())
    {
        return Err(reject());
    }
    Ok(Some(StaticSourceSealV1 {
        original: Rc::clone(original),
        claim,
    }))
}

/// Exact source initializer before this Loop, with an I64 Static result.
/// The checked input condition belongs to the original target; this loan
/// does not assert that a tagged argument is already Integer.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn issue_loop_entry_static_i64_source_loan_v1(
    claims: &QualifiedStaticCallClaimIndexV1,
    incoming: &PreparedBorrowedFormalIngressV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    caller: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_site: &SourceStmtSiteV1,
    declaration: &SourceBindingSiteV1,
) -> Result<Option<LoopEntryStaticI64SourceLoanV1>, String> {
    let reject = || "[freeze:contract][borrowed-static/loop-entry-source-identity]".to_owned();
    input
        .function()
        .loop_region_bundle(loop_site)
        .map_err(|_| reject())?;
    let SourceBindingSiteV1::Local { statement, .. } = declaration else {
        return Ok(None);
    };
    let (
        Some((SourcePathSegmentV1::Body(loop_index), loop_parent)),
        Some((SourcePathSegmentV1::Body(local_index), local_parent)),
    ) = (
        loop_site.node().segments().split_last(),
        statement.node().segments().split_last(),
    )
    else {
        return Ok(None);
    };
    if loop_parent != local_parent || local_index >= loop_index {
        return Ok(None);
    }
    let Some(relation) = input
        .function()
        .expression_source()
        .initializer(declaration)
    else {
        return Err(reject());
    };
    let Some(site) = relation.initializer_site() else {
        return Ok(None);
    };
    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
    let Some(seal) =
        issue_static_source_seal_v1(claims, incoming, selected, contracts, caller, input, &owned)?
    else {
        return Ok(None);
    };
    Ok(Some(LoopEntryStaticI64SourceLoanV1 {
        loop_site: loop_site.clone(),
        declaration: declaration.clone(),
        original: seal.original,
        claim: seal.claim,
    }))
}

/// Seal the exact final `return me.m()` after a top-level Loop. The name of
/// `m` is never a selector: the existing target/claim issuer owns that join.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package) fn issue_loop_tail_static_i64_source_loan_v1(
    claims: &QualifiedStaticCallClaimIndexV1,
    incoming: &PreparedBorrowedFormalIngressV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    caller: &CanonicalSameModuleCallableKeyV1,
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_site: &SourceStmtSiteV1,
) -> Result<Option<LoopTailStaticI64SourceLoanV1>, String> {
    let reject = || "[freeze:contract][borrowed-static/loop-tail-source-identity]".to_owned();
    let Some((SourcePathSegmentV1::Body(loop_index), parent)) =
        loop_site.node().segments().split_last()
    else {
        return Ok(None);
    };
    if !parent.is_empty() {
        return Ok(None);
    }
    let body = input.source().root_body().map_err(|_| reject())?;
    let tail_index = (*loop_index as usize).checked_add(1).ok_or_else(reject)?;
    if tail_index.checked_add(1) != Some(body.statements().len()) {
        return Ok(None);
    }
    let tail = input
        .source()
        .body_stmt(&body, tail_index)
        .map_err(|_| reject())?;
    let ASTNode::Return {
        value: Some(value), ..
    } = tail.node()
    else {
        return Ok(None);
    };
    let ASTNode::MethodCall {
        object, arguments, ..
    } = value.as_ref()
    else {
        return Ok(None);
    };
    if !matches!(object.as_ref(), ASTNode::Me { .. }) || !arguments.is_empty() {
        return Ok(None);
    }
    let mut path = tail.site().node().segments().to_vec();
    path.push(SourcePathSegmentV1::Value);
    let site = OwnedExprSiteV1::new(
        input.owner(),
        SourceExprSiteV1::from_node(
            crate::mir::resolved_semantics::SourceNodeSiteV1::from_segments(path),
        ),
    );
    let Some(seal) =
        issue_static_source_seal_v1(claims, incoming, selected, contracts, caller, input, &site)?
    else {
        return Ok(None);
    };
    if !seal.original.argument_sites().is_empty()
        || !seal.claim.corroborates_source(
            &site,
            ResolvedMethodCallReceiverSourceV1::CurrentOwner,
            0,
        )
    {
        return Err(reject());
    }
    Ok(Some(LoopTailStaticI64SourceLoanV1 {
        loop_site: loop_site.clone(),
        return_site: tail.site().clone(),
        original: seal.original,
        claim: seal.claim,
    }))
}

impl OrdinaryNewClaimLedgerV1 {
    /// Retain only source evidence in the same package issuance that still
    /// holds the claim index. Selected physical callers take rows later.
    pub(in crate::mir::normal_callable_semantic_package) fn retain_loop_static_source_loans_v1(
        &mut self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        claims: &QualifiedStaticCallClaimIndexV1,
    ) -> Result<(), OrdinaryNewCoSealIssueV1> {
        let Some(Ok(incoming)) = self.borrowed_formal_source.as_ref() else {
            return Ok(());
        };
        let mut rows = std::collections::BTreeMap::new();
        let mut entry_rows = std::collections::BTreeMap::new();
        let mut tail_rows = std::collections::BTreeMap::new();
        let mut home_rows = std::collections::BTreeMap::new();
        let mut tagged_rows = std::collections::BTreeMap::new();
        for declaration in batch.declarations() {
            let slot = declaration.batch_slot();
            let Some(SelectedNormalCallableKeyV1::Cataloged(caller)) =
                selected.key_for_batch_slot(slot)
            else {
                continue;
            };
            batch
                .with_lowering_input(slot, |input| {
                    for loop_site in input.function().loop_sites() {
                        let tail = issue_loop_tail_static_i64_source_loan_v1(
                            claims, incoming, selected, contracts, &caller, input, loop_site,
                        );
                        let selected_tail = matches!(&tail, Ok(Some(_)));
                        if selected_tail {
                            home_rows.insert(loop_site.clone(),
                                super::static_home_effect::issue_closed_static_loop_home_neutral_v1(
                                    batch, selected, contracts, claims, incoming, &caller, loop_site,
                                ));
                        }
                        if !matches!(&tail, Ok(None)) && tail_rows.insert(
                            loop_site.clone(), tail.and_then(|row| row.ok_or_else(||
                                "[freeze:contract][borrowed-static/loop-tail-claim-missing]".to_owned()
                            )),
                        ).is_some() {
                            return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
                        }
                        for relation in input.function().expression_source().initializers() {
                            let Some(initializer) = relation.initializer_site() else { continue; };
                            if !input.function().method_calls().any(|(site, call)|
                                site == initializer &&
                                call.receiver() == ResolvedMethodCallReceiverSourceV1::CurrentOwner
                            ) { continue; }
                            let loan = issue_loop_entry_static_i64_source_loan_v1(
                                claims, incoming, selected, contracts, &caller,
                                input, loop_site, relation.declaration_site(),
                            );
                            if selected_tail {
                                if let Ok(Some(ref entry)) = loan {
                                    let key = (loop_site.clone(), relation.declaration_site().clone());
                                    if tagged_rows.insert(key,
                                        super::static_loop_tagged_entry::issue_static_loop_tagged_entry_source_v1(
                                            input, &caller, contracts, incoming, entry,
                                        ),
                                    ).is_some() {
                                        return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
                                    }
                                }
                            }
                            if !matches!(&loan, Ok(None)) && entry_rows.insert(
                                (loop_site.clone(), relation.declaration_site().clone()),
                                loan.and_then(|row| row.ok_or_else(||
                                    "[freeze:contract][borrowed-static/loop-entry-claim-missing]".to_owned()
                                )),
                            ).is_some() {
                                return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
                            }
                        }
                        for (site, call) in input.function().method_calls() {
                            if call.receiver() != ResolvedMethodCallReceiverSourceV1::CurrentOwner {
                                continue;
                            }
                            let inside = input
                                .function()
                                .resolved_loop_placement(loop_site, site)
                                .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
                            if inside.is_none() {
                                continue;
                            }
                            let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
                            let loan = issue_loop_static_source_call_loan_v1(
                                claims, incoming, selected, contracts, &caller,
                                input, loop_site, &owned,
                            );
                            if !matches!(&loan, Ok(None))
                                && rows.insert((loop_site.clone(), owned), loan.and_then(|row| {
                                    row.ok_or_else(|| "[freeze:contract][borrowed-static/loop-claim-missing]".to_owned())
                                })).is_some()
                            {
                                return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
                            }
                        }
                    }
                    Ok(())
                })
                .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;
        }
        self.loop_static_source_loop_sites = rows
            .keys()
            .map(|(loop_site, call)| (call.owner(), loop_site.clone()))
            .collect();
        *self.loop_static_source_loans.get_mut() = rows;
        *self.loop_entry_static_i64_source_loans.get_mut() = entry_rows;
        *self.loop_tail_static_i64_source_loans.get_mut() = tail_rows;
        *self.loop_static_home_neutral.get_mut() = home_rows;
        *self.loop_static_tagged_entry.get_mut() = tagged_rows;
        Ok(())
    }

    /// An affine source-only row. No executable actuals or packet are inside.
    pub(in crate::mir) fn take_loop_static_source_call_loan_v1(
        &self,
        loop_site: &SourceStmtSiteV1,
        site: &OwnedExprSiteV1,
    ) -> Option<Result<LoopStaticSourceCallLoanV1, String>> {
        self.loop_static_source_loans
            .borrow_mut()
            .remove(&(loop_site.clone(), site.clone()))
    }

    /// Stable issued-site census, retained after an affine loan is taken.
    /// Re-entering a selected Loop cannot fall through to the V1 route.
    pub(in crate::mir) fn expects_loop_static_source_loan_v1(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        loop_site: &SourceStmtSiteV1,
    ) -> bool {
        self.loop_static_source_loop_sites
            .contains(&(owner, loop_site.clone()))
    }

    /// One source-only I64 result initializer before the selected Loop.
    pub(in crate::mir) fn take_loop_entry_static_i64_source_loan_v1(
        &self,
        loop_site: &SourceStmtSiteV1,
        declaration: &SourceBindingSiteV1,
    ) -> Option<Result<LoopEntryStaticI64SourceLoanV1, String>> {
        self.loop_entry_static_i64_source_loans
            .borrow_mut()
            .remove(&(loop_site.clone(), declaration.clone()))
    }

    pub(in crate::mir) fn take_loop_tail_static_i64_source_loan_v1(
        &self,
        loop_site: &SourceStmtSiteV1,
    ) -> Option<Result<LoopTailStaticI64SourceLoanV1, String>> {
        self.loop_tail_static_i64_source_loans
            .borrow_mut()
            .remove(loop_site)
    }

    pub(in crate::mir) fn take_loop_static_home_neutral_v1(
        &self,
        loop_site: &SourceStmtSiteV1,
    ) -> Option<Result<super::static_home_effect::VerifiedClosedStaticLoopHomeNeutralV1, String>> {
        self.loop_static_home_neutral.borrow_mut().remove(loop_site)
    }

    pub(in crate::mir) fn take_loop_static_tagged_entry_v1(
        &self,
        loop_site: &SourceStmtSiteV1,
        declaration: &SourceBindingSiteV1,
    ) -> Option<Result<super::static_loop_tagged_entry::VerifiedStaticLoopTaggedEntrySourceV1, String>> {
        self.loop_static_tagged_entry
            .borrow_mut()
            .remove(&(loop_site.clone(), declaration.clone()))
    }
}
