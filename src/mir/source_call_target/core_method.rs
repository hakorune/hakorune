//! Source-bound CoreMethod relations for selected callable method calls.
//!
//! Exact call sites and the sole Loop membership come from the consumed
//! typed-input product. CoreMethod targets come from one existing target
//! issuer session. This module only co-seals those authorities; it does not
//! select by name, issue Recipe keys, or observe MIR/physical identity.

use std::collections::BTreeMap;

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_semantic_batch::{S6CCallSitePairRefV1, VerifiedS6CTypedInputRelationV1};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    issue_core_method_manifest_row_ref_v2, lookup_core_method_result_row_v2,
    CoreMethodManifestRowRefV2, CORE_METHOD_MANIFEST_BRAND_V2,
};
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, CoreMethodHomeSchemaV1, CoreMethodInstanceTargetIssuerV1,
    CoreMethodInstanceTargetRejectV1, ResolvedLoopPlacementV1, ResolvedLoopRegionLookupErrorV1,
    ResolverCoreMethodCallableContractIssuerV1, ResolverCoreMethodCallableContractRejectV1,
    SourceExprSiteV1, VerifiedCoreMethodInstanceTargetV1, VerifiedResolvedMethodCallSourceV1,
    VerifiedResolverCoreMethodCallableContractV1,
};

use super::{VerifiedSourceCallTargetCatalogV1, VerifiedSourceCallTargetV1};

/// One route-neutral source-bound CoreMethod row.  The contract already owns
/// the resolver call, loop placement and generated target; this wrapper only
/// records its catalog arm.
#[derive(Debug)]
pub(crate) struct VerifiedSourceBoundCoreMethodCallV1 {
    contract: VerifiedResolverCoreMethodCallableContractV1,
}

impl VerifiedSourceBoundCoreMethodCallV1 {
    pub(crate) fn new(contract: VerifiedResolverCoreMethodCallableContractV1) -> Self {
        Self { contract }
    }

    pub(crate) fn contract(&self) -> &VerifiedResolverCoreMethodCallableContractV1 {
        &self.contract
    }

    pub(crate) fn into_contract(self) -> VerifiedResolverCoreMethodCallableContractV1 {
        self.contract
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SourceBoundCoreMethodTargetIssueV1 {
    LoopLookup(ResolvedLoopRegionLookupErrorV1),
    LoopMembership(ResolvedLoopRegionLookupErrorV1),
    Target(CoreMethodInstanceTargetRejectV1),
    Contract(ResolverCoreMethodCallableContractRejectV1),
    DuplicateSite(SourceExprSiteV1),
    CatalogCollision(SourceExprSiteV1),
    ForeignCaller(CanonicalSameModuleCallableKeyV1),
}

/// Issue all loop-local StringBox CoreMethod contracts for one resolver owner.
///
/// Calls outside a loop are deliberately omitted: this slice only arms the
/// LoopCond source consumer.  Calls inside a loop are selected by the resolver
/// placement and exact source site, never by AST/name rescanning.
pub(crate) fn issue_source_bound_core_method_calls_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
) -> Result<
    Box<[(SourceExprSiteV1, VerifiedSourceBoundCoreMethodCallV1)]>,
    SourceBoundCoreMethodTargetIssueV1,
> {
    let mut rows = BTreeMap::new();
    let loop_sites = ledger.loop_sites().cloned().collect::<Vec<_>>();
    for (site, call) in ledger.method_calls() {
        let Some(manifest_row) =
            lookup_core_method_result_row_v2("StringBox", call.selector(), call.arity())
        else {
            continue;
        };
        let call_arity = call.arity();
        let Some(expected_placement) = supported_placement(manifest_row.op, call_arity) else {
            continue;
        };
        let candidates = loop_sites
            .iter()
            .filter_map(|loop_site| {
                Some((loop_site, ledger.resolved_loop_placement(loop_site, site)))
            })
            .map(|(loop_site, placement)| {
                placement
                    .map(|placement| (loop_site, placement))
                    .map_err(SourceBoundCoreMethodTargetIssueV1::LoopLookup)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter_map(|(loop_site, placement)| {
                (placement == Some(expected_placement)).then_some(loop_site)
            })
            .collect::<Vec<_>>();
        let Some(loop_site) = nearest_loop(candidates)? else {
            continue;
        };
        let membership = ledger
            .resolved_loop_source(loop_site)
            .map_err(SourceBoundCoreMethodTargetIssueV1::LoopMembership)?;
        let manifest_row = issue_manifest_row(manifest_row.op, call_arity).ok_or(
            SourceBoundCoreMethodTargetIssueV1::Target(
                CoreMethodInstanceTargetRejectV1::UnsupportedOperation {
                    op: manifest_row.op,
                    arity: call_arity,
                },
            ),
        )?;
        let mut issuer =
            CoreMethodInstanceTargetIssuerV1::string_box_text(CORE_METHOD_MANIFEST_BRAND_V2)
                .map_err(SourceBoundCoreMethodTargetIssueV1::Target)?;
        let target = issuer
            .issue(manifest_row)
            .map_err(SourceBoundCoreMethodTargetIssueV1::Target)?;
        let contract = match ResolverCoreMethodCallableContractIssuerV1::issue(
            ledger,
            call,
            &membership,
            expected_placement,
            target,
        ) {
            Ok(contract) => contract,
            Err(ResolverCoreMethodCallableContractRejectV1::UnsupportedReceiver) => {
                // This bounded LoopCond arm only owns lexical local receivers.
                // Qualified/current-owner/other receivers remain unarmed for
                // their existing owner; they are not a package-wide error.
                continue;
            }
            Err(error) => return Err(SourceBoundCoreMethodTargetIssueV1::Contract(error)),
        };
        let site = site.clone();
        if rows
            .insert(
                site.clone(),
                VerifiedSourceBoundCoreMethodCallV1::new(contract),
            )
            .is_some()
        {
            return Err(SourceBoundCoreMethodTargetIssueV1::DuplicateSite(site));
        }
    }
    Ok(rows.into_iter().collect())
}

fn issue_manifest_row(op: CoreMethodOp, arity: u32) -> Option<CoreMethodManifestRowRefV2> {
    issue_core_method_manifest_row_ref_v2(op, arity)
}

fn supported_placement(op: CoreMethodOp, arity: u32) -> Option<ResolvedLoopPlacementV1> {
    match (op, arity) {
        (CoreMethodOp::StringLen, 0) => Some(ResolvedLoopPlacementV1::Condition),
        (CoreMethodOp::StringSubstring, 2) => Some(ResolvedLoopPlacementV1::Body),
        _ => None,
    }
}

fn nearest_loop<'a>(
    candidates: Vec<&'a crate::mir::resolved_semantics::SourceStmtSiteV1>,
) -> Result<
    Option<&'a crate::mir::resolved_semantics::SourceStmtSiteV1>,
    SourceBoundCoreMethodTargetIssueV1,
> {
    let Some(max_depth) = candidates
        .iter()
        .map(|site| site.node().segments().len())
        .max()
    else {
        return Ok(None);
    };
    let candidate_count = candidates.len();
    let mut nearest = candidates
        .into_iter()
        .filter(|site| site.node().segments().len() == max_depth);
    let Some(first) = nearest.next() else {
        return Ok(None);
    };
    if nearest.next().is_some() {
        return Err(SourceBoundCoreMethodTargetIssueV1::LoopLookup(
            ResolvedLoopRegionLookupErrorV1::NoUniqueLoopSite {
                actual: candidate_count,
            },
        ));
    }
    Ok(Some(first))
}

impl<'catalog> VerifiedSourceCallTargetCatalogV1<'catalog> {
    /// Atomically add the resolver-owned CoreMethod arm to this existing
    /// route-neutral catalog.  The caller/site key remains the same identity
    /// used by Static and DynamicMember rows.
    pub(crate) fn extend_core_method_calls(
        mut self,
        caller: CanonicalSameModuleCallableKeyV1,
        rows: impl IntoIterator<Item = (SourceExprSiteV1, VerifiedSourceBoundCoreMethodCallV1)>,
    ) -> Result<Self, SourceBoundCoreMethodTargetIssueV1> {
        if self.declarations.declaration(&caller).is_none() {
            return Err(SourceBoundCoreMethodTargetIssueV1::ForeignCaller(caller));
        }
        for (site, target) in rows {
            let key = (caller.clone(), site.clone());
            if self.rows.contains_key(&key) {
                return Err(SourceBoundCoreMethodTargetIssueV1::CatalogCollision(site));
            }
            self.rows
                .insert(key, VerifiedSourceCallTargetV1::CoreMethod(target));
        }
        Ok(self)
    }

    /// Consume the catalog arm into the selected callable's lowering state.
    /// This is a transport projection; it does not create another target map.
    pub(crate) fn into_core_method_calls(
        self,
        caller: &CanonicalSameModuleCallableKeyV1,
    ) -> Result<BTreeMap<SourceExprSiteV1, VerifiedSourceBoundCoreMethodCallV1>, SourceExprSiteV1>
    {
        let mut rows = BTreeMap::new();
        for ((row_caller, site), target) in self.rows {
            if &row_caller != caller {
                continue;
            }
            let VerifiedSourceCallTargetV1::CoreMethod(target) = target else {
                return Err(site);
            };
            if rows.insert(site.clone(), target).is_some() {
                return Err(site);
            }
        }
        Ok(rows)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CSourceBoundCallRoleV1 {
    Length,
    Substring,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum S6CSourceBoundCallRelationRejectV1 {
    MixedManifestBrand,
    MixedSchema,
    MixedRelationBrand,
    DuplicateTargetBrand,
    WrongTargetRole {
        role: S6CSourceBoundCallRoleV1,
        op: CoreMethodOp,
        arity: u32,
    },
    CallSiteCoverage {
        role: S6CSourceBoundCallRoleV1,
        actual: usize,
    },
    Callable {
        role: S6CSourceBoundCallRoleV1,
        reject: ResolverCoreMethodCallableContractRejectV1,
    },
}

/// Borrow-only view over the fixed source-bound relation.
#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CSourceBoundCallRelationRefV1<'a> {
    typed: &'a VerifiedS6CTypedInputRelationV1,
    length: &'a VerifiedResolverCoreMethodCallableContractV1,
    substring: &'a VerifiedResolverCoreMethodCallableContractV1,
}

impl<'a> S6CSourceBoundCallRelationRefV1<'a> {
    pub(crate) const fn typed(self) -> &'a VerifiedS6CTypedInputRelationV1 {
        self.typed
    }

    pub(crate) const fn length(self) -> &'a VerifiedResolverCoreMethodCallableContractV1 {
        self.length
    }

    pub(crate) const fn substring(self) -> &'a VerifiedResolverCoreMethodCallableContractV1 {
        self.substring
    }
}

/// Non-Clone fixed relation for `length/0` and `substring/2`.
#[derive(Debug)]
pub(crate) struct VerifiedSourceBoundS6CCallRelationV1 {
    typed: VerifiedS6CTypedInputRelationV1,
    length: VerifiedResolverCoreMethodCallableContractV1,
    substring: VerifiedResolverCoreMethodCallableContractV1,
}

impl VerifiedSourceBoundS6CCallRelationV1 {
    pub(crate) fn with_relation<R>(
        &self,
        callback: impl for<'relation> FnOnce(S6CSourceBoundCallRelationRefV1<'relation>) -> R,
    ) -> R {
        callback(S6CSourceBoundCallRelationRefV1 {
            typed: &self.typed,
            length: &self.length,
            substring: &self.substring,
        })
    }
}

pub(crate) fn issue_source_bound_s6c_call_relation_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    typed: VerifiedS6CTypedInputRelationV1,
    length_target: VerifiedCoreMethodInstanceTargetV1,
    substring_target: VerifiedCoreMethodInstanceTargetV1,
) -> Result<VerifiedSourceBoundS6CCallRelationV1, S6CSourceBoundCallRelationRejectV1> {
    verify_target_pair(&length_target, &substring_target)?;

    let (length, substring) = typed.with_call_sites(|sites| {
        let length_call = exact_call(ledger, sites, S6CSourceBoundCallRoleV1::Length)?;
        let substring_call = exact_call(ledger, sites, S6CSourceBoundCallRoleV1::Substring)?;
        let length = ResolverCoreMethodCallableContractIssuerV1::issue(
            ledger,
            length_call,
            typed.membership(),
            sites.length_placement(),
            length_target,
        )
        .map_err(|reject| S6CSourceBoundCallRelationRejectV1::Callable {
            role: S6CSourceBoundCallRoleV1::Length,
            reject,
        })?;
        let substring = ResolverCoreMethodCallableContractIssuerV1::issue(
            ledger,
            substring_call,
            typed.membership(),
            sites.substring_placement(),
            substring_target,
        )
        .map_err(|reject| S6CSourceBoundCallRelationRejectV1::Callable {
            role: S6CSourceBoundCallRoleV1::Substring,
            reject,
        })?;
        Ok((length, substring))
    })?;

    Ok(VerifiedSourceBoundS6CCallRelationV1 {
        typed,
        length,
        substring,
    })
}

fn verify_target_pair(
    length: &VerifiedCoreMethodInstanceTargetV1,
    substring: &VerifiedCoreMethodInstanceTargetV1,
) -> Result<(), S6CSourceBoundCallRelationRejectV1> {
    if length.manifest_brand() != substring.manifest_brand() {
        return Err(S6CSourceBoundCallRelationRejectV1::MixedManifestBrand);
    }
    if length.schema() != substring.schema()
        || length.schema() != CoreMethodHomeSchemaV1::StringBoxText
    {
        return Err(S6CSourceBoundCallRelationRejectV1::MixedSchema);
    }
    if length.relation_brand() != substring.relation_brand() {
        return Err(S6CSourceBoundCallRelationRejectV1::MixedRelationBrand);
    }
    if length.target_brand() == substring.target_brand() {
        return Err(S6CSourceBoundCallRelationRejectV1::DuplicateTargetBrand);
    }
    require_target_role(
        length,
        S6CSourceBoundCallRoleV1::Length,
        CoreMethodOp::StringLen,
        0,
    )?;
    require_target_role(
        substring,
        S6CSourceBoundCallRoleV1::Substring,
        CoreMethodOp::StringSubstring,
        2,
    )
}

fn require_target_role(
    target: &VerifiedCoreMethodInstanceTargetV1,
    role: S6CSourceBoundCallRoleV1,
    expected_op: CoreMethodOp,
    expected_arity: u32,
) -> Result<(), S6CSourceBoundCallRelationRejectV1> {
    let row = target.row();
    if row.row().op != expected_op || row.arity() != expected_arity {
        return Err(S6CSourceBoundCallRelationRejectV1::WrongTargetRole {
            role,
            op: row.row().op,
            arity: row.arity(),
        });
    }
    Ok(())
}

fn exact_call<'a>(
    ledger: &'a CallableSemanticSourceLedgerView<'_>,
    sites: S6CCallSitePairRefV1<'_>,
    role: S6CSourceBoundCallRoleV1,
) -> Result<&'a VerifiedResolvedMethodCallSourceV1, S6CSourceBoundCallRelationRejectV1> {
    let site = match role {
        S6CSourceBoundCallRoleV1::Length => sites.length_site(),
        S6CSourceBoundCallRoleV1::Substring => sites.substring_site(),
    };
    let matching = ledger
        .method_calls()
        .filter(|(candidate, _)| *candidate == site)
        .map(|(_, call)| call)
        .collect::<Vec<_>>();
    match matching.as_slice() {
        [call] => Ok(*call),
        rows => Err(S6CSourceBoundCallRelationRejectV1::CallSiteCoverage {
            role,
            actual: rows.len(),
        }),
    }
}
