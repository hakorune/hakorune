---
Status: Consultation requested — no production code authorized
Date: 2026-07-14
Scope: B0-L4 exact located Loop/CorePlan source coverage boundary
Work kind: BoxShape decision stop
Parent:
  - mirbuilder-b0-l4-located-coreplan-source-coverage-design-stop-2026-07-14.md
Related:
  - mirbuilder-b0-l3b-a-plus-implementation-task-2026-07-13.md
  - mirbuilder-resolved-region-flow-v1-task-2026-07-13.md
  - src/mir/compiler/located.rs
  - src/mir/resolved_region_flow/
  - src/mir/builder/control_flow/plan/
---

# B0-L4 Located CorePlan Source Coverage Consultation

## Decision request

Select the first exact-source Loop planning boundary after B0-L3b.

The repository currently has three physically distinct routes:

```text
canonical resolved Lower:
  exact LocatedBody / LocatedStmt
  + sealed BindingRef / ScopeId / RegionId
  + If-only verified RegionFlow

legacy normalization suffix:
  raw &[ASTNode]
  + consumed: usize
  + dev-only StepTree / JoinIR shadow execution

legacy Loop CorePlan:
  raw AST LoopRouteContext
  + Builder-time facts/composition
  + ValueId / BasicBlockId / PHI / String-keyed final state
```

The last two routes have no canonical source-site or coverage witness. They
must not be treated as the canonical continuation merely because their names
contain `plan` or consume a body suffix.

The decision must name:

```text
pre-Builder Loop semantic/flow product
exact source-coverage owner and non-owner
request/result carrier types
first disconnected versus runtime slice
Builder-time materialization product
RegionId consume-only versus SA4 publication boundary
```

## P0 production inventory

| Seam | Live input | Result | Exact source identity | RegionFlow | Consumer / status |
| --- | --- | --- | --- | --- | --- |
| canonical resolved body | `LocatedBodyV1`, then exact `LocatedStmtV1` | direct one-statement Lower | retained | If only | `CanonicalFunctionLowererV1`; Loop rejected before Builder |
| legacy normalization suffix | raw `&[ASTNode]`, cloned `variable_map` | `NormalizationPlan { consumed: usize }`, then `Option<usize>` | lost | absent | dev-only StepTree/JoinIR shadow; not CorePlan |
| legacy Loop router | raw `ASTNode::Loop` parts | `LoopRouteContext` | lost | absent | live legacy Loop route |
| single planner | raw condition/body/function body | `PlanBuildOutcome` | lost | absent | registry and recipe composer |
| recipe composition | cloned recipe-local AST plus `&mut MirBuilder` | `LoweredRecipe = CorePlan` | lost | absent | already allocates MIR identities |
| verified CorePlan entry | `CorePlan` | MIR or `Option<ValueId>` | absent | absent | `PlanVerifier` then `PlanLowerer` |
| canonical suffix carrier | `LocatedBodySuffixV1` | none | retained | none | passive; production consumers = 0 |

### Canonical route

The canonical body route already retains exact identity:

```text
CanonicalFunctionLowererV1::lower_body
  -> FunctionSourceViewV1::body_stmt
  -> CanonicalFunctionLowererV1::lower_stmt
```

`LocatedBodySuffixV1` exists and validates owner and bounds, but has no
Planner or Lower consumer. `ConsumedSourceRangeV1` does not exist.

Exact Loop navigation already exists:

```text
ExprChildRoleV1::LoopCondition
BodyChildRoleV1::LoopBody
SourceBodyKindV1::Loop
```

Canonical capability preflight has no Loop arm and rejects it before Builder
effects. This is the correct current fail-fast behavior.

### Legacy suffix route is not the CorePlan route

The suffix router is a separate dev-gated normalization experiment:

```text
build_block
  -> NormalizedShadowSuffixRouterBox
  -> NormalizationPlanBox
  -> NormalizationExecuteBox
  -> StepTree / normalized JoinIR
```

Its successful protocol is only a count. It does not carry owner, body site,
first statement site, nested coverage, or RegionFlow. Canonical B0-L4 must not
adapt this route by adding an unrelated source cursor.

### Current CorePlan is already materialized

The live Loop route is:

```text
cf_loop
  -> LoopRouteContext
  -> route_loop
  -> facts / registry / RecipeComposer
  -> CorePlan
  -> PlanVerifier
  -> PlanLowerer
```

Current `CorePlan` is not an ID-free pre-Builder semantic recipe. It contains:

```text
BasicBlockId
ValueId
Frag
PHI rows
Vec<(String, ValueId)> final_values
```

Recipe composers receive `&mut MirBuilder`, allocate blocks and values, and
consult or mutate legacy name-keyed state before `PlanVerifier`. Recipe bodies
own cloned AST and use recipe-local statement indexes. Those indexes are not
canonical source identity.

Therefore the existing `CorePlan` cannot own the first canonical source-flow
contract without collapsing the already accepted boundary:

```text
resolved_semantics:
  semantic identity

resolved_region_flow:
  state/control flow and exact source coverage

resolved_lowering:
  MIR materialization
```

## Loop semantic inventory

The resolver already establishes this topology:

```text
surrounding region/scope
  evaluate Loop condition
  close any condition BlockExpr
  enter one RegionKindV1::Loop + ScopeKindV1::LoopBody pair
  resolve body
  leave pair
```

The Loop RegionId is also the exact target identity for nearest-loop
`Break`/`Continue`. The verified function product contains the authoritative
arena records, but lacks:

```text
ResolvedLoopRegionBundleV1
exact Loop site index/query
Loop-specific pair/cardinality seal verification
VerifiedResolvedLoopFlowV1
Loop port/effect/source-coverage product
```

Current `VerifiedResolvedFunctionFlowV1` owns If rows only. Lowering a Loop
would otherwise require Builder to rediscover repeated condition effects,
loop-carried bindings, latch state, false-exit state, and eventually
Break/Continue ports.

## Options

### A — new pre-Builder canonical Loop contract, legacy CorePlan isolated

Recommended.

Introduce an ID-free, owner-closed product before Builder effects:

```rust
pub struct ResolvedLoopRegionBundleV1 {
    pair: ResolvedScopeRegionPairV1,
}

pub struct VerifiedResolvedLoopFlowV1 {
    site: SourceStmtSiteV1,
    regions: ResolvedLoopRegionBundleV1,
    condition: VerifiedLoopConditionFlowV1,
    body: VerifiedLoopBodyFlowV1,
    carriers: ResolvedLoopCarrierContractV1,
    ports: ResolvedLoopPortsV1,
    coverage: VerifiedLoopSourceCoverageV1,
}
```

Names are illustrative; the selected task must freeze the exact vocabulary.
The product contains only source sites, BindingRefs, ScopeIds, RegionIds, and
closed flow/source-coverage contracts. It contains no ValueId, BasicBlockId,
raw AST clone, String-keyed binding state, or `&mut MirBuilder`.

Builder then creates a transaction-local materialization product:

```rust
pub struct MaterializedLoopRegionV1 {
    preheader: BasicBlockId,
    header: BasicBlockId,
    body_entry: BasicBlockId,
    latch: BasicBlockId,
    after: BasicBlockId,
}
```

The existing CorePlan route remains explicitly legacy until a later retirement
decision. Canonical Lower may share low-level MIR emission primitives, but not
`LoopRouteContext`, recipe-local AST owners, name-keyed state, or fallback.

Why recommended:

- preserves the B0-L3b authority split;
- keeps exact source coverage pre-Builder;
- prevents current MIR IDs from becoming semantic inputs;
- gives later Break/Continue/Return ports one coherent RegionFlow owner;
- permits role-aware Region materialization without prematurely claiming SA4.

Tradeoff: a canonical Loop recipe/materializer is a new small box instead of
an adapter over the current legacy CorePlan.

### B — version CorePlan into semantic and materialized layers

Split the current name into:

```text
ResolvedCorePlanV1:
  ID-free, exact source + BindingRef flow

MaterializedCorePlanV1:
  ValueId / BasicBlockId / PHI / fragments
```

This can converge legacy and canonical terminology later, but only if the
current `CorePlan` is renamed or physically isolated first. Retrofitting
source fields into the current type without removing Builder-time IDs is not
this option.

Tradeoff: broader BoxShape refactor before the first canonical Loop and a high
risk of mixing legacy name policy into the neutral layer.

### C — attach a coverage sidecar to the current CorePlan

Rejected for the first canonical slice.

A sidecar can say which syntax was visited, but cannot make a Builder-mutating,
ValueId-bearing, name-keyed plan into a pre-Builder semantic authority. It
would certify source coverage after semantic and materialization effects have
already been mixed.

### D — reuse raw suffix plus consumed count

Rejected.

`consumed: usize` cannot prove owner, body, first site, nested child coverage,
or exact plan-node correspondence. Lower must not infer these facts after the
route has erased them.

## Secondary decisions

### 1. Coverage owner and carrier

Recommended:

```text
request:
  LocatedBodySuffixV1

successful outer result:
  ConsumedSourceRangeV1
    exact SourceBodySiteV1
    exact start index
    exact count

complete verified result:
  owner-closed/co-sealed coverage product
    outer consumed range
    ordered exact statement/expression claims for nested plan nodes
```

A bare `ConsumedSourceRangeV1` proves only contiguous outer-body consumption.
It is not sufficient evidence for nested Loop condition/body, BlockExpr, or
future batched plan nodes. The semantic coverage product should be co-sealed
with the ID-free resolved plan/flow, while current mechanical CorePlan remains
a non-owner.

Decision requested:

```text
A1:
  coverage is a field of VerifiedResolvedLoopFlowV1

A2:
  coverage is an owner-closed sidecar paired by one sealed wrapper
```

Recommendation: A2 if coverage will be shared across Loop/CorePlan families;
A1 if the first product is intentionally Loop-only. In both variants, callers
receive one wrapper and cannot mix a plan with foreign coverage.

### 2. First landing

Recommended:

```text
B0-L4a:
  disconnected exact carrier + coverage vocabulary
  exact Loop region bundle
  no Builder connection
  production Loop activation = 0

B0-L4b:
  VerifiedResolvedLoopFlowV1
  closed Loop port/carrier algebra
  no Builder connection

B0-L4c:
  atomic canonical Loop materialization
```

Current RegionFlow has no Loop vocabulary and current CorePlan is already
materialized. Combining all three in the first commit would make the semantic
boundary unreviewable.

If runtime activation is required immediately, restrict the first grammar to:

```text
one ASTNode::Loop family
exact condition + straight-line body
local declarations, assignment, variable use, BlockExpr
false-exit fallthrough
no nested If/Loop
no Break/Continue/Return/QMark/Throw/Try
no LoopRange, suffix multi-consume, or step extraction
```

### 3. Loop port algebra

Before runtime code, the decision must fix:

```text
condition effects on first entry and every backedge
entry/header value source
body-exit/latch value source
loop-carried BindingRef rows
false-exit value source
zero-iteration behavior
eventual Break/Continue port extension point
```

If V1 excludes Break/Continue, their exact resolver records remain sealed but
preflight must reject them before Builder effects. A boolean such as
`has_break` must not create partially supported port states.

### 4. RegionId boundary

Recommended:

```text
B0-L4:
  consume exact Loop pair once
  record coverage
  keep preheader/header/body/latch/after mapping transaction-local

SA4:
  publish durable role-aware RegionId materialization authority
```

Do not publish a scalar `RegionId -> BasicBlockId`: one Loop region owns
multiple block roles.

## Proposed task order after consultation

### B0-L4-S1 — passive exact Loop bundle

```text
ResolvedLoopRegionBundleV1
seal-derived exact site index/query
Loop pair/cardinality/origin/parent/reciprocal verification
production activation = 0
```

### B0-L4-S2 — disconnected located coverage contract

```text
ConsumedSourceRangeV1
ordered nested exact-source coverage vocabulary
LocatedBodySuffixV1 request boundary
foreign/missing/overlap/gap fixtures
Planner/Builder connection = 0
```

### B0-L4-S3 — verified Loop flow

```text
condition/body effects
loop-carried BindingRef rows
header/latch/false-exit source matrix
whole-function exact coverage and preorder
ValueId / BasicBlockId = 0
```

### B0-L4-I1 — disconnected materialization transaction

```text
role-aware transaction-local blocks
BindingRef-directed header PHIs
exact predecessor checks
error-safe value/region/scope/current-block restoration
canonical Loop activation = 0
```

### B0-L4-I2 — atomic first canonical Loop

```text
preflight + verified Loop flow + located source + materializer
all connected in one production commit
legacy retry = 0
coverage and stack verification before function publication
```

The exact series begins only after the consultation chooses A/B and the V1
Loop port algebra.

## Required fixtures and gates

### Identity and source coverage

```text
same Span Loop sites remain distinct
condition and body sites are exact siblings/children as sealed
condition BlockExpr closes before Loop body pair enters
LocatedBodySuffix owner/bounds/start exact
outer consumed range has no gap or overlap
nested statements/expressions covered exactly once
foreign plan/coverage pairing rejected
raw AST clone/pointer/Span/name lookup count = 0
```

### Flow

```text
zero-iteration false exit uses post-condition entry
condition rebind is visible to body and false exit as specified
one loop-carried outer BindingRef
multiple loop-carried outer BindingRefs in deterministic order
body-local and same-name shadow excluded from carrier rows
outer rebind survives loop exit
unsupported exit rejected before Builder
```

### Materialization

```text
actual header/latch/after predecessors exact
every PHI predecessor is an actual CFG predecessor
PHI define succeeds before BindingRef publication
same-input PHI follows the selected baseline policy
error restores value environment, region/scope stacks, and current block
partial function publication = 0
verified MIR and VM/reference result equality
```

### Authority counters

```text
canonical LoopRouteContext construction = 0
canonical legacy CorePlan calls = 0
Lower full-map diff = 0
Lower semantic carrier discovery = 0
String/name binding lookup = 0
RegionFlow ValueId allocation = 0
RegionFlow BasicBlockId allocation = 0
source coverage inferred from consumed usize = 0
silent fallback/retry = 0
durable RegionId materialization publication = 0
```

## May claim after each boundary

### S1/S2

```text
exact Loop identity and located coverage carriers are sealed
production Loop activation remains zero
```

### S3

```text
the selected closed Loop grammar has one immutable pre-Builder flow contract
Lower/Builder production activation remains zero
```

### I2

```text
the selected closed canonical Loop grammar consumes exact source, semantic
identity, Loop flow, and transaction-local block roles without legacy route,
name lookup, or map-diff effect discovery
```

## Must not claim

```text
all Loop/CorePlan families supported
Break/Continue/Return/QMark/Throw/Try ports supported
nested Loop/CorePlan source coverage supported unless explicitly selected
current legacy CorePlan retired
legacy normalization suffix retired
durable RegionId materialization or SA4 cutover
Lambda/capture support
ProgramV0 source authority
default source route cutover
Hako Lower parity
```

## Stop conditions

Stop implementation or publication if a proposal:

```text
uses current ValueId-bearing CorePlan as pre-Builder semantic authority
adds source fields to current CorePlan without removing Builder-time IDs
passes raw AST plus an unrelated source cursor
uses Span, pointer, name, or encounter order as identity
uses consumed usize as complete source coverage proof
lets Lower discover carriers/effects by comparing value maps
lets then/body-mutated state become an independent baseline accidentally
mixes RegionFlow source coverage with MIR block allocation
publishes RegionId-to-block authority before SA4
discovers unsupported exits after Builder effects start
retries the legacy Loop route after canonical failure
mixes the legacy suffix defect fix into this BoxShape series
extends the first slice to Lambda, ProgramV0, or default-route cutover
```

## Separate legacy defect row

Read-only inventory found a likely defect in the dev-only normalization suffix
caller:

```text
successful suffix execution
  -> idx += consumed
  -> no continue
  -> statements[idx] is read in the same iteration
```

If the consumed suffix reaches the body end, this can index past the slice.
This is not yet a runtime-reproduced claim. Track it separately as:

```text
LEGACY-NORMALIZATION-SUFFIX-CONSUMED-INDEX-001

first task:
  add one focused failing fixture for a final consumed Loop suffix

then:
  repair control transfer structurally
  prove exact-once lowering and no out-of-bounds
  keep it outside the B0-L4 canonical authority series
```

## Requested answer

Please decide:

```text
1. architecture:
   A new canonical pre-Builder Loop contract
   or B versioned semantic/materialized CorePlan layers

2. coverage form:
   A1 Loop-flow field
   or A2 one owner-closed paired sidecar

3. first landing:
   disconnected S1/S2/S3 before runtime
   or one immediately atomic narrow runtime family

4. V1 Loop ports:
   false-exit only
   or include exact Break/Continue now

5. RegionId:
   transaction-local role mapping until SA4
   or a different explicitly bounded publication
```

Recommendation:

```text
A + A2
-> disconnected S1/S2/S3
-> false-exit-only first Loop
-> transaction-local role-aware materialization until SA4
```

No production code edit is authorized until these choices are accepted.
