Status: Closed — A + A2′ accepted; implementation task opened at S1
Date: 2026-07-14
Blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-B0-L4-LOCATED-COREPLAN-SOURCE-COVERAGE-DESIGN-STOP-001

# B0-L4 Located CorePlan Source Coverage Design Stop

## Current boundary

B0-L3b is closed. Fallthrough statement `If` now consumes one pre-Builder
`VerifiedResolvedFunctionFlowV1`; Lower neither diffs value maps nor discovers
write sets. Exact If/branch RegionIds are consumed and covered, while durable
RegionId-to-MIR-block publication remains zero until SA4.

The next accepted order names B0-L4 as the located CorePlan/Loop boundary, but
does not yet select its exact carrier or first runtime grammar. Implementation
must therefore stop at consultation.

P0 inventory is closed. The shareable decision sheet is:

```text
docs/development/current/main/investigations/
  mirbuilder-b0-l4-located-coreplan-source-coverage-consultation-2026-07-14.md
```

The consultation accepts A + A2′. Production implementation is authorized
only through the bounded task card:

```text
docs/development/current/main/investigations/
  mirbuilder-b0-l4-a-a2prime-implementation-task-2026-07-14.md

current slice:
  B0-L4-S1 passive exact Loop bundle
```

## Why this is a new boundary

The immutable carrier `LocatedBodySuffixV1` already exists, but the active
Planner/normalization surface still includes raw `&[ASTNode]` suffix inputs and
`consumed: usize` results. Inventory proves that current `CorePlan` is a legacy
Builder-time materialization product rather than a pre-Builder control recipe.
RegionFlow owns binding/port effects and canonical Lower owns ValueIds and
BasicBlockIds. Moving source identity into the current CorePlan would recreate
a second semantic authority after Builder effects have already begun.

## Inventory task B0-L4-P0

Produce one bounded table before proposing code:

```text
row:
  production caller
  raw body/suffix input type
  CorePlan/Recipe output type
  consumed-count/range output
  source identity retained or lost
  RegionFlow product available or absent
  Lower consumer
  retirement owner
```

Inventory only these seams:

```text
canonical resolved Lower statement/body entry
normalization suffix router and plan box
CorePlan construction/verification/lowering entry
Loop condition/body RegionId and ScopeId topology
source coverage and consumed-range accounting
legacy JoinIR/name-keyed state dependencies
```

Do not inventory every `&[ASTNode]` helper in the repository. Start from live
production callers and follow only the selected route.

## P0 result

The live routes are three separate seams:

```text
canonical resolved Lower:
  exact LocatedBody/LocatedStmt identity
  If RegionFlow connected
  Loop rejected before Builder

legacy normalization suffix:
  raw &[ASTNode] + consumed usize
  dev-only StepTree/JoinIR shadow route
  not CorePlan

legacy Loop CorePlan:
  raw LoopRouteContext
  Builder-time facts/composition
  ValueId/BasicBlockId/PHI/String-keyed final state
```

`LocatedBodySuffixV1` exists with an exact bounds-checked constructor, but has
zero production consumers. `ConsumedSourceRangeV1`, Loop bundle lookup, and
Loop RegionFlow are absent.

The current `CorePlan` is already a mechanical MIR materialization product,
not an ID-free pre-Builder semantic recipe. Adding a source cursor or coverage
field to it would not restore the B0-L3b authority split.

One separate legacy candidate defect was found: the dev-only suffix caller
increments `idx` by `consumed` without continuing before its next
`statements[idx]` access. The consultation records it as
`LEGACY-NORMALIZATION-SUFFIX-CONSUMED-INDEX-001`; it must not be fixed in the
B0-L4 BoxShape series without a focused reproducer.

## Consultation decisions required

1. Do we add a new ID-free canonical Loop contract, or version CorePlan into
   physically separate semantic and materialized products?
2. Is exact source coverage a field of the Loop flow or one owner-closed
   sidecar paired by a sealed wrapper?
3. Is `LocatedBodySuffixV1` the only Planner request, and does successful
   coverage require both an outer `ConsumedSourceRangeV1` and ordered nested
   exact-site claims?
4. What is the first closed runtime grammar: false-exit-only Loop, or a
   disconnected carrier/coverage/flow series before any Loop activation?
5. Which role-aware RegionId materialization remains transaction-local in
   B0-L4, and which publication is deferred to SA4?

## Nonclaims

```text
Loop/CorePlan canonical runtime support
Return/QMark/Break/Continue/Throw ports
durable RegionId materialization map
SA4 cutover
legacy Planner/JoinIR retirement
Lambda/capture support
ProgramV0 source authority
default source route cutover
```

## Stop conditions

Stop consultation or publication if a proposal:

```text
passes raw AST plus an unrelated source cursor
uses Span, name, pointer, or encounter order as identity
lets Lower infer consumed source ranges from consumed usize
lets a pre-Builder canonical product allocate BindingId, ValueId, or BasicBlockId
duplicates RegionFlow port/write-set analysis in Lower
publishes a durable RegionId-to-block map before SA4
mixes Loop activation with Lambda, ProgramV0, or default-route cutover
```

## Acceptance for the next decision

The consultation response must name:

```text
source coverage owner and non-owner
exact request/result carrier types
first closed runtime grammar
RegionFlow/CorePlan/Lower responsibility split
RegionId consume-only versus SA4 publication boundary
atomic landing order
required fixtures, gates, may-claim, must-not-claim, stop conditions
```

This design-stop card is closed. The implementation task authorizes S1 only;
later slices open only after the preceding slice is green and published.
