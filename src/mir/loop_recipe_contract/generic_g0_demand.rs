//! Caller-zero Generic G0 demand product.
//!
//! This test-only seam consumes the canonical family selection exactly once.
//! It retains the resolver window, the handoff brand, the typed source bundle,
//! post-loop completion read, and the common admission context.  It does not
//! issue Recipe keys, relations, JoinSig, After, Builder, or MIR products.

#![cfg(test)]

use crate::mir::loop_route_policy::{
    CanonicalLoopFamilySelectionV1, GenericG0CoverageV1, GenericG0ObservationEvidenceV1,
    GenericG0PolicyContextV1, GenericG0PolicyModeV1, GenericG0PolicyProfileV1,
    LoopFamilyAdmissionCoverageV1, LoopFamilyAdmissionModeV1, LoopFamilyTagV1,
};
use crate::mir::loop_structural_facts::generic_g0::{
    GenericG0SourceBrandV1, VerifiedGenericG0PolicyHandoffV1, VerifiedGenericG0PostLoopReadV1,
    VerifiedGenericTypedSourceBundleG0,
};
use crate::mir::loop_structural_facts::{
    GenericG0ObservationCoverageV1, GenericG0ObservationModeV1,
};
use crate::mir::resolved_semantics::VerifiedLoopFamilyWindowLeaseV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0RoleLeaseRejectV1 {
    BindingRoleMismatch,
    ReturnRelationMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0RoleLeaseV1 {
    _seal: GenericG0RoleLeaseSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericG0RoleLeaseSealV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0RecipeDemandIssueV1 {
    SelectedOtherFamily(LoopFamilyTagV1),
    EvidenceIdentityMismatch,
    EvidenceModeMismatch,
    EvidenceCoverageMismatch,
    PolicyOwnerMismatch,
    PolicyProfileMismatch,
    PolicyModeMismatch,
    PolicyCoverageMismatch,
    LeaseBrandMismatch,
    TargetMismatch,
    Role(GenericG0RoleLeaseRejectV1),
}

/// S3's move-only demand.  The role lease is only an opaque proof that the
/// handoff's existing exact role rows were checked; it does not duplicate
/// any source site or BindingRef.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericRecipeDemandG0 {
    window_lease: VerifiedLoopFamilyWindowLeaseV1,
    source_brand: GenericG0SourceBrandV1,
    source_bundle: VerifiedGenericTypedSourceBundleG0,
    post_loop_read: VerifiedGenericG0PostLoopReadV1,
    profile: GenericG0PolicyProfileV1,
    mode: LoopFamilyAdmissionModeV1,
    coverage: LoopFamilyAdmissionCoverageV1,
    role_lease: VerifiedGenericG0RoleLeaseV1,
}

impl VerifiedGenericRecipeDemandG0 {
    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopFamilyWindowLeaseV1,
        GenericG0SourceBrandV1,
        VerifiedGenericTypedSourceBundleG0,
        VerifiedGenericG0PostLoopReadV1,
        GenericG0PolicyProfileV1,
        LoopFamilyAdmissionModeV1,
        LoopFamilyAdmissionCoverageV1,
        VerifiedGenericG0RoleLeaseV1,
    ) {
        (
            self.window_lease,
            self.source_brand,
            self.source_bundle,
            self.post_loop_read,
            self.profile,
            self.mode,
            self.coverage,
            self.role_lease,
        )
    }
}

pub(crate) fn issue_generic_g0_recipe_demand_v1(
    selection: CanonicalLoopFamilySelectionV1,
) -> Result<VerifiedGenericRecipeDemandG0, GenericG0RecipeDemandIssueV1> {
    let (window_lease, mode, coverage, candidate) = selection.into_parts();
    let candidate = candidate
        .into_generic_g0()
        .map_err(|candidate| GenericG0RecipeDemandIssueV1::SelectedOtherFamily(candidate.tag()))?;
    let (observation, evidence) = candidate.into_parts();
    verify_evidence(&evidence, &window_lease, mode, coverage)?;

    let (handoff, policy_context) = observation.into_parts();
    verify_policy_context(&policy_context, &window_lease, mode, coverage)?;
    if !handoff.brand().matches_window(&window_lease) {
        return Err(GenericG0RecipeDemandIssueV1::LeaseBrandMismatch);
    }
    let role_lease = issue_role_lease(&handoff).map_err(GenericG0RecipeDemandIssueV1::Role)?;
    let (source_brand, source_bundle, post_loop_read, target) = handoff.into_parts();
    if source_bundle.numeric().target() != target {
        return Err(GenericG0RecipeDemandIssueV1::TargetMismatch);
    }

    Ok(VerifiedGenericRecipeDemandG0 {
        window_lease,
        source_brand,
        source_bundle,
        post_loop_read,
        profile: policy_context.profile(),
        mode,
        coverage,
        role_lease,
    })
}

fn verify_evidence(
    evidence: &GenericG0ObservationEvidenceV1,
    lease: &VerifiedLoopFamilyWindowLeaseV1,
    mode: LoopFamilyAdmissionModeV1,
    coverage: LoopFamilyAdmissionCoverageV1,
) -> Result<(), GenericG0RecipeDemandIssueV1> {
    if !identity_matches_lease(evidence.expected().identity(), lease)
        || !identity_matches_lease(evidence.observed_identity(), lease)
    {
        return Err(GenericG0RecipeDemandIssueV1::EvidenceIdentityMismatch);
    }
    let Some(expected_mode) = evidence.expected().mode() else {
        return Err(GenericG0RecipeDemandIssueV1::EvidenceModeMismatch);
    };
    let Some(observed_mode) = evidence.observed_mode() else {
        return Err(GenericG0RecipeDemandIssueV1::EvidenceModeMismatch);
    };
    if map_observation_mode(expected_mode) != mode || map_observation_mode(observed_mode) != mode {
        return Err(GenericG0RecipeDemandIssueV1::EvidenceModeMismatch);
    }
    if map_observation_coverage(evidence.expected().coverage()) != coverage
        || map_observation_coverage(evidence.observed_coverage()) != coverage
    {
        return Err(GenericG0RecipeDemandIssueV1::EvidenceCoverageMismatch);
    }
    Ok(())
}

fn verify_policy_context(
    context: &GenericG0PolicyContextV1,
    lease: &VerifiedLoopFamilyWindowLeaseV1,
    mode: LoopFamilyAdmissionModeV1,
    coverage: LoopFamilyAdmissionCoverageV1,
) -> Result<(), GenericG0RecipeDemandIssueV1> {
    if context.owner() != lease.owner() {
        return Err(GenericG0RecipeDemandIssueV1::PolicyOwnerMismatch);
    }
    if context.profile() != GenericG0PolicyProfileV1::G0 {
        return Err(GenericG0RecipeDemandIssueV1::PolicyProfileMismatch);
    }
    if map_policy_mode(context.mode()) != mode {
        return Err(GenericG0RecipeDemandIssueV1::PolicyModeMismatch);
    }
    if map_policy_coverage(context.coverage()) != coverage {
        return Err(GenericG0RecipeDemandIssueV1::PolicyCoverageMismatch);
    }
    Ok(())
}

fn identity_matches_lease(
    identity: &crate::mir::loop_structural_facts::GenericG0SourceIdentityV1,
    lease: &VerifiedLoopFamilyWindowLeaseV1,
) -> bool {
    identity.owner() == lease.owner()
        && identity.function_origin() == lease.function_origin()
        && identity.source_kind() == lease.source_kind()
        && identity.site() == lease.site()
        && identity.frame().matches(&lease.frame())
}

fn map_observation_mode(mode: GenericG0ObservationModeV1) -> LoopFamilyAdmissionModeV1 {
    match mode {
        GenericG0ObservationModeV1::Release => LoopFamilyAdmissionModeV1::Release,
        GenericG0ObservationModeV1::Strict => LoopFamilyAdmissionModeV1::Strict,
        GenericG0ObservationModeV1::StrictPlannerRequired => {
            LoopFamilyAdmissionModeV1::StrictPlannerRequired
        }
    }
}

fn map_observation_coverage(
    coverage: GenericG0ObservationCoverageV1,
) -> LoopFamilyAdmissionCoverageV1 {
    match coverage {
        GenericG0ObservationCoverageV1::Complete => LoopFamilyAdmissionCoverageV1::Complete,
        GenericG0ObservationCoverageV1::Incomplete => LoopFamilyAdmissionCoverageV1::Incomplete,
    }
}

fn map_policy_mode(mode: GenericG0PolicyModeV1) -> LoopFamilyAdmissionModeV1 {
    match mode {
        GenericG0PolicyModeV1::Release => LoopFamilyAdmissionModeV1::Release,
        GenericG0PolicyModeV1::Strict => LoopFamilyAdmissionModeV1::Strict,
        GenericG0PolicyModeV1::StrictPlannerRequired => {
            LoopFamilyAdmissionModeV1::StrictPlannerRequired
        }
    }
}

fn map_policy_coverage(coverage: GenericG0CoverageV1) -> LoopFamilyAdmissionCoverageV1 {
    match coverage {
        GenericG0CoverageV1::Complete => LoopFamilyAdmissionCoverageV1::Complete,
        GenericG0CoverageV1::Incomplete => LoopFamilyAdmissionCoverageV1::Incomplete,
    }
}

fn issue_role_lease(
    handoff: &VerifiedGenericG0PolicyHandoffV1,
) -> Result<VerifiedGenericG0RoleLeaseV1, GenericG0RoleLeaseRejectV1> {
    let structural = handoff.bundle().source().structural();
    let outer_binding = structural.outer_condition().binding;
    let inner_binding = structural.inner_condition().binding;
    if outer_binding != structural.outer_update().binding
        || inner_binding != structural.inner_update().binding
        || inner_binding != structural.tail().binding
        || outer_binding == inner_binding
    {
        return Err(GenericG0RoleLeaseRejectV1::BindingRoleMismatch);
    }
    let post_loop_read = handoff.post_loop_read();
    if post_loop_read.statement() != &structural.tail().statement
        || post_loop_read.value() != &structural.tail().value
        || post_loop_read.binding() != structural.tail().binding
    {
        return Err(GenericG0RoleLeaseRejectV1::ReturnRelationMismatch);
    }
    Ok(VerifiedGenericG0RoleLeaseV1 {
        _seal: GenericG0RoleLeaseSealV1,
    })
}
