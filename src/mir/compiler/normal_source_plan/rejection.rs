use super::product::NormalSourcePlanOwnerV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NormalSourcePlanStageV1 {
    RootSurface,
    SourceEntry,
    FamilyClosure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalSourcePlanIdentityFieldV1 {
    SourceIdentity,
    Digest,
    GrammarProfile,
    Utf8Length,
    ReadCount,
    ParseCount,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NormalUnsupportedTopLevelKindV1 {
    NestedProgram,
    Using,
    Import,
    BuildGate,
    Box,
    Enum,
    Brand,
    TypeAlias,
    Global,
    StaticConstTable,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NormalSourcePlanErrorV1 {
    SourceAuthorityUnavailable,
    CompatibilitySourceUnavailable,
    ParserSourceIncomplete,
    ParserSourceIntegrityInvalid,
    SourceLineageUnavailable,
    SourceIdentityMismatch {
        field: NormalSourcePlanIdentityFieldV1,
    },
    RootNotProgram,
    ParserSurfaceObservation(crate::parser::NormalSourcePlanSurfaceLoanErrorV1),
    RootExecutionRelationMismatch,
    MissingSourceEntry,
    DuplicateMain,
    MainMustBeStatic,
    MainMethodMissing,
    MainMethodMustBeFunction,
    MainMethodMustBeStatic,
    MainMethodNameMismatch {
        method_key: Box<str>,
        declaration_name: Box<str>,
    },
    MainArityMismatch {
        actual: usize,
    },
    MainMemberCoverageMismatch {
        observed: u32,
        callable: usize,
    },
    MainHelperMustBeFunction {
        method_key: Box<str>,
    },
    MainHelperNameMismatch {
        method_key: Box<str>,
        declaration_name: Box<str>,
    },
    MixedSourceFamilies,
    UnsupportedTopLevelSurface {
        statement_index: usize,
        kind: NormalUnsupportedTopLevelKindV1,
    },
}

impl NormalSourcePlanErrorV1 {
    pub(crate) fn stage(&self) -> NormalSourcePlanStageV1 {
        match self {
            Self::SourceAuthorityUnavailable
            | Self::CompatibilitySourceUnavailable
            | Self::ParserSourceIncomplete
            | Self::ParserSourceIntegrityInvalid
            | Self::SourceLineageUnavailable
            | Self::SourceIdentityMismatch { .. }
            | Self::RootNotProgram
            | Self::ParserSurfaceObservation(_)
            | Self::RootExecutionRelationMismatch
            | Self::UnsupportedTopLevelSurface { .. } => NormalSourcePlanStageV1::RootSurface,
            Self::MixedSourceFamilies => NormalSourcePlanStageV1::FamilyClosure,
            Self::MissingSourceEntry
            | Self::DuplicateMain
            | Self::MainMustBeStatic
            | Self::MainMethodMissing
            | Self::MainMethodMustBeFunction
            | Self::MainMethodMustBeStatic
            | Self::MainMethodNameMismatch { .. }
            | Self::MainArityMismatch { .. }
            | Self::MainMemberCoverageMismatch { .. }
            | Self::MainHelperMustBeFunction { .. }
            | Self::MainHelperNameMismatch { .. } => NormalSourcePlanStageV1::SourceEntry,
        }
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNormalSourcePlanV1 {
    owner: NormalSourcePlanOwnerV1,
    stage: NormalSourcePlanStageV1,
    error: NormalSourcePlanErrorV1,
}

impl RejectedNormalSourcePlanV1 {
    pub(super) fn new(
        owner: impl Into<NormalSourcePlanOwnerV1>,
        error: NormalSourcePlanErrorV1,
    ) -> Self {
        let stage = error.stage();
        Self {
            owner: owner.into(),
            stage,
            error,
        }
    }

    pub(super) fn from_owner(
        owner: NormalSourcePlanOwnerV1,
        error: NormalSourcePlanErrorV1,
    ) -> Self {
        let stage = error.stage();
        Self {
            owner,
            stage,
            error,
        }
    }

    pub(crate) fn stage(&self) -> &NormalSourcePlanStageV1 {
        &self.stage
    }

    pub(crate) fn error(&self) -> &NormalSourcePlanErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        let Self {
            owner,
            stage,
            error,
        } = self;
        owner.discard_after_source_plan_terminal();
        drop((stage, error));
    }
}

impl std::fmt::Display for RejectedNormalSourcePlanV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[normal-source-plan/rejected] stage={:?} error={:?}",
            self.stage, self.error
        )
    }
}

impl std::error::Error for RejectedNormalSourcePlanV1 {}
