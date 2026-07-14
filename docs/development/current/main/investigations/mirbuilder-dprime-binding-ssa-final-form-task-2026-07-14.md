---
Status: Active — D0, B0-L4-S2′, and SSA-P0 closed; SSA-L0 selected
Date: 2026-07-14
Decision: D′ — SSA-first, control-contract-preserving, function-owner-atomic
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-L0-PHI-INPUT-MATERIALIZER-SPLIT-001
Work mode: Refactor Series Mode followed by bounded capability slices
Supersedes:
  - mirbuilder-b0-l4-a-a2prime-implementation-task-2026-07-14.md after its closed S1 slice
  - mirbuilder-resolved-control-flow-v2-final-form-extraction-task-2026-07-14.md as the effect-bearing final form
Retains:
  - closed B0-L4-S1 exact Loop/LoopBody identity bundle
  - closed B0-L4-S2′ generic located source range and coverage schema
Related:
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - mirbuilder-resolved-region-flow-v1-task-2026-07-13.md
  - mirbuilder-b0-l3b-a-plus-implementation-task-2026-07-13.md
  - mirbuilder-resolved-semantic-owner-forest-design-stop-2026-07-13.md
---

# D′ Binding SSA Final-Form Taskboard

Architectural authority lives in
`../design/binding-ssa-first-control-lowering-ssot.md`. This card owns work
order, acceptance gates, current blocker, and retirement sequencing.

## Objective

Make one function-scoped Binding SSA construction mechanism the only
production `BindingRefV1 -> ValueId` merge authority for canonical source
Lowering.

```text
canonical syntax
+ VerifiedResolvedFunctionV1
        │
        ├─ VerifiedLocated*ControlV1
        │    exact source coverage
        │    exact ScopeId / RegionId topology
        │    typed reachable ports and exact targets
        │    cleanup obligations
        │    no BindingRef effect or carrier rows
        │
        ▼
CanonicalSsaFunctionLowererV2
  ResolvedIdentityLedgerV2
  ResolvedSemanticStackV1
  ControlCoverageConsumptionV2
  BindingSsaBuilderV1
        │
        ▼
verified MIR SSA
        │
        ▼
optional post-MIR derived analysis
```

The final split is:

```text
resolver:
  lexical and control identity

pre-Builder control contract:
  where control goes, which ports exist, what source is covered,
  and which cleanup obligations must run

Binding SSA:
  which ValueId reaches each BindingRef use and which PHIs are required

post-MIR analysis:
  whether a resulting PHI is an induction variable, recurrence, or invariant
```

The current `CorePlan`, legacy `LoopRouteContext`, names, value-map diffs, and
precomputed carrier rows are not canonical authorities.

Roadmap-wide completion cannot be docs-only:

```text
roadmap_docs_only_closeout = forbidden
code_or_artifact_delta_required_after_D0 = 1
```

## Accepted decision

| Boundary | Decision |
| --- | --- |
| Value merge authority | one function-owned `BindingSsaBuilderV1` |
| Production cutover unit | whole canonical function owner, never one control arm |
| Pre-Builder owner | exact coverage/topology/ports/targets/cleanup only |
| If / Loop commonality | shared SSA and CFG edge/seal substrate; family CFG boxes stay separate |
| CFG predecessor truth | MIR terminators are SSOT; cached predecessors are a checked witness |
| RegionId materialization | transaction-local roles until an independently accepted SA4 consumer |
| PHI simplification | same-input/self PHIs may remain; later generic simplifier owns removal |
| Optimizer loop facts | post-MIR derived analysis, created only for a concrete consumer |
| Legacy failure | typed failure; no retry or fallback |

The former A + A2′ plan was internally coherent while RegionFlow owned every
effect and PHI-source decision. It is superseded because a generic CFG SSA
baseline would otherwise make RegionFlow and Lower independently decide the
same PHI domain.

## Normative design reference

The sole architecture authority is
`../design/binding-ssa-first-control-lowering-ssot.md`. In particular it
owns the authority matrix, identity/value split, CFG seal law, Binding SSA
algorithm, open-PHI fact rule, RC/lifetime law, nesting model, physical
layout, atomic cutover rule, and final completion definition.

This taskboard owns only execution order and acceptance. Every row preserves:

```text
pre-Builder = exact source/control/cleanup, never value-merge effects
one BindingSsaBuilderV1 per canonical function owner
MIR terminators = CFG truth; cached predecessors = verified witness
family-specific If/Loop CFG boxes over the same SSA/edge substrate
whole-owner production cutover; no old-environment synchronization bridge
no canonical fallback to legacy If/Loop/CorePlan
```

Only owner-local binding rebinds enter Binding SSA. Upvar/captured-by-reference,
field, and index writes stay with their storage owners or fail preflight. New
or modified source/check files stay below 800 lines.

## Implementation order

### D0 — authority decision and taskization — closed by this card

```text
D′ is the final canonical value authority
old A+A2′ S3/I1/I2 are unauthorized
S1 identity and S2 coverage intent are retained
the next blocker is behavior-neutral SSA-P0 seam inventory
```

Production behavior delta: zero.

### B0-L4-S2′ — generic located source coverage — closed

Land only the authority-neutral portion of the preserved S2 WIP:

```text
ConsumedSourceRangeV1 with checked nonzero count
FunctionSourceViewV1-owned suffix first/range/advance
CoveredSourceSiteV1
private VerifiedLocatedSourceCoverageV1
owner/body/start/bounds/order/duplicate verifier
public constructor / Clone / into_parts = 0
Lower receives coverage separately from its family product = 0
new resolved_control_flow coverage production consumers = 0
Binding SSA and Loop production activation = 0
existing A+ If production remains unchanged
```

Prefer `resolved_control_flow/source_coverage.rs`; do not land the historical
`PlanSourceCoverage` name or carrier-oriented README wording unchanged.

Closed evidence:

```text
compiler-owned checked range navigation fixtures = 7
private coverage verification fixtures = 3
resolved_control_flow production consumers = 0
effect/carrier rows = 0
Binding SSA / Loop runtime activation = 0
existing A+ If production behavior = unchanged
```

Production behavior delta: zero.

### SSA-P0 — canonical SSA seam inventory

Before changing ownership, close one exhaustive source/caller table for:

```text
every canonical binding declaration/read/rebind and scope-exit read
every current flat value-environment access
every canonical CFG edge emitter and predecessor writer
every PHI reserve/define/expose/patch/rollback path
every assignment and scope-exit RC read/release path
every function finish/publication barrier
the exact currently accepted function-body terminal Return shape
all old If effect/join/snapshot consumers
```

Classify each row as `move to Binding SSA`, `control-only retain`, `legacy
isolate`, or `caller-zero delete`. This is evidence only: production behavior
and accepted grammar remain unchanged.

Closed evidence:

```text
machine rows = 92
binding/value = 18
CFG/predecessor = 12
PHI lifecycle = 23
RC/lifetime = 7
finish/publication = 10
terminal Return = 10
old A+ If authority = 12
production behavior delta = 0
accepted grammar delta = 0
```

Evidence card:
`mirbuilder-canonical-ssa-seam-inventory-2026-07-14.md`.

### SSA-L0 — mandatory oversized PHI-helper split

`src/mir/builder/ssa/phi_input_materializer.rs` is already above 800 lines.
Before SSA-C1, SSA-P1, or SSA-S1 edits, split it by existing responsibility in
one behavior-neutral commit:

```text
facade
edge_rematerialization:
  analysis, diagnostics, recursive rematerialization, for_pred
function_repair:
  whole-function repair, pruning, missing-input completion
separate focused test modules
no API/semantic/grammar change
all existing PHI lifecycle tests green
each resulting source file below 800 lines
no Binding SSA acceptance code in the split commit
```

### SSA-C1 — canonical CFG/seal prerequisite

```text
one canonical edge facade
late predecessor veto
computed/cached predecessor equality
seal-twice and edge-after-seal errors
terminator-derived predecessor truth; cached-successor recompute is not proof
PHI analysis/update_cfg side-effect repair forbidden on canonical edges
```

Production activation remains zero. Existing If continues on its old path.

### SSA-P1 — PHI transaction cleanup prerequisite

Close the reusable failure lifecycle independently of SSA acceptance:

```text
every pending rollback is attempted
cleanup continues after one rollback failure
primary and cleanup errors are both retained
partial PHI/function publication = 0
success commits exactly once
```

No accepted syntax or production Binding SSA call is added.

### SSA-V0 — canonical publication/verifier prerequisite

Close the invalid-publication boundary before Binding SSA production work:

```text
post-RC MIR verifier failure is a typed compile failure
candidate module commit after verifier failure = 0
duplicate same-name canonical function publication is a typed failure
function/module publication before seal/SSA completion = 0
verification_result = Err cannot cross CanonicalModuleLoweringSessionV1::commit
```

This row changes no accepted source grammar. It must land before SSA-S1 is
connected to production. Legacy result-reporting behavior, if still required,
stays behind explicit legacy provenance rather than weakening the canonical
barrier.

### SSA-S1 — disconnected Binding SSA

Implement `builder/ssa/binding/` with a fake/narrow IR test adapter and no AST,
source, RegionFlow, or name dependency.

Focused fixtures:

```text
entry definition and same-block overwrite
single predecessor
diamond with no/one-sided/two-sided assignment
nested diamonds
open Loop header and one backedge
zero iteration
multiple backedges
same-input and self PHIs retained
missing definition
foreign BindingRef owner
duplicate edge and late edge
unsealed/incomplete finish
PHI patch/rollback failure
all inputs are exact actual predecessors and dominate their edges
```

Production activation remains zero.

### SSA-S2 — identity/value separation

Refactor the canonical Lowerer behind the old production behavior:

```text
ResolvedIdentityLedgerV2 owns claims/lifetime only
old If value path remains the sole production value owner
Binding SSA production calls = 0
all current canonical fixtures remain green
```

This is Refactor Series Mode and adds no source grammar.

### SSA-E0 — preserved terminal Return contract

Before the owner cutover, seal only the already accepted function-body
terminal Return and implicit fallthrough completion cases:

```text
exact function target
exact terminal statement site
unreachable suffix count = 0
ordered crossed-scope cleanup obligations are explicit, including empty
implicit Void completion is represented separately and closes after SSA finish
nested If/Loop Return activation = 0
accepted source grammar delta = 0
```

This row preserves existing behavior; it does not authorize a new Return port.
General Return through If/Loop waits for the later EXIT rows.

### SSA-S3 — disconnected carrier-free If control product

Seal the future If-side control contract without changing production:

```text
exact If/IfThen/optional IfElse topology
else=None versus else=Some(empty)
fallthrough-only V1 ports
inseparable exact source coverage
typed unsupported control errors
no effects, may_rebind, join-source rows, ValueId, or BasicBlockId
```

The historical A+ product remains the sole production If path until SSA-I1.
Do not run both analyzers as production authorities for one function.

### SSA-I1 — atomic current-owner cutover

In one production commit, move the entire currently accepted canonical owner
grammar to Binding SSA:

```text
parameters, locals, and Outbox declarations
variable reads
binding assignments and ReleaseStrong(previous)
straight-line statements
BlockExpr
fallthrough statement If, including nested If
only the SSA-E0-sealed function-body terminal Return
```

If remains a family-specific CFG/semantic box. It stops querying effect sets
or join-source rows. Both branches use block-local definitions, merge closes
after every predecessor is known, and later `read` creates only required PHIs.

Atomic acceptance:

```text
all current canonical If/BlockExpr runtime fixtures green
all canonical declaration/read/rebind operations use Binding SSA
then definitions do not leak into else compilation state
scope leave retires identity without deleting historical SSA definitions
assignment ReleaseStrong reads the previous value through Binding SSA
scope-exit cleanup reads the current merged value through Binding SSA
self-assignment and BlockExpr tail/alias ownership fixtures are green
flat value-map merge authority calls = 0
canonical If may_rebind/join-source queries = 0
manual branch snapshot/restore = 0
canonical materialize_all_phi_inputs repair calls = 0
function verifier green before publication
canonical failure legacy retry = 0
```

No Loop syntax is activated in this commit.

### SSA-R1 — retire old If value-flow authority

After SSA-I1 is green and exact production caller counts are zero, physically
delete the old canonical If effect/join products and branch snapshot
transaction: condition/whole-effect summaries, `may_rebind_outer`, join-source
rows, and the effect-driven PHI materializer. Temporary isolation is not a
completion state. Keep exact If topology, source coverage, semantic stacks,
predecessor checks, and the runtime fixtures.

Production behavior delta: zero; authority count decreases.

### B0-L4-S3′ — carrier-free Loop control contract

Add a disconnected `VerifiedLocatedLoopControlV1` owning only:

```text
exact Loop statement site
closed Loop/LoopBody bundle from S1
condition-false-only V1 topology
fallthrough-only body proof
inseparable exact located coverage from S2′
typed unsupported control errors
```

It contains no `may_rebind`, carrier rows, ValueId, BasicBlockId, or source
matrix. Nested If/Loop and nonlocal exits remain preflight rejects in this
first product.

Do not recreate the old `SharedPostState` restriction. Outcome-dependent
binding state is a CFG/SSA concern. A short-circuit condition may be rejected
in the first slice only when the current exact expression/control grammar
cannot lower its CFG, never because a carrier/effect row is unavailable.

Production Loop activation: zero.

### B0-L4-I1′ — disconnected Loop CFG transaction

Build the family-specific CFG box over the common canonical CFG and Binding
SSA substrate:

```text
preheader
header, open until every backedge exists
body entry
latch
after, open until every false/break edge exists
transaction-local RegionId roles
LoopBody exact scope/region enter/leave around body only
```

The condition is lowered outside the LoopBody pair on every runtime
iteration. No carrier snapshot, restore, or publication API exists.

Production Loop activation: zero.

### B0-L4-I2′ — atomic first canonical Loop

Connect S1 + S2′ + S3′ + I1′ in one production commit.

Accepted first grammar:

```text
statement Loop
condition-false external exit only
condition/tail/RHS expressions:
  Literal
  Variable
  eager non-And/Or BinaryOp
  closed BlockExpr over the same expression set
body statements:
  local declaration
  BindingRef assignment
  expression statement from the closed set
zero or more runtime iterations
```

Preflight rejects before Builder effects:

```text
And / Or short-circuit control
Call / MethodCall / FunctionCall
Outbox
nested If / Loop
Break / Continue / Return
LoopRange / ForRange
QMark / Throw / Try
Lambda execution
every expression/statement kind not listed above
```

The grammar list is fixed by S3′ fixtures. I2′ may not inherit newly landed
expression capabilities implicitly.

Required runtime fixtures:

```text
zero / one / multiple iterations
condition-only, body-only, and combined outer rebind
multiple independent bindings
binding with no downstream read does not require a predeclared carrier/PHI row
condition BlockExpr outer rebind reaches body and after
condition/body locals and same-name shadow do not leak
after-loop read receives the correct SSA value
header seals only after latch edge
after seals only after every V1 exit edge
actual/cached predecessor equality
all PHIs pass CFG, SSA, dominance, and RC verification
VM/reference result parity
```

Legacy `LoopRouteContext`, current `CorePlan`, normalization suffix, name
lookup, map diff, and retry counts remain zero on the canonical route.

### N1 — If inside Loop

Add exactly one fallthrough nesting shape:

```text
If in Loop
BlockExpr at every condition/body boundary
```

The inner If merge seals before the Loop latch consumes its reaching
definitions. No family effect summary is propagated.

### N2 — Loop inside If

Add exactly one fallthrough nesting shape. The inner Loop after block seals
before the outer branch merge reads its definitions.

### N3 — Loop inside Loop

Add exactly one fallthrough nesting shape. The inner Loop after definitions
feed the outer latch/backedge through the same function SSA instance.

### N4 — bounded depth-independent nesting proof

Add no new syntax. Fix bounded witnesses such as:

```text
Loop -> If -> Loop -> If
same-name shadows at multiple depths
condition BlockExpr at nested boundaries
error cleanup at each child session boundary
```

Only after N4 may the supported If/Loop grammar claim finite nesting under the
same depth-independent rules.

### EXIT-S0 — semantic exit, cleanup, and disposition contract

Before adding a nonlocal exit shape, seal a disconnected pre-Builder
`ResolvedExitCleanupContractV1` owning only:

```text
exact source exit site and typed port kind
exact target RegionId or function region
ordered crossed-scope cleanup obligations
unreachable source disposition:
  Materialized
  SkippedAfterTerminator
  OwnedByChildFunction
ValueId / BasicBlockId / transaction-local block roles = 0
new exit production activation = 0
existing Binding SSA production remains unchanged
```

### EXIT-S1 — disconnected Lower target-role registry

Add `ActiveControlTargetsV1` under resolved Lowering as a separate,
transaction-local materialization registry:

```text
RegionId -> accepted Continue/Break target roles
function region -> Return target role
ordered cleanup emission cursor
durable publication = 0
pre-Builder semantic ownership = 0
new exit production activation = 0
```

It may hold materialized block roles because it is Lower-owned. It never
becomes resolver/RegionFlow authority and is discarded with the function
transaction.

### EXIT-I1 — Continue from the current Loop body

Activate one shape: straight-line Continue in the current Loop body, targeting
the exact current Loop RegionId. The Loop CFG contract selects its continue
role; Binding SSA observes only the emitted edge.

### EXIT-I2 — Continue through nested If

Activate one shape: Continue inside an already-supported nested If branch to
that If's enclosing current Loop. Prove branch cleanup and exact RegionId
routing; do not add labeled/outer-Loop syntax.

### EXIT-I3 — Break from the current Loop body

Activate one shape: straight-line Break in the current Loop body to its exact
after role. Keep the after block open through every accepted current-Loop exit,
then seal once.

### EXIT-I4 — Break through nested If

Activate one shape: Break inside an already-supported nested If branch to that
If's enclosing current Loop. Prove branch cleanup and exact predecessor
accounting; do not add labeled/outer-Loop syntax.

### EXIT-I5 — Return through If

Activate one shape: Return from a statement-If branch to the exact current
function region, with its sealed cleanup obligations. Cover one- and
zero-reachable branch merge cases without fabricating a value for unreachable
source.

### EXIT-I6 — Return through Loop

Activate one shape: Return from the current Loop body to the exact current
function region. Cover the remaining condition-false path and Return path
without forcing unreachable declarations into ValueIds.

### EXIT-I7 — nested exit closure proof

Add no syntax or port kind. Combine only already activated shapes to prove:

```text
Continue/Break in nested If inside Loop
Return through nested If/Loop compositions
cleanup order is inner-to-outer exactly once
zero/one/two reachable predecessor handling
every unreachable declaration has one disposition
all resulting blocks seal from actual predecessor sets
```

Only after I7 may the supported grammar claim nested exit closure.

### EXIT-P0 — parked labeled or outer-Loop targets

Do not infer labeled Break/Continue or an inner-Loop-to-outer-Loop transfer
syntax from exact RegionId infrastructure. If the language later accepts such
a source form, open a separate language decision and one-shape implementation
row. Until then activation is zero.

QMark, Throw, and Try/Finally remain separate design-stop rows until their
language, resolver, and cleanup contracts are independently accepted:

```text
pre-Builder contract:
  exact target + ordered cleanup chain + reachable port kind

Lower:
  emit cleanup and exact CFG edge/terminator

Binding SSA:
  resolve values on the resulting CFG only
```

Unsupported syntax fails before Builder effects. Do not add partial bool or
Option-shaped port support.

### X0 — non-blocking parked three-family control-only extraction

This optional appendix never blocks F1, retirement, or canonical-source
completion. It opens only when its independent evidence gate is satisfied.

After If, Loop, and one independent third control family are production-closed,
inventory their landed code mechanically. Extract a private common envelope
only when all three prove identical ownership for:

```text
source coverage lifetime
owner/source closure
typed control port vocabulary
cleanup/rollback/commit lifecycle
```

Effect ordering, may-rebind sets, carriers, and family PHI lifecycles are not
extraction candidates. If the three families do not prove a smaller useful
envelope, keep the family wrappers separate.

### F1a — instance method and constructor owner family

Cut over one closed receiver-bearing owner capability set. Receiver,
parameters, locals, reads, writes, RC, SSA finish, and publication switch
atomically; accepted control grammar does not widen in this row.

### F1b — source entry owner family

Cut over one closed source-entry owner capability set without changing the
synthetic wrapper policy or adding Main/Lambda behavior.

### F1c — Main.main owner plus entry thunk

Lower source `Main.main` exactly once as one source owner. The synthetic entry
is a call-only thunk; inline and callable copies of the same source body are
forbidden.

### F1d — Lambda child owner family

Open only after capture mode, cell/slot layout, child-owner transport, and
Upvar storage authority are independently accepted. Cut over the complete
parent/child source unit atomically; parent canonical / child legacy is
forbidden.

Every later function owner family gets its own `F1x` row. REPL and ProgramV0
remain explicit legacy inputs until their separate lifetime/source-authority
decisions.

### RET-P0 — legacy caller inventory

Inventory every remaining caller of:

```text
LoopRouteContext and current CorePlan
legacy IfForm value-map joins
manual If/Loop carrier classification
name-keyed final_values
raw &[ASTNode] + consumed usize source protocol
route-specific PHI materializers
```

Classify callers by canonical source, explicit legacy source, ProgramV0, REPL,
or dead. This row changes no production behavior.

### RET-I1 — canonical legacy-call veto

Make legacy If/Loop/CorePlan imports, constructors, calls, name lookups, and
retry paths zero on every supported canonical source route. Guard the boundary
without deleting explicit legacy consumers prematurely.

### RET-I2 — explicit legacy isolation

Confine remaining legacy control routes behind explicit
`LegacyModuleLoweringInputV1` provenance. Canonical failure never enters this
boundary.

### F2 — default canonical source route

Switch the ordinary canonical source frontend only after whole-unit preflight
proves every owner and control family supported. Failure never retries the
legacy source route.

### RET-R1 — caller-zero manual authority retirement

Delete caller-zero mechanisms:

```text
legacy IfForm value-map joins
manual LoopForm carrier classification
name-keyed final_values
route-specific PHI materializers
```

### RET-R2 — conditional CorePlan retirement

Delete current `CorePlan`, `LoopRouteContext`, and the raw suffix protocol only
after the complete repository caller inventory reaches zero. If ProgramV0,
REPL, or another explicit legacy input still calls them, keep the isolated
implementation and claim only that canonical source has zero legacy authority.

ProgramV0 compatibility is never silently promoted or deleted by this series.

### O0 — optional durable RegionId materialization

Open SA4 only when a named production consumer needs durable region-to-MIR
roles. The product must be role-aware (`entry`, ports, owned blocks), derived
from verified materialization, and invalidated with MIR changes. A scalar
`RegionId -> BasicBlockId` map remains forbidden.

### O1 — optional derived Loop analysis

Create post-MIR `DerivedLoopAnalysisV1` only when a named production optimizer
consumer exists and proves that completed SSA cannot be queried directly.

Possible derived fields:

```text
header PHIs
preheader/backedge incoming values
induction candidates
recurrences
invariants
```

The result is invalidated by MIR/CFG changes and never becomes source or
Lower route authority. Structured `LoopRegionSignature` is a separate IR
decision and cannot coexist as a second generic-baseline truth.

## Production activation table

| Milestone | Binding SSA production | If production owner | Loop production | Source grammar delta |
| --- | ---: | --- | ---: | ---: |
| D0 / S2′ / SSA-P0 / SSA-L0 / SSA-C1 / SSA-P1 / SSA-V0 / SSA-S1 / SSA-S2 / SSA-E0 / SSA-S3 | 0 | current A+ path | 0 | 0 |
| SSA-I1 | 1 whole owner | Binding SSA + If CFG box | 0 | 0 |
| SSA-R1 / S3′ / I1′ | 1 whole owner | Binding SSA | 0 | 0 |
| I2′ | 1 whole owner | Binding SSA | 1 closed Loop family | +1 family |
| N1-N3 | 1 whole owner | Binding SSA | bounded nesting expansion | one nesting shape per slice |
| N4 / EXIT-S0 / EXIT-S1 / EXIT-I7 | 1 whole owner | Binding SSA | existing accepted families | 0 |
| EXIT-I1-I6 | 1 whole owner | Binding SSA | bounded typed exits | one source shape per slice |
| F1a-F1d / F2 | bounded then all supported owners | Binding SSA | Binding SSA | one owner family per F1 row |

## Required counters

Before SSA-I1:

```text
BindingSsaBuilder production sessions = 0
carrier-free If/Loop control production consumers = 0
canonical accepted grammar delta = 0
```

At SSA-I1 and thereafter on the canonical route:

```text
BindingRef value merge authorities = 1
variable reads bypassing Binding SSA = 0
binding definitions bypassing Binding SSA = 0
flat map branch snapshots = 0
If may_rebind queries = 0
If join-source queries = 0
Lower full-map diff = 0
String/name binding lookup = 0
Reserve-only PHI publication = 0
edge-after-seal acceptance = 0
silent legacy retry/fallback = 0
```

At B0-L4-I2′ and thereafter:

```text
canonical Loop carrier rows = 0
canonical LoopRouteContext constructions = 0
canonical current CorePlan calls = 0
coverage inferred from consumed usize = 0
durable RegionId materialization publication = 0 until SA4
```

## Error and finish gates

Inject failures at:

```text
edge emission
provisional PHI definition
recursive predecessor read
PHI patch
block seal
declaration/read/assignment coverage
scope/region leave
RC validation
SSA finish
function verification
function finalization
```

Every failure proves:

```text
all pending cleanup attempts run
primary and cleanup failures are both preserved
caller current function/block/context is restored
semantic stacks are restored or the draft is discarded
partial function/module publication = 0
canonical legacy retry = 0
```

Per-block seal order:

```text
1. every accepted incoming edge is emitted
2. terminator-derived and cached predecessors are exactly equal
3. CanonicalCfgSession seals and yields VerifiedPredecessors
4. BindingSsaBuilder seals from that witness
5. incomplete PHIs for the block are patched
```

Success publication order after every block seal:

```text
1. all control coverage consumed
2. identity coverage complete
3. scope/region stacks balanced
4. every touched block sealed
5. incomplete PHIs = 0
6. PhiTxn committed
7. final post-RC CFG/SSA/dominance/RC/MIR reverify green
8. resolved authority finished
9. function session restores caller state
10. sealed function draft commits to the module
```

## Guard plan

Do not create another public row guard.

```text
stable public entry:
  tools/checks/resolved_region_flow_authority_guard.sh

private reusable helpers:
  tools/checks/lib/resolved_control_lowering_contract.sh
  tools/checks/lib/resolved_control_flow_contract.sh
  tools/checks/lib/resolved_binding_ssa_contract.sh
  tools/checks/lib/resolved_if_lowering_contract.sh
  tools/checks/lib/resolved_loop_lowering_contract.sh
```

The top authority guard must remain below 800 lines. The control aggregator
sources the control-flow, Binding SSA, If, and Loop helpers. The control-flow
helper owns the module manifest, forbidden effect/MIR imports, co-sealed
coverage boundary, production caller counts, and source-size check. Update
each private helper in the slice that changes its real contract; do not
front-load speculative regexes.

Guard transition is ordered:

```text
S2′:
  add disconnected resolved_control_flow contract checks

SSA-S3:
  line-neutrally admit resolved_control_flow as a disconnected consumer
  keep old production If S2/I1 checks

SSA-I1 atomic commit:
  replace production If effect/join assertions with Binding SSA/control-only assertions

SSA-R1:
  assert exact old symbol and caller counts are zero
  physically remove old effect/join files and their allowlist
```

Common per-code-slice gates:

```bash
bash tools/checks/resolved_region_flow_authority_guard.sh
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Add focused unit/runtime commands named by each milestone before committing.

## May claim

| Milestone | Maximum claim |
| --- | --- |
| S2′ | exact located range/coverage schema exists; its production consumers are zero and existing A+ If is unchanged |
| SSA-P0/L0/P1/C1 | the inventory, physical split, CFG seal, and PHI cleanup prerequisites are closed; production SSA calls are zero |
| SSA-V0 | canonical verifier failure and duplicate function publication cannot commit; grammar delta is zero |
| SSA-S1 | disconnected Binding SSA handles tested CFG shapes; production calls are zero |
| SSA-E0 | the already accepted terminal Return has an exact preservation contract; grammar delta is zero |
| SSA-S3 | one carrier-free If control product is sealed; production If still uses A+ |
| SSA-I1 | the current closed canonical owner has one BindingRef value/PHI authority |
| S3′ | one carrier-free Loop control contract is sealed; Builder connection is zero |
| I2′ | the first closed canonical Loop uses exact control plus generic Binding SSA |
| N1/N2/N3 | the selected single nesting shape uses one function SSA authority |
| N4 | the supported If/Loop grammar has bounded depth-independent nesting evidence |
| EXIT-I1-I6 | the selected single Continue/Break/Return source shape is production-supported |
| EXIT-I7 | accepted nested exit cleanup and predecessor closure are proven without a new shape |
| F2 | every preflight-supported canonical source owner uses the no-retry SSA route |

## Must not claim

```text
all source/control families supported before their capability gates
captured-by-reference or Upvar layout from local Binding SSA
field/index writes are local SSA definitions
typed open-PHI facts without a separate proof
Break/Continue/Return/Try support from generic SSA alone
durable RegionId materialization or SA4 completion
ProgramV0 source authority
REPL owner lifetime completion
Hako Lower parity
current CorePlan retirement before its final callers close
post-MIR recurrence authority without a production consumer
```

## Stop conditions

Stop implementation or publication if any slice:

```text
uses Binding SSA only for Loop while the same owner's If uses a flat map
seeds SSA at a Loop boundary and exports all visible bindings afterward
adds an old-environment/SSA synchronization bridge or recursive mode Option
keeps effect/carrier rows as permanent PHI-placement verification
passes AST, SourceSite, RegionId, Span, pointer, or name into Binding SSA
puts ValueId, BasicBlockId, or materialized target roles in a pre-Builder product
adds an independently mutable third predecessor truth
emits an edge and registers its predecessor through unguarded separate calls
adds a predecessor after seal
uses cached successors as the terminator-truth predecessor proof
repairs canonical CFG or missing PHI inputs during post-Lower materialization
exposes a Reserve-only PHI dst
infers a concrete open-PHI fact from only the entry input
erases historical SSA definitions on lexical scope leave
routes Upvar/field/index writes through local Binding SSA
discovers unsupported control after Builder effects
lets PHI/cleanup failure overwrite the primary error
publishes before SSA/coverage/stack/function verification finishes
commits a canonical module while verification_result is Err
silently overwrites a same-name canonical function
retries legacy If/Loop/CorePlan after canonical failure
lets one activation row accept more than one source/control shape
inherits a newly landed expression kind without updating the row's closed grammar
adds a universal optional-field control product before three-family evidence
adds DerivedLoopAnalysis without a named consumer
lets a new or modified source/check file reach 800 lines
```

## Final completion definition

This roadmap reaches its canonical-source final form when:

```text
one BindingSsaBuilder instance owns all local BindingRef reaching values per function
all canonical CFG edges use one late-edge-safe facade
pre-Builder products contain only source/control/cleanup semantics
If, Loop, and nested typed exits use family boxes over the same SSA substrate
all supported source owners cut over atomically with no fallback
old canonical effect/carrier/manual-PHI callers are zero
remaining legacy mechanisms are isolated behind LegacyModuleLoweringInputV1
repository-wide physical deletion occurs only after global caller zero
function publication is gated by exact coverage, seal, SSA, CFG, RC, and MIR verification
optimizer loop facts are derived from completed MIR only when consumed
```

ProgramV0, REPL lifetime, Hako Lower parity, and structured-loop IR remain
independent decisions. They are not hidden prerequisites or accidental claims
of this final form.

## Immediate next action

Close SSA-L0 only:

```text
split phi_input_materializer by existing responsibility
facade + edge_rematerialization + function_repair + focused test modules
preserve every current API/caller and runtime behavior
add no Binding SSA, CFG seal, PHI cleanup, or grammar acceptance code
keep every resulting source/check file below 800 lines
```
