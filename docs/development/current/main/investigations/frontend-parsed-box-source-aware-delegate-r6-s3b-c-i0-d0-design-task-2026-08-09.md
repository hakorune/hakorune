---
Status: accepted design; implementation closed
Date: 2026-08-09
Decision: C-I0 batch boundary is accepted after independent authority audit; bounded implementation receipt is closed and R6-S3B-D remains unopened
Parent: `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-d0-design-task-2026-08-08.md`
Reference: `docs/development/current/main/design/parser-postpass-source-handoff-ssot.md`
---

# FRONTEND-PARSED-BOX-SOURCE-AWARE-DELEGATE-R6-S3B-C-I0-D0

## Purpose

Close the design boundary for the first source-aware delegate transaction
after the landed C-S1 private borrowed target index. This receipt closes the
design stop but does not open implementation in the same slice. Do not
implement generated placement, final-seal expansion, resolver targets,
Recipe/CallSlot, or runtime/provider routes here.

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

## Accepted product and ownership

The one C-I0 execution product is parser-private and unpublished:

```text
PreparedDelegatePostpassBatchV1
  owns staged per-host generated AST batches
  owns expected BoxMethodInventoryPlacementReceiptV1 rows
  owns GeneratedDelegateSourceRelationV1 rows
  owns exact host/target source paths and target method source references
```

`OpenParserPostpassProductV1::prepare_delegate_batch` (the exact method name
may be chosen during implementation) borrows the C-S1
`DelegateTargetIndexV1<'product>` only while preparing. The staged product
owns every path, source relation reference, generated name, and placement
receipt before it is moved into the commit step. No C-I0 relation is a
`Verified*` semantic product and no second final source seal is issued.

The parser source session remains the sole transport owner. The generated
relation rows must be carried inside the prepared source payload through
prune and finalization so R6-S3B-D can verify complete coverage without
re-scanning AST or inventing a second source authority. C-I0 still keeps those
rows outside `ParserBoxSourceSealV1`; only D may promote complete relation
coverage into the final non-Clone seal.

The C-S1 borrowed target reference currently carries the exact target path and
explicit source relation. C-I0 may obtain a borrowed descriptive target
method declaration/signature view through the same index/product. That view is
only input to deterministic forwarding-method construction; it is not source
identity, a semantic contract, or a resolver target.

## Atomic preflight and commit

The postpass is a consume-return transaction. No AST or inventory mutation is
observable until the complete batch is preflighted:

```text
1. collect every ordinary host Box and align it to the same-brand source path
2. match every parser-issued DelegateSourceDeclarationV1 expose exactly once
3. resolve every target field/type and explicit target method through C-S1
4. obtain the descriptive target method signature needed for the forwarder AST
5. preflight generated names, existing names, duplicate exposes, and collisions
6. stage each host batch against a clone/staging inventory
7. record one placement receipt per generated method and pair it with its
   owned GeneratedDelegateSourceRelationV1
8. verify zero orphan/duplicate source rows and complete host/expose coverage
9. commit the staged AST, inventory placements, and relation payload once
10. return one new OpenParserPostpassProductV1
```

The staged inventory is only a placement calculator. It never becomes source
authority and it never replaces the prepared explicit source inventory. The
commit step compares actual placement receipts with the staged expected rows
before replacing the product's AST/source-session fields. A zero-delegate
ordinary program is a valid exact no-op and returns a fresh product without
generated rows.

`GeneratedDelegateSourceRelationV1` must contain, at minimum:

```text
host Box source path + host delegate member source site
expose ordinal
delegate field, source method, exposed/generated method names
target Box source path
existing explicit target method source reference
generated inventory placement receipt
generated-name provenance
```

The row is parser evidence, not a resolver or Recipe product. Names are
selectors/diagnostics; source paths, expose ordinal, existing source relation,
and exact placement are the identity evidence.

## Complete failure/disposition matrix

The whole unpublished postpass product is consumed and dropped on every
failure. There is no partial per-host commit, rollback repair, same-session
retry, or name-based fallback.

```text
NoSafeSlice:
  required staged issuer/typed transaction is not implemented (development
  state only; never a source disposition)

Rejected:
  foreign brand/path; duplicate or ambiguous host/expose; orphan or duplicate
  parser row; AST/source-row mismatch; generated-name collision; missing method
  relation for a present target; malformed provenance; staged-vs-actual
  placement mismatch; commit cardinality mismatch

Unresolved:
  incomplete source path, field/type, target signature, or source alignment
  evidence needed to decide the bounded cohort

Declined:
  fully observed but outside C-I0: generated-only target, delegate chain,
  CompatibilityOnly, interface/static/record, Hako/provider declaration,
  overload, or ambiguous unsupported cohort

Candidate:
  every host/expose is exact, all staged batches and relation rows are
  complete, and the one consume-return commit is ready
```

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

The accepted answers are: the staged batch owns all preflight evidence;
`ParserSourceSessionV1` transports relation rows through finalization; C-I0
owns placement and complete generated-batch coverage; R6-S3B-D alone issues
the resolver-visible final relation/seal coverage.

## Nonclaims

```text
no implementation
no final ParserBoxSourceSealV1 extension
no resolver target catalog
no Recipe/CallSlot/Builder/MIR/provider/runtime
no fallback/retry or AST rewrite
```

## Exit gate

The design stop is now closed by this accepted receipt. Do not open C-I0
implementation until the dedicated implementation card and guard are present.
The implementation slice must update the source-handoff SSOT, parser README,
language reference receipt, focused tests, and guard in the same commit.
Keep all later relation/seal authority closed until R6-S3B-D.
