use super::product::PreparedNormalSourcePlanInputV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NormalSourcePlanStageV1 {
    RootSurface,
    SourceEntry,
    FamilyClosure,
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
    RootNotProgram,
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
    fn stage(&self) -> NormalSourcePlanStageV1 {
        match self {
            Self::RootNotProgram | Self::UnsupportedTopLevelSurface { .. } => {
                NormalSourcePlanStageV1::RootSurface
            }
            Self::MixedSourceFamilies => NormalSourcePlanStageV1::FamilyClosure,
            Self::MissingSourceEntry
            | Self::DuplicateMain
            | Self::MainMustBeStatic
            | Self::MainMethodMissing
            | Self::MainMethodMustBeFunction
            | Self::MainMethodMustBeStatic
            | Self::MainMethodNameMismatch { .. }
            | Self::MainArityMismatch { .. }
            | Self::MainHelperMustBeFunction { .. }
            | Self::MainHelperNameMismatch { .. } => NormalSourcePlanStageV1::SourceEntry,
        }
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNormalSourcePlanV1 {
    owner: PreparedNormalSourcePlanInputV1,
    stage: NormalSourcePlanStageV1,
    error: NormalSourcePlanErrorV1,
}

impl RejectedNormalSourcePlanV1 {
    pub(super) fn new(
        owner: PreparedNormalSourcePlanInputV1,
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
        drop(self);
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
