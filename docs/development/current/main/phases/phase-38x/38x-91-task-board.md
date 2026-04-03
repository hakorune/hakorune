Status: Active
Date: 2026-04-03

# 38x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `38xA legacy smoke archive` | active | archive former top-level embedded Stage1 smoke |
| 2 | `38xB delete-ready shim sweep` | queued-next | remove shims with no live callers and drained current pointers |
| 3 | `38xC archive-later queue freeze` | queued | pin top-level bootstrap/plugin/deadcode shims as archive-later until doc drain lands |
| 4 | `38xD closeout` | queued | return current front to next cleanup/source lane |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `38xA1` | active | `tools/stage1_smoke.sh` -> `tools/archive/legacy-selfhost/stage1_embedded_smoke.sh` |
| `38xB1` | queued-next | `tools/hako_check_deadblocks_smoke.sh` delete-ready pointer sweep |
| `38xC1` | queued | freeze `tools/bootstrap_selfhost_smoke.sh`, `tools/plugin_v2_smoke.sh`, `tools/hako_check_deadcode_smoke.sh` as archive-later |
| `38xD1` | queued | closeout and handoff |
