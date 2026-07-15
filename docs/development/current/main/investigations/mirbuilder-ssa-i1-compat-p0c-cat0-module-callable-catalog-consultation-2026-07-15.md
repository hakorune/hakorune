---
Status: Consultation packet — answer pending
Date: 2026-07-15
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-P0C-CAT0-MODULE-CALLABLE-CATALOG-DESIGN-STOP-001
Parent taskboard: mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
Previous closed card: mirbuilder-ssa-i1-compat-p0c-i64-call-design-stop-2026-07-15.md
Public baseline commit: ec570ea696
Decision requested: select the exact CAT0 source/catalog authority and CAT0/MP0 boundary
---

# ChatGPT Pro Consultation: P0c-CAT0 Module Callable Catalog

## Executive question

P0c-I1 now executes one exact static current-owner self call. Its target is a
generic `ResolvedCallableRefV1`, resolved before Builder effects through a
source-unit-owned one-entry `VerifiedCallableIndexV1`. Lower consumes one
co-sealed direct-call row and performs no source-name lookup or fallback.

The next step is the catalog prerequisite for sibling calls. Please select the
cleanest CAT0 design without pulling body resolution, sibling-call execution,
or multi-function publication into the same row.

Our working hypothesis is:

> **A′ — seal one immutable, module-owned, multi-header callable catalog from
> an exact function-only `ASTNode::Program`; keep every top-level function as
> its own single-root owner forest; defer body resolution and unpublished MIR
> draft publication to MP0.**

Please validate or reject A′. If it is wrong, identify the exact authority
conflict and select one alternative. Do not widen the answer into MethodCall,
receiver, Box ownership, imports, separate compilation, or backend support.

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

### Catalog

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
```

The index remains the sole header/name-resolution authority. Declaration rows
prove exact source membership only; they do not duplicate the signature or
symbol.

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

### Seal order

```text
1. validate the exact function-only Program surface
2. issue one invocation-local owner per declaration
3. seal every exact header into one immutable catalog
4. verify all source keys, owner refs, sites, and symbols are bijective
5. publish the catalog
6. only later, in MP0, analyze any function body
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

Required typed seal errors include:

```text
EmptyCatalog
UnsupportedProgramStatement { site, actual }
DuplicateSourceKey { key, first_site, second_site }
DuplicateCallableIdentity { callable, first_key, second_key }
DuplicateDeclarationSite { site }
PhysicalSymbolCollision { symbol, first_key, second_key }
MixedCompilationBrand
HeaderOutsideExactI64Profile
```

Any error publishes no partial catalog.

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
    catalog: VerifiedCallableCatalogV1,
    functions:
        BTreeMap<ResolvedCallableRefV1,
                 VerifiedResolvedFunctionUnitV1>,
}

pub struct VerifiedResolvedFunctionUnitV1 {
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
}
```

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

## Alternatives to judge

### A′ — function-only Program catalog, body-free CAT0

Recommended working hypothesis. It creates the missing source authority while
preserving one-root function forests and leaving publication to MP0.

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

## Requested decisions

Please answer all six:

1. Select A′, B, C, D, or a precisely bounded alternative.
2. Is function-only `ASTNode::Program` the correct first free-static module
   source authority?
3. Should `VerifiedSemanticOwnerForestV1` remain single-root, with the module
   owning one forest per top-level function?
4. Should owner IDs be pre-issued for every header before any body resolution?
5. Should CAT0 remain header-only, with body resolution and exact source
   projections deferred to MP0?
6. Must MP0 batch unpublished function drafts before insertion into the
   candidate module to claim internal declaration-order independence?

## Proposed task order if A′ is accepted

```text
CAT0-D0:
  decision lock and exact non-claims

CAT0-L0:
  behavior-neutral index storage cleanup
  one primary header store + reverse foreign-key indexes
  checked sole-header P0c facade

CAT0-S0:
  CallableModuleHeaderSyntaxViewV1
  SourceCallableDeclarationSiteV1
  function-only Program coverage

CAT0-C0:
  disconnected VerifiedCallableCatalogV1 seal
  all owner/header rows complete before body access

CAT0-G0:
  fixtures, normalized reorder parity, authority/caller-zero guards

MP0-S0:
  exact multi-function source-unit carrier

MP0-R0:
  two-pass per-function body/forest resolution against complete catalog

MP0-P0:
  whole-module preflight before Builder effects

MP0-TX0:
  unpublished function draft set and batch publication

P0c-B1:
  exactly two static exact-i64 functions and one sibling edge

P0c-MR:
  mutual recursion / callable SCC as a separate row
```

Each implementation row must contain code or generated artifact delta. CAT0
does not activate sibling calls merely because multiple headers exist.

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

## Final question

Is A′ the cleanest next row?

> Seal all free-static headers from one exact function-only Program before any
> body is read; keep the catalog module-owned and immutable; keep each function
> forest single-root; defer body resolution and unpublished draft publication
> to MP0.

