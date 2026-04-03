---
Status: Active
Date: 2026-04-03
---

# 40x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `40xA archive candidate inventory` | active | remaining vm-rust/bootstrap surfaces を exact に inventory する |
| 2 | `40xB keep/archive classification` | queued | explicit keep と archive-later を分ける |
| 3 | `40xC archive/delete sweep` | queued | drained shims と stale compat wrappers を live surface から外す |
| 4 | `40xD closeout` | queued | next source lane に handoff する |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `40xA1` | landed | archive candidate inventory を fixed surfaces に落とす |
| `40xA2` | active | keep/archive classification を固定する |
| `40xB1` | queued | top-level shim caller drain map を作る |
| `40xB2` | queued | vm-rust explicit keep freeze を固定する |
| `40xC1` | queued | archive/delete sweep を実行する |
| `40xD1` | queued | proof / closeout を戻す |
