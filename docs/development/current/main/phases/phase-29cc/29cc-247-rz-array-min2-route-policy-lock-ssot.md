---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: RZ-ARRAY-min2 として RuntimeDataBox array-route を policy 化し、default 不変で切替境界を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-246-rz-array-min1-route-selector-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-217-runtime-vm-aot-route-lock-ssot.md
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
---

# 29cc-247 RZ-ARRAY-min2 Route Policy Lock

## Purpose

`RZ-ARRAY-min1` で作った selector 境界を使い、default route を変えずに policy 切替を可能にする。

## Decision

- ENV SSOT: `NYASH_RUNTIME_DATA_ARRAY_ROUTE_POLICY`
- 受理値:
  - `array_mono`（default, 既存挙動維持）
  - `runtime_data_only`（明示切替）
- 無効値は fail-fast（`RuntimeError`）で停止する。
- `29cc-217` route lock との整合のため、default は `array_mono` のまま固定する。

## Acceptance

- default 実行で `phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh` が green。
- 新規 unit test で policy parse と fail-fast を固定:
  - default: array_mono
  - runtime_data_only: false
  - invalid: RuntimeError
