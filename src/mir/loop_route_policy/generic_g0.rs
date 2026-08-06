//! Generic G0 candidate policy over the sealed S0C source bundle.
//!
//! This module owns only the bounded `Less`/positive-`Add` admission rule. It
//! consumes the AST-free S0C product by value and returns one move-only
//! observation or a typed terminal disposition. It does not select a family,
//! issue Recipe keys, inspect AST, touch Builder/MIR, retry, or fallback.

use crate::mir::loop_structural_facts::generic_g0::{
    GenericG0ConditionOperatorV1, GenericG0UpdateOperatorV1, VerifiedGenericG0PolicyHandoffV1,
    VerifiedGenericTypedSourceBundleG0,
};
use crate::mir::numeric_substrate::generic_g0::GenericG0NumericLiteralRoleV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum GenericG0PolicyModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum GenericG0CoverageV1 {
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum GenericG0PolicyProfileV1 {
    G0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GenericG0PolicyContextV1 {
    owner: FunctionOwnerIdV1,
    profile: GenericG0PolicyProfileV1,
    mode: GenericG0PolicyModeV1,
    coverage: GenericG0CoverageV1,
    _seal: GenericG0PolicyContextSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GenericG0PolicyContextSealV1;

impl GenericG0PolicyContextV1 {
    pub(crate) const fn from_observation(
        owner: FunctionOwnerIdV1,
        profile: GenericG0PolicyProfileV1,
        mode: GenericG0PolicyModeV1,
        coverage: GenericG0CoverageV1,
    ) -> Self {
        Self {
            owner,
            profile,
            mode,
            coverage,
            _seal: GenericG0PolicyContextSealV1,
        }
    }

    #[cfg(test)]
    pub(crate) const fn for_test(
        owner: FunctionOwnerIdV1,
        profile: GenericG0PolicyProfileV1,
        mode: GenericG0PolicyModeV1,
        coverage: GenericG0CoverageV1,
    ) -> Self {
        Self::from_observation(owner, profile, mode, coverage)
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn mode(&self) -> GenericG0PolicyModeV1 {
        self.mode
    }

    pub(crate) const fn profile(&self) -> GenericG0PolicyProfileV1 {
        self.profile
    }

    pub(crate) const fn coverage(&self) -> GenericG0CoverageV1 {
        self.coverage
    }
}

/// The only S1 positive product. The complete S0C bundle remains inside the
/// observation so later rows cannot reconstruct source facts by name.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericFamilyObservationG0 {
    handoff: VerifiedGenericG0PolicyHandoffV1,
    context: GenericG0PolicyContextV1,
}

impl VerifiedGenericFamilyObservationG0 {
    pub(crate) fn bundle(&self) -> &VerifiedGenericTypedSourceBundleG0 {
        self.handoff.bundle()
    }

    pub(crate) fn handoff(&self) -> &VerifiedGenericG0PolicyHandoffV1 {
        &self.handoff
    }

    pub(crate) const fn context(&self) -> GenericG0PolicyContextV1 {
        self.context
    }

    pub(crate) fn into_parts(self) -> (VerifiedGenericG0PolicyHandoffV1, GenericG0PolicyContextV1) {
        (self.handoff, self.context)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0PolicyUnresolvedV1 {
    IncompleteCoverage,
    UnsupportedComparison,
    UnsupportedUpdate,
    NonProgressingStep { role: GenericG0NumericLiteralRoleV1 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0PolicyRejectV1 {
    ForeignContext,
    DirectionMismatch,
    MissingLiteralRole { role: GenericG0NumericLiteralRoleV1 },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0PolicyOutcomeV1 {
    Candidate(VerifiedGenericFamilyObservationG0),
    Unresolved(GenericG0PolicyUnresolvedV1),
    Rejected(GenericG0PolicyRejectV1),
}

pub(crate) fn issue_generic_g0_candidate_v1(
    handoff: VerifiedGenericG0PolicyHandoffV1,
    context: GenericG0PolicyContextV1,
) -> GenericG0PolicyOutcomeV1 {
    let bundle = handoff.bundle();
    if bundle.source().structural().owner() != context.owner() {
        return GenericG0PolicyOutcomeV1::Rejected(GenericG0PolicyRejectV1::ForeignContext);
    }
    if context.coverage() == GenericG0CoverageV1::Incomplete {
        return GenericG0PolicyOutcomeV1::Unresolved(
            GenericG0PolicyUnresolvedV1::IncompleteCoverage,
        );
    }

    let structural = bundle.source().structural();
    if unsupported_condition(structural.outer_condition().operator())
        || unsupported_condition(structural.inner_condition().operator())
    {
        return GenericG0PolicyOutcomeV1::Unresolved(
            GenericG0PolicyUnresolvedV1::UnsupportedComparison,
        );
    }
    if structural.outer_condition().operator() != GenericG0ConditionOperatorV1::Less
        || structural.inner_condition().operator() != GenericG0ConditionOperatorV1::Less
    {
        return GenericG0PolicyOutcomeV1::Rejected(GenericG0PolicyRejectV1::DirectionMismatch);
    }

    if structural.outer_update().operator() == GenericG0UpdateOperatorV1::Other
        || structural.inner_update().operator() == GenericG0UpdateOperatorV1::Other
    {
        return GenericG0PolicyOutcomeV1::Unresolved(
            GenericG0PolicyUnresolvedV1::UnsupportedUpdate,
        );
    }
    if structural.outer_update().operator() != GenericG0UpdateOperatorV1::Add
        || structural.inner_update().operator() != GenericG0UpdateOperatorV1::Add
    {
        return GenericG0PolicyOutcomeV1::Rejected(GenericG0PolicyRejectV1::DirectionMismatch);
    }

    for role in [
        GenericG0NumericLiteralRoleV1::OuterUpdateRhs,
        GenericG0NumericLiteralRoleV1::InnerUpdateRhs,
    ] {
        let Some(value) = literal_value(bundle.numeric(), role) else {
            return GenericG0PolicyOutcomeV1::Rejected(
                GenericG0PolicyRejectV1::MissingLiteralRole { role },
            );
        };
        if value == 0 {
            return GenericG0PolicyOutcomeV1::Unresolved(
                GenericG0PolicyUnresolvedV1::NonProgressingStep { role },
            );
        }
        if value < 0 {
            return GenericG0PolicyOutcomeV1::Rejected(GenericG0PolicyRejectV1::DirectionMismatch);
        }
    }

    GenericG0PolicyOutcomeV1::Candidate(VerifiedGenericFamilyObservationG0 { handoff, context })
}

fn unsupported_condition(operator: GenericG0ConditionOperatorV1) -> bool {
    matches!(
        operator,
        GenericG0ConditionOperatorV1::LessEqual
            | GenericG0ConditionOperatorV1::Equal
            | GenericG0ConditionOperatorV1::NotEqual
            | GenericG0ConditionOperatorV1::Other
    )
}

fn literal_value(
    numeric: &crate::mir::numeric_substrate::generic_g0::VerifiedGenericNumericFactLeaseG0,
    role: GenericG0NumericLiteralRoleV1,
) -> Option<i128> {
    numeric
        .literals()
        .iter()
        .find(|literal| literal.role == role)
        .map(|literal| literal.value)
}
