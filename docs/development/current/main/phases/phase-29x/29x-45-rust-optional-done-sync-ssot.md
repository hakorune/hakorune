---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X23 Rust-optional done 判定と docs 同期の基準固定。
Related:
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-44-vm-route-three-day-gate-evidence.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - docs/reference/language/lifecycle.md
  - tools/selfhost/check_phase29x_x22_evidence.sh
  - tools/selfhost/check_phase29x_x23_readiness.sh
---

# Phase 29x X23: Rust-Optional Done Sync SSOT

## 0. Goal

X23 は「感覚的に完了」ではなく、機械判定できる基準で
Rust-optional done を docs と同期する。

## 1. Required Conditions

1. X22 完了:
   - `tools/selfhost/check_phase29x_x22_evidence.sh --strict` が PASS
2. Route contracts:
   - X20/X21 の route smoke が green 維持
3. GC optional policy:
   - `runtime-gc-policy-and-order-ssot.md` と `lifecycle.md` の
     「GCは意味論必須でない」記述が維持されている
4. Docs sync:
   - `README.md` / `29x-90` / `29x-91` / `CURRENT_TASK.md` が同じ完了状態を指している

## 2. Preflight Command

- `tools/selfhost/check_phase29x_x23_readiness.sh`
- strict 判定:
  - `tools/selfhost/check_phase29x_x23_readiness.sh --strict`

## 3. Completion Output (X23)

X23 完了時は次を更新する。

1. `docs/development/current/main/phases/phase-29x/README.md`
2. `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
3. `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`
4. `CURRENT_TASK.md`

## 4. Sync Rule with Master Map

- 本文書の strict readiness は `de-rust-master-task-map-ssot.md` の L4 判定として扱う。
- de-rust done 宣言時は X23 strict readiness と X32-X35 replay の両方を必須とする。
