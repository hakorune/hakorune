//! Caller-zero loop-node winner + Recipe spine (M10b-I0-P1).
//!
//! One located loop site plus one resolved function lowering input is
//! co-sealed through: resolver lease -> five source attempts -> five family
//! rows -> admission window -> whole-unit coverage -> canonical selector ->
//! family Recipe demand -> family producer -> unified recipe product.
//!
//! This module has no production caller, mutates no Builder state, and
//! contains no fallback, retry, `Option`-skip, or re-decision edge. Every
//! non-issued outcome is a typed terminal: `Declined` for the selector's
//! `NoCandidate` arm, `Unresolved` for missing source/lease/window evidence,
//! and `Rejected` for every typed reject anywhere in the chain.

use super::direct_accum_observation::issue_direct_accum_source_attempt_v1;
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::generic_g0_observation::issue_generic_g0_source_attempt_with_window_v1;
use super::located::LocatedStmtV1;
use super::loop_cond_break_continue_observation::issue_loop_cond_source_attempt_v1;
use super::loop_cond_break_continue_typed_map::LoopCondTypedSourceMapRejectV1;
use super::loop_cond_break_continue_typed_map_issue::issue_loop_cond_break_continue_typed_source_map_v1;
use super::loop_true_break_continue_observation::issue_loop_true_source_attempt_v1;
use super::nested_predicate_observation::issue_nested_predicate_source_attempt_v1;
use super::nested_predicate_producer::{
    produce_nested_predicate_recipe_v1, NestedPredicateRecipeProducerRejectV1,
    VerifiedNestedPredicateRecipeProductV1,
};
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_recipe_contract::{
    issue_generic_g0_recipe_demand_v1, produce_direct_accum_recipe_v1,
    produce_generic_g0_recipe_v1, produce_loop_cond_break_continue_recipe_v1,
    produce_loop_true_break_continue_recipe_v1, DirectAccumRecipeProducerRejectV1,
    GenericG0RecipeDemandIssueV1, GenericG0RecipeProducerRejectV1,
    LoopCondBreakContinueRecipeProducerRejectV1, LoopRecipeProducerIdV1,
    LoopTrueBreakContinueRecipeProducerRejectV1, VerifiedDirectAccumRecipeProductV1,
    VerifiedGenericRecipeProductG0, VerifiedLoopCondBreakContinueRecipeProductV1,
    VerifiedLoopTrueBreakContinueRecipeProductV1,
};
use crate::mir::loop_route_policy::{
    assemble_loop_family_admission_window_v1,
    freeze_loop_route_schedule_v1, issue_all_route_observation_set_v1,
    issue_direct_accum_route_admission_v1, issue_loop_cond_break_continue_policy_demand_v1,
    issue_loop_true_break_continue_policy_demand_v1, issue_whole_unit_loop_coverage_proof_v1,
    select_canonical_loop_family_v1, CanonicalLoopFamilyCandidateV1,
    CanonicalLoopFamilySelectionFailureV1, CanonicalLoopFamilySelectionOutcomeV1,
    DirectAccumObservationContextV1, DirectAccumRouteAdmissionRejectV1,
    FrozenLoopRouteScheduleRejectV1, FrozenLoopRouteObservationV1, GenericG0ObservationContextV1,
    LoopAllRouteObservationRowV1, LoopAllRouteObservationSetRejectV1,
    LoopCondBreakContinuePolicyDemandRejectV1, LoopCondObservationContextV1,
    LoopFamilyAdmissionAssemblyOutcomeV1, LoopFamilyAdmissionFailureEvidenceV1,
    LoopFamilyObservationRowV1, LoopRouteCandidateFactsV1, LoopRouteObservationOutcomeV1,
    LoopRoutePolicyEvidenceV1, LoopRoutePolicySourceDeclineReasonV1, LoopRouteRecipeBackingV1,
    LoopRouteSourceDispositionV1, LoopRouteSuppressionDispositionV1, LoopTrueObservationContextV1,
    LoopGlobalEntryDispositionV1, LoopModeReleaseSnapshotV1, LoopReleaseAdmissionObservationV1,
    NestedPredicateObservationContextV1, VerifiedLoopFamilyAdmissionRowsV1,
    WholeUnitLoopCoverageProofV1, CANONICAL_LOOP_ROUTE_ORDER_V1,
    issue_direct_accum_family_observation_v1, issue_generic_g0_family_observation_v1,
    issue_loop_cond_family_observation_v1, issue_loop_true_family_observation_v1,
    issue_nested_predicate_family_observation_v1,
    DirectAccumFamilyObservationV1, GenericG0FamilyObservationV1, LoopCondFamilyObservationV1,
    LoopTrueFamilyObservationV1, NestedPredicateFamilyObservationV1,
    FrozenLoopRouteScheduleV1, LoopTrueBreakContinuePolicyDemandRejectV1,
};
use crate::mir::loop_structural_facts::{
    issue_selected_loop_recipe_demand_v1, DirectAccumObservationCoverageV1,
    DirectAccumObservationModeV1, DirectAccumSourceIdentityV1, GenericG0ObservationCoverageV1,
    GenericG0ObservationModeV1, GenericG0SourceIdentityV1, LoopCondObservationCoverageV1,
    LoopCondObservationModeV1, LoopCondSourceIdentityV1, LoopTrueObservationCoverageV1,
    LoopTrueObservationModeV1, LoopTrueSourceIdentityV1, NestedPredicateObservationCoverageV1,
    NestedPredicateObservationModeV1, NestedPredicateSourceIdentityV1, SelectedLoopDemandRejectV1,
};
use crate::mir::numeric_substrate::NumericTarget;
use crate::mir::resolved_semantics::{
    LoopFamilyWindowLeaseIssueV1, ResolvedLoopRegionLookupErrorV1,
};

/// Unified caller-zero recipe product. Exactly one family producer issues
/// exactly one product per selected unit; no second writer exists.
#[derive(Debug)]
pub(crate) enum LoopNodeWinnerRecipeV1 {
    DirectAccum(VerifiedDirectAccumRecipeProductV1),
    NestedPredicate(VerifiedNestedPredicateRecipeProductV1),
    LoopTrue(VerifiedLoopTrueBreakContinueRecipeProductV1),
    LoopCond(VerifiedLoopCondBreakContinueRecipeProductV1),
    GenericG0(VerifiedGenericRecipeProductG0),
}

/// Typed terminal failure at one spine stage. Carries the stage's own
/// sealed failure product; nothing is flattened to strings.
#[derive(Debug)]
pub(crate) enum LoopNodeWinnerSpineFailureV1 {
    Lease(LoopFamilyWindowLeaseIssueV1),
    LoopSource(ResolvedLoopRegionLookupErrorV1),
    WindowAssemble(LoopFamilyAdmissionFailureEvidenceV1),
    CoverageSet(LoopAllRouteObservationSetRejectV1),
    Selection(CanonicalLoopFamilySelectionFailureV1),
    DirectAccumAdmission(DirectAccumRouteAdmissionRejectV1),
    DirectAccumDemand(SelectedLoopDemandRejectV1),
    DirectAccumProducer(DirectAccumRecipeProducerRejectV1),
    NestedPredicateProducer(NestedPredicateRecipeProducerRejectV1),
    LoopTrueSchedule(FrozenLoopRouteScheduleRejectV1),
    LoopTrueDemand(LoopTrueBreakContinuePolicyDemandRejectV1),
    LoopTrueProducer(LoopTrueBreakContinueRecipeProducerRejectV1),
    LoopCondTypedMap(LoopCondTypedSourceMapRejectV1),
    LoopCondSchedule(FrozenLoopRouteScheduleRejectV1),
    LoopCondDemand(LoopCondBreakContinuePolicyDemandRejectV1),
    LoopCondProducer(LoopCondBreakContinueRecipeProducerRejectV1),
    GenericG0Demand(GenericG0RecipeDemandIssueV1),
    GenericG0Producer(GenericG0RecipeProducerRejectV1),
}

/// One-shot spine outcome. `Issued` is the only non-terminal arm; the other
/// three are the typed terminal vocabulary the R0 switch freezes on.
#[derive(Debug)]
pub(crate) enum LoopNodeWinnerSpineOutcomeV1 {
    Issued(LoopNodeWinnerRecipeV1),
    Declined(WholeUnitLoopCoverageProofV1),
    Unresolved(LoopNodeWinnerSpineFailureV1),
    Rejected(LoopNodeWinnerSpineFailureV1),
}

/// Run the caller-zero winner+Recipe spine for one located loop site.
///
/// `mode` seals every family attempt in the same admission mode; `coverage`
/// must be `Complete` for the window to assemble. `numeric` feeds only the
/// GenericG0 arm's numeric projection.
pub(crate) fn issue_loop_node_winner_recipe_v1<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    numeric: NumericTarget,
) -> LoopNodeWinnerSpineOutcomeV1 {
    use LoopNodeWinnerSpineOutcomeV1 as Outcome;

    let site = loop_stmt.site().clone();
    let lease = match input.function().issue_loop_family_window_lease_v1(&site) {
        Ok(lease) => lease,
        Err(issue) => {
            return Outcome::Unresolved(LoopNodeWinnerSpineFailureV1::Lease(issue))
        }
    };
    let frame = lease.frame();
    let identity = |site: &crate::mir::resolved_semantics::SourceStmtSiteV1| {
        (
            input.owner(),
            input.function().function_origin(),
            input.function().source_kind(),
            site.clone(),
            frame.clone(),
        )
    };
    let (owner, origin, source_kind, site, frame) = identity(&site);

    let reissue_source = |site: &crate::mir::resolved_semantics::SourceStmtSiteV1| {
        input.function().resolved_loop_source(site)
    };
    let source = match reissue_source(&site) {
        Ok(source) => source,
        Err(error) => {
            return Outcome::Unresolved(LoopNodeWinnerSpineFailureV1::LoopSource(error))
        }
    };
    let direct_attempt = issue_direct_accum_source_attempt_v1(
        input,
        loop_stmt.clone(),
        source,
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
    );
    let source = match reissue_source(&site) {
        Ok(source) => source,
        Err(error) => {
            return Outcome::Unresolved(LoopNodeWinnerSpineFailureV1::LoopSource(error))
        }
    };
    let nested_attempt = issue_nested_predicate_source_attempt_v1(
        input,
        loop_stmt.clone(),
        source,
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
    );
    let source = match reissue_source(&site) {
        Ok(source) => source,
        Err(error) => {
            return Outcome::Unresolved(LoopNodeWinnerSpineFailureV1::LoopSource(error))
        }
    };
    let loop_true_attempt = issue_loop_true_source_attempt_v1(
        input,
        loop_stmt.clone(),
        source,
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
    );
    let source = match reissue_source(&site) {
        Ok(source) => source,
        Err(error) => {
            return Outcome::Unresolved(LoopNodeWinnerSpineFailureV1::LoopSource(error))
        }
    };
    let loop_cond_attempt = issue_loop_cond_source_attempt_v1(
        input,
        loop_stmt.clone(),
        source,
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
    );
    let source = match reissue_source(&site) {
        Ok(source) => source,
        Err(error) => {
            return Outcome::Unresolved(LoopNodeWinnerSpineFailureV1::LoopSource(error))
        }
    };
    let generic_attempt = issue_generic_g0_source_attempt_with_window_v1(
        input,
        loop_stmt,
        source,
        &lease,
        numeric,
        Some(GenericG0ObservationModeV1::Release),
        GenericG0ObservationCoverageV1::Complete,
    );

    let rows: Box<[LoopFamilyObservationRowV1]> = Box::new([
        issue_direct_accum_family_observation_v1(
            direct_attempt,
            DirectAccumObservationContextV1::issue(
                DirectAccumSourceIdentityV1::new(
                    owner,
                    origin,
                    source_kind,
                    site.clone(),
                    frame.clone(),
                ),
                Some(DirectAccumObservationModeV1::Release),
                DirectAccumObservationCoverageV1::Complete,
            ),
        )
        .into_admission_row(),
        issue_nested_predicate_family_observation_v1(
            nested_attempt,
            NestedPredicateObservationContextV1::issue(
                NestedPredicateSourceIdentityV1::new(
                    owner,
                    origin,
                    source_kind,
                    site.clone(),
                    frame.clone(),
                ),
                Some(NestedPredicateObservationModeV1::Release),
                NestedPredicateObservationCoverageV1::Complete,
            ),
        )
        .into_admission_row(),
        issue_loop_true_family_observation_v1(
            loop_true_attempt,
            LoopTrueObservationContextV1::issue(
                LoopTrueSourceIdentityV1::new(
                    owner,
                    origin,
                    source_kind,
                    site.clone(),
                    frame.clone(),
                ),
                Some(LoopTrueObservationModeV1::Release),
                LoopTrueObservationCoverageV1::Complete,
            ),
        )
        .into_admission_row(),
        issue_loop_cond_family_observation_v1(
            loop_cond_attempt,
            LoopCondObservationContextV1::issue(
                LoopCondSourceIdentityV1::new(
                    owner,
                    origin,
                    source_kind,
                    site.clone(),
                    frame.clone(),
                ),
                Some(LoopCondObservationModeV1::Release),
                LoopCondObservationCoverageV1::Complete,
            ),
        )
        .into_admission_row(),
        issue_generic_g0_family_observation_v1(
            generic_attempt,
            GenericG0ObservationContextV1::issue(
                GenericG0SourceIdentityV1::new(
                    owner,
                    origin,
                    source_kind,
                    site.clone(),
                    frame.clone(),
                ),
                Some(GenericG0ObservationModeV1::Release),
                GenericG0ObservationCoverageV1::Complete,
            ),
        )
        .into_admission_row(),
    ]);

    let window = match assemble_loop_family_admission_window_v1(lease, rows) {
        LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) => window,
        LoopFamilyAdmissionAssemblyOutcomeV1::Unresolved(evidence) => {
            return Outcome::Unresolved(LoopNodeWinnerSpineFailureV1::WindowAssemble(
                evidence,
            ))
        }
        LoopFamilyAdmissionAssemblyOutcomeV1::Rejected(evidence) => {
            return Outcome::Rejected(LoopNodeWinnerSpineFailureV1::WindowAssemble(
                evidence,
            ))
        }
    };

    let coverage_set = match issue_all_route_observation_set_v1(unit_route_rows(window.rows())) {
        Ok(set) => set,
        Err(reject) => {
            return Outcome::Rejected(LoopNodeWinnerSpineFailureV1::CoverageSet(reject))
        }
    };
    let unit_coverage = issue_whole_unit_loop_coverage_proof_v1(coverage_set, window.lease());

    let selection = match select_canonical_loop_family_v1(window, unit_coverage) {
        CanonicalLoopFamilySelectionOutcomeV1::Selected(selection) => selection,
        CanonicalLoopFamilySelectionOutcomeV1::NoCandidate(proof) => {
            return Outcome::Declined(proof)
        }
        CanonicalLoopFamilySelectionOutcomeV1::Rejected(failure) => {
            return Outcome::Rejected(LoopNodeWinnerSpineFailureV1::Selection(failure))
        }
    };

    if matches!(
        selection.candidate(),
        CanonicalLoopFamilyCandidateV1::GenericG0(_)
    ) {
        // The Generic demand consumes the sealed selection itself (window
        // lease + candidate + coverage proof), so this arm issues before
        // `into_parts`.
        let demand = match issue_generic_g0_recipe_demand_v1(selection) {
            Ok(demand) => demand,
            Err(issue) => {
                return Outcome::Rejected(LoopNodeWinnerSpineFailureV1::GenericG0Demand(
                    issue,
                ))
            }
        };
        return match produce_generic_g0_recipe_v1(demand) {
            Ok(product) => Outcome::Issued(LoopNodeWinnerRecipeV1::GenericG0(product)),
            Err(reject) => Outcome::Rejected(LoopNodeWinnerSpineFailureV1::GenericG0Producer(
                reject,
            )),
        };
    }

    let (_, _, _, candidate, _) = selection.into_parts();
    match candidate {
        CanonicalLoopFamilyCandidateV1::DirectAccum(candidate) => {
            issue_direct_accum_recipe(input, candidate).map_or_else(
                |failure| Outcome::Rejected(failure),
                |product| Outcome::Issued(LoopNodeWinnerRecipeV1::DirectAccum(product)),
            )
        }
        CanonicalLoopFamilyCandidateV1::NestedPredicate(candidate) => {
            let (projection, _) = candidate.into_parts();
            produce_nested_predicate_recipe_v1(projection, input.function()).map_or_else(
                |reject| {
                    Outcome::Rejected(LoopNodeWinnerSpineFailureV1::NestedPredicateProducer(
                        reject,
                    ))
                },
                |product| Outcome::Issued(LoopNodeWinnerRecipeV1::NestedPredicate(product)),
            )
        }
        CanonicalLoopFamilyCandidateV1::LoopTrue(candidate) => {
            issue_loop_true_recipe(input, candidate).map_or_else(
                |failure| Outcome::Rejected(failure),
                |product| Outcome::Issued(LoopNodeWinnerRecipeV1::LoopTrue(product)),
            )
        }
        CanonicalLoopFamilyCandidateV1::LoopCond(candidate) => {
            issue_loop_cond_recipe(input, candidate).map_or_else(
                |failure| Outcome::Rejected(failure),
                |product| Outcome::Issued(LoopNodeWinnerRecipeV1::LoopCond(product)),
            )
        }
        CanonicalLoopFamilyCandidateV1::GenericG0(_) => {
            unreachable!("GenericG0 is dispatched before into_parts")
        }
    }
}

/// Empirical family-to-route admission evidence (M10b probe, 2026-09-24).
/// `RecipeBacked` marks only the canonical route whose landed cohort the
/// single family candidate attests; `GenericG0` is a semantic family outside
/// the canonical route inventory (S6G) and therefore marks no route row.
/// Route IDs never dispatch — this table is migration coverage only.
fn unit_route_rows(
    rows: &VerifiedLoopFamilyAdmissionRowsV1,
) -> Box<[LoopAllRouteObservationRowV1]> {
    let portable = LoopRouteRecipeBackingV1::PortableProducer;
    let backed = family_backed_route(rows).map(|route| {
        let backing = match route {
            LoopRouteId::AccumConstLoop => portable(LoopRecipeProducerIdV1::DirectAccumV1),
            LoopRouteId::NestedLoopMinimal => portable(LoopRecipeProducerIdV1::NestedPredicateV1),
            LoopRouteId::LoopTrueBreakContinue => {
                portable(LoopRecipeProducerIdV1::LoopTrueBreakContinueV1)
            }
            LoopRouteId::LoopCondBreakContinue => {
                portable(LoopRecipeProducerIdV1::LoopCondBreakContinueV1)
            }
            _ => unreachable!("only family-owned attested routes reach the backed arm"),
        };
        (route, backing)
    });
    CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .map(|route| LoopAllRouteObservationRowV1 {
            route: *route,
            outcome: match backed {
                Some((backed_route, backing)) if *route == backed_route => {
                    LoopRouteObservationOutcomeV1::RecipeBacked(backing)
                }
                _ => LoopRouteObservationOutcomeV1::PreEffectDeclined(
                    LoopRoutePolicySourceDeclineReasonV1::PreEffectDeclined,
                ),
            },
        })
        .collect()
}

fn family_backed_route(rows: &VerifiedLoopFamilyAdmissionRowsV1) -> Option<LoopRouteId> {
    let mut backed = None;
    let mut count = 0usize;
    if matches!(
        rows.direct_accum(),
        DirectAccumFamilyObservationV1::Candidate(_)
    ) {
        backed = Some(LoopRouteId::AccumConstLoop);
        count += 1;
    }
    if matches!(
        rows.nested_predicate(),
        NestedPredicateFamilyObservationV1::Candidate(_)
    ) {
        backed = Some(LoopRouteId::NestedLoopMinimal);
        count += 1;
    }
    if matches!(
        rows.loop_true(),
        LoopTrueFamilyObservationV1::Candidate(_)
    ) {
        backed = Some(LoopRouteId::LoopTrueBreakContinue);
        count += 1;
    }
    if matches!(
        rows.loop_cond(),
        LoopCondFamilyObservationV1::Candidate(_)
    ) {
        backed = Some(LoopRouteId::LoopCondBreakContinue);
        count += 1;
    }
    // GenericG0 marks no route row: the S6G inventory places the G0 profile
    // outside the canonical route vocabulary.
    let _ = matches!(
        rows.generic_g0(),
        GenericG0FamilyObservationV1::Candidate(_)
    );
    if count == 1 { backed } else { None }
}

fn issue_direct_accum_recipe<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    candidate: crate::mir::loop_route_policy::VerifiedDirectAccumFamilyCandidateV1,
) -> Result<VerifiedDirectAccumRecipeProductV1, LoopNodeWinnerSpineFailureV1> {
    let (observation, _) = candidate.into_parts();
    let handoff = issue_direct_accum_route_admission_v1(observation)
        .map_err(LoopNodeWinnerSpineFailureV1::DirectAccumAdmission)?;
    let (admission, observation) = handoff.into_parts();
    let winner = admission.into_policy_winner();
    let (facts, source) = observation.into_parts();
    let demand = issue_selected_loop_recipe_demand_v1(winner, facts, source)
        .map_err(LoopNodeWinnerSpineFailureV1::DirectAccumDemand)?;
    produce_direct_accum_recipe_v1(demand, input.function())
        .map_err(LoopNodeWinnerSpineFailureV1::DirectAccumProducer)
}

fn issue_loop_true_recipe<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    candidate: crate::mir::loop_route_policy::VerifiedLoopTrueFamilyCandidateV1,
) -> Result<VerifiedLoopTrueBreakContinueRecipeProductV1, LoopNodeWinnerSpineFailureV1> {
    let (projection, _) = candidate.into_parts();
    let schedule = family_route_schedule(LoopRouteId::LoopTrueBreakContinue)
        .map_err(LoopNodeWinnerSpineFailureV1::LoopTrueSchedule)?;
    let demand = issue_loop_true_break_continue_policy_demand_v1(projection, schedule)
        .map_err(LoopNodeWinnerSpineFailureV1::LoopTrueDemand)?;
    produce_loop_true_break_continue_recipe_v1(demand, input.function())
        .map_err(LoopNodeWinnerSpineFailureV1::LoopTrueProducer)
}

fn issue_loop_cond_recipe<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    candidate: crate::mir::loop_route_policy::VerifiedLoopCondFamilyCandidateV1,
) -> Result<VerifiedLoopCondBreakContinueRecipeProductV1, LoopNodeWinnerSpineFailureV1> {
    let (projection, _) = candidate.into_parts();
    let map = issue_loop_cond_break_continue_typed_source_map_v1(input, projection)
        .map_err(LoopNodeWinnerSpineFailureV1::LoopCondTypedMap)?;
    let schedule = family_route_schedule(LoopRouteId::LoopCondBreakContinue)
        .map_err(LoopNodeWinnerSpineFailureV1::LoopCondSchedule)?;
    let demand = issue_loop_cond_break_continue_policy_demand_v1(map, schedule)
        .map_err(LoopNodeWinnerSpineFailureV1::LoopCondDemand)?;
    produce_loop_cond_break_continue_recipe_v1(demand, input.function())
        .map_err(LoopNodeWinnerSpineFailureV1::LoopCondProducer)
}

/// Frozen-schedule provenance for one family demand: the family's owned
/// canonical route is the single `Candidate` row; every other row is a typed
/// source decline. The demand issuer evaluates this schedule internally and
/// cursor-checks the winner — retained provenance, never a second selector.
fn family_route_schedule(
    owned_route: LoopRouteId,
) -> Result<FrozenLoopRouteScheduleV1, FrozenLoopRouteScheduleRejectV1> {
    let observations = CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .map(|route| {
            let evidence = if *route == owned_route {
                LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable)
            } else {
                LoopRoutePolicyEvidenceV1::SourceDeclined(
                    LoopRoutePolicySourceDeclineReasonV1::ExcludedByVerifiedSingletonObservation,
                )
            };
            FrozenLoopRouteObservationV1::new(
                LoopRouteSuppressionDispositionV1::Retained,
                LoopModeReleaseSnapshotV1::Release {
                    admission: LoopReleaseAdmissionObservationV1::Allowed,
                },
                LoopGlobalEntryDispositionV1::Allowed,
                LoopRouteSourceDispositionV1::Available,
                evidence,
            )
        })
        .collect::<Box<[_]>>();
    freeze_loop_route_schedule_v1(CANONICAL_LOOP_ROUTE_ORDER_V1.into(), observations)
}
