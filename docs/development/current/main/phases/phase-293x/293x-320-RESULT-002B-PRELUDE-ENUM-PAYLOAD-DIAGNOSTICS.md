# 293x-320 RESULT-002B prelude enum payload diagnostics

Status: complete

## Decision

Decision: accepted.

Prelude `Option<T>` / `Result<T,E>` constructor payload arity mismatches now use
`[enum/payload][prelude]` diagnostics while preserving the existing enum
constructor lowering lane.

## Scope

- Add tagged payload arity diagnostics for `Option::Some`, `Option::None`,
  `Result::Ok`, and `Result::Err`.
- Keep `Option::Some(null|void)` on the existing nullish payload contract.
- Keep non-prelude enum constructor diagnostics unchanged.

## Non-goals

- No generic expected-type inference.
- No payload type compatibility checker.
- No Result/Option propagation sugar.
- No new JSON v0 enum constructor shape.

## Guard

- `tools/checks/k2_wide_result_option_payload_diagnostics_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_result_option_payload_diagnostics_guard.sh` passed locally.
