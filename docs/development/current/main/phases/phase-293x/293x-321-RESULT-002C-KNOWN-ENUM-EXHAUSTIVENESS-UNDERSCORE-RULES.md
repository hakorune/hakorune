# 293x-321 RESULT-002C known-enum exhaustiveness underscore rules

Status: complete

## Decision

Decision: accepted.

Known-enum matches now produce a tagged diagnostic when `_` is present but
explicit variant arms are still missing. `_` remains a fallback expression, not
an exhaustiveness substitute.

## Scope

- Add `[enum/exhaustiveness][underscore]` for non-prelude known-enum matches
  where `_` is present but variants are missing.
- Preserve prelude-specific `[enum/missing-arm][prelude]` diagnostics.
- Preserve existing known-enum match lowering shape.

## Non-goals

- No new match semantics.
- No generic expected-type diagnostics.
- No guard-let or Result propagation sugar.

## Guard

- `tools/checks/k2_wide_known_enum_underscore_exhaustiveness_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_known_enum_underscore_exhaustiveness_guard.sh` passed locally.
