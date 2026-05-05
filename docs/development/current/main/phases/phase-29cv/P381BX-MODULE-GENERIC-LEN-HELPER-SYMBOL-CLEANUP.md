---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: remove duplicated helper-symbol selection in module generic len emission
Related:
  - docs/development/current/main/phases/phase-29cv/P381BW-MODULE-GENERIC-GET-HELPER-SYMBOL-CLEANUP.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BX: Module Generic Len Helper-Symbol Cleanup

## Problem

The module generic `len` emitter selected the helper symbol twice: once in the
`dst` branch and once in the non-`dst` branch. Both branches selected between:

```text
nyash.array.slot_len_h
nyash.string.len_fast_h
```

That duplication made the string/array route split harder to audit.

## Decision

Compute `helper_symbol` once after route acceptance, then use it in both
emission branches. Route predicates, argument construction, and string-length
recording remain in the existing emitter body.

This is behavior-preserving cleanup. It does not add a new accepted `len` shape.

## Failure-First Probe

Before the cleanup, representative string and array direct-ABI fixtures were
confirmed green:

```bash
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_len_directabi_min_v1.mir.json \
  --out /tmp/hakorune-string-len-directabi-before.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_len_directabi_min_v1.mir.json \
  --out /tmp/hakorune-array-len-directabi-before.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_string_length_ascii_min_v1.mir.json \
  --out /tmp/hakorune-runtime-string-len-before.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_array_length_min_v1.mir.json \
  --out /tmp/hakorune-runtime-array-len-before.o
```

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_len_directabi_min_v1.mir.json \
  --out /tmp/hakorune-string-len-directabi.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_len_directabi_min_v1.mir.json \
  --out /tmp/hakorune-array-len-directabi.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_string_length_ascii_min_v1.mir.json \
  --out /tmp/hakorune-runtime-string-len.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_array_length_min_v1.mir.json \
  --out /tmp/hakorune-runtime-array-len.o
cargo test --release generic_method_route_plan -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
