# Phase 192x: DCE pass module split

Status: Landed

Purpose
- split the oversized `src/mir/passes/dce.rs` file into a thin facade plus focused modules before any further lane-A2 / lane-B widening
- keep this cut BoxShape-only: no new DCE semantics, no new memory reasoning, no lane mixing

Scope
- extract elimination flow into a focused module
- extract local-field pruning helpers into a focused module
- move monolithic test bodies into topic-scoped test modules
- sync pointer/docs so the next optimization step still reads as lane-A2

Non-goals
- no new DCE widening
- no generic memory `Store` / `Load`
- no `Debug` / terminator cleanup
- no string or backend seam work

Acceptance
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::dce::tests -- --nocapture`
- `bash tools/checks/dev_gate.sh quick`
- `git diff --check`

Result
- `src/mir/passes/dce.rs` is now a thin facade over `dce/elimination.rs`, `dce/local_fields.rs`, and topic-split tests
- lane-A2 and later DCE slices can continue without carrying a 2000+ line pass file
