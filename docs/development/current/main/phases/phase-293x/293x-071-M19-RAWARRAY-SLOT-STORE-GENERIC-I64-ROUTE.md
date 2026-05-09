---
Status: done
Date: 2026-05-09
Scope: M19 RawArray slot_store_i64 generic-i64 route
---

# 293x-071 M19 RawArray Slot Store Generic-I64 Route

## Decision

`M19 RawArray slot_store_i64 generic-i64 route` accepts the next narrow
pure-first shape after M18:

```text
RawArrayCoreBox.slot_store_i64(handle, idx, value)
  -> OwnershipCoreBox.ensure_handle_writable_i64(handle)
  -> BoundsCoreBox.ensure_index_i64(handle, idx)
  -> PtrCoreBox.slot_store_i64(handle, idx, value)
  -> RawArrayCoreBox._trace(tag)
  -> return scalar i64
```

The only new leaf route in this row is `nyash.array.slot_store_hii`.
Store-handle/string variants remain outside this row. The backend remains a
table-driven reader of MIR-owned route facts and must not infer RawArray or Ptr
semantics from names.

## Owned

- `nyash.array.slot_store_hii` as a scalar-i64 extern route.
- `PtrCoreBox.slot_store_i64/3` as a generic-i64 wrapper over that extern route.
- `RawArrayCoreBox.slot_store_i64/3` as a generic-i64 same-module global target
  when ownership, bounds, and pointer store child routes are available.

## Not Owned

- RawArray store-handle/string parity.
- `PtrCoreBox.slot_store_string_handle/3`.
- ArrayBox generic method parity changes.
- Native pointer attrs or allocator ownership proofs.
- Backend symbol-name guessing outside MIR-owned route fact tables.

## Acceptance

```bash
bash tools/checks/k2_wide_rawarray_slot_store_exe_guard.sh
cargo test -q generic_i64_body_accepts_array_slot_store_extern_route -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: green on 2026-05-09.

## Next Reading

After this row, the raw-page probe should no longer be blocked by
`RawArrayCoreBox.slot_load_i64/slot_store_i64`. Any remaining blocker must be
recorded as a fresh row instead of widening this card.
