# Phase 194x: remaining oversized module split series

Status: Landed

Purpose
- split the remaining oversized Rust files before any new optimization or semantics work resumes
- keep this phase BoxShape-only: no behavior widening, no schema change, no backend route change

Scope
- split `src/boxes/array/mod.rs`
- split `src/runner/mir_json_emit/mod.rs`
- split `src/mir/string_corridor_placement.rs`
- sync docs/pointers so restart paths point at the new module seams

Non-goals
- no ArrayBox semantics change
- no MIR JSON schema change
- no string corridor candidate or plan behavior widening
- no DCE widening

Acceptance
- `cargo fmt --check`
- targeted unit suites for the touched modules stay green
- `bash tools/checks/dev_gate.sh quick`
- `git diff --check`

Series order
1. array module split
2. MIR JSON emit module split
3. string corridor placement module split

Result
- `src/boxes/array/mod.rs` is now a thin facade over focused storage/ops/traits/tests modules
- `src/runner/mir_json_emit/mod.rs` is now a thin facade over root/order/decls/plans/io/tests modules
- `src/mir/string_corridor_placement.rs` is now retired in favor of `src/mir/string_corridor_placement/`
