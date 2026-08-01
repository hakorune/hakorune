# RAW-SCRIPT-ENUM-MATCH-DIRECT-SCRUTINEE0-I0-R0

Decision: Accept — T2, one atomic selected-Script cutover.

Prerequisite closed: `ENUM-MATCH-SOURCE-OWNER-FILE-SPLIT0-S0` moved ScopeBox
ownership to its private sibling, leaving `exprs_enum_match.rs` below 800 lines.

## Closure

```hako
enum Flag { On Off }
local x = Flag::On()
enum_match Flag x { On => true, Off => false }
```

Only a declaration-fact-proven final non-generic direct enum route may
Complete. The enum declaration is a typed retained transfer and the existing
variant producer supplies `x`; `EnumMatchScrutinee` is the only recursive child
demand. Arms are existing enum-owner observations, never Script lexical child
demands.

## One authority

Extract the positive direct-route preflight from `PreparedRawEnumMatchV1` into
one private kernel. Both the existing raw preparation (installed context) and a
borrowed declaration-facts `EnumMatchDemandV1` use it. The semantic view returns
only an opaque positive admission; it owns no diagnostic, schema copy, or AST.

```text
CatalogSeal -> declaration facts once -> borrowed enum-match view
-> shared Script traversal -> exact receipt -> Complete
-> existing enum lowering once
```

Negative preflight returns Deferred before scrutinee/arm observation; RootLower
retains its existing diagnostic/order. Raw/reference remain unchanged.

## Same-commit deletion

```text
positive direct EnumMatch
-> Script profile rejection -> Deferred -> bare script_root()
```

becomes receipt-backed Complete with one exact `EnumMatchScrutinee` source.

## Proof and stops

Require parser-driven selected/legacy MIR+verification parity, exact root
ordinal receipts, no arm descent, Deferred diagnostic parity for unknown,
generic, non-exhaustive, else, and unsupported-payload forms, plus fresh reuse.
Stop if policy is duplicated, mutable CompilationContext is read by semantics,
an enum map is cloned/recollected, arms become lexical demands, Call/Object is
generalized, or a touched source/check file reaches 800 lines.
