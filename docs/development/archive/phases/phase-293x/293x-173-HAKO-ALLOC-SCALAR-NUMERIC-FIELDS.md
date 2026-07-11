---
Status: Complete
Date: 2026-05-12
Scope: hako_alloc numeric field annotations converge on scalar substrate names.
Related:
  - lang/src/hako_alloc/memory/page_box.hako
  - lang/src/hako_alloc/memory/page_queue_box.hako
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/allocator_facade_box.hako
  - src/mir/builder/calls/special_handlers.rs
  - docs/reference/runtime/substrate-capabilities.md
---

# 293x-173 Hako Alloc Scalar Numeric Fields

## Goal

Make allocator policy/state code read like scalar allocator code, not boxed
object code.

After 293x-171/172, stored field initializers are the right shape for fixed
defaults. This row tightens the numeric side:

```hako
used: i64 = 0
free_top: i64 = 0
reject_count: i64 = 0
```

The port deliberately uses `i64`, not `usize`, because the current runtime lane
is still dynamic `Integer(i64)`. Pointer-sized exact semantics remain reserved
until a later row proves them.

## Changes

- `hako_alloc/memory` numeric stored fields now use `i64` annotations instead
  of `IntegerBox`.
- MIR type-name parsing maps numeric substrate names (`i64`, `usize`, etc.) to
  the current integer lane for declared field get/set metadata.
- Existing typed-object storage inference already treats these numeric substrate
  names as inline i64 storage; this row aligns the general field-lowering path
  with that reading.

## Non-goals

- No exact-width integer semantics.
- No unsigned range / overflow / pointer-sized verifier.
- No `usize` migration for allocator counters yet.
- No allocator algorithm advancement beyond source-style cleanup.

## Proof

```bash
cargo test -q parse_type_name_to_mir_maps_numeric_substrate_names_to_integer_lane
cargo check --bin hakorune
bash tools/checks/k2_wide_mimalloc_page_model_guard.sh
bash tools/checks/k2_wide_mimalloc_page_queue_guard.sh
bash tools/checks/k2_wide_mimalloc_layout_migration_guard.sh
bash tools/checks/k2_wide_hako_alloc_production_facade_stress_exe_guard.sh
```
