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

/// Route-neutral lowering context for source-backed callable loop plans.
///
/// The callable Recipe already owns route selection and source identity; this
/// product only carries the diagnostics the physical lowerer may read. It
/// never re-classifies route kind or inspects AST.
pub(in crate::mir::builder) struct CallableLoopPlanLoweringContextV1 {
    function_name: Box<str>,
    debug_enabled: bool,
    in_static_box: bool,
}

impl CallableLoopPlanLoweringContextV1 {
    pub(in crate::mir::builder) fn new(
        function_name: &str,
        debug_enabled: bool,
        in_static_box: bool,
    ) -> Self {
        Self {
            function_name: function_name.into(),
            debug_enabled,
            in_static_box,
        }
    }
}

impl PlanLoweringContext for CallableLoopPlanLoweringContextV1 {
    fn function_name(&self) -> &str {
        &self.function_name
    }

    fn debug_enabled(&self) -> bool {
        self.debug_enabled
    }

    fn in_static_box(&self) -> bool {
        self.in_static_box
    }
}
