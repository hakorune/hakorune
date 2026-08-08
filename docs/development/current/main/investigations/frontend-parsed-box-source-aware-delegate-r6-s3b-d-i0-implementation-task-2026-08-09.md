---
Status: closed implementation receipt
Date: 2026-08-09
Decision: implement only the accepted R6-S3B-D final source-seal extension
Parent: `frontend-parsed-box-source-aware-delegate-r6-s3b-d-d0-design-task-2026-08-09.md`
---

# FRONTEND-PARSED-BOX-SOURCE-AWARE-DELEGATE-R6-S3B-D-I0

## Scope

This is the active bounded implementation row after the D0 design stop. It
consumes the parser-private `GeneratedDelegateSourceRelationV1` rows
already carried by `ParsedProgramWithSourceV1`, verify complete final AST and
inventory coverage, and issue the sole non-Clone `ParserBoxSourceSealV1` with
generated-delegate relations included.

It must not add a resolver target, semantic callable contract, Recipe/CallSlot,
Builder/MIR route, provider/runtime route, fallback, retry, or AST rewrite.
no fallback is permitted after the finalizer starts coverage.
There is no partial seal, fallback, or retry: any coverage failure discards the
unpublished parsed product.

## Required implementation

```text
1. finalizer-owned relation coverage plan
2. canonical relation-key and placement-receipt comparison
3. same-brand host/target/path validation
4. explicit/property prefix plus generated delegate suffix validation
5. one final non-Clone seal issuer
6. removal of the bounded S3A generated-suffix adapter only after caller-zero
7. focused positive/negative tests and a reusable guard
```

The finalizer must consume the prepared payload. It may not re-read AST names,
rebuild source rows from inventory ordinals, or call the private target index.
The exact placement receipt is the C-I0
`GeneratedDelegateSourceRelationV1::generated_inventory_placement` field; the
finalizer must not replace it with AST/name/ordinal reconstruction (no AST/name/ordinal recovery).

## Acceptance matrix

```text
positive:
  one expose
  multiple exposes
  selected-gate host after path rebasing
  explicit/property prefix followed by delegate suffix
  zero-delegate exact no-op

negative:
  missing/duplicate relation key
  foreign brand/path
  orphan generated inventory row
  staged-vs-final placement mismatch
  non-delegate generated suffix
  duplicate final AST Box path
  outside-cohort target/provenance
```

## Required same-slice updates

The implementation commit must update all of these together:

```text
src/parser/source_seal.rs (or an owner split before the 760-line trigger)
src/parser/source_seal_* focused tests
src/parser/README.md
docs/development/current/main/design/parser-postpass-source-handoff-ssot.md
docs/reference/language/callable-contracts.md
this task receipt and the task map
the D-I0 guard and docs/tools/check-scripts-index.md
CURRENT_STATE.toml only when the row actually opens/closes
```

## Stop lines

If final relation ownership, placement identity, or complete source coverage
cannot be issued from the existing parser payload, stop with `NoSafeSlice` and
return to D0 design. Do not add a test constructor, name fallback, sidecar
ordinal, partial seal, or resolver adapter.

The row closes when the bounded rich path has zero direct legacy delegate
callers, complete final relation coverage, and green focused tests, guard,
source-file line-count guard, and same-slice documentation receipt. The three
wide public AST-only callers are not part of this bounded row; their later
cutover is tracked by
`parser-public-ast-postpass-cutover-d0-design-task-2026-08-09.md`.

## Landed bounded implementation receipt

The finalizer implementation and focused tests are landed. The bounded rich
path `parse_from_string_with_source_seal` now has zero direct
`delegate_lowering::lower_delegate_exposes` callers and retains complete
generated relation rows in the sole `ParserBoxSourceSealV1` after exact
same-brand relation-key, provenance/selection, and placement coverage. The
wide public AST-only callers remain compatibility nonclaims because the rich
finalizer is ordinary-Box-only; their total postpass/caller cutover is a
separate design stop.
