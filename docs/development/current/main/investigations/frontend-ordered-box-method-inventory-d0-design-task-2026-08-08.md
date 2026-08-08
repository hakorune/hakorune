---
Status: R6-D0/R6-S0/R6-S1 closed — parser source-seal/final parse-product boundary fixed; R6-S2 next
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
try_merge_selected_gate(unpublished selected inventory, gate site)
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
R6-S2  ordinary Box direct/property/gate/delegate producer cutover;
       delete sidecar/raw ordinal/parallel merge APIs
R6-S3  canonical rich parse output across prune + delegate postpass;
       AST-only projections, final seal, focused end-to-end guard
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
transaction ownership, no early final-seal constructor, parser
non-connection, and the below-800-line boundary.

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
  R6-S1/S2/S3 parser-private transaction, producer cutover, final rich parse
      output, and sidecar retirement

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
carrier substrate, R6-S0 vocabulary slice, and R6-S1 parser-private source
authority substrate are closed. The current frontier is R6-S2: ordinary
producer cutover and sidecar retirement. R6
must close its canonical rich parse-output and delegate-postpass boundary
before H2-H6, H5 parity, or any resolver declaration row. No caller-zero model
may become a second AST or production route.

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
