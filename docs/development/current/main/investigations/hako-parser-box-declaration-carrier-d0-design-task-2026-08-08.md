---
Status: H1 implementation active — disconnected parser substrate only
Date: 2026-08-08
Decision: reuse the parser-private source-carrier lifecycle; separate exact
source sites from selected inventory placement; issue one non-Clone parser
source seal; no semantic inventory issuer in H1
Parent: `frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
---

# HAKO-PARSER-BOX-DECLARATION-CARRIER-D0

## Problem

The `.hako` compiler has no canonical ordinary Box declaration product today.
Existing surfaces are not promotable:

```text
FuncScannerBox / StageBRuneBox:
  compatibility rescans and source rewriting

tools/hako_parser:
  tool-only line scanner and name-sorted output

ParserDeclarationBox:
  narrow declaration evidence string; semantic publication disabled

source_carrier_v1:
  typed parser-private substrate; declaration vocabulary/connection absent
```

Creating `HakoBoxMethodInventory.scan(source)` would introduce a second parser
authority and is rejected.

## Required architecture

```text
ParserProgramBox
  -> ParserDeclarationBox typed product entry
  -> one ordinary Box parser branch
  -> declaration children built once into an unpublished transaction
  -> ordered AST inventory + non-Clone parser-owned source seal
  -> ProgramJSON/string only as one-way compatibility projection
```

The D0 must decide the typed declaration carrier vocabulary, source refs,
builder/sealer owner, build-gate selection transaction, duplicate/site errors,
and compatibility projection. It must preserve the source-carrier rule that
semantic nodes are not raw MapBox or JSON.

## D0 decision

The existing `source_carrier_v1` remains the only parser-private construction
authority. A second declaration scanner, a second parser tree, or a resolver
semantic product is not introduced. Its `Open`/`Poisoned`/`Sealed` lifecycle,
one-shot sealer, and typed outcome discipline are reused, but the current
index-only refs do not provide declaration identity: declaration refs must
carry a parser-carrier brand.

`BoxMethodInventoryV1` remains a Clone-capable descriptive AST carrier. Its
`ExplicitSource` provenance and selected entry order are data, not resolver
authority. The non-Clone parser seal is the sole capability that proves that
one complete Box was parsed, selected, duplicate-checked, and related to exact
as-written method sites.

```text
ParserProgramBox
  -> ParserDeclarationBox ordinary-Box branch
  -> ordered unpublished member transaction
  -> source-carrier sealer
  -> (BoxMethodInventoryV1, SealedParserBoxDeclarationV1)
  -> one-way ProgramJSON compatibility projection
```

The first typed product is a parser product, not a `Verified*` semantic
receipt. It may preserve source facts and exact source references, but it does
not issue nominal Box identity, Home ABI, effect, callable contract, resolver
target, Recipe key, or MIR facts. Those remain later resolver owners.

The branch is the sole ordinary-Box parser authority. `FuncScannerBox`,
`StageBRuneBox`, `tools/hako_parser`, and the current declaration-evidence
string remain compatibility/diagnostic routes and cannot be imported by the
canonical branch.

## Body handling is one-pass and non-semantic

The first declaration product seals the Box header/method inventory together
with the body result already produced by the same parser pass. It must not
save a body slice for a later `FuncScannerBox`/text reparse.

```text
ordinary Box branch
  -> parse header/member once
  -> parse each body once
  -> ParserNodeProductV1(Typed | CompatOnly | ParseError)
  -> seal header/inventory + body disposition
```

`Typed` body nodes are parser evidence only. `CompatOnly` fragments are
owned exclusively by the existing ProgramV0 compatibility projector and
cannot become declaration, callable-contract, or body-conformance authority.
If the ordinary branch cannot obtain a one-pass `ParserNodeProductV1` for a
body, H2 is `NoSafeSlice`; it must not reintroduce a body scanner or a saved
source range.

## Typed vocabulary and ownership

H1 reuses the existing source-carrier lifecycle, draft/sealer split, and
outcome domains. It adds declaration-specific branded refs/records under the
same source-carrier authority; it does not pretend that the current
index-only refs provide arena ownership. Declaration vocabulary is added only
after the H1 guard confirms that no existing module already owns the same
meaning.

```text
ParserCarrierBrandV1
  invocation-local parser-carrier identity; never a filename or semantic id

ParserSourceUnitRefV1
  carrier brand + source-unit identity; issued by the parser invocation

ParserBoxDeclarationSiteV1
  source-unit ref + top-level statement ordinal; the program cursor is the
  only issuer of the ordinal

ParserBoxMemberSiteV1
  Box declaration site + exact as-written member path; Direct carries the
  Box member ordinal, while SelectedBuildGate carries the outer-to-inner gate
  path and branch-local member ordinal

ParserBoxMethodSourceSiteV1
  exact as-written method member site; present only for explicit methods

ParserBoxInventoryOrdinalV1
  placement in the selected/generated inventory; assigned at the atomic
  commit and never used as declaration identity

ParserSelectedGateStepV1
  outer-to-inner gate path + gate site + branch-local member ordinal; used
  for selected-state rebase and never treated as source identity

ParserBoxMethodDraftV1
  method source site, method name, instance/static kind, arity/result tokens,
  source refs, and typed parser syntax rows

ParserGeneratedMethodDraftV1
  generated role plus the originating member site; it has no method source
  site and cannot be promoted to an explicit declaration

ParserBoxDeclarationDraftV1
  Box site, Box name/kind, ordered method drafts, and duplicate ledger

SealedParserBoxDeclarationV1
  non-Clone parser authority; sealed Box identity, complete explicit-method
  source rows, generated-origin rows, and exact source-site -> inventory-slot
  relations
```

`Span` is diagnostic metadata, not semantic identity. Names and arity are
lookup attributes, not declaration identity. The source identity is the
structural coordinate `(source_unit, box_statement_ordinal,
member_ordinal)`.

The parser builder owns drafts and duplicate preflight. Every ref/site/draft
must carry the same `ParserCarrierBrandV1`; foreign-brand and invalid-site
inputs fail before mutation. The sealer owns the single commit and immutable
publication. `ParserBoxDeclarationProductV1` is the only product that crosses
the parser branch boundary. No public partial receipt constructor is allowed.

Structural ordinals are not caller-supplied constructor arguments. The
`ParserProgramBox` cursor advances the program-statement ordinal, the ordinary
Box branch advances the exact member path, and the declaration sealer alone
issues selected inventory ordinals. Generated property/delegate rows may
consume inventory slots, but never change an explicit method's source
identity. Diagnostic start/end offsets may be passed by the recognizing
parser branch, but offsets never participate in identity.

## Rust parser source-site correction

The current Rust parser still keeps one source relation in two places:

```text
BoxMethodEntryV1.site.selected_method_ordinal
+ BoxMemberState.method_source_member_ordinals
```

The first is an all-row inventory position; generated property/delegate rows
consume it. The second is a parallel `Vec<u32>` reconstructed from length
deltas and zipped back during gate merge. Length checks do not prove the
correct pairing. Before Rust rows can enter resolver semantics, one bounded
BoxShape correction must establish:

```text
explicit method draft owns exact SourceBoxMethodSiteV1
generated draft owns source-member origin + generated role only
atomic parser seal assigns independent BoxMethodInventoryOrdinalV1
source seal relates explicit source site -> selected inventory entry
```

The selected-inventory JSON v2 key may remain a descriptive compatibility
spelling; JSON cannot reconstruct or issue the parser seal. R4/R5 behavior is
not reopened, and Builder consumers continue to receive only the inventory.

Retirement is complete only when
`method_source_member_ordinals`, `record_new_methods_since`, the merge API's
parallel ordinal slice, and raw delegate ordinal sidecars have zero callers.

## Atomic build-gate transaction

The selected build gate owns one declaration transaction per source unit. The
transaction has no published inventory until all selected Box members have
passed source/site/duplicate checks. A branch-local member ordinal is kept
while parsing; the selected build gate assigns the final member ordinal only
at commit, after gate-path and outer-collision checks are known.

```text
open source-unit transaction
  -> parse then/else into unpublished branch states
  -> preflight duplicate, site, order, brand, and rune placement
  -> compare selected-build surface signatures
  -> select one build gate
  -> prepend gate path/rebase selected ordinals
  -> seal one BoxDeclarationProductV1
  -> commit selected product to the program transaction
  -> project compatibility JSON only after commit
```

A rejected Box does not leave a partial ordered list for later consumers. A
compatibility projection is one-way from the sealed product; no consumer may
reconstruct source order or sites from JSON, a `HashMap`, or a sorted name
list. Duplicate diagnostics retain both first and duplicate sites. Duplicate
failure consumes neither an ordinal nor a partial entry; rollback repair is
not the recovery mechanism. The selected build gate is the only route allowed
to commit the product; direct/deferred/compatibility branches cannot publish a
second copy. If the Hako side has no canonical build-config evaluator, H4 is
`NoSafeSlice` and does not invent one in the parser.

Delegate helpers currently expand in a whole-program typed postpass. H3 must
choose exactly one atomic boundary: either final Box sealing occurs after that
postpass, or the postpass consumes and returns the branded parser product while
preserving the seal relation. A detached AST inventory may not be mutated
after it has been claimed as sealed. Generated delegate helpers still receive
no method source site.

## Failure boundary

These are parser/source errors, not semantic Candidate/Declined outcomes:

```text
duplicate method in one Box                 -> SourceParseError
duplicate CallableContract rune             -> SourceParseError
invalid rune/member placement               -> SourceParseError
malformed member/source reference           -> SourceParseError
foreign draft/site or broken ref invariant  -> InternalCarrierContractViolation
```

`NoSafeSlice` remains a development state while an issuer or parser branch is
missing. It is not a source disposition. H1 only proves that typed drafts can
be built/sealed and that malformed/duplicate input fails before publication.
`Candidate/Declined/Unresolved/Rejected` belongs to the later resolver
declaration/profile issuer and must not be invented in the parser carrier.

## CallableContract carriage boundary

H1/H2 may carry generic rune syntax as parser data, but it is not resolver
input. H6 replaces raw name/argument interpretation with a typed
`CallableContractSyntaxV1::Query` source row carrying the exact method/rune
site and selected-source provenance.
The Hako parser does not issue `CallableContract(query)` semantics, Home
demand, Pure/effect, receiver ABI, or body conformance. H6 may consume the
sealed parser product only after Rust/Hako normalized inventory parity is a
test-only fact. Resolver code never re-parses `"CallableContract"` or
`"query"` strings.

H1 is disconnected by design: it proves branded refs/sites, exact source
member paths, separate inventory ordinals, ordered member drafts,
duplicate-without-mutation, one-Box seal, foreign-brand and invalid-site
negatives, and double-finish rejection. It does not wire
`ParserProgramBox`, build-gate selection, ProgramJSON, rune carriage, or
semantic publication. The H1 card records both the H2 connection condition
and the removal condition so this substrate cannot become a permanent
parallel authority.

## Candidate physical layout

```text
lang/src/compiler/parser/source_carrier_v1/
  source_declaration_refs_v1.hako
  source_declaration_records_v1.hako
  source_declaration_builder_v1.hako
  source_declaration_sealer_v1.hako
  (existing README/outcome/lifecycle remain the authority)

lang/src/compiler/parser/decl/
  parser_box_declaration_product_box.hako
  parser_box_method_inventory_box.hako (only after H3 issuer is open)
```

Names remain provisional until H1 code/owner census confirms they do not
duplicate the existing source-carrier vocabulary. A new top-level
`declaration_carrier_v1` authority is not allowed.

## Mandatory stop lines

```text
no FuncScanner/StageBRune/tools parser import
no source/body slice rescan
no ProgramJSON/MapBox semantic carrier
no name sorting as source order
no parser_box.hako growth (currently 787 lines)
no semantic publication before typed product seal
no CallableContract semantic issuance before Rust/.hako inventory parity
no resolver target/Recipe/MIR fact from parser product
no second declaration builder/sealer outside source_carrier_v1
```

If `ParserBox` facade wiring is unavoidable, split it below the hard limit
before adding a call. Every new `.hako` file stays below 800 lines.

## Planned rows (implementation order)

```text
H0 D0 owner/vocabulary/API decision
H1 disconnected typed transaction/source-site/placement substrate + guard
Rust R6 parser-owned source-site/seal cutover + sidecar retirement
H2 canonical ordinary Box parser member-draft branch
H3 atomic inventory + parser source-seal issuer
H4 selected build-gate transaction/rebase
H5 test-only normalized Rust/.hako parity
H6 typed CallableContract(query) syntax carriage + reference update
```

The cross-language executable order is owned by
`callable-contract-and-instance-call-implementation-task-map-2026-08-08.md`.
In particular, H2 does not open until the R6 source-site/seal boundary has
replaced the Rust sidecar model.

## H1 implementation handoff

H1 is now open as a BoxShape-only implementation slice. It may add only the
branded parser refs/sites, ordered drafts, duplicate-before-mutation checks,
one-Box non-Clone seal substrate, focused negative tests, and the existing
`hako_parser_source_carrier_p0_guard.sh` coverage. It must not connect the
ordinary parser, build-gate, delegate postpass, JSON projection, typed rune
carriage, resolver, Recipe, Builder, or publication.

Acceptance:

```text
all touched Hako source files < 800 lines
source-carrier guard green
positive one-Box seal
foreign brand/site rejection
invalid path rejection
duplicate rejection leaves no partial publication
double finish and post-seal mutation reject
worktree clean at closeout
implementation + focused tests + README/reference receipt in one commit
```

The current source-carrier guard is updated in H1 before any parser connection.
H1 is intentionally disconnected and must close in one small implementation
slice; it cannot become a long-lived caller-zero asset. H2/H3 are the first
connected rows, and H4 is the only selected build-gate publication point.

```text
H0  D0 owner/vocabulary/API decision                         accepted here
H1  disconnected typed transaction/source-site substrate     next
H2  canonical ordinary Box member-draft branch               after H1
H3  atomic inventory + parser source seal                    after H2
H4  selected build-gate transaction/rebase                   after H3
H5  test-only normalized Rust/.hako parity                   after H4
H6  typed CallableContract(query) carriage + reference       after H5
```

The Rust-side `FRONTEND-PARSED-BOX-SOURCE-SEAL-R6` BoxShape series may land
after H1 but must close before H5 or any resolver declaration row. It removes
the parallel source-ordinal sidecar and gives Rust/Hako parity the same source
site versus inventory-placement model; it is not part of the H1 commit.

Each implementation row must close with focused tests, the owning README,
the affected reference receipt, the active card/task pointer, and a check
that all touched source files remain below 800 lines. H1 updates only the
parser-private carrier README/guard; H2-H6 update the exact landed
language/parser reference in the same commit. No future contract is written
as landed behavior.

## Parity evidence

The normalized parity report is test-only and ordered:

```text
inventory_v1
box=<box-statement-ordinal>:<name>:<instance|static>
method=<inventory-ordinal>:<source-site-or-generated-origin>:<name>:<arity>:<result-token>:<provenance>:<typed-runes>
```

Rust and `.hako` consume the same source and selected build profile. The gate
compares ordered rows and duplicate disposition. Spans/brands are not compared
as strings; their structural presence is asserted separately.
