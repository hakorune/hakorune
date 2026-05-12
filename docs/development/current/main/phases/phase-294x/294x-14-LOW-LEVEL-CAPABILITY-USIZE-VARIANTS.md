---
Status: Complete
Date: 2026-05-12
Scope: low-level RawArray/Buf/bounds `usize` aliases over the current lane.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/reference/runtime/substrate-capabilities.md
---

# 294x-14 Low-Level Capability Usize Variants

## Decision

The second low-level `usize` capability row widens shape/index vocabulary
without claiming native unsigned storage.

Live v0 meaning:

```text
usize capability argument = non-negative current-lane i64 subset
```

The shared predicate is `CurrentLaneBox.is_usize_i64`. The row adds no native
ABI leaves and no pointer-sized field storage.

## Live Surface

Buf:

```text
BufCoreBox.len_usize(handle)
BufCoreBox.cap_usize(handle)
BufCoreBox.reserve_usize(handle, additional: usize)
BufCoreBox.grow_usize(handle, target_capacity: usize)
```

Bounds / initialized range:

```text
BoundsCoreBox.ensure_index_usize(handle, idx: usize)
BoundsCoreBox.ensure_insert_index_usize(handle, idx: usize)
InitializedRangeCoreBox.ensure_initialized_index_usize(handle, idx: usize)
```

RawArray:

```text
RawArrayCoreBox.slot_load_usize(handle, idx: usize)
RawArrayCoreBox.slot_store_usize(handle, idx: usize, value)
RawArrayCoreBox.slot_store_string_handle_usize(handle, idx: usize, value_h)
RawArrayCoreBox.slot_len_usize(handle)
RawArrayCoreBox.slot_cap_usize(handle)
RawArrayCoreBox.slot_remove_any_usize(handle, idx: usize)
RawArrayCoreBox.slot_insert_any_usize(handle, idx: usize, value_any)
RawArrayCoreBox.slot_slice_any_usize(handle, start: usize, end: usize)
RawArrayCoreBox.slot_reserve_usize(handle, additional: usize)
RawArrayCoreBox.slot_grow_usize(handle, target_capacity: usize)
```

## Out Of Scope

- Native `usize` slots.
- New array/ptr ABI symbols.
- RawBuf len/cap policy. RawBuf remains a raw byte-buffer allocation facade.
- hako_alloc field migration.
- mimalloc M167+ resume.

## Verification

```text
bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh
bash tools/checks/dev_gate.sh quick
```

