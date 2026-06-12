//! Box callable registry.
//!
//! This module is the future callable truth for builtin, plugin, user, and
//! intrinsic Box call targets. It intentionally does not depend on Type ABI or
//! BoxDescriptor projection code.

pub mod model;
pub mod providers;
pub mod registry;
pub mod report;

pub use model::{
    BoxCallableKey, BoxCallableRole, BoxCallableTarget, BoxKey, CallableName, FunctionId,
    IntrinsicId,
};
pub use registry::BoxCallableRegistry;
