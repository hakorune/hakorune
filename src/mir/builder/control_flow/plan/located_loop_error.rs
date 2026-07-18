//! Typed construction failures for the disconnected located Loop plan seal.

use crate::mir::callable_result_representation::CallableResultLoopClaimScheduleErrorV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum LocatedCoreLoopPlanErrorV1 {
    PlanVerification(String),
    ClaimSchedule(CallableResultLoopClaimScheduleErrorV1),
    ExpectedLoopPlan,
    MissingLocatedOccurrence(SourceExprSiteV1),
    DuplicateLocatedOccurrence(SourceExprSiteV1),
    UnexpectedLocatedOccurrence(SourceExprSiteV1),
}
