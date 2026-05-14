---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: RESULT-001 Result/Option prelude and enum variant diagnostics.
Related:
  - docs/development/current/main/design/array-result-option-canonical-surface-ssot.md
  - docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md
  - docs/development/current/main/design/result-option-missing-arm-diagnostics-ssot.md
  - docs/development/current/main/design/result-option-payload-diagnostics-ssot.md
  - docs/development/current/main/design/result-option-expected-type-diagnostics-ssot.md
  - docs/reference/language/EBNF.md
---

# Result / Option Prelude Diagnostics SSOT

## Decision

`Option<T>` and `Result<T,E>` are built-in enum surface names for Stage1
diagnostics and constructor lowering.

Canonical:

```hako
local empty: Option<i64> = Option::None
local ok: Result<i64, String> = Result::Ok(7)
local err: Result<i64, String> = Result::Err("bad")
```

Non-canonical:

```hako
local ok = Result.Ok(7)
local empty = Option.None
```

Known enum variants use `Type::Variant`. Dot remains object field / method
access and must fail-fast when it names a known enum variant.

## Prelude Shape

```hako
enum Option<T> {
    Some(T)
    None
}

enum Result<T, E> {
    Ok(T)
    Err(E)
}
```

The prelude feeds parser known-enum unit-variant recognition and Stage1
constructor validation. It does not force synthetic `enum_decls` into Program
JSON v0 output.

## Stage Split

Stage0/parser owns:

```text
known Option/Result unit-variant constructor shape
explicit Type::Variant transport as FromCall
```

Stage1 owns:

```text
Option/Result constructor validation
Option::Some nullish payload rejection
dot variant fail-fast diagnostics
prelude missing-arm diagnostics as RESULT-002A
prelude payload arity diagnostics as RESULT-002B
prelude expected-type diagnostics as RESULT-002D for local initializers
generic arity diagnostics through existing GEN-002 checker
```

Stage1 does not own here:

```text
try / throw / ? sugar
guard-let
new match semantics beyond existing known-enum lane
generic enum type inference
```

## Stop Lines

```text
no Result.Ok dot canonicalization
no unqualified Ok(x) canonical constructor
no nullable Result
no exception family
no Stage0 special Result control-flow semantics
```

## Retire Condition

This Rust-side prelude helper can retire when the selfhost Stage1 owner has a
single prelude enum registry that feeds parser metadata, constructor lowering,
and diagnostics with the same shapes.
