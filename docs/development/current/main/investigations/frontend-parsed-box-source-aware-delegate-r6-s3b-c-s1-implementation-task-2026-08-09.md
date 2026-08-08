---
Status: closed implementation receipt
Date: 2026-08-09
Decision: implement only the accepted private borrowed target-index slice; do not open generated placement or final seal
Parent: `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-s1-d0-design-task-2026-08-09.md`
Reference: `docs/development/current/main/design/parser-postpass-source-handoff-ssot.md`
---

# FRONTEND-PARSED-BOX-SOURCE-AWARE-DELEGATE-R6-S3B-C-S1

## Scope

Implement the smallest parser-private target lookup product after C-S0:

```text
prepared source seals
  -> same-brand exact Box-path index
  -> one target Box candidate
  -> one existing explicit method source relation
  -> borrowed target reference
```

The product is a lookup/borrow proof only. It must not mutate AST, method
inventory, prepared seals, final seals, or generated delegate rows.

The owner is a private `DelegateTargetIndexV1<'product>` borrowing
`OpenParserPostpassProductV1`. Since source paths have no `Hash`/`Ord`
identity, use a `Vec<TargetBoxEntryV1>` with a private name-to-candidate-index
view. Every returned `TargetMethodRef<'product>` must re-check same-brand exact
path equality and match one existing explicit method relation by source site
and placement. The same borrowed reference may be reused by multiple exposes.

## Authority and API boundary

```text
authority:
  ParserInvocationBrandV1
  SourceBoxDeclarationPathV1
  existing ExplicitMethodSourceRelationV1

query selectors only:
  host delegate field name
  field declared type name
  expose source method name

returned private capability:
  exact target Box path
  borrowed explicit method source relation
```

The index is an implementation detail. A private linear vector or a private
derived map is permitted, but map order/key identity must not escape. No
`HashMap` or name-only target is a source authority.

## Bounded cohort

```text
ordinary top-level Rust Boxes
same parser invocation/brand
field has one declared target Box type
target path is unique
target method is one explicit source relation
```

Generated-property-only, generated-delegate, delegate-chain,
compatibility-only, interface/static/record/Hako/provider, and overload cases
are outside this row.

## Acceptance tests

```text
positive:
  one target path + one explicit method relation -> borrowed candidate
  same target borrowed by multiple exposes

unresolved:
  missing field declaration/type
  incomplete target source inventory or path alignment

declined:
  complete same-brand index but no admitted ordinary target
  generated-only target
  delegate-chain target
  compatibility-only target
  overload/outside-cohort target

rejected:
  foreign invocation brand/path
  duplicate Box path/name candidate
  duplicate or contradictory method relation
  missing explicit method for a present target
  source-path/relation/placement mismatch
```

The focused suite must prove no AST/inventory mutation and that a failed query
does not create a partial target capability. A fresh parser product can repeat
the same query after a prior success or failure.

## Closeout requirements

The implementation commit must update, in the same slice:

```text
parser source-handoff SSOT
callable-contracts reference
parser/source-authority README or target-index owner README
this task receipt
dedicated C-S1 guard and check-scripts index
CURRENT_STATE/current task map
```

Every touched Rust source file remains below 800 lines. The guard must assert
that no final seal extension, resolver target catalog, generated placement,
Recipe/CallSlot, Builder/MIR, provider/runtime route, fallback, retry, or AST
rewrite was added.

## Nonclaims

```text
no GeneratedDelegateSourceRelation
no all-host/expose batch preflight
no generated AST/inventory commit
no final ParserBoxSourceSealV1 extension
no resolver-visible target catalog
no Recipe/CallSlot/Builder/MIR/ABI/Home/provider/runtime
no Hako parity
```

## Landed C-S1 receipt (2026-08-09)

`src/parser/delegate_target_index.rs` now owns the bounded private index. It
borrows the unpublished postpass product, aligns the pruned ordinary-Box AST
with parser-issued prepared seals and exact `SourceBoxDeclarationPathV1`
values, and validates existing explicit method relations before issuing the
index. A query uses the host field's declared type and expose source method
only as selectors; a successful `TargetMethodRefV1` carries the exact target
path and borrowed explicit relation. The same reference can be queried again
without consuming or mutating the product.

The focused suite covers the positive reusable candidate, missing field
(`Unresolved`), missing method (`Rejected` without fallback), and duplicate
target name (`Rejected` at index issue). No AST/inventory/prepared-seal/final-
seal mutation, generated placement, batch commit, resolver target, or runtime
route was added. All touched Rust source files remain below 800 lines.

The accepted next boundary is the separately tracked C-I0 implementation for
all-host/expose preflight and one atomic generated-batch commit; this row does
not open that implementation.
