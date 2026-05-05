---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: remove duplicated helper-symbol selection in module generic get emission
Related:
  - docs/development/current/main/phases/phase-29cv/P381BV-MODULE-GENERIC-STRING-NEEDLE-CALL-HELPER-CLEANUP.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BW: Module Generic Get Helper-Symbol Cleanup

## Problem

The module generic `get` emitter selected the helper symbol twice: once for the
`dst` emission branch and once for the non-`dst` branch. Both copies carried the
same long array-load predicate list and selected between:

```text
nyash.array.slot_load_hi
nyash.runtime_data.get_hh
```

This made the emitter harder to review because a future route addition would
need to update two lists in the same function.

## Decision

Compute the classification once:

```text
is_array_load
helper_symbol
```

Then use `helper_symbol` in both emission branches. Route acceptance and origin
publication stay in the existing get emitter body.

This is behavior-preserving cleanup. It does not add a new accepted get shape and
does not move any LoweringPlan predicate.

## Failure-First Probe

Before the cleanup, representative array and map get direct-ABI fixtures were
confirmed green:

```bash
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_array_get_missing_min_v1.mir.json \
  --out /tmp/hakorune-runtime-array-get.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_map_get_missing_min_v1.mir.json \
  --out /tmp/hakorune-runtime-map-get.o
```

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_array_get_missing_min_v1.mir.json \
  --out /tmp/hakorune-runtime-array-get.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_map_get_missing_min_v1.mir.json \
  --out /tmp/hakorune-runtime-map-get.o
cargo test --release generic_method_route_plan -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
