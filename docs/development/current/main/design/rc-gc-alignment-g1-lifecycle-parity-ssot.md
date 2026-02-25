---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: RC/GC Alignment G-RC-1（VM/LLVM lifecycle parity fixtures stable）を inventory + guard + gate で固定する。
Related:
  - docs/reference/language/lifecycle.md
  - CURRENT_TASK.md
  - docs/development/current/main/20-Decisions.md
  - tools/checks/rc_gc_alignment_g1_lifecycle_cases.txt
  - tools/checks/rc_gc_alignment_g1_guard.sh
  - tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g1_lifecycle_parity_vm_llvm.sh
---

# RC/GC Alignment G-RC-1: Lifecycle Parity Fixtures Lock (SSOT)

## 0. Goal

- `CURRENT_TASK.md` の G-RC-1 を実装タスクとして固定する。
- lifecycle の代表 fixture で VM/LLVM parity（exit-code semantics）を継続検証する。
- RCの責務分離 Decision（provisional）を崩さず、backend drift を fail-fast 検出する。

## 1. Lifecycle parity inventory

Source of truth:
- `tools/checks/rc_gc_alignment_g1_lifecycle_cases.txt`

Fixed cases:
1. `rc_explicit_drop_min`
   - fixture: `apps/tests/phase29x_rc_explicit_drop_min.hako`
   - expected exit: `0`
2. `weak_upgrade_success`
   - fixture: `apps/tests/phase285_weak_basic.hako`
   - expected exit: `2`
3. `weak_upgrade_fail_after_drop`
   - fixture: `apps/tests/phase285_p2_weak_upgrade_fail_min.hako`
   - expected exit: `1`

## 2. Contract

1. VM と LLVM は全caseで expected exit を満たす。
2. VM と LLVM は同一caseで exit code parity を満たす。
3. parity contract は inventory 駆動で実行し、個別スクリプト依存を増やさない。
4. LLVM backend 非対応ビルドでは gate は `SKIP` とし、誤検知を防ぐ。

## 3. Gate

- Guard:
  - `tools/checks/rc_gc_alignment_g1_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g1_lifecycle_parity_vm_llvm.sh`

Gate steps:
1. guard（inventory/docs/gate wiring）
2. VM/LLVM execution per case
3. expected exit + parity check

## 4. Evidence command

- `bash tools/checks/rc_gc_alignment_g1_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g1_lifecycle_parity_vm_llvm.sh`

## 5. Next

- G-RC-2: fast gate + milestone regression suites に RC非依存前提の検証を追加する。
