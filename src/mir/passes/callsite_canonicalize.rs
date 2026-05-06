//! MIR callsite canonicalization pass.
//!
//! Post-RCL-3:
//! - `MirInstruction::BoxCall` / `MirInstruction::ExternCall` are retired.
//! - pass keeps MCL-5 compatibility rewrite only:
//!   `Call(callee=None, func=<const-string>) -> Call(callee=Global, func=INVALID)`.
//! - NCL-0 keeps closure creation canonical as `NewClosure`:
//!   `Call(callee=Closure, dst=Some(_)) -> NewClosure`.
//! - NCL-1 keeps `NewClosure` thin by externalizing inline bodies:
//!   `NewClosure{body=[...], body_id=None} -> NewClosure{body=[], body_id=Some(id)}`.
//! - NCL-2 fixes closure-call shape boundary:
//!   only `dst=Some(_) + args=[]` is canonicalized to `NewClosure`.
//! - UCM-1 canonicalizes known user-box receiver methods onto
//!   `Call(callee=Method{certainty=Known, box_kind=UserDefined})` so later
//!   thin-entry consumers can bind physical entries without backend-local
//!   receiver guessing.
//! - Stage1 Program(JSON) build-surrogate BuildBox calls
//!   (`BuildBox.emit_program_json_v0/2` with static `null` opts and the direct
//!   `BuildBox._emit_program_json_from_scan_src/1` helper)
//!   canonicalize to the explicit `nyash.stage1.emit_program_json_v0_h` extern
//!   route. This mirrors the JSON v0 bridge contract and keeps Stage0 from
//!   selecting the BuildBox parser body as a same-module helper.

#[path = "callsite_canonicalize/helpers.rs"]
mod helpers;
#[path = "callsite_canonicalize/pass.rs"]
mod pass;

pub use pass::canonicalize_callsites;

#[cfg(test)]
#[path = "callsite_canonicalize/tests.rs"]
mod tests;
