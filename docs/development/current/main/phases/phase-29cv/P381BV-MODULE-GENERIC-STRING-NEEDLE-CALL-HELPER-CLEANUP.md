---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: remove duplicated one-argument string needle call emission in the module generic string body emitter
Related:
  - docs/development/current/main/phases/phase-29cv/P381BU-MODULE-GENERIC-ARRAY-APPEND-EMIT-HELPER-CLEANUP.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BV: Module Generic String Needle Call Helper Cleanup

## Problem

The module generic string body emitter carried three copies of the same
one-argument string needle call mechanics:

- `StringBox.indexOf(needle)`
- `StringBox.lastIndexOf(needle)`
- `StringBox.contains(needle)`

Each copy read the single needle argument, formatted `(recv, needle)`, emitted
an `i64` helper call, and set the destination type when `dst` existed.

## Decision

Move only the shared call mechanics into:

```text
module_generic_string_emit_string_needle_i64_call(...)
```

Route acceptance stays outside the helper:

- `indexOf` still requires the direct `generic_method.indexOf` LoweringPlan row
- `lastIndexOf` still requires the direct `generic_method.lastIndexOf` row
- `contains` still requires the direct `generic_method.contains` row
- the two-argument `indexOf(needle, start)` lowering remains in the existing
  dedicated path

This is behavior-preserving cleanup. It does not merge route predicates, does
not widen accepted argument shapes, and does not change helper symbols.

## Failure-First Probe

Before the cleanup, the existing direct-ABI `indexOf` fixture was confirmed
green:

```bash
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_indexof_directabi_min_v1.mir.json \
  --out /tmp/hakorune-indexof-directabi.o
```

The existing VM smokes for `StringUtils.index_of`, `StringUtils.last_index_of`,
and `StringUtils.contains` are not suitable for this emitter cleanup because
they currently fail before this C emitter path:

```text
extern not supported: StringUtils.{index_of,last_index_of,contains}/2
```

After `cargo build --release -p nyash-integer-plugin`, the missing plugin error
disappears, but the StringUtils extern owner failure remains. To keep this
cleanup locally verifiable, add direct-ABI MIR fixtures for `lastIndexOf` and
`contains` alongside the existing `indexOf` fixture.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_indexof_directabi_min_v1.mir.json \
  --out /tmp/hakorune-indexof-directabi.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_lastindexof_directabi_min_v1.mir.json \
  --out /tmp/hakorune-lastindexof-directabi.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_contains_directabi_min_v1.mir.json \
  --out /tmp/hakorune-contains-directabi.o
cargo test --release generic_method_route_plan -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
