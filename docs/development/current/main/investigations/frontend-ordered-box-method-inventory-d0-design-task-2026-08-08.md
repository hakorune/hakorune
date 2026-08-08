---
Status: closed — authority, API, census, retirement, and implementation order fixed
Date: 2026-08-08
Decision: one AST-owned ordered inventory; selected-gate source remains explicit source
Parent: `language-typed-callable-profile-d0-design-task-2026-08-08.md`
Next: `frontend-ordered-box-method-inventory-r0-implementation-task-2026-08-08.md`
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
    site: BoxMethodDeclarationSiteV1,
    diagnostic_span: Span,
}
```

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

The selected compilation identity is:

```text
source/catalog brand
program Box statement ordinal
selected method ordinal
```

The raw inventory owns the Box-local selected method ordinal as an ordered AST
carrier. It is not itself a resolver-grade source capability. A later
parser-owned seal proves complete parsing, selected Box membership, duplicate
freedom, and exact source origin. The program/resolver
inventory later co-seals the Box statement ordinal and brand. `Span` is
diagnostic only. Selected-gate provenance retains every gate site and original
branch member ordinal in outer-to-inner order while the selected ordinal is
rebased once at commit.

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

## Ordered implementation series

```text
Rust BoxShape series:
  R0 AST inventory model + focused model tests
  R0A inventory API authority correction
  R1 AST field + CompatibilityOnly consumer cutover
  R2 ordinary/interface/static parser ExplicitSource issuance
     + duplicate/site sealing
  R3 selected-gate and generated-producer atomic transactions
  R4 JSON Box codec split + ordered v2 / CompatibilityOnly v1
  R5 Builder compatibility consumer migration + old helper retirement

Hako prerequisite and parity:
  H0 HAKO-PARSER-BOX-DECLARATION-CARRIER-D0
  H1 typed declaration carrier substrate
  H2 ordinary Box parser branch
  H3 ordered inventory + duplicate/site issuer
  H4 selected-gate transaction
  H5 test-only normalized Rust/.hako parity
  H6 CallableContract(query) carriage and reference closeout

Then:
  RESOLVER-DECLARED-QUERY-INSTANCE-CONTRACT-I0
```

R0 and R0A are closed and R1 is the active executable row. Temporary
caller-zero model code may exist only inside this bounded Refactor Series and
must connect through R1/R2; it cannot become a second AST or production route.

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
all touched source files below 800 lines
```

Normalized Rust/`.hako` parity is test evidence only, not semantic transport.
It compares ordered Box/method identity, arity/result token, provenance, and
normalized runes from the same source/profile; spans and runtime addresses are
verified separately.

Every implementation cell updates its owner README, focused guard/index, this
task receipt, and the exact `docs/reference/**` status in the same commit.
