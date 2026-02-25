use crate::ast::BinaryOperator;
use crate::mir::builder::control_flow::plan::facts::reject_reason::RejectReason;
use crate::mir::policies::CondProfile;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConditionCanon {
    pub loop_var_candidates: Vec<String>,
    pub cond_profile: CondProfile,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct UpdateCanon {
    pub op: BinaryOperator,
    pub step: i64,
    pub commuted: bool,
}

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
