---
Status: design stop
Date: 2026-08-09
Decision: C-S1 target lookup boundary is not opened; design only
Parent: `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-d0-design-task-2026-08-08.md`
Reference: `docs/development/current/main/design/parser-postpass-source-handoff-ssot.md`
---

# FRONTEND-PARSED-BOX-SOURCE-AWARE-DELEGATE-R6-S3B-C-S1-D0

## Purpose

Define the next private target-lookup boundary after the closed C-S0 parser
transport. This is a design-only row. It must not add a resolver target,
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
