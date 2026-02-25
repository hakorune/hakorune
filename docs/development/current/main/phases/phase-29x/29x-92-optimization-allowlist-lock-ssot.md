---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X63 optimization allowlist lock（const_fold / dce / cfg_simplify）を guard + gate で固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-89-runtime-core-integrated-gate-ssot.md
  - src/mir/optimizer.rs
  - tools/checks/phase29x_optimization_allowlist.txt
  - tools/checks/phase29x_optimization_allowlist_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_optimization_allowlist_lock_vm.sh
---

# 29x-92: Optimization Allowlist Lock (SSOT)

## 0. Goal

- X63 として optimization safe-set 語彙を `const_fold / dce / cfg_simplify` の 3項目へ固定する。
- vocabulary drift（追加/削除/順序変更）を guard で fail-fast 検出する。
- runtime core lane 順序を維持するため、X62 integrated gate を前提 step に固定する。

## 1. Allowlist SSOT

Allowlist source of truth:
- `tools/checks/phase29x_optimization_allowlist.txt`

Fixed order:
1. `const_fold`
2. `dce`
3. `cfg_simplify`

Code-side lock:
- `src/mir/optimizer.rs` の `PHASE29X_OPT_SAFESET`
- `mir_optimizer_phase29x_allowlist_lock` テスト

## 2. Contract

1. allowlist は 3件ちょうど、重複なし、固定順を維持する。
2. `src/mir/optimizer.rs` の `PHASE29X_OPT_SAFESET` は allowlist と同一語彙を持つ。
3. X63 gate は X62 precondition（`phase29x_runtime_core_gate_vm.sh`）を前提実行する。
4. X63 gate は `cargo test -q mir_optimizer_phase29x_allowlist_lock -- --nocapture` を実行し、語彙ロックを継続検証する。

## 3. Integration gate

- Guard:
  - `tools/checks/phase29x_optimization_allowlist_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_optimization_allowlist_lock_vm.sh`

Gate steps:
1. X63 guard（allowlist/docs/code/gate wiring）
2. X62 precondition（runtime core integrated gate）
3. allowlist cargo test

## 4. Evidence command

- `bash tools/checks/phase29x_optimization_allowlist_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_optimization_allowlist_lock_vm.sh`

## 5. Next step

- X64: optimization parity fixtures + reject fixtures を固定する。
