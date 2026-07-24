//! DECLACCESS-COINSTALL0 physical-side registration seam.
//!
//! The aggregate installer currently performs its physical checks through the
//! read-only methods on `RawRootPhysicalStateV1`.  This child module reserves
//! the only future Builder sibling location for the consuming shell/collector
//! installation terminal; no second physical owner is introduced here.

use super::RawRootPhysicalStateV1;

/// Marker documenting the sole future physical environment terminal.
///
/// It intentionally has no constructor or production consumer in COINSTALL0:
/// the source manifest projection and shell declaration-fact primitive must be
/// sealed first.  Keeping this marker in the physical sibling prevents a
/// later compiler module from adding a raw `(shell, collector, ledger)` path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct RawRootPhysicalEnvironmentTerminalV1;

impl RawRootPhysicalStateV1 {
    pub(in crate::mir::builder) const fn environment_terminal(
        &self,
    ) -> RawRootPhysicalEnvironmentTerminalV1 {
        RawRootPhysicalEnvironmentTerminalV1
    }
}
