# Resolved Region Flow V1 — Taskboard

Status: SA2 verifier/seal hardening closed; SA3 atomic BindingId authority cutover is next.
Date: 2026-07-13
Decision: `function_semantic_resolver_v1_owner_scoped_arena` followed by
`recursive_structured_region_flow`.
Final retirement target: legacy mandatory single-`loop_var` family = 0 callers.

## Purpose

Represent every structured control region as a recursive summary from one
entry binding state to multiple typed exit ports.  A generic loop is a region
with feedback and exit ports, not a search for one progression variable.

```text
canonical AST
  + canonical lexical resolution
  + resolved control targets
      -> ResolvedRegionViewV1
      -> RegionFlowSummaryV1
      -> LoopStateContractV1
      -> LoopStateClosureVerifierV1
      -> VerifiedGenericWhilePlanV1
      -> Lower
```

This follows the established structured-region/gamma-theta family of ideas,
but the implementation must not claim to be a complete RVSDG or MLIR model.

## Accepted prerequisite — owner-scoped resolved semantic arena

R0 proved that the existing `hakorune_mir_core::BindingId` has the correct
lexical meaning but is allocated too late, while Lower executes declarations.
It also proved that source exits have no exact resolved control owner before
Planner.  The accepted correction is one function-owned, verified semantic
sidecar constructed before Planner and Lower:

```text
canonical function AST
  -> FunctionSemanticResolverV1
  -> ResolvedFunctionDraftV1
  -> ResolvedFunctionVerifierV1
  -> VerifiedResolvedFunctionV1
       bindings arena
       scopes arena
       regions arena
       declaration/use/assignment indexes
       exact control-exit targets
  -> ResolvedRegionViewV1
  -> RegionFlowSummaryV1
  -> LoopStateContractV1
  -> VerifiedGenericWhilePlanV1
  -> Lower
```

`VerifiedResolvedFunctionV1` is the canonical semantic sidecar authority for
one immutable function AST.  It is not a replacement AST and stores no cloned
`ASTNode` payload.  Consumers receive syntax plus the sealed semantic sidecar.

```text
source syntax/node meaning:
  canonical AST

lexical/control semantic authority:
  VerifiedResolvedFunctionV1

lexical identity handle:
  owner-scoped BindingId

lexical lifetime/name visibility:
  owner-scoped ScopeId

control identity/target:
  owner-scoped RegionId

MIR representation:
  Lower-owned ValueId / BasicBlockId

diagnostic and parity provenance:
  BindingOriginV1 / RegionOriginV1 / structural source sites
```

Raw numeric IDs and origin paths are not semantic equality authorities across
functions or frontends.  The arena record addressed by an owner-scoped ID is
the authority.  Origin fields are checked provenance and normalized parity
keys only.

### Closed arena shape

```text
VerifiedResolvedFunctionV1 {
  function_origin,
  bindings: Arena<BindingId, ResolvedBindingRecordV1>,
  scopes: Arena<ScopeId, ResolvedScopeRecordV1>,
  regions: Arena<RegionId, ResolvedRegionRecordV1>,
  declarations: SourceBindingSiteV1 -> BindingId,
  variable_uses: SourceExprSiteV1 -> BindingId,
  assignment_targets: SourceExprSiteV1 -> ResolvedAssignmentTargetV1,
  control_exits: SourceStmtSiteV1 -> ResolvedControlExitV1,
}
```

The indexes are sealed, rebuildable witnesses over the arenas.  They never
allocate identities and cannot override arena records.

```text
ResolvedAssignmentTargetV1 =
  BindingRebind(BindingRefV1)
  | FieldWrite
  | IndexWrite

ResolvedControlExitV1 =
  Continue(target_loop_region_id)
  | Break(target_loop_region_id)
  | Return(target_function_region_id)
```

`ScopeId` and `RegionId` remain distinct: scope owns name visibility and
lifetime; region owns control structure and ports.  The control-target
resolver consumes already allocated RegionIds and never allocates them.

Unsupported assignment syntax is a resolver outcome and prevents sealing. It
is not stored as a variant inside `VerifiedResolvedFunctionV1`.

Public lookups use an invocation-local function owner brand:

```text
BindingRefV1 = FunctionOwnerIdV1 + canonical BindingId
ScopeId      = FunctionOwnerIdV1 + scope slot
RegionId     = FunctionOwnerIdV1 + region slot
```

The owner brand detects accidental cross-function mixing but is not source or
parity identity. Canonical BindingIds are not assumed dense or zero-based in a
function; the passive arena is keyed explicitly. The sealed product also
publishes exact `function_scope` and `function_region` roots.

### Authority-cutover invariant

There may be a non-authoritative shadow resolver, but one function invocation
must never have two production lexical identity allocators.

```text
shadow phase:
  ShadowBindingOrdinalV0 only
  plan/lower input = 0

canonical cutover:
  FunctionSemanticResolverV1 allocates BindingId
  CoreContext/MirBuilder Lower-time BindingId allocation = 0
  Lower receives exact resolved declaration BindingId
```

The cutover is one Refactor Series Mode objective.  Intermediate commits must
build, but no commit may allow the same function to allocate one lexical
binding independently in both resolver and Lower.  Unsupported syntax rejects
before semantic product publication; there is no legacy retry or fallback.

### Lower boundary after cutover

```text
ResolverScopeStackV1:
  mutable name -> BindingId during resolution only
  discarded at seal

LowerValueEnvironmentV1:
  mutable BindingId -> current ValueId during Lower only

Region materialization:
  RegionId -> BasicBlockId/entry/continue/break/after targets
```

Lower may read syntax to emit expressions.  It may not rediscover lexical
identity, control targets, carrier roles, or loop depth from syntax.

## Minimal principles

```text
analysis simulates SSA value versions = 0
analysis allocates lexical binding IDs = 0
name-based binding authority = 0
normalized cloned AST = 0
one selected loop_var = 0
step filtering in generic baseline = 0

recursive Port -> MayRebind(BindingId set) summary = 1
Lower owns BindingId -> current ValueId materialization = 1
```

`may_rebind` is intentionally conservative.  Extra state slots may add PHIs
but cannot change semantics.  Dead state/PHI removal is a later generic state
optimization, not canonical-loop specialization.

## Four identities

Keep these independent:

```text
SourceStmtSiteV1:
  source statement and coverage identity

BindingId:
  compilation-local canonical lexical binding handle

RegionIdV1 / ScopeIdV1:
  structured control and lexical scope handles

ValueId:
  MIR representation allocated and owned only by Lower
```

Cross-frontend `BindingKeyV1` is deferred until parity work.  Raw numeric
BindingId is never a Rust/Hako semantic identity.

## Canonical BindingId rule

Region analysis consumes resolver-owned BindingIds for all references,
including body-local declarations and shadowed names.

```text
allowed:
  read-only canonical binding-context snapshot/adapter

forbidden:
  RegionFlow allocating private BindingIds
  shadow-name sets deciding identity
  variable names as map keys for state authority
```

Names may appear only in diagnostics.

## ResolvedRegionViewV1

The view is an immutable, closed structural observation.  It is not a second
AST and contains no cloned `ASTNode` payload.

```text
ResolvedRegionViewV1 {
  binding_inventory_ref,
  scope_inventory,
  region_arena,
  source_projection,
}

ResolvedRegionNodeV1 =
  Sequence(children)
  | LexicalScope(scope_id, child)
  | If(site, condition_ref, then_region, else_region?)
  | Loop(site, loop_region_id, condition_ref, body_region)
  | Statement(site, ResolvedStatementEffectV1)
  | Exit(site, ResolvedControlTargetV1)
```

Canonical AST remains language/node-meaning authority.  The region view owns
only IDs, roles, order, resolved binding references, and control targets.

## Source coverage

`SourceStmtSiteV1` remains structural source identity.  ScopeBox has two
consumer-specific meanings:

```text
Recipe execution order:
  transparent certified container

lexical lifetime / region flow:
  real LexicalScope boundary
```

Source projection publishes ordered sites and child roles, never normalized
AST clones.  Recipe/provenance are later co-constructed and co-sealed.

## Resolved control targets

Resolve break/continue once:

```text
ResolvedControlTargetV1 =
  Continue(target_loop_region_id)
  | Break(target_loop_region_id)
  | Return(target_function_region_id)
```

RegionFlow and Lower never recount nesting depth.  A nested child consumes
ports targeting itself and propagates ports targeting ancestors.

## RegionFlowSummaryV1

The only correctness state summary is port-indexed may-rebind information:

```text
RegionFlowSummaryV1 {
  ports: OrderedMap<RegionPortV1, PortEffectV1>,
  non_binding_effects: NonBindingEffectSummaryV1,
}

PortEffectV1 {
  may_rebind_outer: OrderedSet<BindingId>,
}

RegionPortV1 =
  Fallthrough
  | Continue(target_loop_region_id)
  | Break(target_loop_region_id)
  | Return(target_function_region_id)
```

It stores no current value/version tokens.

### Composition

```text
Statement rebind:
  Fallthrough += target BindingId

Sequence A; B:
  compose A.Fallthrough into every B port
  preserve A terminal ports

If:
  union summaries by port
  missing else = identity Fallthrough

LexicalScope:
  recursively summarize child
  remove scope-local BindingIds from every outgoing port
  propagate outer rebinds

Nested Loop:
  consume self-targeted Continue/Break
  publish one verified boundary summary to parent
  parent does not rescan child AST
```

Scope-exit handling applies to fallthrough, continue, break, and return.  Fini
or cleanup effects require a separate decision before activation.

## Binding rebind versus non-binding effects

```text
arr = new Array():
  BindingRebind(arr)

arr[i] = value:
  NonBindingEffect(IndexWrite)

obj.field = value:
  NonBindingEffect(FieldWrite)
```

Complex lvalues do not rebind their receiver without an explicit Place/binding
proof.  The first slice may use an opaque non-binding-effect marker; it must
not invent a full heap-effect authority.

## LoopStateContractV1

Derive two domains from port summaries:

```text
HeaderStateBindings =
  VisibleAtLoopEntry
  ∩
  MayRebindOn(
    BodyFallthrough
    ∪ Continue(target=this_loop)
  )

AfterStateBindings =
  VisibleAtLoopEntry
  ∩
  (
    HeaderStateBindings
    ∪ MayRebindOn(Break(target=this_loop))
  )
```

Return-only rebinds belong to the return port and do not enter header/after
state.  This is a conservative construction domain; liveness minimization is
not required for correctness.

```text
LoopStateContractV1 {
  header_state_bindings,
  after_state_bindings,
  feedback_edges,
  exit_edges,
}
```

No invariant/carrier/local complete partition is stored.  Those are derived
views from scope ancestry and port summaries.

## Edge contracts

Analysis publishes required binding handles, not SSA values:

```text
LoopEdgeStateContractV1 {
  edge_site,
  edge_kind,
  required_bindings: OrderedSet<BindingId>,
}
```

Lower owns:

```text
BindingValueEnv = OrderedMap<BindingId, ValueId>
```

At each verified edge, Lower captures the current ValueId for every required
BindingId.  It does not ask whether a statement occurred before continue or
rediscover state from AST.

## Generic while product

```text
VerifiedGenericWhilePlanV1 {
  source_coverage,
  condition_plan,
  complete_body_recipe,
  region_flow_summary,
  loop_state_contract,
  edge_state_contracts,
  port_contract,
}
```

Forbidden fields:

```text
loop_var
loop_increment
progression owner
BodyManagedCursor
analysis StateVersion
cloned normalized AST
```

Lower consumes this verified product only.

## Correctness and optimization split

```text
Generic correctness:
  resolved regions
  recursive port summaries
  complete body Recipe
  source coverage
  loop state/edge closure

Generic state optimization:
  dead state binding / dead PHI elimination
  invariant hoisting

Canonical loop optimization:
  exact step motion
  recurrence/induction specialization
```

These remain separate owners and commits.

## Existing owner seams to inventory

R0 must close the following before executable design work:

```text
canonical hakorune_mir_core::BindingId producer
binding_ctx snapshot/read-only access seam
VariableContext String -> ValueId compatibility owner
BindingContext String -> BindingId owner
scope enter/exit and shadow restoration owner
assignment target -> BindingId/Place owner
break/continue target-resolution owner
Facts/Planner access timing
join_ir/ownership private BindingId definition and callers
```

The private `join_ir/ownership` BindingId must be classified explicitly:

```text
CanonicalAlias
CompatAdapterWithRetirement
IndependentNonLexicalIdentity
SchemaMismatchStop
```

It may not silently become a second lexical identity authority.

## Parked WIP

```text
f74e5961e1
  wip/g0-source-projection before resolved-region-flow supersession

05fb9b0577
  earlier source-projection WIP before generic-baseline supersession
```

`f74e5961e1` has 32 focused progression tests and its acceptance-neutral
contract-pin guard green, but must not be applied wholesale.  R1 may reuse
path/schema/tests selectively.  The first code action is to remove published
`ProjectedStmtV0.node: ASTNode` clone ownership and keep structural IDs/roles.

## Task order

### R0 — binding/region seam inventory (closed)

1. Inventory every BindingId type, producer, allocator, and consumer.
2. Identify the one canonical lexical BindingId owner.
3. Specify the immutable binding-context snapshot/read-only seam.
4. Inventory scope ancestry and shadow restoration.
5. Inventory break/continue target resolution and decide RegionId target seam.
6. Classify `join_ir/ownership` private BindingId and name retirement if compat.
7. Prove RegionFlow allocates no IDs and uses no names for identity.
8. Add dependency guards before implementation.

Output:

```text
ResolvedRegionSeamInventoryV1
```

No behavior, planner, Recipe, Lower, or product change.

Closed evidence and the A/B design question:

```text
docs/development/current/main/investigations/
  mirbuilder-resolved-region-flow-r0-seam-inventory-2026-07-13.md
```

R0 proves canonical BindingId exists but body-local allocation happens during
Lower, after AST-only generic Facts.  No immutable pre-plan resolved binding
tree or resolved control-target RegionId owner exists.  The private ownership
BindingId is `SchemaMismatchStop`.

Consultation closes this fork with
`function_semantic_resolver_v1_owner_scoped_arena`:
`FunctionSemanticResolverV1` becomes the sole eventual BindingId/ScopeId/
RegionId allocator for one function and publishes
`VerifiedResolvedFunctionV1` before Planner.  Structural source keys remain
provenance, not a second lexical identity authority.

### SA0 — passive semantic-arena schema and ownership guard (closed)

Add the closed owner-scoped ID/record/index vocabulary and module boundary.
This is a code-facing, behavior-neutral slice.

```text
add:
  BindingOriginV1 / RegionOriginV1
  ScopeId / RegionId owner-scoped handles
  resolved binding/scope/region records
  resolved assignment/control-exit vocabulary
  draft/sealed product type boundary

prove:
  AST clone fields = 0
  BindingId allocation in new module = 0
  ValueId/BasicBlockId imports = 0
  Planner/Lower connection = 0
  private ownership BindingId import = 0
```

Do not split a new crate in SA0.  Start in one neutral MIR-adjacent module
family with an explicit README/layer guard.  Moving passive IDs/records into a
dedicated semantic-core crate is a later dependency-direction cleanup after
the API and caller inventory are stable.

Closed evidence:

```text
module:
  src/mir/resolved_semantics/

publication:
  crate-private module
  crate-private draft/data
  immutable VerifiedResolvedFunctionV1 facade
  production verified constructors = 0

membership:
  FunctionOwnerIdV1 brand
  BindingRefV1 = owner + canonical BindingId
  ScopeId / RegionId = owner + slot
  cross-owner lookup rejects

arena layout:
  BTreeMap keyed identities
  canonical BindingId dense/zero-based assumption = 0
  explicit function scope/region roots

focused tests:
  6 passed

reusable guard:
  tools/checks/resolved_region_flow_authority_guard.sh
  summary=ok

behavior/planner/lower connection:
  0
```

All SA0 Rust files are below 800 lines; the largest is below 250 lines.  The
guard owns an exact source manifest, repository-wide consumer scan, public API
allowlist, focused test invocation, and alias-resistant BindingId allocation
ban.

### SA1 — non-authoritative shadow resolver

Resolve the current closed function-syntax inventory with private
`ShadowBindingOrdinalV0`, source sites, scope ancestry, region ancestry,
assignment kind, and exact exit targets.  The shadow product is dump/test only
and cannot enter Facts, Planner, Recipe, or Lower.

SA1 uses a separate `ShadowResolvedFunctionV0` product parameterized only by
shadow ordinals. It must not populate `ResolvedFunctionDraftV1`, wrap an
ordinal in `BindingId`, or publish `VerifiedResolvedFunctionV1`.

Before any canonical product producer is connected, SA2/SA3 must name the
`FunctionOwnerIdV1` issuance owner and prove one unique brand per active
function compilation.  Raw owner values never participate in Rust/Hako
parity.

Rules:

```text
initializer resolution occurs before declaration insertion
same-scope redeclaration rejects
shadowing creates a distinct shadow ordinal
field/index writes are not BindingRebind
break/continue choose nearest active loop RegionId
return chooses the owning function RegionId
unsupported syntax rejects; no old-resolver retry
```

Closed evidence:

```text
src/mir/resolved_semantics/shadow/
  ids.rs       shadow-only binding/scope/region handles
  product.rs   disconnected records, indexes, and typed failures
  path.rs      structural source-site construction
  resolver.rs  function owner, lexical stack, region/control stack
  expr.rs      closed expression/use/assignment-target traversal
  stmt.rs      declaration/scope/If/Loop/exit traversal
  tests.rs     22 focused fixtures
```

The function receiver and parameters occupy the function scope; the function
body has a child lexical scope, so body locals may shadow parameters or `me`
without reusing an ordinal. All Local initializers are resolved before any
binding from that declaration is inserted. Outbox initializer payload remains
non-semantic compatibility data, matching its current Lower owner. ScopeBox
and If branches own independent lexical scopes, and function/branch/loop/scope
body containers have distinct structural origins. Loop exits bind directly to
the nearest shadow loop region, while Return binds to the function region.

The reusable authority guard proves:

```text
canonical BindingId in shadow family = 0
canonical draft/product construction = 0
AST payload retained by shadow product = 0
external consumers = 0
Facts/Planner/Recipe/Lower connections = 0
all source files < 800 lines
```

Focused tests cover receiver/parameter/body-local identity, same-scope
rejection, all-initializers-first resolution, ScopeBox non-leak and outer
rebind, independent If scopes, Outbox, variable/field/index assignment
classification, exact inner/outer loop targets, Return ownership, malformed
declarations, unresolved names, and unsupported syntax. SA1 publishes no
canonical semantic authority; SA2 remains responsible for verification,
sealing, and normalized parity graphs.

### SA2 — verifier, seal, and normalized semantic graph

Status: closed on 2026-07-13 after bounded worker-review hardening. The compilation-scoped owner issuer is the sole
brand constructor; draft publication requires verification and seal; the
test-only bypass is removed; and normalized equality excludes raw owner,
binding, scope, and region numbers. Planner and Lower connections remain zero.
SA2 proves internal referential integrity of the supplied closed indexes. It
does not claim canonical-AST site totality before the SA3 resolver producer
co-constructs those indexes; omission against syntax cannot be proven from a
syntax-free sidecar alone.

Add `ResolvedFunctionVerifierV1` and seal only complete products.  Verify arena
ownership, parent acyclicity, source-index bijections, resolved use/assignment
totality, and ancestor control targets.  Publish a deterministic normalized
graph for Rust/Hako parity using origin records, never raw numeric IDs.

The shadow resolver must cover the complete production declaration/use/control
inventory before SA3.  Wildcard acceptance is forbidden.

### SA3 — atomic canonical BindingId authority cutover

Decision:

```text
D: owner_scoped_resolved_semantic_arena
```

SA3 is an authority relocation, not merely an allocator replacement.  The
single pre-Planner semantic authority is:

```text
Canonical Function AST
  -> FunctionSemanticResolverV1
  -> ResolvedFunctionDraftV1
  -> ResolvedFunctionVerifierV1
  -> VerifiedResolvedFunctionV1
```

`VerifiedResolvedFunctionV1` owns the sealed function-local binding, scope,
and control-region arenas plus exact declaration/use/assignment/exit indexes.
The identities remain strictly separate:

```text
SourceNodeSiteV1  source provenance
BindingId         lexical declaration identity within one function owner
ScopeId           lexical visibility and lifetime identity
RegionId          structured control identity
ValueId           Lower-owned SSA value identity
```

`BindingOriginV1` and `RegionOriginV1` are provenance used for diagnostics,
normalized Rust/Hako parity, and deterministic dumps.  They are not equality
or lookup authority.  Raw owner-local ordinals have no meaning outside their
sealed function owner.  `ResolvedFunctionV1` is a syntax-free semantic sidecar,
not a replacement AST and not an execution IR.

Construction ownership is closed:

```text
FunctionSemanticResolverSessionV1:
  issues the function owner
  allocates BindingId / ScopeId / RegionId once
  resolves names and exact control targets

RegionFlow / Planner:
  consume the verified arena
  allocate no semantic or SSA identities

Lower:
  BindingId -> current ValueId
  RegionId  -> materialized BasicBlockId targets
  performs no name or control-target rediscovery
```

SA3-A behavior-neutral transport preparation:

```text
ResolvedBindingLoweringStateV1:
  optional sealed function product
  exact declaration claims
  BindingId -> current ValueId

production canonical resolver installs = 0
production resolved declaration calls = 0
legacy allocator remains the only active allocator
```

SA3-B performs the atomic producer/caller switch. SA3-C deletes the then-dead
CoreContext/MirBuilder allocator surface.

SA3-B0 lands the disconnected canonical producer first. Its only canonical
`BindingId::new` site is the draft-to-arena conversion inside
`FunctionSemanticResolverSessionV1`; no production function installs the
result yet. `FunctionSyntaxViewV1` borrows params/body and never reconstructs
or stores an AST clone. Construction-local draft IDs remain unpublished and
are discarded before seal.

SA3-B1 closes the complete production declaration/use/control inventory.
Every canonical AST branch is classified exhaustively as resolved,
semantically transparent, or explicitly unsupported.  Wildcards, legacy
resolver retry, and partial publication are forbidden.

Inventory evidence on 2026-07-13:

```text
canonical AST variants = 57
current explicit resolver variants = 20
unclassified variants = 37
unclassified but production-accepted/no-op variants = 25
```

Therefore SA3-B1 is a required BoxShape closeout, not an incidental extension
inside the production cutover.  It is split into closed behavior-neutral
slices:

```text
B1-C  exhaustive 57-variant disposition classifier; wildcard = 0
B1-P  structural source paths for collections, calls, match, try, and blocks
B1-E  expression/container traversal with no declaration/control ownership
B1-A  CompoundAssignment and GroupedAssignment exact target resolution
B1-D  Nowait/Catch/Pattern declaration ownership and initializer ordering
B1-R  Try/Catch/Finally and match-arm scope/region construction
B1-N  nested-function/Lambda owner stop or independent product decision
B1-I  table-driven production entry installation proof
```

`B1-C` closed on 2026-07-13 with an inventory-only exhaustive Rust match over
all 57 variants.  It changes no resolver acceptance.  Transparent rows remain
candidates until their own traversal proof is connected; `This` is recorded
as explicit unsupported pending removal of the legacy resolver acceptance.
The authority guard forbids wildcard reintroduction and runs the focused
classifier fixtures.

Known production boundary mismatches must be resolved explicitly before B2:

```text
Nowait currently bypasses BindingId publication
TryCatch does not publish the catch binder
CorePlan local publication lacks an exact SourceBindingSiteV1
inline Main.main bypasses ordinary parameter publication
BlockExpr scope semantics are not yet an explicit resolver contract
Lambda owns a separate capture/function boundary
```

The B1-I installation matrix includes static/free functions, static box
methods, instance methods/constructors, inline and callable `Main.main`,
synthetic script/test `main`, rewritten REPL wrappers, and CorePlan local
publication.  Every applicable path must reach exactly one resolve/verify/seal
before parameter or body Lower.  Unsupported paths fail before Lower; they do
not use an old resolver.

SA3-B2 is the atomic production cutover:

```text
resolve + verify + seal before function Lower
install exactly one sealed product
claim receiver / parameters / locals / outboxes by exact SourceBindingSiteV1
consume each one-shot claim when publishing its ValueId
materialize BindingId -> ValueId only
finish with zero unclaimed declarations
```

SA3-B2 may use a temporary dual ValueId check, but there is still exactly one
lexical identity authority.  It must cover ordinary static/instance lowering,
CorePlan-local declarations, and all production function entry paths before
SA3-C removes the legacy allocator.

SA3-C retires every active Lower-time BindingId constructor and name-keyed
lexical authority only after SA3-B2 is green.  The exact caller-zero set is:

```text
CoreContext::next_binding_id
CoreContext::next_binding
MirBuilder::allocate_binding_id
declare-local APIs that allocate BindingId
Lower-side String -> BindingId authority
```

Stop SA3 before production cutover if any of the following remains:

```text
unsupported syntax requires silent fallback
Resolver is not the only BindingId allocator
BindingOrigin is used as binding equality
Lower resolves a name or exit target again
another function owner's BindingId / ScopeId / RegionId is accepted
one declaration/use/assignment/exit lacks an exact sealed index
same-scope redeclaration semantics differ between producer and consumer
ResolvedFunctionV1 owns or publishes cloned AST payloads
```

In one Refactor Series Mode objective:

```text
FunctionSemanticResolverV1:
  allocates canonical hakorune_mir_core::BindingId once

Lower declaration APIs:
  receive the resolved BindingId
  allocate ValueId only

retire from active production path:
  CoreContext::next_binding
  MirBuilder::allocate_binding_id
  Lower-time declaration BindingId allocation
```

Temporary dual verification may compare the old name-to-ValueId cache with the
new BindingId-to-ValueId environment, but it may not allocate a second lexical
identity.  Any mismatch is fail-fast; no fallback is permitted.

### SA4 — exact control-target cutover

Make resolved RegionId the production target authority.  A temporary checked
`RegionId + legacy_depth` witness is allowed only to prove equivalence.  Lower
uses `RegionId -> materialized blocks`; it does not select by stack depth.

After parity:

```text
Recipe depth synthesis = 0
plan/verifier depth recount = 0
Lower stack.len() - depth target selection = 0
```

The structured construction stack may remain for LIFO/resource invariants but
is not target authority.

### SA5 — duplicate-owner retirement

Migrate only the recursive traversal/effect-propagation shape from
`join_ir/ownership/ast_analyzer`.  Replace its name resolver and private ID
allocator with `VerifiedResolvedFunctionV1` lookups, then delete the private
BindingId family and old mutable name-based lexical authority.

SA5 also closes caller-zero for legacy declaration BindingId allocation and
numeric exit-depth target selection before R1 begins.

### R1 — minimal resolved region view

Consume `VerifiedResolvedFunctionV1` and implement the closed
`Sequence/Scope/If/Loop/Statement/Exit` structural vocabulary with canonical
BindingId references and exact target RegionIds.  Source projection stores no
AST clones.  Shadowing, scope exit, and source order are fixed by focused
fixtures.  Product connection remains zero.

### R2 — recursive region effect summary

Bottom-up port-indexed `may_rebind_outer` for Sequence, Scope, If, Loop,
Break, Continue, and Return.  Separate binding rebind from non-binding effect.

### R3 — loop state contract shadow

Derive header/after state domains and all feedback/exit edge requirements.
Keep existing planner/Lower unchanged.

### R4 — closure verifier

Verify every required binding is visible exactly once on every edge, child
locals never escape, outer rebinds are not lost, and target RegionIds are
closed.

### R5 — disconnected generic while plan

Construct `VerifiedGenericWhilePlanV1` for mandatory fixtures without one
progression owner or body filtering.  Planner/Lower connection remains zero.

### R6 — test-only generic while Lower

Materialize a neutral header/body/latch/after skeleton from verified contracts.
Lower performs no source/name/candidate rediscovery.

### R7 — differential corpus

Run explicit legacy and new routes and compare result, final outer bindings,
effects, condition order/count, continue, break, and return behavior.  No
runtime route switching.

### R8 — authority cutover

Make `VerifiedGenericWhilePlanV1` generic acceptance/Lower authority.  Move
canonical-step analysis to an optional optimization family.

### R9 — legacy single-loop-var retirement

After caller-zero proof, delete mandatory `loop_var`, `loop_increment`,
dedicated loop-var PHI/skeleton fields, carrier exclusion, step filtering,
BodyManagedCursor/sentinel, and candidate ambiguity generic freezes.

R9 is required for Epic completion.

## Mandatory fixtures

```text
same-scope redeclaration:
  resolver rejects before seal

initializer-before-declaration:
  local x initializer resolves an outer x before the new x is inserted

parameter/receiver/local identity:
  distinct binding kinds and origins; shadowing never reuses an ID

multi-local declaration:
  stable binder ordinals and exactly one declaration index per binding

shadowing:
  inner same-name local does not rebind outer BindingId

outer rebind through ScopeBox:
  outer BindingId appears on outgoing port

conditional rebind:
  then updates; identity else preserves incoming value

nested loop outer rebind:
  child summary exposes outer BindingId without parent AST rescan

heap mutation:
  arr[i] write is non-binding effect; arr binding remains invariant

binding rebind:
  arr = new Array() enters state domain

continue:
  updated a and incoming b are both required at edge

break:
  updated result reaches after; header-false carries current header result

return-only write:
  return port owns result; header state excludes it

nested exit targets:
  inner break/continue resolve to the inner loop RegionId exactly once

invalid exit placement:
  break/continue outside a loop reject during resolution

multiple condition/state variables:
  no selected progression owner
```

## Required gates

```text
canonical BindingId owner count = 1
one active BindingId allocator per function = 1
resolved semantic product partial publication = 0
resolved semantic product AST clone ownership = 0
origin/path used as lexical equality authority = 0
raw ID used for Rust/Hako parity = 0
RegionFlow BindingId allocation = 0
name-keyed state authority = 0
resolved target depth recount = 0
normalized AST clone publication = 0
child region parent-rescan = 0
source omission/duplicate = 0
scope-local escape = 0
every edge state contract total
generic body filtering = 0
planner/Lower connection before R5/R6 = 0
legacy/new differential green before R8
legacy caller-zero before R9
all source files < 800 lines
```

SA0 focused guard must print at least:

```text
semantic_arena_schema=present
semantic_arena_ast_clone_fields=0
semantic_arena_binding_allocator_calls=0
semantic_arena_value_id_imports=0
semantic_arena_basic_block_id_imports=0
semantic_arena_external_consumers=0
semantic_arena_planner_connection=0
semantic_arena_lower_connection=0
semantic_arena_source_files_under_800=1
ownership_private_binding_id_imports=0
summary=ok
```

## May claim after R5

```text
binding identity is independent of names
scope effects compose recursively
nested regions export verified outer-binding effects
generic loop state is a set, not one selected variable
binding rebind and non-binding effects are separate
every declared loop edge has a complete binding contract
```

## May claim after SA2

```text
the declared syntax subset has a deterministic, sealed shadow semantic graph
binding/scope/region provenance is normalized without raw-ID parity
shadowing and exact control-target resolution are independently verifiable
production BindingId allocation and Lower behavior remain unchanged
```

## May claim after SA5

```text
supported functions have one pre-plan lexical/control semantic authority
Lower allocates ValueIds but not BindingIds
Lower materializes RegionIds but does not rediscover exit targets
the private ownership BindingId allocator has no production caller
```

## Must not claim

```text
complete RVSDG implementation
no binding tracking is needed
BindingId and ValueId are interchangeable
all closures/captures or condition-side rebinds supported
all fini/cleanup shapes supported
liveness optimization complete
raw BindingId stable across frontends
Recipe verification alone proves state closure
canonical step extraction is generic semantics
ResolvedFunctionV1 replaces the canonical AST
BindingOrigin or source path is binding equality authority
SA1 shadow ordinals are production lexical identities
Lower-time BindingId allocation retired before SA3 is actually complete
exit depth retired before SA4 equivalence and caller-zero gates are green
```

## Stop conditions

1. RegionFlow allocates or reconstructs BindingIds.
2. Shadow-name sets decide lexical identity.
3. Private ownership BindingId remains an unexplained second authority.
4. RegionEffectAnalyzer allocates ValueIds or simulates SSA versions.
5. Lower performs name resolution or AST effect analysis.
6. Parent rescans nested-loop AST instead of consuming a verified summary.
7. Scope child bindings appear in a parent port.
8. Complex lvalue becomes binding rebind without Place proof.
9. Continue/break omits a required binding.
10. Target depth is recalculated after RegionId resolution.
11. Source projection publishes a cloned normalized AST.
12. Generic acceptance selects one progression owner.
13. Dead-state optimization mixes into correctness.
14. Canonical-step optimization mixes into state closure.
15. New Recipe/CFG vocabulary is required without a separate BoxShape card.
16. ProgramV0 or parser/source-carrier P1 becomes authority.
17. Unsupported backend falls back to VM.
18. Any source file reaches 800 lines.
19. Resolver and Lower allocate distinct BindingIds for the same declaration.
20. A shadow ordinal enters Facts, Planner, Recipe, or Lower.
21. BindingOrigin/source path decides lexical equality.
22. Raw owner-local IDs are compared across Rust/Hako implementations.
23. `ScopeId` and `RegionId` are collapsed into one identity.
24. Control-target resolution allocates RegionIds.
25. A partial or poisoned resolved-function draft is published.
26. ResolvedFunction stores cloned AST nodes or pointer identity.
27. Unsupported resolver syntax retries the legacy Lower resolver.
28. A synthetic binding receives a fabricated source declaration path.
29. SA3 begins before the shadow resolver covers the production inventory.
30. Old and new target authorities silently disagree during SA4.
