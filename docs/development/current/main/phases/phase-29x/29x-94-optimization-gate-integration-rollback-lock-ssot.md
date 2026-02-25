---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X65 optimization gate integration + rollback lock を guard + gate で固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-92-optimization-allowlist-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-93-optimization-parity-fixtures-lock-ssot.md
  - tools/checks/phase29x_optimization_gate_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_optimization_gate_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_optimization_allowlist_lock_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_optimization_parity_fixtures_vm.sh
  - apps/tests/phase29x_optimization_parity_const_fold_min.hako
---

# 29x-94: Optimization Gate Integration + Rollback Lock (SSOT)

## 0. Goal

- X65 として optimization lane（X63 + X64）を single-entry gate へ統合する。
- rollback switch を docs と gate で固定し、回帰時の戻し経路を可観測にする。
- allowlist drift と parity drift を同時に再生できる入口を 1 コマンドに限定する。

## 1. Single-entry integration gate

- Guard:
  - `tools/checks/phase29x_optimization_gate_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_optimization_gate_vm.sh`

Fixed replay order:
1. X65 guard（wiring + rollback contract）
2. X63 gate（`phase29x_optimization_allowlist_lock_vm.sh`）
3. X64 gate（`phase29x_optimization_parity_fixtures_vm.sh`）
4. rollback probe（`--no-optimize` + `phase29x_optimization_parity_const_fold_min.hako`）

## 2. Rollback lock contract

1. rollback switch は `--no-optimize` のみを使う（silent fallback 禁止）。
2. rollback probe は optimize OFF 実行で expected rc=6 / stdout=`6` を満たす。
3. rollback 実施時でも X63/X64 gate は同じ single-entry 順序で replay する。
4. rollback は緊急運用手段であり、allowlist/parity fixture SSOT を改変してはならない。

## 3. Evidence command

- `bash tools/checks/phase29x_optimization_gate_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_optimization_gate_vm.sh`

## 4. Next step

- X66: optional GC lane bootstrap（docs-only, semantics unchanged）を固定する。
