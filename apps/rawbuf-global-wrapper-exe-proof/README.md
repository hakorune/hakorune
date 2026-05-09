# rawbuf-global-wrapper-exe-proof

Purpose: M15 pure-first EXE proof for `RawBufCoreBox.alloc_bytes_i64` /
`RawBufCoreBox.free_bytes_i64` as generic-i64 global wrappers over hako.mem.

## Accepted Shape

- Source calls the RawBuf facade rather than direct `MemCoreBox` or direct
  `hako_mem_*` externcalls.
- MIR owns acceptance through `global_call_routes`.
- `RawBufCoreBox._trace` remains a normal void-sentinel side call.
- pure-first emits same-module definitions for the RawBuf wrappers and their
  M14 hako.mem leaves.

## Non-Goals

- No `RawBufCoreBox.realloc_bytes_i64`.
- No RawArrayCoreBox slot parity.
- No ownership/bounds/initialized-range/PtrCoreBox widening.
- No strong pointer attrs or native pointer proof.
