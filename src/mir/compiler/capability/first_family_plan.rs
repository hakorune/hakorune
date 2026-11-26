#[cfg(test)]
use super::super::function_input::ResolvedFunctionLoweringInputV1;
#[cfg(test)]
use super::resolved_owner_header::{
    ResolvedOwnerHeaderFamilyV1, ResolvedOwnerHeaderSealErrorV1, VerifiedResolvedOwnerHeaderV1,
};
use super::{CanonicalCurrentAPlusPlanV1, CanonicalTrivialBindingSsaPlanV1};

/// One whole-unit canonical value-authority selection.
///
/// The variant is sealed before the module candidate is opened. A later
/// lowering failure cannot be reclassified as the temporary A+ route.
#[derive(Debug)]
pub(crate) enum CanonicalFirstFamilyPlanV1<'a> {
    TrivialBindingSsa(CanonicalTrivialBindingSsaPlanV1<'a>),
    CurrentCanonicalAPlus(CanonicalCurrentAPlusPlanV1<'a>),
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalFirstFamilyPlanBrandV1(ResolvedOwnerHeaderFamilyV1);

#[cfg(test)]
impl CanonicalFirstFamilyPlanBrandV1 {
    pub(in crate::mir::compiler) const fn from_family(family: ResolvedOwnerHeaderFamilyV1) -> Self {
        Self(family)
    }

    pub(in crate::mir::compiler) const fn family(self) -> ResolvedOwnerHeaderFamilyV1 {
        self.0
    }
}

#[cfg(test)]
impl<'a> CanonicalFirstFamilyPlanV1<'a> {
    pub(in crate::mir::compiler) fn brand(&self) -> CanonicalFirstFamilyPlanBrandV1 {
        let family = match self {
            Self::TrivialBindingSsa(_) => {
                ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa
            }
            Self::CurrentCanonicalAPlus(_) => ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus,
        };
        CanonicalFirstFamilyPlanBrandV1::from_family(family)
    }

    pub(in crate::mir::compiler) fn function_input(&self) -> ResolvedFunctionLoweringInputV1<'a> {
        match self {
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
