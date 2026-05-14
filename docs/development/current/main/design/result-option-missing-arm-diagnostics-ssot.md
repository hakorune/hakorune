---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: RESULT-002A prelude enum missing-arm diagnostics.
Related:
  - docs/development/current/main/design/result-option-prelude-diagnostics-ssot.md
  - docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md
  - docs/reference/language/option.md
  - docs/development/current/main/phases/phase-293x/293x-319-RESULT-002A-PRELUDE-ENUM-MISSING-ARM-DIAGNOSTICS.md
---

# Result / Option Missing-Arm Diagnostics SSOT

## Decision

Decision: accepted.

Known-enum exhaustiveness already rejects missing variant arms. For the built-in
prelude enums `Option<T>` and `Result<T,E>`, the diagnostic must name the missing
canonical constructors with `Type::Variant` spelling and a stable tag:

```text
[enum/missing-arm][prelude]
```

## Canonical diagnostics

For `Option<T>`:

```hako
match value {
  Some(v) => v
}
```

Missing arm diagnostics name `Option::None`.

For `Result<T,E>`:

```hako
match value {
  Ok(v) => v
  _ => fallback
}
```

Missing arm diagnostics name `Result::Err` and explain that `_` does not satisfy
known-enum exhaustiveness for the prelude lane.

## Non-goals

- No new match semantics.
- No change to the known-enum exhaustiveness rule.
- No payload-shape diagnostics; RESULT-002B owns that row.
- No generic enum expected-type diagnostics; RESULT-002D owns that row.
