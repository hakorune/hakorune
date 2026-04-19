# 137x-94 TextLane / Value Lane / Allocator Implementation Gate

- Status: Active implementation gate
- Date: 2026-04-20
- Purpose: replace the old "deferred successor" stop-line with an explicit implementation order before returning to the next kilo optimization pass.

## Decision

The next kilo optimization should not continue as another helper-local edit until the storage/value/allocator lanes are opened in a controlled order.

This gate opens:

1. `137x-E0`: MIR / backend seam closeout before `TextLane`
2. `137x-E`: minimal `TextLane` / `ArrayStorage::Text` implementation
3. `137x-F`: runtime-wide `Value Lane` implementation bridge
4. `137x-G`: allocator / arena lane pilot
5. `137x-H`: return to kilo optimization after the implementation gates have landed or been explicitly rejected

## Scope

- `String = value` remains the semantic rule.
- `TextLane` is storage/residence only; it is not semantic truth.
- MIR/lowering owns legality, provenance, demand, and publication contracts.
- Rust runtime owns residence mutation, cache mechanics, and runtime-private helper implementation.
- `.inc` code owns backend transport and emit shape only; it must not grow new legality or provenance decisions.
- Public ABI stays unchanged unless a later explicit ABI phase opens it.

## Opened Work

- `137x-E0 MIR / backend seam closeout` (closed)
  - SSOT: `137x-95-mir-backend-seam-closeout-before-textlane.md`
  - Push read-side alias continuation legality into MIR-owned metadata.
  - Demote `.inc` string emit paths from delayed planner to metadata-consuming emitter.
  - Classify exact seed bridges as temporary surfaces with removal gates.
  - Split `array_string_slot.rs` by runtime mechanism without making it a semantic owner.

- `137x-E TextLane` (active next)
  - Start with array string hot paths and `KernelTextSlot` replacement pressure.
  - Accept only a narrow storage implementation that preserves existing public behavior.
  - Require exact/middle/whole proof before calling it a keeper.

- `137x-F Value Lane`
  - Use the phase-289x ledgers as the vocabulary and demand SSOT.
  - Move from planning-only to a constrained implementation bridge.
  - Keep Array / Map public identity unchanged while internal residence becomes lane-aware.

- `137x-G allocator / arena`
  - Open after TextLane / Value Lane show whether copy/allocation cost remains structural.
  - Treat `memmove`, `malloc`, and `_int_malloc` as evidence, not as permission for a broad allocator rewrite.
  - Keep rollback small and gate every allocator change with exact/middle/whole proof.

## Still Deferred

- `publish.any`
- typed map lane implementation
- heterogeneous / union array slot layout
- public ABI widening
- MIR legality expansion beyond the contracts needed by `137x-E/F/G`

## Acceptance Gates

Each implementation slice must record:

- front: exact / middle / whole
- current owner
- hot transition
- rejected alternatives
- commands and results

Minimum commands:

```bash
git status -sb
tools/checks/dev_gate.sh quick
PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_string_store 1 3
PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_meso_substring_concat_array_set_loopcarry 1 3
PERF_VM_FORCE_NO_FALLBACK=1 PERF_REQUIRE_AOT_RESULT_PARITY=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 3
```

## Exit Rule

Return to `137x-H` kilo optimization only after `137x-E0/E/F/G` have either:

- landed with gates green, or
- been explicitly rejected with evidence and rollback notes.
