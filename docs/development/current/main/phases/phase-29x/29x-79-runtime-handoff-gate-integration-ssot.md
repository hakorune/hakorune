---
Status: Active
Decision: provisional
Date: 2026-02-13
Scope: X52 の runtime handoff gate integration（X48-X51 one-command replay）を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-75-vm-route-pin-inventory-guard-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-76-vm-hako-strict-dev-replay-gate-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-77-newclosure-contract-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-78-core-cabi-delegation-inventory-guard-ssot.md
  - tools/checks/phase29x_runtime_handoff_gate_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_runtime_handoff_gate_vm.sh
---

# 29x-79: Runtime Handoff Gate Integration (SSOT)

## 0. Goal

- X48-X51 の契約を 1 コマンドで再生可能にする。
- gate wiring の欠落を guard で fail-fast 検出する。

## 1. Integration gate

- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_handoff_gate_vm.sh`
- Guard:
  - `tools/checks/phase29x_runtime_handoff_gate_guard.sh`

Integrated steps:
1. `phase29x_vm_route_pin_guard_vm.sh`（X48）
2. `phase29x_vm_hako_strict_dev_replay_vm.sh`（X49）
3. `phase29x_vm_hako_newclosure_contract_vm.sh`（X50）
4. `phase29x_core_cabi_delegation_guard_vm.sh`（X51）

## 2. Contract

1. gate は上記 4 step を固定順で実行する。
2. どれか 1 step でも失敗したら gate 全体は fail-fast する。
3. wiring 欠落（step削除/置換）は guard が fail-fast する。

## 3. Evidence command

- `bash tools/checks/phase29x_runtime_handoff_gate_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_handoff_gate_vm.sh`

