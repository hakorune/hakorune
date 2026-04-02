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

- lane: `phase-32x product / engineering split`
- `phase-30x` landed: backend roles and docs/artifact/smoke ownership are settled
- `phase-31x` landed: low-blast engineering rehome and shim drain are complete
- `32xA1` landed: `build.rs` mixed ownership inventory is fixed
- `32xA2` landed: `phase2100` mixed aggregator inventory is fixed
- `32xB2` landed: helper-first extraction thinned `src/runner/build.rs` without changing owner behavior
- active micro task: `32xF1 shared helper follow-up gate`
- next micro task: `32xG1 raw backend default/token remains last`
- backend reading:
  - `llvm/exe` = `product`
  - `rust-vm` = `engineering(stage0/bootstrap + tooling keep)`
  - `vm-hako` = `reference/conformance`
  - `wasm` = `experimental`
- raw backend default flip stays deferred beyond `phase-30x`
- source/smoke cleanup rule:
  - `split/rehome/drain -> delete`

## Read Next

1. read `CURRENT_TASK.md`
2. read `15-Workstream-Map.md`
3. read `docs/development/current/main/phases/phase-32x/README.md`
4. read `docs/development/current/main/phases/phase-32x/32x-90-product-engineering-split-ssot.md`
5. read `docs/development/current/main/phases/phase-32x/32x-91-task-board.md`
