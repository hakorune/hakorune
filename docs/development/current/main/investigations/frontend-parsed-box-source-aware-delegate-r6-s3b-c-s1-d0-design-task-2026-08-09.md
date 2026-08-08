---
Status: accepted design; implementation closed
Date: 2026-08-09
Decision: C-S1 target lookup boundary is closed by its bounded implementation receipt
Parent: `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-d0-design-task-2026-08-08.md`
Reference: `docs/development/current/main/design/parser-postpass-source-handoff-ssot.md`
---

# FRONTEND-PARSED-BOX-SOURCE-AWARE-DELEGATE-R6-S3B-C-S1-D0

## Purpose

Define the next private target-lookup boundary after the closed C-S0 parser
transport. This was a design-only row. It must not add a resolver target,
generated placement relation, final source-seal field, or name-based fallback.

## Authority to preserve

```text
parser-issued same-brand Box source paths
  + exact explicit method source relations
  + parser-private DelegateSourceDeclarationV1 rows
    -> private target index / target relation proof
```

`BoxMethodInventoryV1`, generated suffix order, AST order, and `HashMap` order
remain descriptive or compatibility-only. Inventory ordinals are placement
coordinates, never declaration identity. C-S0 remains the only landed C
implementation: rows are carried through `PreparedBoxSourceSealV1` and are
dropped by the final resolver-visible source seal.

## Design questions to close before implementation

1. What exact path key indexes a target Box without reconstructing source
   identity from names or inventory ordinals?
2. How does a target method relation borrow from the same parser invocation
   brand and remain reusable for multiple exposes?
3. Which target cases are `Candidate`, `Declined`, `Unresolved`, or `Rejected`?
4. How are missing target Box, missing explicit method, foreign brand, duplicate
   method relation, generated-only target, and delegate-chain targets diagnosed?
5. What is the smallest private target index product that can be consumed by a
   later all-host/expose batch preflight without becoming a second source
   authority?

## Required result of this design stop

```text
one target-index owner
one exact source-path/method-relation key
one reusable borrowed target reference shape
one failure/disposition matrix
one C-S1 implementation task with focused fixtures and guard contract
```

The design must state that the target index is a private lookup aid. The exact
parser-issued path and existing explicit method source relation remain the
semantic authority. Any candidate design that uses method name, exposed name,
inventory ordinal, AST order, or runtime/provider lookup as identity is
rejected.

## Accepted C-S1 boundary

The C-S1 product is a parser-private borrowed index, not a new source seal or
resolver target catalog:

```text
OpenParserPostpassProductV1
  -> DelegateTargetIndexV1<'product>
       exact SourceBoxDeclarationPathV1 -> source-backed target entry
       explicit MethodSourceRelationV1 rows only
```

`DelegateTargetIndexV1<'product>` borrows the unpublished postpass product's
source session and AST. It is not stored, published, or passed to the resolver.
Because `SourceBoxDeclarationPathV1` deliberately has no `Hash`/`Ord` identity,
the minimal implementation is a `Vec<TargetBoxEntryV1>` with a private
name-to-candidate-index lookup view. Exact path equality and parser-brand
identity are checked before every borrowed result; map order and map keys do
not escape as source authority.

Each entry carries the exact target path, a diagnostic Box-name label, and
explicit method relation entries. A borrowed `TargetMethodRef<'product>` may
be reused by multiple exposes and is not one-shot.

The target Box selector is the host delegate field's declared type name. The
field name identifies the host field; the declared type name only selects
candidate source paths. The bounded cohort requires exactly one same-brand Box
path for that selector. Missing source path/alignment or missing field/type
evidence is `Unresolved`. If the same-brand index is complete but no admitted
ordinary target exists, the target is `Declined`; multiple candidates are
`Rejected` as ambiguous source identity.

Within the selected target path, the source method selector is the delegate's
`source_method_name`. C-S1 admits exactly one existing explicit
`MethodSourceRelationV1::Explicit` row, matching its target path, source site,
and placement. A generated-property-only target, generated-delegate target,
delegate chain, compatibility-only target, or overload/ambiguous row is
`Declined` when fully observed. A missing explicit method for an otherwise
present target, foreign/duplicate/contradictory relation, or path/brand
mismatch is `Rejected`; there is no name-only fallback.

The reusable result is a private borrowed target reference:

```text
TargetMethodRef<'product>
  target_box_path: SourceBoxDeclarationPathV1
  explicit_method_source: &'product ExplicitMethodSourceRelationV1
```

It contains no generated inventory placement, AST node, function pointer,
resolver identity, Recipe key, ValueId, provider handle, or runtime route.
Multiple exposes may borrow the same target reference. C-S1 does not mutate
the AST, inventory, prepared seal, or final source seal.

## C-S1 disposition precedence

```text
NoSafeSlice
  canonical private target-index/source-relation issuer is not implemented

Rejected
  foreign brand/path, duplicate Box candidate, duplicate/contradictory source
  relation, malformed source capability, or ambiguous target identity

Unresolved
  source field/type or target source inventory is unavailable/incomplete

Declined
  fully observed generated-only target, delegate chain, compatibility-only
  target, overload, or other outside-cohort target shape

Candidate
  exactly one same-brand target path and exactly one existing explicit method
  source relation are borrowed without mutation
```

`NoSafeSlice` is development state and is never serialized as a source
disposition. C-S1 remains a lookup/borrow proof; complete host/expose
preflight, generated placement, and atomic AST/inventory/relation commit stay
in C-I0.

## Nonclaims

```text
no target implementation
no GeneratedDelegateSourceRelation
no generated placement relation
no all-host/expose batch commit
no final ParserBoxSourceSealV1 extension
no resolver target catalog
no Recipe/CallSlot/Builder/MIR/provider/runtime
no Hako parity
no fallback/retry or AST rewrite
```

## Exit gate

Do not open C-S1 implementation until this card has an accepted design receipt
in the parser source-handoff SSOT, an explicit negative matrix, and a focused
guard/task entry. The implementation slice must update its reference and owner
README in the same commit and keep every touched Rust source file below 800
lines.

The child C-S1 implementation receipt is now closed. The accepted next
execution frontier is the unopened C-I0 implementation for all-host/expose
preflight and one atomic generated-batch commit.
