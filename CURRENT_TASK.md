# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-22
Scope: repo root の再起動入口。詳細ログは `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で current blocker / next fixed order に到達する。
- 本ファイルは薄い入口に保ち、長文履歴は archive に逃がす。
- cleanup lane の単一正本は `docs/development/current/main/phases/phase-29cr/README.md`。
- current-task history archive の単一正本は `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`。

## Quick Restart Pointer

- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `git status -sb`
- `tools/checks/dev_gate.sh quick`
- `tools/checks/dev_gate.sh runtime-exec-zero`

## Current Priority

- `phase-29cr` P3: `src/mir` navigation-first cleanup
- landed slice:
  - `box_arithmetic.rs` -> `pub mod box_arithmetic { ... }` inline facade
  - `box_operators.rs` -> `src/boxes/operators/`
  - `runner_plugin_init.rs` -> `src/runner/plugin_init.rs`
  - `box_trait.rs` -> `src/boxes/box_trait.rs`
  - `operator_traits.rs` -> `src/boxes/operator_traits.rs`
  - `channel_box.rs` / `environment.rs` / `exception_box.rs` / `finalization.rs`
    / `instance_v2.rs` / `method_box.rs` / `scope_tracker.rs` / `type_box.rs`
    / `value.rs` / `ast.rs` / `benchmarks.rs` / `wasm_test.rs`
    -> directory modules
  - `src/mir/README.md`
  - `src/mir/builder/README.md`
  - `src/mir/join_ir/README.md`
  - `src/mir/loop_canonicalizer/README.md`
  - `src/mir/passes/README.md`
- next exact files:
  - `src/mir/builder/control_flow/normalization/README.md`
  - `src/mir/join_ir/lowering/README.md`
  - `src/mir/join_ir/ownership/README.md`
  - `src/mir/control_tree/step_tree/`
  - `src/mir/control_tree/normalized_shadow/`
- keep-root allowlist:
  - `basic_test.hako`
  - `test.hako`
- archive now:
  - `docs/archive/cleanup/root-hygiene/`
  - `tools/archive/root-hygiene/`
- P0 landed:
  - root archive relocation
  - `*.err` / `*.backup*` ignore policy

## Lane Pointers

- `phase-29cm`: collection owner cutover = done-enough stop line
- `phase-29y`: runtime `.hako` migration / boxcall contract = parked strict-polish
- `phase-21_5`: raw substrate perf = parked until boundary deepens
- `phase-29cr`: repo physical cleanup lane = active until P3 lands

## Archive

- full snapshot archive:
  - `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`
- prior archives:
  - `docs/development/current/main/investigations/current_task_archive_2026-03-04.md`
  - `docs/development/current/main/investigations/current_task_archive_2026-03-06_compiler_cleanup.md`
