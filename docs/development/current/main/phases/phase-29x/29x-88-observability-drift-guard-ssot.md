---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X61 observability drift guard（5 root categories）を guard + gate で固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-30-observability-extension-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-87-rc-insertion-phase2-queue-lock-ssot.md
  - docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md
  - src/runtime/leak_tracker.rs
  - tools/checks/phase29x_observability_categories.txt
  - tools/checks/phase29x_observability_drift_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_observability_drift_guard_vm.sh
---

# 29x-88: Observability Drift Guard (SSOT)

## 0. Goal

- X61 として root surface 5カテゴリ（`handles/locals/temps/heap_fields/singletons`）の drift を guard で fail-fast 検出する。
- 既存 observability smokes（X14-X17）を single-entry gate へ統合し、再生手順を 1コマンド化する。
- runtime core lane 順序を崩さないため、X60 RC queue gate を前提 step に固定する。

## 1. Category inventory SSOT

Inventory source of truth:
- `tools/checks/phase29x_observability_categories.txt`

Fixed order:
1. `handles`
2. `locals`
3. `temps`
4. `heap_fields`
5. `singletons`

## 2. Contract

1. category inventory は 5件ちょうど、重複なし、固定順（上記）を維持する。
2. `src/runtime/leak_tracker.rs` は 5カテゴリすべての stable output line を持つ。
3. `phase29x_observability_summary_vm.sh` は category inventory を参照し、順序・一意性を検証する。
4. X61 gate は X60 precondition（`phase29x_rc_phase2_queue_lock_vm.sh`）を前提実行する。
5. X61 gate は X14-X17 observability smokes（temps/heap_fields/singletons/summary）を再生し、5カテゴリ契約の drift を検出する。

## 3. Integration gate

- Guard:
  - `tools/checks/phase29x_observability_drift_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_observability_drift_guard_vm.sh`

Gate steps:
1. X61 guard（inventory/leak_tracker/summary-smoke/gate wiring）
2. X60 precondition（RC phase2 queue single-entry gate）
3. observability replay（X14-X17 smokes）

## 4. Evidence command

- `bash tools/checks/phase29x_observability_drift_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_observability_drift_guard_vm.sh`

## 5. Next step

- X62 は `29x-89-runtime-core-integrated-gate-ssot.md` で完了。
- X63 は `29x-92-optimization-allowlist-lock-ssot.md` で完了。
- 次タスクは X64（optimization parity fixtures/reject fixtures）。
