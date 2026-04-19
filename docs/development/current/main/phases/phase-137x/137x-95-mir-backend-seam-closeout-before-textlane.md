# 137x-95 MIR / Backend Seam Closeout Before TextLane

- Status: Closed preflight for `137x-E`
- Date: 2026-04-20
- Purpose: close the remaining MIR -> backend responsibility leak before the minimal `TextLane` storage implementation starts.

## Decision

`137x-E` remains the active implementation gate. Its first slice, `137x-E0`, is closed and `137x-E1` may start the minimal `TextLane` / `ArrayStorage::Text` storage implementation.

Before adding `TextLane` / `ArrayStorage::Text`, the current string lane must prove that MIR owns legality and publication decisions, while C shims and runtime helpers only execute already-decided contracts.

## Boundary Rule

MIR / lowering owns:

- `publication_boundary`
- `publication_contract`
- `publish_reason`
- `publish_repr_policy`
- `borrow_contract`
- `stable_view_provenance`
- read-side alias continuation legality:
  - same receiver
  - source window
  - follow-up substring
  - piecewise subrange
  - shared receiver

The backend `.inc` layer may:

- read MIR plan metadata
- normalize operands for backend-local emit
- select between already-legal emit variants
- keep temporary exact matchers only when the phase README names their removal gate

The backend `.inc` layer must not:

- rediscover publication defer legality
- infer provenance
- decide whether `StableView` is legal
- decide whether a read-side alias lane may continue unpublished
- grow exact-route bridges into keeper design

Rust runtime owns:

- objectization mechanics
- fresh handle / keep / cache / handle table mechanics
- `KernelTextSlot` state transitions
- `BorrowedHandleBox` retarget / fallback mechanics
- string bytes kernels and same-slot mutation mechanics

Runtime helpers may mirror enum names for dispatch, but the choice of reason / legality must come from MIR-owned metadata.

## Work Items

- [x] E0.1: make read-side alias continuation facts explicit in the MIR plan shape consumed by backend emit
  - facts must cover same receiver, source window, follow-up substring, piecewise subrange, and shared receiver
  - facts must be verifier-visible or downgrade before backend emit
- [x] E0.2: demote `hako_llvmc_ffi_string_concat_emit*` from delayed planner to metadata-consuming emitter
  - retain backend-local operand normalization only
  - remove or quarantine legality/provenance rediscovery
- [x] E0.3: classify `hako_llvmc_ffi_array_string_store_seed.inc` as temporary bridge surface
  - do not treat exact-route seed logic as keeper design
  - tie each remaining matcher to the Legacy Retirement Ledger removal gate
- [x] E0.4: finish `array_string_slot.rs` responsibility split without widening public ABI
  - read-side alias helpers
  - same-slot write kernel
  - store boundary adapter
  - publication/materialize substrate
  - keep `StoreArrayStrPlan` runtime-local and physical; it must not become a semantic owner

## Implementation Record

- MIR now emits `StringKernelPlan.read_alias` facts:
  - `same_receiver`
  - `source_window`
  - `followup_substring`
  - `piecewise_subrange`
  - `shared_receiver`
- MIR JSON exposes those facts under `metadata.string_kernel_plans[*].read_alias`.
- C shim readers now consume `string_kernel_plans` for read-alias facts, source windows, publication contracts, and insert-mid window values before falling back to legacy bridges.
- `hako_llvmc_ffi_string_concat_emit_routes.inc` and `hako_llvmc_ffi_string_chain_policy.inc` no longer call the shared-receiver scan directly for active routing; the scan is legacy fallback behind the kernel-plan reader.
- `hako_llvmc_ffi_array_string_store_seed.inc` remains a temporary exact seed bridge and is tied to the phase-137x Legacy Retirement Ledger.
- `array_string_slot.rs` is now a thin facade over:
  - `array_string_slot_helpers.rs`
  - `array_string_slot_indexof.rs`
  - `array_string_slot_write.rs`
  - `array_string_slot_store.rs`

## Acceptance Gates

- `StringKernelPlan` or its direct MIR-side companions carry the legality facts needed by the backend for the active read-side alias route.
- `.inc` code no longer performs semantic recovery for the active route; it consumes plan fields and emits calls.
- temporary exact-route matchers are listed in the Legacy Retirement Ledger or moved to explicit legacy fixtures.
- runtime array/string slot code is split by mechanism, with no new public ABI and no new semantic policy source.
- validation:

```bash
git diff --check
tools/checks/dev_gate.sh quick
cargo check -q -p nyash_kernel
cargo test -q --lib string_kernel_plan
```

## Exit Rule

`137x-E0` is closed. Next implementation slice: `137x-E1` minimal `TextLane` / `ArrayStorage::Text`.
