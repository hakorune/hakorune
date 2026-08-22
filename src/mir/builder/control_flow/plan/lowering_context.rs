//! Route-neutral diagnostic context for CorePlan physical lowering.
//!
//! This context is intentionally smaller than `LoopRouteContext`.  A source-
//! backed Recipe already owns route selection; the physical lowerer needs only
//! stable diagnostics and the static-box policy bit.  Legacy callers still
//! implement this trait through their existing route context.

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;

pub(in crate::mir::builder) trait PlanLoweringContext {
    fn function_name(&self) -> &str;
    fn debug_enabled(&self) -> bool;
    fn in_static_box(&self) -> bool;
}

impl<'a> PlanLoweringContext for LoopRouteContext<'a> {
    fn function_name(&self) -> &str {
        self.func_name
    }

    fn debug_enabled(&self) -> bool {
        self.debug
    }

    fn in_static_box(&self) -> bool {
        self.in_static_box
    }
}
