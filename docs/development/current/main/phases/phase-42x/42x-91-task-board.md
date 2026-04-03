---
Status: Active
Date: 2026-04-03
---

# 42x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `42xA target lock` | landed | starvation targets and proof-only keeps are fixed before more migration work |
| 2 | `42xB facade migration` | active | `selfhost_build.sh` / `run.sh` stop feeding day-to-day work into vm-gated routes |
| 3 | `42xC vm owner drain` | queued | `child.rs` / `vm.rs` / compat keep lose live caller pressure and shrink |
| 4 | `42xD closeout` | queued | direct/core route is the mainline owner and vm stays proof/compat keep |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `42xA1` | landed | caller starvation target lock |
| `42xA2` | landed | proof-only VM keep freeze |
| `42xB1` | active | `selfhost_build.sh` downstream caller starvation |
| `42xB2` | queued | `run.sh` route-only facade migration |
| `42xC1` | queued | `child.rs` shell-only drain |
| `42xC2` | queued | `vm.rs` preflight/source-prepare split |
| `42xC3` | queued | `vm_user_factory` / `vm_fallback` drain |
| `42xC4` | queued | `core.hako` compat hold line |
| `42xD1` | queued | proof / closeout |
