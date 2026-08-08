---
Status: planned; implementation not opened
Date: 2026-08-09
Decision: implement only the accepted C-I0 parser-private atomic delegate batch
Parent: `docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-d0-design-task-2026-08-09.md`
Reference: `docs/development/current/main/design/parser-postpass-source-handoff-ssot.md`
---

# FRONTEND-PARSED-BOX-SOURCE-AWARE-DELEGATE-R6-S3B-C-I0

## Scope

Implement the accepted parser-private C-I0 batch boundary:

```text
all ordinary host/expose rows
  -> complete preflight
  -> staged generated AST/inventory/relation batch
  -> one consume-return postpass commit
```

This row does not issue a resolver target or extend the final source seal.
`GeneratedDelegateSourceRelationV1` remains parser evidence and must reach the
later R6-S3B-D finalizer without being re-created from AST or names.

## Required product and owners

Implement one private `PreparedDelegatePostpassBatchV1` (or a clearly
equivalent name) that owns:

```text
per-host generated method drafts
expected BoxMethodInventoryPlacementReceiptV1 rows
owned GeneratedDelegateSourceRelationV1 rows
exact same-brand host/target paths
existing explicit target method source references
```

`DelegateTargetIndexV1<'product>` is borrowed only during preparation. The
index must expose a borrowed descriptive target method declaration/signature
view needed by the existing forwarding-method constructor. That view is not a
new semantic authority.

`ParserSourceSessionV1` or its prepared payload is the only transport owner for
generated relation rows through prune and finalization. `ParserBoxSourceSealV1`
must remain unchanged in this row.

## Ordered implementation steps

1. Add a complete all-host/expose coverage census. Match every parser-issued
   `DelegateSourceDeclarationV1` to exactly one final AST host/member/expose;
   reject orphan, duplicate, foreign, or missing rows.
2. Extend the private C-S1 index with a borrowed descriptive target method
   declaration/signature accessor; do not copy source identity into it.
3. Prepare forwarding method drafts for every admitted expose. Validate target
   field/type, explicit target relation, generated name, source/provenance, and
   duplicate/collision policy before any product mutation.
4. Stage each host batch against a clone/staging inventory and retain one
   placement receipt per generated method.
5. Construct owned `GeneratedDelegateSourceRelationV1` rows by pairing exact
   target source references with the staged placement receipts.
6. Commit all staged AST, inventory placement, and relation payload once via
   consume-return. Verify actual placement receipts equal expected receipts.
7. Add fresh-product success/failure tests, including zero-delegate no-op,
   multiple hosts/exposes, collision, orphan/duplicate relation, placement
   mismatch, and failure after an earlier host was staged.

## Disposition and failure contract

```text
NoSafeSlice:
  required staged issuer/typed transaction missing (development state only)
Rejected:
  foreign/duplicate/mismatch/orphan/collision/placement/cardinality failure
Unresolved:
  incomplete source path, field/type, signature, or source alignment evidence
Declined:
  generated-only, delegate-chain, CompatibilityOnly, interface/static/record,
  Hako/provider, overload, or other fully observed outside-C target
Candidate:
  complete exact preflight and staged batch ready for one commit
```

Every failure consumes and drops the whole unpublished postpass product. No
partial host commit, rollback repair, same-session retry, or name fallback is
allowed. A zero-delegate ordinary program is a valid exact no-op.

## Acceptance gates

```text
focused parser tests for all-host/expose coverage and atomic discard
source row ↔ AST one-to-one proof
staged-vs-actual placement receipt equality
fresh-product repeat after success and failure
source_seal.rs / source_authority.rs / delegate modules < 800 lines
guard and parser README/reference update in the same commit
```

## Explicit nonclaims

```text
no ParserBoxSourceSealV1 extension
no resolver declaration/target catalog
no CallableContract/Home/ABI semantics
no Recipe/CallSlot/Builder/MIR/provider/runtime
no Hako parser parity
no generated-delegate chain semantics
no fallback/retry/AST rewrite
no production selection or legacy deletion
```
