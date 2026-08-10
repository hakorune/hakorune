# Dynamic carrier ingress lifecycle

Status: ingress and parameter-transfer authority Decisions accepted; Hako R0a/R0b closed
Date: 2026-08-10
Parent: `DYNAMIC-CARRIER-REBIND-TRANSACTION-D0`
Current implementation row: `DYNAMIC-CARRIER-INGRESS-LIFECYCLE-I0`
Exception: T2 source-authority boundary required before several implementation rows.
ParentCurrentCard: this file is the rolling card for parameter demand through carrier ingress.

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

### 4. `DYNAMIC-CARRIER-INGRESS-LIFECYCLE-I0` — selected

Consume the whole parameter-demand catalog and whole Dynamic lifecycle
program. Seal parameter #1 through Pos/initializer/local/V1/C0/L0/B0 and the
exact JoinSig Enter payload. Publish one private borrowed ingress row.

Required negatives include wrong initializer BindingRef, local binding,
Recipe input, carrier owner/binding/class/entry, missing or duplicate Enter,
extra root carrier, caller-selected disposition, Clone, and split API.

### 5. Follow-on order

```text
DYNAMIC-CARRIER-REBIND-TRANSACTION-I0
-> DYNAMIC-CARRIER-FLOW-D0/I0
-> cleanup projection / Completion / exit transaction
-> physicalization
```

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
