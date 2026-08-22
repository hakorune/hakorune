//! Route-neutral context seam for GenericLoop physical lowering.
//!
//! `LoopRouteContext` remains the legacy route owner's context.  The
//! source-backed GenericLoop lane must not construct one because its route was
//! already selected by the source/Facts issuer.  This trait exposes only the
//! diagnostic settings and an explicit legacy nested-loop capability.

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::lowering_context::PlanLoweringContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum GenericLoopV1LoweringContextRejectV1 {
    UnsupportedFirstCohort,
}

impl std::fmt::Display for GenericLoopV1LoweringContextRejectV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedFirstCohort => formatter
                .write_str("generic_loop_v1: nested lowering is outside the source first cohort"),
        }
    }
}

/// Narrow context consumed by GenericLoop body/pipeline helpers.
pub(in crate::mir::builder) trait GenericLoopV1LoweringContext:
    PlanLoweringContext
{
    /// A source-backed context returns `None`: nested lowering is not part of
    /// the first source cohort and must fail before physical effects instead
    /// of re-entering route classification.
    fn legacy_route_context(&self) -> Option<&LoopRouteContext<'_>>;
}

/// Physical settings for the source-backed first cohort.
///
/// This is deliberately not a route context.  It carries no route kind,
/// registry handle, AST classifier, or Builder state.
#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct GenericLoopV1SourceLoweringContextV1 {
    debug: bool,
    in_static_box: bool,
}

impl GenericLoopV1SourceLoweringContextV1 {
    pub(in crate::mir::builder) const fn new(debug: bool, in_static_box: bool) -> Self {
        Self {
            debug,
            in_static_box,
        }
    }
}

impl GenericLoopV1LoweringContext for GenericLoopV1SourceLoweringContextV1 {
    fn legacy_route_context(&self) -> Option<&LoopRouteContext<'_>> {
        None
    }
}

impl PlanLoweringContext for GenericLoopV1SourceLoweringContextV1 {
    fn function_name(&self) -> &str {
        "<source-generic-loop>"
    }

    fn debug_enabled(&self) -> bool {
        self.debug
    }

    fn in_static_box(&self) -> bool {
        self.in_static_box
    }
}

impl<'a> GenericLoopV1LoweringContext for LoopRouteContext<'a> {
    fn legacy_route_context(&self) -> Option<&LoopRouteContext<'_>> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{GenericLoopV1LoweringContext, GenericLoopV1SourceLoweringContextV1};
    use crate::mir::builder::control_flow::plan::lowering_context::PlanLoweringContext;

    #[test]
    fn source_context_has_no_legacy_route_capability() {
        let context = GenericLoopV1SourceLoweringContextV1::new(true, false);

        assert_eq!(context.function_name(), "<source-generic-loop>");
        assert!(context.debug_enabled());
        assert!(!context.in_static_box());
        assert!(context.legacy_route_context().is_none());
    }
}
