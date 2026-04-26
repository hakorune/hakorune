//! Stable semantic facades for route detector support modules.
//!
//! This module owns non-legacy caller paths. The current implementation still
//! lives in private `legacy/` storage; callers should depend on these semantic
//! facades so physical file moves can happen later without widening `legacy`.

/// Break-condition structural analysis support.
pub mod break_condition;

/// Loop-body local promotion support.
pub mod body_local {
    /// Carrier promotion support.
    pub mod carrier {
        pub use crate::mir::loop_route_detection::legacy::loop_body_carrier_promoter::*;
    }

    /// Condition promotion support.
    pub mod condition {
        pub use crate::mir::loop_route_detection::legacy::loop_body_cond_promoter::*;
    }
}

/// Condition-scope analysis support.
pub mod condition_scope {
    pub use crate::mir::loop_route_detection::legacy::loop_condition_scope::*;
}

/// Function-scope capture analysis support.
pub mod function_scope;

/// Local-variable analyzer support.
pub mod locals;

/// Trim-route support.
pub mod trim;
