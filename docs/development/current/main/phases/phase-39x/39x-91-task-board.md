---
Status: Active
Date: 2026-04-03
---

# 39x-91 Task Board

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `39xA stage0 gate route inventory` | active | remaining vm-gated bootstrap surfaces を exact に inventory する |
| 2 | `39xB direct route selection` | queued | direct bootstrap mainline と explicit vm keep を分ける |
| 3 | `39xC caller drain / keep freeze` | queued | mixed routes から callers を drain し、keep set を freeze する |
| 4 | `39xD closeout` | queued | next source lane に handoff する |

## Exact Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `39xA1` | active | `selfhost_build.sh` / `run_stageb_compiler_vm.sh` / `run.sh` callers を inventory する |
| `39xA2` | queued | `vm 必須` / `direct` / `core_executor` を classify する |
| `39xB1` | queued | bootstrap mainline direct route を選ぶ |
| `39xB2` | queued | explicit vm gate keep set を freeze する |
| `39xC1` | queued | caller drain map を作る |
| `39xD1` | queued | proof / closeout を戻す |
