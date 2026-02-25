---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: RC/GC Alignment G-RC-3（weak/strong cycle と explicit-drop timing）を matrix + guard + single-entry gate で固定する。
Related:
  - docs/reference/language/lifecycle.md
  - docs/development/current/main/design/rc-gc-alignment-g1-lifecycle-parity-ssot.md
  - docs/development/current/main/design/rc-gc-alignment-g2-fast-milestone-gate-ssot.md
  - CURRENT_TASK.md
  - docs/development/current/main/20-Decisions.md
  - tools/checks/rc_gc_alignment_g3_cycle_timing_cases.txt
  - tools/checks/rc_gc_alignment_g3_cycle_timing_guard.sh
  - tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g3_cycle_timing_gate.sh
---

# RC/GC Alignment G-RC-3: Weak/Strong Cycle + Explicit-Drop Timing Lock (SSOT)

## 0. Goal

- `CURRENT_TASK.md` の G-RC-3 を実装タスクとして固定する。
- weak/strong cycle と explicit-drop timing の代表 fixture を gate matrix で固定する。
- G-RC-1/G-RC-2 の前提を崩さず、lifecycle 意味論と timing 観測の drift を fail-fast 検出する。

## 1. Cycle/timing matrix inventory

Source of truth:
- `tools/checks/rc_gc_alignment_g3_cycle_timing_cases.txt`

Fixed matrix:
1. `g1_weak_and_drop_parity` (`weak_and_drop`)
   - gate: `tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g1_lifecycle_parity_vm_llvm.sh`
   - coverage: weak upgrade success/fail + explicit drop parity
2. `strong_cycle_observability` (`strong_cycle`)
   - gate: `tools/smokes/v2/profiles/integration/apps/phase29x_observability_summary_vm.sh`
   - coverage: strong cycle fixture (`apps/tests/phase285_leak_report.hako`) の root category 観測
3. `explicit_drop_release_timing` (`explicit_drop_timing`)
   - gate: `tools/smokes/v2/profiles/integration/apps/phase29x_rc_explicit_drop_vm.sh`
   - coverage: `x = null` で `release_strong` 挿入タイミングを固定
4. `scope_end_release_timing` (`scope_end_timing`)
   - gate: `tools/smokes/v2/profiles/integration/apps/phase29x_rc_scope_end_release_vm.sh`
   - coverage: return 終端 cleanup の `release_strong` タイミングを固定

## 2. Contract

1. matrix は `weak_and_drop` / `strong_cycle` / `explicit_drop_timing` / `scope_end_timing` を最低1件ずつ含む。
2. matrix は `g1_weak_and_drop_parity` を必須依存として持つ（G-RC-1を再利用）。
3. strong cycle の扱いは lifecycle SSOT どおり「collector無しではリークしうる」を許容し、観測契約で固定する。
4. explicit-drop / scope-end timing は RC insertion smoke で `release_strong` 契約を固定する。

## 3. Integration gate

- Guard:
  - `tools/checks/rc_gc_alignment_g3_cycle_timing_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g3_cycle_timing_gate.sh`

Gate steps:
1. guard（matrix/docs/gate wiring）
2. weak/drop parity replay（G-RC-1）
3. strong-cycle observability replay
4. explicit-drop / scope-end timing replay

## 4. Evidence command

- `bash tools/checks/rc_gc_alignment_g3_cycle_timing_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g3_cycle_timing_gate.sh`

## 5. Next

- G-RC-4: RC/GC alignment decision を `provisional` から `accepted` に昇格し、rollback note を固定する。
