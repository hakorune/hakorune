//! Package-side membership index for qualified static-box call claims.
//!
//! `local x = Alias.m(..)` sites join the homes-aware I64 lane only through
//! this index.  The authority chain is the existing one: the sealed import
//! alias view resolves receiver names, the whole-source static target
//! inventory proves a `QualifiedStatic` route to a `StaticBoxMethod`
//! declaration, and the result solver proves the target's `ExactI64`
//! disposition.  This module issues no new authority — it composes the
//! already-sealed products into one membership row per call site and stays
//! silent (absent row) for every non-claim.
//!
//! Qualified rows remain distinct from original `CurrentOwnerStatic` rows.
//! The homes predicate composes bounded CurrentOwner ExactI64 source claims
//! from the original typed incoming loan. Arity-bearing sites still require
//! the same original SourceStatic actuals; no claim issues executable entry.
//! Non-I64 results retain no homes claim.

use std::collections::BTreeMap;

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SelectedNormalCallableKeyV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::callable_result_representation::{
    CallableResultCatalogErrorV1, VerifiedCallableResultDispositionV1,
    VerifiedSameModuleCallableResultCatalogV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    QualifiedStaticCallClaimV1, StaticI64CallClaimV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::{
    StaticImportAliasViewErrorV1, VerifiedSourceStaticCallTargetV1,
    VerifiedStaticImportAliasViewV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
    WholeSourceStaticCallTargetInventoryErrorV1,
};

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) enum QualifiedStaticCallClaimIndexIssueV1 {
    ImportAlias(StaticImportAliasViewErrorV1),
    TargetInventory(WholeSourceStaticCallTargetInventoryErrorV1),
    ResultCatalog(CallableResultCatalogErrorV1),
}

/// `(caller, site) -> claim` membership for qualified static-box calls.
///
/// Each row pairs the claim with the sealed `StaticBoxMethod` target the
/// target inventory proved for that exact site — the claim and its target
/// come from one seal, never two lookups. Construction plans consume the
/// pair as their AST-free provider-argument fact.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct QualifiedStaticCallClaimIndexV1 {
    catalog_brand: crate::mir::builder::SameModuleCallableCatalogBrandV1,
    // These are the SAME solver-issued function rows used for call-site
    // claims. Selected unannotated Static entries may borrow one exact row.
    result_rows: BTreeMap<CanonicalSameModuleCallableKeyV1, VerifiedCallableResultDispositionV1>,
    // The original current-owner route stays disjoint from qualified claims.
    // Retaining its result disposition grants no incoming or executable ABI.
    current_owner_rows: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        CurrentOwnerStaticCallSourceV1,
    >,
    rows: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        (QualifiedStaticCallClaimV1, CanonicalSameModuleCallableKeyV1),
    >,
}

/// Original whole-inventory route and result, retained by the same index issuer.
/// No receiver/argument classification or physical permission is reconstructed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) struct CurrentOwnerStaticCallSourceV1 {
    route: crate::mir::source_call_target::VerifiedCurrentOwnerStaticCallTargetV1,
    result: VerifiedCallableResultDispositionV1,
}

impl CurrentOwnerStaticCallSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn route(
        &self,
    ) -> &crate::mir::source_call_target::VerifiedCurrentOwnerStaticCallTargetV1 {
        &self.route
    }

    pub(in crate::mir::normal_callable_semantic_package) fn result(
        &self,
    ) -> &VerifiedCallableResultDispositionV1 {
        &self.result
    }
}

#[path = "qualified_static_incoming_source.rs"]
pub(super) mod incoming_source;

impl QualifiedStaticCallClaimIndexV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn catalog_brand(
        &self,
    ) -> &crate::mir::builder::SameModuleCallableCatalogBrandV1 {
        &self.catalog_brand
    }

    pub(in crate::mir::normal_callable_semantic_package) fn issue(
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        import_rows: impl IntoIterator<Item = (String, String)>,
    ) -> Result<Self, QualifiedStaticCallClaimIndexIssueV1> {
        let imports = VerifiedStaticImportAliasViewV1::seal(declarations, import_rows)
            .map_err(QualifiedStaticCallClaimIndexIssueV1::ImportAlias)?;
        let inventory =
            VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, &imports)
                .map_err(QualifiedStaticCallClaimIndexIssueV1::TargetInventory)?;
        let targets = inventory.into_targets();
        let results = VerifiedSameModuleCallableResultCatalogV1::verify(declarations, &targets)
            .map_err(QualifiedStaticCallClaimIndexIssueV1::ResultCatalog)?;
        let mut rows = BTreeMap::new();
        let mut current_owner_rows = BTreeMap::new();
        for ((caller, site), source_target) in targets.rows() {
            if let VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic(route) = source_target {
                let result = results.disposition(route.target()).ok_or_else(|| {
                    QualifiedStaticCallClaimIndexIssueV1::ResultCatalog(
                        CallableResultCatalogErrorV1::StableResultDrift {
                            key: route.target().clone(),
                        },
                    )
                })?;
                current_owner_rows.insert(
                    (caller.clone(), site.clone()),
                    CurrentOwnerStaticCallSourceV1 {
                        route: route.clone(),
                        result: result.clone(),
                    },
                );
                continue;
            }
            // Only qualified receivers (`Alias.m(..)`) join the claim lane;
            // `me.m(..)` inside a static box keeps its own route family.
            if !matches!(
                source_target,
                VerifiedSourceStaticCallTargetV1::QualifiedStatic(_)
            ) {
                continue;
            }
            let Some(VerifiedCallableResultDispositionV1::ExactI64 {
                required_i64_arguments,
            }) = results.disposition(source_target.target())
            else {
                continue;
            };
            rows.insert(
                (caller.clone(), site.clone()),
                (
                    QualifiedStaticCallClaimV1::new(required_i64_arguments.clone()),
                    source_target.target().clone(),
                ),
            );
        }
        let result_rows = results.into_dispositions();
        Ok(Self {
            catalog_brand: declarations.brand().clone(),
            result_rows,
            rows,
            current_owner_rows,
        })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn result_for_key(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&VerifiedCallableResultDispositionV1> {
        self.result_rows.get(key)
    }

    fn current_owner_i64_source(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<&CurrentOwnerStaticCallSourceV1> {
        self.current_owner_source(caller, site).filter(|row| {
            matches!(
                row.result(),
                VerifiedCallableResultDispositionV1::ExactI64 { .. }
            )
        })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn current_owner_source(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<&CurrentOwnerStaticCallSourceV1> {
        self.current_owner_rows.get(&(caller.clone(), site.clone()))
    }

    #[cfg(test)]
    pub(super) fn current_owner_sources_for_test(
        &self,
    ) -> impl Iterator<
        Item = (
            &CanonicalSameModuleCallableKeyV1,
            &SourceExprSiteV1,
            &CurrentOwnerStaticCallSourceV1,
        ),
    > {
        self.current_owner_rows
            .iter()
            .map(|((caller, site), row)| (caller, site, row))
    }

    /// Bounded raw-census membership; this does not seed a borrowed definition.
    pub(super) fn contains_zeroarg_i64_input_target(
        &self,
        target: &CanonicalSameModuleCallableKeyV1,
    ) -> bool {
        target.arity() == 0
            && (self.rows.values().any(|(claim, original)| {
                original == target && claim.required_i64_arguments().is_empty()
            }) || self.current_owner_rows.values().any(|row| {
                row.route().target() == target
                    && matches!(row.result(),
                        VerifiedCallableResultDispositionV1::ExactI64 { required_i64_arguments }
                            if required_i64_arguments.is_empty())
            }))
    }

    /// Original sealed ExactI64 target membership only; no transport permission.
    pub(super) fn contains_exact_i64_target(
        &self,
        target: &CanonicalSameModuleCallableKeyV1,
    ) -> bool {
        self.rows.values().any(|(_, original)| original == target)
    }

    /// Membership lookup for the homes-aware predicate. `Some` means the
    /// site's qualified receiver sealed to a `StaticBoxMethod` whose result
    /// disposition is `ExactI64`; the carried ordinals are the callee's
    /// required-i64 parameter positions for the argument seal.
    pub(in crate::mir::normal_callable_semantic_package) fn claim(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<QualifiedStaticCallClaimV1> {
        self.rows
            .get(&(caller.clone(), site.clone()))
            .map(|(claim, _)| claim)
            .cloned()
    }

    /// Membership lookup carrying the sealed target with the claim — the
    /// construction issuer's provider-argument seal needs both from one
    /// row; absent row means the site is no qualified static claim.
    pub(in crate::mir::normal_callable_semantic_package) fn claim_target(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<&(QualifiedStaticCallClaimV1, CanonicalSameModuleCallableKeyV1)> {
        self.rows.get(&(caller.clone(), site.clone()))
    }
}

/// Resolve one caller's claim key: selected rows translate through
/// `caller_key_for_batch_slot`; App Main is never a selected row, so its
/// key falls back to the catalog's `source_backed_app_main` co-seal key
/// supplied by the issuer.
pub(in crate::mir::normal_callable_semantic_package) fn caller_key_for_function(
    selected: &super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    batch_slot: u32,
    is_app_main: bool,
    app_main_claim_key: Option<&CanonicalSameModuleCallableKeyV1>,
) -> Option<CanonicalSameModuleCallableKeyV1> {
    caller_key_for_batch_slot(selected, batch_slot)
        .or_else(|| is_app_main.then(|| app_main_claim_key.cloned()).flatten())
}

/// The homes-aware membership predicate for one caller — the probe and the
/// verified walk share this closure so readiness never diverges.
pub(in crate::mir::normal_callable_semantic_package) fn local_static_call_predicate<'index>(
    claims: &'index QualifiedStaticCallClaimIndexV1,
    caller_key: Option<CanonicalSameModuleCallableKeyV1>,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'index>,
    selected: &'index super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    contracts: &'index [super::model::OwnedCallableParameterContractDeclarationV1],
    app_main: Option<&'index super::ordinary_new_coseal::BorrowedAppMainSourceLoanV1<'_>>,
) -> (
    impl FnMut(
            &crate::mir::resolved_semantics::OwnedExprSiteV1,
        ) -> Result<
            Option<StaticI64CallClaimV1>,
            super::ordinary_new_coseal::OrdinaryNewCoSealIssueV1,
        > + 'index,
    bool,
) {
    let has_current_owner_i64_source = caller_key.as_ref().is_some_and(|key| {
        claims.current_owner_rows.keys().any(|(caller, site)| {
            caller == key && claims.current_owner_i64_source(caller, site).is_some()
        })
    });
    (
        move |site| {
            claim_for_source_site_v1(
                claims,
                caller_key.as_ref(),
                &input,
                site,
                selected,
                contracts,
                app_main,
            )
        },
        has_current_owner_i64_source,
    )
}

/// Issue the Home walk's exact site claim through a reusable source port.
/// A source-bound loop producer can use this same issuer; ordered actuals and
/// executable entry still need separate proof from the original Static Rc.
pub(in crate::mir::normal_callable_semantic_package) fn claim_for_source_site_v1(
    claims: &QualifiedStaticCallClaimIndexV1,
    caller_key: Option<&CanonicalSameModuleCallableKeyV1>,
    input: &crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    selected: &super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    contracts: &[super::model::OwnedCallableParameterContractDeclarationV1],
    app_main: Option<&super::ordinary_new_coseal::BorrowedAppMainSourceLoanV1<'_>>,
) -> Result<Option<StaticI64CallClaimV1>, super::ordinary_new_coseal::OrdinaryNewCoSealIssueV1> {
    let Some(key) = caller_key else {
        return Ok(None);
    };
    if let Some(claim) = claims.claim(key, site.site()) {
        return Ok(Some(StaticI64CallClaimV1::qualified(claim)));
    }
    let Some(source_row) = claims.current_owner_i64_source(key, site.site()) else {
        return Ok(None);
    };
    let reject = || super::ordinary_new_coseal::OrdinaryNewCoSealIssueV1::SourceNavigation {
        site: site.clone(),
    };
    if site.owner() != input.owner() {
        return Err(reject());
    }
    let mut calls = input
        .function()
        .method_calls()
        .filter(|(observed, _)| *observed == site.site());
    let Some((_, call)) = calls.next() else {
        return Err(reject());
    };
    if calls.next().is_some() {
        return Err(reject());
    }
    let loan = claims
        .incoming_source(key, site, call, selected, contracts, app_main)
        .map_err(|issue| {
            super::ordinary_new_coseal::OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue,
            }
        })?
        .ok_or_else(reject)?;
    if loan.contract().parameters.len() != call.arguments().len() {
        return Err(reject());
    }
    let VerifiedCallableResultDispositionV1::ExactI64 {
        required_i64_arguments,
    } = source_row.result()
    else {
        return Err(reject());
    };
    if call.arguments().is_empty() {
        if !required_i64_arguments.is_empty() {
            return Err(reject());
        }
        return Ok(Some(StaticI64CallClaimV1::current_owner_zeroarg(
            site.clone(),
        )));
    }
    if required_i64_arguments
        .iter()
        .any(|ordinal| *ordinal as usize >= call.arguments().len())
    {
        return Err(reject());
    }
    Ok(Some(StaticI64CallClaimV1::current_owner_source(
        site.clone(),
        call.arity(),
        required_i64_arguments.clone(),
    )))
}

/// Translate the caller for `batch_slot` into the canonical same-module
/// key the index is keyed on. Selected members keep their cataloged key;
/// selected top-level functions translate by declared name and arity.
/// App Main is never a selected row — its key comes from the catalog's
/// `source_backed_app_main` co-seal, passed in by the cohort issuer.
fn caller_key_for_batch_slot(
    selected: &super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    batch_slot: u32,
) -> Option<CanonicalSameModuleCallableKeyV1> {
    selected
        .key_for_batch_slot(batch_slot)
        .and_then(|key| match key {
            SelectedNormalCallableKeyV1::Cataloged(key) => Some(key.clone()),
            SelectedNormalCallableKeyV1::TopLevel(top) => {
                u32::try_from(top.declared_arity()).ok().map(|arity| {
                    CanonicalSameModuleCallableKeyV1::free_function(top.declared_name(), arity)
                })
            }
        })
}
