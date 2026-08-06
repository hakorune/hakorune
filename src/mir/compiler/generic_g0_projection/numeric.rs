//! S0C adapter from the sealed S0B source bundle to the neutral numeric issuer.
//!
//! This is the only layer that knows both source-site inventory and numeric
//! substrate DTOs. It retains the original S0B bundle and adds one numeric
//! lease; no source row is reconstructed or cloned.

use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::numeric_substrate::generic_g0::{
    issue_generic_g0_numeric_fact_lease_v1, GenericG0NumericIssueV1,
    GenericG0NumericLiteralInputV1, GenericG0NumericLiteralRoleV1,
    GenericG0NumericParameterInputV1, GenericG0NumericSourceViewV1,
    VerifiedGenericNumericFactLeaseG0,
};
use crate::mir::numeric_substrate::NumericTarget;
use crate::mir::resolved_semantics::generic_g0::{
    GenericG0LiteralRoleV1, GenericG0LiteralSyntaxV1,
};

use super::VerifiedGenericSourceBundleG0;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericTypedSourceBundleG0 {
    source: VerifiedGenericSourceBundleG0,
    numeric: VerifiedGenericNumericFactLeaseG0,
    return_abi: ExactTrivialReturnAbiV1,
}

impl VerifiedGenericTypedSourceBundleG0 {
    pub(crate) fn source(&self) -> &VerifiedGenericSourceBundleG0 {
        &self.source
    }

    pub(crate) fn numeric(&self) -> &VerifiedGenericNumericFactLeaseG0 {
        &self.numeric
    }

    pub(crate) const fn return_abi(&self) -> ExactTrivialReturnAbiV1 {
        self.return_abi
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedGenericSourceBundleG0,
        VerifiedGenericNumericFactLeaseG0,
        ExactTrivialReturnAbiV1,
    ) {
        (self.source, self.numeric, self.return_abi)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0NumericProjectionRejectV1 {
    ParameterShape,
    LiteralShape,
    NonIntegerLiteral { role: GenericG0LiteralRoleV1 },
    ReturnAbi,
    Numeric(GenericG0NumericIssueV1),
}

pub(crate) fn issue_generic_g0_typed_source_bundle_v1(
    bundle: VerifiedGenericSourceBundleG0,
    target: NumericTarget,
) -> Result<VerifiedGenericTypedSourceBundleG0, GenericG0NumericProjectionRejectV1> {
    let return_type = bundle
        .source_types()
        .result()
        .declared_type_name
        .as_deref()
        .ok_or(GenericG0NumericProjectionRejectV1::ReturnAbi)?;
    let return_abi = ExactTrivialReturnAbiV1::classify(return_type)
        .ok_or(GenericG0NumericProjectionRejectV1::ReturnAbi)?;
    let view = neutral_numeric_view(&bundle, target)?;
    let numeric = issue_generic_g0_numeric_fact_lease_v1(view)
        .map_err(GenericG0NumericProjectionRejectV1::Numeric)?;
    Ok(VerifiedGenericTypedSourceBundleG0 {
        source: bundle,
        numeric,
        return_abi,
    })
}

fn neutral_numeric_view<'a>(
    bundle: &'a VerifiedGenericSourceBundleG0,
    target: NumericTarget,
) -> Result<GenericG0NumericSourceViewV1<'a>, GenericG0NumericProjectionRejectV1> {
    let source_types = bundle.source_types();
    let parameters = source_types.parameters();
    if parameters.len() != 2 {
        return Err(GenericG0NumericProjectionRejectV1::ParameterShape);
    }
    let parameter_inputs = [
        GenericG0NumericParameterInputV1 {
            index: parameters[0].index,
            declared_type_name: parameters[0].declared_type_name.as_deref(),
        },
        GenericG0NumericParameterInputV1 {
            index: parameters[1].index,
            declared_type_name: parameters[1].declared_type_name.as_deref(),
        },
    ];

    let expected_roles = [
        GenericG0LiteralRoleV1::OuterConditionRhs,
        GenericG0LiteralRoleV1::InnerConditionRhs,
        GenericG0LiteralRoleV1::OuterUpdateRhs,
        GenericG0LiteralRoleV1::InnerUpdateRhs,
    ];
    let mut literal_inputs = Vec::with_capacity(expected_roles.len());
    for role in expected_roles {
        let row = source_types
            .literals()
            .iter()
            .find(|row| row.role == role)
            .ok_or(GenericG0NumericProjectionRejectV1::LiteralShape)?;
        let (value, explicit_type_name) = match &row.syntax {
            GenericG0LiteralSyntaxV1::PlainInteger(value) => (*value as i128, None),
            GenericG0LiteralSyntaxV1::TypedInteger {
                value,
                declared_type_name,
            } => (*value as i128, Some(declared_type_name.as_ref())),
            GenericG0LiteralSyntaxV1::Other(_) => {
                return Err(GenericG0NumericProjectionRejectV1::NonIntegerLiteral { role });
            }
        };
        let contextual_parameter_index = parameters
            .iter()
            .find(|parameter| parameter.binding == row.binding)
            .map(|parameter| parameter.index);
        literal_inputs.push(GenericG0NumericLiteralInputV1 {
            role: numeric_role(role),
            value,
            explicit_type_name,
            contextual_parameter_index,
        });
    }

    Ok(GenericG0NumericSourceViewV1 {
        target: Some(target),
        parameters: vec![parameter_inputs[0], parameter_inputs[1]].into_boxed_slice(),
        literals: literal_inputs.into_boxed_slice(),
    })
}

fn numeric_role(role: GenericG0LiteralRoleV1) -> GenericG0NumericLiteralRoleV1 {
    match role {
        GenericG0LiteralRoleV1::OuterConditionRhs => {
            GenericG0NumericLiteralRoleV1::OuterConditionRhs
        }
        GenericG0LiteralRoleV1::InnerConditionRhs => {
            GenericG0NumericLiteralRoleV1::InnerConditionRhs
        }
        GenericG0LiteralRoleV1::OuterUpdateRhs => GenericG0NumericLiteralRoleV1::OuterUpdateRhs,
        GenericG0LiteralRoleV1::InnerUpdateRhs => GenericG0NumericLiteralRoleV1::InnerUpdateRhs,
    }
}
