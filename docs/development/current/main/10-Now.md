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

- lane: `phase-33x shared helper family recut`
- `phase-30x` landed: backend roles and docs/artifact/smoke ownership are settled
- `phase-31x` landed: low-blast engineering rehome and shim drain are complete
- `phase-32x` landed: mixed-owner source/smoke split and raw default/token defer are fixed
- `33xA1` landed: helper family caller inventory is fixed for `hako_check` and `emit_mir`
- active micro task: `33xB2 hako_check.sh top-level keep gate`
- next micro task: `33xD1 closeout/docs cleanup`
- backend reading:
  - `llvm/exe` = `product`
  - `rust-vm` = `engineering(stage0/bootstrap + tooling keep)`
  - `vm-hako` = `reference/conformance`
  - `wasm` = `experimental`
- raw backend default/token rewrite stays deferred beyond `phase-33x`
- source/smoke cleanup rule:
  - `split/rehome/drain -> delete`

## Read Next

1. read `CURRENT_TASK.md`
2. read `15-Workstream-Map.md`
3. read `docs/development/current/main/phases/phase-33x/README.md`
4. read `docs/development/current/main/phases/phase-33x/33x-90-shared-helper-family-recut-ssot.md`
5. read `docs/development/current/main/phases/phase-33x/33x-91-task-board.md`
