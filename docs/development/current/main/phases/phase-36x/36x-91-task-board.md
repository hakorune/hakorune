---
Status: Active
Decision: provisional
Date: 2026-04-03
Scope: `phase-36x selfhost source / stage1 bridge split` の concrete queue と evidence command をまとめる。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-36x/README.md
  - docs/development/current/main/phases/phase-36x/36x-90-selfhost-source-stage1-bridge-split-ssot.md
---

# 36x-91 Task Board

## Current Queue

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `36xA selfhost source split` | landed | `selfhost.rs` から source materialization を外す |
| 2 | `36xB stage1 raw bridge split` | landed | `stage1_cli` raw subcommand bridge を thin adapter に寄せる |
| 3 | `36xC proof/closeout` | landed | selfhost/stage1 split を evidence 化して handoff する |

## Ordered Slice Detail

| Order | Slice | Status | Read as |
| --- | --- | --- | --- |
| 1 | `36xA1` | landed | selfhost source prepare split |
| 2 | `36xA2` | landed | selfhost orchestration-only reread |
| 3 | `36xB1` | landed | stage1 emit-mir raw adapter split |
| 4 | `36xB2` | landed | stage1 run raw adapter split |
| 5 | `36xC1` | landed | proof/closeout |

## Evidence Commands

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
git diff --check
rg -n 'prepare_selfhost|try_run_selfhost_pipeline|_cmd_emit_mir_json|_run_raw_request|resolve_program_json_for_' \
  src/runner/selfhost.rs \
  src/runner/modes/common_util/selfhost/source_prepare.rs \
  lang/src/runner/stage1_cli/core.hako \
  lang/src/runner/stage1_cli/program_json_input.hako \
  docs/development/current/main/phases/phase-36x
cargo check --bin hakorune
```

## Current Result

- current front:
  - `phase-36x closeout review`
- current residue reading:
  - `source_prepare.rs` now owns source extension gate / source read / using merge / preexpand / tmp staging
  - `selfhost.rs` keeps macro pre-expand gate, fallback ordering, and terminal accept
  - `stage_a_route.rs` already owns Stage-A child spawn/setup and captured payload handoff
  - `stage1_cli/program_json_input.hako` already owns most source/program-json materialization helpers
  - `stage1_cli/raw_subcommand_emit_mir.hako` now owns raw `emit mir-json` request parse / materialization / emit glue
  - `stage1_cli/raw_subcommand_run.hako` now owns raw `run` request parse / script-args env / Program(JSON) materialization
  - `stage1_cli/core.hako` now keeps thin handoff only for raw `run`
  - `cargo check --bin hakorune` and `git diff --check` are green
  - `tools/stage1_smoke.sh mir-json` still fails with the inherited embedded `BuildBox` parse error, so this phase closes with that red unchanged rather than attributing it to the split
  - `tools/stage1_smoke.sh` now reads as a legacy embedded bridge smoke; current mainline smoke is `tools/selfhost/stage1_mainline_smoke.sh`
