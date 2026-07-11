---
Status: Landed
Date: 2026-04-23
Scope: Move the remaining array get/set micro exact seed matcher behind MIR-owned route metadata.
Related:
  - docs/development/current/main/phases/phase-292x/292x-101-exact-seed-ladder-function-route-tags-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_array_micro_seed.inc
  - benchmarks/bench_kilo_micro_array_getset.hako
---

# 292x-110: Array Get/Set Micro Seed Route

## Intent

Remove the last temporary exact seed matcher in
`hako_llvmc_ffi_array_micro_seed.inc`:

- `hako_llvmc_match_array_getset_micro_seed`

The current direct MIR for `bench_kilo_micro_array_getset.hako` already exposes
the inner `array_rmw_window_routes` proof. This slice adds a whole-function
`FunctionMetadata.array_getset_micro_seed_route` so the backend boundary can
select the existing stack-array emitter without scanning raw MIR JSON blocks.

## Route Contract

- owner: `FunctionMetadata.array_getset_micro_seed_route`
- backend tag: `metadata.exact_seed_backend_route.tag = "array_getset_micro"`
- selected source route: `array_getset_micro_seed_route`
- proof: `kilo_micro_array_getset_7block`
- required inner proof: `array_get_add1_set_same_slot`
- consumer capability: `direct_stack_array_getset_micro`
- publication boundary: `none`

## C Boundary Rules

The C boundary may:

- validate the route metadata fields,
- validate the referenced inner RMW proof fields,
- select the existing `hako_llvmc_emit_array_getset_micro_ir` emitter.

The C boundary must not walk `blocks`, `instructions`, or raw `op` fields to
rediscover the shape.

## Acceptance

```bash
cargo fmt --check
cargo test -q array_getset_micro_seed --lib
cargo test -q exact_seed_backend_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_getset_micro_contract.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed as `FunctionMetadata.array_getset_micro_seed_route`.

- MIR owns the current 7-block canonical whole-function proof for
  `bench_kilo_micro_array_getset.hako`.
- The route references the existing `array_rmw_window_routes` proof
  `array_get_add1_set_same_slot`.
- C consumes metadata through `hako_llvmc_consume_array_getset_micro_route` and
  no longer walks raw `blocks` / `instructions` / `op` fields for this seed.
- `hako_llvmc_match_array_getset_micro_seed` was removed.
- The exact seed matcher family now has zero remaining
  `hako_llvmc_match_*seed` definitions.
- Debt guard baseline moved from `6 files / 52 lines` to
  `5 files / 47 lines`.

The next cleanup target is not another exact seed matcher. It should start with
a dedicated owner card for the remaining generic/minimal-path raw scanners,
especially `hako_llvmc_ffi_pure_compile_minimal_paths.inc`.

## Verification

```bash
cargo fmt --check
cargo test -q array_getset_micro_seed --lib
cargo test -q exact_seed_backend_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_getset_micro_contract.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
```
