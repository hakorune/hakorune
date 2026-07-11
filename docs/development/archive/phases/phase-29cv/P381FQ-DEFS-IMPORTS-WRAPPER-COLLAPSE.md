# P381FQ Defs/Imports Wrapper Collapse

Date: 2026-05-06
Scope: collapse unnecessary wrapper methods in BuildProgramFragmentBox defs and imports injection logic.

## Context

After P381FO/FP, the imports wrappers were documented as removed but not actually committed, and the defs injection still had similar wrapper methods:

**Defs side:**
- `_build_defs_fragment_json/2` (wrapper around FuncScannerBox + _build_defs_json_with_source)
- `_inject_defs_fragment_if_present/2` (trivial guard + inject_json_fragment)

**Imports side (still present):**
- `_build_imports_fragment_json/1` (wrapper around UsingCollectorBox + convert_usings_to_imports)
- `_inject_imports_fragment_if_present/2` (trivial guard + inject_json_fragment)

These wrapper methods add indirection without providing value. Each _inject method should directly call the collectors and inject_json_fragment.

## Change

Collapsed all four wrapper methods into their calling sites:

**Defs cleanup:**
- Removed `_build_defs_fragment_json/2` (7 lines)
- Removed `_inject_defs_fragment_if_present/2` (4 lines)
- `_inject_defs_json/2` now directly calls FuncScannerBox.scan_all_boxes and inject_json_fragment

**Imports cleanup:**
- Removed `_build_imports_fragment_json/1` (8 lines)
- Removed `_inject_imports_fragment_if_present/2` (4 lines)
- `_inject_imports_json/2` now directly calls UsingCollectorBox.collect and inject_json_fragment

Total: 20 lines removed (net), no behavior change.

All three injection methods (`_inject_defs_json`, `_inject_imports_json`, `_inject_enum_decls_json`) now follow the same direct pattern without unnecessary wrappers.

## Verification

- Grep confirms no references to removed methods remain
- cargo check passes with no new errors
- Semantics unchanged: same call chain, just inlined wrappers

## Result

BuildProgramFragmentBox is now consistently BoxShape-clean with no unnecessary indirection. The defs, imports, and enum_decls injection all follow the same direct pattern.

## Classification

- BoxShape cleanup: wrapper collapse
- No Stage0 impact
- No blocker order change
- Completes the body-cleanup followup slice
