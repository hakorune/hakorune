# 293x-1199 MIMAP-569A Phase-293x Mimalloc Blueprint Lane Closeout

Status: selected current
Date: 2026-05-22

## Purpose

Close `phase-293x` using the fixed terminal planning pilot, close criteria, and
inventory/carryover boundary.

## Required Evidence

```text
✅ bash tools/checks/k2_wide_record_ergonomics_surface_guard.sh
   → 127 unit tests all PASS
   → record default fill + shorthand + same-name namespace + `with` update syntax
   → Rust + .hako parser parity achieved

✅ bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_explicit_runner_planning_pilot_guard.sh
   → MIMAP-566A terminal planning pilot green

✅ bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_phase293x_close_criteria_guard.sh
   → MIMAP-567A close criteria green

✅ bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_phase293x_inventory_carryover_guard.sh
   → MIMAP-568A inventory/carryover green
```

## Closeout Conditions

- Terminal planning pilot (`MIMAP-566A`) remains green. ✅
- Close criteria (`MIMAP-567A`) remains synchronized with SSOT. ✅
- Inventory/carryover boundary (`MIMAP-568A`) is fixed and stable. ✅
- Record ergonomics feature complete: Rust + .hako parser parity, 127 tests green. ✅
- Execution seams remain closed in this phase. ✅

## Next Lane Candidate

```text
phase-294x explicit C mimalloc evidence execution lane
```

## Deferred & Follow-on Work

### Record Ergonomics Expansion Lane

Related deferred lane (separate from mimalloc execution closeout):

```text
record ergonomics expansion lane
docs/development/current/main/design/record-ergonomics-expansion-post-293x-ssot.md
```

Feature complete; defer further DX/ergonomics improvements.

### Selfhost Compiler Infrastructure Optimization (JIT)

**Status**: Known issue identified; defer to phase-294x.

**Details**:
- `--emit-mir-json lang/src/compiler/entry/compiler.hako` times out at >15 minutes due to `parser_box` emit CPU stall.
- Root cause: Combinatorial explosion or O(n²) in parser inventory/Stage-3 loop expansion.
- Affects: selfhost compiler EXE build (`tools/build_compiler_exe.sh`).
- Resolution: Optimize `parser_box` dependency chain or split into smaller modules.
- Priority: Phase-294x infrastructure optimization (not critical for blueprint lane closeout).

**Evidence**:
- Parser keyword collision fixed: `guard` → `loop_guard` in 5 parser modules.
- Stage-B namespace resolved: direct box call pattern in `compiler.hako`.
- Remaining: `parser_box` emit performance profiling & optimization.
