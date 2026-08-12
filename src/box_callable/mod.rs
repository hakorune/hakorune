//! Box callable registry.
//!
//! This module is the future callable truth for builtin, plugin, user, and
//! intrinsic Box call targets. It intentionally does not depend on Type ABI or
//! BoxDescriptor projection code.

pub mod model;
pub(crate) mod admitted;
pub(crate) mod text_scan;
pub mod providers;
pub mod registry;
pub mod report;
pub mod route_plan;

pub use model::{
    BoxCallableEntry, BoxCallableKey, BoxCallableRole, BoxCallableSource, BoxCallableTarget,
    BoxKey, CallableName, FunctionId, IntrinsicId,
};
pub use registry::BoxCallableRegistry;
pub use route_plan::{DropBoxRoutePlan, InvokeRoutePlan, MethodCallRoutePlan, NewBoxRoutePlan};
