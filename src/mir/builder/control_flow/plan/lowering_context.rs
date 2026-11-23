//! Route-neutral diagnostics and explicit completion capability for CorePlan.
//!
//! A source-backed Recipe owns source identity and route selection. The
//! physical lowerer reads the existing function name and debug flag.

pub(in crate::mir::builder) trait PlanLoweringContext {
    fn function_name(&self) -> &str;
    fn debug_enabled(&self) -> bool;
}

/// Route-neutral lowering context for source-backed callable loop plans.
///
/// The callable Recipe already owns route selection and source identity; this
/// product only carries the diagnostics the physical lowerer may read. It
/// never re-classifies route kind or inspects AST.
pub(in crate::mir::builder) struct CallableLoopPlanLoweringContextV1 {
    function_name: Box<str>,
    debug_enabled: bool,
}

impl CallableLoopPlanLoweringContextV1 {
    pub(in crate::mir::builder) fn new(function_name: &str, debug_enabled: bool) -> Self {
        Self {
            function_name: function_name.into(),
            debug_enabled,
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
}
