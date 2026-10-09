//! Source-only Static call evidence for a resolver-owned callable Loop.
//!
//! The package's existing claim issuer and original incoming Rc are joined
//! here. No actual value, entry carrier, Recipe key, or physical call is issued.

use std::rc::Rc;

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::home_new_prefix::StaticI64CallClaimV1;
use crate::mir::resolved_semantics::{
    OwnedExprSiteV1, ResolvedLoopPlacementV1, ResolvedMethodCallReceiverSourceV1, SourceStmtSiteV1,
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
    if site.owner() != input.owner() || !claims.catalog_brand().is_same(selected.catalog_brand()) {
        return Err(reject());
    }
    let placement = input
        .function()
        .resolved_loop_placement(loop_site, site.site())
        .map_err(|_| reject())?
        .ok_or_else(reject)?;
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
    Ok(Some(LoopStaticSourceCallLoanV1 {
        loop_site: loop_site.clone(),
        placement,
        original: Rc::clone(original),
        claim,
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
        *self.loop_static_source_loans.get_mut() = rows;
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
}
