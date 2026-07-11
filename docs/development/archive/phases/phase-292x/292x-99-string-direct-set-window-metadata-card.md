---
Status: Landed
Date: 2026-04-22
Scope: next cleanup card for moving string concat/direct-set source-window matching out of `.inc` analysis and into MIR-owned metadata.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
---

# 292x-99: String Direct-Set Window Metadata

## Problem

`array_string_len_window` and `array_rmw_window` are now metadata-only, but
`hako_llvmc_ffi_generic_method_get_window.inc` still has a hidden C-side
source-window matcher used by substring/direct-set lowering:

- `struct ArrayStringPiecewiseDirectSetSourceReuseMatch`
- `match_array_string_piecewise_concat3_direct_set_source_reuse`

This is still route-legality analysis in `.inc`. It should become MIR-owned
metadata before larger generic method route-policy cleanup.

## Decision

Move the piecewise direct-set source-window decision to MIR metadata. `.inc`
may keep only:

- metadata reader / field validation
- selected helper emission
- skip marking
- fail-fast on malformed metadata

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_insert_mid_direct_set_min.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_string_piecewise_direct_set_min.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
```

## Result

- Added MIR-owned `StringDirectSetWindowRoute` metadata for
  `substring + substring + substring_concat3_hhhii -> direct set`.
- `.inc` no longer contains `ArrayStringPiecewiseDirectSetSourceReuseMatch` or
  `match_array_string_piecewise_concat3_direct_set_source_reuse`.
- `hako_llvmc_ffi_generic_method_substring_policy.inc` now consumes
  `string_direct_set_window_routes` and records the deferred piecewise route
  from metadata.
- Updated stale boundary smokes so route-only probes pass when the current
  boundary recipe stops after `unsupported pure shape`.
