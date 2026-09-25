//! MIR callsite canonicalization pass.
//!
//! Post-RCL-3:
//! - `MirInstruction::BoxCall` / `MirInstruction::ExternCall` are retired.
//! - unresolved `callee=None` calls are no longer repaired here; ingress owns
//!   target resolution and this pass consumes typed Call targets only.
//! - NCL-0 is retired (R7-S8): no production ingress mints
//!   `LegacyCallV0{Closure}`; lambdas mint `NewClosure` directly and
//!   residual rows hit the backend named-stops, not this pass.
//! - NCL-1 keeps `NewClosure` thin by externalizing inline bodies:
//!   `NewClosure{body=[...], body_id=None} -> NewClosure{body=[], body_id=Some(id)}`.
//! - NCL-2 fixes closure-call shape boundary:
//!   only `dst=Some(_) + args=[]` is canonicalized to `NewClosure`.
//! - UCM-1 is retired (R7-S6): no production ingress mints
//!   `LegacyCallV0` anymore, so the user-box receiver repair arm was
//!   dead on every input and has been deleted. Residual legacy rows are
//!   rejected by the backend named-stops, not repaired here.
//! - Stage1 Program(JSON) BuildBox routing now lives in MIR-owned
//!   `global_call_routes` metadata. This pass no longer rewrites BuildBox
//!   authority calls to the Stage1 extern helper.

#[path = "callsite_canonicalize/pass.rs"]
mod pass;
#[path = "callsite_canonicalize/receiver_operand.rs"]
mod receiver_operand;
#[path = "callsite_canonicalize/schedule.rs"]
mod schedule;

pub use pass::canonicalize_callsites;
pub use schedule::{canonicalize_for_site, CallsiteCanonicalizeScheduleSite};

#[cfg(test)]
#[path = "callsite_canonicalize/tests/mod.rs"]
mod tests;
