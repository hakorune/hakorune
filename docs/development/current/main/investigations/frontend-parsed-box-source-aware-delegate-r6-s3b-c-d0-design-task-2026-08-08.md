---
Status: accepted design boundary; implementation not opened
Date: 2026-08-08
Decision: R6-S3B-C is a parser-private source-aware delegate transaction; R6-S3B-D alone may extend the resolver-visible final seal
Parent: `docs/development/current/main/design/parser-postpass-source-handoff-ssot.md`
Next: `R6-S3B-C-S0` source transport, then `R6-S3B-C-I0` atomic delegate batch
---

# FRONTEND-PARSED-BOX-SOURCE-AWARE-DELEGATE-R6-S3B-C-D0

## Decision

The next delegate slice is a parser/source-transport correction, not a
resolver or semantic-contract implementation.

```text
ParserSourceSessionV1
  -> source-aware delegate transaction
  -> parser-private GeneratedDelegateSourceRelationV1 rows
  -> one atomic AST/inventory/relation batch
  -> OpenParserPostpassProductV1
  -> R6-S3B-D final complete source seal
```

`GeneratedDelegateSourceRelationV1` is not a `Verified*` resolver product in
this row. It is parser-private evidence transported by the same unpublished
source transaction that owns the AST and method inventory. Until D closes,
valid generated delegate rows remain outside `ParserBoxSourceSealV1`; malformed
or provenance-invalid rows reject the whole unpublished product.

The source authority is one-way:

```text
parser-issued source declaration/relation
  -> transaction-owned relation row
  -> finalizer coverage in D
```

The following are never source authority:

```text
final AST order
BoxMethodInventoryOrdinalV1
delegate provenance alone
HashMap<String, BoxInfo>
method/Box names or name-sorted order
AST rescans after the postpass
```

## Sole owners

| Meaning | Sole owner | Forbidden reconstruction |
| --- | --- | --- |
| parser invocation, source paths, unpublished rows | `ParserSourceSessionV1` / `OpenBoxMethodSourceTransactionV1` | names, spans, AST order, JSON |
| selected/generated placement | `BoxMethodInventoryV1` | resolver identity |
| as-written delegate member/expose site | parser-issued `DelegateSourceDeclarationV1` | generated method suffix |
| target Box/method source relation | private path-based target index backed by parser source transactions | `HashMap` entry order, method-name-only lookup |
| generated delegate relation coverage | C transaction batch | per-host partial mutation |
| resolver-grade final source authority | finalizer in R6-S3B-D | early C relation rows, AST-only constructors |

`BoxMethodInventoryV1` remains Clone-capable descriptive data. The final
non-Clone seal remains the only resolver-facing issuer. C does not add a
second final seal issuer or a second source registry.

## Canonical relation

There is exactly one relation row per parsed `expose` entry. The parser records
the expose ordinal while the delegate declaration is read; the ordinal is not
reconstructed from generated inventory placement.

```rust
GeneratedDelegateSourceRelationV1 {
    host_box_path: SourceBoxDeclarationPathV1,
    host_delegate_member: SourceBoxMethodSiteV1,
    expose_ordinal: u32,
    delegate_field_name: Box<str>,
    source_method_name: Box<str>,
    exposed_method_name: Box<str>,
    target_box_path: SourceBoxDeclarationPathV1,
    target_method_source_ref: ExistingTargetMethodSourceRefV1,
    generated_inventory_placement: BoxMethodInventoryOrdinalV1,
    generated_name_provenance: GeneratedDelegateOriginReceiptV1,
}
```

Names are lookup and diagnostics only. The authority is the same-brand host
path, delegate member site, expose ordinal, target path, and existing target
method source relation. `generated_inventory_placement` records where the
generated method was placed; it is never a declaration identity.

`ExistingTargetMethodSourceRefV1` is limited to an already source-backed
explicit method relation (and, if the bounded cohort explicitly admits it, an
existing generated-property relation). A generated-delegate target or a
delegate chain is outside this cohort and must not be promoted by name lookup.

## Parser-time transport and transaction

The source record is captured before postpass AST generation:

```text
parse delegate declaration
  -> record one DelegateSourceDeclarationV1 per expose
     (host member site, field, source/exposed names, expose ordinal,
      selection/path relation)
  -> finish the Box source transaction
```

The postpass then performs one consume-return transaction:

```text
1. match every host Box to its parser-issued path
2. build a private target index keyed by exact source Box path
3. resolve every target method to an existing source relation
4. preflight every host/expose, target, source path, generated name,
   placement, duplicate, collision, and provenance
5. stage the complete generated AST batch, inventory placements, and
   GeneratedDelegateSourceRelation rows
6. commit all staged changes once and return a new postpass product
```

Any failure drops the complete unpublished product. There is no partial
per-host commit, rollback repair, same-session retry, or fallback to a
name-based target. Duplicate Box names in the private index are rejected;
names may assist lookup only after path/brand identity has been established.

## Bounded C cohort

The first implementation cohort is intentionally narrow:

```text
ordinary top-level Rust Box declarations after the B3 path/finalizer gate
direct explicit target method
same parser invocation/source brand
one generated forwarding method per expose
```

The following remain `Declined` when fully observed, or are not opened in C:

```text
generated-delegate target and delegate chains
CompatibilityOnly delegates
interface/static/record declarations
Hako parser delegates
provider/plugin-generated declarations
overload or ambiguous target selection
```

Missing parser issuer or typed relation transport is development-state
`NoSafeSlice`, not a source disposition. Fully observed unsupported target
provenance is `Declined`. Foreign, duplicate, malformed, missing-but-required,
or contradictory source evidence is `Rejected`; unavailable source evidence
is `Unresolved`.

## Acceptance matrix

```text
positive:
  one direct target method, one selected-gate host, multiple exposes
  exact host/target paths, one row per expose, generated placement receipt

negative:
  duplicate Box name/path
  foreign invocation brand or path
  missing target Box/method relation
  generated-delegate target/chain
  duplicate expose/relation or generated-name collision
  AST/inventory/provenance mismatch
  partial batch failure after another host was staged
  fresh product repeat after success and after failure
```

The positive case proves only parser source transport and atomic generated
batch coverage. It does not prove resolver declaration, CallableContract,
Home/ABI, target catalog, Recipe/CallSlot, Builder/MIR, provider dispatch, or
production publication.

## Ordered implementation ladder

```text
R6-S3B-C-D0 (this row)
  source authority, relation fields, cohort, disposition, atomicity, and
  nonclaims; docs-only design stop

R6-S3B-C-S0
  parser-time DelegateSourceDeclarationV1 transport and expose ordinals;
  selected-gate path rebasing; focused malformed/foreign/duplicate tests

R6-S3B-C-S1
  private path-based target index and existing target-method source relation
  lookup; no name-based source identity

R6-S3B-C-I0
  prepare all hosts/exposes, stage complete generated batch, commit AST +
  inventory placement + relation rows atomically through the postpass product

R6-S3B-C-T0
  positive/negative/fresh-product guard and same-slice reference/module docs

R6-S3B-D
  extend finalizer relation coverage, issue the complete non-Clone seal,
  retire the bounded generated-suffix adapter, and close AST-only parity
```

Every implementation slice must update its owning reference, module README,
focused tests, and guard in the same commit. No C implementation begins from
this design card until the current design-stop guard is green.

## Nonclaims until R6-S3B-D

```text
no resolver-visible GeneratedDelegateSourceRelation
no final seal expansion
no CallableContract or instance target
no Recipe/CallSlot/Builder/MIR/ABI/Home/provider/runtime connection
no Hako parser parity
no generated-delegate chain semantics
no fallback, retry, partial publication, or AST rewrite
no production selection or legacy deletion
```

## File and review rules

The implementation must preserve the repository cleanliness contract:

```text
source_seal.rs and delegate modules remain below 800 lines
one authority per source relation
one consume-return transaction per postpass
tests encode the exact acceptance matrix
debug output is absent unless an existing guarded diagnostic contract is used
```

This card is a design receipt, not an implementation receipt. The next safe
action after this docs-only closeout is S0 source transport; if any source
identity or transaction invariant cannot be issued, stop at `NoSafeSlice` and
revise the design instead of adding a test constructor or a by-name shortcut.
