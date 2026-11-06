//! Port-aware adapter for ordinary direct-body join If lowering.
//!
//! The branch snapshot/join state remains owned by `lower_if_join_state_core`.
//! This file only injects exact expression/body carriers and the associated
//! condition/body lowerer used by the disconnected direct-body core.


