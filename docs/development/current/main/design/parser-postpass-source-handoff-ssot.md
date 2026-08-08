---
Status: accepted design; R6-S3B-A/B1/B2/B3-D0/B3-I0/C-D0/C-I0/D-D0/D-I0 closed; broad public AST postpass cutover D0 accepted, I0 parked
Date: 2026-08-08
Decision: one typed parser postpass product owns AST and source transport
Related:
  - docs/development/current/main/investigations/frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md
  - docs/development/current/main/investigations/callable-contract-and-instance-call-implementation-task-map-2026-08-08.md
  - docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-c-d0-design-task-2026-08-08.md
  - docs/development/current/main/investigations/frontend-parsed-box-source-aware-delegate-r6-s3b-d-d0-design-task-2026-08-09.md
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
       { final AST, complete explicit-source non-Clone seals }
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
delegate relations exist, a generated suffix remains descriptive compatibility
data outside the resolver-visible seal; it is not enough to issue a generated
source relation or to extend explicit-source coverage.

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
that index is not a source authority. For every parsed `expose` it must
eventually produce one relation row and atomically commit:

```text
generated inventory placement
+ GeneratedDelegateSourceRelation
   (host Box path, delegate member/expose ordinal, target Box/method path)
```

R6-S3B-C records this relation parser-privately and keeps it outside the
resolver-visible final seal. The relation is source evidence, not a semantic
`Verified*` product. The parser records the delegate member and expose ordinal
while reading source; it never reconstructs them from generated AST,
inventory ordinal, or a name `HashMap`. The first C cohort admits ordinary
top-level Rust Boxes with direct explicit target methods only. Generated
delegate chains, compatibility-only delegates, interface/static/record/Hako,
and provider-generated declarations remain outside the cohort. Missing typed
transport is `NoSafeSlice`, fully observed unsupported provenance is
`Declined`, unavailable source evidence is `Unresolved`, and foreign,
duplicate, malformed, or contradictory evidence is `Rejected`.

The C transaction preflights all hosts, exposes, exact source paths, target
relations, generated names, placement, and collisions before staging one
complete batch. AST mutation, inventory placement, and relation rows commit
once through `OpenParserPostpassProductV1`; any error discards the complete
unpublished product. Partial per-host commit, same-session retry, and
name-based target fallback are forbidden. R6-S3B-D alone extends complete
relation coverage into the final non-Clone `ParserBoxSourceSealV1` and retires
the bounded generated-suffix adapter.

The generated declaration and relation are issued by the same transaction when
the source-aware delegate row is opened. The existing AST-only delegate pass
is a descriptive compatibility adapter only; it must not feed a final
resolver-grade seal. Until the generated relation is implemented, the rich
path may preserve a valid generated suffix in its AST/descriptive
compatibility projection, but it keeps that suffix outside the
resolver-visible source seal. A malformed or provenance-invalid suffix
rejects the whole unpublished product.

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

R6-S3B-D  final complete-coverage seal and retire the bounded S3A
           generated-suffix adapter

PARSER-PUBLIC-AST-POSTPASS-CUTOVER-D0/I0
           total typed postpass envelope before switching broad AST-only APIs

```

## R6-S3B-D-D0 design receipt — accepted; D-I0 implementation closed

The final-seal boundary is fixed by:

```text
ParsedProgramWithSourceV1
  -> one finalizer-owned relation/placement coverage plan
  -> sole non-Clone ParserBoxSourceSealV1 issuer
```

The finalizer consumes the parser-private `GeneratedDelegateSourceRelationV1`
rows carried by C-I0. It verifies same-brand host/target paths, one relation
per `(host path, delegate member source site, expose ordinal)`, exact final
inventory placement receipts, explicit/property prefix coverage, and the
absence of orphan/non-delegate generated rows. It does not rescan AST names,
reconstruct source identity from inventory ordinals, or issue semantic
callable/target/Recipe products.

The accepted implementation cohort is ordinary top-level Rust Boxes with
direct explicit target methods only. Generated chains, compatibility-only,
interface/static/record/Hako/provider declarations, overloads, and resolver
connection remain closed. Any failure discards the unpublished parsed product;
there is no partial seal, retry, or fallback. The D-I0 task and guard are
closed for the bounded ordinary-Box rich path; the broad public AST postpass
cutover is the next design stop.

## R6-S3B-D-I0 implementation receipt — closed (2026-08-09)

The finalizer now consumes the C-I0 `GeneratedDelegateSourceRelationV1` rows
and validates them against the final inventory before issuing the sole
non-Clone `ParserBoxSourceSealV1`. Coverage is keyed by the exact host path,
delegate member source site, and expose ordinal; placement is checked through
the carried `generated_inventory_placement` receipt. Same-brand host/target
paths, generated provenance/selection, explicit/property prefix coverage, and
one-to-one generated suffix coverage are required. Duplicate, missing/orphan,
foreign, non-generated, or mismatched rows reject the unpublished product.
The final seal retains the generated relation rows. No AST/name/ordinal
reconstruction, resolver target, Recipe, Builder/MIR, provider/runtime,
fallback, or retry is opened by this slice. The three broad public AST-only
`NyashParser` paths remain compatibility nonclaims because the rich finalizer
is ordinary-Box-only; their total postpass envelope is the following D0 row.

## PARSER-PUBLIC-AST-POSTPASS-CUTOVER-D0 — active design stop

The broad AST API family is not allowed to call the ordinary-only rich path
and catch rejection. It must first converge on one total parser postpass
owner whose output distinguishes AST transport from resolver-visible source
authority:

```text
parse_program once
  -> PreparedBuildGateDecisionSetV1 (all structural gate paths)
  -> one prune/delegate coordinator
  -> one final cohort coverage pass
  -> CompletedParserPostpassV1

CompletedParserPostpassV1 {
  ast,
  metadata,
  explain,
  box_coverage,
}

box_coverage row
  = SourceSealedOrdinary { ParserBoxSourceSealV1 }
  | AstOnlyCompatibility { typed cohort receipt }
```

`SourceSealedOrdinary` is the only row that may cross into resolver source
authority. `AstOnlyCompatibility` is a successful AST/metadata/explain
projection for interface/static/record/mixed compatibility and is never an
empty seal, target, Recipe input, or fallback result. Final AST placement is
coverage only; parser-issued source paths/sites remain identity.

The coordinator is the sole postpass owner for `NyashParser::parse`, the
string/build-config family, metadata wrappers, and the explain route. It
consumes the parser invocation, source session, AST, and metadata once. No
wrapper reparses, rescans names, reconstructs identity from inventory ordinals,
or retries another postpass arm.

### Gate, fuel, metadata, and compatibility contract

The existing explain traversal covers all nested BuildGate nodes, whereas the
source gate ledger is top-level scoped. Therefore a private typed
`PreparedBuildGateDecisionSetV1` must evaluate all structural gate paths once;
the same decision set is consumed by AST prune, explain-report projection, and
top-level ordinary source-path rebase. The full decision set and the
top-level source path remain distinct types joined by one relation receipt.

Fuel is configured once at parser construction and remains observable through
all wrapper projections. Metadata is taken once by the postpass envelope;
AST-only projections discard it, metadata projections return it. Explain is
captured only when requested and is derived from the shared decision set, not a
second AST semantic walk.

Ordinary selected gate branches may receive `SourceSealedOrdinary` only when
the selected branch is proven to be the supported ordinary cohort. Fully
observed interface/static/record/mixed cohorts receive typed compatibility
rows. Foreign, duplicate, malformed, or missing source relations reject the
whole unpublished product; source/cohort alignment gaps are unresolved;
compatibility selection is not a decline and never triggers rich-then-legacy
fallback. `NoSafeSlice` remains a development state for an unimplemented
coordinator/issuer rather than a source disposition.

The first implementation series is deliberately finite:

```text
S0   private total envelope/cohort admission/caller census, no public switch
I0-A string/build-config wrapper family + fuel/AST/diagnostic parity
I0-B NyashParser::parse + metadata projection, no re-tokenization
I0-C full gate-decision-set + explain parity
FINAL old whole-root delegate caller-zero and compatibility quarantine
```

The ordinary rich D-I0 receipt remains closed and unchanged. This D0/I0 row
must update its parser README, language reference, focused parity/negative
tests, task map, CURRENT_STATE, and consolidated guard in each implementation
commit.

### S0 implementation receipt — closed (2026-08-09)

S0 adds the private `CompletedParserPostpassV1` envelope and the explicit
`OpenParserPostpassProductV1::finish_total_s0` coordinator. Structural cohort
classification is performed after the shared prune transaction. Ordinary
cohorts enter the existing source-aware delegate batch and sole finalizer;
interface/static/record/mixed/no-Box cohorts enter the isolated compatibility
delegate arm and produce AST-only coverage rows. No public parser caller was
switched, no explain report was synthesized, and no source seal was issued
for a compatibility row. The rich ordinary API now reuses the common
postpass-opening helper. Focused tests cover the two variants, mixed coverage,
constructor separation, and the explicit parked explain-capture diagnostic.

The next implementation is I0-A for the string/build-config wrapper family;
I0-B metadata/`NyashParser::parse` and I0-C full BuildGate decision-set/
explain parity remain ordered and unopened.

### I0-A design acceptance — implementation boundary (2026-08-09)

I0-A is accepted as one bounded production-edge replacement. Its source
authority is the existing parser invocation, its parser-issued source session,
and the AST produced by that same `parse_program` call. The S0 coordinator is
the sole postpass issuer for this edge:

```text
normalize/tokenize once
  -> parser construction (fuel/config once)
  -> parse_program once
  -> open_postpass_product once
  -> finish_total_s0(PostpassDemandV1::None)
  -> CompletedParserPostpassV1::into_ast once
```

The only production caller replaced is
`parse_from_string_with_fuel_and_build_config`; its three convenience wrappers
inherit the edge. Grammar-evidence APIs, metadata, `NyashParser::parse`, and
explain-report APIs are separate contracts and remain parked. The old
delegate-only route is not a fallback or a second production authority; it is
removed from the selected edge in the I0-A implementation commit.

The fail-fast boundary is the existing `ParseError` family. Tokenization,
parse, source-session/cohort, and explicit compatibility lowering errors are
returned directly; no catch, retry, reparse, name scan, ordinal reconstruction,
or rich-then-legacy fallback is allowed. Compatibility rows are successful AST
transport, not a decline.

I0-A acceptance is AST-shape and diagnostic-family parity for the matrix in
the task card, with fuel assigned exactly once. It does not claim metadata,
explain, resolver, Recipe, Builder, MIR, runtime, or remaining legacy-caller
retirement. The implementation may proceed only in `fast` mode after this
brief and the task card are committed.

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
R6-S3B-B2  parser gate-ledger transport, typed selection receipts, and consume-return ParserSourceSession prune/rebase (closed)
R6-S3B-B3-D0  finalizer AST/source exact-coverage alignment (closed)
R6-S3B-B3-I0  private finalizer coverage plan and suffix isolation (closed)
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

## R6-S3B-B2 implementation boundary

B1 is closed and the B2 design is accepted after an independent authority
audit. B2 implements only the
parser-issued gate ledger, one typed selection receipt per gate, and the
consume-return atomic prune/rebase transaction. The AST remains free of gate
IDs. The parser owns the gate source records and transports them inside
`ParserSourceSessionV1`; the source-aware AST walk validates those records and
does not become a second source-identity authority.

The parser ledger is a typed, parser-private record stream:

```text
PreparedBuildGateSourceRecordV1 {
  invocation_brand,
  gate_id,
  gate_path: SourceBuildGatePathV1,
  scope,
  predicate/source evidence,
}
```

The ledger order is normative: records are emitted in parser source-preorder
and are consumed in that same order by the original-AST validation cursor.
No sorting, `HashMap`, name order, post-prune reindexing, or final-AST ordinal
may establish the order. The cursor must reject a missing record, a duplicate
record, a leftover record at end of walk, or a record whose path/predicate does
not match the current original AST gate.

The distinct gate-path grammar is:

```text
SourceBuildGatePathV1 {
  invocation_brand,
  segments: [
    RootTopLevel { statement_ordinal },
    BranchChild { parent_gate_id, branch, child_ordinal },
    ...
  ]
}
```

The path points to the gate node itself. A top-level gate starts at
`RootTopLevel`; a nested gate uses a `BranchChild` segment whose
`parent_gate_id` names the enclosing gate. The nested gate's own `gate_id` is
separate. `SourceBoxDeclarationPathV1` may use analogous coordinates for Box
source sites, but the two path types are never interchangeable.

Only the bounded top-level build-gate item scope is opened in B2. The outer
top-level gate and nested gates parsed inside its selected Then/Else items
inherit `TopLevelItem` scope. A gate found
inside a Box method/body is not silently registered as a top-level source gate;
the same applies to constructor bodies, field initializers, and other member
scopes. Those scopes are closed/nonclaim and their records are not issued by
the B2 ledger. A Box declaration path is never reused as a general
method/body gate path.

The source-aware walk traverses the original AST once for validation and
pruning. It matches the parser ledger by structural cursor, evaluates each
opened gate once, and issues a private/non-forgeable
`BuildGateSelectionReceiptV1` containing the same invocation brand, parser gate
ID, path, and selected branch. Empty and nested gates still require a ledger
record and exactly one receipt. The receipt owner is the postpass prune
transaction, not the AST node and not a detached vector.

`ParserSourceSessionV1::prepare_prune(&self, plan)` is a no-mutation validation
step. It validates same-brand identity, unique
IDs/paths, exact gate-record/AST coverage, and candidate path membership. Its
validated `commit_prune(self, prepared)` consume-return step filters source
seals by the selected branch while
preserving surviving structural paths. Any AST/source mismatch, missing,
foreign, duplicate, or lost seal consumes and rejects the whole unpublished
product; the old session is never reused.

`SourceBuildGatePathV1` is a distinct parser-private node-path type. It must
not reuse `SourceBoxDeclarationPathV1` as if a gate were a Box, even when both
paths share structural segments. The AST remains free of gate IDs; the parser
ledger is the sole gate identity authority.

### B2 implementation acceptance (2026-08-08)

```text
parser gate records are issued during parse and moved into the postpass product
AST contains no gate ID or source identity reconstruction
one selection receipt per opened `TopLevelItem` gate exactly (empty and nested
gates included); declined/unresolved/rejected products issue no receipt
direct, sibling, nested, and empty-gate path coverage is exact
selected-branch Box source seals survive with original paths
unselected-branch seals are dropped atomically
method/body gates are outside the opened top-level cohort
foreign/duplicate/missing/leftover/order/cursor-overflow evidence fails fast
```

### B2 nonclaims

```text
final complete ParserBoxSourceSealV1
generated delegate relation
method/body gate source authority
Hako parser parity
resolver/CallableContract/Recipe/Builder/MIR/provider/runtime integration
finalizer expansion beyond the bounded rich product
```

The B2 implementation receipt is closed by the parser source-session tests and
the dedicated guard
`tools/checks/frontend_parsed_box_source_seal_r6_s3b_b2_guard.sh`. The focused
Rust suites cover source-preordered ledger transport, method/body scope
closure, direct/sibling/nested/empty gate pruning, selected-branch seal
preservation, and consume-return session validation. The B2 guard also checks
the distinct gate-path grammar, one receipt per opened gate, exact
end-of-stream coverage, and the below-800-line source boundary. B3-D0 and
B3-I0 are closed; no new resolver implementation is open from this receipt.

## R6-S3B-B3-D0 design stop — finalizer alignment

```text
Decision:
  close the finalizer's exact AST↔source-seal coverage contract before opening
  any delegate relation implementation; do not broaden the source cohort here.

Source authority + canonical issuer:
  ParserSourceSessionV1 owns the unpublished source transactions, and the
  existing finalizer is the sole issuer of ParserBoxSourceSealV1 after prune
  and delegate postpass. B3 adds no new semantic receipt or second issuer.

Non-authority:
  final-AST/inventory ordinals, generated delegate suffix provenance alone,
  delegate-lowering AST mutation, and AST-only compatibility projections.

Fail-fast boundary:
  every final ordinary-Box explicit/property row and source relation must have
  exact same-brand coverage. A valid generated delegate suffix may remain in
  the final AST/descriptive compatibility projection, but it is outside the
  resolver-visible source seal until GeneratedDelegateSourceRelation exists.
  A malformed or provenance-invalid generated suffix rejects/no-seal; a valid
  suffix is never treated as an explicit source relation.

Smallest next slice:
  a bounded finalizer alignment implementation/guard that proves explicit and
  property rows seal through a private source-path coverage plan, keeps the
  generated-delegate placement canary outside the final source seal, and
  rejects malformed/foreign/missing/duplicate coverage. The plan must match
  parser source paths to final AST Boxes one-to-one; no positional zip,
  inventory/name order, HashMap order, or postpass ordinal is identity. No new
  public semantic receipt is introduced; `FinalizerCoveragePlanV1` is private
  implementation state only.

Non-claims:
  no GeneratedDelegateSourceRelation, delegate transaction, interface/static/
  record cohort, Hako parity, resolver, Recipe, Builder, MIR, provider, or
runtime integration, source rescan, fallback, or AST rewrite.
```

R6-S3B-B3-I0 is now closed. Its implementation adds only the private
`FinalizerCoveragePlanV1` and parser-private final Box source paths. The plan
matches prepared source paths to final AST ordinary Boxes one-to-one, rejects
count/duplicate/foreign/missing coverage, and removes final-AST positional
order as an identity source. The valid generated delegate placement canary
remains in the descriptive AST inventory but outside the resolver-visible
source seal. Focused parser tests, the B3 guard, this reference, and the
parser-module owner comments landed in the same slice.

R6-S3B-C remains the next design boundary for a source-aware generated
delegate relation; it is not opened by B3-I0.

The current bounded S3A test accepts a generated delegate suffix only as a
temporary placement canary. That acceptance is not a resolver-grade source
claim: B3 keeps it explicitly outside the final source seal, while malformed
or provenance-invalid suffixes still reject. R6-S3B-C owns the later
source-aware delegate transaction and relation; B3-I0 does not implement it
indirectly.

## R6-S3B-C-D0 design receipt — accepted; C implementation remains closed

The source-aware delegate boundary is now fixed by
`frontend-parsed-box-source-aware-delegate-r6-s3b-c-d0-design-task-2026-08-08.md`.
The C row has one parser-private `GeneratedDelegateSourceRelationV1` per
`expose`, issued from parser-time source transport and committed together with
generated AST rows and inventory placement. A private target index may use
names for lookup, but exact parser-issued Box paths, same-brand source
relations, and existing target method relations are the authority. Generated
delegate chains are outside the bounded cohort.

The transaction preflights every host/expose/target/collision before one
consume-return commit. Any failure discards the whole unpublished
`OpenParserPostpassProductV1`; no partial host commit, same-session retry, or
name-based fallback is permitted. C does not widen `ParserBoxSourceSealV1`.
Only R6-S3B-D may consume complete relation coverage, issue the final
non-Clone seal, and retire the bounded generated-delegate suffix adapter.

## R6-S3B-C-S0 implementation receipt — closed (2026-08-09)

The first C implementation slice is intentionally narrower than the C-D0
transaction design. Parsing an explicit delegate now issues exactly one
parser-private `DelegateSourceDeclarationV1` per `expose` into the open source
transaction. The row carries the parser-issued host member source site, expose
ordinal, delegate field, source method, and exposed method names. A selected
member-gate merge consumes and rebases those rows by prepending the selected
gate path while preserving the branch member ordinal.

The closed S0 contract is one row per expose; no expose is silently folded or
reconstructed from the generated inventory.

`PreparedBoxSourceSealV1` transports the rows across the parser postpass. The
final `ParserBoxSourceSealV1` deliberately drops them: generated delegate rows
are still outside the resolver-visible source seal until C/D issues complete
relation coverage. Compatibility-only delegates reject before source authority
is acquired.

This receipt proves parser-time source transport, selected-gate path rebasing,
prepared-payload carriage, and final-seal isolation only. It does not open
target lookup, generated placement, all-host/expose preflight, resolver target
issuance, Recipe/CallSlot, or a second source authority. The focused tests and
`frontend_parsed_box_source_seal_r6_s3b_c_s0_guard.sh` landed with the
implementation; source authority and source-seal modules remain below 800
lines.

## R6-S3B-C-S1-D0 design receipt — accepted; implementation closed

C-S1 is a private borrowed target-index boundary. A
`DelegateTargetIndexV1<'product>` borrows the unpublished
`OpenParserPostpassProductV1` source session and AST. It is built from the same
parser invocation's prepared source seals and exact
`SourceBoxDeclarationPathV1` values; it does not issue a resolver target or
extend the final source seal. Because paths deliberately have no `Hash`/`Ord`
identity, a private `Vec<TargetBoxEntryV1>` plus a private name-to-candidate
lookup view is sufficient. The index is never stored, published, or passed to
the resolver.

The index may use a host delegate field's declared type name and an expose's
source method name as query selectors, but a successful result must carry the
exact same-brand target Box path and exactly one existing explicit
`MethodSourceRelationV1` row. Names, AST order, inventory ordinals, generated
suffix order, and map order never become identity. A reusable borrowed
`TargetMethodRef<'product>` may serve multiple exposes and is not one-shot.

The bounded selector requires one target path. Missing source path/alignment or
missing field/type or incomplete source inventory is `Unresolved`. If a complete
same-brand index has no admitted ordinary target, the target is `Declined`;
multiple Box candidates are `Rejected` as ambiguous identity. A generated-only,
delegate-chain, compatibility-only, overload, or otherwise outside-cohort
target is `Declined` when fully observed. A missing explicit method for an
otherwise present target, foreign, duplicate, contradictory, or brand/path-
mismatched evidence is `Rejected`. A canonical issuer gap is development-state
`NoSafeSlice`, not a source disposition.

The only reusable result is a parser-private borrowed target reference carrying
the exact target path and explicit source relation. It contains no generated
placement, AST node, resolver identity, Recipe key, ValueId, provider handle,
or runtime route. C-S1 does not mutate AST/inventory/seal state. Complete
all-host/expose preflight and atomic generated batch commit are the accepted
C-I0 boundary; final resolver-visible relation coverage remains R6-S3B-D.

## R6-S3B-C-S1 implementation receipt — closed (2026-08-09)

The parser-private `DelegateTargetIndexV1` is implemented in
`src/parser/delegate_target_index.rs`. It borrows the unpublished postpass
product and validates that ordinary Box AST entries, prepared source seals,
and `SourceBoxDeclarationPathV1` values have complete same-brand alignment.
Target field/type and expose source-method names are selectors only. A
successful query returns a reusable `TargetMethodRefV1` carrying the exact
target path and one existing explicit `MethodSourceRelationV1`; missing field
evidence is `Unresolved`, missing method is `Rejected`, and duplicate target
names are rejected when the index is issued. No AST, inventory, prepared seal,
final seal, generated placement, resolver target, Recipe/CallSlot, or runtime
state is mutated.

The four focused tests landed in the same slice and prove positive reuse,
missing-field unresolved, missing-method no-fallback rejection, and duplicate
target rejection. All touched Rust files remain below 800 lines. The
separately tracked C-I0 implementation is now closed at the current clean
stop; the next R6-S3B-D final-seal design remains unopened.

The accepted C-I0 design and closed implementation card are:

```text
frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-d0-design-task-2026-08-09.md
frontend-parsed-box-source-aware-delegate-r6-s3b-c-i0-implementation-task-2026-08-09.md
```

## R6-S3B-C-I0-D0 design receipt — accepted; implementation closed

C-I0 is now design-closed after an independent authority audit. The row does
not extend `ParserBoxSourceSealV1`; it defines the parser-private staged batch
that the later implementation must produce:

```text
PreparedDelegatePostpassBatchV1
  staged per-host generated AST batches
  expected BoxMethodInventoryPlacementReceiptV1 rows
  owned GeneratedDelegateSourceRelationV1 rows
  exact same-brand host/target paths and target source references
```

The batch is issued by a consume-only prepare step on
`OpenParserPostpassProductV1` and committed by one consume-return step. The
prepare step borrows the C-S1 target index only while it is open; the
staged batch owns all paths, source references, generated names, and placement
receipts before commit. A borrowed descriptive target method
signature/declaration view is allowed solely to construct the forwarding AST;
it is not source identity, a semantic contract, or a resolver target.

The complete preflight covers every ordinary host Box and every parser-issued
delegate expose exactly once. It verifies host/source-path alignment, target
field/type, explicit target method relation, generated name and collision
policy, generated provenance, staged inventory placement, and AST/source-row
one-to-one coverage. Placement is computed against a staging inventory and
actual commit receipts must equal the expected receipts. A zero-delegate
ordinary program is an exact no-op.

Generated relation rows are carried by `ParserSourceSessionV1`/the prepared
source payload through prune and finalization so R6-S3B-D can verify complete
coverage without AST re-scan. C-I0 keeps them outside the final non-Clone
`ParserBoxSourceSealV1`; only D can issue resolver-visible relation/seal
coverage. Any failure consumes and drops the whole unpublished product: no
partial host commit, rollback repair, same-session retry, or name fallback.

Disposition is fixed as follows:

```text
NoSafeSlice = missing staged issuer/typed transaction (development state)
Rejected    = foreign/duplicate/mismatch/collision/orphan/placement failure
Unresolved  = incomplete source path/field/type/signature evidence
Declined    = fully observed outside the bounded C cohort
Candidate   = all rows exact and the complete staged batch is ready
```

The dedicated implementation card, guard, parser README, language reference
receipt, focused tests, and current task pointers landed together with the
bounded C-I0 implementation. `ParserBoxSourceSealV1` remains unchanged; the
relation rows are available only from `ParsedProgramWithSourceV1` for the later
R6-S3B-D finalizer.

## R6-S3B-C-I0 implementation receipt — closed (2026-08-09)

`src/parser/delegate_batch.rs` now owns one
`PreparedDelegatePostpassBatchV1`. It borrows the C-S1 target index while
preparing every ordinary host/expose row, obtains a descriptive target method
declaration only for forwarding AST construction, stages each inventory
append, and records one placement receipt plus one owned
`GeneratedDelegateSourceRelationV1` per expose. The original postpass product
is not mutated until all hosts pass preflight; a later-host failure therefore
drops the unpublished batch without partial AST mutation.

The consume-return commit applies the cloned AST and attaches relation rows to
`ParserSourceSessionV1`. Finalization carries those rows into
`ParsedProgramWithSourceV1` without an AST/name rescan, while the final
non-Clone source seal still excludes generated delegate relations. Focused
tests cover exact zero-delegate no-op, later-host failure, generated-name
collision, duplicate source rows, placement-receipt mismatch, and persisted
relation output. No resolver, Recipe/CallSlot, Builder/MIR, provider/runtime,
fallback, or production authority is opened.

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
