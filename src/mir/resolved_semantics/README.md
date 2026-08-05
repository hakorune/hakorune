# Resolved Semantics

This module family owns the passive, function-scoped semantic product that is
constructed before Planner and Lower.

```text
canonical function AST
  -> FunctionSemanticResolverV1       (SA1+)
  -> ResolvedFunctionDraftV1
  -> ResolvedFunctionVerifierV1       (SA2+)
  -> VerifiedResolvedFunctionV1
```

SA0 adds only the closed canonical schema and publication boundary. SA1 adds
a physically separate, disconnected shadow resolver. The shadow resolver may
read canonical syntax, but it owns only `Shadow*V0` handles and can publish
only `ShadowResolvedFunctionV0` for tests and deterministic inspection. It
cannot populate the canonical draft, allocate `BindingId`, plan control flow,
or lower MIR.

## OF0/UP0 owner-forest boundary

`VerifiedSemanticOwnerForestV1` is the first cross-owner authority. One
`FunctionSemanticResolverSessionV1` reuses one compilation-scoped owner issuer
and recursively resolves a root function plus non-capturing Lambda children.

```text
borrowed canonical syntax
  -> owner-local shadow traversal
  -> owner-local VerifiedResolvedFunctionV1
  -> primary owners + parent edges
  -> forest verify/derive
  -> VerifiedSemanticOwnerForestV1
```

The forest directly owns every sealed owner product. Its only primary
topology is the owner map plus child-to-parent edges; the single root,
definition-site child index, and normalized graph are derived exactly once at
seal. OF0 rejects a second root, mixed compilation brands, cycles, duplicate
parent/definition sites, and a parent scope that is not the exact lexical
scope containing the Lambda definition.

Lambda syntax is borrowed through an AST-derived view and never cloned into a
semantic product. A child declaration uses a child-local `BindingId`; raw IDs
may repeat across owners without aliasing.

UP0 adds only a structural read relation:

```text
ResolvedLexicalRefV1
  = Local(BindingRefV1)
  | Upvar(UpvarRefV1 {
      capturing_owner,
      source: strict-ancestor BindingRefV1,
    })
```

The relation is deduplicated by `(capturing_owner, source)` and grandchild
references point directly to the original declaration. Forest seal verifies
the source exists, belongs to a strict ancestor, was visible at every Lambda
definition boundary, and is not hidden by a nearer same-name declaration.
Normalized parity uses structural owner keys and binding origins, never raw
owner or binding numbers.

UP1 adds exact-site `Read | Rebind` observations over the same structural
relation. A write is `ResolvedAssignmentTargetV1::UpvarRebind`; it is never
misreported as a local `BindingRebind`. The unique Upvar inventory and ordered
access observations are both seal-derived. They are capture-plan input only:
UP1 creates no CaptureId, synthetic child BindingId, capture mode, forwarding
binding, runtime slot, ValueId, BasicBlockId, Recipe, Planner, or Lower
connection.

## B0-F lexical BlockExpr resolver contract

B0-S adds a disconnected canonical vocabulary for lexical BlockExpr identity:
`ScopeKindV1::BlockExpr` and `RegionKindV1::BlockExpr`. A sealed draft may
represent the expression only as one reciprocal scope/region pair. Both
records carry the same `Source(BlockExprPreludeRoot)` origin; that root is the
lexical anchor for the ordered prelude and the required tail together.

The seal verifier rejects a partial kind pair or different origins and treats
only `BlockExprPrelude(index)` and `BlockExprTail` descendants with the same
prefix as members. Existing owner-branded IDs, parent graphs, and normalized
records remain the only identity and ancestry mechanisms.

B0-F installs one dedicated shadow traversal box over that sealed pair. It
resolves the ordered prelude before the tail, restores the outer lexical scope
after the expression, and gives nested BlockExprs independent pairs. Existing
binding and owner-forest machinery supplies initializer-before-declaration,
outer rebind, and Lambda declaration-order behavior without a second policy.

The neutral AST non-local-exit query is the acceptance SSOT. A rejected
prelude reports its exact statement container and a rejected tail reports its
exact expression container; loop-local Break/Continue remain valid. Planner,
RegionFlow, ProgramV0, and compatibility fallback remain disconnected.
B0-L3a is the first bounded Lower consumer: it queries the sealed BlockExpr
pair by exact expression site and does not change resolver ownership.

## B0-L3b-S1 exact If identity bundle

S1 derives one private ID-only index while sealing
`VerifiedResolvedFunctionV1`. Each exact self-relative statement site maps to
one `If` control RegionId, one required reciprocal `IfThen` scope/region pair,
and zero or one reciprocal `IfElse` pair. The control has no lexical scope;
each branch region is a child of the control and each branch scope is a child
of the surrounding lexical scope.

The authoritative records remain the owner-scoped scope and region arenas.
The private index is a rebuildable seal witness and exists neither in mutable
drafts nor in `ResolvedFunctionDataV1`. Consumers use
`if_region_bundle(site)`; they may not rescan the arenas. Because
`SourceStmtSiteV1` is relative to one function root, this query means “site
inside this verified product.” S2 will prove the cross-owner source/product
closure before RegionFlow consumes it.

S1 proves arena topology only. Matching `else=None` versus
`else=Some(empty)` to located syntax remains an S2 obligation. RegionFlow,
Builder, Lower, ValueId, BasicBlockId, branch effects, ports, join contracts,
and canonical If runtime activation remain disconnected.

## B0-L4-S1 exact Loop identity bundle

S1 derives one private ID-only Loop index while sealing
`VerifiedResolvedFunctionV1`. Each exact self-relative Loop statement site
maps to one reciprocal `Loop` region / `LoopBody` scope pair. The Loop region
is a child of the exact surrounding region; its scope is a child of the exact
surrounding lexical scope. The region origin is the Loop statement site and
the scope origin is its `LoopBodyRoot` child.

The authoritative records remain the owner-scoped scope and region arenas.
The private index is a rebuildable seal witness and exists neither in mutable
drafts nor in `ResolvedFunctionDataV1`. Consumers use
`loop_region_bundle(site)` and may not rescan the arenas. Because
`SourceStmtSiteV1` is relative to one function root, cross-owner
source/product closure remains a later RegionFlow obligation.

S1 proves identity topology only. Located source coverage, carrier effects,
ports, ValueId, BasicBlockId, PHI policy, Builder/Lower connections, and
canonical Loop runtime activation remain disconnected.

## Generic nested-carrier identity boundary

The closed D2-B4-S2 witness consumed resolver-issued `BindingRefV1`
assignment/read sites for one actual nested-loop source fixture. Only an inner
write and post-loop outer read that resolve to the same strict-ancestor binding
under the same function/frame/source identity may issue test-only disjointness
evidence. A shadowing local must resolve to a different binding and remain
`UnresolvedStop`. This identity witness is not a Planner, Recipe, PHI, MIR, or
runtime authority.

The S2 identity witness is closed as test-only evidence. Scoped D3 design
consultation is recorded in
`docs/development/current/main/investigations/joinir-generic-nested-carrier-d3-bindingref-design-2026-08-05.md`;
only the exact resolver-issued BindingRef/source/frame class may proceed to a
typed mismatch matrix, and no production handoff is implied.

The D3 typed matrix is closed as cfg(test)-only evidence. Production selection
still lacks a co-sealed resolver/source/facts/preflight capability; that
boundary is designed separately in
`docs/development/current/main/investigations/joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md`.

The projector row
`JOINIR-GENERIC-RESOLVED-CARRIER-PROJECTOR-DESIGN0-D0` is closed as five
cfg(test)-only tests. Its parsed S2A nested-`IfThen` positive uses
`FunctionSourceViewV1` navigation and co-seals resolver forest/BindingRef,
function/source/frame, and private facts-only identity; cross-invocation facts
mismatch rejects before effects. Seed/invocation seal, neutral facts issuance,
and production selection remain later boundaries.
The source-backed bridge, planner-suppression row, Index/Ambiguous row, and
`JOINIR-GENERIC-RESOLVED-CARRIER-ELIGIBILITY-PROTOCOL0-D3-S0` are closed as
cfg(test)-only evidence. D3-S0 co-seals actual natural-Both
resolver/source/frame, BindingRef, facts, mode, and raw evidence into a
private non-`Clone` test witness with typed planner/shadowing/missing/
cross-invocation rejects. It is not the production neutral eligibility issuer;
production selection remains stopped. The closed source-matrix row is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-COMPOUND-UNAVAILABLE0-D2-S3`:
one parsed nested `CompoundAssignment` under scoped basic sugar co-seals
actual resolver/source/frame/BindingRef/facts evidence and preserves exact
`Unavailable("CompoundAssignment")` as typed pre-effect
`UnresolvedStop(CompoundUnavailableCarrier)`. Its raw Release/Strict schedule
is now measured as `[V0,V1]`; parent D3 production selection remains stopped.

The selected source-premise task was
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-TOPLEVEL-COMPOUND-PREMISE0-D2-S4`.
It is a parsed, cfg(test)-only observation of the top-level compound path. The
resolver/source/frame witness is co-sealed, but the current facts product is
absent and Release/Strict both measure raw schedule `[]`; the typed result is
`NoStandaloneRow`. No non-recursive source row, neutral issuer, selector,
production handoff, Recipe, PHI, or MIR authority is added. Reference and
current mirrors closed in the same implementation commit.

The design stop is now sealed for one implementation child:
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-NORECURSIVE-DISPOSITION0-D2-S5-D0`.
It is docs-only until the parsed one-loop source shape, resolver identity,
facts disposition, and raw-schedule owner are fixed. The existing recursive
two-member projector must not be reused implicitly; no production capability
or selector is introduced. The child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-BOTH-NORECURSIVE0-D2-S5-S1`;
it consumes only the exact parsed one-loop shape and returns to the design
stop on any facts, schedule, mode, or identity mismatch.

S1 is closed as a test-only one-loop source witness. Its exact facts and raw
schedule are retained as provenance, while the one-member shape remains
out-of-target for recursive eligibility; no neutral capability or selector
consumer was added.

The accepted D3-S1 disposition matrix
`JOINIR-GENERIC-RESOLVED-CARRIER-SELECTION-DISPOSITION-MATRIX0-D3-S1-D0`.
It keeps resolver/source/frame identity separate from the future AST-free
`GenericCarrierFactsSnapshotV1` handoff and leaves natural recursive Both as
typed `UnresolvedStop(WinnerCorrectnessUnavailable)` until correctness and
disjointness are proven. The selected V1-only local child is closed as
cfg(test)-only evidence: it co-seals V0=false, V1=true,
`CompleteNoRecursiveCarrier`, `has_body_local=false`, actual frame flags, no
recipe contract, and raw `[V1]`, with typed
`UnresolvedStop(V1OnlyNonRecursive)`.

The D3-S1-S2 candidate-stage source bridge is also closed as cfg(test)-only
evidence. It co-seals the parsed natural-Both resolver forest/BindingRef
obligation with fresh V0/V1 plan projections and proves Release/Strict raw
`[V0,V1]`, direct `LowerSome`/`GenericComposer`, stable route-order snapshots,
distinct resolver owners, V0 outer-carrier absence, and V1 outer `j` plus
carrier/step PHI labels. The projection is name-backed corroboration only;
there is no typed BindingRef-to-ValueId provenance or full-return parity. The
actual legacy trace remains V0 terminal/no-debt, planner-required remains
`[V1]` unresolved, and production selection/issuer authority remains zero.

The typed provenance handoff design
`JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-HANDOFF-DESIGN0-D3-S2-D0`
remains the authority for the upper source-to-selection boundary. D3-S2-D1
realigns that authority before production work: resolver output owns only
source identity/projection; neutral facts may exist only after the P0/P1
source-site totality boundary; and the Recipe producer is the sole issuer of
recipe-local `LoopBindingKeyV1` values and their `BindingRefV1` relation.
Binding SSA remains the sole physical `BindingRefV1 -> ValueId/PHI` owner and
does not mint Recipe keys. Independent preflight-seed, invocation-seal, and
four-field selection-input authorities are rejected; a later selector may
consume only one verified non-`Clone` canonical plan. P0 is closed by its
machine-readable facts/producer-arm matrix. P1 is also closed by the
machine-readable source-projection witness
`joinir-generic-resolved-carrier-source-projection-d3-s2-p1-matrix-2026-08-05.tsv`.
The next row is the test-only neutral AST-free facts snapshot
`JOINIR-GENERIC-RESOLVED-CARRIER-FACTS-SNAPSHOT0-D3-S2-P2`; no production
caller is authorized.

The first two observation children are closed as cfg(test)-only evidence, and the
bounded passive product
`JOINIR-GENERIC-RESOLVED-CARRIER-PROVENANCE-PRODUCT0-D3-S2-S2` is now closed
as cfg(test)-only evidence. `LoopRouteContext` remains a loop-fragment owner;
it does not lower post-loop Return/ABI/Home semantics. Scalar full-function
Return projection, natural debt-to-different-winner, and Home-bearing
evidence remain separate design rows. No selector, Generic snapshot/key/seed,
opaque input, Recipe, PHI, Builder, MIR, retry, fallback, or runtime caller is
authorized by this product.

The D3-S2-P2 neutral facts snapshot is now closed as a separate cfg(test)-only
sibling. It consumes exactly one sealed `VerifiedResolvedCarrierProvenanceV1`
and adds only the mode-neutral `NestedWriteWithPostLoopRead` disposition. It
does not re-validate or re-issue P1 source authority, and it does not modify
`LoopFacts`, Generic V0/V1 facts, selector, Recipe, Builder, MIR, PHI, Home,
debt, retry, fallback, or runtime ownership. No production caller is
authorized.

The selected D3-S2-S3 repeat audit remains cfg(test)-only historical evidence. It consumes two
complete S2 products as one non-`Clone` pair, observes repeated source
topology/roles and distinct resolver brands, and records equal raw frame
coordinates without treating them as identity. Mismatches reject before
effects. It adds no Generic key, selector, DirectAccum frame, Builder, MIR, or
production authority and does not authorize the superseded independent
seed/seal model.

The passive product consumes one resolver-issued owner/forest/frame/role
handoff as a single non-`Clone` value and publishes one opaque AST-free source
witness. It rejects mixed owner brands, foreign/unequal bindings, duplicate or
unsupported roles, incomplete forest shape, source/frame mismatch, and wrong
strict-ancestor shape before any Builder effect. DirectAccum's ownerless
structural frame remains unchanged. The factory is test-only; its
`for_test(...)` constructors are fixture ingress, not semantic issuers.

The observation child is now closed as cfg(test)-only evidence. It records
only resolver-owned forest/frame and exact `BindingRefV1` role/ancestry facts;
the production resolver and neutral modules gained no Generic policy or
caller. Four focused tests cover natural success and typed mismatch rejects.
The forest/frame coordinates are not yet an owner-branded cross-session
capability: equal `FunctionOriginV1`/site coordinates from fresh resolver
sessions can still be mixed. A brand audit must precede any shared provenance
product or snapshot issuer.

The bounded cfg(test) witness is green with:

```bash
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib generic_d2_b4_s2 -- --nocapture
```

Its evidence kind is resolver/source identity plus canonical Generic facts, not
runtime-result parity. Planner-required V0 suppression is recorded as a typed
unavailable row; the V0 composer is not called without V0 facts. Synthetic
legacy carrier labels/tags are corroboration only. The row remains a design
stop and does not authorize a production caller or route policy.

The follow-up nested-`IfThen` carrier row is also closed as `cfg(test)`-only
evidence. One parsed source keeps a separate inner canonical `j` step while a
nested write and post-loop read resolve to the same strict-ancestor
`BindingRefV1`; the sealed loop forest contains exactly the outer and inner
loops and preserves function/source/frame identity. Release/Strict direct
observations are fresh and stable (`[V0, V1]`, `LowerSome`, distinct semantic
digests), while the legacy witness remains a V0 terminal. This does not issue
a neutral capability or change Planner, Registry, Builder, MIR, or runtime
authority; parent Generic D2 and the co-sealed handoff remain unresolved.

## B0-L3b-I1a exact lowering roots

The verified product owns one seal-derived ID-only lowering-root carrier. It
pairs the authoritative Function scope/region roots with exactly one lexical
function-body scope and `Sequence` region rooted at `FunctionBody` or
`LambdaBodyRoot`. The draft/data schema remains carrier-free.

Lower consumes this carrier directly when seeding separate lexical-scope and
control-region stacks. It must not scan the arenas or reconstruct source paths
to rediscover the function-body root. The carrier is a rebuildable seal
witness, not a second scope/region authority, and adds no Builder or runtime
connection by itself.

## B0-L2b shared source-path boundary

`SourcePathSegmentV1` remains the only path grammar. B0-L2b promotes its small
immutable builder to `SourcePathV1` in `source_site.rs`; the shadow resolver's
historical `ShadowSourcePathV0` name is only a local alias. The compiler-side
source projection consumes these paths but this semantic module never imports
compiler or Lower code.

The sealed compiler projection validates that every owner root follows the
forest's exact Lambda definition chain and that semantic declaration/use/
assignment/exit/scope/region sites project to the expected syntax family.
It stores no AST reference or pointer. Future Lower obtains borrowed syntax
only through an immutable function source view and located-node carriers.
B0-L2b adds no resolver acceptance, production source-unit construction,
Planner/RegionFlow/Recipe connection, or MIR materialization.

SA2 installs the real publication boundary. A compilation-scoped issuer with
a process-unique compilation brand is the only owner-brand constructor;
`ResolvedFunctionDraftV1::seal` verifies owner membership, graph
roots/ancestry, binding accounting, supplied source-index integrity, and exact
RegionId control targets before publishing. The sealed product also owns a
deterministic origin-keyed normalized graph for parity. Raw owner, binding,
scope, and region numbers never enter that graph. SA2 still has no canonical
AST resolver producer and no Planner or Lower connection; those are later
authority-cutover slices.

Because the canonical AST is intentionally not stored here, SA2 cannot prove
that a caller omitted no syntax site. Canonical-AST site totality becomes
checkable only when the SA3 resolver co-constructs the closed indexes. SA2
claims referential integrity of supplied indexes, not syntax-independent
omniscience.

## SA3 transport boundary

SA3-A adds a behavior-neutral Lower transport in
`builder/vars/resolved_binding_state.rs`. It can hold one sealed product,
claim an exact `SourceBindingSiteV1` once, then consume that one-shot claim to
publish `BindingId -> ValueId`.  Order, name, and kind alone are never
declaration identity.
Resolved parameter/local publication APIs exist but have no production caller
until the atomic SA3-B switch. During SA3-A the legacy Lower allocator remains
the single active canonical BindingId owner; installing a canonical product is
still zero.

The canonical `FunctionSemanticResolverSessionV1` also exists as a
disconnected producer. It derives and borrows `FunctionSyntaxViewV1` from one
canonical function AST, uses
construction-local draft indices during one traversal, converts them once to
owner-scoped canonical IDs, and immediately verifies/seals. Draft indices
never enter Lower or the normalized parity graph. Production installation
remains zero until the accepted syntax inventory and all declaration callers
can switch together.

## AST-BIND0-L0 qualified-receiver observation

The source-call lane may ask the existing shadow traversal to classify an
exact pre-verified `MethodCall` receiver site as `Bound` or `ProvenUnbound`.
This is an observation mode of the same lexical walker, not a second resolver.
Only requested receiver sites may convert an otherwise unresolved Variable
into positive `ProvenUnbound`; every other unresolved Variable keeps the
existing `UnresolvedName` failure. Requested and published site sets must
match exactly before the observation is returned.

Catalog-owned declarations enter through `FunctionSyntaxViewV1` over their
borrowed parameter/body parts and an explicit receiver policy. The observation
allocates no `FunctionOriginV1`, canonical Binding identity, scope product, or
MIR state. Shadow binding ordinals stay construction-local and are collapsed
to the disposition `Bound`; they never enter the source-call product.

## Callable-result PATH0 structural observation

Callable-result activation reuses this same shadow traversal to inventory all
`MethodCall` sites.  Compiler located views, shadow resolution, and later
located legacy lowering share one neutral child-role-to-`SourcePathSegmentV1`
policy from `resolved_semantics`; a second AST walker or a second path table is
not permitted.

The all-call observation is read-only.  It may classify a receiver as the
current owner, a lexically bound qualified value, a proven-unbound qualified
owner spelling, or another dynamic expression.  It does not resolve targets,
infer result types, allocate canonical bindings, or publish MIR state.
Production callable-result consumers remain zero through PATH0.

## P0c callable-header boundary

`CallableHeaderSyntaxViewV1` is a separate body-free view over one
`FunctionDeclaration`. It exists so source callable name, arity, exact scalar
signature, and physical symbol projection are observed once without widening
the body-oriented `FunctionSyntaxViewV1` or teaching Lower to read raw names.

`VerifiedCallableIndexV1` is the sole callable-header authority. CAT0 seals one
or more static, non-main, all-`i64` headers through the same deterministic
`seal_many` path. A singleton Program is ordinary catalog cardinality one; it
has no separate one-entry seal, sole-header facade, or callable-forest sidecar.

`CallableFunctionSyntaxViewV1` derives the header and body views from the same
function AST for CAT0/MP0 resolution. The Program/catalog source unit owns the index once; each
`VerifiedResolvedFunctionV1` stores only exact
`SourceExprSiteV1 -> ResolvedDirectCallTargetV1` identity rows. Full headers,
symbols, and signatures are never copied into the function product. The
body-only resolver remains call-disabled and produces zero direct-call target
rows.

```text
CanonicalCallableKeyV1:
  source lookup key

ResolvedCallableRefV1:
  typed reference to the existing FunctionOwnerIdV1 identity

CanonicalCallableSymbolV1:
  one-way physical MIR/backend projection
```

The symbol is never parsed back into source identity. The runtime/plugin
Box-callable registry, MIR module function table, legacy global resolver, and
Lower name comparisons are not callable-index authorities.

CAT0-S0 adds a separate Program-owned header source-unit shell. It validates
the complete top-level surface as one non-empty function-only Program before
exposing opaque `SourceCallableDeclarationSiteV1` rows. Callers can borrow
only `CallableHeaderSyntaxViewV1` through those sites; the owned Program, raw
statement list, function bodies, and a constructor that pairs independent
Program/catalog products remain hidden. S0 does not validate exact-i64 header
profiles, issue owners, build a multi-header index, resolve bodies, or connect
Builder/Lower/runtime behavior. Those responsibilities begin at CAT0-C0a and
later rows.

CAT0-C0a consumes that S0 source unit into one owner-free candidate product.
The primary candidate store is declaration-site keyed; exact callable key and
physical symbol indexes point back to those sites and do not duplicate a
candidate. Every header must satisfy the exact static non-main all-`i64`
profile, and duplicate keys reject with both source sites before any
`FunctionOriginV1` or `FunctionOwnerIdV1` is issued. Candidate sealing still
does not resolve bodies, create the immutable owned catalog, or connect a
production caller.

## Authority

- The canonical AST owns syntax and source execution order.
- A sealed `VerifiedResolvedFunctionV1` owns resolved binding, scope, region,
  assignment-target, and control-target facts for one function.
- `BindingOriginV1`, `RegionOriginV1`, and source sites are provenance. They
  are not lexical/control equality authorities.
- `BindingRefV1`, `ScopeId`, and `RegionId` carry an invocation-local function
  owner brand. The brand detects cross-function mixing but is not source or
  parity identity. `BindingRefV1` wraps the canonical `BindingId`; it does not
  replace it.
- Lower owns `BindingId -> ValueId` and `RegionId -> BasicBlockId`
  materialization only.

## Forbidden dependencies

This module family must not import or call:

- canonical arena files must not own cloned/normalized AST payloads;
  `FunctionSyntaxViewV1` and the construction resolver may only borrow syntax;
- Planner, Recipe, JoinIR, or Lower modules;
- `ValueId` or `BasicBlockId`;
- `MirBuilder`, `CoreContext`, or any BindingId allocator;
- the GC/debug `mir::region::RegionId`;

Mutable drafts remain crate-private. Only a verified sealed product may become
a public consumer input. Unsupported resolution never retries a legacy path.

SA1's `ShadowBindingOrdinalV0` is now also the construction-local draft index
used inside canonical resolution. It is converted and discarded before seal;
it never enters Planner or Lower and is not semantic identity. Unsupported
syntax returns a typed resolver error and never retries an old resolver. There
is no test-only unverified publication bypass.
