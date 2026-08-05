//! Neutral positive admission for direct EnumMatch scrutinee demand.
//!
//! The declaration-facts owner proves only that the existing direct enum-match
//! lowering has one supported route.  Script semantics receives neither enum
//! diagnostics nor a copied declaration inventory.

use crate::ast::{ASTNode, EnumMatchArm, EnumVariantDecl, LiteralValue};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum EnumMatchAdmissionV1 {
    PayloadProjection {
        variant_name: Box<str>,
        tag: u32,
        declared_payload_type_name: Option<Box<str>>,
    },
    BoolSelect {
        specs: Box<[(u32, bool)]>,
    },
}

/// Shared positive direct-route preflight.
///
/// `None` deliberately has no diagnostic meaning. It leaves the entire
/// operation to the existing raw owner, before any Script child descent.
pub(crate) fn admit_direct_enum_match_v1(
    type_parameters: &[String],
    variants: &[EnumVariantDecl],
    arms: &[EnumMatchArm],
    else_expr: Option<&ASTNode>,
) -> Option<EnumMatchAdmissionV1> {
    if !type_parameters.is_empty()
        || else_expr.is_some()
        || arms.is_empty()
        || arms.len() != variants.len()
    {
        return None;
    }
    let observed = arms
        .iter()
        .map(|arm| {
            let variant = variants
                .iter()
                .position(|variant| variant.name == arm.variant_name)?;
            Some((variant, arm))
        })
        .collect::<Option<Vec<_>>>()?;
    if observed
        .iter()
        .any(|(index, _)| observed.iter().filter(|(other, _)| other == index).count() != 1)
    {
        return None;
    }

    let projections = observed
        .iter()
        .enumerate()
        .filter(|(_, (_, arm))| {
            matches!(
                (&arm.binding_name, &arm.body),
                (Some(binding), ASTNode::Variable { name, .. }) if binding == name
            )
        })
        .collect::<Vec<_>>();
    let all_projection_or_null = observed.iter().all(|(_, arm)| {
        matches!(
            (&arm.binding_name, &arm.body),
            (Some(binding), ASTNode::Variable { name, .. }) if binding == name
        ) || (arm.binding_name.is_none()
            && matches!(
                arm.body,
                ASTNode::Literal {
                    value: LiteralValue::Null,
                    ..
                }
            ))
    });
    if all_projection_or_null && projections.len() == 1 {
        let (arm_index, (variant_index, arm)) = projections[0];
        let variant = variants.get(*variant_index)?;
        if variant.has_payload() && !variant.requires_compat_payload_box() {
            return Some(EnumMatchAdmissionV1::PayloadProjection {
                variant_name: arm.variant_name.clone().into(),
                tag: u32::try_from(*variant_index).ok()?,
                declared_payload_type_name: variant.payload_type_name.clone().map(Into::into),
            });
        }
        let _ = arm_index;
        return None;
    }

    let specs = observed
        .iter()
        .map(|(variant_index, arm)| {
            if arm.binding_name.is_some() {
                return None;
            }
            let ASTNode::Literal {
                value: LiteralValue::Bool(value),
                ..
            } = &arm.body
            else {
                return None;
            };
            Some((u32::try_from(*variant_index).ok()?, *value))
        })
        .collect::<Option<Vec<_>>>()?;
    Some(EnumMatchAdmissionV1::BoolSelect {
        specs: specs.into(),
    })
}

/// Source-only positive proof supplied by Program declaration facts.
pub(crate) trait EnumMatchDemandV1 {
    fn admit_direct_enum_match(
        &self,
        enum_name: &str,
        arms: &[EnumMatchArm],
        else_expr: Option<&ASTNode>,
    ) -> Option<EnumMatchAdmissionV1>;
}

impl EnumMatchDemandV1 for () {
    fn admit_direct_enum_match(
        &self,
        _enum_name: &str,
        _arms: &[EnumMatchArm],
        _else_expr: Option<&ASTNode>,
    ) -> Option<EnumMatchAdmissionV1> {
        None
    }
}
