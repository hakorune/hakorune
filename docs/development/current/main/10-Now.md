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

- lane: `phase-30x backend surface simplification`
- `30xE1` landed: `README.md` / `README.ja.md` now read `llvm/exe` first, `rust-vm` engineering keep, `vm-hako` reference, `wasm` experimental
- `30xE2` landed: `cli-options.md` is role-first and `nyash-help.md` is explicitly historical
- `30xE3` landed: stage1/runtime guides now read `rust-vm` as engineering/bootstrap keep
- `30xE4` landed: user-facing wasm/reference docs no longer read as co-main
- `30xF1` landed: raw default flip is still blocked by launcher/default/orchestrator surfaces
- `30xF2` landed: phase-30x keeps raw backend token/default stable and treats ownership flip as sufficient
- `30xG1` landed: low-blast manual smoke residues moved under `tools/archive/manual-smokes/`
- `30xG2` landed: `docs/tools/nyash-help.md` is now a thin stub and the historical snapshot lives under `docs/archive/tools/`
- active micro task: `30xG3 compare/manual helper archive pass`
- next micro task: `30xG4 post-switch docs cleanup`
- backend reading:
  - `llvm/exe` = `product`
  - `rust-vm` = `engineering(stage0/bootstrap + tooling keep)`
  - `vm-hako` = `reference/conformance`
  - `wasm` = `experimental`
- raw backend default flip stays deferred to `30xF`
- legacy residue stays on `explicit keep / rewrite in 30xE / archive-delete in 30xG`

## Read Next

1. read `CURRENT_TASK.md`
2. read `15-Workstream-Map.md`
3. read `docs/development/current/main/phases/phase-30x/README.md`
4. read `docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md`
5. read `docs/development/current/main/phases/phase-30x/30x-91-task-board.md`
