---
Status: done
Date: 2026-05-09
Scope: M16 RawArray slot_append_any generic-i64 route
---

# 293x-068 M16 RawArray Slot Append Generic-I64 Route

## Decision

`M16 RawArray slot_append_any generic-i64 route` accepts the next narrow
pure-first shape after M15:

```text
RawArrayCoreBox.slot_append_any(handle, value_any)
  -> OwnershipCoreBox.ensure_handle_writable_i64(handle)
  -> OwnershipCoreBox.ensure_any_readable_i64(value_any)
  -> PtrCoreBox.slot_append_any(handle, value_any)
  -> RawArrayCoreBox._trace(tag)
  -> return scalar i64
```

The semantic owner stays in MIR route facts:

- `OwnershipCoreBox._handle_live_i64/1` uses an explicit extern route for
  `nyash.any.handle_live_h`.
- `PtrCoreBox.slot_append_any/2` uses an explicit extern route for
  `nyash.array.slot_append_hh`.
- `RawArrayCoreBox.slot_append_any/2` is accepted only when those child route
  facts prove a generic-i64 same-module body.

The backend stays a table-driven reader of `extern_call_routes` and
`global_call_routes`; it must not special-case RawArray/Ownership/Ptr names.

## Owned

- `nyash.any.handle_live_h` as a scalar-i64 extern route.
- `nyash.array.slot_append_hh` as a scalar-i64 extern route.
- `OwnershipCoreBox.ensure_handle_writable_i64/1` and
  `OwnershipCoreBox.ensure_any_readable_i64/1` as generic-i64 wrappers when
  their `_handle_live_i64` child route is available.
- `PtrCoreBox.slot_append_any/2` as a generic-i64 wrapper over the array append
  extern route.
- `RawArrayCoreBox.slot_append_any/2` as a generic-i64 same-module global
  target for a narrow EXE fixture.

## Not Owned

- RawArray slot load/store parity.
- `BoundsCoreBox` or `InitializedRangeCoreBox` route acceptance.
- `PtrCoreBox.slot_len_i64/1`, `slot_load_i64/2`, or `slot_store_i64/3`.
- RawArray full parity for `apps/mimalloc-raw-page-proof`.
- ArrayBox generic method parity changes.
- Native pointer attrs or allocator ownership proofs.
- Backend symbol-name guessing outside MIR-owned route fact tables.

## Acceptance

```bash
bash tools/checks/k2_wide_rawarray_slot_append_exe_guard.sh
cargo test -q generic_i64_body_accepts_any_handle_live_and_array_slot_append_extern_routes -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: green on 2026-05-09.

## Next Reading

After this row, the raw-page probe is expected to move from append seeding
toward `RawArrayCoreBox.slot_load_i64/slot_store_i64`, where
`BoundsCoreBox`, `InitializedRangeCoreBox`, and `PtrCoreBox.slot_len/load/store`
must land as separate rows.
