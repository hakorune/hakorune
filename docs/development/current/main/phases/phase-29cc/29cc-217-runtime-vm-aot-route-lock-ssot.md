---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: execution-path-zero 運用で VM+AOT の kilo 主要ルートを同時監査する lock を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-215-runtime-execution-path-observability-lock-ssot.md
  - tools/checks/phase29cc_runtime_vm_aot_route_lock_guard.sh
  - tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_text_concat_contract_vm.sh
  - tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-217 Runtime VM+AOT Route Lock

## Purpose

`execution-path-zero` の日常運用で、kilo の VM+AOT ルートドリフトを同時に止める。

## Guard contract

1. 正本 guard は `tools/checks/phase29cc_runtime_vm_aot_route_lock_guard.sh` とする。
2. guard は次の2本を実行し、両方 green を要求する。
   - `phase21_5_perf_kilo_text_concat_contract_vm.sh`
   - `phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh`
3. 日常実行は `tools/checks/dev_gate.sh runtime-exec-zero` に統合する。

## Acceptance

1. `phase29cc_runtime_vm_aot_route_lock_guard.sh` が green。
2. `dev_gate.sh runtime-exec-zero` が green。
3. `CURRENT_TASK.md` / `10-Now.md` / `phase-29cc/README.md` に lock 参照が同期されている。

## Not in this lock

1. 観測タグ契約（`[runtime/exec-path]`）の定義更新
2. plugin loader 経路の機能置換
3. runtime V0 helper 語彙追加
