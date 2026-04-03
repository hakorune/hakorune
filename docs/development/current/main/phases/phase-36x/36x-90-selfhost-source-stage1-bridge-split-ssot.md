---
Status: SSOT
Decision: provisional
Date: 2026-04-03
Scope: `selfhost.rs` source materialization と `stage1_cli` raw bridge residue を thin owner へ寄せる順番を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-36x/README.md
  - docs/development/current/main/phases/phase-36x/36x-91-task-board.md
  - docs/development/current/main/phases/phase-35x/35x-90-stage-a-compat-route-thinning-ssot.md
---

# 36x-90 Selfhost Source / Stage1 Bridge Split

## Goal

- `selfhost.rs` の source extension gate / source read / using merge / preexpand / tmp staging を `source_prepare.rs` の thin owner へ寄せる。
- `stage1_cli/core.hako` では raw subcommand adapter と source/program-json materialization owner をさらに分離する。

## Fixed Rules

- keep `selfhost.rs` focused on route sequencing / fallback ordering / terminal accept only
- keep selfhost source materialization under `source_prepare.rs`
- keep `stage1_cli/core.hako` raw `run` / `emit mir-json` branches narrow and no-widen
- keep `core_executor` as the direct MIR(JSON) owner below stage0
- raw backend default/token truthification remains deferred

## Macro Tasks

| Wave | Status | Goal | Acceptance |
| --- | --- | --- | --- |
| `36xA selfhost source split` | active | `selfhost.rs` から source materialization を外す | `selfhost.rs` が route sequencing / fallback ordering 中心になる |
| `36xB stage1 raw bridge split` | queued | `stage1_cli` raw subcommand bridge を thin adapter に寄せる | request parse / source materialization / raw handoff owner が path で読める |
| `36xC proof/closeout` | queued | slimmed selfhost/stage1 split を evidence 化して handoff する | next runtime design が raw compat residue を再拡張しなくて済む |

## Micro Tasks

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `36xA1` | landed | selfhost source prepare split | `source_prepare.rs` が extension gate / source read / using merge / preexpand / tmp staging を持つ |
| `36xA2` | landed | selfhost orchestration-only reread | `selfhost.rs` は route ordering / macro pre-expand gate / terminal accept 中心に読める |
| `36xB1` | landed | stage1 emit-mir raw adapter split | `_cmd_emit_mir_json` が thin adapter になり、materialization owner が box 側に寄る |
| `36xB2` | landed | stage1 run raw adapter split | `_run_raw_request` が thin adapter になり、Program(JSON) materialization owner が box 側に寄る |
| `36xC1` | landed | proof/closeout | selfhost source split と stage1 raw bridge split が evidence command まで固定される |

## Current Focus

- active macro wave: `landed precursor`
- active micro task: `phase-37x bootstrap owner split`
- next queued micro task: `37xA1 Stage-B producer isolation`
- current blocker: `none`
- exact reading:
  - `source_prepare.rs` now owns source extension gate / source read / using merge / preexpand / tmp staging
  - `selfhost.rs` keeps macro pre-expand gate, fallback ordering, and terminal accept
  - `stage_a_route.rs` already owns Stage-A child spawn/setup and captured payload handoff
  - `stage1_cli/program_json_input.hako` already owns most source/program-json materialization helpers
  - `stage1_cli/raw_subcommand_emit_mir.hako` now owns raw `emit mir-json` request parse / materialization / emit glue
  - `stage1_cli/raw_subcommand_run.hako` now owns raw `run` request parse / script-args env / Program(JSON) materialization
  - `stage1_cli/core.hako` keeps only thin adapter handoff into `run_program_json`
  - `cargo check --bin hakorune` and `git diff --check` are green after the split
  - `tools/stage1_smoke.sh mir-json` is still red with the pre-existing embedded `BuildBox` parse failure; current evidence does not localize that failure to `raw_subcommand_*`
  - `tools/stage1_smoke.sh` now reads as a legacy embedded bridge smoke; current mainline smoke is `tools/selfhost/stage1_mainline_smoke.sh`
