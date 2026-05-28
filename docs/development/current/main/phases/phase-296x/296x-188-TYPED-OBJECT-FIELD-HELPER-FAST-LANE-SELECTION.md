---
Status: Landed
Date: 2026-05-28
Scope: select the first typed-object field helper fast-lane probe after row187.
Blocker: TYPED-OBJECT-FIELD-HELPER-FAST-LANE-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-187-FIELD-ARRAY-RUNTIME-LOWERING-BOUNDARY-PROBE.md
---

# 296x-188 Typed Object Field Helper Fast Lane Selection

## Purpose

Select one narrow owner for the typed-object field helper hot path. Row187
showed field helpers consume most exact-EXE samples. This row chooses the first
probe before changing runtime or compiler behavior.

## Current Boundary

The Python LLVM lowering path maps exact typed-object field access to exported
runtime helpers:

```text
FieldGet unsigned -> nyash.object.field_get_u64_hii
FieldGet handle   -> nyash.object.field_get_hii
FieldSet unsigned -> nyash.object.field_set_u64_hiu
FieldSet handle   -> nyash.object.field_set_hii
```

The exported helpers then access global typed-object storage through
`typed_objects().lock()` on each field get/set.

## Candidate Families

```text
1. typed_object_helper_lock_cost_probe
   - measure and isolate the Mutex/global-slab cost in field_get/field_set
   - no semantic change

2. typed_object_runtime_single_thread_fast_lane
   - reduce lock/global lookup overhead in the runtime helper layer
   - requires a clear single-thread/exact-EXE contract before implementation

3. mir_scalar_field_residence
   - avoid field helper calls for proven non-escaping/local objects
   - likely larger compiler work; do not start until helper lock cost is known
```

## Selection

```text
selected_next=typed_object_helper_lock_cost_probe
selected_reason=field helpers dominate perf and each helper takes typed_objects().lock()
defer_runtime_fast_lane_until=lock_cost_probe_confirms_helper_storage_cost
defer_mir_scalar_residence_until=runtime_helper_cost_is_quantified
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Next Row

```text
TYPED-OBJECT-HELPER-LOCK-COST-PROBE-296X-001
```

The next row should be a probe, not a keeper. It should answer:

```text
- How much of field_get/field_set time is Mutex/global slab lookup?
- Is a single-thread/exact-EXE helper lane plausible?
- Would MIR scalar field residence need to come first instead?
```

## Non-Goals

```text
- Do not replace Mutex/global storage in this row.
- Do not add by-name special cases for hako_alloc classes.
- Do not start MIR scalar residence without a separate SSOT/proof row.
- Do not optimize ArrayBox in the same row.
```
