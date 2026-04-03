---
Status: Active
Date: 2026-04-04
---

# 48x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `48xA inventory and classify` | landed | inventory the remaining rust-vm-facing smoke/source routes and split day-to-day callers from proof-only / compat keeps |
| 2 | `48xB smoke cleanup` | active | clean smoke scripts that still make vm look like a day-to-day route |
| 3 | `48xC source cleanup` | queued | trim source helpers and fallback rails so vm stays thin |
| 4 | `48xD docs/examples cleanup` | queued | remove stale `--backend vm` examples and commentary |
| 5 | `48xE proof / closeout` | queued | prove the cleanup and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `48xA1` | landed | residual vm surface inventory lock |
| `48xA2` | landed | proof-only / compat keep classification |
| `48xB1` | landed | smoke script stale-route cleanup |
| `48xB2` | landed | proof-only smoke gate lock |
| `48xC1` | active | source helper stale-route cleanup |
| `48xC2` | queued | vm.rs / vm_fallback thin keep trim |
| `48xD1` | queued | README/example command cleanup |
| `48xD2` | queued | stale `--backend vm` commentary cleanup |
| `48xE1` | queued | proof / closeout |

## Current Front

| Item | State |
| --- | --- |
| Now | `48xC1 source helper stale-route cleanup` |
| Blocker | `none` |
| Next | `48xC2 vm.rs / vm_fallback thin keep trim` |
| After Next | `48xD1 README/example command cleanup` |
