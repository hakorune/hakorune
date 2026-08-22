//! Callback-scoped structural view for the existing loop normalizer owner.
//!
//! This is a transport seam only.  It does not expose route decisions,
//! semantic plan authority, execution state, or physical allocation.

use super::route_entry::router::LoopRouteContext;

/// One borrowed structural view minted by the existing `cf_loop_joinir_impl`
/// Context owner.  The fields remain private so callers cannot rebuild or
/// re-pair a route context from independent inputs.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopStructuralPortV1<'view> {
    diagnostic_label: &'view str,
    debug: bool,
}

impl CallableLoopStructuralPortV1<'_> {
    /// Diagnostic-only label; it is not a route or semantic identity key.
    pub(in crate::mir::builder) fn diagnostic_label(&self) -> &str {
        self.diagnostic_label
    }

    /// Borrowed debug mode copied from the existing structural owner.
    pub(in crate::mir::builder) const fn debug_enabled(&self) -> bool {
        self.debug
    }
}

/// Lend the existing structural view for exactly one callback scope.
///
/// The higher-ranked callback prevents the borrowed port from becoming a
/// storable source product.  This helper is caller-zero infrastructure until
/// a separate named normalizer consumer is accepted.
pub(in crate::mir::builder) fn with_existing_structural_port<R>(
    ctx: &LoopRouteContext<'_>,
    use_port: impl for<'view> FnOnce(CallableLoopStructuralPortV1<'view>) -> R,
) -> R {
    use_port(CallableLoopStructuralPortV1 {
        diagnostic_label: ctx.func_name,
        debug: ctx.debug,
    })
}

#[cfg(test)]
#[path = "structural_port_tests.rs"]
mod tests;
