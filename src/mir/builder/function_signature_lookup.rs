//! HEADERPORT0-S0: neutral signature/header lookup surface.
//!
//! The trait is deliberately narrower than `MirModule` and carries no
//! collector, draft, Builder, or lowering capability.  Later header ports may
//! implement it without making lowering depend on the module's storage shape.

use crate::mir::FunctionSignature;

/// Read-only function-header lookup used by port-aware finalization.
///
/// This is S0 vocabulary only.  No production finalizer consumes the trait
/// until the capture/commit cutover has proved that module fallback is gone.
pub(in crate::mir::builder) trait FunctionSignatureLookupV1 {
    fn signature(&self, symbol: &str) -> Option<&FunctionSignature>;

    fn contains_symbol(&self, symbol: &str) -> bool;

    fn symbol_count(&self) -> usize;

    fn visit_symbols(&self, visitor: &mut dyn FnMut(&str));
}
