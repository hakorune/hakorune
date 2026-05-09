---
Status: done
Date: 2026-05-09
Scope: M15 RawBuf global wrapper generic-i64 route
---

# 293x-067 M15 RawBuf Global Wrapper Generic-I64 Route

## Decision

`M15 RawBuf global wrapper generic-i64 route` accepts the next narrow
pure-first shape after M14:

```text
RawBufCoreBox.alloc_bytes_i64(size)
  -> MemCoreBox.alloc_i64(size)
  -> hako_mem_alloc(size)
  -> RawBufCoreBox._trace(tag)
  -> return ptr_bits as i64

RawBufCoreBox.free_bytes_i64(ptr_bits)
  -> MemCoreBox.free_i64(ptr_bits)
  -> hako_mem_free(ptr_bits)
  -> RawBufCoreBox._trace(tag)
  -> return 0
```

The semantic owner stays in MIR `global_call_routes`. The backend does not
special-case `RawBufCoreBox` names. The route works because generic-i64 body
classification preserves `VoidSentinelI64Zero` call results as void sentinels
instead of trying to reclassify a `void`-typed trace result as scalar `i64`.

## Owned

- `RawBufCoreBox.alloc_bytes_i64/1` and `RawBufCoreBox.free_bytes_i64/1`
  recognized as generic-i64 same-module global targets when their only side
  call is a verified void-sentinel logging route.
- A narrow pure-first fixture that calls only those RawBuf wrappers from app
  code and reaches EXE without `unsupported_pure_shape`.
- A unit lock proving generic-i64 body accepts a scalar wrapper with an unused
  void-sentinel global call result.

## Not Owned

- `RawBufCoreBox.realloc_bytes_i64/2`.
- RawArrayCoreBox slot load/store/append parity.
- Ownership/Bounds/InitializedRange/PtrCoreBox wrapper widening.
- Full `apps/mimalloc-raw-page-proof` EXE.
- Native pointer proof, noalias, nonnull, dereferenceable, or alignment attrs.
- Backend symbol-name guessing for RawBuf wrappers.

## Acceptance

```bash
bash tools/checks/k2_wide_rawbuf_global_wrapper_exe_guard.sh
cargo test -q generic_i64_body_accepts_void_sentinel_global_side_call -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: green on 2026-05-09.

## Next Reading

After this row, the raw-page probe is expected to move from RawBuf allocation
wrappers toward RawArray/Ownership/Bounds/Ptr wrapper routes. Those must land as
separate rows so RawArray parity does not get hidden inside M15.
