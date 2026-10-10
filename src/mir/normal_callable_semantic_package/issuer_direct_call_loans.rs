use super::super::direct_call_loan::{
    DirectCallDispositionLoanV1, DirectCallDispositionLoansV1, DirectCallDispositionRowV1,
};
use super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use super::DirectCallDispositionIssueV1;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1,
    VerifiedSourceBackedSameModuleCallableCatalogV1,
};
use crate::mir::callable_semantic_batch::{
    ResolvedCallableDeclarationModeV1, VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;

/// Move the exact direct-call products for each eligible owner into private
/// package loans.
///
/// The resolver has already co-issued the source observations and the
/// callable index.  This helper only joins those existing products by their
/// owner/site relation; it never resolves a name or emits a new target.
/// Eligible owners are App Main itself and the selected static children of
/// App Main's box, because only those owners lower through the raw
/// direct-call consumer.
pub(super) fn issue_direct_call_loans_v1(
    catalog: &VerifiedSourceBackedSameModuleCallableCatalogV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    app_main_identity: &crate::parser::CallableDeclarationIdentityV1,
) -> Result<Option<DirectCallDispositionLoansV1>, DirectCallDispositionIssueV1> {
    let Some((main_slot, callable_index)) = batch.main_callable_index() else {
        return Ok(None);
    };
    let mut loans = Vec::new();
    let mut main_matched = false;
    for declaration in batch.declarations() {
        let is_app_main = declaration.identity().same_as(app_main_identity);
        let batch_slot = declaration.batch_slot();
        let eligible = is_app_main
            || selected
                .role_for_batch_slot(batch_slot)
                .is_some_and(|role| role.is_main_static_child());
        if !eligible {
            continue;
        }
        if declaration.mode() != ResolvedCallableDeclarationModeV1::StaticBoxMethod {
            return Err(DirectCallDispositionIssueV1::SourceCoverage);
        }
        if is_app_main {
            if main_matched || batch_slot != main_slot {
                return Err(DirectCallDispositionIssueV1::SourceCoverage);
            }
            main_matched = true;
        }
        let owner = declaration.owner();
        let rows = collect_direct_call_rows_v1(
            catalog,
            batch,
            selected,
            callable_index,
            owner,
            batch_slot,
        )?;
        if rows.is_empty() {
            continue;
        }
        loans.push(
            DirectCallDispositionLoanV1::from_rows(owner, rows)
                .map_err(DirectCallDispositionIssueV1::Loan)?,
        );
    }
    if !main_matched {
        return Err(DirectCallDispositionIssueV1::SourceCoverage);
    }
    if loans.is_empty() {
        return Ok(None);
    }
    DirectCallDispositionLoansV1::issue(loans)
        .map(Some)
        .map_err(DirectCallDispositionIssueV1::Loan)
}

/// Assemble the loan rows for one owner from its own sealed observations.
/// The owner's lowering forest must be single-rooted at that owner, and no
/// nested owner may carry observations of its own.
fn collect_direct_call_rows_v1(
    catalog: &VerifiedSourceBackedSameModuleCallableCatalogV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    callable_index: &crate::mir::resolved_semantics::VerifiedCallableIndexV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    batch_slot: u32,
) -> Result<Vec<(SourceExprSiteV1, DirectCallDispositionRowV1)>, DirectCallDispositionIssueV1> {
    let mut rows = Vec::new();
    batch
        .with_lowering_input(batch_slot, |input| {
            let [root] = input.forest().roots() else {
                return Err(DirectCallDispositionIssueV1::SourceCoverage);
            };
            if *root != owner {
                return Err(DirectCallDispositionIssueV1::SourceCoverage);
            }
            for (candidate, function) in input.forest().owners() {
                if candidate != owner {
                    if function.direct_call_observations().next().is_some() {
                        return Err(
                            DirectCallDispositionIssueV1::NestedOwnerObservation,
                        );
                    }
                    continue;
                }
                for (site, observation) in function.direct_call_observations() {
                    let target = function
                        .direct_call_target(site)
                        .ok_or(DirectCallDispositionIssueV1::TargetMissing)?;
                    let header = callable_index
                        .header_for_callable(target.callable())
                        .map_err(DirectCallDispositionIssueV1::HeaderLookup)?;
                    if header.callable().owner() != target.callable().owner() {
                        return Err(DirectCallDispositionIssueV1::TargetOwnerMismatch);
                    }
                    let published_key = {
                        let mut matches = batch
                            .declarations()
                            .filter(|declaration| declaration.owner() == target.callable().owner());
                        let declaration = matches
                            .next()
                            .ok_or(DirectCallDispositionIssueV1::PublishedTargetMissing)?;
                        if matches.next().is_some() {
                            return Err(
                                DirectCallDispositionIssueV1::PublishedTargetDuplicate,
                            );
                        }
                        let selected_key = selected
                            .key_for_batch_slot(declaration.batch_slot())
                            .ok_or(
                                DirectCallDispositionIssueV1::PublishedTargetMissing,
                            )?;
                        let (key, expected_namespace) = match selected_key {
                            SelectedNormalCallableKeyV1::Cataloged(key) => {
                                (key.clone(), SameModuleCallableNamespaceV1::StaticBoxMethod)
                            }
                            SelectedNormalCallableKeyV1::TopLevel(top_level) => {
                                let arity = u32::try_from(top_level.declared_arity()).map_err(
                                    |_| {
                                        DirectCallDispositionIssueV1::PublishedTargetArityMismatch
                                    },
                                )?;
                                (
                                    CanonicalSameModuleCallableKeyV1::free_function(
                                        top_level.declared_name(),
                                        arity,
                                    ),
                                    SameModuleCallableNamespaceV1::FreeFunction,
                                )
                            }
                        };
                        if key.namespace() != expected_namespace {
                            return Err(
                                DirectCallDispositionIssueV1::PublishedTargetNamespaceMismatch,
                            );
                        }
                        if key.name() != header.source_key().name() {
                            return Err(
                                DirectCallDispositionIssueV1::PublishedTargetNameMismatch,
                            );
                        }
                        if key.arity() != header.source_key().arity() {
                            return Err(
                                DirectCallDispositionIssueV1::PublishedTargetArityMismatch,
                            );
                        }
                        if catalog.catalog().declaration(&key).is_none() {
                            return Err(
                                DirectCallDispositionIssueV1::PublishedTargetMissing,
                            );
                        }
                        key.clone()
                    };
                    if header.callable().owner().compilation_brand()
                        != owner.compilation_brand()
                    {
                        return Err(
                            DirectCallDispositionIssueV1::CompilationBrandMismatch,
                        );
                    }
                    if header.source_key().name() != observation.name() {
                        return Err(DirectCallDispositionIssueV1::TargetNameMismatch);
                    }
                    if header.signature().arity() != observation.arity() as usize {
                        return Err(DirectCallDispositionIssueV1::ArityMismatch);
                    }
                    if header.signature().arity() != observation.argument_sites().len() {
                        return Err(DirectCallDispositionIssueV1::ArgumentSiteMismatch);
                    }
                    let emission = crate::mir::canonical_direct_call::VerifiedCanonicalDirectCallEmissionV1::from_header_with_published_key(
                        header,
                        published_key,
                    );
                    rows.push((
                        site.clone(),
                        DirectCallDispositionRowV1::new(
                            observation.argument_sites().to_vec().into_boxed_slice(),
                            emission,
                        ),
                    ));
                }
            }
            Ok(())
        })
        .map_err(DirectCallDispositionIssueV1::BatchLoan)??;
    Ok(rows)
}
