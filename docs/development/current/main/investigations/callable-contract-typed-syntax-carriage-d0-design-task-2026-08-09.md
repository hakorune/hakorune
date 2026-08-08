---
Status: accepted design stop — implementation not opened
Date: 2026-08-09
Decision: parser-owned typed `CallableContract(query)` syntax carriage only
Parent: `callable-contract-and-instance-call-implementation-task-map-2026-08-08.md`
Reference: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-CONTRACT-TYPED-SYNTAX-CARRIAGE-D0

## Decision

The next bounded row carries the accepted source rune into a typed parser
syntax DTO. It does not issue resolver meaning.

```text
parser-owned rich Box source seal
  + exact method/rune source site
  + raw rune attribute
      ↓
CallableContractSyntaxV1::Query
      ↓
future resolver declaration/profile issuer
```

The accepted source spelling is:

```hako
@rune CallableContract(query)
```

The parser normalizer owns syntax validation and source-site preservation.
Resolver owns semantic declaration, signature, Home ABI, effect/control, and
target issuance in later rows.

## Source authority

```text
authoritative:
  ParsedProgramWithSourceV1 / parser-owned non-Clone source seal
  SourceBoxMethodSiteV1
  exact rune attribute site and declaration placement

descriptive only:
  cloneable BoxMethodInventoryV1
  inventory ordinal / selected placement
  JSON compatibility projection
  AST-only compatibility projection
```

The parser must not recover source identity from names, HashMap order, JSON,
or selected/generated inventory ordinal. Generated/property rows cannot issue
the first explicit source contract.

## Typed syntax contract

```rust
CallableContractSyntaxV1::Query {
    source_site: CallableContractRuneSiteV1,
}
```

This DTO is syntax, not a `Verified*` semantic product. It carries no:

```text
Home demand
semantic parameter/result types
Pure/Query body proof
effect mask
suspension/control proof
ABI or MirType
resolver target
Recipe/CallSlot
Builder/MIR/CFG/PHI
```

No raw rune name/value strings may cross the parser-to-resolver boundary.

## Fail-fast matrix

```text
unknown CallableContract value       -> parser error
wrong placement                     -> parser error
duplicate CallableContract rune     -> parser error
duplicate method declaration         -> parser error
conflicting Profile/Contract rune   -> parser error
foreign/stale source seal            -> Rejected at source handoff
missing typed syntax row             -> Declined for a source without it
issuer not implemented               -> NoSafeSlice development state
```

The parser does not silently ignore an unknown value or fall back to the raw
rune representation. `CallableContract(query)` remains non-repeatable and
declaration-local for the first cohort.

## Implementation boundary

The later I0 may add only:

1. Rust parser and `.hako` parser parity for the typed DTO;
2. exact source-site and placement tests;
3. duplicate/unknown/conflicting metadata negatives;
4. parser module README and language/reference receipt updates in the same
   implementation commit;
5. existing parser guard integration without a new semantic owner.

No resolver issuer, Home ABI, instance target, source-bound relation, Recipe
CallSlot projection, ScanWithInit observer, physicalizer, production selector,
fallback, retry, or legacy retirement is opened by this D0.

## Acceptance and stop lines

```text
positive: exact Query syntax preserves one method/rune site
negative: unknown value, duplicate rune, wrong placement, conflicting rune
parity: Rust/.hako normalized syntax agrees
guard: source seal remains the only parser authority
budget: every changed Rust source file stays below 760 lines
```

If the rich parser seal cannot provide the exact method/rune site, stop at
`NoSafeSlice`; do not synthesize it from inventory placement. After this D0,
the implementation row must close before resolver declaration or S6C scan
work is selected.
