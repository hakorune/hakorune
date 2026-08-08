---
Status: R6-D0/R6-S0/R6-S1/R6-S2a/R6-S2/R6-S3A/R6-S3B-D0/R6-S3B-A/R6-S3B-B0/R6-S3B-B1/R6-S3B-B2 closed — R6-S3B-B3-D0 design stop; B3-I0 not opened
Date: 2026-08-08
Decision: one AST-owned ordered inventory; selected-gate source remains explicit source
Parent: `language-typed-callable-profile-d0-design-task-2026-08-08.md`
Next: `callable-contract-and-instance-call-implementation-task-map-2026-08-08.md`
---

# FRONTEND-ORDERED-BOX-METHOD-INVENTORY-D0

## Decision

```text
BoxMethodInventoryV1
  = sole Box method authority
  = ordered entries + private derived name index

source iteration:
  lexical/selected source order only

legacy execution iteration:
  explicit iter_compat_name_order() projection only
```

`ASTNode::BoxDeclaration.methods: HashMap<String, ASTNode>` is retired. No
resolver, Builder, JSON codec, or compatibility scanner may reconstruct source
order, duplicates, provenance, or declaration identity from names.

## Canonical model

```rust
pub struct BoxMethodInventoryV1 {
    entries: Vec<BoxMethodEntryV1>,
    lookup: HashMap<Box<str>, usize>, // private derived index
}

pub struct BoxMethodEntryV1 {
    name: Box<str>,
    declaration: ASTNode,
    provenance: BoxMethodProvenanceV1,
    site: BoxMethodInventoryOrdinalV1,
    diagnostic_span: Span,
}
```

This is the landed R1-R5 descriptive inventory model. R6 does not replace its
ordered lookup role; it adds an independent exact source site and one
parser-owned non-Clone seal. `BoxMethodInventoryOrdinalV1` therefore remains
an inventory-placement record and is not promoted into resolver identity.

The exact provenance is:

```text
ExplicitSource {
  selection:
    Direct
    | SelectedBuildGate {
        path: [
          { gate_site, branch_member_ordinal },
          ...
        ]
      }
}

GeneratedProperty { exact property origin }
GeneratedDelegate { exact delegate/expose origin }
GeneratedMacroOrImport { exact generator origin }
CompatibilityOnly
```

`SelectedBuildGate` is not a generated provenance. A method written by the
user inside the selected branch remains `ExplicitSource`; its outer-to-inner
selection path is nested metadata. Only rows lent through the future
parser-owned explicit-source seal may back the first resolver
`CallableContract(query)` declaration. Raw `ExplicitSource` provenance is
descriptive and cannot authorize resolution.

Constructors remain a separate authority. Source properties are Box members
whose emitted helpers carry `GeneratedProperty`; they never borrow a source
method ordinal.

## Structural identity

R1-R5 own selected inventory placement only:

```text
BoxMethodInventoryOrdinalV1
  = position among selected/generated rows
  = descriptive AST placement
```

R6 introduces the independent resolver-grade source coordinate:

```text
source/catalog brand
+ program Box statement site
+ exact as-written method member path
  Direct(box_member_ordinal)
  | SelectedBuildGate(path, branch_member_ordinal)
```

The parser-owned seal proves complete parsing, selected Box membership,
duplicate freedom, and the exact relation from each explicit method source
site to one selected inventory entry. Generated property/delegate rows have a
generated origin and inventory ordinal but no explicit method source site.
`Span` is diagnostic only. Neither JSON nor `selected_method_ordinal` can
reconstruct or issue this capability.

## Public API and forbidden API

Allowed API:

```text
iter_selected_declaration_order()
get(name)
into_selected_declaration_order()
try_push(parser-issued entry)
try_merge_selected_gate(unpublished selected transaction, gate site)
try_from_compatibility_entries(entries, compatibility origin)
iter_compat_name_order()
```

Forbidden API:

```text
public lookup map
unordered iteration
insert / extend
From<HashMap> as source authority
caller-supplied source ordinal
caller-forged ExplicitSource provenance
Deref<Target = HashMap<...>>
```

Compatibility JSON v1 may create `CompatibilityOnly` rows through an
explicitly named compatibility decoder. This entry cannot become a resolver
source capability.

## Rust producer census

| Class | Current producer | Required cutover |
| --- | --- | --- |
| ExplicitSource/Direct | `src/parser/declarations/box_def/body.rs` | common parser-issued source entry; duplicate reject |
| ExplicitSource/Direct | `src/parser/declarations/box_def/interface.rs` | same entry API; exact method span |
| ExplicitSource/Direct | `src/parser/declarations/static_def/members.rs` | same entry API; exact method span |
| ExplicitSource/SelectedBuildGate | `box_def/body.rs` + `box_def/state.rs` | unpublished branch inventory; preflight then atomic merge/rebase |
| GeneratedProperty | `box_def/members/property_emit.rs` | exact generated origin; collision preflight |
| GeneratedDelegate | `crates/hakorune_frontend_parser/.../delegate_lowering.rs` | exact delegate origin; whole-batch preflight |
| GeneratedMacroOrImport | `src/macro/engine.rs` | generated origin; existing explicit rows remain authoritative |
| CompatibilityOnly | `src/macro/ast_json/roundtrip.rs` | ordered v2 codec; explicit v1 compatibility decoder |
| CompatibilityOnly | `src/macro/ast_json/joinir_compat/**` | one-way compatibility projection only |
| Separate authority | constructors in `box_def/body.rs` | never enter method inventory |
| Explicit non-method | record declaration | empty method inventory |

Current ordinary/interface/static insertion, property insertion, build-gate
`HashMap::extend`, and JSON `collect::<HashMap>` silently discard information.
All become fallible before mutation.

## Rust consumer boundary

Existing deterministic name-order consumers are compatibility behavior, not
source authority. They move to `iter_compat_name_order()`:

```text
builder/declaration_order
callable_declaration_catalog
instance_box_declaration_metadata
instance_box_method_batch
nonmain_static_box_method_batch
main_expansion
program_declaration_facts
raw_static_main_compat_batch
```

Exact-name compatibility consumers use only `get()`. Whole-map transports
move/clone the inventory. No compatibility consumer receives the private
lookup map or claims source order.

## Transaction rules

Every multi-row operation follows:

```text
validate all keys, declaration shapes, provenance, sites, and ordinal rebase
  -> produce complete unpublished rows/index
  -> one infallible commit
```

This applies to selected build-gate merge, generated property batches,
delegate batches, macro-derived rows, and JSON decoding. Partial insertion and
rollback repair are forbidden.

The existing order-insensitive build-gate branch-signature comparison may
remain a derived compatibility check; it is not source identity.

## R6-S2 external-review reconciliation (2026-08-08)

The external review and worker audit agree that the remaining R6 work is a
BoxShape cutover, not a new source-authority design. The following rules are
now normative for R6-S2:

```text
BoxMethodInventoryV1:
  Clone-capable selected/generated placement carrier

ParserBoxSourceSealV1:
  non-Clone parser-owned authority
  exact source site, brand, and relation coverage

resolver identity:
  source/catalog brand + Box source site + as-written member path
  never inventory ordinal, JSON, HashMap, or generated placement
```

`BoxMethodInventoryOrdinalV1` is only a selected/generated placement number.
It may be used to index a parser-private relation table while a transaction is
open, but it must never be promoted into a declaration identity or source
capability.

The transaction is the single owner of the unpublished inventory and its
source relations. It must cover all ordinary producer rows in this slice:

```text
explicit direct method
generated property / once / birth_once batch
selected member-gate branch inventory
```

Generated rows receive a generated origin and a placement receipt; they do
not receive an explicit method source site. An explicit method source site is
independent of the selected/generated inventory ordinal. The old
`method_source_member_ordinals` sidecar, length-delta reconstruction, and
parallel gate ordinal slice therefore have no place in the final owner.

The selected-gate merge must consume the branch transaction's relation map
and perform path rebasing atomically. A caller-supplied `&[u32]` parallel
slice is not an authority and is forbidden after the S2 cutover. Generated
batch APIs may return placement receipts, but may not expose private rows or
make the parser reconstruct source identity from a length delta.

R6-S2 deliberately does **not** issue the final parser seal. Delegate AST-only
postpass, raw `DelegateDecl` ordinal retirement, rich parse output, and final
seal issuance remain R6-S3. The seal is issued only from the final parse
product after build-gate prune/rebase and delegate lowering have completed.
No resolver, `CallableContract`, target, Recipe, JSON, or Builder semantic
connection is opened by R6-S2.

### R6-S2 implementation cells

1. **R6-S2b-AST receipt support — landed**
   - return generated placement receipts from a complete batch commit;
   - replace the parallel selected-gate ordinal merge with a transaction-owned
     relation lookup/rebase API;
   - do not reintroduce the removed ordinal-slice API or a second source
     authority.
2. **R6-S2 transaction owner cutover**
   - move `BoxMemberState` inventory, source relations, and member cursor into
     one parser transaction owner;
   - route ordinary direct/property/once/birth_once/member-gate producers
     through that owner with duplicate-first/commit-once behavior;
   - preserve generated provenance and exact explicit source sites.
3. **R6-S2 sidecar retirement — landed**
   - `method_source_member_ordinals`, `record_new_methods_since`,
     length-delta reconstruction, and the AST ordinal-slice merge are deleted;
   - focused parser/AST tests, owner READMEs, reference receipt, and the R6-S2
     guard are updated in the same slice.

Each cell is a behavior-preserving Refactor Series commit. No new resolver
issuer, rich parse output, delegate postpass, or semantic callable contract
may be smuggled into these cells. If a required transaction relation cannot
be issued from the current parser inputs, stop at `NoSafeSlice` and add the
missing source-ingress design instead of inferring it from names or ordinals.

### R6-S2 API freeze from worker audit

Before claiming the old parallel gate-merge API retired, the AST carrier must
stop accepting a parser-owned `&[u32]` source-ordinal slice. The preferred
bridge is a typed append/rebase boundary:

```text
source transaction:
  consume branch inventory + transaction-owned method-row relations
  prepend the selected-gate path to each branch row
  prepare one complete rebased append

AST inventory:
  validate names, declaration identity, duplicate collisions, and contiguous
  selected placement
  commit the prepared append atomically
```

The concrete API may be named
`PreparedBoxMethodInventoryAppendV1` with
`BoxMethodInventoryV1::prepare_append` / `commit_prepared_append`. The AST
crate must remain ignorant of parser brands and source-site authority. A
consumed unpublished branch inventory may expose selected entries only to the
source transaction; the transaction owns the source-site/path rebase. The
former `try_merge_selected_gate(selected, &[u32], gate_site)` API was not a
rename target; it was removed when S2 closed. The live API accepts only the
typed transaction-owned relation/rebase product.

The transaction-side minimum is:

```text
open_for_box(brand, active_statement)
branch()
current_member_site() / current_gate_site()
finish_member()
commit_explicit_at_current(...)
commit_generated_property_batch_at_current(...)
prepare_selected_gate_merge(...)
commit_selected_gate_merge(...)
finish()
```

`BoxMemberState` retains non-method metadata and delegates, but the sole
unpublished method/source owner becomes one `source_tx` field. No
`&mut BoxMethodInventoryV1` crosses parser producers after the cutover.
Top-level/interface/static parser paths and delegate AST-only postpass remain
outside this ordinary-Rust-`box` S2 cell; if their source ingress is missing,
the correct result is a later `NoSafeSlice`, not a guessed outer gate ordinal.

### R6-S2b AST receipt support — landed

The AST-side first cell is now implemented and tested. It adds:

```text
PreparedBoxMethodInventoryAppendV1
BoxMethodEntryV1::prepend_selected_gate(...)
BoxMethodInventoryV1::commit_prepared_append(...)
try_commit_generated_batch_with_placements(...)
```

The append product is validated before mutation and returns exact placement
receipts. The AST crate remains source-authority agnostic: parser brands,
source sites, and gate-path relation rebasing stay in the transaction owner.
The old AST selected-gate API accepting a caller-supplied ordinal slice was
removed in the R6-S2 transaction cutover; the AST now accepts only the typed
prepared append product.

### R6-S2 transaction cutover — landed in the current slice

The ordinary Rust `box` parser now routes direct methods, generated
property/once/birth_once batches, and selected member-gate branches through one
`OpenBoxMethodSourceTransactionV1` held by `BoxMemberState`. The transaction
owns the unpublished ordered inventory, exact explicit source relations,
generated-row relations, and member cursor. Branch merge consumes the typed
relation table, prepends selected-gate provenance, prepares one complete AST
append, and commits only after duplicate/name/placement validation.

The former ordinary-parser sidecars and reconstruction path are gone:

```text
method_source_member_ordinals      deleted
record_new_methods_since           deleted
length-delta source reconstruction  deleted
AST &[u32] selected-gate merge      deleted
```

Interface/static compatibility lanes may still use the explicit compatibility
sink because they are outside the bounded ordinary-`box` R6-S2 source-seal
claim. R6-S3A now opens only the bounded rich ordinary-Box output and final
non-Clone seal; source-aware delegate transport and top-level gate rebase
remain later nonclaims. The nested selected-else
fixture in the historical R2 parser pack remains a pre-existing red baseline;
it is not used as evidence for this behavior-neutral transaction cutover.

The current source transaction still does not authorize resolver semantics.
R6-S3A consumes its prepared payload only once, after the existing prune and
delegate postpass, to issue the bounded final rich parse product.

## Compatibility retirement

The compatibility surface closes only when all are true:

1. `BoxDeclaration.methods` is `BoxMethodInventoryV1` everywhere.
2. Every parser/generated producer uses typed provenance and fallible commit.
3. Builder name-sorted callers use only `iter_compat_name_order()`.
4. JSON v2 preserves ordered entries, provenance, and sites.
5. JSON v1 decoding creates only `CompatibilityOnly` rows.
6. Box-method ownership uses of `HashMap<String, ASTNode>` are zero.
7. Resolver imports of compat name sorting/HashMap reconstruction are zero.

Then `src/mir/builder/declaration_order.rs` method-map helper is deleted.

## `.hako` audit correction

There is no canonical `.hako` ordinary Box method parser/issuer today.

```text
FuncScannerBox / StageBRuneBox:
  mode-B compatibility rescan; never authority

tools/hako_parser:
  tool-only name-sorted scanner; never authority

ParserDeclarationBox:
  narrow declaration evidence; semantic_publication_allowed=false

source_carrier_v1:
  clean typed substrate, but declaration vocabulary and parser connection are 0
```

Therefore a standalone `scan(source)` inventory issuer is rejected. The
parked `HAKO-PARSER-BOX-DECLARATION-CARRIER-D0` must first define one typed
ordinary Box parser branch and declaration product. The inventory is emitted
while that branch parses each declaration once; no source slice, ProgramJSON,
MapBox, or function scanner is reread.

The current `parser_box.hako` is already 787 lines and receives no new
responsibility. Any necessary edit requires a prior facade split.

## R6-D0 correction: one parser output and one final source seal

The first R6 proposal was incomplete: it retired the Rust ordinal sidecar but
did not define how a non-Clone parser seal survives the prune and delegate
postpass boundary. The parser's public APIs currently return `ASTNode`, while
`lower_delegate_exposes` appends generated rows after the initial parse. A seal
issued before that postpass would describe an intermediate inventory and could
not be the resolver source authority.

R6 therefore remains a design stop until the following single path is fixed:

```text
source text
  -> one parser invocation/source-authority session
       (fresh invocation brand + top-level statement coordinates)
  -> parse AST + prepared Box source-seal payloads
  -> build-gate prune/rebase carries the payload without fabricating sites
  -> delegate postpass consumes the typed source-aware transaction and returns
       the updated unpublished inventory/payload
  -> finalizer checks the final AST inventory and exact source relations
  -> ParserBoxSourceSealV1 / ParsedProgramWithSourceV1 (non-Clone)
  -> legacy AST-only APIs project the AST from that same path and discard seal
```

The parser invocation session is the sole owner of the fresh brand, source
Box/member sites, member cursor, unpublished ordered inventory, and prepared
seal payload. `ParserMetadata` remains cloneable diagnostic metadata and is not
source authority. AST data, JSON, Builder, MIR, resolver, provider, and runtime
must never own or forge a seal.

The delegate postpass must not accept only an AST and append generated rows to
a detached inventory. It either consumes/returns the unpublished source-aware
transaction or remains outside the seal-producing path. The final seal is
issued only after delegate lowering has completed. AST-only compatibility APIs
must call the rich canonical parse path once; a second scanner/rescan is not an
authority.

R6 I0 is bounded to ordinary Rust `box` parsing. Interface/static inventories,
top-level build-gated source selection, Hako parser connection, and resolver
semantic publication remain nonclaims unless a later row explicitly opens
their source-authority seam. For a build-gated cohort, the session must either
survive prune with a typed rebase or issue no seal; it must never invent a
post-prune ordinal from the final AST.

### R6-D0 owner and removal contract

```text
AST / JSON:
  descriptive ordered inventory and inventory placement only

Parser transaction:
  fresh brand, exact source sites, typed generated origins, unpublished rows

ParserBoxSourceSeal:
  non-Clone final source authority, issued exactly once after postpasses

Resolver:
  consumes the final seal; never reconstructs from names/ordinals/maps
```

The sidecar and raw ordinal retirement is one transaction cutover, not a
partial patch. `method_source_member_ordinals`,
`record_new_methods_since`, raw `DelegateDecl` source ordinals, and parallel
ordinal-slice merge APIs are deleted in the same series that makes every
ordinary producer use the transaction. A seal constructor is not public and
cannot be created by tests with arbitrary rows.

The bounded implementation series is:

```text
R6-S0  rename/clarify inventory ordinal vocabulary; preserve JSON wire spelling
R6-S1  parser-private brand, source sites, transaction, prepared/final seal
R6-S2a parser-session ingress: one invocation brand and exact top-level
       statement cursor; no producer or sidecar cutover yet
R6-S2  ordinary Box direct/property/gate/delegate producer cutover;
       delete sidecar/raw ordinal/parallel merge APIs
  R6-S3A bounded canonical rich parse output across prune + delegate
       postpass for ordinary Rust Box; final seal and focused guard
  R6-S3B AST-only projection cutover, top-level build-gate rebase, and
       source-aware delegate transport
```

Each slice is behavior-preserving and updates focused tests, the owner README,
the active task receipt, and the applicable reference in the same commit.
R6-D0 is now accepted after the parser-path audit. R6-S0 is closed: it only
renames the descriptive inventory-placement vocabulary and preserves the JSON
wire spelling; it does not connect a parser seal or change parser behavior.

### R6-S0 implementation receipt

`BoxMethodDeclarationSiteV1` is retired from the Rust AST carrier. The
descriptive placement type is now `BoxMethodInventoryOrdinalV1`, with
`inventory_ordinal()` as the canonical accessor. The existing
`selected_method_ordinal` JSON-v2 field and accessor remain only as explicit
wire/compatibility vocabulary. No parser source identity, resolver capability,
or non-Clone seal is issued by this row. The focused AST tests and frontend
crate checks are green.

### R6-S1 implementation receipt

The parser now has a disconnected private source-authority substrate in
`src/parser/source_authority.rs`. `ParserInvocationBrandV1` issues a fresh
invocation identity; declaration/member/method source-site types carry that
brand; and `OpenBoxMethodSourceTransactionV1` is the single unpublished owner
for the ordered inventory plus explicit source relations. Duplicate inventory
entries and foreign invocation sites fail before a partial relation is
published. `PreparedBoxSourceSealV1` is non-Clone, while
`ParserBoxSourceSealV1` is private, non-Clone, and has no constructor in this
slice. It can only be issued by the later final rich parse-output row after
prune and delegate postpass.

R6-S1 deliberately does not connect AST parse output, build-gate pruning,
delegate lowering, resolver, JSON, or sidecar retirement. The focused
`frontend_parsed_box_source_seal_r6_s1_guard.sh` proves the type vocabulary,
transaction ownership, no early final-seal constructor, and the
below-800-line boundary. Later R6 rows may consume this substrate; this guard
does not assert historical disconnection after those rows land.

### R6-S2a implementation receipt

`NyashParser` now owns one fresh `ParserInvocationBrandV1`, a monotonic
top-level statement cursor, and the active top-level statement ordinal. The
cursor is assigned once before each top-level statement parse and cleared after
successful parsing. Focused parser-session tests prove fresh invocation brands,
two-statement cursor advancement, and inactive state after completion.

This is ingress only: no Box producer opens the source transaction yet, no
build-gated branch is treated as an ordinary Box source site, no sidecar is
retired, and no rich parse product, final seal, delegate postpass, or resolver
connection is introduced. The `frontend_parsed_box_source_seal_r6_s2a_guard.sh`
owns this boundary.

### R6-S3A bounded rich parse product (implementation slice)

R6-S3A opens only the smallest final-output slice needed to prove that the
parser-owned payload survives the existing Rust parse postpasses. The canonical
entry is:

```text
parse_from_string_with_source_seal
  -> one parser invocation/source transaction
  -> AST parse
  -> build-gate prune (ordinary direct Box cohort only)
  -> existing delegate AST postpass
  -> source_seal::finalize_program
  -> ParsedProgramWithSourceV1 (non-Clone)
```

The finalizer is the sole issuer of `ParserBoxSourceSealV1`. It compares the
prepared ordered inventory with the final AST inventory, requires the prepared
rows to remain an exact prefix, and accepts only delegate-generated suffix rows
as generated provenance. The final product exposes the AST as a compatibility
projection and keeps the parser seal private/non-Clone; no resolver or semantic
callable product is issued here.

R6-S3A is intentionally bounded to direct top-level ordinary Rust `box`
declarations. It rejects top-level build-gate nodes and interface/static/record
Box declarations instead of issuing a partial seal. The existing delegate
lowerer remains an AST-only postpass in this slice; generated delegate rows are
validated as generated placement, while source-aware delegate relation transport
remains a later R6-S3B/H3 seam. This is not a second source authority and does
not authorize a rescan, name lookup, ordinal reconstruction, or AST rewrite.

Acceptance for this slice is the focused rich-product test/guard plus the
existing parser-session and AST inventory gates. The implementation and its
reference receipt landed together. R6-S3B-D0 and S3B-A are closed. The current
design stop is S3B-B for top-level gate path/cursor and atomic source-session
prune/rebase; source-aware delegate transport remains later S3B-C/D work. No
resolver `CallableContract`, target, Recipe, Builder, MIR, provider, or runtime
work opens here.

### R6-S3B design and implementation ladder

External top-down audit receipt (2026-08-08): **revise before
implementation**. R6-S3A is accepted only as a bounded ordinary-Box canary;
it is not the final resolver-grade architecture. In particular, the current
AST-only delegate postpass adds generated suffix placement, while the current
seal relation set has no `GeneratedDelegate` source relation. S3B therefore
must not treat that suffix as complete source authority. The accepted S3B
handoff and owner contract is the dedicated SSOT:
`docs/development/current/main/design/parser-postpass-source-handoff-ssot.md`.

The audit fixes the following before code is allowed:

```text
one move-only ParserPostpassProductV1 owns AST + source session + metadata
parser-issued structural Box paths include gate branch and child coordinates
gate prune consumes/returns the same product and preserves original paths
delegate lowering consumes/returns the same product and commits a
  GeneratedDelegateSourceRelation atomically
AST-only APIs project from the rich path exactly once
final ParserBoxSourceSealV1 is issued only after complete relation coverage
```

Until the delegate relation exists, the rich path may keep a valid generated
delegate suffix explicitly outside the final source seal. No resolver-grade
seal may silently treat an AST-only generated suffix as an explicit source
relation; malformed or provenance-invalid suffixes still reject the product.

The design question is closed; the following implementation ladder is the only
permitted continuation:

```text
Which single typed product crosses the postpass boundary so that
AST-only callers project from the rich parse result, while build-gate
selection and delegate lowering cannot forge or lose source identity?
```

Before implementation, the owner table and negative matrix must fix all three
seams:

```text
AST-only API:
  call the rich canonical path once and discard the non-Clone seal;
  no second scan and no `ParserMetadata` authority.

Top-level build gate:
  either carry the transaction through prune with an exact typed rebase,
  or reject the rich product; never invent post-prune ordinals.

Delegate postpass:
  either consume/return the branded unpublished transaction, or remain an
  explicitly descriptive generated-suffix adapter outside source relations;
  it must not mutate a detached resolver-grade inventory.
```

Required S3B decision receipts are:

1. one postpass handoff type and one issuer/consumer owner;
2. exact mapping for source member sites, selected-gate paths, generated
   origins, and inventory placement;
3. `NoSafeSlice`/`Rejected` diagnostics for unsupported gate or delegate
   cohorts;
4. projection parity tests proving AST-only and rich paths parse once;
5. a guard and reference update in the same implementation slice later.

The S3B-D0 receipts are accepted and S3B-A/B0/B1/B2 are closed. `R6-S3B-B3-D0`
is the design-stop cell after parser-issued gate-ledger transport,
explicit top-level scope, a distinct gate-path type, typed selection receipts,
and atomic source-session prune/rebase. Do not add
top-level gate rebase,
source-aware delegate transport, interface/static/record seals, Hako parser
parity, resolver declaration, `CallableContract`, target, Recipe, Builder,
MIR, provider, or runtime code.

### R6-S3B-A implementation receipt — closed

The first S3B-A cell now owns one non-Clone
`OpenParserPostpassProductV1` containing the AST, a named
`ParserSourceSessionV1` for prepared parser source payloads, and cloneable
diagnostic `ParserMetadata`. The bounded rich path
consumes and returns that product through prune and delegate lowering, then
the product alone may issue `ParsedProgramWithSourceV1`. A crate-visible
`parse_from_string_with_source_seal_ast` helper projects the finalized rich
result to AST exactly once for the direct ordinary-Box cohort.

This cell does not add gate structural paths or `GeneratedDelegateSourceRelation`;
the existing gate/delegate behavior remains a compatibility canary and no new
resolver-grade relation claim is made. The focused S3B-A guard and tests prove
product handoff, diagnostic metadata separation, and rich/projection AST
parity. The implementation/reference/guard closeout lands together.

### R6-S3B-B design receipt and B1 implementation boundary

The design receipt is accepted in
`docs/development/current/main/design/parser-postpass-source-handoff-ssot.md#r6-s3b-b-design-receipt--accepted-b1-implementation-opened`.
The current `statement_ordinal` cannot identify multiple Boxes inside one
gate. B must add parser-issued gate id/branch/child structural paths and one
`BuildGateSelectionReceiptV1`, then make product prune consume/return a
complete source session atomically. The final AST ordinal remains forbidden
as source identity. B1 and B2 are closed: B2 owns the parser ledger,
source-preordered cursor, one selection receipt per opened gate, and
consume-return source-session prune/rebase. B3 is now the design stop. Delegate relation,
interface/static/record, Hako, resolver, and MIR work stay closed.

### R6-S3B-B1 implementation receipt

B1 adds only parser-private structural transport. `NyashParser` issues a
monotonic `SourceBuildGateIdV1`, tracks the active
`SourceBoxDeclarationPathV1`, and installs a `SourceBoxPathCursorV1` for each
gate branch. Multiple Boxes in one branch receive different child ordinals;
nested gates append segments. `OpenBoxMethodSourceTransactionV1` consumes the
path directly, so no final-AST or inventory-ordinal reconstruction is used.

Focused parser-session tests cover direct, sibling, and nested gate paths. The
B1 guard and callable-contract reference are updated in the same implementation
slice. B2 now closes the selection receipt, source-preorder ledger, and
consume-return source-session prune/rebase; later resolver/delegate work remains
unimplemented.

### R6-S3B-B2 implementation boundary

B1 is complete and pushed. The B2 design is accepted after an independent
authority audit. B2 is limited to a
parser-owned gate source ledger,
non-forgeable `BuildGateSelectionReceiptV1`, and a consume-return atomic
AST/source prune transaction. AST nodes do not receive gate IDs; the AST walk
validates parser-issued records using a structural cursor and does not issue
source identity.

The ledger record must carry the invocation brand, parser gate ID, a distinct
`SourceBuildGatePathV1`, and an explicit scope. Only top-level build-gate items are opened
in this row. Gates encountered inside Box methods or method bodies remain a
documented nonclaim/reject boundary; a Box declaration path must not be reused
as their source path.

Acceptance requires one receipt per parsed top-level gate, exact direct/
sibling/nested/empty-gate coverage, selected-branch seal preservation,
unselected-branch removal, same-brand validation, parser-preorder record order,
leftover/end-of-stream coverage, duplicate/missing/foreign rejection, and
whole-product discard on any mismatch. No delegate relation,
finalizer expansion, Hako parity, resolver, Recipe, Builder, or MIR work opens.

### R6-S3B-B2 implementation receipt — closed

The B2 parser implementation is landed. Parser source-preorder records are
transported in `ParserSourceSessionV1`; the original AST is validated once by
the source-aware cursor, and each opened top-level gate issues exactly one
private `BuildGateSelectionReceiptV1`. Direct, sibling, nested, and empty
gates are covered; selected source seals retain their original Box paths and
unselected seals are removed through consume-return pruning. Method/body and
other closed scopes do not enter the top-level ledger. Focused parser tests,
the B2 guard, `cargo check --bin hakorune`, formatting, and diff checks were
green in this slice. B3-D0 remains a design stop; B3-I0 and delegate relations, Hako
parity, resolver, Recipe, Builder, MIR, provider, and runtime work stay
closed.

### R6-S3B-B3-D0 design stop — finalizer alignment

The next row is design-only. Its decision is to close the exact final
AST↔source-seal coverage contract before opening any delegate relation
implementation. `ParserSourceSessionV1` remains the sole unpublished source
transaction owner and the existing finalizer remains the sole issuer of the
non-Clone `ParserBoxSourceSealV1`; B3 introduces no new semantic receipt or
second issuer. Final-AST/inventory ordinals, generated delegate suffix
provenance alone, delegate AST mutation, and AST-only projections are not
source authorities.

The bounded next implementation, after this design stop is accepted, is only
finalizer alignment: explicit and generated-property rows retain exact
same-brand coverage, while a valid generated-delegate suffix remains in the
AST/descriptive compatibility projection but stays outside the resolver-
visible source seal. A malformed or provenance-invalid suffix rejects the
rich product. The existing generated-delegate placement test remains a
descriptive canary, not a resolver-grade claim. `R6-S3B-C` owns the later
source-aware delegate transaction and `GeneratedDelegateSourceRelation`.

The implementation must use a private `FinalizerCoveragePlanV1` to match
parser source paths to final AST Boxes one-to-one. Final AST/inventory
ordinal, name order, `HashMap` order, positional `zip`, and postpass reindexing
are not source identity. B3 adds no public semantic receipt or second issuer.

Nonclaims for B3 are delegate relation implementation, interface/static/record
cohorts, Hako parity, resolver, `CallableContract`, target, Recipe, Builder,
MIR, provider, runtime, fallback, source rescan, and AST rewrite.

The next row is `R6-S3B-B3-I0` and is not opened in this design-stop slice.
It may implement only the private finalizer coverage plan and its focused
ordinary-Box positive/negative guard, while preserving the valid generated
delegate placement canary outside the final source seal. The same slice must
update the landed reference and parser-module owner documentation.

## Ordered implementation series

```text
Rust BoxShape series:
  R0 AST inventory model + focused model tests
  R0A inventory API authority correction
  R1 AST field + CompatibilityOnly consumer cutover
  R2 shared pending/direct issuance substrate
     + interface/static ExplicitSource cutover
     + build_cfg metadata-preserving transform
  R3 ordinary sole-inventory cutover
     + selected-gate/property/delegate atomic transactions
  R4 JSON Box codec split + ordered v2 / CompatibilityOnly v1
  R5 Builder compatibility consumer migration + old helper retirement
  R6-D0 accepted: parser-owned source seal includes the final parse product
      after prune and delegate postpass
  R6-S0 closed: inventory ordinal vocabulary clarified; JSON wire spelling
      preserved
  R6-S1/S2a/S2/S3A parser-private transaction, parser-session ingress,
  producer cutover, bounded final rich parse output, and sidecar retirement
  R6-S3B-B2 parser gate-ledger, typed selection receipt, source-preorder
  cursor, and atomic source-session prune/rebase (closed)

Hako prerequisite and parity:
  H0 HAKO-PARSER-BOX-DECLARATION-CARRIER-D0
  H1 typed transaction/source-site substrate
  H2 ordinary Box parser member-draft branch
  H3 atomic inventory + parser source-seal issuer
  H4 selected-gate transaction
  H5 test-only normalized Rust/.hako parity
  H6 typed CallableContract(query) carriage and reference closeout

Then:
  resolver semantic declaration/signature
  OWN-HOME-ABI0-S0/query
  RESOLVER-DECLARED-QUERY-INSTANCE-CONTRACT-I0
```

Historical execution note: R0/R0A and R1-R5 are closed. H1's disconnected
carrier substrate, R6-S0 vocabulary slice, R6-S1 parser-private source
authority substrate, R6-S2a parser-session ingress, and R6-S2 ordinary
transaction cutover are closed. The current frontier is R6-S3A: bounded rich
ordinary-Box parse output and final non-Clone seal after existing postpasses.
Source-aware delegate relation transport, top-level build-gate rebase, raw
DelegateDecl ordinal retirement, AST-only projection cutover, H2-H6, H5
parity, and all resolver declaration rows remain later design stops. No
caller-zero model may become a second AST or production route.

## Verification contract

Focused positive/negative coverage includes:

```text
ordered direct methods
direct duplicate
selected branch order/rebase
outer/selected collision
generated/source collision
generated provenance cannot forge ExplicitSource
v2 ordered codec roundtrip
v1 rows are CompatibilityOnly
name-order compatibility behavior unchanged
no resolver compat import
one parser invocation/source-authority session across prune and delegate postpass
final seal only after delegate-generated inventory rows are committed
AST-only APIs project the same rich parse path and do not rescan
non-Clone seal cannot be issued from AST/JSON/ParserMetadata/test constructors
all touched source files below 800 lines
```

Normalized Rust/`.hako` parity is test evidence only, not semantic transport.
It compares ordered Box/method identity, arity/result token, provenance, and
normalized runes from the same source/profile; spans and runtime addresses are
verified separately.

Every implementation cell updates its owner README, focused guard/index, this
task receipt, and the exact `docs/reference/**` status in the same commit.
