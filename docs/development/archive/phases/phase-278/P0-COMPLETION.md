# Phase 278 P0: Deprecated PHI env vars removal — completion

Status: ✅ completed (2025-12-22)

Goal:
- Remove legacy PHI debug environment variables (previously consolidated in Phase 277 P2).
- Enforce a single SSOT set of PHI debug knobs (fail-fast on deprecated inputs).

SSOT (kept):
- `NYASH_LLVM_DEBUG_PHI=1`
- `NYASH_LLVM_DEBUG_PHI_TRACE=1`
- `NYASH_LLVM_PHI_STRICT=1`

Removed inputs (deprecated variables):
- `NYASH_LLVM_PHI_DEBUG`
- `NYASH_PHI_TYPE_DEBUG`
- `NYASH_PHI_ORDERING_DEBUG`
- `NYASH_LLVM_TRACE_PHI`
- `NYASH_LLVM_VMAP_TRACE`

Behavior:
- If any removed variable is set, the harness errors with a replacement hint and exits non-zero.
- No new environment variables introduced.

Docs:
- `docs/reference/environment-variables.md` updated with:
  - removed variable list
  - migration table (old → new)
  - example error message

Tests:
- Added a dedicated smoke verifying:
  - deprecated vars fail
  - SSOT vars still work
