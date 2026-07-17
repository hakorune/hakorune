//! Disconnected CorePlan Add-result representation decision.
//!
//! The decision observes only already-published operand representations. It
//! does not inspect source names, routes, recipes, runtime values, or Builder
//! state, and it does not allocate or publish a destination.

use crate::mir::MirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CorePlanAddOperandClassV1 {
    String,
    Float,
    Other,
}

/// Single-use result prepared before a CorePlan Add destination is allocated.
///
/// This product intentionally does not implement `Clone`: the eventual
/// production consumer must use one decision for one destination allocation.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct PreparedCorePlanAddResultRepresentationV1 {
    exact_type: MirType,
}

impl PreparedCorePlanAddResultRepresentationV1 {
    pub(in crate::mir::builder) fn exact_type(&self) -> &MirType {
        &self.exact_type
    }
}

fn classify_operand(ty: Option<&MirType>) -> CorePlanAddOperandClassV1 {
    match ty {
        Some(MirType::String) => CorePlanAddOperandClassV1::String,
        Some(MirType::Box(name)) if name == "StringBox" => CorePlanAddOperandClassV1::String,
        Some(MirType::Float) => CorePlanAddOperandClassV1::Float,
        _ => CorePlanAddOperandClassV1::Other,
    }
}

/// Prepares the exact first-row representation for one CorePlan Add result.
///
/// String has priority because the existing runtime result is String whenever
/// either operand is String. Otherwise this preserves the current normalizer
/// behavior: Float if either operand is Float, and Integer for every remaining
/// pair, including missing and `Unknown` facts.
pub(in crate::mir::builder) fn prepare_coreplan_add_result_representation_v1(
    lhs_type: Option<&MirType>,
    rhs_type: Option<&MirType>,
) -> PreparedCorePlanAddResultRepresentationV1 {
    use CorePlanAddOperandClassV1::{Float, Other, String};

    let exact_type = match (classify_operand(lhs_type), classify_operand(rhs_type)) {
        (String, _) | (_, String) => MirType::String,
        (Float, _) | (_, Float) => MirType::Float,
        (Other, Other) => MirType::Integer,
    };

    PreparedCorePlanAddResultRepresentationV1 { exact_type }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepared(lhs: Option<&MirType>, rhs: Option<&MirType>) -> MirType {
        prepare_coreplan_add_result_representation_v1(lhs, rhs)
            .exact_type()
            .clone()
    }

    #[test]
    fn coreplan_add_result_exact_string_or_string_box_has_priority() {
        let string_box = MirType::Box("StringBox".to_string());
        let other_box = MirType::Box("ArrayBox".to_string());

        for (lhs, rhs) in [
            (Some(&MirType::String), Some(&MirType::String)),
            (Some(&MirType::String), Some(&MirType::Unknown)),
            (Some(&MirType::Unknown), Some(&MirType::String)),
            (Some(&string_box), Some(&other_box)),
            (Some(&other_box), Some(&string_box)),
            (Some(&MirType::String), Some(&MirType::Float)),
            (Some(&MirType::Float), Some(&MirType::String)),
            (Some(&MirType::String), None),
            (None, Some(&MirType::String)),
        ] {
            assert_eq!(prepared(lhs, rhs), MirType::String);
        }
    }

    #[test]
    fn coreplan_add_result_non_string_pairs_preserve_float_else_integer_behavior() {
        for (lhs, rhs, expected) in [
            (
                Some(&MirType::Float),
                Some(&MirType::Integer),
                MirType::Float,
            ),
            (
                Some(&MirType::Integer),
                Some(&MirType::Float),
                MirType::Float,
            ),
            (
                Some(&MirType::Integer),
                Some(&MirType::Integer),
                MirType::Integer,
            ),
            (
                Some(&MirType::Unknown),
                Some(&MirType::Unknown),
                MirType::Integer,
            ),
            (None, None, MirType::Integer),
        ] {
            assert_eq!(prepared(lhs, rhs), expected);
        }
    }
}
