---
Status: planned design stop; implementation not opened
Date: 2026-08-09
Decision: C-I0 batch boundary requires a separate design receipt after C-S1
Parent: `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-d0-design-task-2026-08-08.md`
Reference: `docs/development/current/main/design/parser-postpass-source-handoff-ssot.md`
---

# FRONTEND-PARSED-BOX-SOURCE-AWARE-DELEGATE-R6-S3B-C-I0-D0

## Purpose

Close the design boundary for the first source-aware delegate transaction
after the landed C-S1 private borrowed target index. This is a design-only
row. Do not implement generated placement, final-seal expansion, resolver
targets, Recipe/CallSlot, or runtime/provider routes here.

## Required authority

```text
parser-issued DelegateSourceDeclarationV1 rows
  + C-S1 exact borrowed target references
  + generated inventory placement receipts
  + one postpass-owned batch transaction
```

The C-I0 design must specify one complete preflight over all hosts/exposes,
including exact host/target paths, source methods, generated names, inventory
placement, collision policy, and relation rows. It must state which existing
parser source rows are consumed and which relation fields are issued. Names
and generated inventory order remain selectors/placement only, never source
identity.

## Mandatory questions

1. What product owns the complete staged batch before AST/inventory mutation?
2. How are all host/expose queries preflighted before any generated suffix is
   committed?
3. What exact placement receipt is paired with each generated method and
   source relation row?
4. How are duplicate, foreign, missing, generated-only, chained, and
   compatibility-only cases classified?
5. How does any failure discard the whole unpublished postpass product with no
   same-session retry or partial host commit?
6. Which relation coverage is still private C-I0 and which belongs only to
   final R6-S3B-D seal issuance?

## Nonclaims

```text
no implementation
no final ParserBoxSourceSealV1 extension
no resolver target catalog
no Recipe/CallSlot/Builder/MIR/provider/runtime
no fallback/retry or AST rewrite
```

## Exit gate

Do not open C-I0 implementation until this card has an accepted design
receipt in the parser source-handoff SSOT, a typed failure/discard matrix, a
focused guard contract, and an explicit same-commit reference/task update
plan. Keep all later relation/seal authority closed until that receipt is
accepted.
