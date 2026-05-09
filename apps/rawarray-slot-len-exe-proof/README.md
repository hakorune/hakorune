# rawarray-slot-len-exe-proof

Purpose: M17 pure-first EXE proof for `RawArrayCoreBox.slot_len_i64` as a
generic-i64 global wrapper over ownership and pointer substrate leaves.

## Accepted Shape

- Source calls the RawArray facade rather than direct `PtrCoreBox` or direct
  `nyash.array.slot_len_h` externcalls.
- MIR owns acceptance through `global_call_routes` and `extern_call_routes`.
- `PtrCoreBox.slot_len_i64` routes through `nyash.array.slot_len_h`.
- `BufCoreBox.len_i64`, bounds, and initialized-range wrappers become
  generic-i64 only by reading the same pointer length route fact.
- pure-first emits same-module definitions for the wrapper chain.

## Non-Goals

- No RawArray slot load/store parity.
- No PtrCoreBox slot_load/slot_store widening.
- No full raw-page EXE parity.
