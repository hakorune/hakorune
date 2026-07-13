---
Status: P0/E0/OF0/UP0/UP1/B0-D/B0-P/B0-S/B0-F/B0-L0/B0-L1/B0-L2a/B0-L2b/B0-L2c/SA3-B/B0-L3a closed; B0-C skipped; B0-L3b canonical If branch-flow design stop is active
Date: 2026-07-13
Scope: Resolved Semantic Owner Forest V1 design and implementation task order
Parent: mirbuilder-resolved-region-flow-v1-task-2026-07-13.md
Decision: A-double-prime sealed_owner_forest_with_structural_upvar_edges
---

# Resolved Semantic Owner Forest V1 — Decision and Task Card

## Outcome

The nested-owner design stop is closed with the smallest owner-forest model:

```text
canonical owner syntax
  -> SemanticOwnerForestResolverSessionV1
  -> owner-local sealed products
  -> forest verification
  -> VerifiedSemanticOwnerForestV1
  -> Planner / RegionFlow
  -> Lower materialization
```

The final semantic vocabulary is:

```text
FunctionOwnerIdV1:
  one function/lambda owner membership handle

BindingId:
  one source lexical declaration

UpvarRefV1:
  one child-owner reference to an ancestor BindingRefV1

ResolvedExitSiteV1:
  exact statement or expression exit origin

VerifiedRegionFlowV1:
  normal/exit/result/cleanup/state flow

ValueId / BasicBlockId:
  Lower-only MIR identity
```

There is no resolver-owned `CaptureId`, no child synthetic capture
`BindingId`, and no resolver-owned closure layout slot. A dense
`CaptureSlotId` may be created later only by a verified capture transport
plan.

## Current evidence

The disconnected canonical resolver producer, seal/verifier, exhaustive
57-variant classifier, leaf-expression traversal, assignment-target
resolution, Nowait binding identity, and TaskScope/FastMem lexical containers
are green. Production resolver installation remains zero.

The current code also fixes the migration seams that this card must replace:

```text
Lambda Lower:
  HashSet<String> free-variable rediscovery

resolved function product:
  parallel control_exits / control_exit_regions maps

SourceExprSiteV1:
  relative to one owner root, not globally unique in a forest

BlockExpr Lower:
  sequential prelude today, no lexical-scope push/pop

canonical surface:
  throw prohibited
  question-mark propagation noncanonical
```

Therefore owner topology, exact cross-owner source identity, and exit record
shape may be implemented now. Capture mode, QMark language activation,
source `throw`, BlockExpr scope semantics, match result flow, and cleanup flow
remain separate decisions or later verified products.

## Canonical authority

```text
owner-local declarations/scopes/regions:
  SealedResolvedOwnerV1

cross-owner topology and Upvar validity:
  VerifiedSemanticOwnerForestV1

source syntax and execution order:
  canonical AST

source provenance:
  owner-branded Source*SiteV1 values

capture mode and runtime transport:
  future VerifiedCapturePlanV1

control/state execution:
  future VerifiedRegionFlowV1

MIR materialization:
  Lower
```

`VerifiedResolvedFunctionV1` remains the current migration name. Before
cross-owner references are publicly consumable, its final role must be made
explicit as either `SealedResolvedOwnerV1` or an owner-local product view.
Only the verified forest is complete cross-owner authority.

## Non-authority

```text
variable/function names
raw owner-local integer IDs
source path by itself
AST pointer identity
first capture use
HashSet iteration order
Lower loop-stack depth
Recipe success alone
CaptureSlotId before capture planning
ProgramV0
```

## Minimal data model

### Owner topology

The forest stores only owner products and parent edges as primary data.

```rust
pub struct VerifiedSemanticOwnerForestV1 {
    owners: BTreeMap<FunctionOwnerIdV1, SealedResolvedOwnerV1>,
    parents: BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>,

    // roots, child_at, and upvar indexes are seal-derived witnesses.
}
```

```rust
pub struct OwnerParentEdgeV1 {
    parent_owner: FunctionOwnerIdV1,
    definition_site: OwnedExprSiteV1,
    parent_scope: ScopeId,
}
```

`roots`, `child_at`, and the unique Upvar inventory must not be mutable
parallel authorities. Forest seal derives them from `owners`, `parents`, and
resolved lexical references.

### Owner-branded source sites

`SourceExprSiteV1` is relative to one owner root. Every cross-owner index must
therefore carry its owner explicitly.

```rust
pub struct OwnedExprSiteV1 {
    owner: FunctionOwnerIdV1,
    site: SourceExprSiteV1,
}
```

The same pattern may be used for statement/node sites when they cross an
owner boundary. A forest map keyed by bare `SourceExprSiteV1` is forbidden.

### Structural Upvar relation

```rust
pub struct UpvarRefV1 {
    capturing_owner: FunctionOwnerIdV1,
    source: BindingRefV1,
}
```

```rust
pub enum ResolvedLexicalRefV1 {
    Local(BindingRefV1),
    Upvar(UpvarRefV1),
}
```

The source of an Upvar is the original declaration in a strict lexical
ancestor. Grandparent capture is represented directly:

```text
inner UpvarRefV1
  -> grandparent BindingRefV1
```

Intermediate runtime forwarding is not a resolver fact. A later
`VerifiedCapturePlanV1` may create forwarding slots while preserving the
direct semantic source edge.

Multiple uses of the same `(capturing_owner, source BindingRefV1)` relation
are naturally one Upvar. No integer capture allocator or capture arena is
needed.

### Exact exit record

```rust
pub enum ResolvedExitSiteV1 {
    Statement(SourceStmtSiteV1),
    Expression(SourceExprSiteV1),
}
```

```rust
pub struct ResolvedExitRecordV1 {
    source_region: RegionId,
    origin: ResolvedExitOriginV1,
    transfer: ResolvedControlTransferV1,
}
```

```text
resolved_exits:
  BTreeMap<ResolvedExitSiteV1, ResolvedExitRecordV1>
```

This one record replaces the current parallel `control_exits` and
`control_exit_regions` maps.

The first accepted transfer vocabulary remains:

```text
Continue(target_loop)
Break(target_loop)
Return(target_function)
```

If the existing QMark compatibility path must be retained, it is represented
without losing origin:

```text
origin:
  QMarkPropagate

transfer:
  Return(current function)
```

This does not activate `?` as canonical Result/Option syntax. Source `throw`
is prohibited by the current language SSOT and is not part of the first exit
vocabulary.

## Construction and sealing

One resolver session owns the existing function-owner issuer. A separate
owner-inventory authority or prepass is not required.

```text
enter root owner
  -> allocate owner-local binding/scope/region IDs
  -> encounter Lambda
     -> allocate child owner
     -> record parent edge and exact OwnedExprSiteV1
     -> resolve child with ancestor lexical frames visible
     -> local seal child
  -> local seal parent
  -> verify and seal complete forest
```

Name lookup first searches current-owner lexical scopes. If it reaches a
strict ancestor owner, it records `UpvarRefV1`. It never creates a child
capture BindingId.

Initializer-before-declaration ordering remains unchanged. Therefore implicit
self/recursive capture is not backpatched. Recursive closure support requires
a later closed `RecursiveOwnerGroupV1`-style decision and remains
Unsupported.

Local sealing verifies owner-local shape. Forest sealing verifies:

```text
products key == product owner
every non-root owner has exactly one parent
parent graph acyclic
definition site belongs to parent owner
parent_scope belongs to parent owner
derived child_at index is unique
every Upvar source exists
Upvar source owner is a strict ancestor
source declaration is visible at the lexical definition chain
sibling/descendant/foreign-owner Upvar rejected
partial forest publication = 0
```

Planner and RegionFlow receive only a forest-derived verified owner view.
They do not consume a standalone owner product containing unchecked foreign
references.

## Resolver / RegionFlow / Lower split

```text
Semantic resolver owns:
  owner/binding/scope/region identity
  local and Upvar reference resolution
  exact declaration/use/assignment indices
  exact break/continue/return target identity
  match/catch/pattern binder identity

RegionFlow owns:
  branch/arm result flow
  QMark success/early-return flow
  catch/cleanup propagation
  per-port binding effects
  edge state closure

CapturePlan owns:
  by-value/cell/weak/move policy
  transitive forwarding
  environment field ordering
  CaptureSlotId

Lower owns:
  BindingId/Upvar/CaptureSlot -> ValueId
  RegionId -> BasicBlockId
  verified edge arguments and MIR instructions
```

Lower may read syntax to emit expression operations. It may not rediscover
names, free variables, capture mode, loop targets, match merge policy, or
cleanup obligations.

## Match / cleanup / BlockExpr boundary

Match identity may use existing IDs only:

```text
RegionId:
  Match / MatchArm

ScopeId:
  arm lexical scope when a binder/lifetime requires it

BindingId:
  PatternBinder
```

There is no separate ArmId. Match dispatch, result merge, and PHI policy are
RegionFlow/Lower responsibilities.

`TryCatch` is the current compatibility carrier for catch/cleanup. Semantic
vocabulary should call the finally-style region `Cleanup`, matching the
canonical surface. Catch selection and cleanup continuation are not resolver
facts. Source `throw` remains rejected.

The clean final BlockExpr model is lexical scope + ordered prelude + one tail
result. Current Lower does not create that lexical scope and the language
document remains provisional. Therefore scope activation requires its own
language `Decision: accepted` and differential gate. The owner-forest
migration must not silently change BlockExpr visibility.

## Implementation task order

Only one row is active at a time. Every row is a separate green commit.

### P0 — closed source-role vocabulary (closed)

Add exact path roles needed by the remaining owner/control variants without
changing resolver acceptance:

```text
LambdaBodyRoot / LambdaBody(index)
QMarkOperand
MatchScrutinee / MatchArm(index) / MatchElse
EnumMatchScrutinee / EnumMatchArm(index) / EnumMatchElse
BlockExprPreludeRoot / BlockExprPrelude(index) / BlockExprTail
TryBodyRoot / TryBody(index)
CatchClause(index) / CatchBodyRoot / CatchBody(index)
CleanupBodyRoot / CleanupBody(index)
```

Also add `OwnedExprSiteV1` as passive owner-branded provenance. Do not add
owner forest, Upvar, exit behavior, or new accepted syntax in P0.

P0 gates:

```text
all new path roles have deterministic equality/order/debug formatting
same relative expression site in two owners remains distinct when branded
existing normalized graph fixtures unchanged
resolver accepted-variant count unchanged
Planner/Lower imports = 0
all source files < 800 lines
```

P0 closeout evidence:

```text
closed path roles = 19
owner-branded passive expression provenance = OwnedExprSiteV1
same relative site across two owners = distinct
resolver accepted vocabulary change = 0
owner forest / Upvar / exit behavior installation = 0
Planner / Lower connection = 0
resolved-region-flow-authority guard = green
release compiler check = green
all source files < 800 lines
```

### E0 — behavior-neutral exit record unification (closed)

```text
add ResolvedExitSiteV1
add ResolvedExitRecordV1
merge control_exits + control_exit_regions
migrate Break/Continue/Return only
verify exact source region and target ancestry
QMark/Throw acceptance unchanged
```

E0 closeout evidence:

```text
canonical and shadow parallel exit maps = 0
one atomic source-region/origin/transfer record = active
accepted transfers = explicit Break / Continue / Return only
Expression exit publication = 0
QMark / Throw acceptance change = 0
source-region containment = closed root/member role policy
top-level Return canonical resolver fixture = green
loop Break/Continue canonical resolver fixture = green
normalized exit parity across owner/raw IDs = green
resolved-region-flow-authority guard = green
release compiler check = green
all source files < 800 lines
```

### OF0 — non-capturing owner forest (closed)

```text
reuse existing FunctionOwnerIssuerV1 inside one forest resolver session
add owner product map and parent edges
resolve one non-capturing Lambda as a child owner
derive roots and child_at during seal
child return targets child function region
capture encounter -> exact Unsupported
AST clone / ValueId / BasicBlockId = 0
Planner/RegionFlow/Lower connection = 0
```

OF0 closeout evidence:

```text
forest primary authority = sealed owner products + child-to-parent edges
forest-owned owner products = direct ownership; per-owner Arc escape = 0
derived witnesses = one root + exact child_at + normalized forest graph
non-capturing Lambda child owner = green
child parameter/local BindingId domain = child owner-local
child Return target = child function region
strict-ancestor variable/receiver read = structural UpvarRefV1
strict-ancestor rebind = exact UnsupportedUpvarRebind
initializer self-reference backpatch = 0
second root / mixed compilation / cycle / duplicate parent = rejected
parent_scope = exact lexical definition scope, verified at seal
AST clone / ValueId / BasicBlockId = 0
structural read-only UpvarRefV1 = 1
CaptureId / synthetic child BindingId / capture mode / runtime slot = 0
Planner / RegionFlow / Lower connection = 0
normalized forest parity across independent owner issuers = green
resolved-region-flow-authority guard = green
release compiler check = green
all source files < 800 lines
```

### UP0 — read-only structural Upvar (closed)

```text
add UpvarRefV1 and ResolvedLexicalRefV1
resolve outer parameter/local/receiver reads
deduplicate by structural relation
grandparent source points directly to original BindingRefV1
capture mode/layout/slot allocation = 0
Lower connection = 0
```

UP0 closeout evidence:

```text
variable-use authority = ResolvedLexicalRefV1(Local | Upvar)
Upvar identity allocator = 0
Upvar relation = capturing owner + original strict-ancestor BindingRefV1
outer local / parameter / receiver reads = green
multiple reads of one relation = deduplicated
grandparent reference = direct original BindingRefV1
nearer local shadow = local reference; no Upvar
later declaration / initializer self-reference = not visible
forest seal checks source existence / strict ancestry / visibility / shadowing
normalized parity uses NormalizedOwnerKeyV1; raw owner IDs = 0
Upvar rebind = exact UnsupportedUpvarRebind
capture mode / forwarding / runtime slot = 0
Planner / RegionFlow / Lower connection = 0
focused owner-forest fixtures = 17 green
resolved-region-flow-authority guard = green
all source files < 800 lines
```

### UP1 — Upvar writes and capture-plan input (closed)

```text
classify Upvar rebind separately from local BindingRebind
publish read/rebind observations for CapturePlan
do not select by-value/cell/weak mode in resolver
do not materialize runtime forwarding
```

UP1 closeout evidence:

```text
outer assignment target = UpvarRebind(UpvarRefV1)
outer compound assignment = Read + Rebind at one exact source site
local assignment target = BindingRebind(BindingRefV1)
unique Upvar relation is shared by all read/rebind observations
forest publication = ordered UpvarObservationV1[] + deduplicated UpvarRefV1[]
grandparent read/rebind source = original ancestor BindingRefV1
normalized parity includes access kind and structural owner keys
capture mode / forwarding / runtime slot = 0
Planner / RegionFlow / Lower connection = 0
focused owner-forest fixtures = 18 green
resolved-region-flow-authority guard = green
all source files < 800 lines
```

### B0 — lexical BlockExpr and compatibility isolation

Decision:

```text
C — lexical_blockexpr + inventory-gated internal CompatSequenceExpr

canonical language authority:
  A — every source BlockExpr is lexical

compatibility structure:
  a distinct non-wire carrier only when a live producer is proven
```

C does not create two source meanings. It is A plus an isolation boundary for
any compatibility sequencing that survives the producer inventory.

#### Evidence inventory

Before B0-D, the repository contained a real contract split:

```text
docs/reference/language/block-expressions-and-map-literals.md:
  Status = Draft
  Decision = provisional
  statement order and required tail are specified
  local lifetime / leakage is not specified

block-expressions-and-condition-blocks-ssot.md:
  says BlockExpr enables "condition is a scope"
  says later parser sugar must keep those semantics fixed

Rust direct MIR Lower:
  lowers each prelude statement in the current builder scope
  performs no BlockExpr-specific lexical scope push/pop

ProgramV0 JSON bridge:
  mutates the caller vars map directly
  its scope-exit path is cleanup-aware, not proof of lexical isolation

current BlockExpr fixture:
  proves prelude order, tail value, and condition use
  does not prove local leakage, shadow restoration, or outer rebind behavior
```

Therefore current execution is evidence of compatibility behavior, not
language authority. B0-D now fixes the language meaning explicitly.

#### Accepted canonical semantics

```text
scope begins:
  before the first BlockExpr prelude statement

scope contains:
  ordered prelude statements + one required tail expression

scope ends:
  immediately after the tail has been evaluated exactly once

escapes:
  tail result value
  effects on already-visible outer BindingIds

does not escape:
  bindings declared in the BlockExpr scope
  their lexical visibility
```

Every canonical BlockExpr owns both a `ScopeId` and a `RegionId`, including a
block with no local declaration. A condition-position BlockExpr ends before
then/else or loop-body entry. A future branch-visible `if-local` requires a
separate AST/scope owner and must not desugar to plain BlockExpr.

Non-local exits remain rejected recursively in the accepted v1 BlockExpr
subset. B0 does not activate expression-level Return, Break, Continue, QMark,
or Throw flow.

#### ProgramV0 correction

ProgramV0 is not a lexical-scope authority. The accepted Hako source-carrier
contract already fixes:

```text
source Assignment -> ProgramV0 Local
ProgramV0 schema widening = 0
ProgramV0 source-kind recovery = 0
```

Consequently, ProgramV0 cannot distinguish a new shadowing declaration from
an outer-binding rebind. The following mappings are forbidden:

```text
ProgramV0 BlockExpr -> canonical lexical BlockExpr
add ProgramV0 ExprV0::CompatSequenceExpr
infer lexical/sequence meaning from producer name, route, or body contents
```

The final source-sensitive boundary is:

```text
Rust authoritative parser
  -> canonical AST BlockExpr
  -> lexical BlockExpr

Hako authoritative parser
  -> parser-private HakoSourceTreeV1 BlockExpr
  -> lexical BlockExpr

ProgramV0 BlockExpr
  -> explicit legacy compatibility lane only
  -> no source semantics or Rust/Hako parity claim
```

An internal `CompatSequenceExprV0` may be introduced only if B0-P proves at
least one live non-wire producer that cannot be retired in the same bounded
slice. If added, it lives outside `ASTNode` and ProgramV0, is never emitted by
a source parser, has no `ScopeId`, and has an explicit `RegionId`. If the live
producer count is zero, the type is not created.

Unknown or saved ProgramV0 artifacts are never heuristically upgraded to
canonical lexical source. A caller requiring canonical semantics must use the
typed source path, regenerate from source, or fail fast.

#### Authority and non-authority

```text
language meaning:
  docs/reference/language/block-expressions-and-map-literals.md

source syntax/order:
  canonical Rust AST
  future parser-private HakoSourceTreeV1

lexical/control identity:
  VerifiedSemanticOwnerForestV1 ScopeId + RegionId

execution/state flow:
  future VerifiedRegionFlowV1

compatibility producer inventory:
  B0-P generated fixture
```

Non-authority:

```text
ProgramV0 tag or Local record
current direct-Lower scope mutation
current JSON bridge vars map
variable names
producer/module/function names
AST pointer identity
Lower success
legacy fixture output alone
```

#### B0 task slices

Only one row is active at a time. B0 is a BoxShape migration; it does not add
accepted source syntax or non-local exit vocabulary.

**B0-D — decision lock (closed)**

```text
reference Decision = accepted lexical_blockexpr
canonical BlockExpr always owns one lexical scope
condition BlockExpr scope ends before branch/body entry
branch-visible B3 plain-BlockExpr desugar = rejected
ProgramV0 schema widening/source recovery = 0
```

**B0-P — machine-readable producer inventory (closed)**

Materialize one generated or checked-in fixture consumed by the reusable
`resolved_region_flow_authority_guard.sh`:

```text
tools/checks/fixtures/blockexpr_producer_inventory_v1.json
```

Its closed row schema is:

```text
producer_id
producer_path
producer_status = Active | Planned | TestOnly | Dead
output_family
consumer_entry
classification
required_scope_semantics
live_caller_evidence[]
retirement_owner
```

Enumerate every active producer of BlockExpr-shaped syntax or compatibility
sequencing and classify it exhaustively as:

```text
CanonicalRustSource
CanonicalHakoTypedSourcePlanned
CompilerGeneratedCanonical
LegacyProgramV0Compatibility
InternalSequenceRequired
TestOnly
RejectedOrUnknown
```

Each row records producer path, output family, consumer entry, required scope
semantics, live caller evidence, and retirement owner. Wildcards and
name-based semantic inference are forbidden.

Acceptance:

```text
all active producers classified exactly once
ProgramV0 schema delta = 0
ProgramV0 source-kind recovery = 0
source parser CompatSequence producer count = 0
unknown production producer count = 0
InternalSequenceRequired count selects B0-C or skip mechanically
resolver/Planner/Lower behavior change = 0
```

Closeout:

```text
fixture rows = 18
active producers = 8
planned producers = 1
test-only producers = 8
dead producers = 1
InternalSequenceRequired = 0
source parser CompatSequence producers = 0
unknown production producers = 0
ProgramV0 schema/source-kind delta = 0
B0-C = skipped_by_zero_callers
selected next slice = B0-S
```

The reusable authority guard validates the closed row schema, exact evidence,
producer/status/classification uniqueness, all current Rust
`ASTNode::BlockExpr` construction sites, the unchanged ProgramV0 variant, and
the zero-result selector. Resolver, Planner, RegionFlow, and Lower behavior
remain unchanged.

**B0-S — disconnected canonical scope/region schema (closed)**

```text
ScopeKindV1::BlockExpr
RegionKindV1::BlockExpr
every represented canonical BlockExpr owns both identities as one sealed pair
scope ancestry and exact source origin verified at seal
Planner / RegionFlow / Lower connection = 0
```

##### B0-S executable task card

Classification:

```text
work kind = BoxShape
accepted resolver syntax delta = 0
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

The B0 aggregate end-state requires every canonical source BlockExpr to receive
both identities. B0-S does not claim production AST coverage yet. It adds the
disconnected vocabulary and seal law only: when a BlockExpr pair is supplied
to a canonical draft, it is complete, internally consistent, and cannot be
published as a partial scope-only or region-only truth. B0-F owns the first
canonical AST traversal and lexical behavior fixtures.

Source authority and non-authority:

```text
authority:
  records.rs ScopeKindV1 / RegionKindV1
  verifier.rs sealed pair/origin/containment contract
  source_site.rs existing BlockExprPreludeRoot / Prelude(index) / Tail

non-authority:
  shadow resolver acceptance
  ProgramV0 BlockExpr
  Planner / RegionFlow / Lower success
  names, AST pointers, producer paths, or local presence
```

Required implementation slice:

1. Add only `ScopeKindV1::BlockExpr` and `RegionKindV1::BlockExpr`; reuse the
   existing owner-branded `ScopeId`, `RegionId`, records, and normalized graph.
   No new identity, record family, or source-path segment is allowed.
2. Add one seal-time BlockExpr pair verifier. A BlockExpr scope and its owner
   region must point to each other, both kinds must be `BlockExpr`, and both
   origins must be the same `Source(BlockExprPreludeRoot)` node. The root token
   is the lexical-root anchor for the entire expression, including the tail;
   it is not evidence that the tail lives outside the scope.
3. Extend source containment for that origin to exactly
   `BlockExprPrelude(index)` and `BlockExprTail` descendants with the same
   prefix. Sibling/outer/another-BlockExpr sites are excluded. Existing generic
   scope/region ancestry remains the authority; do not create a second ancestry
   index.
4. Add `resolved_semantics/block_expr_tests.rs` instead of growing the existing
   large test modules. Cover a valid sealed pair, scope-only/region-only kind
   mismatch, different exact origins, prelude/tail containment, nested-site
   exclusion, normalized parity, and the unchanged typed resolver rejection of
   `ASTNode::BlockExpr`.
5. Extend only `resolved_region_flow_authority_guard.sh`. Reuse the B0-P
   producer inventory; do not add another JSON fixture or a B0-S-specific shell
   guard. Update its source manifest, required vocabulary/verifier anchors,
   focused test call, and stable summary fields.
6. Update `resolved_semantics/README.md` with the passive BlockExpr pair law and
   the B0-F installation boundary.

Explicitly forbidden in B0-S:

```text
ShadowScopeKindV0::BlockExpr / ShadowRegionKindV0::BlockExpr
shadow/vocabulary.rs acceptance change
shadow/expr.rs BlockExpr traversal
canonical shadow-to-kind mapping
owner_forest declaration-order changes
non-local exit scanning or activation
Planner / RegionFlow / Lower connection
ASTNode or ProgramV0 schema change
CompatSequence vocabulary or fallback
```

These are not omissions. A resolver arm without B0-F's recursive non-local
exit rejection and BlockExpr-local Lambda declaration-order proof would widen
acceptance with a partial semantic contract. B0-F must add those pieces
together before it may move BlockExpr from `ExplicitUnsupported` into resolved
expression vocabulary.

Acceptance:

```bash
cargo test -q --lib mir::resolved_semantics::block_expr_tests
cargo test -q --lib mir::resolved_semantics::resolver_tests
bash tools/checks/resolved_region_flow_authority_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

Expected stable evidence:

```text
blockexpr_scope_kind=present
blockexpr_region_kind=present
blockexpr_identity_pair=verified
blockexpr_exact_origin=verified
blockexpr_resolver_acceptance=0
blockexpr_planner_regionflow_lower_connections=0
```

B0-S closeout evidence:

```text
ScopeKindV1::BlockExpr / RegionKindV1::BlockExpr = present
reciprocal sealed pair / shared BlockExprPreludeRoot origin = verified
Prelude(index) / Tail exact containment = verified
independent normalized parity = green
canonical resolver BlockExpr acceptance = 0
Planner / RegionFlow / Lower connection = 0
focused BlockExpr schema fixtures = 6 green
resolved-region-flow-authority guard = green
all changed source files < 800 lines
```

B0-S mechanically selects B0-F.

**B0-C — optional internal compatibility carrier (skipped)**

Run only when B0-P proves a non-zero `InternalSequenceRequired` caller set.

```text
compiler-private typed carrier outside ASTNode and ProgramV0
explicit origin and RegionId
ScopeId = none
source parser producer = 0
silent fallback = 0
```

If B0-P proves zero callers, record `B0-C = skipped_by_zero_callers` and do not
add the vocabulary.

B0-P proved zero callers. No compatibility carrier, AST variant, ProgramV0
tag, parser producer, or fallback is added.

**B0-F — lexical fixture lock (closed)**

Pin tail visibility, inner-local non-leakage, same-name shadow restoration,
outer rebind propagation, initializer-before-declaration, nested BlockExpr,
condition-scope end, repeated loop-header scope, same-scope redeclaration,
and recursive non-local-exit rejection.

##### B0-F executable task card

```text
work kind = BoxShape canonical installation
resolver shape delta = exactly ASTNode::BlockExpr
new source syntax = 0
new ID / AST rewrite / fallback = 0
```

Implementation owner:

1. Add `shadow/block_expr.rs` as the single traversal box. It preflights the
   prelude and tail with the existing neutral AST observation
   `contains_non_local_exit_outside_loops()`, enters one
   `ShadowScopeKindV0::BlockExpr` / `ShadowRegionKindV0::BlockExpr` pair at the
   shared `BlockExprPreludeRoot`, resolves `BlockExprPrelude(index)` in source
   order and `BlockExprTail` exactly once, then leaves the pair on success or
   error. Every nested BlockExpr invokes the same entry and therefore resets
   the non-local-exit boundary.
2. Add one typed `BlockExprNonLocalExit { site: ResolvedExitSiteV1 }` error.
   Prelude failure uses the containing Statement site; tail failure uses its
   Expression site. Return/Throw always fail the neutral observation;
   Break/Continue pass only inside a loop nested within that BlockExpr.
   Lambda/function/box owner boundaries remain opaque to the observation.
3. Move BlockExpr from `ExplicitUnsupported` to
   `CurrentResolvedExpression`, add its explicit expression arm, shadow kinds,
   and exhaustive shadow-to-canonical kind mappings. No wildcard or retry is
   allowed.
4. Reuse the existing scope stack and declaration algorithm. Initializers are
   resolved before declarations; the tail sees completed prelude declarations;
   leaving restores outer shadowing; an assignment to an already-visible outer
   name retains that outer BindingRef.
5. Extend only `owner_forest.rs::direct_member_index`: prelude members use
   their source index and the tail uses `u32::MAX`. Existing `record_lambda`,
   parent-scope snapshot, and `visible_bindings_for_child` remain unchanged.
   This proves a prelude Lambda sees earlier declarations but never later ones,
   while a tail Lambda sees all completed prelude declarations.
6. Extend the existing `block_expr_tests.rs`; do not grow the 748-line
   `owner_forest_tests.rs`. Cover empty pair creation, tail visibility,
   non-leakage, same-name restoration, outer rebind, initializer ordering,
   nested BlockExpr, condition/loop scope end, redeclaration, recursive exits,
   nested-loop Break/Continue acceptance, and prelude/tail Lambda declaration
   order. Keep the test file below 800 lines.
7. Extend the existing authority guard without creating a new shell or JSON
   fixture. Preserve the B0-P inventory selector as landed history and add
   stable B0-F acceptance/ordering evidence.

Forbidden:

```text
Planner / RegionFlow / Lower connection
ProgramV0 or ASTNode schema change
CompatSequence carrier
name-based scope or binding reconstruction
independent non-local-exit semantics in the resolver
owner-forest visible-binding rescan
expression-level Return/QMark/Throw activation
```

Acceptance:

```bash
cargo test -q --lib mir::resolved_semantics::block_expr_tests
cargo test -q --lib mir::resolved_semantics::owner_forest_tests
bash tools/checks/resolved_region_flow_authority_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

B0-F closes only when all lexical fixtures and normalized forest parity are
green with Planner/RegionFlow/Lower connections still zero. Closeout selects
B0-L; it does not claim runtime scope cutover.

B0-F closeout evidence:

```text
dedicated shadow/block_expr.rs traversal owner = installed
tail visibility / nonleak / shadow restore / outer rebind = green
condition If/Loop scope end / nested BlockExpr = green
recursive non-local exit rejection / nested-loop exits = green
prelude and tail Lambda declaration order / normalized forest parity = green
canonical resolver BlockExpr acceptance = 1
Planner / RegionFlow / Lower connection = 0
focused BlockExpr fixtures = 16 green
resolved-region-flow-authority guard = green
all changed source files < 800 lines
```

**B0-L0 — canonical Lower ingress/site-carrier design stop (closed)**

Worker inventory proves that B0-L cannot begin as a local BlockExpr edit:

```text
canonical resolver / owner forest production install = 0
Lower SourceStmtSiteV1 / SourceExprSiteV1 carrier = 0
resolved declaration production callers = 0
direct Rust BlockExpr Lower = unscoped ASTNode arm
```

The existing `ResolvedBindingLoweringStateV1` is a disconnected declaration
transport seam. Installing a product while BlockExpr locals still use the
legacy BindingId allocator would create two identities for the same
declaration. Adding only `LexicalScopeGuard` would improve name restoration
but would not enter the resolved ScopeId/RegionId pair and therefore cannot be
claimed as B0-L.

##### B0-L0 executable consultation card

Inventory and decide, without changing Lower behavior:

1. Enumerate every canonical function ingress before params/body are split:
   free/static function, constructor, instance/static method, inline/callable
   Main, script main, REPL, Lambda child, and CorePlan-produced body.
2. Select one typed route boundary before Lower. Candidate canonical input is
   `ResolvedFunctionLoweringInputV1 { syntax, sealed owner product/forest }`;
   legacy and ProgramV0 inputs remain distinct typed non-authority routes.
3. Select the owner of a borrowed structural source cursor from `Body(index)`
   through `BlockExprPrelude(index)` / `BlockExprTail` and every nested child.
   Decide whether exact-site threading can be an independent slice or must be
   the same atomic SA3-B declaration/BindingId cutover.
4. Define one exact product query from source site to the sealed BlockExpr
   ScopeId/RegionId pair and one error-safe resolved scope guard. Function
   lowering must restore resolved state, region/scope stacks, function
   context, and body AST on both success and failure.
5. Decide the all-or-nothing declaration boundary. Receiver, parameters,
   locals, outbox, nested owner products, use sites, and assignment targets
   may not mix resolved and legacy BindingId allocation under one installed
   product. Name-keyed Planner helpers remain B0-R retirement work.

Authority and non-authority:

```text
authority = VerifiedResolvedFunctionV1 / VerifiedSemanticOwnerForestV1
identity lookup = exact SourceStmtSiteV1 / SourceExprSiteV1 only
non-authority = AST pointer, Span, name, traversal order, producer path, ProgramV0
route failure after selection = fail-fast; legacy retry = forbidden
Planner / RegionFlow / Recipe connection = forbidden in B0-L0
```

Acceptance:

```text
all canonical function ingress families classified
typed route owner and exact source-cursor owner selected
SA3-B dependency/order decided explicitly
success/error cleanup owner and fail-fast sites fixed
canonical Lower route remains 0 during consultation
resolved ScopeId consumer remains 0 during consultation
heuristic scope lookup / fallback / retry remains 0
```

Decision A′ is accepted:

```text
B0-L1  ingress inventory
B0-L2a typed source-unit ingress (activation zero)
B0-L2b immutable FunctionSourceView / LocatedNode navigator (activation zero)
B0-L2c closure-scoped function transaction (behavior preserving)
SA3-B   first closed owner-family atomic identity activation
B0-L3a straight-line BlockExpr resolved pair consumption
B0-L3b/B0-L4/B0-L5 located control flow, CorePlan, Lambda child transport
```

`FunctionSourceViewV1` and located node values replace a mutable source cursor.
Family activation is staged, but one source unit cannot mix canonical and
legacy owners. `CanonicalFunctionLoweringSessionV1` owns explicit fallible
cleanup. Carrier infrastructure may land disconnected; production activation
must atomically include sealed identity, exact sites, legacy-allocation veto,
coverage, cleanup, and publication.

**B0-L1 — ingress inventory (closed)**

The machine-readable inventory is:

```text
tools/checks/fixtures/resolved_lowering_ingress_inventory_v1.json
module ingress rows = 5
function family rows = 10
raw body-route seam rows = 2
production semantic activation = 0
canonical Lower route = 0
exact source-site transport = 0
resolved ScopeId consumer = 0
```

It classifies free/static functions, app/script static methods, instance
constructors/methods, script root, optional callable and inline Main, Lambda,
REPL, ProgramV0 compatibility ingress, the raw suffix router, and the synthetic
Program body wrapper. Every row carries live source evidence and a staged
capability order. The existing Resolved Region Flow family guard invokes the
inventory validator and mechanically selects B0-L2a.

**B0-L2a — typed source-unit ingress (closed)**

Add only disconnected transport vocabulary above `MirBuilder`:

```text
VerifiedResolvedSourceUnitV1
ResolvedModuleLoweringInputV1<'a>
LegacyModuleLoweringInputV1
MirLoweringRequestV1 (match once; never recurse)
CanonicalLoweringErrorV1 capability/preflight vocabulary
```

Contract:

1. `VerifiedResolvedSourceUnitV1` bundles the one immutable canonical syntax
   owner and its sealed owner forest; it is not a rewritten/resolved AST.
2. A resolved module input can only be derived from that verified bundle.
   Bare AST, ProgramV0, and REPL compatibility remain explicit legacy inputs.
3. Route selection belongs in `MirCompiler`, before `MirBuilder::prepare_module`.
   B0-L2a adds the types and one-match boundary only; it adds no production
   canonical caller and never invokes semantic resolution or Lower.
4. The route enum is consumed immediately by distinct resolved/legacy methods.
   It must not become an `Option`, boolean mode, or recursive Lower parameter.
5. Unsupported canonical capability is typed and would fail before module,
   entry block, FunctionRegion, resolver session, or Lower effects. There is no
   legacy retry.
6. B0-L2a does not yet define child navigation. That belongs exclusively to
   B0-L2b `FunctionSourceViewV1` / located node vocabulary.

Acceptance:

```text
types compile and remain constructor-bounded
resolved input cannot be built from bare AST
legacy input cannot carry a sealed forest
request dispatch has one match site above MirBuilder
production resolved request constructors/callers = 0
prepare_module remains reachable only through the legacy implementation
semantic product install / exact source-site transport / ScopeId consumer = 0
Planner / RegionFlow / Recipe connection = 0
all source files < 800 lines
```

Closeout evidence:

```text
typed compiler ingress = present
request match sites above Builder = 1
production VerifiedResolvedSourceUnitV1 constructors = 0
production compile_resolved callers = 0
canonical capability stop before Builder effects = verified
bare AST / ProgramV0 / REPL legacy provenance = explicit
semantic product install / exact source navigation / ScopeId consumer = 0
Planner / RegionFlow / Recipe connection = 0
focused compiler ingress tests = 3 green
```

The boundary lives in `src/mir/compiler/lowering_input.rs`; its ownership and
forbidden edges are fixed by `src/mir/compiler/README.md`. The existing legacy
Builder implementation is unchanged behind the legacy request branch.

**B0-L2b — immutable exact source navigator (closed)**

Add disconnected source projection and located-node vocabulary without
activating canonical Lower:

```text
VerifiedSourceProjectionV1
FunctionSourceViewV1<'a>
LocatedBodyV1<'a>
LocatedStmtV1<'a>
LocatedExprV1<'a>
LocatedBodySuffixV1<'a>
SourceNavigationErrorV1
closed ExprChildRoleV1 / BodyChildRoleV1
```

Contract:

1. `VerifiedResolvedSourceUnitV1` owns canonical syntax, its sealed owner
   forest, and the source projection verified for that same syntax. A view can
   only be derived for an owner/product/source-root triple in that unit.
2. Navigation is immutable and parent-relative. Recursive callers carry a
   borrowed AST node and its exact typed site together; there is no Builder
   cursor to save or restore.
3. The existing `SourcePathSegmentV1` grammar remains the path SSOT. Lower must
   not reproduce path construction. Closed child-role APIs delegate to the
   verified projection and reject wrong node family, missing child, foreign
   owner, and site/node mismatch.
4. AST pointer, Span, name, encounter order, reconstructed ProgramV0, and AST
   clone are forbidden identity sources. Equal Span values must not alias two
   distinct sites.
5. Body suffix transport is typed as `(LocatedBodyV1, start_index)` only. The
   raw Planner seam is not changed and no `consumed: usize` route is connected
   in this slice.
6. The verified source-unit production constructor, `compile_resolved` caller,
   semantic-product install, recursive Lower consumer, and Planner connection
   all remain zero. Focused tests may use the bounded test factory.

Acceptance:

```text
FunctionBody / body statement navigation is exact
Local initializer and assignment target/value navigation is exact
BlockExpr prelude/tail and nested BlockExpr navigation is exact
Lambda definition-site navigation preserves owner boundary
wrong family / missing child / foreign owner / site-node mismatch reject
same-Span sibling nodes remain distinct
mutable source cursor / pointer lookup / Span lookup / name lookup = 0
production verified-unit constructors / compile_resolved callers = 0
Builder / Planner / RegionFlow / Recipe connections = 0
all source files < 800 lines
```

Closeout evidence:

```text
shared source path builder = SourcePathV1
shadow resolver duplicate path builder = 0 (local alias only)
verified syntax/forest projection = present
projection stored syntax pointers / Span / names / AST clones = 0
located carrier safe-code factories outside FunctionSourceViewV1 = 0
exact FunctionBody/Local/Assignment/nested BlockExpr navigation = verified
Lambda child owner transition / foreign-owner rejection = verified
same-Span sibling site distinction = verified
syntax/product signature mismatch rejection = verified
focused source-navigation fixtures = 4 green
production verified-unit constructors / compile_resolved callers = 0
Builder / Planner / RegionFlow / Recipe consumers = 0
resolved_region_flow_authority_guard = green
selected next slice = B0-L2c
```

The source unit owns `VerifiedSourceProjectionV1` beside syntax and the sealed
forest. The projection stores only owner definition chains. Located carriers
are sealed by an unforgeable `SourceViewSealV1`; safe code outside
`FunctionSourceViewV1` cannot assemble an arbitrary site/node pair. Physical
AST-field navigation remains centralized in `source_projection.rs`, while the
resolver and compiler share the one `SourcePathV1` builder.

**B0-L2c — closure-scoped function transaction (closed)**

This is a behavior-preserving BoxShape slice over the existing static/instance
function Lower. It activates neither the resolved source view nor semantic
identity. The cleanup owner is `CanonicalFunctionLoweringSessionV1`; the name
reserves the future canonical boundary, while this slice proves the lifecycle
using the current legacy body path.

Required structure:

1. Add one `calls/function_session.rs` lifecycle box. Both
   `lower_static_method_as_function` and `lower_method_as_function` enter it;
   neither may call prepare/restore, FunctionRegion pop, or `fn_body_ast`
   clear manually.
2. The session owns the complete caller snapshot before any fallible skeleton,
   parameter, body, or finalize step. Include current function/block, binding
   and resolved-binding state, variable/type context, lexical/loop/if/debug/
   FastMem stacks, SSA caches, try/cleanup flags, recursion/re-entry guards,
   slot registry, reserved values, `fn_body_ast`, FragEmitSession, current
   source Span, and observer Region stack. Existing `BoxCompilationContext`
   clear-only mode remains an explicit isolation policy, not caller state.
3. Finalization returns an unpublished `MirFunction` draft. The session first
   verifies/restores caller state, then commits that draft to the module.
   Any primary or cleanup error publishes no function.
4. Cleanup is explicit and returns `Result`; Drop is only a debug assertion
   and panic-restoration backstop. If primary and cleanup both fail, preserve
   both diagnostics under one stable contract error.
5. Restore the exact observer Region stack snapshot rather than assuming one
   unconditional pop. Success and every `?` error path use the same close
   operation.

Acceptance:

```text
manual prepare/restore pairs in lowering.rs = 0
manual FunctionRegion pop in lowering.rs = 0
manual fn_body_ast set/clear in lowering.rs = 0
static and instance method lifecycle owner = one session
unpublished draft before cleanup = verified
skeleton / parameter / body / finalize injected errors publish functions = 0
success and error restore caller function/block and all snapshotted state
primary + cleanup error preserves both diagnostics
resolved source-view Builder consumers = 0
production semantic activation / BindingId authority change = 0
Planner / RegionFlow / Recipe connection = 0
focused transaction tests and existing representative function gates = green
all source files < 800 lines
```

Stop if closure ownership requires `Option<VerifiedResolvedFunctionV1>`, a
mutable source cursor, a new environment toggle, or any canonical/legacy
identity mixture. Those belong to atomic SA3-B, not this lifecycle slice.

Closeout evidence:

```text
cleanup owner = CanonicalFunctionLoweringSessionV1
static session entries = 1
instance session entries = 1
manual prepare/restore pairs in lowering.rs = 0
manual FunctionRegion pops in lowering.rs = 0
manual fn_body_ast mutations in lowering.rs = 0
finalize result = unpublished MirFunction draft
module commit order = caller restore/validation then add_function
injected checkpoints = before skeleton / after skeleton / after params /
                       after body / after finalize
primary + cleanup diagnostic preservation = verified
panic Drop restoration = verified
focused transaction fixtures = 4 green
builder calls tests = 23 green
resolved_region_flow_authority_guard = green
dev_gate quick = green
resolved source-view Builder consumers / semantic activation = 0
selected next slice = atomic SA3-B first family
```

The complete caller snapshot now includes the previously unowned FastMem stack,
reserved ValueIds, `fn_body_ast`, FragEmitSession, current Span, observer Region
stack, recursion/re-entry guards, and the existing binding/scope/SSA/cleanup
state. Error paths discard the partial function; success also remains
unpublished until cleanup is verified.

**SA3-B — first closed canonical BindingId authority family (closed)**

This is one atomic authority cutover, not a partial transport experiment. The
first capability is exactly one non-main static/free function source unit with
one owner and a straight-line body. Lambda, If, Loop, CorePlan, BlockExpr,
Try/Catch/Cleanup, Match, QMark, Main inline/callable, instance receiver, REPL,
and ProgramV0 are unsupported before any Builder/module effect.

Required structure:

1. Add `ResolvedFunctionLoweringInputV1` derived only from
   `VerifiedResolvedSourceUnitV1`. It always carries one owner,
   `FunctionSourceViewV1`, the matching `VerifiedResolvedFunctionV1`, and the
   forest; syntax/product/definition origin cannot be supplied independently.
2. Add a whole-unit `CanonicalLoweringPreflightV1` before module preparation.
   It accepts only the closed first-family grammar and `owner_count = 1`.
   Unsupported syntax/owner/control returns typed canonical error and never
   retries legacy.
3. Add a distinct `CanonicalFunctionLowererV1`. Recursive canonical lowering
   accepts only `LocatedBodyV1` / `LocatedStmtV1` / `LocatedExprV1`; it may
   share emission primitives but may not call legacy statement/expression
   dispatch for declarations, variables, or assignment targets.
4. Install the sealed product at function-session entry. Receiver/parameter/
   local/outbox declaration claims, variable uses, and assignment targets use
   exact source sites and the product's `BindingRefV1`. The canonical value
   environment is keyed by `BindingRefV1`, not names.
5. Veto every Lower-side `allocate_binding_id()` while the resolved product is
   installed. Parameter, local, use, and assignment cut over together for the
   owner; no owner-local mixed mode is allowed.
6. Split completion into identity adoption and source coverage. Every source
   declaration receives one disposition; materialized declarations publish
   exactly once. Coverage and session stack checks finish before the draft is
   committed.

First-family acceptance:

```text
production verified-unit constructor / compile_resolved path = 1 closed route
whole-unit capability preflight occurs before module/entry/function effects
canonical failure -> legacy retry = 0
source-unit canonical/legacy owner mixture = 0
exact parameter/local declaration claims = verified
exact variable-use and assignment-target lookup = verified
legacy BindingId allocation while resolved product installed = 0
name lookup as canonical identity = 0
all materialized bindings originate in the sealed product
duplicate/foreign/unclaimed identity rejects before function commit
partial function publication on every error = 0
BlockExpr / If / Loop / CorePlan / Lambda runtime claims = 0
ProgramV0 / REPL / Main / instance-method canonical claims = 0
Planner / RegionFlow connection = 0
fixture + fast gate + existing legacy route regression = green
all source files < 800 lines
```

Stop immediately if the implementation introduces an optional resolved
product/site, calls the legacy allocator after install, recovers a site by
name/Span/pointer/order, mixes canonical and legacy owners in one source unit,
or discovers unsupported capability after Builder effects. B0-L3 starts only
after this first family is atomically green.

Closeout evidence:

```text
VerifiedResolvedSourceUnitV1::resolve_function production constructor = 1
CanonicalLoweringPreflightV1 before candidate Builder = verified
CanonicalModuleLoweringSessionV1 candidate commit/discard = verified
ResolvedFunctionLoweringInputV1 independent assembly seams = 0
CanonicalFunctionLowererV1 legacy recursive dispatch calls = 0
canonical value environment key = BindingRefV1
parameter/local/outbox exact adoption = verified
variable-use/assignment-target exact coverage = verified
legacy BindingId allocation while product installed = rejected
identity/source coverage finish before function draft commit = verified
partial function publication on injected error = 0
focused SA3-B tests = 6 green
resolved_region_flow_authority_guard = green
Planner / RegionFlow / BlockExpr / If / Loop / Lambda connections = 0
```

The source unit constructor resolves and projects the same owned syntax; a
foreign syntax/product pair cannot be supplied. A fresh candidate Builder is
the module transaction, so preflight failure and every later error leave the
caller's Builder untouched. The function session installs the sealed product,
the lowerer finishes identity/source coverage, and only then returns an
unpublished draft for cleanup verification and commit.

**B0-L3a — straight-line canonical BlockExpr Lower (closed)**

The next slice widens only the already-closed canonical expression grammar
with BlockExpr. It consumes the resolver-sealed ScopeId/RegionId pair through
one `ResolvedScopeSessionV1`, tracks declarations by BindingRef, removes only
inner declarations on scope exit, preserves outer rebinds, and returns the
tail ValueId after balanced leave.

Classification and implementation owners:

```text
work kind = BoxShape canonical-consumer activation
accepted source syntax delta = 0
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1

exact pair lookup:
  VerifiedResolvedFunctionV1

scope transaction:
  resolved_lowering/scope.rs ResolvedScopeSessionV1

value lifetime disposition:
  resolved_lowering/identity.rs

source navigation:
  existing FunctionSourceViewV1 located carriers only
```

Required structure:

1. Preflight walks located bodies and expressions rather than rebuilding path
   grammar. It admits only Local, Outbox, binding Assignment, the existing
   literal/variable/non-short-circuit binary expressions, and nested
   BlockExpr. If, Loop, Call, Lambda, CorePlan, and non-local exits remain
   unsupported before Builder effects.
2. Add one typed exact-site query from a located BlockExpr expression to its
   sealed ScopeId/RegionId pair. It verifies owner, BlockExpr kinds, reciprocal
   links, and the shared `site + BlockExprPreludeRoot` origin. Span, name,
   pointer, encounter-order, and generic containment recovery are forbidden.
3. Add `resolved_lowering/scope.rs`. `ResolvedScopeSessionV1` consumes each
   pair once, enforces nested LIFO parentage, and closes explicitly on success
   and error. Cleanup errors preserve the primary Lower error.
4. Split the BindingRef value environment into active and retired
   dispositions. Scope leave retires only the declarations listed by the
   sealed scope record. It never snapshots or restores the whole environment;
   therefore an outer rebind remains published while an inner shadow is
   removed.
5. Lower BlockExpr through `BlockExprPrelude` and `BlockExprTail` located
   carriers. Prelude statements run in order, the tail is lowered exactly
   once, and its ValueId is returned after the pair is closed.
6. Keep the reusable authority guard below 800 lines. Put B0-L3a static
   anchors in one small library helper instead of growing the already-large
   top-level guard.

```text
BlockExpr prelude: Local / Outbox / Assignment / closed expressions only
tail: existing first-family expression grammar plus nested BlockExpr
exact sealed ScopeId + RegionId pair consumed = 1 per BlockExpr
tail lower count = 1
inner binding leak = 0
same-name shadow restoration = resolver BindingRef identity
outer rebind survives scope leave
error path balances resolved scope/region session
If / Loop / CorePlan / Lambda / call runtime widening = 0
```

Focused fixtures:

```text
empty prelude and tail value
initializer before declaration publication
inner local visible to tail and retired at leave
same-name shadow restores the outer BindingRef
outer rebind survives leave
nested BlockExpr consumes independent exact pairs
tail lowered exactly once and ValueId survives leave
wrong/missing pair rejects
error path balances pair and preserves the primary error
verified MIR/runtime result
```

Acceptance:

```bash
cargo test -q --lib mir::builder::resolved_lowering
cargo test -q --lib mir::resolved_semantics::block_expr_tests
bash tools/checks/resolved_region_flow_authority_guard.sh
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
```

Stop if this requires the legacy `LexicalScopeGuard`, name-keyed state,
optional resolved authority/site, mutable source cursor, raw AST recursion in
the new BlockExpr path, or any If/Loop/CorePlan/Lambda/Call activation.

Closeout evidence:

```text
exact product query = block_expr_scope_region_pair
scope owner = ResolvedScopeSessionV1
value disposition = active or retired BindingRef
located prelude/tail transport = 1/1
tail constant lower count = exactly once
shadow restoration and outer rebind VM result = 5
nested tail-after-leave VM result = 9
wrong/foreign pair rejection = verified
error close balance and pair reconsumption veto = verified
focused default resolved_lowering tests = 9 green
focused VM-reference BlockExpr tests = 5 green
resolved_region_flow_authority_guard = green
canonical BlockExpr Lower connection = 1
Planner / RegionFlow / If / Loop / Lambda connections = 0
all guarded Rust source files < 800 lines
selected next slice = B0-L3b canonical If branch-flow design decision
```

B0-L3b inventory found that exact If source carriers and sealed passive
region/scope kinds exist, but the branch-effect/PHI request owner is not fixed.
The accepted architecture assigns per-port binding effects to RegionFlow and
ValueId/BasicBlockId materialization to Lower, while the tempting minimal
implementation would discover changes by diffing Lower's mutable value
environment. Code is stopped until this boundary is decided in:

```text
docs/development/current/main/investigations/
  mirbuilder-b0-l3b-located-if-branch-flow-consultation-2026-07-13.md
```

Canonical If production activation remains zero. The legacy IfForm and name-keyed PHI
paths are not canonical implementation candidates.

**B0-L — explicit Rust canonical Lower cutover (ordered after B0-L2 and SA3-B)**

```text
enter resolved BlockExpr ScopeId
lower prelude in source order
lower tail exactly once
retain tail ValueId
leave scope
publish verified outer rebind effects
```

Legacy and canonical routes are selected before Lower by distinct typed
inputs. Failure after route selection is a contract violation; there is no
retry to another route.

**B0-H — Hako typed-source parity (source-carrier dependency)**

After HakoSourceTreeV1 preserves Local/Assignment and BlockExpr scope, compare
normalized BlockExpr scope/region/origin graphs from independent Rust and Hako
parsers. ProgramV0 is not an input or oracle for this parity.

**B0-R — retirement**

Retire direct unscoped Rust lowering, name-keyed planner BlockExpr state, and
legacy ProgramV0 bridge behavior only after their exact caller and parity
gates close. A compatibility carrier is deleted at zero callers; it is never
promoted to permanent IR without a new decision.

#### B0 stop conditions

```text
same BlockExpr variant changes semantics by context/producer/route
source parser emits CompatSequenceExpr
canonical BlockExpr lacks ScopeId or RegionId
scope existence depends on whether a local is present
Lower infers scope, binding identity, or outer rebind from names
condition-local visibility leaks into then/else or loop body
ProgramV0 is used to recover Local versus Assignment
ProgramV0 v0 schema gains a compatibility expression tag
untagged artifacts are classified heuristically
lexical Lower failure retries legacy sequence Lower
child-local BindingId escapes through RegionFlow state
outer rebind is lost at BlockExpr scope exit
tail ValueId lifetime is confused with child binding visibility
scope-sensitive fixtures are absent at cutover
non-local exit support is mixed into B0
```

### M0 — Match structural identity

```text
Match/arm RegionId
optional arm ScopeId
PatternBinder BindingId
no dispatch/result merge/ValueId
```

### T0 — catch/cleanup structural identity

```text
Try/Catch/Cleanup RegionId and ScopeId
CatchBinder BindingId
no source Throw activation
no catch selection or cleanup continuation
no MIR connection
```

### F0 — RegionFlow and production cutover

After P0 through the required structural rows are green:

```text
build VerifiedRegionFlowV1
build VerifiedCapturePlanV1 where needed
connect one disconnected differential Lower route
prove behavior parity
perform atomic production semantic-authority cutover
retire name/depth/free-variable rediscovery only after caller-zero gates
```

## Required fixtures

Owner forest:

```text
one non-capturing Lambda
two sibling Lambdas with owner-local BindingId(0)
nested Lambda parent chain
duplicate definition site
missing child product
multiple parents
parent cycle
foreign parent_scope
partial forest publication
```

Upvar:

```text
outer parameter/local/receiver read
nearest shadow wins
child local prevents Upvar
multiple uses -> one structural relation
grandparent direct source
sibling/descendant source rejected
self/recursive capture rejected
global/static symbol is not Upvar
Upvar rebind is not local BindingRebind
```

Exit:

```text
statement/expression sites cannot alias
one record owns source region + origin + transfer
nested break/continue exact nearest loop
return exact current function
QMark in child, if compatibility-retained, targets child function
Lower depth recount = 0
```

Block/Match/Cleanup:

```text
BlockExpr tail sees inner local
BlockExpr inner local does not escape
BlockExpr shadow restores outer binding
BlockExpr outer rebind propagates
condition BlockExpr scope ends before branch/body entry
separate match-arm binders are distinct
arm/catch binder does not escape
Cleanup is not a new function owner
resolver allocates no result ValueId or cleanup block
```

Rust/Hako parity compares normalized owner origins, source sites, declaration
origins, Upvar source edges, region origins, and exit records. It never
compares raw owner/binding/scope/region numbers or iteration order.

## Implementation may claim

After P0:

```text
remaining nested/control syntax has exact, non-overloaded structural path
vocabulary
cross-owner expression sites can be owner-branded
resolver acceptance is unchanged
```

After OF0:

```text
nested function syntax owns a distinct sealed semantic owner
child declarations never enter the parent binding arena
parent/child identity is explicit and acyclic
```

After UP0:

```text
free-variable identity is structural and name-independent
Upvar source resolves to one strict-ancestor declaration
capture runtime policy remains unselected
```

After B0-D:

```text
canonical source BlockExpr has one accepted lexical meaning
condition BlockExpr bindings do not escape into branch/body regions
ProgramV0 is not BlockExpr source-scope authority
compatibility sequencing requires a distinct non-wire typed carrier if any
live producer remains
```

## Implementation must not claim

```text
closure lowering complete
capture mode or transport complete
recursive closure support
QMark canonical language activation
source Throw activation
Match result lowering
catch selection / cleanup flow
BlockExpr production scope cutover before B0-F/B0-L gates
full AST resolver support
production semantic-authority cutover
legacy loop_var retirement
raw IDs stable across source edits or Rust/Hako implementations
```

## Retirement path

After verified replacement and caller-zero gates:

```text
exprs_lambda.rs name-based collect_vars
vars/free_vars.rs name resolver authority
name-keyed closure capture lookup
closure body AST clone ownership
parallel control_exits / control_exit_regions
QMark direct CFG authority
Match direct merge/PHI authority
Try/Cleanup mutable control-policy flags
Lower-side break/continue depth resolution
mandatory loop_var / loop_increment / BodyManagedCursor
```

Traversal fixtures may be moved before deleting old walkers. Runtime emit
helpers may remain after their semantic decision authority is removed.

## Stop conditions

```text
1. Lambda declarations enter the parent owner arena.
2. A parent BindingId is reused raw inside a child owner.
3. Resolver allocates CaptureId or CaptureSlotId.
4. Resolver creates a child synthetic capture BindingId.
5. Upvar source is not a strict lexical ancestor.
6. Grandparent Upvar is rewritten into semantic forwarding bindings.
7. Capture mode/layout is selected in the resolver.
8. A cross-owner index uses bare SourceExprSiteV1.
9. roots/child_at/upvar inventory becomes mutable parallel authority.
10. A standalone owner product with unchecked foreign refs reaches Planner.
11. QMark/Throw receives a fabricated statement path.
12. QMark becomes canonical syntax through resolver implementation alone.
13. Source Throw is accepted despite the language prohibition.
14. BlockExpr scope changes without a language decision and differential gate.
15. Resolver owns Match result merge or Cleanup continuation.
16. Resolver or RegionFlow allocates ValueId/BasicBlockId.
17. Lower rediscovers names, free variables, loop depth, or cleanup targets.
18. Unsupported syntax retries a legacy resolver.
19. Partial owner/forest products publish.
20. Rust/Hako parity compares raw IDs or map iteration order.
21. Any source file reaches 800 lines.
```

## Closeout condition

This card leaves the design-stop state only through P0 code. The next commit
must be source-role vocabulary, executable tests, or an explicit source
contradiction that reopens this decision. Another docs-only refinement is not
an accepted next action.
