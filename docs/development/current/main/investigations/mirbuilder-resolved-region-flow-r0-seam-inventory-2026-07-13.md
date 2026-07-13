# Resolved Region Flow V1 — R0 Seam Inventory

Status: Closed inventory; consultation resolved by owner-scoped semantic arena.
Date: 2026-07-13
Classification: two missing pre-plan resolution owners plus one duplicate authority.

## Executive result

```text
canonical lexical BindingId type/allocator:
  exists

complete resolved binding view at Facts/Planner time:
  missing

resolved break/continue RegionId owner:
  missing

join_ir/ownership private BindingId:
  SchemaMismatchStop
```

R1 cannot consume resolver-owned binding and target identities without a new
pre-plan seam.  RegionFlow must not fill the gap with names, private IDs,
planner-side allocation, or Lower-time rediscovery.

## Canonical lexical identity

Canonical type and allocator:

```text
type:
  hakorune_mir_core::BindingId

allocator:
  CoreContext::next_binding
  -> MirBuilder::allocate_binding_id

current lookup:
  BindingContext String -> BindingId

declaration entry:
  MirBuilder::declare_local_in_current_scope
```

Canonical semantics are correct: BindingId is independent from ValueId,
shadowing allocates a new ID, and scope exit restores the outer ID.

## Timing mismatch

Generic-loop Facts are pure AST functions:

```text
try_extract_generic_loop_v1_facts(
  condition: &ASTNode,
  body: &[ASTNode],
)
```

They receive neither MirBuilder nor BindingContext.

Body-local canonical BindingIds are allocated later, while Recipe/Parts Lower
executes declarations through `declare_local_in_current_scope`.

Current snapshots:

```text
BindingContext::snapshot:
  current String -> BindingId map only

LocalBindingStateSnapshot:
  current String -> ValueId
  + current BindingContext
  private temporary-lowering rollback owner
```

Neither snapshot contains:

```text
declaration inventory
declaration SourceStmtSite
scope ancestry / stable ScopeId
body-local IDs before Lower
resolved assignment references for the whole region
```

Therefore `binding_ctx.snapshot()` is not the required read-only resolved
function seam.

## Scope owner

Current MIR lexical scope machinery correctly owns runtime builder state:

```text
LexicalScopeFrame:
  declared names
  previous ValueId mappings
  previous BindingId mappings

push/pop:
  ScopeContext lexical stack

scope exit:
  restore ValueId and BindingId maps together
```

It is a Lower-time stack, not an immutable pre-plan scope tree.  Debug scope
strings are diagnostics only and cannot become RegionId authority.

## Private ownership BindingId

`src/mir/join_ir/ownership/ast_analyzer` defines a private:

```text
BindingId(u32)
next_binding_id
env_stack<String, private BindingId>
```

It independently resolves parameters, locals, outbox, range binders, and
catch binders; records lexical reads/writes; and distinguishes shadowing.
There is no alias, conversion, canonical snapshot input, or raw-number
contract with `hakorune_mir_core::BindingId`.

It also differs on same-frame redeclaration: the private analyzer reuses an
existing ID while canonical declaration rejects redeclaration.

Classification:

```text
CanonicalAlias:
  no

CompatAdapterWithRetirement:
  no current adapter

IndependentNonLexicalIdentity:
  no; meaning is lexical identity

SchemaMismatchStop:
  yes
```

The private analyzer has no observed product caller outside its own module
tests.  Reusable structure is limited to recursive scope traversal,
parent links, child-local filtering, and bottom-up effect propagation.  Its
allocator and name resolver are forbidden inputs to RegionFlow.

## Control-target inventory

Source AST:

```text
Break { span }
Continue { span }
```

No label, depth, LoopId, or RegionId is stored.

Current production interpretation is distributed:

```text
Facts/AST utilities:
  recursively count/skip nested loops

Recipe producers:
  synthesize Break/Continue depth = 1

Recipe verifier:
  current capability rejects depth != 1

CoreExitPlan:
  carries numeric depth

Plan verifier:
  recomputes loop depth

Lower:
  resolves stack.len() - depth to BasicBlockId
```

No production owner binds an exit SourceStmtSite to one source loop identity.

Existing lookalikes are non-authority:

```text
hakorune_mir_core::LoopId:
  useful downstream vocabulary
  no source identity producer
  production JoinIR uses fixed LoopId(0) in observed seams

mir::region::RegionId:
  GC/debug observer
  global atomic/environment-controlled allocation
  forbidden as compilation structural identity

EdgeCFG target-qualified ExitKind:
  test-only composition seam in current inventory
```

## Required missing owner

The accepted architecture requires one pre-plan resolved function product:

```text
ResolvedFunctionV1 {
  binding inventory,
  immutable scope tree,
  region inventory,
  resolved binding references,
  resolved control targets,
  source sites,
}
```

Target resolution should be a private component of its construction:

```text
ResolvedControlTargetResolverV1

input:
  preassigned RegionId inventory
  source sites / canonical AST

state:
  active loop RegionId stack

output:
  Break/Continue -> exact target RegionId
  Return -> function RegionId
```

The resolver does not allocate RegionIds and does not emit Recipe depths,
BasicBlockIds, or ValueIds.

## Design consultation

Decision closed on 2026-07-13:

```text
function_semantic_resolver_v1_owner_scoped_arena

FunctionSemanticResolverV1
  -> VerifiedResolvedFunctionV1
       owner-scoped BindingId arena
       ScopeId arena
       RegionId arena
       resolved uses/assignments/control exits
  -> Planner / ResolvedRegionFlowV1 / Lower consumers
```

The existing canonical BindingId meaning is retained but allocation moves to
the pre-plan function resolver at the atomic authority cutover.  Structural
binding/source keys are provenance for diagnostics and Rust/Hako normalized
graph parity, not a second identity authority.  The private ownership
BindingId remains `SchemaMismatchStop` and is retired after its recursive
effect-walker shape is migrated to consume the sealed product.

The active implementation order and stop conditions are owned by:

```text
docs/development/current/main/investigations/
  mirbuilder-resolved-region-flow-v1-task-2026-07-13.md
```

Historical alternatives considered before the decision follow.  The choice is
closed; these are not active implementation options.

### Option A — lift canonical BindingId resolution before Planner

```text
canonical lexical resolver
  allocates hakorune_mir_core::BindingId once
  builds immutable ResolvedFunctionV1

Planner/Facts:
  read only

Lower:
  adopts resolved IDs; allocates ValueIds only
```

Questions:

```text
how existing CoreContext allocation moves or is shared
how parameters/synthetic locals are handled
how legacy Lower-time declaration avoids double allocation
how numeric-ID behavior and metadata remain compatible
```

### Option B — structural BindingKey is pre-plan semantic identity

```text
BindingKeyV1 = declaration SourceStmtSite + binding ordinal/kind

Planner/Facts:
  use BindingKeyV1

Lower:
  constructs a checked BindingKeyV1 <-> canonical BindingId bijection
```

Questions:

```text
whether this intentionally supersedes BindingId as semantic authority
whether it duplicates existing lexical identity
how synthetic bindings and cross-frontend parity are versioned
```

### Rejected directions

```text
C: Facts/Planner mutates MirBuilder allocator
  effectful classification and order-dependent IDs

D: RegionFlow allocates private analysis BindingIds
  repeats ownership SchemaMismatch

E: defer resolution to Lower
  violates plan-before-Lower and forces rediscovery

F: shadow-name sets as identity
  fails lexical authority and shadowing contract
```

RegionId strategy must be chosen with A/B: structural region keys may be
canonical source-relative identity with compilation-local RegionId handles,
but no existing production RegionId owner can be silently reused.

## Implementation may claim

```text
canonical MIR BindingId owner is singular today
current BindingContext owns live name lookup and scope restoration
no complete resolved binding view exists before generic Facts
private ownership BindingId is a duplicate lexical authority
no resolved source control-target owner exists in production
R1 requires an explicit pre-plan resolution decision
```

## Implementation must not claim

```text
BindingContext snapshot is a resolved function tree
private ownership BindingId numbers correspond to canonical IDs
LoopId or debug RegionId already owns source loop identity
Recipe depth is resolved target identity
R1 may safely infer bindings from names
```

## Stop conditions

```text
R1 starts before A/B decision
RegionFlow allocates IDs
Facts receives mutable MirBuilder only to allocate identity
private ownership BindingId is adapted without explicit mapping/retirement
Lower continues target-depth recount while resolved-target claim is made
```
