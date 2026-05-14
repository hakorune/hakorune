---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: RESULT-002C known-enum exhaustiveness underscore rules.
Related:
  - docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md
  - docs/development/current/main/design/result-option-missing-arm-diagnostics-ssot.md
  - docs/reference/language/EBNF.md
  - docs/development/current/main/phases/phase-293x/293x-321-RESULT-002C-KNOWN-ENUM-EXHAUSTIVENESS-UNDERSCORE-RULES.md
---

# Known Enum Underscore Exhaustiveness SSOT

## Decision

Decision: accepted.

For known-enum matches, `_` is a fallback expression but it does not satisfy
exhaustiveness. Missing explicit variants fail-fast. When `_` is present but a
known enum variant is missing, diagnostics carry this stable tag:

```text
[enum/exhaustiveness][underscore]
```

## Canonical rule

```hako
enum PageState {
  Active
  Retired
}

match state {
  Active => 1
  _ => 0
}
```

This is rejected because `Retired` is not named explicitly.

## Non-goals

- No new match lowering semantics.
- No rule change for fully explicit matches.
- No guard-let or Result propagation sugar.
- No generic enum expected-type diagnostics; RESULT-002D owns that row.
