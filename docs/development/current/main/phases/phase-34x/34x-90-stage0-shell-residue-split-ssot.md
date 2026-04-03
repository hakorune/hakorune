---
Status: SSOT
Decision: provisional
Date: 2026-04-03
Scope: `stage0` engineering lane の shell residue を thin owner へ寄せる順番と no-widen rules を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-34x/README.md
  - docs/development/current/main/phases/phase-34x/34x-91-task-board.md
  - docs/development/current/main/phases/phase-32x/32x-90-product-engineering-split-ssot.md
---

# 34x-90 Stage0 Shell Residue Split

## Goal

- `stage0` engineering lane に残る shell/process/raw compat residue を thin にし、owner boundary を `core_executor` 側へ寄せる。
- `thread` など新しい runtime 設計が `child.rs` / `stage1_cli` / wrapper scripts を再肥大化させるのを防ぐ。

## Fixed Rules

- keep `stage0 hakorune binary` as outer engineering facade
- do not push more runtime capability into shell/process/raw compat branches
- prefer `in-proc core owner` over adding more child shell routes
- `Program(JSON v0)` raw compat stays narrow; do not widen `run_program_json` / `_run_raw_request`
- raw backend default/token truthification remains deferred

## Macro Tasks

| Wave | Status | Goal | Acceptance |
| --- | --- | --- | --- |
| `34xA residue owner lock` | active | exact shell residue / owner split を固定する | `child.rs` / `stage1_cli` / `core_executor` の owner reading が揃う |
| `34xB child runner thinning` | queued | `child.rs` の spawn/capture/process ownership を薄くする | JSON capture route が narrower helper に寄る |
| `34xC stage1 raw compat narrowing` | queued | `stage1_cli/core.hako` raw compat branch を narrow keep に固定する | raw compat branch が新機能で widen しない |
| `34xD direct core handoff` | queued | in-proc `MIR(JSON)` owner を `core_executor` に寄せる | shell residue を経由しない direct seam が増える |

## Micro Tasks

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `34xA1` | landed | `child.rs` exact residue lock | `run_ny_program_capture_json_v0` の責務と caller split が exact に読める |
| `34xA2` | landed | `stage1_cli/core.hako` exact residue lock | `run_program_json` / `_run_raw_request` の compat residue と dispatch split が exact に読める |
| `34xA3` | active | `core_executor` takeover seam lock | direct `MIR(JSON)` owner が shell route と分離して読める |
| `34xB1` | queued | split spawn/timeout/capture from `child.rs` | shell helper が route-neutral helper へ縮む |
| `34xC1` | queued | `run_program_json` no-widen lock | raw compat lane が execution-capability widening を受けない |
| `34xD1` | queued | direct `MIR(JSON)` proof path | already-materialized `MIR(JSON)` run path が `core_executor` 側に pin される |

## Current Focus

- active macro wave: `34xA residue owner lock`
- active micro task: `34xA3 core_executor takeover seam lock`
- next queued micro task: `34xB1 split spawn/timeout/capture from child.rs`
- current blocker: `none`
- exact residue reading:
  - `child.rs` shell/process residue is concentrated in `run_ny_program_capture_json_v0`
  - caller split around `child.rs` is fixed:
    - `selfhost.rs` consumes the shared v0 capture and resolves stage-a payload from `program_line` / `mir_line`
    - `stage_a_compat_bridge.rs` consumes the MIR-only selector wrapper
    - `run_ny_program_capture_json` stays route-neutral and owns no extra policy
  - `stage1_cli/core.hako` raw compat residue is concentrated in `run_program_json` and `_run_raw_request`
  - caller split around `stage1_cli/core.hako` is fixed:
    - `stage1_main` stays dispatcher-only
    - `dispatch_env_mode` drives `_mode_emit_program`, `_mode_emit_mir_from_env_min`, and `_mode_run`
    - `dispatch_emit` drives `_cmd_emit_mir_json`
    - `dispatch_run` drives `_run_raw_request`
    - `run_program_json` keeps default `vm`, accepts only `vm|pyvm`, and rejects `llvm`
  - `core_executor` already owns the direct in-proc `MIR(JSON)` seam; this phase narrows the handoff around it

## Accepted Prior Reading

- `phase-32x` already fixed:
  - `child.rs` residue is concentrated in `run_ny_program_capture_json_v0`
  - `stage1_cli/core.hako` residue is concentrated in `run_program_json` and `_run_raw_request`
  - `core_executor` is the narrow direct-MIR owner for already-materialized `MIR(JSON)`

Read as:
- this phase does not restart inventory from zero
- it turns the accepted inventory into a narrower owner split
