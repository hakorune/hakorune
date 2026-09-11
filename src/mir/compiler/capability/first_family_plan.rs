use super::super::direct_accum_profile::CanonicalDirectAccumPlanV1;
use super::super::function_input::ResolvedFunctionLoweringInputV1;
use super::super::nested_predicate_profile::CanonicalNestedPredicatePlanV1;
use super::resolved_owner_header::{
    ResolvedOwnerHeaderFamilyV1, ResolvedOwnerHeaderSealErrorV1, VerifiedResolvedOwnerHeaderV1,
};
use super::{CanonicalCurrentAPlusPlanV1, CanonicalTrivialBindingSsaPlanV1};
use crate::mir::compiler::generic_g0_source_parent::VerifiedGenericG0SourceParentV1;

/// Semantic Loop-family envelope. Each variant carries one sealed
/// source/body product; the external lifecycle remains BindingSsaTrivial.
#[derive(Debug)]
pub(crate) enum CanonicalLoopFamilyPlanV1<'a> {
    DirectAccum(CanonicalDirectAccumPlanV1<'a>),
    NestedPredicate(CanonicalNestedPredicatePlanV1<'a>),
    GenericG0(CanonicalGenericG0PlanV1<'a>),
}

impl<'a> CanonicalLoopFamilyPlanV1<'a> {
    pub(crate) fn function_input(
        &self,
    ) -> super::super::function_input::ResolvedFunctionLoweringInputV1<'a> {
        match self {
            Self::DirectAccum(plan) => plan.input(),
            Self::NestedPredicate(plan) => plan.input(),
            Self::GenericG0(plan) => plan.source_input(),
        }
    }
}

/// Source-backed Generic G0 capability carried by the existing Loop envelope.
/// The lifecycle family remains `BindingSsaTrivial`; no new physical route is
/// introduced by this semantic plan.
#[derive(Debug)]
pub(crate) struct CanonicalGenericG0PlanV1<'a> {
    source_parent: VerifiedGenericG0SourceParentV1<'a>,
}

impl<'a> CanonicalGenericG0PlanV1<'a> {
    pub(crate) fn new(source_parent: VerifiedGenericG0SourceParentV1<'a>) -> Self {
        Self { source_parent }
    }

    pub(crate) fn source_input(&self) -> ResolvedFunctionLoweringInputV1<'a> {
        self.source_parent.source_input()
    }

    pub(crate) fn source_parent(&self) -> &VerifiedGenericG0SourceParentV1<'a> {
        &self.source_parent
    }

    pub(crate) fn into_source_parent(self) -> VerifiedGenericG0SourceParentV1<'a> {
        self.source_parent
    }

    pub(crate) fn physical_callable_lane_count(&self) -> u32 {
        self.source_parent
            .storage_lane()
            .physical_callable_lane_count()
    }

    pub(crate) fn seal_resolved_owner_header_v1(
        &self,
    ) -> Result<VerifiedResolvedOwnerHeaderV1, ResolvedOwnerHeaderSealErrorV1> {
        VerifiedResolvedOwnerHeaderV1::seal_input(
            CanonicalFirstFamilyPlanBrandV1::from_family(
                ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa,
            ),
            self.source_input(),
        )
    }
}

/// One whole-unit canonical value-authority selection.
///
/// The variant is sealed before the module candidate is opened. A later
/// lowering failure cannot be reclassified as the temporary A+ route.
#[derive(Debug)]
pub(crate) enum CanonicalFirstFamilyPlanV1<'a> {
    /// A whole-function Loop profile. Its external header brand remains the
    /// existing TrivialBindingSsa contract; this is not a Trivial body.
    Loop(CanonicalLoopFamilyPlanV1<'a>),
    TrivialBindingSsa(CanonicalTrivialBindingSsaPlanV1<'a>),
    CurrentCanonicalAPlus(CanonicalCurrentAPlusPlanV1<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalFirstFamilyPlanBrandV1(ResolvedOwnerHeaderFamilyV1);

impl CanonicalFirstFamilyPlanBrandV1 {
    pub(in crate::mir::compiler) const fn from_family(family: ResolvedOwnerHeaderFamilyV1) -> Self {
        Self(family)
    }

    pub(in crate::mir::compiler) const fn family(self) -> ResolvedOwnerHeaderFamilyV1 {
        self.0
    }
}

impl<'a> CanonicalFirstFamilyPlanV1<'a> {
    pub(in crate::mir::compiler) fn brand(&self) -> CanonicalFirstFamilyPlanBrandV1 {
        let family = match self {
            Self::Loop(_) | Self::TrivialBindingSsa(_) => {
                ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa
            }
            Self::CurrentCanonicalAPlus(_) => ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus,
        };
        CanonicalFirstFamilyPlanBrandV1::from_family(family)
    }

    pub(in crate::mir::compiler) fn function_input(&self) -> ResolvedFunctionLoweringInputV1<'a> {
        match self {
            Self::Loop(plan) => plan.function_input(),
            Self::TrivialBindingSsa(plan) => plan.function,
            Self::CurrentCanonicalAPlus(plan) => plan.function,
        }
    }

    pub(crate) fn seal_resolved_owner_header_v1(
        &self,
    ) -> Result<VerifiedResolvedOwnerHeaderV1, ResolvedOwnerHeaderSealErrorV1> {
        VerifiedResolvedOwnerHeaderV1::seal(self.brand(), self)
    }
}

/// DirectAccum reuses the closed resolved-owner header contract without
/// pretending that its Loop body is the Trivial profile.
pub(crate) fn seal_direct_accum_owner_header_v1(
    plan: &CanonicalDirectAccumPlanV1<'_>,
) -> Result<VerifiedResolvedOwnerHeaderV1, ResolvedOwnerHeaderSealErrorV1> {
    VerifiedResolvedOwnerHeaderV1::seal_input(
        CanonicalFirstFamilyPlanBrandV1::from_family(
            ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa,
        ),
        plan.input(),
    )
}
