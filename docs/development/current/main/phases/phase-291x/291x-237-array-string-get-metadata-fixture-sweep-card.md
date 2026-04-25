---
Status: Landed
Date: 2026-04-25
Scope: Add CoreMethod push/get metadata to array-string boundary fixtures that still relied on metadata-absent RuntimeDataBox calls.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-235-runtime-data-get-boundary-smoke-repair-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
---

# 291x-237 Array-String Push/Get Metadata Fixture Sweep Card

## Goal

Convert the remaining array-string boundary fixtures whose
`RuntimeDataBox.push(ArrayBox)` / `RuntimeDataBox.get(ArrayBox)` pair was
metadata-absent into explicit `generic_method.push` and `generic_method.get`
CoreMethod route fixtures.

This removes another blocker before retrying any RuntimeData Array push/get
route fallback prune.

## Boundary

- Do not change `.inc` lowering behavior in this card.
- Do not change expected helper symbols.
- Do not change string len/indexOf producer-window metadata.
- Keep this to ArrayBox-backed `RuntimeDataBox.push/get` boundary fixtures only.

## Contract

Each converted push carries:

```text
route_id = generic_method.push
box_name = RuntimeDataBox
receiver_origin_box = ArrayBox
route_kind = array_append_any
helper_symbol = nyash.array.slot_append_hh
core_method.op = ArrayPush
lowering_tier = cold_fallback
value_demand = write_any
effects = ["mutate.shape"]
```

Each converted get carries:

```text
route_id = generic_method.get
box_name = RuntimeDataBox
receiver_origin_box = ArrayBox
route_kind = array_slot_load_any
helper_symbol = nyash.array.slot_load_hi
core_method.op = ArrayGet
lowering_tier = warm_direct_abi
value_demand = read_ref
effects = ["read.key"]
```

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/array_string_push_get_metadata_fixture_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Note: the existing string boundary smokes still cover later `indexOf` /
`substring` recipe work and may stop after this card's push/get contract is
already consumed. This card therefore uses the focused guard above instead of
those broader smokes as acceptance.

## Result

Landed in 2026-04-25 fixture metadata slice.

- Added `generic_method.push` / `core_method.op=ArrayPush` metadata to nine
  ArrayBox-backed array-string boundary fixtures.
- Added `generic_method.get` / `core_method.op=ArrayGet` metadata to the same
  fixtures.
- Added `tools/checks/array_string_push_get_metadata_fixture_guard.sh` to pin
  both JSON metadata presence and pure-first route-state consumption.
- Left `indexof_line_pure_min_v1.mir.json` out of scope because it is a
  larger multi-function exact seed, not the single-block array-string boundary
  shape swept by this card.
