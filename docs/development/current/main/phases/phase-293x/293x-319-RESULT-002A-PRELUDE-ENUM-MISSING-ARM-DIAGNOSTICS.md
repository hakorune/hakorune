# 293x-319 RESULT-002A prelude enum missing-arm diagnostics

Status: complete

## Decision

Decision: accepted.

Prelude `Option<T>` / `Result<T,E>` missing-arm diagnostics now carry the stable
`[enum/missing-arm][prelude]` tag and name missing variants with canonical
`Type::Variant` spelling.

## Scope

- Reuse the existing known-enum exhaustiveness checker.
- Improve missing-arm text for `Option` and `Result`.
- Preserve the rule that `_` does not satisfy known-enum exhaustiveness.
- Keep non-prelude known-enum diagnostics on the existing general wording.

## Non-goals

- No payload-shape diagnostics.
- No generic enum type inference.
- No guard-let or try/throw/? sugar.
- No new JSON v0 enum match shape.

## Guard

- `tools/checks/k2_wide_result_option_missing_arm_diagnostics_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_result_option_missing_arm_diagnostics_guard.sh` passed locally.
