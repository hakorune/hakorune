---
Status: Active
Date: 2026-04-03
---

# 44x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `44xA stage-b owner cutover` | active | move Stage-B producer defaults away from VM-backed routes |
| 2 | `44xB runtime/direct route cutover` | queued | stop runtime/direct helpers from silently feeding VM defaults |
| 3 | `44xC stage0 capture neutralization` | queued | split route-neutral capture plumbing from backend-specific route builders |
| 4 | `44xD explicit VM gate demotion` | queued | leave VM gate scripts as explicit proof/fallback only |
| 5 | `44xE closeout` | queued | prove the cutover and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `44xA1` | landed | stage-b direct/core target lock |
| `44xA2` | landed | `selfhost_build_stageb.sh` direct/core-first cutover |
| `44xB1` | landed | `selfhost_run_routes.sh` runtime default cutover |
| `44xB2` | landed | `run.sh` direct route fallback explicitization |
| `44xC1` | active | `stage0_capture.rs` route-neutral builder split |
| `44xC2` | queued | `stage_a_route.rs` / compat caller switch |
| `44xD1` | queued | `run_stageb_compiler_vm.sh` proof-only demotion |
| `44xE1` | queued | proof / closeout |
