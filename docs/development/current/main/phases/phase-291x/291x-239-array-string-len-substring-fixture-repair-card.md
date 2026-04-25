---
Status: Landed
Date: 2026-04-25
Scope: Repair array-string len boundary smokes by adding missing StringSubstring and same-slot ArraySet CoreMethod metadata to their fixtures.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-237-array-string-get-metadata-fixture-sweep-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_lowering.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_substring_policy.inc
---

# 291x-239 Array-String Len Substring Fixture Repair Card

## Goal

Restore the active array-string `get -> len -> substring` boundary smokes.
After 291x-237, these fixtures consume push/get and len-window metadata, then
stop at `RuntimeDataBox.substring(...)` because the substring calls are still
metadata-absent.

The two source-only store fixtures also need their final
`RuntimeDataBox.set(ArrayBox)` call to declare the ArraySet storage route, so
the pure backend can emit the already-selected same-slot text helper without
falling into an undeclared helper.

The implementation seam is intentionally narrow: when
`array_string_len_window_routes` metadata matches, the metadata already proves
the array-text source. The backend must remember the get source from that
metadata hit even when the old `runtime_array_string` route-state bit is not
set by metadata-first `generic_method.get`.

## Boundary

- Add only `generic_method.substring` / `StringSubstring` metadata to the
  affected len/substring fixtures, plus the already-proven same-slot
  `generic_method.set` / `ArraySet` metadata for the two source-only store
  fixtures.
- Do not change `.inc` substring lowering.
- Change `.inc` get-window source tracking only to trust the already-matched
  MIR len-window metadata; do not add new method-name classifiers.
- Do not change len-window route metadata.
- Do not touch unrelated indexOf fixtures.

## Contract

Each repaired substring call carries:

```text
route_id = generic_method.substring
box_name = RuntimeDataBox
receiver_origin_box = StringBox
route_kind = string_substring
helper_symbol = nyash.string.substring_hii
core_method.op = StringSubstring
lowering_tier = warm_direct_abi
value_demand = read_ref
effects = ["observe.substring"]
```

The source-only store fixtures additionally carry:

```text
route_id = generic_method.set
box_name = RuntimeDataBox
receiver_origin_box = ArrayBox
route_kind = array_store_any
core_method.op = ArraySet
lowering_tier = cold_fallback
```

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_len_live_after_get_min.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_insert_mid_source_only_min.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed in 2026-04-25 fixture/source-tracking repair slice.

- Added `generic_method.substring` / `StringSubstring` metadata to the three
  active array-string len/substring boundary fixtures.
- Added `generic_method.set` / `ArraySet` metadata to the two source-only
  same-slot store fixtures so ArraySet declarations are metadata-driven.
- Fixed the `.inc` get-window source tracking seam: once
  `array_string_len_window_routes` metadata matches, the backend records the
  get source from that metadata hit instead of requiring the legacy
  `runtime_array_string` route-state bit.
- No method-name classifier rows were added.
