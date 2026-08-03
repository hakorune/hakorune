//! Canonical source-bound plan vocabulary and lifecycle-route mapping.
//!
//! This module owns the plan sum independently from the package that later
//! binds a plan to a compiler domain and invocation token. Keeping the sum
//! here makes the semantic body profile distinct from its lifecycle family.

use super::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1;
use super::capability::{
    CanonicalCurrentAPlusPlanV1, CanonicalFirstFamilyPlanV1, CanonicalTrivialBindingSsaPlanV1,
};
use super::direct_accum_profile::CanonicalDirectAccumPlanV1;
use super::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1;
use crate::mir::module_invocation_identity::ModuleInvocationFamilyV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum CanonicalSourceRouteV1 {
    APlus,
    BindingSsaTrivial,
    BindingSsaAcyclic,
    BindingSsaRecursive,
}

const fn family_for_route(route: CanonicalSourceRouteV1) -> ModuleInvocationFamilyV1 {
    match route {
        CanonicalSourceRouteV1::APlus => ModuleInvocationFamilyV1::CanonicalAPlus,
        CanonicalSourceRouteV1::BindingSsaTrivial => ModuleInvocationFamilyV1::BindingSsaTrivial,
        CanonicalSourceRouteV1::BindingSsaAcyclic => ModuleInvocationFamilyV1::BindingSsaAcyclic,
        CanonicalSourceRouteV1::BindingSsaRecursive => {
            ModuleInvocationFamilyV1::BindingSsaRecursive
        }
    }
}

fn route_for_family(family: ModuleInvocationFamilyV1) -> CanonicalSourceRouteV1 {
    match family {
        ModuleInvocationFamilyV1::CanonicalAPlus => CanonicalSourceRouteV1::APlus,
        ModuleInvocationFamilyV1::BindingSsaTrivial => CanonicalSourceRouteV1::BindingSsaTrivial,
        ModuleInvocationFamilyV1::BindingSsaAcyclic => CanonicalSourceRouteV1::BindingSsaAcyclic,
        ModuleInvocationFamilyV1::BindingSsaRecursive => {
            CanonicalSourceRouteV1::BindingSsaRecursive
        }
        ModuleInvocationFamilyV1::Raw => unreachable!("canonical package cannot carry Raw"),
    }
}

/// The exact canonical preflight plans accepted by the source-bound package.
/// Raw remains on the closed RAW0 chain and is intentionally not rewrapped.
#[derive(Debug)]
pub(in crate::mir) enum ExactCanonicalPreflightPlanV1<'a> {
    APlus(CanonicalCurrentAPlusPlanV1<'a>),
    BindingSsaTrivial(CanonicalTrivialBindingSsaPlanV1<'a>),
    /// Body-specialized candidate; its external lifecycle reuses
    /// `BindingSsaTrivial`.
    DirectAccum(CanonicalDirectAccumPlanV1<'a>),
    BindingSsaAcyclic(VerifiedAcyclicCallableModulePlanV1<'a>),
    BindingSsaRecursive(VerifiedRecursiveCallableModulePlanV1<'a>),
}

impl<'a> ExactCanonicalPreflightPlanV1<'a> {
    pub(crate) fn from_first_family(plan: CanonicalFirstFamilyPlanV1<'a>) -> Self {
        match plan {
            CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(plan) => Self::APlus(plan),
            CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) => Self::BindingSsaTrivial(plan),
            CanonicalFirstFamilyPlanV1::DirectAccum(plan) => Self::DirectAccum(plan),
        }
    }

    pub(crate) fn route(&self) -> CanonicalSourceRouteV1 {
        match self {
            Self::APlus(_) => CanonicalSourceRouteV1::APlus,
            Self::BindingSsaTrivial(_) => CanonicalSourceRouteV1::BindingSsaTrivial,
            Self::DirectAccum(_) => CanonicalSourceRouteV1::BindingSsaTrivial,
            Self::BindingSsaAcyclic(_) => CanonicalSourceRouteV1::BindingSsaAcyclic,
            Self::BindingSsaRecursive(_) => CanonicalSourceRouteV1::BindingSsaRecursive,
        }
    }
}

pub(crate) const fn family_for_route_v1(route: CanonicalSourceRouteV1) -> ModuleInvocationFamilyV1 {
    family_for_route(route)
}

pub(crate) fn route_for_family_v1(family: ModuleInvocationFamilyV1) -> CanonicalSourceRouteV1 {
    route_for_family(family)
}
