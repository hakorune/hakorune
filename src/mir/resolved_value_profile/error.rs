//! Typed profile stops and invariant failures.

use crate::mir::compiler::located::{LocatedExprV1, LocatedStmtV1};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::product::TrivialProfileCoverageSubjectV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TrivialProfileStopSiteV1 {
    Owner(FunctionOwnerIdV1),
    Statement(SourceStmtSiteV1),
    Expression(SourceExprSiteV1),
    Binding(SourceBindingSiteV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TrivialProfileStopReasonV1 {
    OwnerFamilyOutsideProfile,
    FunctionMetadataOutsideProfile,
    TypedSignatureOutsideProfile,
    DeclaredLocalTypeOutsideProfile,
    ParameterRepresentationUnavailable,
    OutboxRepresentationUnavailable,
    MissingLocalInitializer,
    StringRepresentationUnavailable,
    VoidRepresentationUnavailable,
    NullRepresentationUnavailable,
    StatementOutsideProfile,
    ExpressionOutsideProfile,
    BinaryOperatorOutsideProfile,
    BinaryOperandsNotExact,
    IfMergeProfileNotHomogeneous,
    IfConditionNotBool,
    ReturnNotFinal,
    ReturnInsideFallthroughBranch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TrivialProfileStopV1 {
    site: TrivialProfileStopSiteV1,
    reason: TrivialProfileStopReasonV1,
}

impl TrivialProfileStopV1 {
    pub(super) const fn new(
        site: TrivialProfileStopSiteV1,
        reason: TrivialProfileStopReasonV1,
    ) -> Self {
        Self { site, reason }
    }

    pub(crate) const fn site(&self) -> &TrivialProfileStopSiteV1 {
        &self.site
    }

    pub(crate) const fn reason(&self) -> TrivialProfileStopReasonV1 {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TrivialProfileContractErrorV1 {
    OwnerTransportMismatch,
    InvalidFunctionRoot,
    IfControlOwnerMismatch,
    SourceNavigation {
        detail: String,
    },
    MissingDeclarationBinding {
        site: SourceBindingSiteV1,
    },
    MissingVariableResolution {
        site: SourceExprSiteV1,
    },
    NonLocalVariableResolution {
        site: SourceExprSiteV1,
    },
    MissingAssignmentResolution {
        site: SourceExprSiteV1,
    },
    NonBindingAssignmentResolution {
        site: SourceExprSiteV1,
    },
    ForeignBinding {
        binding: BindingRefV1,
    },
    MissingReachingProfile {
        binding: BindingRefV1,
    },
    BlockExprPairNotSealed {
        site: SourceExprSiteV1,
    },
    DuplicateCoverage {
        subject: TrivialProfileCoverageSubjectV1,
    },
    DeclarationFactCoverageMismatch {
        missing: Box<[SourceBindingSiteV1]>,
        extra: Box<[SourceBindingSiteV1]>,
    },
    VariableFactCoverageMismatch {
        missing: Box<[SourceExprSiteV1]>,
        extra: Box<[SourceExprSiteV1]>,
    },
    AssignmentFactCoverageMismatch {
        missing: Box<[SourceExprSiteV1]>,
        extra: Box<[SourceExprSiteV1]>,
    },
    IfControlCoverageMismatch {
        missing: Box<[SourceStmtSiteV1]>,
        extra: Box<[SourceStmtSiteV1]>,
    },
    TerminalCardinality,
}

pub(super) type AnalysisResultV1<T> = Result<T, AnalysisFailureV1>;

pub(super) enum AnalysisFailureV1 {
    Stop(TrivialProfileStopV1),
    Contract(TrivialProfileContractErrorV1),
}

impl From<TrivialProfileContractErrorV1> for AnalysisFailureV1 {
    fn from(error: TrivialProfileContractErrorV1) -> Self {
        Self::Contract(error)
    }
}

pub(super) fn stop<T>(
    site: TrivialProfileStopSiteV1,
    reason: TrivialProfileStopReasonV1,
) -> AnalysisResultV1<T> {
    Err(AnalysisFailureV1::Stop(TrivialProfileStopV1::new(
        site, reason,
    )))
}

pub(super) fn stop_statement<T>(
    statement: &LocatedStmtV1<'_>,
    reason: TrivialProfileStopReasonV1,
) -> AnalysisResultV1<T> {
    stop(
        TrivialProfileStopSiteV1::Statement(statement.site().clone()),
        reason,
    )
}

pub(super) fn stop_expression<T>(
    expression: &LocatedExprV1<'_>,
    reason: TrivialProfileStopReasonV1,
) -> AnalysisResultV1<T> {
    stop(
        TrivialProfileStopSiteV1::Expression(expression.site().clone()),
        reason,
    )
}
