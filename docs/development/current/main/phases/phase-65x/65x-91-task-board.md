---
Status: Active
Date: 2026-04-04
---

# 65x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `65xA inventory/lock` | active | fix the current stage1/selfhost owner set and proof bundle |
| 2 | `65xB cleanup` | queued | narrow runner/shell contract owners around the mainline path |
| 3 | `65xC proof` | queued | refresh focused mainline proofs |
| 4 | `65xD closeout` | queued | prove and hand off cleanly |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `65xA1` | landed | stage1/selfhost owner inventory lock |
| `65xA2` | landed | mainline contract / proof lock |
| `65xB1` | active | runner authority owner cleanup |
| `65xB2` | queued | shell contract owner cleanup |
| `65xC1` | queued | mainline proof bundle refresh |
| `65xD1` | queued | proof / closeout |

## Current Front

| Item | State |
| --- | --- |
| Now | `65xB1 runner authority owner cleanup` |
| Blocker | `focused emit_mir_mainline selfhost-first parse red at build_box.hako` |
| Next | `65xB2 shell contract owner cleanup` |
| After Next | `65xC1 mainline proof bundle refresh` |
