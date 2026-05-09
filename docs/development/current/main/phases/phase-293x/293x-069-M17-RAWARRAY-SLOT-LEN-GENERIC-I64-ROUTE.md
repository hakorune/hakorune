---
Status: done
Date: 2026-05-09
Scope: M17 RawArray slot_len_i64 generic-i64 route
---

# 293x-069 M17 RawArray Slot Len Generic-I64 Route

## Decision

`M17 RawArray slot_len_i64 generic-i64 route` accepts the next narrow
pure-first shape after M16:

```text
RawArrayCoreBox.slot_len_i64(handle)
  -> OwnershipCoreBox.ensure_handle_readable_i64(handle)
  -> PtrCoreBox.slot_len_i64(handle)
  -> RawArrayCoreBox._trace(tag)
  -> return scalar i64
```

The semantic owner stays in MIR route facts:

- `PtrCoreBox.slot_len_i64/1` uses an explicit extern route for
  `nyash.array.slot_len_h`.
- `BufCoreBox.len_i64/1`, bounds checks, initialized-range checks, and
  `RawArrayCoreBox.slot_len_i64/1` are accepted only when that child route fact
  proves a generic-i64 same-module body.
- The backend remains a table-driven reader of `extern_call_routes` and
  `global_call_routes`; it must not special-case RawArray/Buf/Bounds/Ptr names.

## Owned

- `nyash.array.slot_len_h` as a scalar-i64 extern route.
- `PtrCoreBox.slot_len_i64/1` as a generic-i64 wrapper over the array length
  extern route.
- `BufCoreBox.len_i64/1` as the narrow verifier-facing wrapper over the same
  pointer length route.
- `BoundsCoreBox.ensure_index_i64/2` and
  `InitializedRangeCoreBox.ensure_initialized_index_i64/2` as generic-i64
  wrappers when their `BufCoreBox.len_i64/1` child route is available.
- `RawArrayCoreBox.slot_len_i64/1` as a generic-i64 same-module global target
  for a narrow EXE fixture.

## Not Owned

- RawArray slot load/store parity.
- `PtrCoreBox.slot_load_i64/2` or `PtrCoreBox.slot_store_i64/3`.
- RawArray full parity for `apps/mimalloc-raw-page-proof`.
- ArrayBox generic method parity changes.
- Native pointer attrs or allocator ownership proofs.
- Backend symbol-name guessing outside MIR-owned route fact tables.

## Acceptance

```bash
bash tools/checks/k2_wide_rawarray_slot_len_exe_guard.sh
cargo test -q generic_i64_body_accepts_array_slot_len_extern_route -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: green on 2026-05-09.

## Next Reading

After this row, the raw-page probe is expected to move from `slot_len`-blocked
verifier wrappers toward the actual `PtrCoreBox.slot_load_i64/slot_store_i64`
leaves. Those must land as separate rows.
