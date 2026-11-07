use super::CallableResultCatalogErrorV1;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableResultUnavailableReasonV1 {
    DeclaredNonI64Result,
    /// A declared result type that the body's own return proof does not produce.
    DeclaredResultDisagrees,
    NoValueReturn,
    MissingReturn,
    KnownNonI64Return,
    ConflictingReturnRepresentations,
    UnknownExpression,
    UnsupportedExpressionKind,
    UnsupportedStatementKind,
    UnsupportedAssignmentTarget,
    UnboundLocal,
    DuplicateLocal,
    StaticCallTargetAuthorityUnavailable,
    StaticCallResultUnavailable,
    RequiredArgumentRepresentationUnavailable,
    CoreMethodResultUnavailable,
    RecursiveDependency,
    LoopInvariantUnavailable,
    NestedLoopUnsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VerifiedCallableResultRepresentationV1 {
    ExactI64,
    ExactString,
    ExactBool,
    ExactNominalBox { box_name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VerifiedCallableResultDispositionV1 {
    ExactI64 { required_i64_arguments: Box<[u32]> },
    ExactString,
    ExactBool { required_i64_arguments: Box<[u32]> },
    ExactNominalBox { box_name: String, required_i64_arguments: Box<[u32]> },
    Unavailable(CallableResultUnavailableReasonV1),
}

impl VerifiedCallableResultDispositionV1 {
    pub(crate) fn exact_i64(
        key: &CanonicalSameModuleCallableKeyV1,
        requirements: impl IntoIterator<Item = u32>,
    ) -> Result<Self, CallableResultCatalogErrorV1> {
        let required_i64_arguments = super::requirements::seal_requirements(key, requirements)?;
        Ok(Self::ExactI64 {
            required_i64_arguments,
        })
    }

    /// K1: a box result keeps the exact-i64 requirements of its constructor arguments.
    pub(crate) fn exact_nominal_box(
        key: &CanonicalSameModuleCallableKeyV1,
        box_name: String,
        requirements: impl IntoIterator<Item = u32>,
    ) -> Result<Self, CallableResultCatalogErrorV1> {
        let required_i64_arguments = super::requirements::seal_requirements(key, requirements)?;
        Ok(Self::ExactNominalBox { box_name, required_i64_arguments })
    }

    pub(crate) fn exact_bool(
        key: &CanonicalSameModuleCallableKeyV1,
        requirements: impl IntoIterator<Item = u32>,
    ) -> Result<Self, CallableResultCatalogErrorV1> {
        let required_i64_arguments = super::requirements::seal_requirements(key, requirements)?;
        Ok(Self::ExactBool {
            required_i64_arguments,
        })
    }

    /// R13: the requirement is a scalar proof's, whichever kind the result has.
    pub(crate) fn required_i64_arguments(&self) -> Option<&[u32]> {
        match self {
            Self::ExactI64 {
                required_i64_arguments,
            }
            | Self::ExactBool {
                required_i64_arguments,
            } => Some(required_i64_arguments),
            Self::ExactString
            | Self::ExactNominalBox { .. }
            | Self::Unavailable(_) => None,
        }
    }

    /// K1/K5: the exact-i64 requirements of a call whose result is a box handle or a String. The
    /// scalar accessors (`required_i64_arguments`, `scalar_result_class`) stay scalar-only.
    pub(crate) fn non_scalar_required_i64_arguments(&self) -> Option<&[u32]> {
        match self {
            Self::ExactNominalBox { required_i64_arguments, .. } => Some(required_i64_arguments),
            Self::ExactString => Some(&[]),
            _ => None,
        }
    }

    /// Original static claim admission and its input obligations, independent
    /// of the package catalog's result-kind decision. Text keeps its own ABI.
    pub(crate) fn claim_required_i64_arguments(&self) -> Option<&[u32]> {
        match self {
            Self::ExactNominalBox { required_i64_arguments, .. }
            | Self::ExactI64 { required_i64_arguments }
            | Self::ExactBool { required_i64_arguments } => Some(required_i64_arguments),
            Self::ExactString | Self::Unavailable(_) => None,
        }
    }

    pub(crate) fn representation(&self) -> Option<VerifiedCallableResultRepresentationV1> {
        match self {
            Self::ExactI64 { .. } => Some(VerifiedCallableResultRepresentationV1::ExactI64),
            Self::ExactBool { .. } => Some(VerifiedCallableResultRepresentationV1::ExactBool),
            Self::ExactString => Some(VerifiedCallableResultRepresentationV1::ExactString),
            Self::ExactNominalBox { box_name, .. } => {
                Some(VerifiedCallableResultRepresentationV1::ExactNominalBox {
                    box_name: box_name.clone(),
                })
            }
            Self::Unavailable(_) => None,
        }
    }
}
