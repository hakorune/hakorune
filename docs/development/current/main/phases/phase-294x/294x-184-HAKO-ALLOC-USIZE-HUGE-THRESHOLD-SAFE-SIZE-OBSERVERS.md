---
Status: Landed
Date: 2026-05-24
Scope: huge-threshold router safe size-observer exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-182
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-183-HAKO-ALLOC-USIZE-HUGE-THRESHOLD-ROUTER-EXE-ILLEGAL-INSTRUCTION-INVESTIGATION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/huge_threshold_router_box.hako
  - tools/checks/k2_wide_mimalloc_huge_threshold_routing_guard.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
---

# 294x-184 Hako Alloc Usize Huge-Threshold Safe Size Observers

## Decision

Migrate only the safe non-negative huge-threshold router size observers to exact
`usize` storage:

- `HakoAllocHugeThresholdRouter.last_padded_size`
- `HakoAllocHugeThresholdRouter.last_huge_threshold`

Keep this observer signed:

- `HakoAllocHugeThresholdRouter.last_good_size`

`last_good_size` looks like a size observer, but it can carry the
`SizeClassBox.good_size(...) == -1` sentinel when the request is huge. The
pure-first EXE `SIGILL` recorded by `294x-183` was therefore a correct exact
unsigned fail-fast for an invalid field group, not a reason to weaken the exact
`usize` backend path.

## Root Cause

The deferred all-three observer probe changed:

```hako
last_good_size: usize = 0
```

Then the huge request path executed:

```hako
me.last_good_size = SizeClassBox.good_size(padded_size)
```

For huge requests `good_size(...)` returns `-1`. Lowering to an exact unsigned
field correctly emitted a non-negative check and reached `llvm.trap` / `ud2`.

## Stop Line

This row does not migrate:

- `last_good_size`;
- `last_route_kind`;
- `last_result_ptr`;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

Do not mark fields as exact `usize` by name alone. Size-like observers remain
signed if they can carry negative sentinels.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_huge_threshold_routing_guard.sh
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
