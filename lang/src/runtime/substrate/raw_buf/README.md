# lang/src/runtime/substrate/raw_buf - RawBuf Staging

Responsibilities:
- First allocator-substrate consumer above `hako.mem`.
- Keep raw byte-buffer allocation vocabulary separate from allocator policy.
- Provide the narrowest live surface needed before native layout / MaybeInit /
  no_alloc / no_safepoint work starts.

Current live surface:
- `alloc_bytes_i64(size)`
- `alloc_bytes_usize(size: usize)` as a non-negative current-lane i64 subset
  facade over `alloc_bytes_i64`
- `realloc_bytes_i64(ptr, new_size)`
- `realloc_bytes_usize(ptr: i64, new_size: usize)` as a non-negative
  current-lane i64 subset facade over `realloc_bytes_i64`
- `free_bytes_i64(ptr)`

Rules:
- `RawBuf` is not an allocator state machine.
- `RawBuf` is not native layout ownership.
- `RawBuf` stays above `hako.mem` and below `hako_alloc` policy/state rows.
- Keep this row as a thin bridge over `MemCoreBox` until verifier-backed
  layout/state rows are named.

Non-goals:
- No len/cap policy here.
- No `set_len` / `shrink` here.
- No `MaybeInit` here.
- No `repr` / `sizeof` / `alignof` here.
- No TLS / atomic / GC / OS VM policy here.
- No double-free or use-after-free verifier here yet.
