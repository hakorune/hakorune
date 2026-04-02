---
Status: SSOT
Date: 2026-04-02
Scope: main ラインの current summary と正本リンクだけを置く薄い mirror/dashboard。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md
---

# Self Current Task — Now (main)

## Purpose

- この文書は docs 側の薄い mirror/dashboard。
- current summary と正本リンクだけを置く。
- landed history や inventory detail は `CURRENT_TASK` と phase docs に逃がす。

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`
- layout contract: `docs/development/current/main/DOCS_LAYOUT.md`

## Current

- lane: `phase-31x engineering lane isolation`
- `phase-30x` landed: backend roles and docs/artifact/smoke ownership are settled
- `31xA` landed: active lane switched and `tools/engineering/**` is the canonical home
- `31xB1` landed: `tools/engineering/run_vm_stats.sh` now holds the actual script
- `31xB2` landed: `tools/engineering/parity.sh` now holds the actual script
- `31xC` landed: shared helper family is fixed as `keep here`, not low-blast rehome
- active micro task: `31xD1 orchestrator keep vs rehome split`
- next micro task: `31xD2 docs and live path repoint`
- backend reading:
  - `llvm/exe` = `product`
  - `rust-vm` = `engineering(stage0/bootstrap + tooling keep)`
  - `vm-hako` = `reference/conformance`
  - `wasm` = `experimental`
- raw backend default flip stays deferred beyond `phase-30x`
- source/smoke cleanup rule:
  - `rehome -> shim -> drain -> delete`

## Read Next

1. read `CURRENT_TASK.md`
2. read `15-Workstream-Map.md`
3. read `docs/development/current/main/phases/phase-31x/README.md`
4. read `docs/development/current/main/phases/phase-31x/31x-90-engineering-lane-isolation-ssot.md`
5. read `docs/development/current/main/phases/phase-31x/31x-91-task-board.md`
