---
Status: P0/E0/OF0/UP0/UP1/B0-D/B0-P/B0-S closed; B0-C skipped; B0-F is active
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
fixture rows = 17
active producers = 8
planned producers = 1
test-only producers = 7
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

**B0-F — lexical fixture lock (active)**

Pin tail visibility, inner-local non-leakage, same-name shadow restoration,
outer rebind propagation, initializer-before-declaration, nested BlockExpr,
condition-scope end, repeated loop-header scope, same-scope redeclaration,
and recursive non-local-exit rejection.

**B0-L — explicit Rust canonical Lower cutover**

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
