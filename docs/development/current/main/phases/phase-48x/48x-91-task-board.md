---
Status: Active
Date: 2026-04-04
---

# 48x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `48xA inventory and classify` | landed | inventory the remaining rust-vm-facing smoke/source routes and split day-to-day callers from proof-only / compat keeps |
| 2 | `48xB smoke cleanup` | active | clean smoke scripts that still make vm look like a day-to-day route |
| 3 | `48xC source cleanup` | landed | trim source helpers and fallback rails so vm stays thin |
| 4 | `48xD docs/examples cleanup` | active | remove stale `--backend vm` examples and commentary |
| 5 | `48xE proof / closeout` | queued | prove the cleanup and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `48xA1` | landed | residual vm surface inventory lock |
| `48xA2` | landed | proof-only / compat keep classification |
| `48xB1` | landed | smoke script stale-route cleanup |
| `48xB2` | landed | proof-only smoke gate lock |
| `48xC1` | landed | source helper stale-route cleanup |
| `48xC2` | landed | vm.rs / vm_fallback thin keep trim |
| `48xD1` | landed | README/example command cleanup |
| `48xD2` | active | stale `--backend vm` commentary cleanup |
| `48xE1` | queued | proof / closeout |

## Current Front

| Item | State |
| --- | --- |
| Now | `48xD2 stale \`--backend vm\` commentary cleanup` |
| Blocker | `none` |
| Next | `48xE1 proof / closeout` |
| After Next | `none` |
