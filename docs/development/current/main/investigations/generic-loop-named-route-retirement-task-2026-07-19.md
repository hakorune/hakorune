# Generic Loop Named-Route Retirement Task

Status: Parked; decision locked
Date: 2026-07-19
Scope: post-`EXPR0-C0` loop route-authority closeout only
Current active blocker delta: 0
Production behavior delta at taskization: 0

## Decision

The final cleanup target is one callable loop authority with zero redundant
named-route selection authority. This task does not predeclare that every file
whose name starts with `loop_cond_` must be deleted.

```text
retire:
  redundant named-route predicates
  redundant registry entries
  redundant candidate suppression
  route-specific production authority proven covered by GenericLoopV1

retain or move to a neutral owner when still shared:
  cleanup mechanics
  PHI mechanics
  carrier mechanics
  CFG and verification mechanics
```

`Recipe` / `LoweredRecipe` is a common output representation. It does not by
itself prove that `GenericLoopV1` has replaced every producer. In the current
tree, `loop_cond_break_continue` remains an active recipe owner for shapes such
as multiple continue branches with prelude effects. Therefore this task is a
proof-driven authority retirement, not a filename-driven deletion sweep.

## Activation boundary

This task is parked behind the current callable-result Loop sequence.

```text
LOOP0-I0a
  -> LOOP0-I0b
  -> LOOP0-L0
  -> EXPR0-C0
  -> LOOP-ROUTE-RET0-D0
  -> LOOP-ROUTE-RET0-P0
  -> LOOP-ROUTE-RET0-CUT0
  -> LOOP-ROUTE-RET0-G0
```

Taskization does not activate `LOOP-ROUTE-RET0-D0`. After `EXPR0-C0`, an
explicit `CURRENT_STATE.toml` selection is required before any retirement
implementation begins.

The current LOOP0 rows may establish reusable selection, located composition,
source provenance, claim, and emission authorities. They must not
opportunistically delete named routes or change raw fallback behavior.

## Current evidence

The existing re-aggregation board selects named-route/suppression retirement
as the long-term direction, but its first concrete candidate was registry
suppression rather than a lowering route.

The later multi-delta owner selection proves that `loop_cond_break_continue`
is still required by at least one admitted production shape. Consequently:

```text
named-route debt exists = yes
final authority consolidation intended = yes
loop_cond_bc wholesale deletion authorized now = no
current active named-route callers = nonzero
```

The final retirement decision must consume fresh post-`EXPR0-C0` evidence. Old
candidate-only observer output is not sufficient authority for deletion.

## Final authority law

After G0, each admitted Loop source shape has exactly one route-selection and
execution authority.

```text
frozen loop facts / exact located carrier
  -> one ordered selection owner
  -> one selected composition owner
  -> one verified CorePlan
  -> one emission owner
```

Forbidden final states:

```text
GenericLoopV1 and a named route both own the same admitted shape
registry suppression decides semantic ownership
selected-route failure retries another route
route selection depends on historical fixture or method names
dead route code remains reachable as a hidden compatibility fallback
```

A retained helper is not a retained route authority when all of the following
hold:

```text
it has no registry entry or route predicate
it cannot select a source shape
it is called only through the final selected producer
its contract is structural and route-neutral
its tests no longer claim a named-route semantic owner
```

## `LOOP-ROUTE-RET0-D0` — exact post-cutover inventory

Behavior delta: 0.

Inventory every active Loop surface after `EXPR0-C0`:

```text
LoopRouteId rows and ordered registry entries
route predicates
candidate suppression branches
route handler functions
recipe composers and normalizers
facts products consumed by each route
production and test callers
accepted fixture families
selected-route traces
fallback / Ok(None) continuation behavior
shared cleanup / PHI / carrier / verifier mechanics
```

For each named route, record one disposition:

```text
RETIRE_CANDIDATE:
  GenericLoopV1 covers the exact source, plan, MIR, runtime, and failure law

ACTIVE_UNIQUE_OWNER:
  an admitted shape still requires route-specific semantics

SHARED_MECHANIC:
  reusable implementation with no route-selection authority

DEAD_SURFACE:
  production callers and selected fixtures are zero
```

D0 must use actual selected-route evidence. Predicate match, observer
candidate, file count, and historical fixture names are non-authorities.

D0 exit:

```text
unclassified registry entries = 0
unclassified suppression branches = 0
unclassified production callers = 0
unclassified shared mechanics = 0
retirement candidates form one bounded CUT0 batch
```

If no route is safely removable, D0 closes as an inventory and parks CUT0. It
must not manufacture a candidate to keep the sequence moving.

## `LOOP-ROUTE-RET0-P0` — normalized replacement proof

Behavior delta: 0.

For every `RETIRE_CANDIDATE`, compare the current named route against the final
GenericLoopV1 authority over the same exact fixture domain.

Required parity:

```text
accepted and rejected source shapes
route facts and recipe disposition
CorePlan normalized structure
CFG topology
Binding SSA / PHI relation
call-source site inventory
cleanup and exit disposition
MIR instructions and contracts
runtime result and typed failure
debug / release behavior
ownership-operation counts
selected failure with no retry
```

Parity is semantic and normalized. ValueId, BasicBlockId, allocation identity,
temporary object count, and diagnostic prose are not identities.

P0 must include every fixture currently selecting the candidate route, not
only overlapping fixtures where GenericLoopV1 was already preferred.

P0 exit:

```text
candidate production selection count understood = exact
candidate fixture coverage = complete
semantic/MIR/runtime differences = 0
fallback dependence = 0
shared-mechanic extraction plan = sealed when needed
```

## `LOOP-ROUTE-RET0-CUT0` — atomic authority retirement

CUT0 removes one bounded, fully proven route-authority batch in one commit.

Order:

```text
1. route every proven fixture through the final GenericLoopV1 authority
2. remove candidate registry entries
3. remove candidate predicates
4. remove candidate suppression branches made dead by the cutover
5. remove candidate handlers/composers/facades
6. move still-shared mechanics behind neutral names and owners
7. delete route-specific mechanics with caller count zero
8. migrate tests from route-name assertions to final authority assertions
9. verify no hidden retry or compatibility selector remains
```

Do not land a production commit where both authorities own the same shape.
Mechanical neutral-helper extraction may use a short BoxShape refactor series
before CUT0 only when each commit is behavior-neutral, buildable, and keeps the
existing route as the sole production authority until the atomic cutover.

CUT0 must not broaden the accepted source grammar. Any missing GenericLoopV1
shape is a separate BoxCount row completed before retirement resumes.

## `LOOP-ROUTE-RET0-G0` — zero guards and closeout

Behavior delta: 0.

Final guards:

```text
retired LoopRouteId rows = 0
retired registry entries = 0
retired predicates = 0
retired suppression branches = 0
retired route handlers = 0
retired production callers = 0
retired fixture route-name assertions = 0
hidden legacy fallback / retry = 0

one ordered route-selection owner = 1
one selected execution owner per admitted shape = 1
GenericLoopV1 selected failure retry count = 0
unclassified shared mechanics = 0
```

G0 updates route registry documentation and stable public guards. It does not
add a shell guard per retired symbol; prefer the existing reusable Loop lane
guard or one manifest-backed zero inventory.

## Required proof fixtures

The final inventory determines the exact list. At minimum preserve coverage
for:

```text
simple while-style loops
break-only and continue-only loops
break/continue combinations
multiple continue branches with prelude effects
return-in-body and continue-with-return
nested acyclic If recipes
conditional update and carrier PHIs
short-circuit loop conditions
nested Loop shapes currently admitted by raw routes
located callable-result Loop source-site coverage
```

Negative coverage:

```text
ambiguous route ownership
selected failure followed by fallback
candidate predicate match without executable coverage
observer-only evidence used as deletion authority
foreign or missing source provenance
unsupported normalized shadow
unsupported nested source shape
```

## Stop conditions

Stop before CUT0 when any of the following is observed:

1. A candidate route still owns an admitted shape not represented by
   GenericLoopV1.
2. Normalized CorePlan, CFG, Binding SSA, call-site, MIR, runtime, or typed
   failure parity differs.
3. Removing the route requires fallback or selected-route retry.
4. Route identity must be reconstructed from method names, fixture names,
   spans, ValueIds, block IDs, or emission order.
5. A shared cleanup, PHI, carrier, or verifier mechanic cannot be separated
   from route-selection policy without a new design decision.
6. Retirement requires accepting a new source shape in the same row.
7. Raw and located route selection need separate ordered policy tables.
8. A route-specific compatibility facade would retain the old authority.
9. The cutover would land a state with two production owners for one shape.
10. Runtime, backend, ownership, parser, or language semantics must widen.

When a stop condition identifies a missing GenericLoopV1 shape, open exactly
one BoxCount task for that shape and keep this retirement task parked. When it
identifies inseparable shared mechanics, open one BoxShape design task instead
of deleting or duplicating the mechanic.

## Implementation may claim after G0

```text
every retired source domain has one final GenericLoopV1 route authority
retired named-route predicates, entries, suppressions, and callers are zero
selected-route failure never retries a retired compatibility route
shared mechanics retained after cutover have no source-selection authority
route-name historical structure no longer determines semantic ownership
```

## Implementation must not claim

```text
every file containing loop_cond is deleted
all Loop shapes use one identical recipe representation
all loop-specific cleanup / PHI / carrier code is obsolete
new Loop grammar or source-shape support
general normalized-shadow support
runtime, backend, or ownership widening
performance improvement without measurement
source compatibility for private/internal route names
```

## Final decision lock

> After `EXPR0-C0`, the Loop lane must run one explicit retirement closeout
> rather than leaving GenericLoopV1 beside an indefinite set of named route
> authorities. D0 inventories actual selected production routes, suppressions,
> callers, fixtures, and reusable mechanics. P0 authorizes retirement only with
> complete normalized source/plan/CFG/Binding-SSA/site/MIR/runtime/failure
> parity. CUT0 atomically removes each proven route authority and its redundant
> registry/predicate/suppression surfaces while retaining or moving genuinely
> shared cleanup, PHI, carrier, and verifier mechanics behind neutral owners.
> G0 fixes caller and fallback zero. `loop_cond_bc` naming is not itself deletion
> authority, GenericLoopV1 existence is not replacement proof, and any missing
> shape or semantic difference stops retirement for a separate BoxCount or
> BoxShape task. Taskization changes no current blocker or production behavior.
