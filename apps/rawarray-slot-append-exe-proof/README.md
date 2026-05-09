# rawarray-slot-append-exe-proof

Purpose: M16 pure-first EXE proof for `RawArrayCoreBox.slot_append_any` as a
generic-i64 global wrapper over ownership and pointer substrate leaves.

## Accepted Shape

- Source calls the RawArray facade rather than direct `PtrCoreBox` or direct
  `nyash.array.slot_append_hh` externcalls.
- MIR owns acceptance through `global_call_routes` and `extern_call_routes`.
- `OwnershipCoreBox._handle_live_i64` routes through `nyash.any.handle_live_h`.
- `PtrCoreBox.slot_append_any` routes through `nyash.array.slot_append_hh`.
- pure-first emits same-module definitions for the wrapper chain.

## Non-Goals

- No RawArray slot load/store parity.
- No BoundsCoreBox or InitializedRangeCoreBox acceptance.
- No PtrCoreBox slot_len/load/store widening.
- No full raw-page EXE parity.
