//! Route-neutral diagnostics and explicit completion capability for CorePlan.
//!
//! This context is intentionally smaller than `LoopRouteContext`.  A source-
//! backed Recipe already owns route selection; the physical lowerer needs only
//! stable diagnostics, policy and an explicit source completion capability. Legacy callers still
//! implement this trait through their existing route context.

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;

pub(in crate::mir::builder) trait PlanLoweringContext {
    fn function_name(&self) -> &str;
    fn debug_enabled(&self) -> bool;
    fn in_static_box(&self) -> bool;

    fn validate_source_loop_completion(
        &self,
        _values: &super::SourceLoopFinalValuesV1,
    ) -> Result<(), String> {
        Err("[freeze:contract][loop-final-values/source-completion-unavailable]".to_owned())
    }

    fn publish_source_loop_completion(
        &self,
        _values: &super::SourceLoopFinalValuesV1,
    ) -> Result<(), String> {
        Err("[freeze:contract][loop-final-values/source-completion-unavailable]".to_owned())
    }
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
