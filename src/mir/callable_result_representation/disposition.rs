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

/// Value-class evidence independent of the legacy call-admission projection.
/// Scalar/Text proofs already retain their kind; these classes need a separate
/// source fact when input substitution or the scalar-only ABI is unavailable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableSourceValueClassV1 {
    NominalBox(String),
    NullableNominalBox(String),
    Null,
    Map,
}

impl CallableSourceValueClassV1 {
    pub(super) fn join(&self, other: &Self) -> Option<Self> {
        use CallableSourceValueClassV1::*;
        if self == other { return Some(self.clone()); }
        match (self, other) {
            (NominalBox(name) | NullableNominalBox(name), Null)
            | (Null, NominalBox(name) | NullableNominalBox(name)) => Some(NullableNominalBox(name.clone())),
            (NominalBox(a), NullableNominalBox(b)) | (NullableNominalBox(a), NominalBox(b)) if a == b => Some(NullableNominalBox(a.clone())),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VerifiedCallableResultRepresentationV1 {
    ExactI64,
    ExactString,
    ExactBool,
    ExactNominalBox { nullable: bool, box_name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum VerifiedCallableResultDispositionV1 {
    ExactI64 { required_i64_arguments: Box<[u32]> },
    ExactString,
    ExactBool { required_i64_arguments: Box<[u32]> },
    ExactNominalBox { nullable: bool, box_name: String, required_i64_arguments: Box<[u32]> },
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
        nullable: bool,
        requirements: impl IntoIterator<Item = u32>,
    ) -> Result<Self, CallableResultCatalogErrorV1> {
        let required_i64_arguments = super::requirements::seal_requirements(key, requirements)?;
        Ok(Self::ExactNominalBox { box_name, nullable, required_i64_arguments })
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

    /// Original static claim admission and its input obligations, independent
    /// of the package catalog's result-kind decision. Text keeps its own ABI.
    pub(crate) fn admits_static_claim(&self) -> bool {
        self.required_i64_arguments().is_some() || matches!(self, Self::ExactNominalBox { .. })
    }

    pub(crate) fn representation(&self) -> Option<VerifiedCallableResultRepresentationV1> {
        match self {
            Self::ExactI64 { .. } => Some(VerifiedCallableResultRepresentationV1::ExactI64),
            Self::ExactBool { .. } => Some(VerifiedCallableResultRepresentationV1::ExactBool),
            Self::ExactString => Some(VerifiedCallableResultRepresentationV1::ExactString),
            Self::ExactNominalBox { box_name, nullable, .. } => {
                Some(VerifiedCallableResultRepresentationV1::ExactNominalBox {
                    box_name: box_name.clone(),
                    nullable: *nullable,
                })
            }
            Self::Unavailable(_) => None,
        }
    }
}
