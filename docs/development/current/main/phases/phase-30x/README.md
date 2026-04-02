---
Status: Active
Decision: provisional
Date: 2026-04-02
Scope: backend surface を role-first に整理し、`llvm/exe` を product main、`rust-vm` を engineering/bootstrap、`vm-hako` を reference、`wasm` を experimental として分離する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md
  - docs/development/current/main/phases/phase-30x/30x-91-task-board.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/artifact-policy-ssot.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
---

# Phase 30x: Backend Surface Simplification

## Goal

- user-facing main を `llvm/exe` に寄せる。
- `rust-vm` を無理に剥がさず、engineering(stage0/bootstrap + tooling keep) lane として責務を固定する。
- `vm-hako` を reference/conformance lane として main narrative から分離する。
- `wasm` は experimental target のまま扱い、co-main に誤読させない。
- legacy residue は explicit keep / archive / delete に収束させる。

## Fixed Reading

- `phase-29x backend owner cutover prep` は landed precursor として読む。
- `phase-30x` は backend の役割と surface を整理する docs-first phase。
- 先に taxonomy と smoke split を固定し、raw CLI default や deep launcher plumbing は後ろに回す。
- raw default flip より先に、artifact/docs/smoke ownership を role-first に切り替える。

## Non-Goals

- `rust-vm` の即退役
- `wasm` の co-main 昇格
- inventory 前の `--backend vm` 一括置換
- selfhost/bootstrap/plugin/macro lane の強制剥離

## Exact Next

1. `30x-90-backend-surface-simplification-ssot.md`
2. `30x-91-task-board.md`
3. `execution-lanes-and-axis-separation-ssot.md`
4. `artifact-policy-ssot.md`
5. `stage2-aot-native-thin-path-design-note.md`

## Canonical Child Docs

- role taxonomy / phase rules:
  - `30x-90-backend-surface-simplification-ssot.md`
- concrete task order / evidence commands:
  - `30x-91-task-board.md`

## Acceptance Summary

- backend role taxonomy is explicit as `product / engineering / reference / experimental`
- `rust-vm` internal pressure map is explicit before any default/backend flip
- smoke taxonomy is split by role before any broad launcher change
- `llvm/exe` becomes the docs/help main narrative without forcing early bootstrap breakage
- manual legacy residue is now either explicit engineering keep or archive/delete
