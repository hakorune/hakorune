use crate::mir::builder::control_flow::plan::facts::reject_reason::RejectReason;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StepPlacement {
    Last,
    InBody(usize),
    InContinueIf(usize),
    InBreakElseIf(usize),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct StepPlacementDecision {
    pub placement: Option<StepPlacement>,
    pub reject_reason: Option<RejectReason>,
}
