//! Production seam for the first verified If recipe.
//!
//! This adapter owns admission only.  The canonical session remains the
//! physical CFG/SSA/PHI owner; this module must not allocate blocks, emit MIR,
//! select routes, or retry a different lowering path.

use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::if_recipe_contract::{
    IfPhysicalInputRejectReasonV1, IfRecipeSourceOwnerV1, IfSourcePathStepV1,
    VerifiedIfPhysicalInputV1,
};
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_semantics::{
    SourcePathSegmentV1, SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};
use crate::mir::resolved_value_profile::{
    map_trivial_if_recipe_v1, product::VerifiedTrivialCanonicalOwnerV1, IfRecipeMapRejectV1,
};

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalIfRecipePreflightV1 {
    NotThisShape,
    Selected(VerifiedIfPhysicalInputV1),
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalIfRecipeProducerRejectV1 {
    Mapper(IfRecipeMapRejectV1),
    PhysicalInput(IfPhysicalInputRejectReasonV1),
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalIfRecipeAdmissionRejectV1 {
    SourceOwnerMismatch,
    MissingIfClaim,
    InvalidIfClaimPath,
    IfControlCardinality { found: usize },
    IfControlSiteMismatch,
    SelectedIfNotConsumed,
    SelectedIfConsumedTwice,
    UnexpectedIfSite,
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalIfRecipeAdmissionDispositionV1 {
    NotSelected,
    Selected(CanonicalIfRecipeAdmissionV1),
}

impl CanonicalIfRecipeAdmissionDispositionV1 {
    pub(in crate::mir::builder::resolved_lowering) fn claim_if(
        &mut self,
        statement: &LocatedStmtV1<'_>,
    ) -> Result<(), CanonicalIfRecipeAdmissionRejectV1> {
        match self {
            Self::NotSelected => Ok(()),
            Self::Selected(admission) => admission.claim_if(statement),
        }
    }

    pub(in crate::mir::builder::resolved_lowering) fn finish(
        self,
    ) -> Result<(), CanonicalIfRecipeAdmissionRejectV1> {
        match self {
            Self::NotSelected => Ok(()),
            Self::Selected(admission) => admission.finish(),
        }
    }
}

#[derive(Debug)]
pub(in crate::mir::builder::resolved_lowering) struct CanonicalIfRecipeAdmissionV1 {
    expected_site: SourceStmtSiteV1,
    state: CanonicalIfRecipeAdmissionStateV1,
}

#[derive(Debug)]
enum CanonicalIfRecipeAdmissionStateV1 {
    Pending(VerifiedIfPhysicalInputV1),
    Consumed,
}

pub(in crate::mir::builder::resolved_lowering) fn produce_trivial_if_physical_input_v1(
    profile: &VerifiedTrivialCanonicalOwnerV1,
    source_function: &VerifiedResolvedFunctionV1,
) -> Result<CanonicalIfRecipePreflightV1, CanonicalIfRecipeProducerRejectV1> {
    if profile.recipe_facts().is_none() {
        return Ok(CanonicalIfRecipePreflightV1::NotThisShape);
    }
    let artifact = map_trivial_if_recipe_v1(profile, source_function)
        .map_err(CanonicalIfRecipeProducerRejectV1::Mapper)?;
    let physical_input = VerifiedIfPhysicalInputV1::from_artifact(artifact)
        .map_err(CanonicalIfRecipeProducerRejectV1::PhysicalInput)?;
    Ok(CanonicalIfRecipePreflightV1::Selected(physical_input))
}

pub(in crate::mir::builder::resolved_lowering) fn admit_trivial_if_recipe_v1(
    preflight: CanonicalIfRecipePreflightV1,
    source_function: &VerifiedResolvedFunctionV1,
    if_control: &VerifiedResolvedFunctionIfControlV1,
) -> Result<CanonicalIfRecipeAdmissionDispositionV1, CanonicalIfRecipeAdmissionRejectV1> {
    let CanonicalIfRecipePreflightV1::Selected(physical_input) = preflight else {
        return Ok(CanonicalIfRecipeAdmissionDispositionV1::NotSelected);
    };
    let source_binding = physical_input
        .artifact()
        .source_binding()
        .as_source_binding();
    if !source_owner_matches(source_binding.owner, source_function) {
        return Err(CanonicalIfRecipeAdmissionRejectV1::SourceOwnerMismatch);
    }
    let root_index = source_binding
        .claims
        .first()
        .and_then(|claim| claim.path.steps.first())
        .and_then(|step| match step {
            IfSourcePathStepV1::BodyItem { index } => Some(*index),
            _ => None,
        })
        .ok_or(CanonicalIfRecipeAdmissionRejectV1::MissingIfClaim)?;
    if source_binding
        .claims
        .first()
        .map(|claim| claim.path.steps.len())
        != Some(1)
    {
        return Err(CanonicalIfRecipeAdmissionRejectV1::InvalidIfClaimPath);
    }

    let mut sites = if_control.exact_if_sites();
    let expected_site = sites
        .next()
        .cloned()
        .ok_or(CanonicalIfRecipeAdmissionRejectV1::IfControlCardinality { found: 0 })?;
    if sites.next().is_some() {
        return Err(CanonicalIfRecipeAdmissionRejectV1::IfControlCardinality {
            found: if_control.row_count(),
        });
    }
    if !matches!(
        expected_site.node().segments(),
        [SourcePathSegmentV1::Body(index)] if *index == root_index
    ) {
        return Err(CanonicalIfRecipeAdmissionRejectV1::IfControlSiteMismatch);
    }
    Ok(CanonicalIfRecipeAdmissionDispositionV1::Selected(
        CanonicalIfRecipeAdmissionV1 {
            expected_site,
            state: CanonicalIfRecipeAdmissionStateV1::Pending(physical_input),
        },
    ))
}

impl CanonicalIfRecipeAdmissionV1 {
    pub(in crate::mir::builder::resolved_lowering) fn claim_if(
        &mut self,
        statement: &LocatedStmtV1<'_>,
    ) -> Result<(), CanonicalIfRecipeAdmissionRejectV1> {
        if statement.site() != &self.expected_site {
            return Err(CanonicalIfRecipeAdmissionRejectV1::UnexpectedIfSite);
        }
        let state = std::mem::replace(&mut self.state, CanonicalIfRecipeAdmissionStateV1::Consumed);
        match state {
            CanonicalIfRecipeAdmissionStateV1::Pending(_physical_input) => Ok(()),
            CanonicalIfRecipeAdmissionStateV1::Consumed => {
                Err(CanonicalIfRecipeAdmissionRejectV1::SelectedIfConsumedTwice)
            }
        }
    }

    pub(in crate::mir::builder::resolved_lowering) fn finish(
        self,
    ) -> Result<(), CanonicalIfRecipeAdmissionRejectV1> {
        match self.state {
            CanonicalIfRecipeAdmissionStateV1::Consumed => Ok(()),
            CanonicalIfRecipeAdmissionStateV1::Pending(_) => {
                Err(CanonicalIfRecipeAdmissionRejectV1::SelectedIfNotConsumed)
            }
        }
    }
}

fn source_owner_matches(
    owner: IfRecipeSourceOwnerV1,
    source_function: &VerifiedResolvedFunctionV1,
) -> bool {
    let origin = source_function.function_origin();
    matches!(
        owner,
        IfRecipeSourceOwnerV1::FunctionBody {
            compilation_unit_ordinal,
            function_ordinal,
        } if compilation_unit_ordinal == origin.compilation_unit_ordinal()
            && function_ordinal == origin.function_ordinal()
    )
}
