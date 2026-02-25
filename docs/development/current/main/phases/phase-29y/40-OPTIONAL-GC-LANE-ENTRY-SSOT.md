# Phase 29y: Optional GC Lane Entry SSOT（semantics unchanged）

Status: Active (Y3 queue fixed, min1/min2/min3 done)
Scope: optional GC 実装レーンへ進むための入口条件・順序・非目標を固定する。

## 0. Goal

- optional GC 実装を「意味論不変」の契約下でのみ進める。
- ABI / RC insertion / observability の既存 SSOT を前提にし、順序逆転を防ぐ。
- G-RC alignment の gate 証跡を phase entry 条件に固定する。

## 1. Entry preconditions (must pass)

1. RC/GC matrix single-entry:
   - `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g2_fast_milestone_gate.sh`
   - matrix includes:
     - `g1_lifecycle_parity`
     - `g3_cycle_timing_matrix`
     - `g5_gc_mode_semantics_invariance`
2. GC mode semantics invariance:
   - `bash tools/checks/rc_gc_alignment_g5_mode_invariance_guard.sh`
   - `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g5_mode_invariance_vm_llvm.sh`

## 2. Fixed order (do not reorder)

1. Keep lifecycle boundary:
   - MIR = intent only
   - Runtime/Kernel = ownership truth
2. Keep ABI contract:
   - args borrowed / return owned
3. Keep RC insertion SSOT:
   - retain/release/weak_drop insertion points stay single-source
4. Keep observability root surface:
   - locals/temps/heap_fields/handles/singletons
5. Optional GC implementation tasks (future phase):
   - algorithm/collector improvements are allowed only as timing/diagnostics changes

## 3. Non-goals

- lifecycle language semantics update
- borrowed/owned ABI change
- RC insertion ownership split across multiple passes
- making GC mandatory for program correctness

## 4. Acceptance contract

1. `NYASH_GC_MODE=rc+cycle|off` does not change program meaning (exit semantics on pinned fixtures).
2. Backend parity (VM/LLVM) remains locked for lifecycle fixtures.
3. Any optional GC implementation drift is detected by existing RC/GC matrix gates before promotion.

## 5. Related SSOT

- `docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md`
- `docs/development/current/main/design/rc-gc-alignment-g2-fast-milestone-gate-ssot.md`
- `docs/development/current/main/design/rc-gc-alignment-g5-gc-mode-semantics-invariance-ssot.md`
- `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md`
- `docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md`
- `docs/development/current/main/phases/phase-29y/30-OBSERVABILITY-SSOT.md`

## 6. Integration gate

- Guard:
  - `tools/checks/phase29y_optional_gc_lane_entry_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_optional_gc_lane_entry_vm.sh`

Gate steps:
1. guard（docs/gate/matrix wiring）
2. RC/GC alignment single-entry replay（G-RC-2 matrix）

## 7. Evidence command

- `bash tools/checks/phase29y_optional_gc_lane_entry_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_optional_gc_lane_entry_vm.sh`

## 8. Y3 implementation queue (docs-first, fixed 2026-02-16)

Decision: provisional
Execution unit: `1 min task = 1 commit = fixture/gate pin`

### 8.1 Non-negotiable contract

1. Semantics invariance:
   - `NYASH_GC_MODE=rc+cycle|off` でプログラム意味論を変えない。
2. ABI fixed:
   - function ABI は `args borrowed / return owned` のまま固定する。
3. RC insertion single-source:
   - retain/release/weak_drop の挿入責務は 1 箇所（RC insertion pass）から分散させない。

### 8.2 Queue (fixed order)

1. `min1` GC mode boundary lock（wiring only, no behavior change）
   - Status:
     - [done] 2026-02-16
   - Scope:
     - optional GC mode の導線を 1 入口に固定し、既定挙動（release/CI）は不変。
   - Forbidden:
     - collector algorithm の新規導入、lifecycle semantics の変更。
   - Result:
     - CLI/ENV の受理値を `auto|rc+cycle|off` に固定し、`minorgen/stw/rc` は fail-fast へ統一。
   - Acceptance:
     - `bash tools/checks/phase29y_optional_gc_lane_entry_guard.sh`
     - `bash tools/checks/rc_gc_alignment_g5_mode_invariance_guard.sh`
     - `bash tools/smokes/v2/profiles/integration/apps/phase29y_optional_gc_lane_entry_vm.sh`
2. `min2` Optional GC observability pin（dev/diagnostic only）
   - Status:
     - [done] 2026-02-16
   - Scope:
     - optional GC 時の診断観測点を固定（既定OFF、安定タグ、1行）。
   - Forbidden:
     - user-visible output 変更、既定ON 化、意味論に依存する分岐。
   - Result:
     - `NYASH_GC_METRICS=1` かつ `NYASH_GC_MODE=rc+cycle` でのみ `[gc/optional:mode] mode=rc+cycle collect_sp=<...> collect_alloc=<...>` を出す契約を固定（既定OFF）。
     - `phase29y_observability_summary_vm.sh` で metrics OFF 時にタグ不在、metrics ON 時にタグ出現を pin。
   - Acceptance:
     - `bash tools/smokes/v2/profiles/integration/apps/phase29y_observability_summary_vm.sh`
     - `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`
3. `min3` Optional GC pilot execution（guarded rollout）
   - Status:
     - [done] 2026-02-16
   - Scope:
     - optional mode 限定で最小 pilot を導入し、rollback 可能な粒度で固定する。
   - Forbidden:
     - ABI 変更、RC insertion の多重化、GC mandatory 化。
   - Result:
     - `rc_gc_alignment_g2_fast_milestone_gate.sh` と `phase29y_lane_gate_vm.sh` の single-entry replay を固定し、non-negotiable 契約の drift 検出を rollout 導線に統合。
   - Acceptance:
     - `bash tools/smokes/v2/profiles/integration/apps/rc_gc_alignment_g2_fast_milestone_gate.sh`
     - `bash tools/smokes/v2/profiles/integration/apps/phase29y_lane_gate_vm.sh`

### 8.3 Rollback / fail-fast

1. `G-RC-2` / `phase29y_lane_gate_vm.sh` のいずれかが FAIL:
   - まず原因コミットを revert し、queue を次の min へ進めない。
2. 60 分以内に復旧できない:
   - `CURRENT_TASK.md` に詰まりメモを固定して docs-first へ戻す。
3. Non-negotiable violation（ABI drift / semantics drift / multi-source RC insertion）:
   - 即時 fail-fast（promote 禁止）、`20-Decisions.md` の status を `provisional` 維持に戻す。
