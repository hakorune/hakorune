# Dynamic carrier ingress lifecycle

Status: parser callable-source series closed; normal source-carrier cutover selected
Date: 2026-08-10
Parent: `DYNAMIC-CARRIER-REBIND-TRANSACTION-D0`
Current implementation row: `NORMAL-CALLABLE-SOURCE-CARRIER-CUTOVER-R0`
Parked implementation row: none
Prerequisite terminal: initial parser co-seal is closed; no new parser row may be inserted.
First production finish: `ParserScanLoopBox.skip_while/4` switches to the canonical pipeline, deletes the selected old Dynamic route, and retains zero retry/fallback.
Post-cutover promotion gate: `MIRBUILDER-HAKO-MIMALLOC-PROMOTION-GATE0` runs before .hako selfhost MirBuilder/parser migration resumes; its contract remains in the migration-order SSOT.

## Decision

The exact initial root-carrier chain is:

```text
static ParserScanLoopBox.skip_while/4
parameter #1 pos
  -> exact Pos BindingRef
  -> PreludeInitializerPos
  -> local i / induction BindingRef
  -> Recipe input V1
  -> carrier C0 / root L0 / binding B0 / Dynamic
  -> JoinSig Enter(B0 = V1 : Dynamic)
```

A plain parameter has callable-boundary `Handle` demand. Therefore this exact
initial B0 instance is:

```text
BorrowedIngressNoEnd
```

`local i = pos` is borrowed-alias propagation. It is not a Home transfer, an
independent copy, or an owned Dynamic carrier publication. Displacing this
initial B0 instance performs carrier End zero times.

Later V17 is different. The Dynamic operator contract gives V17
`EndExactlyOnceUnlessForwarded`; after atomic forwarding into B0, the current
B0 instance is owned. Carrier flow must retain the disposition of each origin
instead of assigning one lifecycle rule to the B0 key.

## Sole authority

The Dynamic profile must not issue `plain = Handle`. The existing
`VerifiedHomeAbiV1` must not be widened directly: it is a nominal instance
method, receiver-bearing, I64/Unit cohort and cannot honestly classify the
static untyped `skip_while/4` declaration.

The selected common authority is:

```text
parser-sealed parameter transfer syntax
+ exact resolved callable declaration and parameter BindingRefs
  -> VerifiedCallableParameterDemandCatalogV1
       complete ordered rows for every parameter
       Ordinary -> Handle
       Take     -> future accepted Home-demand capability
```

The catalog owns parameter demands only. It owns no receiver, result,
Dynamic, Recipe, carrier, CFG, or physical ABI meaning. Existing and future
callable Home ABI aggregates must consume or project these rows rather than
reissue parameter demand independently.

The parser source authority is a sibling product, not an expansion of the
existing `ParserBoxSourceSealV1`:

```text
same parser invocation provenance
+ exact direct Box-method source coordinate
+ complete ordered parameter syntax rows
  -> ParserCallableParameterSourceCatalogV1
```

`ParserBoxSourceSealV1` remains the ordinary-Box post-prune/delegate owner.
Static boxes currently use an AST-only compatibility lane; forcing
`ParserScanLoopBox` into that seal would incorrectly couple parameter syntax
to inventory/delegate/build-gate policy. The sibling catalog supports direct
static and direct ordinary instance methods through one source coordinate:
parser provenance, Box statement/path, source member ordinal, then parameter
ordinal. Inventory ordinal and method name are diagnostics, not identity.

The cloneable AST `ParamDecl` remains a neutral name/type projection. A
parser-private one-shot parameter-list product owns transfer syntax and lends
that projection to AST construction; neither `ParamDecl` nor its legacy
name-only fallback can issue `Ordinary` evidence.

Parameter type syntax is optional for `Ordinary`. The unchanged
`skip_while(src, pos, end, pred_chars)` declaration is untyped. Missing type
syntax is represented explicitly, never as an error or an empty-string type.
Future accepted `Take` syntax still requires its exact type relation.

The Dynamic ingress issuer is a one-way relational co-seal:

```text
VerifiedCallableParameterDemandCatalogV1
+ whole VerifiedDynamicOperatorCarrierLifecycleProgramV1
+ exact parameter -> prelude -> Recipe carrier -> JoinSig Enter relation
  -> VerifiedDynamicCarrierIngressLifecycleProgramV1
       private ingress row = BorrowedIngressNoEnd
```

The whole result is non-Clone and non-splittable. It exposes at most a
borrow-scoped ingress view. No public constructor accepts an ordinal, site,
demand, Recipe key, or disposition selected by the caller.

## Why implementation is still `NoSafeSlice`

The current Rust `ParamDecl` carries name and optional type but no canonical
`Ordinary | Take` transfer syntax. The current Hako parameter carrier is a
disconnected ordinary-only substrate and is not a Rust/resolver parity
authority. Absence of `take` may not be inferred from an old AST shape.

Existing source/Recipe products already prove Pos, initializer, local, V1,
C0/L0/B0, but they intentionally own no callable parameter demand. Recipe
`Dynamic`, runtime tags, selector/provider names, `MirType`, ValueId,
`ReleaseStrong`, and source names cannot fill this gap.

## Ordered tasks

### 1. `CALLABLE-PARAMETER-TRANSFER-AUTHORITY-D0` — accepted

Close the common Rust/Hako authority contract:

- typed closed syntax vocabulary `Ordinary | Take`;
- exact callable declaration, method/function site, parameter ordinal, and
  parser provenance;
- static and instance declarations use one parameter identity boundary;
- no raw string tag, builder-instance token, old-AST absence inference, or
  Home meaning in the parser seal;
- reuse the existing `HAKO-PARAMETER-TRANSFER-TYPED-SEAL-D0/R0` work instead
  of creating a second Hako vocabulary.

Selected Hako representation:

```text
parser-private ParserParameterTransferKindV1::{Ordinary, Take}
+ opaque transfer wrapper bound to one parameter-list issuer seal
```

There is no raw-kind getter. R0 exposes only the Ordinary issuer; the Take
variant is reserved but has no issuing API until Take I0. The parser source
session issues one exact method-bound parameter-list seal, rejects duplicate
issuance, and the final product exposes only limited same-source/ordinary-row
queries. Builder identity and `sealed_token()` are not provenance.

### 1A. `HAKO-PARAMETER-TRANSFER-TYPED-SEAL-R0A` — closed

Replace raw `"Ordinary"` classification with the closed private vocabulary
and opaque row capability. No grammar or semantic behavior changes.

The landed row also represents an untyped ordinary parameter as explicit
`Absent` declared-type syntax rather than rejecting or inferring an empty
String token. `Take` remains vocabulary-only with no issuer.

### 1B. `HAKO-PARAMETER-TRANSFER-TYPED-SEAL-R0B` — closed

Issue the parameter-list seal from `ParserProgramSourceSessionV1`, bind it to
the exact method, and remove builder-as-token plus `sealed_token()`.

The product now retains only parser-source and exact-method relations. Foreign
session and duplicate method issuance reject before publication, and the
guard prevents direct sealer/session/product bypasses.

### 1C. `PARSER-CALLABLE-PARAMETER-SOURCE-RECUT-R0` — closed

Before adding Rust parameter rows, extract their model/issuer from the
near-limit parser owners. `source_seal.rs` is already above 750 lines and must
not receive the new authority. Keep the sibling catalog in a dedicated
`callable_parameter_source/` module and keep tests separate.

The behavior-neutral recut now owns the existing AST-free name/type model,
the `ParamDecl` compatibility projection, focused tests, and a module-local
README under `src/parser/callable_parameter_source/`. The general resolver
handoff only consumes that owner. No transfer row, parser brand, declaration
identity, `Take`, Home demand, Recipe key, or MIR fact was added;
`source_seal.rs` remains unchanged at 751 lines and every touched Rust source
remains below 800 lines.

### 1D. `PARSER-BOX-MEMBER-SOURCE-CURSOR-RECUT-R0` — closed

Extract the parser brand, exact Box path, and source-member ordinal cursor
from `OpenBoxMethodSourceTransactionV1`. Ordinary Box transactions retain the
same behavior through that cursor; the static Box parser opens the same
cursor and advances it exactly once for every successfully parsed source
member.

This is a behavior-neutral prerequisite. It emits no parameter rows and does
not place static Boxes into `ParserBoxSourceSealV1`. Inventory ordinal, method
name, Box name, and arity remain non-authorities. If every successful static
member arm cannot close one exact cursor step, stop before parameter I0.

Closeout receipt:

- `ParserBoxMemberSourceCursorV1` is the single parser-private owner of the
  parser brand, exact Box declaration path, and source-member ordinal;
- `OpenBoxMethodSourceTransactionV1` embeds and delegates to that cursor;
- the static Box parser advances the same cursor once after each successful
  field, method, initializer, or init-block parse without publishing a seal;
- focused cursor/source-authority tests are green and all touched Rust source
  files remain below 800 lines.

### 2. `CALLABLE-PARAMETER-TRANSFER-SOURCE-SEAL-I0` — closed

Land the complete parser/resolver handoff and Rust/Hako parity. First active
cohort issues exact `Ordinary` rows for direct static Box methods and direct
ordinary instance methods; this does not activate `take`. Top-level functions,
interfaces, constructors, generated methods, and selected build gates remain
closed until their exact source issuer exists.

Required negatives: missing/duplicate/foreign ordinal, wrong parser/catalog
brand, raw `"Ordinary"` construction, builder token as identity, and
line/context drift. Compiler acceptance must be widened if the unchanged
source cannot be represented; source rewriting and fallback are forbidden.

Closeout receipt:

- one parameter parse co-issues the neutral `ParamDecl` projection and exact
  ordered `Ordinary` source rows; no old-AST absence inference remains;
- `ParserCallableParameterSourceCatalogV1` is a non-Clone sibling of
  `ParserBoxSourceSealV1` and covers direct static plus ordinary instance
  methods under one parser brand/source-coordinate authority;
- `ParserScanLoopBox` is accepted unchanged as four methods and fifteen rows;
  `skip_while` parameter #1 remains explicit untyped/`Absent` syntax;
- foreign invocation, duplicate method site, selected build-gate, type/row
  mismatch, and split/reissue surfaces fail before catalog publication;
- Hako R0a/R0b remains the parity vocabulary/issuer receipt. No Hako source
  was rewritten and no `Take` or Home meaning was activated;
- `source_seal.rs` remains unchanged at 751 lines and every touched source is
  below the 800-line hard limit.

### 2A. `CALLABLE-PARAMETER-DECLARATION-PLACEMENT-R0` — closed

The Demand audit found one pre-semantic placement gap. A direct method's
source-member ordinal counts fields and other source members, while
`BoxMethodInventoryV1` contains method rows only. Static Boxes do not have a
`ParserBoxSourceSealV1` relation, so a later resolver could not locate the
exact committed method without falling back to its diagnostic name.

The behavior-neutral correction retains the already-issued
`BoxMethodInventoryOrdinalV1` beside each parameter declaration source row.
Source identity remains the parser brand, Box path, and source-member
ordinal; inventory placement is lookup-only and cannot authorize semantic
pairing. Static and instance fixtures with a preceding field prove the two
ordinals differ while the exact method remains addressable.

This row issues no resolver owner, `BindingRef`, demand, Home meaning, Take,
Recipe, or MIR fact. It is the final parser-side prerequisite for the selected
Demand I0.

### 2B. `CALLABLE-PARAMETER-DECLARATION-SYNTAX-LOAN-R0` — closed

The placement receipt is now consumed by one parser-private exact syntax
loan. `ParsedProgramWithCallableParameterSourceV1` moves the complete catalog
into a callback while lending only the exact committed function declarations
from the same completed postpass. The loan cannot escape the callback and
there is no AST-plus-catalog split API on this path.

The issuer indexes the selected inventory by the already-sealed placement,
then validates direct explicit provenance, static/instance kind, method
identity, and complete parameter name/type/order equality. It never searches
by Box or method name and never rebuilds source identity from the inventory
ordinal. Focused negatives reject static/instance cross-wiring, diagnostic
name repair, and parameter-type reconstruction.

This remains unpublished parser transaction staging. Resolver owner,
`BindingRef`, `HomeDemandV1`, Take, Recipe, MIR, and production activation are
still zero; the selected Demand I0 must consume this loan rather than open a
second AST lookup path.

### 3. `CALLABLE-PARAMETER-DEMAND-I0` — closed

Issue one complete `VerifiedCallableParameterDemandCatalogV1` from the sealed
syntax and resolved declaration. First cohort maps `Ordinary -> Handle` only.
Reject partial coverage, duplicate rows, foreign BindingRefs, and declaration
arity mismatch. Do not refactor the existing instance Home ABI in this row;
record its later convergence boundary and forbid duplicate new demand owners.

Closeout receipt:

- one consuming issuer borrows the exact parser declarations, resolves their
  canonical owner forests, and atomically seals the complete demand catalog;
- direct static and ordinary instance roots share the same issuer/catalog
  boundary while retaining distinct owners and receiver profiles;
- every parameter retains its exact ordinal, `BindingRefV1`, and
  `HomeDemandV1::Handle`; zero-parameter declarations remain explicit empty
  rows;
- the final non-`Clone` product retains the parser catalog and resolved
  forests and exposes borrow-scoped views only;
- focused mixed-declaration, unchanged `ParserScanLoopBox` 4/15, and
  cross-session identity tests are green; `Take`, Dynamic Home, Recipe, MIR,
  retry, and fallback remain zero.

### 3A. `CALLABLE-PARAMETER-DEMAND-SHARED-SEMANTIC-SOURCE-D0` — accepted

The ingress premise audit found that the two intended inputs cannot currently
share canonical identity:

```text
parameter-demand issuer
  -> resolve_selected_callable_forests()
  -> forest allocation A / owner A / BindingRef A

normal Dynamic semantic-source issuer
  -> resolve_selected_callable_forests()
  -> forest allocation B / owner B / BindingRef B
```

Each resolver call issues fresh owners. Equal source coordinates, diagnostic
names, ordinals, or numeric-looking IDs cannot turn these independent forests
into one source authority. The consuming parser syntax loan also cannot escape
its callback to repair the pairing later. Therefore the current ingress I0 is
`NoSafeSlice`, not an implementation row.

The selected owner is a neutral resolved callable semantic batch:

```text
retained parser callable source
  completed parser postpass + complete parameter source catalog
        |
        v
VerifiedResolvedCallableSemanticBatchV1
  sole resolver call
  sole forest/projection ownership
        |-- borrowed normal semantic source view
        `-- borrowed complete parameter-demand view
```

It must:

1. consumes the selected parser declaration source once;
2. resolves every selected callable exactly once;
3. retains the sole verified forests and source projections;
4. deterministically projects the complete parameter-demand catalog and the
   existing Dynamic source/lifecycle chain from those same forests;
5. prevents either projection from owning or issuing a second resolver forest;
6. keeps the final outer aggregate non-Clone and rejects foreign pairings
   before ingress publication.

The existing Builder-owned `VerifiedNormalCallableSemanticSourceV1` is not the
sole owner. It already combines Builder selection keys, same-module catalog
branding, raw invocation lineage, and prepared-loop ingress in a 755-line file.
Making it own neutral parameter semantics would preserve the AST-loan problem
and require a second migration later. It becomes a compatibility facade/view
over the neutral batch instead.

The parameter-demand catalog also becomes a borrow projection. Its resolver
argument, independent `resolve_selected_callable_forests()` call, forest
storage, `resolved_forest()` surface, and owning constructor retire in the
projection cutover row.

Forbidden repairs:

```text
second resolver call
Box/method/parameter name lookup
source-coordinate equality as semantic identity
numeric owner or BindingRef comparison across allocations
normalized-forest equality
test-only forged Verified products
caller-supplied owner, binding, or disposition
```

The final ingress issuer retains the whole semantic batch and internally
derives both the demand and Dynamic lifecycle relations. It never accepts a
caller-paired demand catalog plus lifecycle product.

### 3B. `PARSER-CALLABLE-SOURCE-RETENTION-R0` — closed

Change:
  Keep `CompletedParserPostpassV1` and the complete callable-parameter source
  catalog in one non-Clone parser-owned retained source. Lend exact declaration
  syntax without consuming or splitting that owner.

Contract:
  This is parser transaction staging only. The AST and parameter catalog have
  no public `into_parts`, Clone, raw lookup, or arbitrary constructor. Existing
  parse coverage and exact source coordinates do not change.

Done:
  Existing parser parameter tests remain green; repeated scoped syntax loans
  return the same exact rows; foreign reconstruction and split APIs are absent;
  parser source files remain below 800 lines.

Stop:
  If retention requires cloning the AST/catalog, exposing raw parts, or adding
  resolver/Home/Recipe meaning, return to design.

Closeout:
  `RetainedParserCallableSemanticSourceV1` now atomically owns the completed
  postpass and parameter catalog. Repeated callback-scoped loans retain the
  exact declaration pointers and parser identity; equal source text from a
  foreign invocation remains distinct. Existing consuming callers remain
  unchanged. Focused tests and `cargo check --lib` are green; the new owner and
  tests are 47/55 lines, and `parser/mod.rs` remains 757 lines.

### 3C. `RESOLVED-CALLABLE-SEMANTIC-BATCH-I0` — closed

Change:
  Consume the retained parser source, resolve its complete declaration loan
  exactly once, and retain one verified forest/source projection per row in
  `VerifiedResolvedCallableSemanticBatchV1`.

Contract:
  The batch is the sole resolver/forest/projection owner for this lane. It
  lends exact `ResolvedFunctionLoweringInputV1` rows by the already-sealed
  source row identity; it does not issue parameter demand, Home, Recipe, or
  physical facts and has no name lookup or split API.

Done:
  Static and instance direct methods, zero-parameter rows, and unchanged
  `ParserScanLoopBox` resolve in exact source order. Missing/duplicate/foreign
  rows, deferred resolution, wrong root profile, and projection mismatch fail
  before publication. All files stay below 800 lines.

Stop:
  If the batch needs a second resolver call, cloned parser source, caller-made
  owner/BindingRef, or Builder-selected keys, return to design.

Closeout:
  `VerifiedResolvedCallableSemanticBatchV1` now consumes the retained parser
  source, takes one complete scoped declaration loan, calls the resolver once,
  and retains the exact ordered forest/source-projection rows. Borrow-scoped
  lowering inputs reconstruct from the retained source and the same forest;
  the batch has no split, Clone, name lookup, demand, Home, Recipe, or physical
  authority. Mixed static/instance rows, exact forest/BindingRef reuse,
  unchanged four-row `ParserScanLoopBox`, and missing-row rejection are green.
  The focused four-test suite and `cargo check --lib` pass; all new source files
  and the existing parser/MIR facades remain below 800 lines.

### 3D. `CALLABLE-PARAMETER-DEMAND-PROJECTION-R0` — closed

Change:
  Project the complete ordinary parameter-demand catalog from one borrowed
  `VerifiedResolvedCallableSemanticBatchV1` view.

Contract:
  The projection reuses the batch-owned owner forest and BindingRefs. Delete
  the demand issuer's resolver argument, independent forest resolution, forest
  storage, and `resolved_forest()` authority. Preserve exact source order and
  complete Ordinary-to-Handle rows; do not add Take, Home Flow, Dynamic
  lifecycle, Recipe, Builder/MIR, retry, or fallback.

Done:
  Existing zero/multi-parameter fixtures use the same batch owner and exact
  parameter BindingRefs; missing, duplicate, foreign, or incomplete rows fail
  before a demand catalog is published. The old resolver call and owning
  forest surface are structurally absent, focused tests and `cargo check --lib`
  are green, and all touched files stay below 800 lines.

Stop:
  If the projection must resolve syntax again, own a second forest, compare
  numeric owner/BindingRef values across allocations, or accept caller-paired
  products, return to design.

Closeout:
  `issue_callable_parameter_demands_v1` now accepts only a borrowed
  `VerifiedResolvedCallableSemanticBatchV1`. The demand catalog borrows that
  batch and owns only projected rows; its resolver argument, independent
  `resolve_selected_callable_forests()` call, forest storage, and
  `resolved_forest()` API are absent. Exact static/instance/zero-parameter
  coverage, all fifteen `ParserScanLoopBox` Handle demands, foreign-batch
  identity separation, and a structural no-resolver/no-forest guard are green.
  The focused four-test suite, semantic-batch regression suite, and
  `cargo check --lib` pass; all touched Rust files remain below 800 lines.

### 3E. `NORMAL-CALLABLE-SEMANTIC-SOURCE-PROJECTION-R0` — parked at NoSafeSlice

Change:
  Convert the existing Builder-facing normal callable semantic source into a
  child projection/compatibility facade over the neutral resolved callable
  semantic batch.

Contract:
  The facade borrows the batch-owned syntax, forest, source projection, owner,
  and origin. Delete its independent resolver/forest issuance and prevent
  Builder selection keys or raw invocation lineage from becoming neutral
  semantic authority. Preserve existing prepared-ingress behavior and caller
  shape; add no parameter demand, Home, Dynamic lifecycle, Recipe meaning,
  physical effect, retry, or fallback.

Done:
  Existing normal-source focused tests consume the same batch-owned forest and
  exact lowering input. The old resolver call and forest owner are structurally
  absent. Split the current threshold file into responsibility-named children
  before adding code so every source remains below 800 lines; focused tests,
  `cargo check --lib`, and current guards are green.

Stop:
  If the facade must clone/move batch internals, re-resolve syntax, compare
  numeric identities across allocations, or make Builder lineage the source
  truth, return to design.

Audit result:
  The stop condition is active. The neutral retained parser source covers
  direct Box methods only, whereas the production normal callable inventory
  contains top-level, static, and instance rows. `NormalCompileRequestV1` and
  `PreparedNormalDefaultProgramRootV1` carry only an AST, so they cannot convey
  the retained parser authority or prove its relation to macro/post-transform
  syntax. Batch rows also lack the exact source identity needed to co-seal a
  Builder selected key and catalog brand without name, ordinal, or numeric-ID
  repair. The existing escaping loan surface cannot be reproduced honestly
  from the batch's callback-scoped source loan.

  A behavior-preserving file split is independently safe, but it does not
  authorize projection cutover and must not be presented as semantic progress.

### 3E-D0. `NORMAL-CALLABLE-PRODUCTION-SOURCE-CARRIER-D0` — accepted

Question:
  What single product owns the complete post-transform callable source truth
  transported from parser/macro processing into normal compilation, and how is
  that truth co-sealed exactly once with normal callable selection?

The Decision must fix:

1. the canonical source owner after parser and whole-file macro/postpass work;
2. complete membership for top-level functions, static Box methods, and
   instance Box methods;
3. explicit supported or typed fail-fast disposition for generated,
   selected-build-gate, compatibility, JSON, REPL, and other AST-only origins;
4. one opaque declaration source identity stable across the authorized
   transform boundary;
5. atomic transport through `NormalCompileRequestV1` and
   `PreparedNormalDefaultProgramRootV1`, with no AST-only reconstruction;
6. one co-seal from exact source identity to `SelectedNormalCallableKey`,
   catalog brand, and a private batch slot; and
7. callback-scoped lowering/target loans that cannot escape or create another
   forest/source owner.

Decision:
  Adopt a source-aware Program transform transaction. The parser postpass
  issues the initial complete callable source product. Every owner that changes
  the AST must consume that source-bearing product and atomically reissue the
  transformed Program, complete callable rows, and exact preservation,
  replacement, removal, and generation relations. Only the final product may
  enter normal compilation.

```text
parser postpass
  -> initial callable Program source
  -> source-aware transform transaction(s)
  -> VerifiedFinalCallableProgramSourceV1
  -> NormalCompileRequestV1
  -> normal callable semantic package
```

Rejected alternatives:

- rescanning a bare post-macro AST to reconstruct source authority;
- attaching a pre-transform parser catalog to an independently transformed
  AST at `NormalCompileRequestV1` construction time; and
- recovering identity by name, span, source/inventory ordinal, or numeric
  resolver values.

`VerifiedFinalCallableProgramSourceV1` alone owns the exact final Program,
complete callable inventory, final Program source brand, opaque declaration
identities, and transform lineage receipts. A bare AST can never regain this
authority.

Identity contract:

```text
CallableDeclarationAnchorV1
  issued only by parser or exact generator owner

CallableDeclarationIdV1
  final Program source brand + opaque anchor
```

The fields and constructors remain private. A body-only rewrite or reorder may
preserve an anchor only through an explicit transform receipt. Rename,
signature change, receiver-kind change, replacement, or a new generated
declaration receives a fresh anchor. Source sites and ordinals remain
diagnostic/placement data, never identity.

Complete membership and disposition:

- direct top-level, static Box, and instance Box declarations are supported;
- Main remains in the complete inventory and is excluded only by the typed
  normal selected projection;
- a selected BuildGate branch is supported only with its exact parser gate
  selection receipt; an unselected branch is absent;
- parser property/delegate and later macro derive/lift/test-generated rows are
  supported only with exact generator/transform receipts and fresh anchors;
- interface, record, sync, and declaration-only rows remain accounted typed
  unselected rows rather than disappearing from coverage;
- generic `CompatibilityOnly` provenance and arbitrary/public AST values never
  become semantic source authority;
- AST JSON and REPL paths become supported only after their decoder/rewriter
  consumes and reissues the source transaction; their present bare-AST forms
  are typed compatibility paths; and
- legacy raw-AST macro/normalize output is typed compatibility until its
  source-aware transform row lands.

Compatibility is selected before resolver or Builder effects. A caller never
tries source-backed issuance and then falls back to AST-only lowering.

Request and package contract:

`PreparedNormalDefaultProgramRootV1` ultimately owns the final source product;
the AST is an internal borrow, not a sibling field. Source-backed constructors
move that product atomically. Existing bare-AST constructors remain explicitly
typed compatibility entrances during migration and cannot call the semantic
issuer.

One later issuer co-seals:

```text
VerifiedNormalCallableSemanticPackageV1
  final callable Program source
  + same-module callable catalog
  + sole resolved semantic batch
  + exact selected mapping
```

Each catalog/batch/selection row retains the same opaque declaration ID. The
selected mapping seals declaration ID, `SelectedNormalCallableKeyV1`, catalog
brand, and a private batch slot. The slot is only a cache. Main and other
unselected rows receive explicit dispositions; missing, duplicate, or extra
selected rows reject before publication. No caller may pair an arbitrary
catalog and batch.

The only final consumer surface is scoped:

```text
package.with_projection(|projection| {
    projection.with_loan(key, |loan| { ... })
})
```

The forest, source projection, lowering input, and source ledger cannot escape
the callback. Dynamic target extension and root lowering complete within that
scope. The sole semantic batch remains the only forest/projection owner.

Authority target:

```text
post-transform complete callable source
  + sole resolved semantic batch
  + normal selected callable catalog
      -> exact normal callable selection projection
          -> callback-scoped source/lowering loans
```

Non-authority:

```text
AST shape or name lookup
source/inventory ordinal repair
numeric owner or BindingRef comparison across allocations
Builder catalog lineage as source truth
second resolver or cloned forest
fallback from unsupported source origins
```

Required Decision tests/guards:

- mixed top-level/static/instance exact coverage and order-independent mapping;
- Main exclusion and current Deferred/app behavior remain explicit;
- generated/build-gate/compat rows are supported by capability or fail fast;
- foreign parser/transform source, foreign catalog brand, missing relation,
  duplicate relation, and extra relation reject;
- lowering owner, BindingRefs, and forest are borrowed from the sole batch;
- old normal-source issuer remains live until cutover, then reaches caller zero;
- no production name lookup, ordinal repair, escaping loan, retry, or fallback;
- all source files stay below 760 preferred / 800 hard lines.

NoSafeSlice remains active if any of the complete production source carrier,
post-transform identity, complete membership, exact selection co-seal, or
callback-scoped consumer recut cannot be defined without creating a second
source/resolver authority.

### 3E follow-on order after the Decision

```text
PARSER-FINAL-CALLABLE-SOURCE-COVERAGE-R0
  -> CALLABLE-SOURCE-TRANSFORM-TRANSACTION-R0
  -> NORMAL-COMPILE-SOURCE-CARRIER-I0
  -> NORMAL-CALLABLE-SEMANTIC-SOURCE-SPLIT-R0
  -> NORMAL-CALLABLE-SEMANTIC-PACKAGE-I0
  -> NORMAL-CALLABLE-SEMANTIC-SOURCE-PROJECTION-R0
  -> JSON/REPL origin-specific source transaction cutovers
  -> old resolver/source authority caller-zero guard closeout
```

The parser row issues the complete initial inventory and callback loan for
top-level plus direct static/instance declarations, retains Main, and admits
generated/gate rows only through exact receipts. It adds no transform,
resolver, Builder, Home, Recipe, or production activation.

The transform row is a behavior-invariant Refactor Series over macro
derive/test-harness and normalize/lift owners. Every AST-changing pass consumes
and reissues the transaction; origin-specific REPL/JSON work may remain
separate bounded rows. The request row first switches standard named
source-backed callers and leaves bare-AST callers typed compatibility without
retry. The split row is behavior invariant. The semantic-package row is the
only catalog/batch/selection co-seal. The final projection row deletes
`VerifiedNormalCallableSemanticSourceV1::seal`, its forest/source projection
ownership, `function_at_site`, `view_for_key`, and resolver call, then converts
production consumers to scoped loans.

Required negative matrix:

```text
foreign final Program source brand
preserved anchor without transform receipt
missing / duplicate / extra declaration row
rename, signature, or receiver-kind change retaining an anchor
generated row without generator receipt
selected compatibility, interface, declaration-only, or Main row
gate row from an unselected branch
source-backed catalog paired with a foreign batch
duplicate private batch slot
AST-only request entering the semantic issuer
callback loan escape, split, or Clone
macro/normalize returning a bare AST on a source-backed route
second resolver, name lookup, ordinal repair, retry, or fallback
```

NoSafeSlice remains active for a production caller while any AST-changing pass
between parser and request returns a bare AST, any selected executable origin
lacks an exact source row, transform preservation/generation cannot be proven,
the request carries the AST separately, catalog pairing uses name/order, or a
consumer requires an escaping loan, second resolver, or fallback.

### 3E-R0. Parser initial callable source Refactor Series

`PARSER-FINAL-CALLABLE-SOURCE-COVERAGE-R0` is the parent closeout row.  A
premise audit found that implementing it as one post-hoc issuer over
`CompletedParserPostpassV1::ast()` would create a second source authority:
the attempted shape walked the final AST, issued anchors after parsing, paired
methods through the direct-only parameter catalog, and rejected gate/property/
delegate rows even when exact parser receipts already existed.  That shape is
rejected and must not land.

The parser source authority is instead recut as one behavior-invariant
Refactor Series.  Anchors originate only at the declaration or generator
transaction that creates the callable; later stages may preserve or select an
anchor only through an exact receipt.

```text
PARSER-CALLABLE-PARAMETER-SOURCE-PATH-IDENTITY-R0
  -> PARSER-SOURCE-SEAL-MODULE-SPLIT-R0
  -> PARSER-CALLABLE-DIRECT-ANCHOR-R0
  -> PARSER-CALLABLE-GATE-PROJECTION-R0
  -> PARSER-CALLABLE-GENERATED-ANCHOR-R0
  -> PARSER-INITIAL-CALLABLE-SOURCE-COSEAL-I0
  -> parent PARSER-FINAL-CALLABLE-SOURCE-COVERAGE-R0 closeout
```

#### `PARSER-CALLABLE-PARAMETER-SOURCE-PATH-IDENTITY-R0`

Status: **closed**.

Exact `SourceBoxMethodSiteV1` identity replaced the lossy ordinal pair;
distinct gate branches no longer collide and exact duplicates still reject.

#### `PARSER-SOURCE-SEAL-MODULE-SPLIT-R0`

Status: **closed**.

The near-limit source seal is now a behavior-invariant private directory
owner; focused files remain below the parser split threshold.

#### `PARSER-CALLABLE-DIRECT-ANCHOR-R0`

Status: **closed**.

Add one parser-private callable source session.  The original top-level
function parser and explicit static/instance method commit issue an opaque
`CallableDeclarationAnchorV1` together with the exact parser invocation and
structural source path. `Main.main` is an ordinary retained static Box method.
Selected-gate branch paths are recorded as written; this row neither selects a
branch nor publishes a verified program product.

The durable boundary is:

```text
parser invocation
  -> explicit declaration commit receipt
  -> fresh CallableDeclarationAnchorV1 at that same commit boundary
  -> PreparedDirectCallableSourceV1 {
       anchor,
       exact parser brand,
       structural Program declaration path,
       exact direct declaration kind,
       diagnostic declaration name,
     }
  -> one parser-private callable source session
```

The structural path is Program-wide.  Existing Box-local code may retain a
private compatibility alias while migrating, but a free/static callable row
must not expose `SourceBoxDeclarationPathV1` as its durable contract name.
Path coordinates prove placement and provenance; they do not define callable
identity.  Identity is only the parser-issued opaque anchor within the exact
parser invocation.

The session records these direct origins before any gate selection:

```text
free function
free static function
static Box method
ordinary instance Box method
```

`Main.main` is retained as an ordinary static Box method in this row.  Its
entrypoint role is a later selected projection and must not be reconstructed
here from the diagnostic spellings `Main` and `main`.

For Box methods, the direct issuer consumes data produced by the explicit
method commit receipt.  A generic `(path, kind, name)` constructor is not an
admitted caller surface.  Generated property/delegate/compatibility rows do
not possess that receipt and therefore cannot enter this session.  Callable
anchor issuance is independent of the optional parameter-source projection;
an explicit method without that sibling projection remains a valid direct
callable row.

The static-Box compatibility postpass does not authorize dropping `Main` or
static method rows from this parser-private session.  Conversely, this row
does not change postpass cohort classification or publish those rows to a
resolver.  The existing callable-parameter catalog may remain a migration
projection, but it is neither the source membership inventory nor an anchor
issuer.

Acceptance:

- a mixed direct source records top-level, `Main`, other static, and instance
  rows, with both `Main` and the other static method carrying the ordinary
  static-method origin;
- top-level/member gate children retain exact branch paths before selection;
- duplicate/foreign parser paths reject before session publication;
- anchor equality cannot be reconstructed from name, span, statement/member
  ordinal, inventory placement, AST pointer identity, or arity;
- no completed-AST walk, resolver, Builder, Home, Recipe, or production use.

Required negative coverage:

- a row from a foreign parser invocation is rejected;
- the same anchor cannot be committed twice;
- the same structural path cannot silently authorize two direct rows;
- equal names, spans, arities, and numeric source coordinates in distinct
  parser invocations still produce distinct anchors;
- generated/property/delegate/compatibility origins cannot enter the direct
  issuer through a catch-all arm.

Closeout receipt:

- one parser-private session issues fresh opaque anchors for free functions,
  free static functions, static Box methods, and ordinary instance methods;
- `Main.main` is retained as an ordinary static Box row with no by-name
  entrypoint classification;
- top-level and member-gate paths preserve both as-written branches before
  selection, including nested branch prefixes;
- one same-sink `DirectExplicitMethodSinkV1` commit receipt atomically retains
  the explicit declaration and its exact Program path, while optional
  parameter-source carriage remains a sibling projection;
- generated property/delegate rows cannot construct the direct receipt;
- focused tests cover five mixed rows across four direct kinds, generated
  exclusion, top-level/member/nested gate paths, foreign parser rejection,
  non-Clone anchor carriage, duplicate path rejection, and cross-parser opaque
  identity;
- parser source files remain below the 760-line split trigger, and the focused
  parser regressions plus `cargo check --lib` are green;
- gate selection, generated anchors, verified final Program publication,
  resolver, Builder, Home, Recipe, retry, fallback, and production activation
  remain zero.

#### `PARSER-CALLABLE-GATE-PROJECTION-R0`

Status: **closed**

The one-shot source-aware postpass now prunes direct callable rows with exact
top-level/member receipts, preserving nested paths without predicate replay.

#### `PARSER-CALLABLE-GENERATED-ANCHOR-R0`

Status: **closed**

Property/delegate owners now issue fresh anchors from exact source, placement,
relation, and selected-gate receipts. `MacroOrImport` and
`CompatibilityOnly` remain unsupported.

#### `PARSER-INITIAL-CALLABLE-SOURCE-COSEAL-I0`

Status: **closed**. Commit: `6191e0c45e`.

Change:
  The sole parser finalizer now moves the final Program and complete selected
  direct/property/delegate anchor set into non-Clone
  `VerifiedInitialCallableProgramSourceV1`.

Contract:
  Opaque anchors remain identity. Source-aware top-level slots, explicit
  commit placement, rebased member relations, gate receipts, and generator
  receipts are private placement evidence only. Exact syntax is lent through
  a repeatable higher-ranked callback; there is no Clone, split, or post-hoc
  anchor issuer.

Done:
  Mixed top-level/Main/static/instance, selected top/member gates, generated
  property/delegate, and repeatable syntax loans are green. Missing coverage,
  foreign parser slots, arbitrary AST, and `CompatibilityOnly` reject.
  `parser_initial_callable_program_source_guard.sh` owns the structural proof;
  every touched parser source remains below the 760-line split trigger.
  The focused row is 6/6 green. The broader parser filter is 306/315 green;
  its remaining nine failures are orthogonal pre-existing grammar/profile,
  BuildGate-signature, migration-transport, and rune-diagnostic expectations.

Stop:
  Resolver, Builder, Home, Recipe, retry, fallback, production activation,
  and `VerifiedFinalCallableProgramSourceV1` remain outside this row. The
  legacy parameter catalog is only a migration projection.

#### `PARSER-FINAL-CALLABLE-SOURCE-COVERAGE-R0`

Status: **closed**.

The parser callable-source Refactor Series now has one authority chain from
direct/gate/generated declaration transactions to the atomic initial Program
co-seal. The reusable direct, gate, generated, and initial guards are green;
`cargo check --lib`, the six focused co-seal tests, formatting, naming, and
diff checks are green. No additional parser prerequisite is admitted before
Dynamic ingress lifecycle resumes.

### 3F. Premise correction after parser closeout — accepted

The parser closeout exposed a skipped authority boundary. The previous
section 4 incorrectly allowed a caller to pair these independently-issued
products:

```text
VerifiedCallableParameterDemandCatalogV1
+ VerifiedDynamicOperatorCarrierLifecycleProgramV1
```

That API is withdrawn. The demand catalog borrows the neutral resolved batch,
while the only current Dynamic lifecycle fixture is derived through the old
Builder-owned normal semantic source and a second resolver allocation. Their
`FunctionOwnerIdV1` and `BindingRefV1` values therefore cannot match without
name, source-coordinate, ordinal, or numeric-ID repair. A final aggregate also
cannot own a batch while storing products that borrow that same batch.

This is not an ingress implementation bug. It is the consequence of advancing
the current pointer past the accepted 3E production source-carrier follow-on.
The correction keeps the parser series closed and restores the missing source
transport and semantic-package cutover as at most two bounded Refactor Series.

```text
PARSER-FINAL-CALLABLE-SOURCE-COVERAGE-R0
  -> NORMAL-CALLABLE-SOURCE-CARRIER-CUTOVER-R0
  -> NORMAL-CALLABLE-SEMANTIC-DYNAMIC-CUTOVER-I0
  -> DYNAMIC-CARRIER-INGRESS-LIFECYCLE-I0 closeout
  -> DYNAMIC-CARRIER-REBIND-TRANSACTION-I0
```

This compact order supersedes the seven fine-grained 3E follow-on row names.
It does not remove their contracts; it groups them by one production-cutover
purpose and prevents another unbounded prerequisite chain.

### 3G. `NORMAL-CALLABLE-SOURCE-CARRIER-CUTOVER-R0` — closed

Purpose: carry the final callable Program source through the real normal
compile request before any resolver or Builder effect.

One behavior-preserving series of 2--4 commits may:

1. introduce the source-aware transform transaction;
2. make `NormalCompileRequestV1` and
   `PreparedNormalDefaultProgramRootV1` atomically own the final callable
   source product rather than a sibling bare AST;
3. switch the named normal source-backed caller; and
4. delete that caller's bare-AST source reconstruction edge.

JSON and REPL remain typed compatibility origins and are parked. Selection is
made before effects; issuer failure never retries through compatibility.

Series ledger:

```text
named production caller:
  standard named source-backed NormalCompile request

new authority:
  VerifiedFinalCallableProgramSourceV1 transported atomically

old authority deleted at series end:
  selected caller's bare-AST source-backed ingress/reconstruction edge

fallback after commit:
  zero

distance to first production cutover:
  one remaining semantic/Dynamic cutover series
```

Acceptance:

- exact direct/gate/generated callable anchors survive only with transform
  receipts;
- foreign transform lineage, missing/duplicate/extra declaration rows, and
  bare-AST entry into the semantic issuer reject;
- the selected named caller switches without changing source behavior;
- JSON/REPL/all-origin coverage is not claimed; and
- every touched source remains below 760 preferred / 800 hard lines.

Closeout (2026-08-10):

- the callable-aware parser and total macro transform retain one final
  `VerifiedFinalCallableProgramSourceV1`, rejecting exact-source mutation
  rather than retrying through a bare AST;
- `NormalCompileRequestV1` and
  `PreparedNormalDefaultProgramRootV1` now carry that product atomically;
- `execute_mir_mode` is the named production caller and no longer performs
  `parse_source -> maybe_expand_and_dump -> for_mir_mode(ast)` on its
  source-backed arm;
- the explicit Parser/default-derive/MacroBox compatibility arm remains typed
  and does not act as issuer-failure fallback;
- `normal_callable_source_carrier_cutover_guard.sh`, focused parser/macro and
  request tests, and `cargo check --lib` are green; and
- the release named-caller probe
  `NYASH_MACRO_DISABLE=1 ./target/release/hakorune --backend mir
  lang/src/compiler/parser/scan/parser_scan_loop_box.hako` crosses the new
  source carrier and retains the expected next-row
  `UnknownTransientType { init: ValueId(4) }` fail-fast boundary; it does not
  retry through the removed source-backed AST edge; and
- minimal MIR JSON, JSON artifact, REPL, resolver semantics, Dynamic meaning,
  Home, Recipe, CFG, Completion, and physicalization remain unchanged.

### 3H. `NORMAL-CALLABLE-SEMANTIC-DYNAMIC-CUTOVER-I0` — current

Purpose: establish one resolver allocation and delete the competing Builder
resolver/source authority for the selected Dynamic production route.

#### Recovered premise correction after source-carrier cutover

`VerifiedFinalCallableProgramSourceV1` currently retains the final Program,
exact callable anchors, final slots, and transform lineage, but it does **not**
yet retain the parser-issued callable parameter-source catalog.  The named
`parse_normal_callable_program` path therefore cannot issue parameter demand
from the final source without either reopening parser authority or inferring
transfer from AST absence.  Both are forbidden.

The first responsibility boundary in this series is consequently:

```text
same Parser transaction
  -> final callable anchors / slots
  + exact callable parameter-source rows
  -> atomic final callable Program source
```

The parameter rows survive every exact callable-preserving transform together
with their callable anchor.  Missing, duplicate, foreign, or dropped rows
reject.  Compatibility origins remain typed compatibility and do not acquire
source authority.  Resolver code never reconstructs parameter transfer by
method name, source ordinal, raw parameter text, or the absence of `take`.

The final issuer consumes one whole final semantic package. It does not accept
caller-paired demand and lifecycle products:

```text
VerifiedFinalCallableProgramSourceV1
  -> sole VerifiedResolvedCallableSemanticBatchV1
  -> private callable-family admission
  -> private parameter-demand / Dynamic-source / Recipe / JoinSig projections
  -> owned co-sealed ingress rows
  -> VerifiedDynamicCarrierIngressLifecycleProgramV1
```

Because the current Dynamic source issuer returns one undifferentiated error,
batch-wide selection must first use a private typed disposition:

```text
DynamicFullBodySourceAttemptV1
  Candidate { private_batch_slot, verified_source }
  Declined
  Unresolved
  Rejected
```

Fully-observed shape mismatch is `Declined`; missing navigation/evidence is
`Unresolved`; foreign owner/binding/completion or duplicate identity is
`Rejected`. The package requires exactly one Candidate and zero Unresolved or
Rejected rows. Method or Box names never select the candidate. Selector shape
inside the verified body remains family semantics, not caller selection.

One Refactor Series of at most 5 commits owns the recovered delta:

1. split the near-limit old normal semantic source by responsibility;
2. co-seal and transform-carry exact callable parameter source in the final
   callable Program source;
3. issue the sole batch/package and exact anchor-to-batch-slot mapping;
4. recut Dynamic target/invocation evidence as catalog-neutral, batch-owned
   source relations;
5. replace/delete `VerifiedNormalCallableSemanticSourceV1::seal`, its second
   resolver authority, and caller pairing for the selected route while
   deriving parameter #1 Pos through initializer/local/V1/C0/L0/B0 and the
   exact JoinSig Enter as owned private ingress rows.

The first commit is behavior-preserving BoxShape work.  The 755-line
`normal_callable_semantic_source.rs` must be split before semantic behavior is
added.  `parser/mod.rs` (758 lines), `normal_callable_loop_handoff.rs` (749),
and `recursive_child_lowering.rs` (751) are no-addition surfaces; new owner
logic belongs in sibling modules.  No new prerequisite card is opened.

The final product owns the package/batch and owned relation rows. It never
stores self-referential borrowed catalogs. Scoped views may exist only during
issuance or a callback loan.

Acceptance includes the unchanged `ParserScanLoopBox.skip_while/4` source,
one resolver call, exact parameter #1 BindingRef equality across Pos, prelude,
V1/C0/B0 and JoinSig Enter, plus rejection of foreign package/batch/transform,
wrong initializer/local/carrier/Enter, missing/duplicate Dynamic Candidate,
Clone, split, and forged constructors. Series end requires the selected old
Builder resolver/source edge and caller pairing to be zero.

Structural closeout guards require the package issuer to precede the first
Builder/module effect, exactly zero production calls to
`VerifiedNormalCallableSemanticSourceV1::seal`, no
`extend_complete_dynamic_sources` input from the old source, and one whole
package input for ingress.  Borrowed lowering inputs, ledger views, demand
catalogs, target catalogs, and Dynamic envelope catalogs may exist only inside
issuer callbacks; the final package stores owned relations and is neither
Clone nor splittable.

Series checkpoint (2026-08-10): the final callable Program source now carries
the same parser transaction's direct-method parameter catalog through the
exact macro transform.  Initial co-seal validates parser brand, exact direct
method coverage, source coordinate, declaration kind, parameter coverage, and
AST syntax before the transform.  Selected member-gate Programs retain their
callable anchors with a typed unavailable parameter-source disposition rather
than a forged empty catalog.  The historical rich parameter-source API keeps
its stricter gate rejection, so this carriage neither narrows the normal
callable source lane nor broadens the old API.  Focused normal-source, macro,
and selected-gate tests are green.

The resolved callable semantic batch now consumes that final source directly.
It owns the final Program source plus ordered resolver forest/projection rows,
and calls `resolve_selected_callable_forests` once.  Its lowering-input and
declaration-semantic APIs remain higher-ranked callback loans.  The old
`RetainedParserCallableSemanticSourceV1` is no longer a semantic-batch input;
member-gate parameter unavailability rejects instead of reopening parser
authority.  Focused semantic-batch and parameter-demand tests, including the
unchanged four-method `ParserScanLoopBox`, are green.

The first owned package stage is also landed locally.  It consumes the final
source into the sole resolved batch, copies the complete parameter-demand
projection into owned private rows, applies an exhaustive private Dynamic
`Candidate`/`Declined`/`Unresolved`/`Rejected` classification to every batch
row, requires exactly one Candidate, and consumes that Candidate into the
existing V2 Recipe producer.  The unchanged `ParserScanLoopBox` seals four
declarations, fifteen Handle demands, and one Dynamic candidate without a
method-name or ordinal selector; zero and duplicate candidates reject.  The
package is non-Clone, has no parts projection, and stores no borrowed lowering
input or demand catalog.  Catalog-neutral invocation relations, JoinSig, and
the production edge replacement remain the next half of this same bounded
series; this checkpoint does not close the row.

#### Recovered all-callable coverage stop after the owned-package checkpoint

The checkpoint exposed one authority mismatch that must be closed before the
old Builder seal is deleted:

```text
VerifiedFinalCallableProgramSourceV1
  sources / slots:
    all exact callable anchors

VerifiedResolvedCallableSemanticBatchV1
  current membership driver:
    direct Box-method parameter-source rows only

old VerifiedSelectedNormalCallableSourceInventoryV1
  selected semantic membership:
    top-level functions + cataloged Box methods
```

Therefore the existing structural closeout requirement of globally zero
production calls to `VerifiedNormalCallableSemanticSourceV1::seal` is not yet
authorized. Deleting that call while the new batch cardinality is defined by
the parameter catalog could silently remove top-level callable semantic
authority from a mixed Program. The exact `ParserScanLoopBox` fixture alone
does not prove that this cannot happen.

Repository audit confirms that top-level callables and the Dynamic candidate
can coexist in one source-backed Program. The accepted correction is:

```text
final callable anchors / slots
  -> one all-callable semantic batch and resolver allocation

exact direct-method parameter-source rows
  -> exact partial projection onto matching batch slots

private Dynamic admission
  -> one exact selected row from that same batch
```

The final anchors/slots, not the parameter catalog, own whole-batch membership.
Every final callable is resolved exactly once. The parameter catalog joins
only the exact direct-method subset to issuer-private batch slots. Top-level,
gated, or generated rows without such source evidence receive no inferred
`Ordinary` demand. The selected Dynamic row must have every parameter demand
required by its ingress relation or issuance rejects before Builder/module
effect.

`source_row_index` in the current parameter catalog is not a complete callable
identity and must not be reused as the new batch identity. The co-seal uses an
exact final callable anchor plus an issuer-private batch slot. Missing,
duplicate, foreign, or extra callable mapping rejects; there is no legacy
fallback.

The next bounded correction therefore is:

1. lend complete callable syntax from the final source without exposing AST
   references outside a higher-ranked callback;
2. resolve every final callable anchor in one batch/allocation;
3. project direct-method parameter rows onto exact batch slots and remove the
   general batch-length equality assumption;
4. admit the Dynamic candidate and its owned parameter/ingress relations from
   the same batch; and
5. only then replace the production loan and delete the old seal.

The final cutover deletes these exact production edges together:

- the sole `VerifiedNormalCallableSemanticSourceV1::seal` call in
  `normal_default_root_catalog_lifecycle.rs`;
- `extend_complete_dynamic_sources(&source)` and the old Complete-source
  pairing in that owner; and
- the old source-backed `NormalCallableSemanticLoanPortV1` construction in
  `program_root_lowering.rs`.

Acceptance adds a mixed top-level plus `ParserScanLoopBox` fixture. Final
callable count must equal complete batch count; top-level lowering input must
be loanable from the batch; parameter-demand rows remain an exact Box-method
subset; and the Dynamic candidate remains tied to its exact anchor. Negative
coverage includes a missing selected top-level row, duplicate/foreign final
anchor, foreign partial-projection slot, inferred `Ordinary`, missing required
Dynamic demand, and missing/extra/duplicate selected-inventory mapping.

The structural guard requires zero old production seal, Dynamic extension,
and loan-port construction only after those mixed-coverage tests exist. It
also fixes one final-package batch issuance and forbids parameter-catalog
length as the general batch cardinality contract.

The already-landed direct-method package remains a valid bounded checkpoint,
not a production replacement. Code may now resume with the complete-batch
coverage correction; it must not widen the checkpoint by inference.

Non-claims: Home inference, rebind, CFG, Completion, physicalization,
JSON/REPL/all-origin cutover, provider activation, retry, and fallback.

### 4. `DYNAMIC-CARRIER-INGRESS-LIFECYCLE-I0` — absorbed closeout

The ingress semantic row is no longer a standalone prerequisite. It closes as
the final commit of `NORMAL-CALLABLE-SEMANTIC-DYNAMIC-CUTOVER-I0`, after the
whole package owns the sole batch and derives all private relations internally.
No public API may recreate the withdrawn catalog-plus-lifecycle pairing.

### 5. First-production cutover order

```text
DYNAMIC-CARRIER-REBIND-TRANSACTION-I0
-> DYNAMIC-CARRIER-FLOW-D0/I0
-> cleanup projection / Completion / exit transaction
-> physicalization
-> full skip_while/4 unpublished-session canary
-> named production caller switch
-> selected old Dynamic route deletion; retry/fallback = 0
```

The initial co-seal is the fixed terminal of the accepted parser prerequisite
series. After its parent closeout, infrastructure work is selected only when
the unchanged `skip_while/4` production gate fails at that exact owner.

## Parked independent cleanup

- `CURRENT-POINTER-CROSSFIELD-CONSISTENCY-R0`: strengthen the existing guard
  and table-driven fixtures. `10-Now.md` is already reduced to a field-name
  mirror; the current pointer values are not copied there.
- `DYNAMIC-FAULT-CATALOG-EXHAUSTIVE-R0`: exhaustive operation classification
  is already closed. Only typed reject preservation and caller-zero
  visibility narrowing remain.
- recursive JoinSig topology admission, both-arm exit generalization,
  multi-root-carrier After closure, and module relocation are P2 and must not
  enter the parameter/ingress series.

## Stop / non-claims

Stop at `NoSafeSlice` if the parameter transfer seal, common demand catalog,
or exact JoinSig Enter relation cannot be issued from current source authority.

```text
no explicit Take/owned ingress activation
no Dynamic Home classification
no Home Flow or cleanup execution
no rebind or displaced-current token in the ingress rows
no CFG / SSA / PHI / ValueId
no Completion / Return / DraftSeal
no runtime/provider/physical ABI
no production activation, retry, or fallback
```

Implementation files must be responsibility-split before 760 lines and stay
below the 800-line hard limit. Tests live outside owner files.
