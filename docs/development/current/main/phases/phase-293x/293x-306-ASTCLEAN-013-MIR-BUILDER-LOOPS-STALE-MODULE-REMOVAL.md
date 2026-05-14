# 293x-306 ASTCLEAN-013 MIR builder loops stale module removal

Status: complete

## Decision

Decision: accepted.

`src/mir/builder/loops.rs` was a stale helper shelf: it was declared as a module but had no live callers outside itself. Removing the module is cleaner than preserving its helpers behind `#[allow(dead_code)]`.

## Scope

- Delete `src/mir/builder/loops.rs`.
- Remove `pub(crate) mod loops;` from `src/mir/builder.rs`.
- Keep active loop APIs in `loop_api_impl`, control-flow builder code, and lowerers untouched.

## Non-goals

- No loop lowering change.
- No Loop API change.
- No planner or JoinIR behavior change.

## Guard

- `tools/checks/k2_wide_astclean_mir_builder_loops_stale_module_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_mir_builder_loops_stale_module_guard.sh` passed locally.
