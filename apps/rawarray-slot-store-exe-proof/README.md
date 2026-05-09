# rawarray-slot-store-exe-proof

Purpose: M19 pure-first EXE proof for `RawArrayCoreBox.slot_store_i64` as a
generic-i64 global wrapper over ownership, bounds, and pointer substrate leaves.

## Accepted Shape

- Source calls the RawArray facade rather than direct `PtrCoreBox` or direct
  `nyash.array.slot_store_hii` externcalls.
- MIR owns acceptance through `global_call_routes` and `extern_call_routes`.
- `PtrCoreBox.slot_store_i64` routes through `nyash.array.slot_store_hii`.
- The fixture uses M18 `slot_load_i64` only to verify the store result.
- pure-first emits same-module definitions for the wrapper chain.

## Non-Goals

- No RawArray store-handle/string parity.
- No PtrCoreBox store-string widening.
- No broad ArrayBox method parity changes.
