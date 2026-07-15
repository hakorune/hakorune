---
Status: Accepted and taskized — A′+; MP0-TX0 closed, P0c-B1 next
Date: 2026-07-15
Decision: A′+ — Program/catalog co-seal + single-use resolver continuation + canonical-key module identity
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-P0C-B1-EXACT-SIBLING-CALL-001
Parent taskboard: mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
Previous closed card: mirbuilder-ssa-i1-compat-p0c-i64-call-design-stop-2026-07-15.md
Decision-input baseline commit: ec570ea696
Next code-facing row: P0c-B1
---

# P0c-CAT0 Module Callable Catalog — A′+ Decision and Taskboard

## Decision lock

P0c-I1 now executes one exact static current-owner self call. Its target is a
generic `ResolvedCallableRefV1`, resolved before Builder effects through a
source-unit-owned one-entry `VerifiedCallableIndexV1`. Lower consumes one
co-sealed direct-call row and performs no source-name lookup or fallback.

The catalog prerequisite for sibling calls is fixed as **A′+**:

> Seal one immutable multi-header callable catalog together with its exact
> function-only `ASTNode::Program`; keep every top-level function as its own
> single-root owner forest; return one opaque single-use resolver continuation
> for MP0; key the normalized resolved module by `CanonicalCallableKeyV1`; and
> defer all body resolution and unpublished MIR draft publication to MP0.

A′+ adds three contracts to the former A′ working hypothesis:

1. Program syntax and catalog are co-sealed so foreign source/catalog pairing
   is unrepresentable.
2. CAT0 returns a non-cloneable, single-use continuation containing both the
   next `FunctionOriginV1` ordinal and the same-branded owner issuer. MP0
   consumes it when nested lambda owners are issued.
3. `CanonicalCallableKeyV1` is the normalized module map key;
   `ResolvedCallableRefV1` remains invocation-local callable membership and
   call-target identity.

CAT0-D0 is closed by this decision. The next row is the behavior-neutral
CAT0-L0 index storage cleanup. MethodCall, receiver, Box ownership, imports,
separate compilation, backend widening, and sibling execution remain outside
CAT0.

## Current closed boundary

The following is production green at `ec570ea696`:

```text
source unit:
  one FunctionDeclaration root

callable index:
  exactly one exact static all-i64 header

call target:
  generic ResolvedCallableRefV1(FunctionOwnerIdV1)

call site:
  exactly one FunctionCall
  exact current-owner name and arity
  exact i64 arguments -> i64 result

Lower:
  whole co-sealed VerifiedTrivialDirectCallV1 only
  raw FunctionCall.name reads = 0
  legacy call resolver calls = 0
  fallback/retry = 0

runtime:
  Rust MIR interpreter only
  parameter and return contracts checked on every recursive frame

ownership:
  CopyOwned / DestroyOwned / ReleaseStrong = 0
```

The current one-entry schema already separates:

```text
CanonicalCallableKeyV1:
  source namespace + exact name + arity

ResolvedCallableRefV1:
  typed reference to invocation-local FunctionOwnerIdV1

CanonicalCallableSymbolV1:
  one-way MIR/backend physical spelling

ExactTrivialCallableSignatureV1:
  exact i64 parameter/result ABI
```

## Current structural evidence

### Source syntax

The parser produces:

```rust
ASTNode::Program {
    statements: Vec<ASTNode>,
    ...
}
```

The current canonical ingress instead owns one bare `FunctionDeclaration`.
`VerifiedSourceProjectionV1::seal` rejects any other root and
`root_function_input()` assumes exactly one forest root.

For the first module catalog, the only exact free-static source family visible
without reconstructing source identity is:

```text
ASTNode::Program
  every top-level statement is FunctionDeclaration
```

Silently filtering other Program statements would make coverage incomplete.
Using `BoxDeclaration.methods` is also unsuitable: it is a `HashMap`, may have
already lost duplicate declaration evidence, and belongs to a different
callable namespace.

### Semantic owner topology

`VerifiedSemanticOwnerForestV1` intentionally represents one root function
plus nested Lambda owners and rejects multiple roots. Its normalized graph has
one root identity. `FunctionOriginV1` already has a function ordinal, and one
resolver session can issue multiple invocation-local `FunctionOwnerIdV1`
values under the same compilation brand.

The clean module shape therefore appears to be:

```text
module
  catalog: one source-unit authority
  function A: one single-root owner forest
  function B: one single-root owner forest
  ...
```

not one widened multi-root `VerifiedSemanticOwnerForestV1`.

### Publication

`CanonicalModuleLoweringSessionV1` already protects the caller from partial
external publication: it builds in a fresh candidate Builder and commits only
after whole-module finish and verification.

`CanonicalFunctionLoweringSessionV1`, however, publishes each successful
function immediately into that candidate module. A later failure discards the
candidate externally, but earlier drafts are visible to later Lower steps and
later drafts are not. This is rollback-safe but does not prove internal
declaration-order independence.

Self-call already proves that canonical Lower need not query the MIR module
table: a catalog-derived `Callee::Global` can be emitted before the target body
is installed. The runtime resolves calls only after the completed module is
available.

## Proposed A′ product split

### Module-relative declaration site

Add a source-unit site distinct from function-relative expression sites:

```rust
pub struct SourceCallableDeclarationSiteV1 {
    statement_index: u32,
}
```

It is structural provenance into the exact Program root. Span, pointer, name,
encounter-order recovery, and physical symbol parsing are forbidden.

### Bounded header view

```rust
pub struct CallableModuleHeaderSyntaxViewV1<'a> {
    // borrowed exact Program statements
}
```

It yields located declaration rows only when every Program statement is a
`FunctionDeclaration`. It never traverses function bodies.

### Program/catalog co-seal

```rust
pub struct VerifiedCallableDeclarationV1 {
    site: SourceCallableDeclarationSiteV1,
    origin: FunctionOriginV1,
    callable: ResolvedCallableRefV1,
}

pub struct VerifiedCallableCatalogV1 {
    index: VerifiedCallableIndexV1,
    declarations_by_site:
        BTreeMap<SourceCallableDeclarationSiteV1,
                 VerifiedCallableDeclarationV1>,
}

pub struct VerifiedCallableCatalogSourceUnitV1 {
    syntax: CanonicalProgramSyntaxOwnerV1,
    catalog: VerifiedCallableCatalogV1,
    _seal: CallableCatalogSourceUnitSealV1,
}
```

The index remains the sole header/name-resolution authority. Declaration rows
prove exact source membership only; they do not duplicate the signature or
symbol. `VerifiedCallableCatalogSourceUnitV1` owns the exact Program and
catalog together; CAT0 must not expose a constructor that accepts those two
products independently.

The index should own each `VerifiedCallableHeaderV1` once. Derived reverse maps
may store only a key/index foreign reference:

```text
source key -> primary header
callable ref -> source key/index
physical symbol -> source key/index
```

Do not copy complete headers into three maps. Replace the current O(n)
`header_for_callable` scan with an exact reverse index, and replace the
panic-based `only_header()` with a checked P0c sole-header facade.

### Single-use resolver continuation

The current resolver session owns two pieces of issuance state:

```text
next FunctionOriginV1 function ordinal
same-compilation-brand FunctionOwnerIssuerV1 and its next slot
```

Both must survive catalog sealing because MP0 may discover nested lambdas in
function bodies. Preserving only the owner slot/brand is insufficient: the
lambda also needs a correctly issued `FunctionOriginV1`.

```rust
pub(crate) struct CatalogSealedResolverContinuationV1 {
    next_function_ordinal: u32,
    owner_issuer: FunctionOwnerIssuerV1,
}

pub(crate) struct CallableCatalogSealOutcomeV1 {
    source_unit: VerifiedCallableCatalogSourceUnitV1,
    continuation: CatalogSealedResolverContinuationV1,
}
```

The continuation is opaque, non-`Clone`, single-use, and consumed only by
MP0. The catalog itself remains immutable. A typestate resolver session may be
used instead if it enforces the same contract.

### Seal order

```text
1. validate the complete exact function-only Program surface
2. validate every owner-free header candidate
3. reject duplicate keys/profile failures and deterministic symbol conflicts
4. issue every top-level `(FunctionOriginV1, FunctionOwnerIdV1)` from one session
5. attach owners and build primary headers plus foreign-key reverse indexes
6. verify key/ref/site/symbol cardinality, bijection, and compilation brand
7. co-seal the immutable Program/catalog source unit
8. return the single-use resolver continuation
9. only later, in MP0, analyze any function body
```

Raw owner slots may change when declarations are reordered; they are
invocation-local membership brands, not normalized source identity. Reordering
must not change the normalized `(key, signature, symbol)` rows or lookup
results.

## Exact CAT0 key and duplicate law

```text
source key:
  (CallableNamespaceV1, exact case-sensitive source name, source arity)

bare FunctionCall namespace:
  FreeStatic only

same name, different arity:
  allowed

same exact key:
  reject even when signature/body differs

type-directed overload:
  absent in V1

nearest name/arity or namespace-priority search:
  forbidden

physical slash spelling in source:
  reject
```

Source-reachable typed seal errors include:

```text
EmptyCatalog
StatementIndexOverflow
UnsupportedProgramStatement { site, actual }
DuplicateSourceKey { key, first_site, second_site }
HeaderOutsideExactI64Profile { site, reason }
OwnerIssueExhausted { site, reason }
```

Malformed-draft invariant tests own errors that exact source construction
cannot normally reach:

```text
DuplicateCallableIdentity
DuplicateDeclarationSite
PhysicalSymbolCollision
MixedCompilationBrand
CatalogCardinalityMismatch
```

Any error publishes no partial catalog. In the initial `FreeStatic` namespace,
the exact `name/arity` symbol law normally turns a physical symbol collision
into `DuplicateSourceKey` first; the dedicated collision error remains an
internal invariant for future namespace/symbol policies.

## CAT0/MP0 responsibility boundary

### CAT0 may own

```text
exact Program header inventory
module-relative declaration sites
pre-issued callable owner membership
immutable multi-header catalog
exact lookup and reverse membership
duplicate/collision rejection
one-entry P0c compatibility parity
```

### CAT0 must not own

```text
function body resolution
call-site target rows
per-function source projection
Binding SSA/profile analysis
sibling call materialization
MIR function drafts
module publication
backend capability rows
runtime execution
mutual-recursion effect/SCC analysis
```

### MP0 follow-on

The likely MP0 product is:

```rust
pub struct VerifiedResolvedCallableModuleV1 {
    source: VerifiedCallableCatalogSourceUnitV1,
    functions_by_key:
        BTreeMap<CanonicalCallableKeyV1,
                 VerifiedResolvedFunctionUnitV1>,
}

pub struct VerifiedResolvedFunctionUnitV1 {
    declaration_site: SourceCallableDeclarationSiteV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
}
```

At seal time, MP0 proves:

```text
catalog[key].callable.owner
  ==
functions_by_key[key].forest.root
```

`ResolvedCallableRefV1` remains the exact invocation-local target recorded at
call sites. It is not promoted into normalized source identity.

MP0 would use two passes:

```text
pass A:
  all owners and headers sealed into CAT0 catalog

pass B:
  every body resolved against the complete catalog
  self/forward/backward targets become exact refs
```

Whole-module preflight must finish for every function before Builder effects.
For strict internal order independence, MP0 should collect unpublished verified
function drafts and batch-publish them after catalog/body/draft cardinality is
sealed. The existing outer module candidate remains the external rollback
boundary.

## Alternatives considered

### A′ — function-only Program catalog, body-free CAT0

Accepted only with the three A′+ contracts above. It creates the missing source
authority while preserving one-root function forests and leaving publication
to MP0.

### B — generic explicit header list, Program decision deferred

CAT0 would expose only:

```rust
seal_catalog((owner, CallableHeaderSyntaxViewV1)*)
```

This is smaller, but risks landing another disconnected substrate without
fixing which exact source unit owns sibling declarations. Select B only if the
Program surface is not yet a trustworthy canonical source authority.

### C — catalog plus all resolved bodies in CAT0

This would seal the full module semantic product immediately. It reduces the
number of named rows but mixes header authority, multi-root source projection,
body resolution, forward references, and module verification. We currently
recommend rejecting C as too broad.

### D — catalog plus MIR batch publication in one atomic row

This proves execution sooner, but mixes CAT0, MP0, sibling-call activation,
and publication transactions. We recommend rejecting D.

## Closed decisions

1. A′+ is selected.
2. Exact function-only `ASTNode::Program` is the first CAT0 free-static source
   authority; unsupported statements reject the whole surface.
3. `VerifiedSemanticOwnerForestV1` remains single-root. The resolved module
   owns one forest per top-level function.
4. All owner-free headers are validated first; every top-level origin/owner is
   then reserved before any body is read.
5. CAT0 remains header-only. Body resolution and source projection belong to
   MP0.
6. MP0 must collect, verify, and batch-insert unpublished function drafts
   before internal declaration-order independence may be claimed.

## Canonical task order

```text
CAT0-D0:
  CLOSED
  A′+ decision lock and exact claims/non-claims

CAT0-L0:
  CLOSED — behavior-neutral code row
  behavior-neutral index storage cleanup
  one primary header store + reverse foreign-key indexes
  checked sole-header P0c facade
  add ordering only where required for exact map keys

CAT0-S0:
  CLOSED — code row
  CallableModuleHeaderSyntaxViewV1
  SourceCallableDeclarationSiteV1
  Program-owned VerifiedCallableHeaderSourceUnitV1 shell
  function-only whole-surface coverage; body reads = 0

CAT0-C0a:
  CLOSED — code row
  owner-free header candidate validation
  duplicate/profile/symbol rejection before identity issuance

CAT0-C0b:
  CLOSED — code row
  reserve all top-level origin/owner pairs from one session
  immutable Program/catalog co-seal
  opaque non-Clone single-use continuation preserving
  next function ordinal + same-branded owner issuer

CAT0-G0:
  CLOSED — fixture/generated-guard row
  fixture/generated-guard row
  fixtures, normalized reorder parity, authority/caller-zero guards
  foreign Program/catalog pairing impossible
  source errors separated from malformed-draft invariant tests
  body/MIR/production activation = 0

MP0-S0:
  CLOSED — passive code/guard row
  exact multi-function source-unit carrier
  primary functions_by_key map

MP0-R0:
  CLOSED — code/fixture row
  two-pass per-function body/forest resolution against complete catalog
  consume resolver continuation for same-brand lambda owners

MP0-P0:
  CLOSED — code/fixture/guard row
  whole-module preflight before Builder effects

MP0-TX0:
  code row
  unpublished function draft set and batch publication
  exact catalog/draft key, symbol, and cardinality verification

P0c-B1:
  first production sibling-call activation
  exactly two static exact-i64 functions and one sibling edge

P0c-MR:
  separate later decision
  mutual recursion / callable SCC as a separate row
```

Each implementation row must contain code or generated artifact delta. CAT0
does not activate sibling calls merely because multiple headers exist. Do not
open MP0 until CAT0-G0 is green; do not open P0c-B1 until MP0-TX0 is green.

### CAT0-L0 exit gate

```text
structure:
  one headers_by_key primary store
  callable/symbol reverse maps store keys only
  header_for_callable is indexed, not O(n)
  sole_header returns Result and never panics

compatibility:
  seal_one behavior and one-entry P0c fixtures unchanged
  resolved callable forest uses the checked facade

non-delta:
  Program grammar 0
  multi-header seal 0
  body authority 0
  production sibling calls 0
  runtime/backend/ownership delta 0

verification:
  focused resolved_semantics callable-index tests
  resolved callable authority guard
  cargo build --release --bin hakorune
  bash tools/checks/dev_gate.sh quick
  all touched source/check files < 800 lines
```

State: closed on 2026-07-15.

Closeout evidence:

```text
VerifiedCallableIndexV1 owns one headers_by_key primary store
key_by_callable / key_by_symbol store canonical keys only
callable and symbol reverse lookups are indexed
sole_header returns typed cardinality error and never panics
resolved callable forest consumes the checked facade
legacy only_header and linear header scan callers = 0
resolved semantics = 129/129
release build = green
quick gate = 66/66
all touched source/check files < 800 lines
```

Grammar, Program admission, multi-header sealing, body authority, sibling-call
activation, runtime/backend behavior, and ownership operations remain zero.
The next code-facing row is `CAT0-C0a`.

### CAT0-S0 exit gate

```text
source ownership:
  one owned non-empty ASTNode::Program
  opaque u32 declaration sites derived from statement order
  callers cannot supply Program and site inventory independently

surface:
  every top-level statement is FunctionDeclaration
  unsupported top-level statements reject the whole source unit
  only body-free CallableHeaderSyntaxViewV1 rows are exposed
  function bodies remain unread, even when they contain unsupported nodes

non-delta:
  exact-i64 header/profile validation = 0
  owner/origin issuance = 0
  multi-header callable index seal = 0
  body resolution / sibling call / Builder / Lower / runtime = 0

verification:
  focused source-unit fixtures = 4/4
  resolved semantics = 133/133
  callable authority guard = green
  release build = green
  quick gate = 66/66
  all touched source/check files < 800 lines
```

State: closed on 2026-07-15. The next code-facing row is `CAT0-C0a`.

### CAT0-C0a exit gate

```text
candidate authority:
  one declaration-site-keyed owner-free candidate primary store
  callable key and physical symbol reverse maps store declaration sites only
  exact static non-main all-i64 profile is validated before owner issuance

duplicate law:
  same exact name/arity rejects with first and second declaration sites
  same name with different arity remains legal
  deterministic symbol collisions reject before publication

non-delta:
  FunctionOriginV1 / FunctionOwnerIdV1 issuance = 0
  immutable owned callable catalog = 0
  resolver continuation / body resolution / sibling call = 0
  Builder / Lower / runtime / grammar / ownership behavior = 0

verification:
  focused owner-free candidate fixtures = 4/4
  resolved semantics = 137/137
  callable authority guard = green
  release build = green
  quick gate = 66/66
  all touched source/check files < 800 lines
```

State: closed on 2026-07-15. The next code-facing row is `CAT0-C0b`.

### CAT0-C0b exit gate

```text
owner reservation:
  every top-level FunctionOriginV1 / FunctionOwnerIdV1 pair is issued in
  declaration order from one FunctionSemanticResolverSessionV1
  all top-level owners share one compilation brand

co-seal:
  the S0 shell is named VerifiedCallableHeaderSourceUnitV1
  VerifiedCallableCatalogSourceUnitV1 owns that exact Program plus catalog
  the detached VerifiedCallableCatalogV1 is non-Clone
  key/callable/symbol cardinality and reverse lookup are exact

continuation:
  CatalogSealedResolverContinuationV1 is opaque and non-Clone
  consuming it resumes at the next function ordinal and same-branded owner slot

non-delta:
  body traversal / owner forest / source projection = 0
  production catalog callers / sibling calls = 0
  Builder / Lower / MIR / runtime / backend / ownership delta = 0

verification:
  focused C0b fixtures = 4/4
  resolved semantics = 141/141
  resolved callable authority guard = green
  release build = green
  quick gate = 66/66
  all touched source/check files < 800 lines
```

State: closed on 2026-07-15. The next code-facing row was `CAT0-G0`, which is
closed by the following exit gate.

### CAT0-G0 exit gate

```text
normalization:
  NormalizedCallableCatalogV1 contains only
  namespace / exact name / arity / exact signature / physical symbol
  declaration site / FunctionOriginV1 / FunctionOwnerIdV1 / owner slot /
  compilation brand / ResolvedCallableRefV1 are excluded

parity:
  declaration reorder and a different compilation brand normalize equally
  exact source lookup returns the same key/signature/symbol meaning

invariants:
  source-reachable duplicate/profile errors stay in C0a fixtures
  duplicate callable identity and symbol collision use malformed private draft
  foreign Program/catalog construction remains unavailable
  normalized product production callers = 0

non-delta:
  body authority / owner forest / source projection = 0
  Builder / Lower / MIR / runtime / backend / ownership delta = 0

verification:
  focused G0 fixtures = 3/3
  resolved semantics = 144/144
  resolved callable authority guard = green
  release build = green
  quick gate = 66/66
  all touched source/check files < 800 lines
```

State: closed on 2026-07-15. CAT0 is green through G0; the next code-facing
row is `MP0-S0`.

### MP0-S0 exit gate

```text
structure:
  VerifiedResolvedCallableModuleV1 owns the exact CAT0 source unit once
  functions_by_key is the sole primary function-unit map
  the map key is CanonicalCallableKeyV1
  each VerifiedResolvedFunctionUnitV1 keeps declaration site, one single-root
  semantic forest, and one exact source projection together

authority:
  the carrier lives in the compiler/source-projection layer
  resolved_semantics does not import compiler projection authority
  ResolvedCallableRefV1 is not a normalized/module primary key

passive boundary:
  constructor / resolver continuation consumption = 0
  body traversal / direct-call target resolution = 0
  Builder / Lower / MIR draft / module publication = 0
  production producer / consumer = 0

verification:
  passive compile fixtures = 2/2
  resolved semantics = 144/144
  resolved callable authority guard = green
  release build = green
  quick gate = 66/66
  all touched source/check files < 800 lines
```

State: closed on 2026-07-15. The passive carrier cannot be assembled through
foreign parts; `MP0-R0` is the sole next producer and must consume the CAT0
resolver continuation while resolving every body against the complete catalog.

### MP0-R0 exit gate

```text
source boundary:
  CAT0 source unit is consumed into one bounded body-resolution view
  Program syntax and immutable catalog remain inseparable through finish
  CAT0 callable_catalog.rs remains header-only; body opening is physically
  isolated in callable_catalog_resolution_source.rs

resolution:
  every top-level function reuses its CAT0-reserved origin and owner
  every body resolves against the complete immutable callable index
  self / forward / backward target rows carry exact ResolvedCallableRefV1
  nested Lambda owners consume the single continuation and keep the catalog
  compilation brand
  one exact source projection is sealed beside each single-root forest

publication boundary:
  VerifiedResolvedCallableModuleV1::resolve is the sole carrier constructor
  unknown target or any late function error returns no resolved module
  Builder / Lower / MIR draft / module insertion = 0
  production sibling-call activation = 0

verification:
  focused module-resolution fixtures = 6/6
  resolved semantics = 144/144
  resolved callable authority guard = green
  release build = green
  quick gate = 66/66
  all touched source/check files < 800 lines
```

State: closed on 2026-07-15. The next code-facing row is `MP0-P0`, which must
run whole-module capability/profile preflight over this resolved carrier before
any Builder effect.

### MP0-P0 exit gate

```text
input:
  every CanonicalCallableKeyV1 obtains one exact function input from the same
  Program/catalog-owned VerifiedResolvedCallableModuleV1
  FunctionSourceViewV1 is assembled from the already sealed header/body parts
  complete immutable callable index is reused for every function

preflight:
  existing function-level CanonicalLoweringPreflightV1 remains the sole
  capability/profile admission authority
  every function plan is accumulated in a temporary canonical-keyed map
  the module preflight product is returned only after exact cardinality match
  a late function failure returns no partial module preflight product

proof:
  all-function exact-i64 call-free module = pass
  declaration reorder preserves the canonical plan key set
  invalid later function leaves module product publication at 0
  focused preflight fixtures = 3/3
  resolved callable module fixtures including preflight = 9/9
  capability fixtures = 5/5
  resolved semantics debug suite = 144/144
  generated resolved-callable authority guard = green
  release build = green
  quick gate = 66/66
  all touched source/check files < 800 lines

non-delta:
  Builder / MIR draft / MirFunction / MirModule publication = 0
  runtime / backend / ownership operation delta = 0
  production sibling-call activation = 0
  direct-call profile remains the existing exact one-entry self-call profile
```

State: closed on 2026-07-15. The next code-facing row is `MP0-TX0`, which must
keep every function draft unpublished through individual verification and
batch-publish only after exact catalog/draft correspondence is sealed.

### MP0-TX0 exit gate

```text
resolution:
  every body resolves against the complete immutable catalog
  every function has one canonical-keyed single-root forest/projection
  same-session lambda issuance consumes the resolver continuation once

transaction:
  all function drafts remain unpublished through individual verification
  catalog/draft key, symbol, and cardinality correspondence is exact
  missing, extra, or duplicate drafts reject
  any late draft failure leaves candidate-module function publication at 0
  whole-module preflight completes before Builder effects

proof:
  declaration reorder parity
  legacy lookup and fallback callers = 0
  generated atomic-publication guard is mandatory in TX0
```

State: closed on 2026-07-15. Every function now lowers through a restored
function session into an unpublished `MirFunction`; no earlier successful
draft is inserted while a later draft is lowering. One canonical-keyed draft
set verifies catalog/header lookup, physical symbol, signature arity, unique
draft key/symbol, and exact catalog/function/plan/draft cardinality before one
`MirModule::try_add_functions_atomic` call. Batch publication prechecks both
existing-module and within-batch collisions before mutation.

```text
proof:
  actual two-function call-free module publication = pass
  declaration reorder preserves published symbol set
  injected late second-draft failure leaves function publication at 0
  injected symbol drift leaves function publication at 0
  batch duplicate and late existing-module collision preserve prior module
  resolved-lowering debug suite = 79/79
  transaction release fixtures = 4/4
  generated resolved-callable authority guard = green
  production transaction callers = 0
  incremental transaction publication sites = 0
  atomic batch publication sites = 1
  all touched source/check files < 800 lines

non-delta:
  sibling source-call admission = 0
  MethodCall / receiver / ownership operations = 0
  runtime / backend capability widening = 0
  fallback / retry = 0
```

The next code-facing row is `P0c-B1`: exactly two static exact-i64 functions
and one sibling edge, consuming this already-closed transaction without
widening the module or ownership model.

### P0c-B1 exit gate

```text
accepted source:
  exactly two static exact-i64 functions
  exactly one sibling FunctionCall edge

proof:
  forward/backward declaration order parity
  exact target, argument, and result ABI
  direct-call coverage claimed exactly once
  Rust MIR interpreter capability only
  unsupported backend effects = 0

non-claims:
  multiple/nested calls = 0
  mutual recursion = 0
  MethodCall/receiver = 0
  ownership operations = 0
  fallback/retry = 0
```

### Task-lane separation

```text
active now:
  CAT0 -> MP0 -> P0c-B1

parked and independent:
  Ownership V2 / move-share parser-resolver-MIR work
  .hako selfhost parser/MIR-builder migration
  MethodCall/receiver/Box return ABI
```

CAT0 consumes the existing Rust AST `Program`; it does not add `.hako`
ownership grammar. Ownership parser and MIR-builder work remains visible in
its own parked taskboard and must not be interleaved with this callable-catalog
series.

## Required CAT0 fixtures

Pass:

```text
one-entry catalog preserves P0c behavior
two and three exact i64 headers seal
foo/1 and foo/2 coexist
forward/backward/self header lookup
declaration reorder keeps normalized key/signature/symbol rows equal
callable reverse lookup reaches the primary header
symbol reverse lookup reaches the primary header
all headers seal before any body-analysis probe
```

Reject:

```text
empty Program
non-FunctionDeclaration top-level statement
duplicate exact key
duplicate owner ref
duplicate declaration site
physical symbol collision
mixed compilation brand
slash spelling
malformed/non-i64 header
foreign catalog/syntax pairing
partial publication after a late header error
```

## Non-authorities

```text
MirModule.functions
builder declaration/static-method indexes
legacy global/static resolver
unique-name or method-tail recovery
global_call_route_plan / same-module MIR scans
BoxCallableRegistry / plugin/type ABI catalogs
runtime VM/PyVM function tables
physical symbol parsers
BoxDeclaration.methods HashMap
```

## Stop conditions

Stop CAT0 if any implementation requires:

1. body analysis before all headers are sealed;
2. a mutable catalog during body resolution;
3. per-function copies of the callable catalog;
4. a multi-root rewrite of `VerifiedSemanticOwnerForestV1`;
5. MIR module symbols or legacy indexes as source declaration authority;
6. name-tail, nearest-arity, namespace-priority, or type-directed fallback;
7. physical symbols parsed back into semantic identity;
8. silent filtering of unsupported Program statements;
9. declaration-order first-wins/last-wins behavior;
10. sibling-call materialization or runtime activation in CAT0;
11. incremental MIR publication claimed as internal order independence;
12. MethodCall/receiver/import/plugin/Box ownership/backend widening;
13. any modified source/check file exceeding 800 lines.

## Final lock

> CAT0 consumes and owns one exact function-only `ASTNode::Program`, validates
> every top-level header without body authority, reserves every top-level
> origin/owner in one compilation session, and co-seals one immutable catalog
> plus a single-use resolver continuation. Each function keeps one single-root
> semantic owner forest. MP0 resolves every body against the complete catalog,
> verifies the whole module, collects all MIR drafts unpublished, and
> batch-publishes only after exact catalog/draft correspondence is sealed.
