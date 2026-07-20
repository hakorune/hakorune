---
Status: Parked architecture task
Date: 2026-07-20
Scope: JoinIR-to-D-prime succession, parity, diagnostic rehome, and retirement
Parent:
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
  - mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
Current blocker: FINALIZE0-CENSUS0 remains authoritative; this card does not activate a JoinIR cutover
---

# JoinIR -> D-prime succession and retirement

## Decision

JoinIR is not retained as a permanent checker. It serves only as a temporary
parity oracle while D-prime takes over canonical Loop lowering. Durable
diagnostic laws and fixtures move to their natural owners. Canonical
carrier/PHI authorities retire after canonical caller zero. JoinIR namespaces,
CorePlan, and compatibility harnesses retire physically only after their
separate repository-wide callers reach zero.

The final boundary is:

```text
canonical source
  -> carrier-free verified control contract
       exact source coverage
       family topology and typed ports
       exact exit targets
       cleanup obligations
       typed unsupported preflight
  -> D-prime FunctionLoweringSession
       CanonicalCfgSessionV1
       one function-owned BindingSsaBuilderV1
       demand-driven PHI construction
  -> verified MIR SSA
  -> optional read-only BlockArgument / Join view
```

The historical idea of expressing joins as function or block arguments is not
rejected. Its placement changes:

```text
rejected:
  pre-lowering carrier/join argument inventory as an input obligation

accepted current representation:
  demand-driven MirInstruction::Phi produced by Binding SSA

parked optional representation:
  verified MIR SSA -> derived BlockParams + edge arguments
```

The optional derived view is never a Builder input, route-selection truth,
second SSA authority, or prerequisite for JoinIR retirement.

## Current reality: two JoinIR ledgers with dependency edges

The word `JoinIR` currently names two physically and operationally different
systems. Retirement must not merge their caller inventories. They are not yet
independent: active Builder/CorePlan code imports legacy `join_ir::lowering`
vocabulary such as `JoinInlineBoundary`, layout, and carrier types. Census must
record those cross-import edges before either ledger claims independent
retirement.

### A. Active Builder CorePlan/Recipe Loop ecosystem

Production Loop lowering currently follows:

```text
MirBuilder::cf_loop
  -> try_cf_loop_joinir
  -> route_loop
  -> Facts / RecipeComposer
  -> CorePlan / PlanVerifier
  -> PlanLowerer
  -> MIR
```

2026-07-20 census snapshot:

```text
src/mir/builder/control_flow/plan/
  Rust files = 383
  Rust lines = 74,393

src/mir/builder/control_flow/joinir/route_entry/
  Rust files = 12
  Rust lines = 2,230
```

This is the active production Loop authority. D-prime Loop S3-prime/I1-prime/
I2-prime is designed but is not yet the production replacement.

### B. Legacy JoinModule ecosystem

This ecosystem owns `JoinModule`, continuation-style Join functions,
target-specific lowerers, JSON, runners, and VM bridges. It is not the current
CorePlan-to-MIR Loop route.

2026-07-20 partial census snapshot:

```text
src/mir/join_ir/frontend/   = 5,546 Rust lines
src/mir/join_ir/lowering/  = 16,917 Rust lines
src/mir/join_ir/ownership/ = 2,829 Rust lines
```

Its production, explicit-environment, LowerOnly observation, test, docs, JSON,
runner, macro compatibility, and VM bridge callers get a separate ledger.
Whether it can reach physical caller zero before the active CorePlan Loop
ecosystem is an evidence question for the cross-import census, not an assumed
ordering law.

## Succession matrix

### Move to D-prime control contracts

Move the semantic law and fixtures, not old value-flow products:

```text
exact source coverage
family-specific CFG topology
typed Break / Continue / Return / Throw ports
exact control targets
ordered cleanup obligations
unsupported-shape preflight
stable source-site diagnostics
route fixture source programs
```

Pre-Builder D-prime products must not contain:

```text
ValueId
BasicBlockId
carrier rows
may_rebind summaries
PHI rows
name-keyed final values
manual predecessor values
runtime or backend fallback authority
```

### Move to generic MIR/SSA owners

The following laws survive without importing JoinIR types:

```text
unique SSA definition
PHI destination not redefined
every PHI input value is defined and edge-available
PHI predecessor rows exactly match reachable predecessors
terminator targets exist
no predecessor is added after block seal
no incomplete/provisional PHI remains at function finish
ownership PHI edge transfer is valid
CFG / SSA / dominance / RC verification
```

Natural owners:

```text
CanonicalCfgSessionV1
BindingSsaBuilderV1::finish
shared PHI lifecycle
MirVerifier
Ownership SSA verifier
```

JoinIR-specific debug assertions and generic verifiers must not both survive as
permanent authorities for the same law.

### Retire rather than transplant

Delete after the corresponding caller-zero seal:

```text
precomputed carrier lists
manual LoopForm carrier classification
name-keyed final_values
CorePhiInfo and route-specific PHI rows
preallocated plan ValueId / BasicBlockId value-flow authority
manual header / latch / exit PHI construction
carrier ordering and cardinality cross-checks
boundary carrier vs header-PHI layout checks
exit_bindings vs carrier_inputs checks
ContinueWithPhiArgs / BreakWithPhiArgs carrier arrays
JoinInlineBoundary value-flow authority
reserved PHI ValueId regions and observers
JoinIR-only Select verifier
thread-local planner reject-detail side channel
raw suffix/value-flow protocols after caller zero
```

Do not replace any of these with a D-prime carrier list. The sole reaching-value
chain is:

```text
BindingRef use
  -> BindingSsaBuilderV1::read
  -> exact sealed predecessor set
  -> demand-driven PHI when required
```

## Diagnostic inheritance law

### Source/control rejection

Replace time-dependent planner reject side channels with a typed located
preflight result:

```rust
enum LocatedControlDecisionV1<T> {
    Accepted(T),
    Rejected(LocatedControlRejectV1),
}
```

The reject product may own:

```text
exact source site
control family
stable reason tag
unsupported boundary
one-line next hint
```

It must not own:

```text
ValueId
carrier or PHI prediction
Builder state
fallback or retry authority
thread-local state
```

Required proof:

```text
first structural rejection is deterministic
source declaration reorder does not change the stable reason
reject before Builder effects
fallback / retry after selected reject = 0
foreign source carrier rejects
debug OFF retains the stable freeze tag
method/function-name special cases = 0
```

The current `planner_reject_detail` side channel is shared by JoinIR routing,
plan facts, and `single_planner`; it is not deleted merely because a JoinIR
caller count reaches zero. Its removal is gated by the `PLAN0` typed-preflight
cutover and exact remaining-consumer zero.

### Reserved-PHI diagnostics

`verify_phi_reserved` and its `0..99`, `100..999`, and `1000+` regions are
JoinIR allocation-strategy checks, not semantic SSA laws. They retire.

Their durable intent is covered only by generic fixtures:

```text
duplicate PHI destination rejects
ordinary instruction cannot redefine a PHI destination
reserve-only value never escapes as a definition
provisional PHI outstanding at finish rejects
arbitrary ValueId numbering accepts when SSA is valid
```

No replacement global reserved-region observer is created.

### Zero-progress diagnostics

The existing `verify_progress_for_skip_ws` is not generalized in place. Its
current authority is target-specific:

```text
loop function = JoinFuncId(1)
progress parameter = params[2]
progress observation = an Add using that parameter as lhs before recursion
```

It does not prove that the Add result is the backedge argument. The function is
therefore deleted with the legacy JoinModule verifier unless a named consumer
selects a new post-MIR analysis row.

A future post-MIR progress diagnostic may inspect:

```text
completed CFG loop header / latch
condition dependency graph
header PHIs used by the condition
Copy/Phi-normalized latch incoming values
external Call / field / memory effects capable of changing the condition
```

It must not classify every loop whose PHI inputs are unchanged as invalid.
Field mutation, calls, external state, and other effects can make progress
without changing a local header PHI. The first safe diagnostic admission is:

```text
condition is pure
condition depends on at least one header PHI
all condition-relevant backedge values preserve their entry base
condition-affecting external effects = 0
  -> SuspiciousZeroLocalProgress
```

This begins as a stable diagnostic. A hard compilation error requires a
separate proof that the loop condition observes only closed local state.

## Parity law

The old CorePlan route is a temporary oracle only. It never becomes semantic
truth for D-prime and is never executed twice in the production module.

Old and new lowering run in separate, unpublished function sessions during
proof. Failure or mutation in one draft cannot affect the other.

### Compare

Normalize physical identities before comparison and seal:

```text
CFG blocks and edges
terminator and target semantics
instruction operation and effect order where observable
PHI predecessor-to-value relation
Call / field / memory effects
Break / Continue / Return / cleanup behavior
type facts
ownership instructions and edge transfers
CFG / SSA / dominance / RC verifier outcome
runtime / VM result and observable state
backend fail-fast category
stable source diagnostic category
```

The D-prime draft separately proves its internal `BindingRef` reaching-value
relation through `BindingSsaBuilderV1`. Cross-route parity does not reconstruct
`BindingRef` identity from legacy MIR, because emitted MIR is not source
semantic authority.

### Do not compare as authority

```text
raw ValueId numbers
raw BasicBlockId numbers
preallocated unused blocks
PHI count when a demand-driven equivalent is valid
trivial same-input/self PHI presence before simplification
unobservable instruction ordering
old route or recipe name
carrier list spelling/order
```

Any normalized comparison must preserve real predecessor/value relationships;
it cannot erase a semantic mismatch merely by renumbering.

### First parity profile

```text
simple while
eager non-short-circuit condition
fallthrough-only body
multiple independently updated bindings
condition-only binding
loop-invariant binding
body-local non-leakage
same-name shadowing
branch-dependent update
zero / one / multiple iterations
```

Later existing D-prime rows extend parity only with the source/control family
they admit:

```text
N1: nested If inside Loop
N2: Loop inside If
N3: Loop inside Loop
N4: bounded depth-independent nesting
EXIT-I1..I7: Continue / Break / Return closure
later expression capability rows: short-circuit and effectful conditions
```

Filter, multi-exit, and other historical recipe fixtures map onto those
existing structural rows; they do not create a second widening task family.
Zero-progress fixtures belong only to the optional future post-MIR diagnostic
row and are not a parity prerequisite.

For every D-prime-admitted profile:

```text
normalized semantic/MIR parity = green
runtime parity = green
CFG/SSA/dominance verifier parity = green
ownership/RC parity = green
selected D-prime failure -> legacy retry = 0
```

Legacy-only profiles remain explicitly inventoried; unsupported D-prime
profiles reject during whole-owner capability preflight before Builder effects.

## Fixed task order

This card refines existing clean-architecture and D-prime rows; it creates no
second control-widening roadmap. It does not replace or reorder the current
FINALIZE0 blocker.

```text
current clean-architecture prerequisite:
  FINALIZE0
    -> METAPROP0
    -> PLAN0
    -> PLAN0-RECIPE-RET0
    -> RAWADAPT0

read-only work that may be scheduled when explicitly selected:
JOINIR-DPRIME-BRIDGE0-D0
  -> JOINIR-RET0-CENSUS0
  -> JOINIR-LEGACY-CENSUS0

canonical Loop path:
  JOINIR-RET0-CENSUS0
    -> B0-L4-S3′
    -> B0-L4-I1′

  PLAN0-G0 + PLAN0-RECIPE-RET0 + RAWADAPT0-G0 + B0-L4-I1′
    -> JOINIR-PARITY0-S0
    -> JOINIR-PARITY0-P0
    -> B0-L4-I2′

  existing D-prime widening rows only:
    N1 -> N2 -> N3 -> N4
    -> EXIT-S0 -> EXIT-S1 -> EXIT-S2
    -> EXIT-I1 -> EXIT-I2 -> EXIT-I3 -> EXIT-I4
    -> EXIT-I5 -> EXIT-I6 -> EXIT-I7

  whole-source/default-route and retirement path:
    F0 -> selected F1 owner-family rows
    -> RET-I1 -> RET-I2
    -> F2-S0 -> F2-I1
    -> JOINIR-DIAG-REHOME0
    -> PUB-F0
    -> RET-R1 / JOINIR-MANUAL-AUTH-RET0
    -> JOINIR-CANONICAL-G0
    -> conditional RET-R2 / JOINIR-COREPLAN-RET0

legacy JoinModule path:
  JOINIR-LEGACY-CENSUS0
    -> per-family caller-zero rows
    -> JOINIR-LEGACY-RET0

repository-wide optional physical closeout:
  conditional RET-R2 complete + JOINIR-LEGACY-RET0 complete
    -> JOINIR-REPOSITORY-RET-G0

parked optional:
  MIR-BLOCKARGS0-D0
```

`PLAN0-G0` above means the selected plan boundary is stable and its parity
fixture owner is frozen. Parity must not use a moving CorePlan implementation
as its oracle. `EXIT-P0` remains separately parked as in the parent D-prime
taskboard; this card does not activate labeled/outer-Loop targets.

### `JOINIR-DPRIME-BRIDGE0-D0`

This decision card. Code delta and route delta are zero. It fixes the two
caller ledgers plus their cross-dependencies, succession matrix, parity law,
diagnostic destinations, stop conditions, and physical retirement boundaries.

### `JOINIR-RET0-CENSUS0`

Refine the existing D-prime `RET-P0` inventory without modifying its frozen
SSA evidence. Classify every active CorePlan/Recipe/JoinIR caller as:

```text
canonical production source
explicit BareAst legacy
ProgramV0
REPL
test-only
observation-only
dead
```

Inventory at least:

```text
LoopRouteContext and route registry consumers
CorePlan constructors, clones, remappers, verifiers, and lowerers
Facts / Recipe / Parts source-shape owners
CorePhiInfo / phis / final_values
manual carrier and PHI materializers
raw suffix protocols
planner reject side channels and error tags
generic MIR verifier overlap
imports between active CorePlan and legacy `join_ir::lowering` vocabulary
fixtures, guards, manifests, and docs pointers
```

Production behavior delta is zero.

### `B0-L4-S3′`

Close one carrier-free located Loop control contract owning only exact source
coverage, topology, ports, targets, cleanup, and typed unsupported errors.

Production Loop activation remains zero.

### `B0-L4-I1′`

Build one disconnected Loop CFG transaction over the shared canonical CFG and
function-owned Binding SSA substrate. It owns no carrier snapshots, manual PHI
rows, name-keyed final values, or legacy retry.

Production Loop activation remains zero.

### `JOINIR-PARITY0-S0`

Add one disconnected normalized parity vocabulary and comparator over two
separately unpublished verified drafts. It may not publish either draft or
select production routing. Start only after the `PLAN0`/Recipe/RAWADAPT
boundary freezes the legacy baseline and names the fixture owner.

### `JOINIR-PARITY0-P0`

Close the first common Loop profile against normalized CFG/value/effect/
cleanup/type/ownership/runtime relations. Raw identity equality is a negative
fixture. The old route remains a temporary oracle only.

### `B0-L4-I2′`

Atomically cut one complete canonical whole-owner profile to D-prime. Route
selection occurs before Builder effects. Selected D-prime failure poisons and
discards its draft; legacy retry and partial publication are zero.

### Existing `N1` / `N2` / `N3` / `N4` / `EXIT-*` rows

Move one structural control profile per existing D-prime row. Each admitted
row follows:

```text
located control contract
-> disconnected D-prime materialization
-> normalized old/new proof
-> whole-owner atomic cutover
-> caller/route guard
```

Do not preserve one route box per historical recipe when the profile composes
from common Sequence/If/Loop/Exit/Action vocabulary. BoxCount and BoxShape do
not share a row.

### `JOINIR-CANONICAL-G0`

Seal:

```text
canonical production JoinIR/CorePlan lowering callers = 0
canonical manual carrier/PHI callers = 0
canonical name-keyed final-value callers = 0
canonical failure -> legacy retry = 0
```

Explicit ProgramV0, REPL, or BareAst compatibility callers remain branded and
cannot be counted as canonical callers.

This is the canonical succession closeout. It does not claim repository-wide
CorePlan, JoinModule, runner, bridge, or compatibility deletion.

### `JOINIR-DIAG-REHOME0`

Move stable error tags, located rejection, generic CFG/SSA/PHI/ownership laws,
and retained fixtures to neutral owners. Delete reserved-ID observation,
target-specific Select verification, and obsolete JoinIR debug assertions once
their natural owner proves the same law. Delete thread-local reject detail only
after `PLAN0` typed preflight owns every remaining producer/consumer and its
exact caller count is zero.

The optional zero-progress successor requires a separate named consumer and
task; it is not silently recreated here.

### `JOINIR-MANUAL-AUTH-RET0` / parent `RET-R1`

After canonical manual-authority caller zero, delete:

```text
manual If/Loop carrier classification
name-keyed final_values
route-specific PHI materializers
caller-zero value-flow bridge products
```

This behavior-neutral row does not imply that the `CorePlan` type itself has
repository-wide caller zero.

### `JOINIR-COREPLAN-RET0` / parent conditional `RET-R2`

After repository caller zero for the affected active surfaces, physically
remove in bounded, buildable commits:

```text
route-specific carrier/value-flow code
manual PHI materializers
CorePhiInfo / final_values authority
JoinIR merge/boundary/header/exit PHI machinery
obsolete route recipes and registry rows
raw suffix protocol when its caller count is zero
CorePlan / LoopRouteContext only when complete repository callers are zero
```

If an explicit legacy input still depends on CorePlan, isolate it and claim
canonical caller zero only. Do not overclaim global deletion.

### `JOINIR-LEGACY-CENSUS0`

Independently inventory:

```text
JoinModule / JoinFunction / JoinInst
frontend and target-specific lowerers
ownership bridge
JSON and macro compatibility surfaces
join_ir_runner
join_ir_vm_bridge and dispatch
join_ir_ops
explicit environment variables
Exec vs LowerOnly observation rows
tests and docs
```

This row may begin before D-prime Loop completion, but it changes no behavior
and does not delete active LowerOnly observation routes.

### `JOINIR-LEGACY-RET0`

Delete each legacy family only after its independent production, explicit-env,
observation, test, docs, and external-format callers reach zero or migrate to a
named replacement. Exec and LowerOnly retirement are separate evidence rows.

### `JOINIR-REPOSITORY-RET-G0`

Only after conditional parent `RET-R2` and every legacy JoinModule family are
physically complete, freeze repository-wide definition/caller zero,
diagnostic destination, fixture/manifest replacement, docs pointer updates,
environment-variable retirement, no fallback, and no second SSA/value
authority. If branded ProgramV0, REPL, BareAst, explicit-env, or LowerOnly
callers remain, this row stays open while `JOINIR-CANONICAL-G0` may be green.

### `MIR-BLOCKARGS0-D0` — parked optional row

Only after D-prime SSA and JoinIR retirement are stable, decide whether a
derived read-only view is useful for dump, normalized comparison, or analysis
exchange:

```text
verified MIR SSA
  -> BlockParams
  -> edge arguments
```

The first row creates no second stored IR and no backend/runtime consumer.
Changing physical MIR to block arguments would be a separate broad migration
covering JSON, printer, VM, optimizer, verifier, LLVM/backend, and ownership
edge semantics.

## Required guards and counters

```text
pre-Builder carrier row definitions on D-prime route = 0
pre-Builder ValueId / BasicBlockId on D-prime route = 0
function-owned BindingSsaBuilderV1 instances = exactly 1 per canonical function

old/new comparison production double execution = 0
raw ValueId/BasicBlockId equality requirements = 0
old route used as D-prime semantic authority = 0

canonical selected D-prime failure -> legacy retry = 0
canonical partial module publication after failure = 0

thread-local planner reject authorities after rehome = 0
reserved-PHI numerical-region authorities after retirement = 0
carrier-list parity authorities after retirement = 0

legacy JoinModule and active CorePlan caller ledgers = 2 separate ledgers
cross-ledger imports classified before physical retirement = exact census
unbranded legacy compatibility callers = 0

derived BlockArgument Builder consumers = 0
derived BlockArgument route consumers = 0
second persistent SSA/value maps = 0

new method/function/owner-name special cases = 0
runtime/backend fallback = 0
new accepted source grammar in BoxShape retirement rows = 0
source/check files >= 800 lines = 0
```

## Implementation may claim

After `JOINIR-CANONICAL-G0`:

```text
canonical Loop lowering uses carrier-free control contracts and one
function-owned Binding SSA reaching-value authority

old CorePlan/JoinIR lowering served only as a temporary normalized parity
oracle for the canonical cutover

source/control laws, generic MIR/SSA laws, and retained fixtures were moved to
their natural owners before the old implementations were removed

canonical carrier-list, manual-PHI, and name-keyed-final-value production
callers are zero; mechanisms with repository caller zero are physically retired

no selected canonical failure retries a legacy route
```

Only after `JOINIR-REPOSITORY-RET-G0`:

```text
conditional CorePlan/LoopRouteContext repository retirement is complete

legacy JoinModule/runner/bridge families are physically retired through their
separate caller ledgers and classified cross-dependency edges

remaining diagnostics, fixtures, manifests, docs, and environment surfaces
have named non-JoinIR owners or exact deletion evidence
```

## Implementation must not claim

```text
JoinIR remains a permanent verifier namespace
all old and new MIR is byte-for-byte identical
all unchanged header PHIs prove an infinite loop
changing a header PHI proves termination
block arguments are the canonical physical MIR
all ProgramV0 / REPL / BareAst compatibility is retired
all target-specific LowerOnly observation is unnecessary
all JoinIR error tags are deleted
all CorePlan source-shape fixtures are discarded
PHI simplification is part of Binding SSA construction
optimizer loop facts are pre-Builder authority
```

## Stop conditions

Stop the row if any implementation requires:

1. a D-prime carrier, write-set, or final-value inventory;
2. ValueId or BasicBlockId in a verified pre-Builder control product;
3. source/route decisions reconstructed from emitted MIR;
4. exact raw MIR identity as the parity authority;
5. publishing either side of a parity comparison into the production module;
6. production execution through both old and new lowering;
7. legacy retry after a selected D-prime failure;
8. a second mutable predecessor, reaching-value, PHI, type, or ownership truth;
9. JoinIR-specific checks duplicated permanently beside generic verifiers;
10. every unchanged-PHI loop classified as a hard error;
11. reserved ValueId regions introduced into D-prime;
12. source names, method names, target spellings, spans, or AST equality used as semantic identity;
13. active CorePlan and legacy JoinModule callers merged into one completion count;
14. deletion of ProgramV0, REPL, explicit-env, or LowerOnly callers without independent evidence;
15. physical block-argument MIR migration inside JoinIR retirement;
16. BoxCount source-shape widening mixed with BoxShape retirement;
17. fallback, retry, runtime/backend widening, or ownership widening;
18. a new or modified source/check file reaching 800 lines.

## Completion definition

Canonical succession is complete at `JOINIR-CANONICAL-G0` when:

```text
all canonical source/control families selected by the compatibility threshold
lower through carrier-free D-prime control contracts

one BindingSsaBuilderV1 per canonical function owns every local BindingRef
reaching value and required PHI

canonical production CorePlan/JoinIR/manual-PHI callers are zero

remaining explicit legacy inputs, if any, are branded and isolated with no
canonical fallback edge

surviving canonical diagnostics use located-control or generic MIR/SSA owners

old carrier/final-value authorities are unreachable from canonical routes;
mechanisms with repository caller zero and their obsolete guards are removed

fixtures and manifests have named replacement owners

module publication remains atomic and verified
```

Repository-wide physical retirement is a stronger optional closeout. It is
complete only when:

```text
conditional parent RET-R2 is physically complete

legacy JoinModule production, explicit-env, observation, test, format, and docs
callers are zero or have named non-JoinIR replacements

cross-ledger imports are zero

JoinIR namespaces, runners, bridges, obsolete manifests, environment variables,
and compatibility harnesses have exact definition/caller zero
```

ProgramV0, REPL, BareAst, explicit-env, or LowerOnly callers may keep the
repository-wide closeout open without invalidating canonical succession.

## Immediate action

Do not start this card's code rows while `CURRENT_STATE.toml` names
`FINALIZE0-CENSUS0` as the active design stop. The next action here is only to
keep this task discoverable from the clean-architecture workstream. Execution
begins when the current macro dependency reaches the D-prime Loop/PLAN0 and
retirement boundary named by the parent taskboards.
