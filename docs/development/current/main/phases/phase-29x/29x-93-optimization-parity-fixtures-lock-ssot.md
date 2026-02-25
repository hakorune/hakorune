---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X64 optimization parity fixtures/reject fixture を guard + gate で固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-92-optimization-allowlist-lock-ssot.md
  - apps/tests/phase29x_optimization_parity_const_fold_min.hako
  - apps/tests/phase29x_optimization_parity_cfg_min.hako
  - apps/tests/phase29x_optimization_reject_div_zero_min.hako
  - tools/checks/phase29x_optimization_parity_fixtures.txt
  - tools/checks/phase29x_optimization_reject_fixtures.txt
  - tools/checks/phase29x_optimization_parity_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_optimization_parity_fixtures_vm.sh
---

# 29x-93: Optimization Parity Fixtures Lock (SSOT)

## 0. Goal

- X64 として optimization pre/post（optimize ON/OFF）の parity を fixture で固定する。
- success fixture と reject fixture を分離し、どちらも pre/post parity で drift を検出できるようにする。
- X63 allowlist lock を前提にし、optimization lane の順序を崩さない。

## 1. Fixture inventories (SSOT)

Parity fixtures source of truth:
- `tools/checks/phase29x_optimization_parity_fixtures.txt`

Fixed cases:
1. `apps/tests/phase29x_optimization_parity_const_fold_min.hako`（expected rc=`6`, stdout=`6`）
2. `apps/tests/phase29x_optimization_parity_cfg_min.hako`（expected rc=`42`, stdout=`42`）

Reject fixtures source of truth:
- `tools/checks/phase29x_optimization_reject_fixtures.txt`

Fixed case:
1. `apps/tests/phase29x_optimization_reject_div_zero_min.hako`（optimize ON/OFF とも non-zero）
   - expected failure text: `Division by zero`

## 2. Contract

1. parity fixtures は optimize ON/OFF の `rc` と normalized stdout が一致する。
2. parity fixtures は inventory に固定した期待値（`expected_rc` / `expected_stdout`）を満たす。
3. reject fixture は optimize ON/OFF の両方で non-zero かつ failure text `Division by zero` を含む。
4. reject fixture は optimize ON/OFF で normalized failure output が一致する。
5. X64 gate は X63 precondition（`phase29x_optimization_allowlist_lock_vm.sh`）を前提実行する。

## 3. Integration gate

- Guard:
  - `tools/checks/phase29x_optimization_parity_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_optimization_parity_fixtures_vm.sh`

Gate steps:
1. X64 guard（fixtures/docs/gate wiring）
2. X63 precondition（optimization allowlist lock gate）
3. parity fixtures replay（optimize ON/OFF）
4. reject fixture replay（optimize ON/OFF, non-zero + `Division by zero`）

## 4. Evidence command

- `bash tools/checks/phase29x_optimization_parity_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_optimization_parity_fixtures_vm.sh`

## 5. Next step

- X65: optimization gate integration + rollback lock を固定する。
