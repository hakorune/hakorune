use super::super::direct_accum_profile::CanonicalDirectAccumPlanV1;
use super::super::function_input::ResolvedFunctionLoweringInputV1;
use super::resolved_owner_header::{
    ResolvedOwnerHeaderFamilyV1, ResolvedOwnerHeaderSealErrorV1, VerifiedResolvedOwnerHeaderV1,
};
use super::{CanonicalCurrentAPlusPlanV1, CanonicalTrivialBindingSsaPlanV1};

/// Semantic Loop-family envelope. The DirectAccum pilot is the only admitted
/// variant; other families require their own sealed source/body products.
#[derive(Debug)]
pub(crate) enum CanonicalLoopFamilyPlanV1<'a> {
    DirectAccum(CanonicalDirectAccumPlanV1<'a>),
}

impl<'a> CanonicalLoopFamilyPlanV1<'a> {
    pub(crate) fn function_input(
        &self,
    ) -> super::super::function_input::ResolvedFunctionLoweringInputV1<'a> {
        match self {
            Self::DirectAccum(plan) => plan.input(),
        }
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
