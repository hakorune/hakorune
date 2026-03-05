use crate::mir::loop_pattern_detection::LoopPatternKind;

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct PlannerContext {
    pub route_kind: Option<LoopPatternKind>,
    pub in_static_box: bool,
    pub debug: bool,
}

impl PlannerContext {
    pub(in crate::mir::builder) fn default_for_legacy() -> Self {
        Self {
            route_kind: None,
            in_static_box: false,
            debug: false,
        }
    }
}
