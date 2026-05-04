# Option Values

Status: Provisional public language direction.

`Option<T>` is the planned public optional-value shape for Hakorune/Nyash. It
is distinct from `null` / `void` and must not be used as the compiler helper
no-match carrier on the Stage0 path.

Design SSOT:

- `docs/development/current/main/design/hako-option-null-no-match-policy-ssot.md`
- `docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md`

## Historical Note

Older development history included a Box-first `OptionBox` / `ResultBox`
library implementation, Optional/Null proposal docs, and Phase 12.7-era
`ResultBox` / `?` references. Those are references, not the current canonical
language semantics.

Current rule:

```text
OptionBox:
  optional compatibility facade or implementation detail

Option<T>:
  public language meaning
  enum Option<T> { None, Some(T) }
```

If an `OptionBox` facade is restored later, it must preserve the enum Option
semantics described here.

## Surface Shape

`Option<T>` uses the enum surface:

```hako
enum Option<T> {
  None
  Some(T)
}
```

Construction uses qualified enum constructors:

```hako
local a = Option::Some(42)
local b = Option::None
```

Matching may use known-enum shorthand when the scrutinee resolves to the known
`Option` enum:

```hako
match a {
  Some(v) => print(v)
  None => print("none")
}
```

## Semantics

- `Option::None` is not `null`.
- `Option::Some(null)` is forbidden.
- `Option::Some(void)` is forbidden.
- `Option<T>` is an explicit optional value, not a nullable value.

The first implementation may reject `Some(null)` / `Some(void)` dynamically at
construction time. Static rejection is allowed when the payload is known during
parse or analysis.

## Relationship To `null` / `void`

`null` and `void` remain language literals. Runtime semantics currently treat
both as the same no-value concept.

`Option<T>` does not replace them:

- use `null` / `void` for existing dynamic none / no-value surfaces
- use `Option<T>` for explicit optional values
- do not compare `Option::None` with `null` as a substitute for matching

## Compiler Helper No-Match

Compiler helper no-match / not-found / unsupported is not a language value.

Stage0/selfhost compiler helper routes must not use `Option<T>` for no-match.
Use owner-local text carriers instead:

```text
"" = no-match
non-empty string = payload
```

when empty string is not a legal payload, or a tagged text carrier when it is.

## Sugar Surface

AQ-5 now allows the optional sugar surface on top of the same enum lane:

```hako
some expr
none
if some v = maybe_value {
  print(v)
} else {
  print("none")
}
```

These forms desugar to the same `Option::Some(...)`, `Option::None`, and known-enum
`match` route used by the explicit enum surface. They do not introduce a separate
runtime representation.

`?` propagation is still deferred until function return-shape policy is fixed.
