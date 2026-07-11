# P381FP Imports Dead Wrappers Prune

Date: 2026-05-06
Scope: remove the dead imports wrapper methods left after P381FO collapse.

## Context

After P381FO, the `_inject_imports_json/2` logic was inlined directly, but the
now-unused wrapper methods were not removed:

- `_build_imports_fragment_json/1` (lines 113-119)
- `_inject_imports_fragment_if_present/2` (lines 180-183)

These methods were no longer called anywhere after P381FO.

## Change

Removed both dead methods from `BuildProgramFragmentBox`:

- `_build_imports_fragment_json/1`: 8 lines removed
- `_inject_imports_fragment_if_present/2`: 5 lines removed

Total: 13 lines removed, no behavior change.

## Verification

- `./target/release/hakorune --emit-mir-json` still works correctly
- No references to these methods remain in the codebase
- Grep confirms they are not called anywhere

## Result

The imports cleanup (P381FO) is now complete with no dead code residue.
The blocker order remains the same - next targets are enum_decls and defs
injection helpers.

## Classification

- BoxShape cleanup: dead code removal
- No Stage0 impact
- No blocker order change
- Cleanup of P381FO incomplete removal
