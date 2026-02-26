---
Status: Active
Decision: provisional
Date: 2026-02-13
Scope: X56 vm-hako dual-run parity gate pack（success/reject split）を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-81-vm-hako-s6-vocabulary-inventory-guard-ssot.md
  - tools/checks/phase29x_vm_hako_s6_parity_gate_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_parity_gate_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_vocab_guard_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_array_get_parity_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_array_set_parity_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_await_non_future_reject_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_newclosure_probe_vm.sh
---

# 29x-82: VM-Hako S6 Dual-Run Parity Gate Pack (SSOT)

## 0. Goal

- success/reject の parity probe を 1 コマンドで再生可能にする。
- gate wiring 欠落を guard で fail-fast 検出する。
- X57（NewClosure lane decision）の前提として S6 parity の土台を lock する。

## 1. Integration gate

- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_parity_gate_vm.sh`
- Guard:
  - `tools/checks/phase29x_vm_hako_s6_parity_gate_guard.sh`

Integrated steps:
1. `phase29x_vm_hako_s6_vocab_guard_vm.sh`（X55 inventory drift guard）
2. `phase29z_vm_hako_s5_array_get_parity_vm.sh`（success parity, route pin helper 経由）
3. `phase29z_vm_hako_s5_array_set_parity_vm.sh`（success parity, route pin helper 経由）
4. `phase29z_vm_hako_s5_await_non_future_reject_vm.sh`（reject parity, route pin helper 経由）
5. `phase29z_vm_hako_s5_newclosure_probe_vm.sh`（reject fail-fast contract）

## 2. Contract

1. gate は上記 5 step を固定順で実行する。
2. どれか 1 step でも失敗したら gate 全体は fail-fast する。
3. wiring 欠落（step 削除/置換）は guard が fail-fast する。
4. S5 parity/reject step（2-4）は route drift 防止のため `run_with_vm_route_pin` helper 経由で固定実行する。

## 3. Evidence command

- `bash tools/checks/phase29x_vm_hako_s6_parity_gate_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_parity_gate_vm.sh`

## 4. Next step

- X57: `new_closure` の runtime lane decision（execute/fail-fast boundary）を Decision 明記で固定する。
