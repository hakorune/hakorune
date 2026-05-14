---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: RESULT-002B prelude enum payload diagnostics.
Related:
  - docs/development/current/main/design/result-option-prelude-diagnostics-ssot.md
  - docs/development/current/main/design/result-option-missing-arm-diagnostics-ssot.md
  - docs/reference/language/option.md
  - docs/development/current/main/phases/phase-293x/293x-320-RESULT-002B-PRELUDE-ENUM-PAYLOAD-DIAGNOSTICS.md
---

# Result / Option Payload Diagnostics SSOT

## Decision

Decision: accepted.

Prelude `Option<T>` / `Result<T,E>` constructors use the existing enum
constructor lowering lane, but payload arity mismatches carry a stable prelude
diagnostic tag:

```text
[enum/payload][prelude]
```

## Canonical diagnostics

- `Option::Some()` fails because `Some` needs one payload.
- `Option::None(value)` fails because `None` is a unit variant.
- `Result::Ok()` fails because `Ok` needs one payload.
- `Result::Err()` fails because `Err` needs one payload.

`Option::Some(null)` / `Option::Some(void)` remain owned by the existing
`[freeze:contract][option/some_nullish]` contract.

## Non-goals

- No payload type inference.
- No generic expected-type resolution.
- No Result propagation sugar.
- No new enum JSON shape.
