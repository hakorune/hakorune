---
Status: Active
Decision: accepted
Date: 2026-02-14
Scope: RC/GC Alignment G-RC-5（GC mode ON/OFF semantics invariance）を inventory + guard + gate で固定する。
Related:
  - docs/reference/language/lifecycle.md
  - docs/reference/runtime/gc.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - docs/development/current/main/design/rc-gc-alignment-g1-lifecycle-parity-ssot.md
  - docs/development/current/main/design/rc-gc-alignment-g2-fast-milestone-gate-ssot.md
  - docs/development/current/main/design/rc-gc-alignment-g4-decision-promotion-ssot.md
  - CURRENT_TASK.md
  - docs/development/current/main/20-Decisions.md
  - tools/checks/rc_gc_alignment_g5_mode_invariance_cases.txt
  - tools/checks/rc_gc_alignment_g5_mode_invariance_guard.sh
  - tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g5_mode_invariance_vm_llvm.sh
---

# RC/GC Alignment G-RC-5: GC Mode Semantics Invariance Lock (SSOT)

## 0. Goal

- `runtime-gc-policy-and-order-ssot.md` の non-normative profile（beginner/expert）を実行可能 gate へ固定する。
- `rc+cycle`（beginner）と `off`（expert）で意味論を変えない契約を、VM/LLVM 両backendで fail-fast 検証する。
- G-RC-1..4 accepted 契約を維持したまま、GC optional ポリシーの運用 drift を短距離で検出する。

## 1. Mode invariance inventory

Source of truth:
- `tools/checks/rc_gc_alignment_g5_mode_invariance_cases.txt`

Fixed cases:
1. `gc_mode_explicit_drop` (`explicit_drop`)
   - fixture: `apps/tests/phase29x_rc_explicit_drop_min.hako`
   - expected exit: `0`
2. `gc_mode_scope_end_release` (`scope_end_timing`)
   - fixture: `apps/tests/phase29x_rc_scope_end_release_min.hako`
   - expected exit: `0`
3. `gc_mode_weak_upgrade_success` (`weak_success`)
   - fixture: `apps/tests/phase285_weak_basic.hako`
   - expected exit: `2`
4. `gc_mode_weak_upgrade_fail` (`weak_fail`)
   - fixture: `apps/tests/phase285_p2_weak_upgrade_fail_min.hako`
   - expected exit: `1`

## 2. Contract

1. 各 case で `NYASH_GC_MODE=rc+cycle` と `NYASH_GC_MODE=off` の exit code が一致する。
2. 各 case で VM/LLVM backend parity を mode ごとに満たす。
3. mode invariance gate は inventory 駆動で実行し、ad-hoc case 配線を増やさない。
4. LLVM backend 非対応ビルドでは gate は `SKIP` とし、検証不能状態を誤って PASS 扱いしない。

## 3. Gate

- Guard:
  - `tools/checks/rc_gc_alignment_g5_mode_invariance_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g5_mode_invariance_vm_llvm.sh`

Gate steps:
1. guard（inventory/docs/gate wiring）
2. per case replay:
   - VM rc+cycle
   - VM off
   - LLVM rc+cycle
   - LLVM off
3. expected exit + mode invariance + backend parity check

## 4. Evidence command

- `bash tools/checks/rc_gc_alignment_g5_mode_invariance_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g5_mode_invariance_vm_llvm.sh`

## 5. Note

- G-RC-5 は G-RC-1..4 accepted 契約の post-promotion hardening であり、Decision promotion 手順自体は変更しない。
- G-RC-2 matrix は `g5_gc_mode_semantics_invariance` を milestone replay として含み、single-entry 再生を担保する。
