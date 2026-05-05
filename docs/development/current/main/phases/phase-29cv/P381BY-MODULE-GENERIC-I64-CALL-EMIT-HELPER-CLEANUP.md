---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: share optional-dst i64 call emission in module generic method bodies
Related:
  - docs/development/current/main/phases/phase-29cv/P381BX-MODULE-GENERIC-LEN-HELPER-SYMBOL-CLEANUP.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BY: Module Generic I64 Call Emit Helper Cleanup

## Problem

Several module generic method emitters repeated the same LLVM call shape:

```text
dst exists: emit %rN = call i64 @"symbol"(args), then set T_I64
no dst:     emit call i64 @"symbol"(args)
```

The repeated emission appeared in array push, string needle calls, len,
substring, get, keys, and map set. The route predicates and post-call metadata
publication were already separate and must stay separate.

## Decision

Introduce `module_generic_string_emit_i64_call(dst, symbol, args)` as the shared
emission seam for this optional-`dst` i64 call shape.

Each caller still owns its route acceptance, argument construction, and
post-call metadata:

- `len` keeps string-length recording.
- `substring` keeps string origin publication.
- `get` keeps map/array/string origin refinement.
- `keys` keeps array-birth publication.
- `push` keeps array-string promotion.

This is behavior-preserving cleanup. It does not add a new accepted method
shape and does not move any LoweringPlan predicate.

## Failure-First Probe

Before the cleanup, representative method fixtures were confirmed green:

```bash
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_array_push_min_v1.mir.json \
  --out /tmp/hakorune-array-push-before.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_substring_directabi_min_v1.mir.json \
  --out /tmp/hakorune-substring-directabi-before.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_indexof_directabi_min_v1.mir.json \
  --out /tmp/hakorune-indexof-directabi-before.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_set_coldruntime_min_v1.mir.json \
  --out /tmp/hakorune-map-set-before.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_array_get_missing_min_v1.mir.json \
  --out /tmp/hakorune-runtime-array-get-before.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_map_get_missing_min_v1.mir.json \
  --out /tmp/hakorune-runtime-map-get-before.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_len_directabi_min_v1.mir.json \
  --out /tmp/hakorune-string-len-directabi-before2.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_len_directabi_min_v1.mir.json \
  --out /tmp/hakorune-array-len-directabi-before2.o
```

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_array_push_min_v1.mir.json \
  --out /tmp/hakorune-array-push.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_substring_directabi_min_v1.mir.json \
  --out /tmp/hakorune-substring-directabi.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_indexof_directabi_min_v1.mir.json \
  --out /tmp/hakorune-indexof-directabi.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_set_coldruntime_min_v1.mir.json \
  --out /tmp/hakorune-map-set.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_array_get_missing_min_v1.mir.json \
  --out /tmp/hakorune-runtime-array-get.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/runtime_data_map_get_missing_min_v1.mir.json \
  --out /tmp/hakorune-runtime-map-get.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_string_len_directabi_min_v1.mir.json \
  --out /tmp/hakorune-string-len-directabi.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_array_len_directabi_min_v1.mir.json \
  --out /tmp/hakorune-array-len-directabi.o
cargo test --release generic_method_route_plan -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
