//! Loop source classification, recipe matching and checked-source MIR lowering.
//!
//! The checked source writer lives in `lowerer`. Recipe matching and facts
//! also support the entry diagnostics.

mod call_source;
mod domain;
pub(in crate::mir::builder) mod expression_port;
pub(in crate::mir::builder) mod facts;
pub(in crate::mir) use facts::LoopFactsPolicyFrameV1;
pub(in crate::mir::builder) mod lowerer;
mod trace;
mod generic_loop;
pub(in crate::mir::builder) mod loop_break;
pub(in crate::mir::builder) mod loop_cond_unified_helpers;
pub(in crate::mir::builder) mod loop_cond;
mod loop_true_break_continue;
mod parts;
pub(in crate::mir::builder) mod recipe_tree;
mod emit;
pub(in crate::mir::builder) mod planner;
pub(in crate::mir::builder) mod single_planner;
pub(in crate::mir::builder) use planner::PlanBuildOutcome;
pub(in crate::mir::builder) use self::call_source::CoreCallSourceV1;
pub(in crate::mir::builder) use self::expression_port::LoopPlanExpressionPortErrorV1;
pub(in crate::mir::builder) use self::expression_port::LoopPlanExpressionPortV1;
pub(in crate::mir::builder) use self::domain::LoopBreakStepPlacement;
