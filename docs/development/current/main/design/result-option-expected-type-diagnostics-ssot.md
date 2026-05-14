---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: RESULT-002D generic enum expected-type diagnostics for prelude Option/Result constructors.
Related:
  - docs/development/current/main/design/result-option-prelude-diagnostics-ssot.md
  - docs/development/current/main/design/array-result-option-canonical-surface-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-322-RESULT-002D-GENERIC-ENUM-EXPECTED-TYPE-DIAGNOSTICS.md
  - docs/reference/language/option.md
  - docs/reference/language/EBNF.md
---

# Result / Option Expected-Type Diagnostics SSOT

## Decision

Prelude `Option<T>` and `Result<T,E>` constructors need an explicit expected type
when used as a local initializer. Hakorune does not infer missing generic enum
parameters from constructor payloads in this row.

Accepted:

```hako
local empty: Option<i64> = Option::None
local err: Result<i64, String> = Result::Err("bad")
```

Rejected:

```hako
local empty = Option::None
local err = Result::Err("bad")
```

The diagnostic tag is:

```text
[enum/expected-type][prelude]
```

## Ordering

This diagnostic only runs after the constructor shape is already valid.

Earlier diagnostics remain more specific:

```text
Option::Some()       -> [enum/payload][prelude]
Option::None(value)  -> [enum/payload][prelude]
Result::Err()        -> [enum/payload][prelude]
Option::Some(null)   -> [freeze:contract][option/some_nullish]
```

## Source Enum Shadowing

A same-program enum declaration named `Option` or `Result` is source-owned and
must not be treated as the prelude enum for this diagnostic. This preserves
legacy/source tests while the prelude lane remains a Stage1 diagnostics surface.

## Stage Split

Stage0/parser owns:

```text
Type::Variant transport as FromCall
no generic enum inference
```

Stage1 owns:

```text
local initializer expected-type check for prelude Option/Result constructors
tagged fail-fast diagnostic when local annotation is absent
source enum shadowing awareness
```

Stage1 does not own here:

```text
return-type expected context propagation
general generic enum inference
guard-let pattern sugar
match pattern shorthand changes
```

## Retire Condition

Retire this Rust-side diagnostic when the selfhost Stage1 type context owner can
produce the same fail-fast tag for the same local initializer cases without
adding broad inference.
