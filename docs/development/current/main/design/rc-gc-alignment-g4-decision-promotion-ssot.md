---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: RC/GC Alignment G-RC-4（Decision 昇格 + rollback note）を固定する。
Related:
  - docs/development/current/main/20-Decisions.md
  - CURRENT_TASK.md
  - docs/reference/language/lifecycle.md
  - docs/development/current/main/design/rc-gc-alignment-g1-lifecycle-parity-ssot.md
  - docs/development/current/main/design/rc-gc-alignment-g2-fast-milestone-gate-ssot.md
  - docs/development/current/main/design/rc-gc-alignment-g3-cycle-explicit-drop-ssot.md
---

# RC/GC Alignment G-RC-4: Decision Promotion + Rollback Note (SSOT)

## 0. Goal

- G-RC-1..3 完了を受けて RC/GC alignment Decision を `provisional` から `accepted` へ昇格する。
- rollback を「いつ・どう戻すか」で明文化し、運用迷走を防ぐ。

## 1. Promotion basis

1. G-RC-1 lock:
   - `bash tools/checks/rc_gc_alignment_g1_guard.sh`
   - `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g1_lifecycle_parity_vm_llvm.sh`
2. G-RC-2 lock:
   - `bash tools/checks/rc_gc_alignment_g2_gate_matrix_guard.sh`
   - `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g2_fast_milestone_gate.sh`
3. G-RC-3 lock:
   - `bash tools/checks/rc_gc_alignment_g3_cycle_timing_guard.sh`
   - `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g3_cycle_timing_gate.sh`

## 2. Accepted contract

1. MIR は lifecycle intent（`keepalive`/`release_strong`）のみを表し、numeric refcount policy は持たない。
2. Runtime/Kernel が retain/release/final drop の唯一責務である。
3. GC は optional（意味論要件ではない）で、ON/OFF で program meaning は不変。
4. weak/strong cycle と explicit-drop timing は fixture + gate で固定し続ける。

## 3. Rollback note (required)

Decision status を `provisional` に戻す条件:
1. G-RC-1/G-RC-2/G-RC-3 のいずれかが連続して FAIL し、24時間以内に再現修復できない。
2. lifecycle boundary（MIR intent vs runtime ownership）を破る drift が検出された。
3. RC/GC 意味論に影響する仕様変更が未合意で先行した。

Rollback 手順:
1. `docs/development/current/main/20-Decisions.md` の status を `provisional` に戻す。
2. `CURRENT_TASK.md` の G-RC チェックを FAIL した項目まで戻す。
3. 失敗点を `CURRENT_TASK.md` に blocker として固定し、failure-driven 1件修復へ切り替える。
