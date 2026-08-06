//! Neutral numeric issuer for Generic G0 S0C.
//!
//! This module accepts only AST-free scalar inputs. The compiler projection
//! owns the adapter from source sites; this issuer owns exact target,
//! signedness, width, and literal range decisions. It never imports upper
//! compiler layers, resolver products, lowering products, or policy types.

use std::collections::BTreeSet;

use super::target::{classify_numeric_kind_for_target, NumericKind, NumericTarget};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum GenericG0NumericLiteralRoleV1 {
    OuterConditionRhs,
    InnerConditionRhs,
    OuterUpdateRhs,
    InnerUpdateRhs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GenericG0NumericParameterInputV1<'a> {
    pub(crate) index: u32,
    pub(crate) declared_type_name: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GenericG0NumericLiteralInputV1<'a> {
    pub(crate) role: GenericG0NumericLiteralRoleV1,
    pub(crate) value: i128,
    pub(crate) explicit_type_name: Option<&'a str>,
    pub(crate) contextual_parameter_index: Option<u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0NumericSourceViewV1<'a> {
    pub(crate) target: Option<NumericTarget>,
    pub(crate) parameters: Box<[GenericG0NumericParameterInputV1<'a>]>,
    pub(crate) literals: Box<[GenericG0NumericLiteralInputV1<'a>]>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0NumericParameterFactV1 {
    pub(crate) index: u32,
    pub(crate) kind: NumericKind,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0NumericLiteralFactV1 {
    pub(crate) role: GenericG0NumericLiteralRoleV1,
    pub(crate) kind: NumericKind,
    pub(crate) value: i128,
}

/// Move-only numeric S0C lease. Source provenance remains in the S0B bundle.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericNumericFactLeaseG0 {
    target: NumericTarget,
    parameters: Box<[GenericG0NumericParameterFactV1]>,
    literals: Box<[GenericG0NumericLiteralFactV1]>,
}

impl VerifiedGenericNumericFactLeaseG0 {
    pub(crate) const fn target(&self) -> NumericTarget {
        self.target
    }

    pub(crate) fn parameters(&self) -> &[GenericG0NumericParameterFactV1] {
        &self.parameters
    }

    pub(crate) fn literals(&self) -> &[GenericG0NumericLiteralFactV1] {
        &self.literals
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0NumericIssueV1 {
    Unresolved(GenericG0NumericUnresolvedV1),
    Rejected(GenericG0NumericRejectV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0NumericUnresolvedV1 {
    UnknownTarget,
    ParameterCardinality,
    MissingParameterType {
        index: u32,
    },
    UnknownParameterType {
        index: u32,
        type_name: Box<str>,
    },
    MissingLiteralContext {
        role: GenericG0NumericLiteralRoleV1,
    },
    MissingContextParameter {
        role: GenericG0NumericLiteralRoleV1,
        index: u32,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0NumericRejectV1 {
    ParameterIndex {
        expected: u32,
        actual: u32,
    },
    ParameterTypeMismatch {
        index: u32,
        type_name: Box<str>,
    },
    LiteralCardinality,
    DuplicateLiteralRole {
        role: GenericG0NumericLiteralRoleV1,
    },
    TypedLiteralOutOfProfile {
        role: GenericG0NumericLiteralRoleV1,
    },
    LiteralOutOfRange {
        role: GenericG0NumericLiteralRoleV1,
        value: i128,
        min: i128,
        max: i128,
    },
}

pub(crate) fn issue_generic_g0_numeric_fact_lease_v1(
    view: GenericG0NumericSourceViewV1<'_>,
) -> Result<VerifiedGenericNumericFactLeaseG0, GenericG0NumericIssueV1> {
    let target = view.target.ok_or(GenericG0NumericIssueV1::Unresolved(
        GenericG0NumericUnresolvedV1::UnknownTarget,
    ))?;
    if view.parameters.len() != 2 {
        return Err(GenericG0NumericIssueV1::Unresolved(
            GenericG0NumericUnresolvedV1::ParameterCardinality,
        ));
    }

    let mut parameters = Vec::with_capacity(view.parameters.len());
    for (expected, input) in view.parameters.iter().enumerate() {
        let expected = expected as u32;
        if input.index != expected {
            return Err(GenericG0NumericIssueV1::Rejected(
                GenericG0NumericRejectV1::ParameterIndex {
                    expected,
                    actual: input.index,
                },
            ));
        }
        let Some(type_name) = input.declared_type_name else {
            return Err(GenericG0NumericIssueV1::Unresolved(
                GenericG0NumericUnresolvedV1::MissingParameterType { index: expected },
            ));
        };
        let Some(kind) = classify_numeric_kind_for_target(type_name, target) else {
            return Err(GenericG0NumericIssueV1::Unresolved(
                GenericG0NumericUnresolvedV1::UnknownParameterType {
                    index: expected,
                    type_name: type_name.into(),
                },
            ));
        };
        if type_name != "i64" {
            return Err(GenericG0NumericIssueV1::Rejected(
                GenericG0NumericRejectV1::ParameterTypeMismatch {
                    index: expected,
                    type_name: type_name.into(),
                },
            ));
        }
        parameters.push(GenericG0NumericParameterFactV1 {
            index: expected,
            kind,
        });
    }

    if view.literals.len() != 4 {
        return Err(GenericG0NumericIssueV1::Rejected(
            GenericG0NumericRejectV1::LiteralCardinality,
        ));
    }
    let mut roles = BTreeSet::new();
    let mut literals = Vec::with_capacity(view.literals.len());
    for input in view.literals {
        if !roles.insert(input.role) {
            return Err(GenericG0NumericIssueV1::Rejected(
                GenericG0NumericRejectV1::DuplicateLiteralRole { role: input.role },
            ));
        }
        let Some(parameter_index) = input.contextual_parameter_index else {
            return Err(GenericG0NumericIssueV1::Unresolved(
                GenericG0NumericUnresolvedV1::MissingLiteralContext { role: input.role },
            ));
        };
        let Some(parameter) = parameters.iter().find(|row| row.index == parameter_index) else {
            return Err(GenericG0NumericIssueV1::Unresolved(
                GenericG0NumericUnresolvedV1::MissingContextParameter {
                    role: input.role,
                    index: parameter_index,
                },
            ));
        };
        if input.explicit_type_name.is_some() {
            return Err(GenericG0NumericIssueV1::Rejected(
                GenericG0NumericRejectV1::TypedLiteralOutOfProfile { role: input.role },
            ));
        }
        let range = parameter.kind.value_range();
        if input.value < range.min || input.value > range.max {
            return Err(GenericG0NumericIssueV1::Rejected(
                GenericG0NumericRejectV1::LiteralOutOfRange {
                    role: input.role,
                    value: input.value,
                    min: range.min,
                    max: range.max,
                },
            ));
        }
        literals.push(GenericG0NumericLiteralFactV1 {
            role: input.role,
            kind: parameter.kind,
            value: input.value,
        });
    }

    Ok(VerifiedGenericNumericFactLeaseG0 {
        target,
        parameters: parameters.into_boxed_slice(),
        literals: literals.into_boxed_slice(),
    })
}

#[cfg(test)]
mod tests;
