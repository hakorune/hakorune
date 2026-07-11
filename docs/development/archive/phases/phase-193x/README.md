# Phase 193x: string corridor sink module split

Status: Landed

Purpose
- split the oversized `src/mir/passes/string_corridor_sink.rs` file into a thin facade plus focused modules before any broader string or DCE follow-on
- keep this cut BoxShape-only: no new string corridor semantics, no new publication rules, no new perf route changes

Scope
- extract shared helper logic into a focused module
- extract retained-len, concat-corridor, fusion, and publication logic into focused modules
- move monolithic sink tests into topic-scoped test modules
- sync pointer/docs so later string or DCE work reads the new module boundary instead of the retired single-file seam

Non-goals
- no new string corridor widening
- no new DCE semantics
- no backend shim route changes
- no perf retuning

Acceptance
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::string_corridor_sink::tests -- --nocapture`
- `bash tools/checks/dev_gate.sh quick`
- `git diff --check`

Result
- `src/mir/passes/string_corridor_sink.rs` is retired in favor of a thin facade plus focused implementation and test modules
- later string and optimization work can continue without carrying a 5000-line sink file
