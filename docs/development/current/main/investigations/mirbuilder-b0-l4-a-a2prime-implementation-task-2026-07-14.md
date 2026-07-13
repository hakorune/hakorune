---
Status: Active — A + A2′ accepted; B0-L4-S1 selected
Date: 2026-07-14
Decision: new pre-Builder canonical Loop contract with inseparable generic coverage sidecar
Work mode: Refactor Series Mode; one authority boundary, five code milestones
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-B0-L4-S1-EXACT-LOOP-REGION-BUNDLE-001
Parent:
  - mirbuilder-b0-l4-located-coreplan-source-coverage-consultation-2026-07-14.md
Related:
  - mirbuilder-b0-l4-located-coreplan-source-coverage-design-stop-2026-07-14.md
  - mirbuilder-b0-l3b-a-plus-implementation-task-2026-07-13.md
  - mirbuilder-resolved-region-flow-v1-task-2026-07-13.md
---

# B0-L4 A + A2′ Implementation Task

## Objective

Add one exact-source canonical `ASTNode::Loop` family without treating the
current ValueId-bearing `CorePlan` as a pre-Builder semantic authority.

```text
LocatedBodySuffixV1
+ VerifiedResolvedFunctionV1
        │
        ▼
VerifiedLocatedLoopFlowV1
  ResolvedLoopFlowV1
  + inseparable VerifiedPlanSourceCoverageV1
        │
        ▼
CanonicalLoopMaterializationTxnV1
        │
        ▼
MIR
```

The series preserves the authority split already landed for statement `If`:

```text
resolved_semantics:
  BindingRef / ScopeId / RegionId / exact exit target identity

resolved_region_flow:
  Loop effects, carrier rows, state sources, exact structural coverage

resolved_lowering:
  ValueId / BasicBlockId / PHI / CFG materialization only

current CorePlan:
  explicit legacy materialized route
  canonical authority = 0
```

## Accepted decision

| Boundary | Decision |
| --- | --- |
| Architecture | A — new pre-Builder canonical Loop contract |
| Coverage | A2′ — reusable sidecar type, co-sealed and exposed only through the Loop wrapper |
| Landing order | disconnected S1 → S2 → S3 → disconnected I1 → atomic I2 |
| V1 exit | condition-false only; no optional Break/Continue/Return fields |
| RegionId | exact consume and transaction-local role mapping; durable publication waits for SA4 |

The first V1 has no `has_break`, `falls_through`, optional exit payload, or
legacy retry. Unsupported control is rejected before Builder effects.

## Validation corrections to the consultation response

### 1. Reuse the existing owner-closed input

Do not add a second freely constructible `ResolvedRegionFlowContextV1`.

The repository already has:

```rust
ResolvedFunctionLoweringInputV1<'a> {
    owner,
    source: FunctionSourceViewV1<'a>,
    function: &'a VerifiedResolvedFunctionV1,
    forest: &'a VerifiedSemanticOwnerForestV1,
}
```

It can only be derived from one `VerifiedResolvedSourceUnitV1`. S3 uses this
owner-closed input directly, or a private borrowed view constructible only
from it. It must not accept independently supplied source/function products.

Production continues to call one whole-function RegionFlow analyzer exactly
once. A request-scoped Loop helper may exist internally, but capability must
not run an independent second Loop analysis beside the If analysis.

### 2. S1 site lookup is self-relative

`SourceStmtSiteV1` is owner-relative and carries no owner brand. Therefore:

```rust
VerifiedResolvedFunctionV1::loop_region_bundle(&SourceStmtSiteV1)
```

means “this site within `self.owner()`”. S1 cannot honestly expose a
`ForeignOwner` lookup error for that argument. Cross-owner source/product
pairing is rejected by S3's owner-closed input and co-seal verifier.

Foreign RegionId/ScopeId records remain invalid at the semantic arena seal.

### 3. Source navigation stays in FunctionSourceViewV1

`FunctionSourceViewV1` is the sole safe located-carrier factory. Do not put a
second path grammar in `LocatedBodySuffixV1`.

Use APIs equivalent to:

```rust
fn suffix_first_stmt(
    &self,
    suffix: &LocatedBodySuffixV1<'_>,
) -> Result<LocatedStmtV1<'_>, SourceNavigationErrorV1>;

fn consumed_prefix(
    &self,
    suffix: &LocatedBodySuffixV1<'_>,
    count: NonZeroU32,
) -> Result<ConsumedSourceRangeV1, SourceNavigationErrorV1>;

fn advance_body_suffix(
    &self,
    suffix: LocatedBodySuffixV1<'_>,
    range: &ConsumedSourceRangeV1,
) -> Result<LocatedBodySuffixV1<'_>, SourceNavigationErrorV1>;
```

The Loop analyzer borrows `&LocatedBodySuffixV1`; it does not move the request
before the body driver advances it.

All `usize -> u32` conversions and `start + count` operations become checked.
An empty suffix has a typed error; it is not `None` and does not try another
route.

### 4. Generic schema and Loop completeness are different proofs

S2 may prove only the generic coverage schema:

```text
exact owner/body/start/range
nonzero bounded count
typed structural preorder
no duplicate same typed site
no independently constructible verified product
```

S3 proves completeness against the selected Loop syntax subtree:

```text
no missing or unexpected Loop source node
exact condition/body order
all exact assignment targets owned once
flow effects and structural coverage describe the same subtree
```

The existing `resolved_region_flow/coverage.rs` remains If assignment-site
coverage. It is not renamed into the A2′ structural coverage sidecar.

### 5. Private construction is the seal

`VerifiedLocatedLoopFlowV1` and `VerifiedPlanSourceCoverageV1` have private
fields and verifier-only constructors. A ceremonial runtime `LoopFlowSealV1`
field is unnecessary.

The wrapper exposes borrowed views only:

```rust
fn flow(&self) -> &ResolvedLoopFlowV1;
fn coverage(&self) -> &VerifiedPlanSourceCoverageV1;
```

Forbidden:

```text
into_parts on the individual Loop wrapper
public coverage constructor
Clone of the standalone verified coverage sidecar
flow and coverage supplied as separate Lower arguments
```

The whole-function flow transport may consume its owned rows once at I2.

### 6. Coverage use is an ordered cursor, not a new BitSet dependency

The repository has no shared `BitSet` vocabulary. Because coverage is sealed
in structural preorder, Lower uses one exact cursor:

```rust
LoweringCoverageUseLedgerV1 {
    expected: VerifiedPlanSourceCoverageV1,
    next: usize,
}
```

Conceptually it claims the next typed site and finishes at exact length. It
does not add, reorder, or discover coverage. An equivalent private compact
representation is allowed, but no dependency or unordered second authority is
introduced.

### 7. Initial canonical expression grammar has no calls

Current exact canonical Lower accepts only:

```text
Literal
Variable
BinaryOp except And/Or
BlockExpr over the same closed grammar
```

It has no exact Call arm. Therefore B0-L4-I2 call support is zero unless an
independent exact-call slice lands first. `Outbox`, short-circuit `And/Or`,
LoopRange/ForRange, and every other expression/statement family remain
preflight rejects.

### 8. Materialization ordering corrections

```text
E = Loop entry
H = header PHI environment
C = shared post-condition environment
B = body fallthrough after body locals retire
```

The required corrections are:

1. lower condition in the surrounding semantic stacks and close any condition
   BlockExpr before entering the Loop/LoopBody pair around body only;
2. capture `B` then restore the carrier-scoped compile-time state to `C`, so
   false exit already owns `C` and never restores the full environment to `E`;
3. after every provisional PHI is Defined, batch-rebind existing carrier
   BindingRefs to `H`; this is not declaration publication;
4. use `PhiTxn`, preserve primary plus cleanup errors, and verify computed and
   cached predecessor sets before `patch_phi_inputs()`;
5. before Loop-local frames, validate whole effects against any enclosing If
   frame through `prime_current_effects`.

A failed provisional definition publishes zero header rebinds. Final MIR
verification remains mandatory because patching itself does not prove CFG
predecessors or dominance.

## Closed V1 grammar

### Loop placement

S3 accepts a Loop request from any owner-closed `LocatedBodyV1` in the current
function. This avoids a root-only source special case. I2 may materialize such
a row when every enclosing construct is already in the canonical grammar.

The restrictions below apply inside the Loop condition and Loop body.

### Accepted

```text
ASTNode::Loop statement

condition:
  SharedPostState only
  Literal / Variable / eager non-And/Or BinaryOp / BlockExpr

body:
  fallthrough only
  Local
  binding Assignment
  Literal / Variable / eager non-And/Or BinaryOp / BlockExpr statements

effects:
  outer BindingRef rebind in condition and/or body
  body-local declaration/use/rebind
  same-name body-local shadow

runtime:
  zero, one, or multiple iterations
  condition-false is the only external exit
```

### Rejected before Builder

```text
nested If inside Loop condition/body
nested Loop inside Loop condition/body
Break / Continue / Return
QMarkPropagate / Throw / Try/Catch/Finally
And / Or or any outcome-dependent condition state
Call / MethodCall / FunctionCall
Outbox
LoopRange / ForRange
Lambda execution
step extraction
multi-statement suffix consumption
```

Resolver-unsupported QMark/Throw/Try fail at resolver/preflight. RegionFlow
must not fabricate an exit record that the semantic product never published.

## Product contracts

### ResolvedLoopRegionBundleV1

```rust
pub(crate) struct ResolvedLoopRegionBundleV1 {
    loop_pair: ResolvedScopeRegionPairV1,
}
```

The private seal-derived index belongs only to
`VerifiedResolvedFunctionV1`. Draft/data state remains index-free.

```rust
impl VerifiedResolvedFunctionV1 {
    pub(crate) fn loop_region_bundle(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Result<&ResolvedLoopRegionBundleV1, ResolvedLoopRegionLookupErrorV1>;

    pub(crate) fn loop_region_bundle_count(&self) -> usize;
}
```

The verifier proves:

```text
region.kind == Loop
scope.kind == LoopBody
region.origin == exact Loop statement site
scope.origin == exact LoopBodyRoot site
region.parent == exact surrounding region
scope.parent == exact surrounding lexical scope
region.lexical_scope == scope
scope.owner_region == region
exactly one bundle per Loop site
all Loop regions and LoopBody scopes accounted
```

### Generic coverage carrier

```rust
pub(crate) struct ConsumedSourceRangeV1 {
    body: SourceBodySiteV1,
    start: u32,
    count: NonZeroU32,
}

pub(crate) enum CoveredSourceSiteV1 {
    Body(SourceBodySiteV1),
    Statement(SourceStmtSiteV1),
    Expression(SourceExprSiteV1),
}

pub(crate) struct VerifiedPlanSourceCoverageV1 {
    outer: ConsumedSourceRangeV1,
    preorder: Box<[CoveredSourceSiteV1]>,
}
```

Only `FunctionSourceViewV1` creates a range. Only the S3 co-seal verifier
creates a verified coverage sidecar. S2 test-only constructors do not escape
their module.

For the first Loop:

```text
outer.body == request.body.site
outer.start == request.start_index
outer.count == 1
```

Two typed variants may intentionally refer to the same raw source-node path,
such as an expression statement represented as both Statement and Expression.
Duplicate means the same enum variant and exact site appears twice.

### Structural preorder

The V1 order is fixed:

```text
Loop Statement site
condition expression preorder
Loop Body site
body statement/expression preorder
```

BlockExpr expression preorder includes its expression site, prelude Body site,
prelude statements/expressions, and tail expression subtree in source order.

### VerifiedLocatedLoopFlowV1

```rust
pub(crate) struct VerifiedLocatedLoopFlowV1 {
    owner: FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    flow: ResolvedLoopFlowV1,
    coverage: VerifiedPlanSourceCoverageV1,
}

pub(crate) struct ResolvedLoopFlowV1 {
    regions: ResolvedLoopRegionBundleV1,
    condition: VerifiedLoopConditionFlowV1,
    body: VerifiedLoopBodyFlowV1,
    carriers: ResolvedLoopCarrierContractV1,
}
```

```rust
pub(crate) enum VerifiedLoopConditionFlowV1 {
    SharedPostState {
        may_rebind_outer: Box<[BindingRefV1]>,
    },
}

pub(crate) struct VerifiedLoopBodyFlowV1 {
    fallthrough: ResolvedBackedgePortV1,
}

pub(crate) struct ResolvedBackedgePortV1 {
    may_rebind_outer: Box<[BindingRefV1]>,
}

pub(crate) struct ResolvedLoopCarrierContractV1 {
    rows: Box<[ResolvedLoopCarrierRowV1]>,
}

pub(crate) struct ResolvedLoopCarrierRowV1 {
    binding: BindingRefV1,
}
```

No exit enum exists in V1. The only path leaving the Loop region is the
condition-false edge defined by the product type.

### Carrier set

```text
K = sorted unique(
      condition.SharedPostState.may_rebind_outer
      union body.fallthrough.may_rebind_outer
    )
```

Rows use `BindingRefV1::Ord` within one owner. Raw BindingIds are not used as
cross-run parity keys; normalized source origins remain the parity authority.

The verifier proves:

```text
every K binding belongs to current owner
every K binding is visible at Loop entry
body-local bindings intersect K = empty
all possible outer rebinds reaching backedge are in K
all possible outer rebinds reaching false exit are in K
rows are exact, duplicate-free, and sorted
```

Over-approximation may produce an extra PHI and remains semantically sound.
Under-approximation is a contract error. Lower may verify actual rebinds are a
subset of K; it may not derive K from those observations.

## State and materialization contract

### Zero-iteration law

```text
E -> H -> evaluate condition -> C

C + true:
  enter Loop pair
  body -> B
  leave pair
  latch -> H

C + false:
  after
```

For every carrier `b`:

```text
H[b] = phi(preheader: E[b], latch: B[b])
After[b] = C[b]
```

Zero iterations still evaluate condition exactly once. Condition effects are
present in `C`; body effects are absent.

### Transaction-local block roles

```rust
pub(crate) struct MaterializedLoopRegionV1 {
    preheader: BasicBlockId,
    header: BasicBlockId,
    body_entry: BasicBlockId,
    latch: BasicBlockId,
    after: BasicBlockId,
}
```

This value exists only inside one canonical Loop materialization transaction.
It carries no RegionId and is never inserted into a durable map.

Required predecessor sets:

```text
header = {preheader, latch}
body_entry = {header}
after = {header}
latch = {actual body fallthrough exit}
```

The actual body exit may differ from `body_entry` because a body BlockExpr can
emit internal blocks. The materializer records and verifies the real exit.

### PHI lifecycle

```text
1. capture ordered E rows for K
2. allocate every carrier dst
3. define every header provisional PHI
4. only after all definitions, batch-rebind K to H
5. lower condition to C outside Loop pair
6. lower body from C inside exact Loop pair
7. capture B and restore compile-time K to C
8. verify actual CFG predecessors
9. patch every PHI with (preheader,E), (latch,B)
10. commit PhiTxn
11. enter after with C still active
```

Same-input rows still receive a PHI. Elimination belongs to a later canonical
simplifier.

On error, restore the carrier environment and semantic/effect stacks. Partial
MIR remains only in the unpublished function draft, which the function session
discards. Cleanup errors preserve the primary error.

## Implementation series

### B0-L4-S1 — passive exact Loop bundle

Files:

```text
src/mir/resolved_semantics/loop_region.rs
src/mir/resolved_semantics/loop_region_tests.rs
src/mir/resolved_semantics/{mod.rs,product.rs,verifier.rs,README.md}
tools/checks/lib/resolved_loop_lowering_contract.sh
tools/checks/lib/resolved_control_lowering_contract.sh
tools/checks/resolved_region_flow_authority_guard.sh
```

Tasks:

```text
seal-derived Loop site index
self-relative point query + count
exact pair/origin/parent/reciprocal validation
all Loop/LoopBody records accounted
RegionFlow/Builder/Lower callers = 0
production Loop activation = 0
```

Fixtures:

```text
root, nested-parent-body, and same-Span sibling Loop sites
exact IDs and origins
missing/wrong Loop region or LoopBody scope
wrong region/scope parent
broken reciprocal link
duplicate exact origin/cardinality
orphan Loop region or LoopBody scope
missing site lookup
foreign source/product closure explicitly deferred to S3
```

Line discipline:

```text
new Loop index/verifier logic stays in loop_region.rs
new fixtures stay out of the existing large If fixture
verifier.rs receives only thin derived-artifact wiring
797-line authority guard does not grow
```

### B0-L4-S2 — generic located coverage carrier

Files:

```text
src/mir/compiler/{located.rs,source_view.rs,source_view_tests.rs}
src/mir/resolved_region_flow/plan_source_coverage.rs
src/mir/resolved_region_flow/plan_source_coverage_tests.rs
src/mir/resolved_region_flow/{mod.rs,README.md}
tools/checks/lib/resolved_loop_lowering_contract.sh
```

Tasks:

```text
ConsumedSourceRangeV1 with NonZeroU32 count
checked suffix first/range/advance navigation in FunctionSourceViewV1
typed empty/overflow/body/start/gap/bounds failures
CoveredSourceSiteV1 and private generic verified sidecar
schema-level owner/order/duplicate verification
Loop analyzer/Planner/Builder connection = 0
```

Fixtures:

```text
count=1 exact range
start at beginning/middle/end
empty suffix typed reject
usize/u32 and start+count overflow reject
wrong owner/body/start
gap, overlap, reused range
advance exactly to body end
same-Span sites remain distinct
duplicate same typed site reject
typed Statement/Expression at one raw path remain distinct
raw pointer/Span/name fields = 0
```

S2 does not claim Loop subtree completeness.

### B0-L4-S3 — disconnected verified Loop flow

Files:

```text
src/mir/resolved_region_flow/loop_flow.rs
src/mir/resolved_region_flow/loop_ports.rs
src/mir/resolved_region_flow/loop_analyzer.rs
src/mir/resolved_region_flow/loop_verifier.rs
src/mir/resolved_region_flow/loop_flow_tests.rs
src/mir/resolved_region_flow/{mod.rs,README.md}
tools/checks/lib/resolved_loop_lowering_contract.sh
```

Tasks:

```text
request-scoped private Loop analysis over borrowed located suffix
SharedPostState condition proof
fallthrough-only body proof
exact outer effects and local/shadow filtering
sorted exact carrier union
generic coverage + flow inseparable co-seal
exact Loop assignment and structural coverage
typed unsupported exit/control errors
Builder/Lower production connection = 0
```

The request-scoped helper is not a second production analyzer. At I2 the
existing single whole-function analyzer calls it internally and owns all If
and Loop rows.

Fixtures:

```text
no effects -> zero carriers
condition-only / body-only / both
multiple carriers sorted and deduplicated
body-local and same-name shadow excluded
condition BlockExpr local excluded, outer rebind included
Loop Statement -> condition -> Body -> body preorder exact
missing/unexpected/duplicate/foreign coverage reject
same-Span Loop sites distinct
And/Or split-state reject
nested If/Loop reject
Break/Continue/Return exact typed reject
QMark/Throw/Try resolver/preflight reject without fabricated flow record
partial flow/coverage publication = 0
zero-iteration product law
```

### B0-L4-I1 — disconnected Loop materialization transaction

Files:

```text
src/mir/builder/resolved_lowering/loop_carrier_transaction.rs
src/mir/builder/resolved_lowering/loop_carrier_transaction_tests.rs
src/mir/builder/resolved_lowering/loop_materialization.rs
src/mir/builder/resolved_lowering/loop_materialization_tests.rs
src/mir/builder/resolved_lowering/semantic_stack.rs
src/mir/builder/resolved_lowering/semantic_stack_tests.rs
src/mir/builder/emission/phi_lifecycle.rs
src/mir/builder/resolved_lowering/{mod.rs,README.md}
tools/checks/lib/resolved_loop_lowering_contract.sh
```

Subtasks:

```text
I1a:
  harden PhiTxn abort to attempt all cleanup and preserve primary error

I1b:
  ordered K-only E/H/C/B carrier transaction
  all provisional definitions before one header batch rebind
  body capture restores C, not E

I1c:
  transaction-local five-role CFG session
  computed/cached predecessor equality before PHI patch
  PhiTxn patch/commit and same-input PHI retention

I1d:
  semantic stack expected/consumed Loop-pair count
  enter_loop_body around body only
```

Unit fixtures use mechanical rows and mock stores only. They do not accept raw
AST, Located nodes, `VerifiedLocatedLoopFlowV1`, names, or value-map discovery.

Required failure fixtures:

```text
duplicate/foreign/out-of-domain K
later provisional define fails -> header rebind count 0
body error restores C/E as owned by the transaction
wrong/disconnected body exit
wrong cached predecessor set
PHI patch failure
cleanup failure preserves primary and attempts all cleanup
semantic Loop pair reconsume/unbalanced leave
production canonical Loop callers = 0
```

### B0-L4-I2 — atomic first canonical Loop

This commit alone connects every production piece:

```text
capability admits only the closed V1 grammar
one whole-function pre-Builder analyzer owns If + Loop rows
CanonicalFirstFamilyPlanV1 transports the owned flow once
body driver uses typed suffix request/range advance
flow and structural coverage cursors consume exact rows once
Loop whole effects are primed against enclosing frames
condition lowers outside the exact Loop pair
body lowers inside the pair
carrier transaction materializes header PHIs and false exit C
coverage/stack/identity finish before function publication
legacy retry = 0
```

No intermediate commit may admit Loop syntax without Lower support or connect
Lower without a sealed flow/coverage wrapper.

Runtime fixtures:

```text
zero iteration: condition effects once, body effects zero
one iteration
multiple iterations
condition-only outer rebind
body-only outer rebind
condition + body rebind of same binding
multiple deterministic carrier rows
body local excluded and retired
same-name body-local shadow restores outer binding
condition BlockExpr local does not leak
condition BlockExpr outer rebind reaches body and after
outer rebind survives after
Loop in an already canonical parent body uses parent effect authorization
actual preheader/header/body/latch/after predecessors exact
every PHI input predecessor is an actual CFG predecessor
same-input carrier PHI remains present
MIR verifier and VM/reference values green
```

Preflight fixtures:

```text
nested If/Loop in Loop body
Break/Continue/Return
And/Or split-state condition
Call/Outbox/LoopRange
QMark/Throw/Try resolver rejection
unsupported failure before Builder/function draft effects
canonical failure never retries LoopRouteContext/CorePlan
```

Error injection:

```text
condition
Loop pair enter
first/middle body statement
body pair leave
coverage finish
PHI patch/commit
semantic finish
```

Every error proves:

```text
value environment restored
current block restored
effect stack restored
region/scope stacks restored
primary + cleanup errors preserved
partial function/module publication = 0
```

## Guard structure and line budget

`tools/checks/resolved_region_flow_authority_guard.sh` is 797 lines and must
not grow.

Replace its current If helper source/call lines in place with one reusable
aggregator:

```text
tools/checks/lib/resolved_control_lowering_contract.sh
  sources/calls existing If helper
  sources/calls new resolved_loop_lowering_contract.sh
```

This keeps the top guard line-neutral. The Loop helper is reused from S1
through I2; it is not a one-row public guard.

All new and modified B0-L4 source/check files remain below 800 lines. Keep:

```text
Loop semantic index out of verifier.rs
Loop flow out of current analyzer.rs
Loop materialization out of branch_transaction.rs and located_if.rs
Loop fixtures in dedicated test files
```

The authority helper checks:

```text
new source files < 800
current top guard < 800
current CorePlan/LoopRouteContext imports in canonical path = 0
raw AST/cursor/name/map-diff authority = 0
RegionFlow ValueId/BasicBlockId/MirBuilder imports = 0
standalone flow/coverage split APIs = 0
durable RegionId materialization map = 0
production activation count matches the current slice
```

## Per-slice gates

Every code milestone is one green commit and push. Each runs:

```bash
bash tools/checks/resolved_region_flow_authority_guard.sh
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Focused tests added to that common set:

| Slice | Focused command |
| --- | --- |
| S1 | `cargo test -q --lib mir::resolved_semantics::loop_region_tests` |
| S2 | `cargo test -q --lib mir::compiler::source_view_tests` and `cargo test -q --lib mir::resolved_region_flow::plan_source_coverage_tests` |
| S3 | `cargo test -q --lib mir::resolved_region_flow` |
| I1 | `cargo test -q --lib mir::builder::resolved_lowering` |
| I2 | all S1/S3/I1 tests plus `cargo test -q --features vm-reference --lib mir::builder::resolved_lowering::loop_tests` |

## Authority counters

Through S3/I1:

```text
canonical Loop production activation = 0
canonical LoopRouteContext constructions = 0
canonical current CorePlan calls = 0
Lower full-map diff = 0
Lower carrier discovery = 0
String/name binding lookup = 0
RegionFlow ValueId allocation = 0
RegionFlow BasicBlockId allocation = 0
coverage inferred from consumed usize = 0
silent retry/fallback = 0
durable RegionId materialization publication = 0
```

After I2 only the first counter becomes the one closed canonical V1 family.
Every other zero remains invariant.

## May claim

| Slice | Maximum claim |
| --- | --- |
| S1 | exact statement Loop → Loop/LoopBody pair lookup is seal-derived with no arena scan; consumers remain zero |
| S2 | generic exact located range and structural coverage schema are sealed; Loop completeness/activation remain zero |
| S3 | the closed fallthrough-only grammar has one immutable ID-free flow and inseparable coverage product; Builder connection is zero |
| I1 | Loop carrier/CFG/PHI transaction and Loop-pair ledger are tested; production activation remains zero |
| I2 | the closed Loop V1 consumes exact source/identity, sorted carriers, shared post-condition state, coverage, and local block roles without legacy route, names, or map-diff discovery |

## Must not claim

```text
all Loop/CorePlan families supported
Break/Continue/Return/QMark/Throw/Try ports supported
nested If/Loop inside Loop supported
outcome-dependent condition state supported
Call/Outbox/LoopRange supported by canonical Loop
current CorePlan or normalization suffix retired
durable RegionId materialization or SA4 cutover
Lambda/capture support
ProgramV0 source authority
default source route cutover
Hako Lower parity
```

## Stop conditions

Stop implementation or publication if any slice:

```text
uses current CorePlan as pre-Builder semantic authority
adds source fields only to current CorePlan
passes raw AST plus an independent source cursor
creates another freely constructible flow context
uses Span, pointer, name, or encounter order to recover identity
uses consumed usize as complete coverage proof
moves suffix navigation grammar outside FunctionSourceViewV1
exposes standalone verified coverage construction or flow/coverage into_parts
lets Lower discover carriers from value-map differences
enters the Loop pair while lowering condition
uses body-mutated B as false-exit state
restores false exit to E instead of C
puts body-local BindingRefs in K
republishes carrier declarations instead of rebinding existing bindings
publishes any header carrier before every provisional PHI is Defined
patches PHIs before actual predecessor verification
lets PHI cleanup overwrite the primary error
adds bool-shaped partial Break/Continue support
discovers unsupported control after Builder effects
allocates ValueId/BasicBlockId in RegionFlow
publishes scalar or durable RegionId -> BasicBlockId state
retries the legacy Loop route after canonical failure
publishes function/module before all coverage/stacks finish
mixes the legacy suffix defect fix into this authority series
adds Lambda, ProgramV0, or default-route cutover to B0-L4
lets any new/modified source/check file reach 800 lines
```

## Parked sibling defect

The inventory found:

```text
LEGACY-NORMALIZATION-SUFFIX-CONSUMED-INDEX-001
```

It remains outside this series. Its eventual independent task is:

```text
1. focused failing fixture for a final consumed Loop suffix
2. validate 0 < consumed <= remaining.len()
3. structurally continue after typed index advance
4. prove exact-once lowering and no out-of-bounds
5. one separate commit and push
```

Do not fix or claim the defect from static inspection alone during B0-L4.

## Immediate next action

Implement B0-L4-S1 only:

```text
exact Loop pair index/query/count
Loop-specific seal verification
focused identity fixtures
line-neutral reusable authority guard entry
production activation delta = 0
```
