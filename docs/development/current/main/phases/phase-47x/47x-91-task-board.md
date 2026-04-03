---
Status: Active
Date: 2026-04-04
---

# 47x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `47xA contract lock` | landed | fix exact direct/core-first contracts before cutting helper defaults |
| 2 | `47xB runtime default cutover` | active | move day-to-day runtime off `--backend vm` |
| 3 | `47xC stage-a source->MIR first` | queued | make Stage-A direct MIR first and keep Program(JSON) compat explicit |
| 4 | `47xD stage-b caller drain` | queued | remove default dependence on the VM proof gate |
| 5 | `47xE closeout` | queued | prove the lane and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `47xA1` | landed | runtime/default contract lock |
| `47xA2` | landed | stage1 source->MIR contract lock |
| `47xA3` | landed | Stage-A direct/core contract lock |
| `47xB1` | landed | `selfhost_run_routes.sh` runtime temp-MIR handoff helper |
| `47xB2` | landed | `selfhost_run_routes.sh` runtime default cutover |
| `47xB3` | active | `run.sh` explicit vm compat mode lock |
| `47xC1` | queued | `stage0_capture_route.rs` non-VM builder add |
| `47xC2` | queued | `stage_a_route.rs` source->MIR first switch |
| `47xC3` | queued | `stage_a_compat_bridge.rs` explicit Program(JSON) fallback shrink |
| `47xD1` | queued | `selfhost_build_stageb.sh` MIR mainline artifact contract lock |
| `47xD2` | queued | `selfhost_build_stageb.sh` default-caller drain |
| `47xD3` | queued | `run_stageb_compiler_vm.sh` proof-only local keep |
| `47xE1` | queued | proof / closeout |

## Current Front

| Item | State |
| --- | --- |
| Now | `47xB3 run.sh explicit vm compat mode lock` |
| Blocker | `none` |
| Next | `47xC1 stage0_capture_route.rs non-VM builder add` |
| After Next | `47xC2 stage_a_route.rs source->MIR first switch` |
