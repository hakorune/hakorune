---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: GUARDLET-001 minimal guard-let enum variant sugar.
Related:
  - docs/development/current/main/design/array-result-option-canonical-surface-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-323-GUARDLET-001-GUARD-LET-PATTERN-SUGAR.md
  - docs/reference/language/EBNF.md
---

# Guard-Let Pattern Sugar SSOT

## Decision

`guard let` is accepted as narrow enum-variant early-exit sugar. It does not add
`try`, `throw`, `?`, broad pattern binding, or new exception semantics.

Canonical MVP:

```hako
guard let Result::Ok(value) = result else {
    return Result::Err(reason)
}

return value
```

The MVP accepts explicit qualified single-payload enum variant patterns only:

```text
guard let Type::Variant(binding) = expr else { ... }
```

## Lowering Shape

Parser output is sugar-only and lowers to existing AST pieces:

```text
local __guard_subject = expr
if match __guard_subject { Variant => false, other variants => true } {
  else body
}
local binding = match __guard_subject { Variant(binding) => binding, other variants => null }
```

The failure condition is an `EnumMatchExpr`, not a unary `!`, so Program(JSON v0)
can lower it through the existing enum-match lane.

## Stage Split

Stage0/parser owns:

```text
parse guard let Type::Variant(binding) = expr else block
validate that Type::Variant is a known single-payload enum variant
transport as existing ScopeBox / Local / If / EnumMatchExpr sugar
```

Stage1 owns:

```text
existing EnumMatchExpr lowering
existing known-enum/prelude diagnostics
future type-context and exhaustiveness refinements
```

Direct MIR owns:

```text
known enum constructor lowering to VariantMake
guard-let generated boolean EnumMatchExpr lowering through VariantTag + select
guard-let generated single-payload binding extraction through VariantProject
parser-emitted guard-let ScopeBox binding visibility for the following source statements
```

Stage1 does not own here:

```text
general pattern binding
else-side payload binding
unqualified Ok(value) canonical pattern
unit variant guard-let
record/tuple payload guard-let
try / throw / ? propagation
```

## Stop Lines

```text
no new AST guard-let node in GUARDLET-001
no broad pattern parser reuse yet
no source-level exception family
no implicit Result propagation
```

## Retire Condition

This parser sugar can retire or be rewritten when the selfhost parser/pattern
owner has a canonical pattern-binding representation that still lowers through
the same fail-fast enum-match semantics.
