#[cfg(feature = "rc-insertion-minimal")]
use crate::mir::ValueId;

#[cfg(feature = "rc-insertion-minimal")]
#[derive(Debug, Clone)]
pub(super) struct RcPlan {
    pub(super) drops: Vec<DropSite>,
}

#[cfg(feature = "rc-insertion-minimal")]
#[derive(Debug, Clone)]
pub(super) enum DropPoint {
    BeforeInstr(usize),
    BeforeTerminator,
}

#[cfg(feature = "rc-insertion-minimal")]
#[derive(Debug, Clone)]
pub(super) enum DropReason {
    Overwrite,
    ExplicitNull,
    ReturnCleanup,
    BreakCleanup,
    ContinueCleanup,
}

#[cfg(feature = "rc-insertion-minimal")]
#[derive(Debug, Clone)]
pub(super) struct DropSite {
    pub(super) at: DropPoint,
    pub(super) values: Vec<ValueId>,
    pub(super) reason: DropReason,
}
