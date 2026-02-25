---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X62 runtime core integrated gate（ABI + RC + observability）を single-entry で固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-86-abi-borrowed-owned-conformance-extension-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-87-rc-insertion-phase2-queue-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-88-observability-drift-guard-ssot.md
  - tools/checks/phase29x_runtime_core_gate_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_runtime_core_gate_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_abi_borrowed_owned_conformance_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_rc_phase2_queue_lock_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_observability_drift_guard_vm.sh
---

# 29x-89: Runtime Core Integrated Gate (SSOT)

## 0. Goal

- X62 として runtime core hardening（X59 ABI / X60 RC / X61 observability）を 1つの gate へ統合する。
- 再開時の導線を 1コマンド化し、runtime core lane の replay 手順を固定する。
- wiring 漏れは guard で fail-fast 検出する。

## 1. Contract

1. `phase29x_runtime_core_gate_vm.sh` は次を順序固定で実行する。
   1. `phase29x_runtime_core_gate_guard.sh`
   2. `phase29x_abi_borrowed_owned_conformance_vm.sh`（X59）
   3. `phase29x_rc_phase2_queue_lock_vm.sh`（X60）
   4. `phase29x_observability_drift_guard_vm.sh`（X61）
2. いずれかの step が失敗した場合、integrated gate は即失敗する（silent skip しない）。
3. X62 guard は dependency gate と SSOT wiring を検証する。

## 2. Integration gate

- Guard:
  - `tools/checks/phase29x_runtime_core_gate_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_core_gate_vm.sh`

## 3. Evidence command

- `bash tools/checks/phase29x_runtime_core_gate_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_core_gate_vm.sh`

## 4. Next step

- X63 は `29x-92-optimization-allowlist-lock-ssot.md` で完了。
- 次タスクは X64（optimization parity fixtures/reject fixtures）。
