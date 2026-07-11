---
Status: Landed
Date: 2026-04-04
---

# 56x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `56xA keep lock` | active | lock the exact proof/compat keep surfaces before pruning them |
| 2 | `56xB keep pruning` | queued | remove stale residue from explicit keep surfaces without deleting the keeps |
| 3 | `56xC proof smoke pruning` | landed | shrink proof smoke ownership to the smallest explicit set |
| 4 | `56xD closeout` | landed | prove and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `56xA1` | landed | proof-only keep inventory lock |
| `56xA2` | landed | compat keep boundary freeze |
| `56xB1` | landed | stage-a compat route pruning prep |
| `56xB2` | landed | vm fallback/core.hako keep pruning |
| `56xC1` | landed | proof smoke keep pruning |
| `56xD1` | landed | proof / closeout |

## Inventory Snapshot

| Surface | Current state | Next treatment |
| --- | --- | --- |
| `tools/selfhost/lib/selfhost_run_routes.sh` `stage-a` branch | explicit compat-only keep is still present | prune stale wrapper feeling without deleting the branch |
| `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` | explicit Program(JSON) compat bridge keep | keep-now, but trim stale broader wording/helpers |
| `src/runner/modes/vm_fallback.rs` | explicit fallback keep | narrow comments/helpers and avoid widening |
| `lang/src/runner/stage1_cli/core.hako` | raw compat hold line | keep-now, but prune stale residue |
| `tools/selfhost/run_stageb_compiler_vm.sh` | explicit proof-only keep | keep-now, but prune stale residue around callers/docs |

## Current Front

| Item | State |
| --- | --- |
| Now | `phase-56x landed` |
| Blocker | `none` |
| Next | `57xA1 residual rust-vm delete-ready inventory lock` |
| After Next | `57xA2 keep/delete/archive classification freeze` |
