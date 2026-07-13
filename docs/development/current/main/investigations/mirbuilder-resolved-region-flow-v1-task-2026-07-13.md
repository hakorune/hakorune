# Resolved Region Flow V1 — Taskboard

Status: Design consultation stop; R0 closed, pre-plan lexical identity owner required.
Date: 2026-07-13
Decision: `recursive_structured_region_flow`.
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
BindingId is `SchemaMismatchStop`.  R1 remains blocked until consultation
chooses lifted canonical BindingId resolution or an explicitly authoritative
structural BindingKey with checked Lower mapping.

### R1 — minimal resolved region view

Implement closed `Sequence/Scope/If/Loop/Statement/Exit` structural vocabulary
with canonical BindingId references and resolved target RegionIds.  Source
projection stores no AST clones.  Shadowing, scope exit, and source order are
fixed by focused fixtures.  Product connection remains zero.

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

multiple condition/state variables:
  no selected progression owner
```

## Required gates

```text
canonical BindingId owner count = 1
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

## May claim after R5

```text
binding identity is independent of names
scope effects compose recursively
nested regions export verified outer-binding effects
generic loop state is a set, not one selected variable
binding rebind and non-binding effects are separate
every declared loop edge has a complete binding contract
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
