Status: Design stop — inventory and consultation only
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

## Why this is a new boundary

The immutable carrier `LocatedBodySuffixV1` already exists, but the active
Planner/normalization surface still includes raw `&[ASTNode]` suffix inputs and
`consumed: usize` results. CorePlan owns control recipes; RegionFlow owns
binding/port effects; canonical Lower owns ValueIds and BasicBlockIds. Moving
source identity into the wrong one would recreate a second semantic authority.

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

## Consultation decisions required

1. Does CorePlan carry a co-sealed source coverage witness, or does an
   owner-closed sidecar pair CorePlan nodes with exact source ranges?
2. Is `LocatedBodySuffixV1` the only Planner request, and is
   `ConsumedSourceRangeV1` the only successful coverage result?
3. Which pre-Builder product owns Loop condition/body/port effects without
   importing ValueId or BasicBlockId?
4. What is the first closed runtime grammar: one fallthrough loop family, or a
   disconnected carrier/coverage slice before any Loop activation?
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
lets CorePlan allocate BindingId, ValueId, or BasicBlockId
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

No production code edit is authorized by this card.
