---
Status: done
Date: 2026-05-09
Scope: M18 RawArray slot_load_i64 generic-i64 route
---

# 293x-070 M18 RawArray Slot Load Generic-I64 Route

## Decision

`M18 RawArray slot_load_i64 generic-i64 route` accepts the next narrow
pure-first shape after M17:

```text
RawArrayCoreBox.slot_load_i64(handle, idx)
  -> OwnershipCoreBox.ensure_handle_readable_i64(handle)
  -> BoundsCoreBox.ensure_index_i64(handle, idx)
  -> InitializedRangeCoreBox.ensure_initialized_index_i64(handle, idx)
  -> PtrCoreBox.slot_load_i64(handle, idx)
  -> RawArrayCoreBox._trace(tag)
  -> return scalar i64
```

The only new leaf route in this row is `nyash.array.slot_load_hi`.
`BoundsCoreBox` and `InitializedRangeCoreBox` stay wrappers over the M17
`slot_len` route. The backend remains a table-driven reader of MIR-owned route
facts and must not infer RawArray or Ptr semantics from names.

## Owned

- `nyash.array.slot_load_hi` as a scalar-i64 extern route.
- `PtrCoreBox.slot_load_i64/2` as a generic-i64 wrapper over that extern route.
- `RawArrayCoreBox.slot_load_i64/2` as a generic-i64 same-module global target
  when ownership, bounds, initialized-range, and pointer load child routes are
  all available.

## Not Owned

- RawArray slot store parity.
- `PtrCoreBox.slot_store_i64/3`.
- RawArray full parity for `apps/mimalloc-raw-page-proof`.
- ArrayBox generic method parity changes.
- Native pointer attrs or allocator ownership proofs.
- Backend symbol-name guessing outside MIR-owned route fact tables.

## Acceptance

```bash
bash tools/checks/k2_wide_rawarray_slot_load_exe_guard.sh
cargo test -q generic_i64_body_accepts_array_slot_load_extern_route -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: green on 2026-05-09.

## Next Reading

After this row, the raw-page probe is expected to move to
`PtrCoreBox.slot_store_i64/3`, which must land separately as M19.
