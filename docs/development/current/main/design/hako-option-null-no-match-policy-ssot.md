---
Status: SSOT
Decision: accepted
Date: 2026-05-04
Scope: public `Option<T>` direction, `null`/`void` boundary, and compiler helper no-match carriers.
Related:
  - docs/reference/language/option.md
  - docs/reference/language/types.md
  - docs/reference/language/EBNF.md
  - docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md
  - docs/development/current/main/design/type-system-policy-ssot.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/phases/phase-29cv/P381AQ-OPTION-NULL-NO-MATCH-TASK-MAP.md
---

# Hako Option / Null / No-Match Policy SSOT

## One-Line Rule

`null` is a language value, `Option<T>` is a public null-free optional value,
and compiler helper no-match is compiler control state, not a language value.

## Problem

The same surface words have been used for different jobs:

- public source values: `null` / `void`
- public optional values: future `Option<T>`
- compiler helper control: no-match / not-found / unsupported
- backend representation: `ConstValue::Null`, `ConstValue::Void`, route facts,
  `return_shape`, and TypeView-like observations

Mixing these is what makes Stage0 and selfhost helpers fat:

- `null` payloads create string/null PHI repair pressure
- numeric `0/1` sentinels create i64/i1 PHI repair pressure
- using `Option` as a helper no-match carrier would force enum machinery onto
  the Stage0 source-execution path

## Historical References

Earlier lines discussed and partially implemented Option-like APIs. They are
kept as references, but they are not the current canonical semantics.

Known references:

- `e441b2ba2` / `4cc25f3fb` added `apps/lib/boxes/option.hako`,
  `apps/lib/boxes/result.hako`, and Option/Result docs as a Box-first library
  implementation.
- `6ed7ce368` added Optional/Null semantics docs with future `get?` /
  `indexOf?` style APIs and OptionalBox/MaybeBox discussion.
- Phase 12.7-era archived/reference language material mentions `?`
  propagation and `ResultBox`-style error handling as future/public surface
  direction. It is historical context, not current Option semantics.
- `docs/reference/language/strings.md` still notes that Option/Maybe may
  replace null-style APIs in a future revision.
- `7e0161bb3` records an internal CondProfile/String/Option ownership policy;
  that use of `Option<T>` is analysis state, not public language semantics.

Current decision:

```text
OptionBox / ResultBox:
  historical implementation units or possible compatibility facades

Option<T>:
  public language meaning
  enum Option<T> { None, Some(T) }
```

Do not restore the historical `OptionBox` as the semantic SSOT. If a facade is
needed later, it must sit on top of enum `Option<T>` and preserve the null-free
invariant.

## Fixed Decisions

### 1. Keep `null` and `void` as language literals

`null` remains the source-level none literal. `void` remains the no-value
literal. Runtime semantics continue to treat both as the same no-value concept
unless a backend documents an explicit difference.

This document does not remove `null`.

### 2. Public `Option<T>` is wanted, but it is null-free

Public language direction:

```hako
enum Option<T> {
  None
  Some(T)
}
```

Rules:

- `Option::None` is not `null`
- `Option::Some(null)` is forbidden
- `Option::Some(void)` is forbidden
- construction uses the existing enum constructor surface
- known-enum match shorthand may use `Some(v)` / `None`

The first implementation may enforce the payload rule dynamically at
construction time. Static rejection is allowed only when the payload is known
at parse / analysis time.

### 3. Stage0 helper no-match must not use `Option`

Stage0's job is still:

```text
Canonical MIR -> uniform ABI -> ny-llvmc emitter
```

Stage0 must not learn `Option<T>` just to represent compiler helper failure.
No-match / not-found / unsupported in compiler helpers stays owner-local.

Preferred Stage0 helper forms:

```hako
try_lower_text(x) -> string
```

where:

```text
"" = no-match
non-empty string = payload
```

This is allowed only when empty string is not a valid payload.

When empty string is a valid payload, use a tagged text carrier instead:

```text
0:          = none
1:<payload> = some payload
```

Do not use `Option<T>` or `null` for the active Stage0 no-match path.

### 4. Legacy wrappers may translate to `null`

Compatibility APIs may keep:

```hako
try_lower(x) -> null | string
```

but the active Stage0/source-execution route should call the text-sentinel
owner helper:

```hako
try_lower(x) {
  local out = me.try_lower_text(x)
  if out == "" { return null }
  return out
}
```

Rule:

- legacy wrapper may map `""` to `null`
- active pure-first route must prefer text-sentinel helpers

### 5. Backend must not repair these mistakes broadly

Do not add broad backend repair for:

- string/null PHI
- i64/i1 PHI
- helper no-match expressed as `Option<T>`

Fix the source owner:

- text payloads: `""` or tagged text carrier
- predicates: Bool
- scalar counters/ids: explicit i64 seed

## Parser And Surface Contract

Option work touches both parser fronts.

Any public syntax change must update both:

- Rust parser / AST / Program(JSON)
- `.hako` parser / Stage1 Program(JSON)

No parser-only implementation is accepted for public `Option` syntax.

Phase 1 should avoid new syntax:

- use the existing enum surface for `Option<T>`
- use existing `Option::Some(v)` / `Option::None`
- use existing known-enum `match`

Future sugar, if accepted, must be dual-front:

- `some expr`
- `none`
- `if some v = expr { ... } else { ... }`
- `?` propagation only after function return-shape policy is fixed

## Implementation Order

### O0. Policy lock

Done by this SSOT and the P381AQ task card.

### O1. Inventory current enum/Option surfaces

Document current Rust parser and `.hako` parser support:

- enum declarations
- generic enum parameters
- `Type::Variant(...)`
- `Type::Variant { ... }`
- known-enum shorthand match
- Stage1 `EnumCtor` / `EnumMatch`
- MIR `VariantMake` / `VariantTag` / `VariantProject`

No behavior change.

### O2. Standard `Option<T>` owner

Add or restore the public standard owner for `Option<T>` without adding
Stage0 helper usage.

Acceptance:

- user code can construct and match `Option`
- compiler helper no-match paths remain text-sentinel / tagged-text

### O3. `Some(null)` / `Some(void)` fail-fast

Add construction-time rejection for `Some(null)` and `Some(void)`.

Acceptance:

- positive `Some(1)` / `Some("x")` cases pass
- negative `Some(null)` / `Some(void)` cases fail-fast
- Rust parser and `.hako` parser stay in parity

### O4. Optional sugar only after O2/O3

Sugar is not part of the first implementation.

Allowed later, docs-first:

- `some expr`
- `none`
- `if some v = expr { ... } else { ... }`

Deferred:

- `?` propagation

### O5. Optional compatibility facade only after semantics are fixed

If legacy or user-facing compatibility needs an `OptionBox` API, add it as a
facade over enum `Option<T>`.

Rules:

- no semantic ownership in `OptionBox`
- no Stage0 no-match carrier use
- no `Some(null)` / `Some(void)` escape hatch
- facade documentation must state its compatibility purpose

## Non-Goals

- no Stage0 `Option` lowering just for no-match
- no new `GlobalCallTargetShape` for Option helpers
- no C shim body-specific emitter for Option
- no `null` removal
- no broad backend coercion repair
