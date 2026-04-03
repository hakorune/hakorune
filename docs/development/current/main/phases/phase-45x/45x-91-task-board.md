---
Status: Active
Date: 2026-04-03
---

# 45x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- |
| 1 | `45xA residual vm owner inventory` | active | inventory the remaining vm-backed owner surfaces and caller edges |
| 2 | `45xB proof-only keep boundary freeze` | queued | keep the proof-only VM gates explicit and non-growing |
| 3 | `45xC vm owner shrink` | queued | shrink `vm.rs` / `vm_fallback.rs` and keep compat narrow |
| 4 | `45xD proof / closeout` | queued | prove the residual cleanup and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `45xA1` | active | residual vm owner inventory lock |
| `45xA2` | queued | proof-only keep boundary freeze |
| `45xB1` | queued | `vm.rs` broad owner shrink |
| `45xB2` | queued | `vm_fallback.rs` / shared vm helper drain |
| `45xC1` | queued | `core.hako` compat hold line refresh |
| `45xC2` | queued | `run_stageb_compiler_vm.sh` proof-only gate reinforcement |
| `45xD1` | queued | proof / closeout |

## Current Front

| Item | State |
| --- | --- |
| Now | `45xA1 residual vm owner inventory` |
| Blocker | `none` |
| Next | `45xA2 proof-only keep boundary freeze` |
