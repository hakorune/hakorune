use super::CallableResultCatalogErrorV1;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableResultUnavailableReasonV1 {
    DeclaredNonI64Result,
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
    ExactNominalBox { box_name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VerifiedCallableResultDispositionV1 {
    ExactI64 { required_i64_arguments: Box<[u32]> },
    ExactNominalBox { box_name: String },
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

    pub(crate) fn required_i64_arguments(&self) -> Option<&[u32]> {
        match self {
            Self::ExactI64 {
                required_i64_arguments,
            } => Some(required_i64_arguments),
            Self::ExactNominalBox { .. } | Self::Unavailable(_) => None,
        }
    }

    pub(crate) fn representation(&self) -> Option<VerifiedCallableResultRepresentationV1> {
        match self {
            Self::ExactI64 { .. } => Some(VerifiedCallableResultRepresentationV1::ExactI64),
            Self::ExactNominalBox { box_name } => {
                Some(VerifiedCallableResultRepresentationV1::ExactNominalBox {
                    box_name: box_name.clone(),
                })
            }
            Self::Unavailable(_) => None,
        }
    }
}
