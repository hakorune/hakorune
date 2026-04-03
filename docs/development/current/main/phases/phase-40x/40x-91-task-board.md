---
Status: Active
Date: 2026-04-03
---

# 40x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `40xA archive candidate inventory` | landed | new feature work がまだ `rust-vm` を引きずる route を exact に inventory する |
| 2 | `40xB keep/archive classification` | active | route を `proof-only keep` / `compat keep` / `archive-later` / `direct-owner target` / `must-split-first` に分ける |
| 3 | `40xC archive/delete sweep` | queued | drained vm-facing shims と stale compat wrappers を live surface から外す |
| 4 | `40xD closeout` | queued | `rust-vm` を proof/compat keep に縮めた reading で次 lane に handoff する |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `40xA1` | landed | archive candidate inventory を fixed surfaces に落とす |
| `40xA2` | landed | inventoried routes を `must-split-first` / `proof-only keep` / `compat keep` / `archive-later` / `direct-owner target` に固定する |
| `40xB1` | landed | small proof-only VM gate set を `do-not-grow` として固定した状態を freeze する |
| `40xB2` | active | top-level shim callers を direct/core route 側へ drain する map を作る |
| `40xC1` | queued | drained vm-facing shim / wrapper を archive/delete へ送る |
| `40xD1` | queued | proof を戻して next lane に handoff する |
