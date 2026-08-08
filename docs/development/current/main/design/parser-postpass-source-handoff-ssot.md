---
Status: accepted design; R6-S3B-A and R6-S3B-B1 closed; R6-S3B-B2 design stop; B3 not opened
Date: 2026-08-08
Decision: one typed parser postpass product owns AST and source transport
Related:
  - docs/development/current/main/investigations/frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md
  - docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md
  - docs/reference/language/callable-contracts.md
---

# Parser postpass source handoff

## Decision

R6-S3B uses one move-only postpass product. AST, parser source candidates,
build-gate selection state, and prepared Box transactions cross prune and
delegate lowering together:

```text
parse
  -> OpenParserPostpassProductV1
       { ast, source_session, diagnostic metadata }
  -> build-gate prune/rebase
  -> source-aware delegate lowering
  -> finalizer
  -> ParsedProgramWithSourceV1
       { final AST, complete non-Clone source seals }
```

There is no detached `ASTNode` postpass plus a separately registered seal
vector. A vector or map used inside the product is storage, not a second
authority. The product is the sole owner of the relationship between the AST
and its source records.

## Authority table

| Meaning | Sole owner | Forbidden reconstruction |
| --- | --- | --- |
| ordered AST method placement | `BoxMethodInventoryV1` | source identity, resolver target |
| parser invocation identity | `ParserSourceSessionV1` | names, spans, `HashMap`, JSON |
| Box/member source identity | typed `SourceBoxDeclarationPathV1` in the session | inventory ordinal, post-prune ordinal |
| unpublished Box rows and relations | per-Box transaction held by the session | detached AST inventory mutation |
| selected build-gate branch | postpass product prune transaction | final-AST ordinal guessing |
| generated delegate source relation | source-aware delegate transaction | generated suffix alone |
| final source authority | finalizer after all postpasses | early `Prepared*`, AST-only constructor |

`BoxMethodInventoryV1` remains cloneable descriptive data. Only the final
`ParserBoxSourceSealV1` is non-Clone and resolver-grade. Until generated
delegate relations exist, a generated suffix is not enough to issue a complete
resolver-grade seal.

## Typed source path

`SourceBoxDeclarationSiteV1 { statement_ordinal }` is sufficient for direct
top-level ordinary Boxes but not for Boxes parsed inside a top-level build gate.
S3B must extend the parser-private site to a brand-bound structural path:

```text
SourceBoxDeclarationPathV1 {
  invocation_brand,
  segments: [
    TopLevelStatement { ordinal },
    BuildGateBranch { gate_id, branch_index, item_index },
    ...
  ]
}
```

The path is source identity, not an inventory placement or a post-prune
ordinal. Member-level gate paths remain a separate method-source relation and
are nested under the Box path; the two ordinal vocabularies must not be
merged.

The parser issues paths while reading source. It never reconstructs them from
the final AST. Every branch and nested gate receives a unique parser-issued
`gate_id`; branch/item coordinates are recorded before the branch body is
parsed. If the parser cannot issue an exact path, the rich product stops at
`NoSafeSlice`.

## Postpass product contract

The product is non-Clone and has one consuming API per postpass:

```text
OpenParserPostpassProductV1::prune_build_gates(self, config)
  -> Result<Self, ParserPostpassRejectV1>

OpenParserPostpassProductV1::lower_delegates(self)
  -> Result<Self, ParserPostpassRejectV1>

OpenParserPostpassProductV1::finalize(self)
  -> Result<ParsedProgramWithSourceV1, ParserPostpassRejectV1>
```

Each operation transforms AST and source session atomically. A failure drops
the unpublished product; there is no retry using the same partially mutated
AST or source ledger.

### Build-gate prune

Prune evaluates a gate once, selects one branch, and drops the unselected
branch's AST and source records together. It preserves the original source
path and selected-gate path; it never assigns a new source identity from the
post-prune vector position. The selected branch must have complete source
coverage before the product is returned.

Unsupported top-level gate forms are `Declined` only when the source is fully
observable and outside the opened cohort. Missing path/branch evidence is
`Unresolved`; a foreign or duplicate path is `Rejected`; an issuer or typed
transport that does not yet exist is development-state `NoSafeSlice`.

### Source-aware delegate lowering

Delegate lowering consumes and returns the same product. It may use a private
descriptive target index derived from the product's current AST/session, but
that index is not a source authority. For every generated method it must
atomically commit:

```text
generated inventory placement
+ GeneratedDelegateSourceRelation
   (host Box path, delegate member/expose site, target Box/method path)
```

The generated declaration and relation are issued by the same transaction.
The existing AST-only delegate pass is a descriptive compatibility adapter
only; it must not feed a final resolver-grade seal. Until the generated
relation is implemented, the rich path rejects generated delegate suffixes
instead of accepting an incomplete seal.

### AST-only projections

Every AST-only public parser API calls the canonical rich path exactly once:

```text
rich product
  -> into_ast_projection()
  -> drop source seal/session
```

The projection may keep cloneable diagnostic metadata, but it must not rescan
source, call a second parser, or promote `ParserMetadata` to source authority.

## Finalizer completeness

The finalizer issues `ParserBoxSourceSealV1` exactly once and requires:

```text
all selected ordinary Box paths are present exactly once
all explicit/property/delegate source relations are complete
all relation brands match the parser invocation
final inventories match their source transactions
no unsupported BuildGate/Box kind remains in the opened cohort
```

The final seal is not issued for a partial generated suffix. The final
`ParsedProgramWithSourceV1` is the only product that a future resolver may
borrow.

## Disposition matrix

```text
NoSafeSlice
  required parser issuer or typed postpass transport is not implemented

Rejected
  foreign brand/path, duplicate path, relation mismatch,
  incomplete final coverage after an otherwise observable pass

Unresolved
  source path, branch membership, declaration, or target relation is missing

Declined
  fully observed source is outside the opened R6-S3B cohort

Candidate
  complete same-brand postpass product passes final seal verification
```

`NoSafeSlice` is never silently converted into `Unresolved` or `Declined` by
adding a test constructor or a generated suffix shortcut.

## Ordered implementation slices

```text
R6-S3B-D0  this handoff, path, owner, and disposition decision

R6-S3B-A  ParserPostpassProductV1 and AST-only projection parity
           (ordinary direct Box cohort; no gate/delegate expansion yet)

R6-S3B-B  typed gate path/cursor and transactional prune/rebase
           (selected branch only; no post-prune ordinal reconstruction)

R6-S3B-C  source-aware delegate transaction and
           GeneratedDelegateSourceRelation

R6-S3B-D  final complete-coverage seal, retire the S3A generated-suffix
           adapter, and switch all AST-only APIs to the rich path
```

## R6-S3B-B design receipt — accepted; B1 implementation opened

The top-level build-gate boundary is a parser source-transport problem, not
an AST-only filtering helper. The current `statement_ordinal` site is
insufficient because multiple Boxes inside one gate can share the outer
ordinal. S3B-B introduces no semantic `Verified*` or physical `Prepared*`
product; it adds only parser-private structural coordinates and an atomic
source-session operation.

### Sole owners and types

```text
NyashParser
  owns invocation brand, top-level cursor, gate-id issuer, and predicate policy

SourceBuildGateIdV1
SourceBuildGateBranchV1 { Then | Else }
SourceBoxDeclarationPathV1
  = invocation brand + immutable root/gate/child segments

SourceBoxDeclarationSiteV1
  owns one SourceBoxDeclarationPathV1

BuildGateSelectionReceiptV1
  = one gate id + selected branch + original source path

ParserSourceSessionV1
  owns all unpublished Box source candidates and relations
  prepares/commits prune as one consume-return transaction

OpenParserPostpassProductV1
  owns AST + ParserSourceSessionV1 + diagnostic metadata across the boundary
```

The path grammar is private and parser-issued:

```text
RootStatement { ordinal }
BuildGate { gate_id, branch: Then|Else, child_ordinal }
... nested segments ...
```

Member-level `SourceBoxMethodSiteV1` paths remain nested under the Box path;
their member ordinal vocabulary is not reused for top-level gate children.
Paths retain original coordinates after pruning. A final AST vector position
is never a source identity.

### Atomic prune contract

```text
OpenParserPostpassProductV1::prune_build_gates(self, parser)
  -> Result<Self, ParserPostpassRejectV1>
```

The operation performs one AST/source walk and one predicate selection per
gate. It issues a typed `BuildGateSelectionReceiptV1` for each gate, prepares
the selected source candidates, and commits a complete replacement session
only after all of these invariants hold:

```text
one invocation brand
unique gate ids and paths
every parsed Box candidate has exactly one path
every selected ordinary Box has exactly one source candidate and vice versa
unselected branch AST and source candidates are dropped together
selected source paths retain original coordinates
no missing/duplicate/foreign path
```

Failure drops the unpublished product; it never partially mutates the AST or
reuses a source session. The parser remains the evaluator/cursor owner, but it
does not become a second source-session owner.

### Disposition and negative matrix

```text
NoSafeSlice
  typed path issuer/cursor or source-session transport is not implemented

Rejected
  foreign brand/path, duplicate gate/path, branch/path mismatch,
  cursor overflow, lost/duplicated candidate, final AST/source mismatch

Unresolved
  source path or branch membership is missing from parser evidence

Declined
  fully observed gate is outside the opened S3B-B cohort or its predicate is
  unsupported by the bounded source policy

Candidate
  same-brand product has one selection receipt per gate and exact coverage
```

Normal selection of a supported branch is not an error disposition.

### B implementation ladder

```text
R6-S3B-B0  design receipt and owner/type/negative matrix (closed)
R6-S3B-B1  parser-issued gate id, branch, child cursor, and path transport (closed)
R6-S3B-B2  consume-return ParserSourceSession prune/rebase transaction (design stop)
R6-S3B-B3  AST/source coverage tests, guard, finalizer receipt, and docs
```

B1/B2 must not add delegate relations, interface/static/record seals, Hako
parser parity, resolver publication, or Recipe/Builder/MIR integration. Every
future implementation cell updates its guard, owner README, active task, and
affected `docs/reference/**` document in the same commit.

Each slice is a BoxShape refactor/authority closure, not a new language
acceptance shape. Each implementation slice must update its owner README,
focused tests, guard/index, active task receipt, and the affected
`docs/reference/**` document in the same commit. Every touched source file
uses the 760-line split trigger and may not cross 800 lines.

## R6-S3B-B1 implementation receipt

Closed implementation scope for B1 is parser-private path transport only:

```text
NyashParser
  -> parser-issued SourceBuildGateIdV1
  -> SourceBuildGateBranchV1
  -> SourceBoxPathCursorV1 child ordinals
  -> SourceBoxDeclarationPathV1
  -> OpenBoxMethodSourceTransactionV1::open_with_path
```

The path is installed while each gate child is parsed and restored when that
child returns. Direct top-level Boxes use a root path; Boxes in the same gate
branch receive distinct child ordinals; nested gates append another segment.
The path is carried by `SourceBoxDeclarationSiteV1` and is never reconstructed
from the final AST or inventory order.

This receipt does not open `BuildGateSelectionReceiptV1`, source-session
prune/rebase, delegate relation transport, or final top-level gate sealing.

## R6-S3B-B2 design stop

B1 is closed. Before implementation, freeze the selection receipt's owner,
branch decision, exact path coverage, and the consume-return
`ParserSourceSession` transaction. Prune must consume AST and source session
together, preserve surviving structural paths, and reject missing, foreign,
or duplicate path evidence. Source identity must not be reconstructed from the
final AST, inventory ordinal, or a detached source vector.

## Nonclaims until R6-S3B-D closes

```text
resolver declaration or target issuance
typed CallableContract parser carriage
Recipe/CallSlot/Builder/MIR/provider/runtime connection
interface/static/record source seals
Hako parser parity
fallback/retry or AST rewrite
final source authority for generated delegate suffixes
```
