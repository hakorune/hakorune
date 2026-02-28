---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: RZ-ARRAY-min1 として RuntimeDataBox array mono-route 判断を 1 箇所へ集約し、挙動不変で切替可能境界を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-245-runtime-route-residue-relock-ssot.md
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
---

# 29cc-246 RZ-ARRAY-min1 Route Selector Lock

## Purpose

`nyash.array.get_hi/set_hii` 直ルートの切替作業を安全に進めるため、  
RuntimeDataBox lowering の symbol 選択を単一関数へ集約する。

## Decision

- 新規 selector `select_runtime_data_call_spec()` を SSOT とする。
- default は現行維持（array mono-route 優先）。
- caller が `prefer_array_mono_route=False` を渡した時のみ runtime_data-only route を使う。
- 本minでは route既定値を変更しない（挙動不変）。

## Acceptance

- 既存の auto-specialize 経路は不変（default behavior drift なし）。
- 新規 unit test で下記を固定:
  - default: `set` + i64 hints -> `nyash.array.set_hii`
  - runtime_data-only: 同条件 -> `nyash.runtime_data.set_hhh`
  - 非 RuntimeDataBox は `None`
