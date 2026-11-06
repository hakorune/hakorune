//! DECLACCESS-COINSTALL0 physical-side registration seam.
//!
//! The aggregate installer currently performs its physical checks through the
//! read-only methods on `RawRootPhysicalStateV1`.  This child module reserves
//! the only future Builder sibling location for the consuming shell/collector
//! installation terminal; no second physical owner is introduced here.

