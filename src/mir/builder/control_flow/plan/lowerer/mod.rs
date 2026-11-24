//! Emits checked source loops and records the physical edges for validation.

mod loop_cond_after_role;
mod loop_cond_phi_validation;
mod source_loop_completion;
mod source_loop_control;
mod source_loop_driver;
mod source_loop_edges;
mod source_loop_frames;

pub(in crate::mir) use loop_cond_after_role::VerifiedLoopCondAfterPhysicalRoleV1;
pub(in crate::mir) use loop_cond_phi_validation::capture_exact_loop_phis_v2;
pub(in crate::mir::builder) use loop_cond_phi_validation::LoopPhiInputRowsV2;
pub(in crate::mir) use source_loop_driver::EmittedSourceLoopGraphV2;
pub(in crate::mir::builder) use source_loop_edges::VerifiedSourceLoopEdgesV2;

pub(in crate::mir::builder) struct PlanLowerer;
