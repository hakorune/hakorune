---
Status: Active
Date: 2026-04-04
---

# 83x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `83xA inventory` | active | lock the exact top-level selfhost facade target set |
| 2 | `83xB archive/keep decision` | queued | classify each target and move only true archive-ready aliases |
| 3 | `83xC proof refresh` | queued | re-run the mainline/proof bundle after the decision |
| 4 | `83xD closeout` | queued | hand off cleanly to the next lane |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `83xA1` | active | top-level facade inventory lock |
| `83xA2` | queued | keep/archive decision freeze |
| `83xB1` | queued | archive-ready sweep or explicit keep proof |
| `83xC1` | queued | proof refresh |
| `83xD1` | queued | closeout |

## Current Front

| Item | State |
| --- | --- |
| Now | `83xA1 top-level facade inventory lock` |
| Blocker | `none` |
| Next | `83xA2 keep/archive decision freeze` |
| After Next | `83xB1 archive-ready sweep or explicit keep proof` |
