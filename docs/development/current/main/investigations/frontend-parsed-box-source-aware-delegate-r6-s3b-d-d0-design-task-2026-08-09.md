---
Status: accepted design boundary; implementation not opened
Date: 2026-08-09
Decision: R6-S3B-D is the sole final generated-delegate source-seal extension
Parent: `docs/development/current/main/design/parser-postpass-source-handoff-ssot.md`
Next: one bounded final-seal implementation slice after this design stop
---

# FRONTEND-PARSED-BOX-SOURCE-AWARE-DELEGATE-R6-S3B-D-D0

## Decision

R6-S3B-D is the final parser-source boundary for the bounded ordinary Rust Box
cohort. It consumes the already prepared parser-private relation rows from
`ParsedProgramWithSourceV1` and issues the sole resolver-visible
`ParserBoxSourceSealV1` that contains complete generated-delegate coverage.

```text
ParsedProgramWithSourceV1
  ├─ final AST
  ├─ ordinary explicit/property source seals
  └─ parser-private GeneratedDelegateSourceRelationV1 rows
        │
        ▼ one finalizer-owned coverage check
ParserBoxSourceSealV1
  ├─ explicit/property MethodSourceRelation rows
  ├─ generated delegate relation rows
  ├─ final inventory placement receipts
  └─ same-brand source/path coverage
```

The finalizer-owned relation/placement coverage plan performs no AST/name
reconstruction and does not rescan ASTs or rebuild rows from inventory
ordinals, or infer a semantic callable contract. It consumes the prepared
relation transport issued by C-I0 and verifies that the final AST/inventory
placement is exactly the placement already recorded by the parser transaction.

No second source registry, resolver target, `CallableContract`, Recipe,
`CallSlot`, Builder/MIR, provider, runtime, or fallback route opens in D.

## Sole owners

| Meaning | Sole owner | Forbidden reconstruction |
| --- | --- | --- |
| final AST and selected inventory | `OpenParserPostpassProductV1` | AST/name re-scan after seal preparation |
| explicit/property source relations | `PreparedBoxSourceSealV1` | method names, inventory order |
| generated delegate relation rows | C-I0 parser session payload | generated suffix/name inference |
| exact generated placement | `GeneratedDelegateSourceRelationV1.generated_inventory_placement` | recomputing from final ordinal |
| complete relation coverage | private D finalizer coverage plan | per-row ad hoc checks in resolver |
| resolver-visible source authority | non-Clone `ParserBoxSourceSealV1` | any early/test constructor |

`BoxMethodInventoryOrdinalV1` remains a placement receipt only. A source
declaration identity comes from the parser-issued `SourceBoxMethodSiteV1` and
same-brand Box path carried by the relation row.

## Final aggregate shape

The final seal may extend its prepared payload with an owned relation set, but
must keep the aggregate closed and non-Clone:

```rust
ParserBoxSourceSealV1 {
    prepared: PreparedBoxSourceSealV1 {
        brand,
        box_site,
        inventory,
        method_relations,
        delegate_source_declarations: consumed,
        generated_delegate_source_relations,
    },
}
```

The parser-internal projection exposes generated relations only through this
final seal after complete coverage succeeds. The rich parsed product may
retain a diagnostic/read-only collection for the handoff, but that collection
is not a second authority.

## Coverage contract

For every retained ordinary Box path, D verifies all of the following before
issuing any final seal:

```text
1. one prepared source seal per final ordinary Box path
2. one final AST Box inventory per prepared path
3. explicit/property relation rows match the prepared inventory prefix
4. every parser-issued delegate expose has exactly one generated relation row
5. every generated relation host path equals the retained Box path
6. target path and target method source relation retain the same parser brand
7. generated inventory placement equals the final inventory placement
8. generated provenance is Delegate and has no orphan/non-delegate suffix
9. no duplicate expose key, relation anchor, host path, or generated placement
10. no relation row exists without a final AST/inventory counterpart
```

The relation key for coverage is:

```text
(host Box source path, delegate member source site, expose ordinal)
```

Names remain diagnostic/query attributes. They are not declaration identity.
The finalizer must compare a canonical relation-key set and placement-receipt
set, not rely on vector order. Deterministic ordering is allowed only for
diagnostics and serialized evidence.

## Disposition and failure boundary

```text
NoSafeSlice
  finalizer issuer or complete relation transport is not available

Rejected
  foreign brand/path, duplicate key, orphan row, placement mismatch,
  missing final AST counterpart, contradictory provenance, or incomplete
  relation coverage after all required source rows were observable

Unresolved
  required source path, final inventory, target relation, or placement
  evidence is unavailable

Declined
  fully observed Box/delegate is outside the bounded ordinary direct-target
  cohort (generated chain, compatibility-only, interface/static/record/Hako,
  provider, overload, or ambiguous target policy)

Candidate
  the complete retained product passes all coverage checks and is ready for
  the sole non-Clone final seal issuer
```

The finalizer never converts `NoSafeSlice` into a source disposition by adding
a test constructor or by retaining the old generated-suffix adapter.

## Transaction and publication rules

```text
1. consume `ParsedProgramWithSourceV1` only after C-I0 has committed all rows
2. build a private final coverage plan without mutating the input product
3. reject the whole unpublished product on any coverage failure
4. issue all final non-Clone seals together
5. return the final parsed product once; no same-session retry
6. only then retire the S3A generated-suffix compatibility adapter
```

no partial seal, retry, or fallback is allowed.

The finalizer is still parser-owned. Resolver code receives the resulting
opaque source seal and must not call back into AST, postpass, or inventory
placement helpers.

## Bounded implementation cohort

The first implementation opens only for:

```text
ordinary top-level Rust Box declarations
same parser invocation/source brand
direct explicit target method delegates
one relation row per expose
existing explicit/property source relations
```

It does not open:

```text
generated-delegate chains
compatibility-only delegates
interface/static/record declarations
Hako parser parity
provider/plugin declarations
overloads or ambiguous target selection
resolver target catalog
semantic callable contract or body conformance
```

## Acceptance matrix for the future implementation row

```text
positive:
  direct delegate with one expose
  direct delegate with multiple exposes
  selected build-gate host after path rebase
  property/generated prefix plus delegate suffix
  zero-delegate ordinary program remains an exact no-op

negative:
  missing relation row
  duplicate relation key
  foreign parser brand/path
  orphan generated inventory row
  staged-vs-final placement mismatch
  non-delegate generated suffix
  final AST Box path missing or duplicated
  outside-cohort target/provenance
```

The future implementation card must add focused tests, the parser README,
the language/source reference receipt, and a guard in the same commit. It must
also prove that the old S3A generated-suffix adapter has no production caller
before deleting it.

## Explicit nonclaims before D-I0

```text
no resolver declaration/target issuance
no CallableContract syntax or Home/ABI contract
no Recipe/CallSlot/Builder/MIR connection
no provider/runtime dispatch
no Hako parser parity
no generated-delegate chain semantics
no production selection, fallback, retry, or module publication
```

This is a design receipt only. The current worktree remains at a clean
closeout boundary; implementation is not opened until the current-state mode
is explicitly changed to a bounded D implementation row.
