# rawarray-slot-load-exe-proof

Purpose: M18 pure-first EXE proof for `RawArrayCoreBox.slot_load_i64` as a
generic-i64 global wrapper over ownership, bounds, initialized-range, and
pointer substrate leaves.

## Accepted Shape

- Source calls the RawArray facade rather than direct `PtrCoreBox` or direct
  `nyash.array.slot_load_hi` externcalls.
- MIR owns acceptance through `global_call_routes` and `extern_call_routes`.
- `PtrCoreBox.slot_load_i64` routes through `nyash.array.slot_load_hi`.
- Bounds and initialized-range wrappers rely on the M17 `slot_len` route.
- pure-first emits same-module definitions for the wrapper chain.

## Non-Goals

- No RawArray slot store parity.
- No PtrCoreBox slot_store widening.
- No full raw-page EXE parity.
