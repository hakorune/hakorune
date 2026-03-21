---
Status: Active
Decision: accepted
Date: 2026-02-14
Scope: RC/GC Alignment G-RC-2（fast gate + milestone regression suites）を matrix + guard + single-entry gate で固定する。
Related:
  - docs/development/current/main/design/rc-gc-alignment-g1-lifecycle-parity-ssot.md
  - docs/development/current/main/design/rc-gc-alignment-g3-cycle-explicit-drop-ssot.md
  - docs/development/current/main/design/rc-gc-alignment-g5-gc-mode-semantics-invariance-ssot.md
  - CURRENT_TASK.md
  - docs/development/current/main/20-Decisions.md
  - tools/checks/rc_gc_alignment_g2_gate_matrix_cases.txt
  - tools/checks/rc_gc_alignment_g2_gate_matrix_guard.sh
  - tools/smokes/v2/profiles/integration/rc_gc_alignment/rc_gc_alignment_g2_fast_milestone_gate.sh
---

# RC/GC Alignment G-RC-2: Fast/Milestone Gate Matrix (SSOT)

## 0. Goal

- `CURRENT_TASK.md` の G-RC-2 を実装タスクとして固定する。
- fast gate と milestone regression suites を 1コマンドで再生し、RC前提の特別運用を増やさない。
- G-RC-1 で固定した lifecycle parity を前提 step として常時再検証する。

## 1. Gate matrix inventory

Source of truth:
- `tools/checks/rc_gc_alignment_g2_gate_matrix_cases.txt`

Fixed matrix:
1. `g1_lifecycle_parity` (fast)
   - gate: `tools/smokes/v2/profiles/integration/rc_gc_alignment/rc_gc_alignment_g1_lifecycle_parity_vm_llvm.sh`
2. `runtime_core_integrated` (fast)
   - gate: `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_core_gate_vm.sh`
3. `optimization_gate_regression` (milestone)
   - gate: `tools/smokes/v2/profiles/integration/apps/phase29x_optimization_gate_vm.sh`
4. `g5_gc_mode_semantics_invariance` (milestone)
   - gate: `tools/smokes/v2/profiles/integration/rc_gc_alignment/rc_gc_alignment_g5_mode_invariance_vm_llvm.sh`
5. `g3_cycle_timing_matrix` (milestone)
   - gate: `tools/smokes/v2/profiles/integration/rc_gc_alignment/rc_gc_alignment_g3_cycle_timing_gate.sh`

## 2. Contract

1. matrix には `fast` を最低1件、`milestone` を最低1件含む。
2. matrix は必ず `g1_lifecycle_parity` を含む（G-RC-1 依存を明示固定）。
3. matrix は `g3_cycle_timing_matrix` を含み、weak/strong cycle + explicit-drop timing 契約（G-RC-3）を replay する。
4. matrix は `g5_gc_mode_semantics_invariance` を含み、GC mode invariance の post-promotion hardening を replay する。
5. G-RC-2 gate は matrix 駆動で step を実行し、個別の ad-hoc 配線を増やさない。
6. 既存 gate の実行条件（例: LLVM lane の SKIP/FAIL-fast）はそれぞれの SSOT に委譲し、G-RC-2 側で上書きしない。

## 3. Integration gate

- Guard:
  - `tools/checks/rc_gc_alignment_g2_gate_matrix_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/rc_gc_alignment/rc_gc_alignment_g2_fast_milestone_gate.sh`

Gate steps:
1. guard（matrix/docs/gate wiring）
2. matrix replay（fast + milestone）
3. all-green contract lock

## 4. Evidence command

- `bash tools/checks/rc_gc_alignment_g2_gate_matrix_guard.sh`
- `bash tools/smokes/v2/profiles/integration/rc_gc_alignment/rc_gc_alignment_g2_fast_milestone_gate.sh`

## 5. Next

- G-RC-3: weak/strong cycle と explicit-drop timing fixture を docs + gate で固定する。
