//! Normalization entry point consolidation (Phase 134 P0)
//!
//! ## Purpose
//!
//! Consolidate the dual entry points for Normalized shadow processing:
//! - `routing.rs::try_normalized_shadow()` (loop-only)
//! - `suffix_router_box::try_lower_loop_suffix()` (loop + post)
//!
//! Both now use the same NormalizationPlanBox for shape detection.
//!
//! ## Architecture
//!
//! - **NormalizationPlanBox**: SSOT for "what to normalize" decision
//! - **NormalizationExecuteBox**: SSOT for "how to execute" normalization
//! - **NormalizationPlan**: Data structure for plan details
//!
//! See README.md for full design and contract documentation.

mod plan;
mod plan_box;
mod execute_box;

pub use plan::PlanKind;
pub use plan_box::NormalizationPlanBox;
pub use execute_box::NormalizationExecuteBox;
