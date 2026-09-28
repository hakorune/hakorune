//! Source-bound CoreMethod relations for selected callable method calls.
//!
//! Exact call sites and the sole Loop membership come from the consumed
//! typed-input product. CoreMethod targets come from one existing target
//! issuer session. This module only co-seals those authorities; it does not
//! select by name, issue Recipe keys, or observe MIR/physical identity.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_semantic_batch::{S6CCallSitePairRefV1, VerifiedS6CTypedInputRelationV1};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    issue_core_method_manifest_row_ref_v2, lookup_core_method_result_row_v2,
    CoreMethodManifestRowRefV2, CORE_METHOD_MANIFEST_BRAND_V2,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, CallableSemanticSourceLedgerView, CoreMethodHomeResultRelationV1,
    CoreMethodHomeSchemaV1, CoreMethodInstanceTargetIssuerV1, CoreMethodInstanceTargetRejectV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    ResolvedLoopPlacementV1, ResolvedLoopRegionLookupErrorV1, ResolvedMethodCallReceiverSourceV1,
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
    NamedArray(crate::mir::resolved_semantics::NamedArrayRequirementIssueV1),
    NamedArrayResidence(crate::mir::resolved_semantics::NamedArrayFieldResidenceIssueV1),
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
        let allowed = allowed_placements(manifest_row.op, call_arity);
        if allowed.is_empty() {
            continue;
        }
        // `indexOf/1` is catalog-unique on StringBox but the runtime router
        // also accepts `ArrayBox.indexOf/1`; arming it without evidence
        // would mint `StringIndexOf` on any lexical receiver. The bounded
        // arm therefore requires receiver- and needle-text evidence minted
        // from the same ledger rows — no selector-only issuance.
        if manifest_row.op == CoreMethodOp::StringIndexOf
            && !index_of_has_text_evidence(ledger, call, &rows)
        {
            continue;
        }
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
                placement
                    .filter(|placement| {
                        allowed.contains(placement)
                            // `length/0` is ambiguous across box receivers;
                            // a body-position site is armed only when the
                            // receiver's initializer carries text evidence
                            // (`TextToCaller` or a literal), the same
                            // bounded proof `indexOf` uses. Condition
                            // placement keeps its existing admission.
                            && !(manifest_row.op == CoreMethodOp::StringLen
                                && *placement == ResolvedLoopPlacementV1::Body
                                && !receiver_has_text_evidence(ledger, call, &rows))
                    })
                    .map(|placement| (loop_site, placement))
            })
            .collect::<Vec<_>>();
        let Some((loop_site, placement)) = nearest_loop(candidates)? else {
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
            placement,
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

/// Bounded text evidence for `StringIndexOf/1` — receiver and needle
/// must each resolve to a string literal or a `TextToCaller` contract
/// minted at the exact initializer site. Parameters carry no
/// initializer relation, so parameter receivers stay unarmed; a
/// non-text local (e.g. `local a = new ArrayBox()`) fails the same
/// check and the runtime router's `ArrayBox.indexOf` row never leaks
/// into this arm.
fn index_of_has_text_evidence(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    call: &VerifiedResolvedMethodCallSourceV1,
    rows: &BTreeMap<SourceExprSiteV1, VerifiedSourceBoundCoreMethodCallV1>,
) -> bool {
    if !receiver_has_text_evidence(ledger, call, rows) {
        return false;
    }
    let [needle] = call.arguments() else {
        return false;
    };
    text_source_at(ledger, needle.site(), rows, &mut BTreeSet::new(), 0)
}

/// Bounded receiver text evidence shared by the gated placements: the
/// receiver must be a caller-owned local binding with exactly one
/// initializer, no rebind, and a text-producing initializer site (string
/// literal or a `TextToCaller` contract minted in the same issuance).
/// Parameters carry no initializer relation, so parameter receivers stay
/// unarmed; a non-text local (e.g. `local a = new ArrayBox()`) fails the
/// same check and the runtime router's `ArrayBox` rows never leak in.
fn receiver_has_text_evidence(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    call: &VerifiedResolvedMethodCallSourceV1,
    rows: &BTreeMap<SourceExprSiteV1, VerifiedSourceBoundCoreMethodCallV1>,
) -> bool {
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        call.receiver()
    else {
        return false;
    };
    if binding.owner() != ledger.owner()
        || ledger.variable_ref(call.receiver_site())
            != Some(ResolvedLexicalRefV1::Local(binding))
        || binding_has_rebind(ledger, binding)
    {
        return false;
    }
    let Some(initializer_site) = single_initializer_site(ledger, binding) else {
        return false;
    };
    text_source_at(ledger, &initializer_site, rows, &mut BTreeSet::new(), 0)
}

fn single_initializer_site(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    binding: BindingRefV1,
) -> Option<SourceExprSiteV1> {
    let mut initializers = ledger
        .initializer_relations()
        .filter(|row| row.binding() == binding);
    let initializer = initializers.next()?;
    if initializers.next().is_some() {
        return None;
    }
    initializer.initializer_site().cloned()
}

fn binding_has_rebind(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    binding: BindingRefV1,
) -> bool {
    ledger.assignment_targets().any(|(_, target)| {
        matches!(target, ResolvedAssignmentTargetV1::BindingRebind(actual) if *actual == binding)
    })
}

const TEXT_SOURCE_DEPTH: u32 = 8;

/// Bounded text-source proof over exact resolver rows: a string literal,
/// a `TextToCaller` contract minted at this site in the same issuance, or
/// a local binding whose single initializer is text-producing and which
/// is never rebound. Everything else — integers, bools, handles, unknown
/// calls — is foreign to this family and leaves the call unarmed.
fn text_source_at(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceExprSiteV1,
    rows: &BTreeMap<SourceExprSiteV1, VerifiedSourceBoundCoreMethodCallV1>,
    visited: &mut BTreeSet<BindingRefV1>,
    depth: u32,
) -> bool {
    if depth > TEXT_SOURCE_DEPTH {
        return false;
    }
    if matches!(
        ledger.literal_source(site),
        Some(ResolvedLiteralSourceV1::String(_))
    ) {
        return true;
    }
    if let Some(row) = rows.get(site) {
        return row.contract().target().result()
            == CoreMethodHomeResultRelationV1::TextToCaller;
    }
    let Some(ResolvedLexicalRefV1::Local(binding)) = ledger.variable_ref(site) else {
        return false;
    };
    if !visited.insert(binding) {
        return true;
    }
    if binding_has_rebind(ledger, binding) {
        return false;
    }
    let Some(initializer_site) = single_initializer_site(ledger, binding) else {
        return false;
    };
    text_source_at(ledger, &initializer_site, rows, visited, depth + 1)
}

/// Bounded Loop placements admitted for one (op, arity) row. A member of
/// this set is exact vocabulary — the issued contract records the site's
/// actual resolved placement; an op absent here stays unarmed.
fn allowed_placements(op: CoreMethodOp, arity: u32) -> &'static [ResolvedLoopPlacementV1] {
    match (op, arity) {
        (CoreMethodOp::StringLen, 0) => &[
            ResolvedLoopPlacementV1::Condition,
            ResolvedLoopPlacementV1::Body,
        ],
        (CoreMethodOp::StringSubstring, 2) => &[
            ResolvedLoopPlacementV1::Body,
            ResolvedLoopPlacementV1::Condition,
        ],
        (CoreMethodOp::StringIndexOf, 1) => &[ResolvedLoopPlacementV1::Body],
        _ => &[],
    }
}

pub(super) fn nearest_loop<'a, T>(
    candidates: Vec<(&'a crate::mir::resolved_semantics::SourceStmtSiteV1, T)>,
) -> Result<
    Option<(&'a crate::mir::resolved_semantics::SourceStmtSiteV1, T)>,
    SourceBoundCoreMethodTargetIssueV1,
> {
    let Some(max_depth) = candidates
        .iter()
        .map(|(site, _)| site.node().segments().len())
        .max()
    else {
        return Ok(None);
    };
    let candidate_count = candidates.len();
    let mut nearest = candidates
        .into_iter()
        .filter(|(site, _)| site.node().segments().len() == max_depth);
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
