---
Status: Landed
Date: 2026-04-22
Scope: A2d cleanup card for deleting the legacy `.inc` `array_string_len_window` analyzer after all modes are MIR metadata-owned.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
  - docs/development/current/main/phases/phase-292x/292x-96-array-string-len-source-only-route-card.md
---

# 292x-97: Delete `array_string_len_window` C Analyzer

## Problem

`array_string_len_window` now has MIR-owned route metadata for all current modes:

- `len_only`
- `keep_get_live`
- `source_only_insert_mid`

The remaining C analyzer is legacy fallback debt. Keeping it in `.inc` leaves a
second route-legality owner after the MIR metadata contract is already pinned.

## Decision

Delete the raw MIR shape analyzer for this route family. `.inc` may keep only:

- metadata reader / field validation
- selected helper emission
- skip marking
- fail-fast on malformed metadata

Missing metadata may fall through to generic lowering for legacy JSON, but
route-required fixtures and app smokes must assert `reason=mir_route_metadata`
so coverage regressions are visible.

## Deleted

- `struct ArrayStringLenWindowMatch`
- `inst_is_array_string_len_safe_reuse`
- `inst_is_array_string_len_insert_mid_source_reuse`
- `trace_array_string_len_window_candidate`
- `analyze_array_string_len_window_candidate`
- fallback branches in `hako_llvmc_ffi_generic_method_get_lowering.inc`

## Kept

- `struct ArrayStringPiecewiseDirectSetSourceReuseMatch`
- `match_array_string_piecewise_concat3_direct_set_source_reuse`

These still belong to the separate substring/direct-set policy path. They are
not part of the retired `array_string_len_window` route owner.

## Acceptance

```bash
cargo test -q array_string_len_window
cargo test -q build_mir_json_root_emits_array_string_len_window_routes
cargo build --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_insert_mid_source_only_min.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_len_live_after_get_min.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
```

## Result

- `.inc` no longer contains `analyze_array_string_len_window_candidate` or its
  trace fallback.
- `array_string_len_window` route selection is metadata-only for the migrated
  family.
- `tools/checks/inc_codegen_thin_shim_debt_allowlist.tsv` was reduced in the
  same slice:
  - `hako_llvmc_ffi_generic_method_get_lowering.inc`: `4 -> 2`
  - `hako_llvmc_ffi_generic_method_get_window.inc`: `6 -> 3`
