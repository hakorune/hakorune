use std::collections::BTreeSet;

use super::expression_proof::I64ExpressionFactV1;
use super::requirements::{union_requirements, RequirementSetV1};
use super::CallableResultUnavailableReasonV1;

/// Substitutes a callee's required parameter ordinals through one source call.
///
/// Every argument is proved by the caller before this pure operation runs.
/// Only ordinals required by the callee affect the result representation;
/// unused dynamic arguments do not create a second call-admission policy.
pub(super) fn substitute_required_arguments(
    required_ordinals: &[u32],
    arguments: &[I64ExpressionFactV1],
) -> I64ExpressionFactV1 {
    let mut substituted = BTreeSet::new();
    let mut pending = false;
    for ordinal in required_ordinals {
        let Some(argument) = arguments.get(*ordinal as usize) else {
            return unavailable();
        };
        match argument {
            I64ExpressionFactV1::Exact(requirements) => {
                substituted = union_requirements(&substituted, requirements);
            }
            I64ExpressionFactV1::PendingDependency => pending = true,
            I64ExpressionFactV1::KnownNonI64
            | I64ExpressionFactV1::Unknown(_)
            | I64ExpressionFactV1::Conflict => return unavailable(),
        }
    }
    if pending {
        I64ExpressionFactV1::PendingDependency
    } else {
        I64ExpressionFactV1::Exact(substituted)
    }
}

pub(super) fn exact_requirements(fact: &I64ExpressionFactV1) -> Option<&RequirementSetV1> {
    match fact {
        I64ExpressionFactV1::Exact(requirements) => Some(requirements),
        I64ExpressionFactV1::KnownNonI64
        | I64ExpressionFactV1::Unknown(_)
        | I64ExpressionFactV1::PendingDependency
        | I64ExpressionFactV1::Conflict => None,
    }
}

fn unavailable() -> I64ExpressionFactV1 {
    I64ExpressionFactV1::Unknown(
        CallableResultUnavailableReasonV1::RequiredArgumentRepresentationUnavailable,
    )
}
