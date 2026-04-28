use crate::mir::loop_route_detection::LoopRouteKind;

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct PlannerContext {
    pub route_kind: Option<LoopRouteKind>,
    pub in_static_box: bool,
    pub debug: bool,
}
