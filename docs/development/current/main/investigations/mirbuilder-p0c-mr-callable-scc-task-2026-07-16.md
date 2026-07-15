---
Status: P0c-MR-R0-P0 closed; atomic CUT0 next
Date: 2026-07-16
Decision: C-prime accepted with bounded implementation refinements
Current row: P0c-MR-R0-CUT0 atomic Program-authority cutover
Production baseline: 4bed234ceb
Consultation: mirbuilder-p0c-mr-scc-consultation-question-2026-07-16.md
R0 consultation: mirbuilder-p0c-mr-r0-self-call-retirement-consultation-question-2026-07-16.md
---

# P0c-MR Callable SCC Task

## Decision lock

The durable mutual-recursion architecture is:

```text
P0c-MR-G0  shared callable graph inventory extraction
P0c-MR-S0  disconnected deterministic SCC partition
P0c-MR-V0  disconnected recursive exact-i64 module plan
P0c-MR-C0  passive recursive-module backend capability
P0c-MR-I1  explicit atomic VM-only activation
P0c-MR-R0  later one-function self-call authority retirement
```

G0 through I1 and the runtime frame-restoration proof tail are landed. R0 has
selected Program-only authority with `S0 -> P0 -> CUT0 -> G0`. S0 and P0 are
closed. CUT0 is the sole next code-facing row and must atomically activate the
singleton Program route while deleting the old authority.

Candidate A's two-function even/odd shape is a fixture, not a semantic
authority. Candidate B's general finite SCC module is the activation target,
but it begins only after G0 extracts topology inventory once. This is C-prime:
shared inventory first, then the general SCC proof and activation.

## Verification result

The answer matches the current implementation boundaries:

```text
catalog and call targets:
  already resolved pre-Builder by canonical identity

per-call ABI/effect:
  already co-sealed in VerifiedTrivialDirectCallV1

current acyclic graph:
  inventory extraction and DAG proof are currently one responsibility

publication:
  already unpublished-draft collection plus atomic batch insertion

runtime:
  frame transactions, MAX_CALL_DEPTH, and final-callee contracts already exist
```

No new name resolver, call ABI owner, effect fixed point, publication
transaction, or runtime graph discovery is required.

## Bounded refinements

### Error ownership follows product ownership

`CallableGraphInventoryErrorV1` owns node-set, caller/root, target projection,
target membership, and duplicate-site failures. `AcyclicCallableGraphErrorV1`
wraps inventory failure and continues to own self-edge, Kahn cardinality, and
cycle failures. SCC construction owns partition/condensation failures only.

### Sealed graph products are non-Clone

Rows and IDs may be cloneable for diagnostics, but
`VerifiedCallableGraphInventoryV1` and
`VerifiedCallableSccPartitionV1` must not implement `Clone`. Each selected
proof path consumes one inventory by value and retains it. "Shared inventory"
means one schema and extraction implementation per selected route, not one
physical value copied or shared by `Arc` between two proofs.

### SCC traversal is host-stack safe

The public SCC contract is algorithm-independent. Deterministic Kosaraju is a
valid first implementation, but traversal must use an explicit work stack or
another bounded iterative implementation. The accepted grammar has no
arbitrary function-count limit, so recursive host-language DFS is not an
acceptable fail-fast boundary.

### Recursive capability publication is atomic

The recursive marker is one private module-level capability with an exact
schema version. Publication prechecks both the function batch and marker
absence/schema before candidate mutation. Function insertion and marker
installation then use one non-fallible candidate mutation boundary, preferably
a typed `publish_recursive_into` facade. The outer candidate stays isolated
until canonical finish and session commit.

### Serialized transport remains inactive

The first activation is in-memory Rust MIR interpreter only. MIR JSON and other
serialized transports remain inactive. A later transport row must carry the
recursive marker exactly or fail before transport/backend effects; silently
losing the marker is forbidden.

## Frozen first grammar

```text
source root:
  one exact function-only ASTNode::Program

function count:
  at least 2

signature:
  static, non-main, non-override, metadata empty
  one-or-more exact i64 parameters, exact i64 result

direct calls:
  one-or-more total sites
  repeated, nested, argument-position, and multi-target calls allowed
  calls in both fallthrough If arms allowed

topology:
  at least one recursive SCC
  singleton self SCC allowed
  arbitrary finite mutual SCC size allowed
  multiple recursive SCCs and mixed DAG plus SCC allowed

still rejected:
  one-function Program before R0
  early Return or Return inside If arm
  Loop, MethodCall/receiver, Lambda/capture
  non-i64 ABI, ownership operations, imports, plugins, FFI

backend:
  Rust MIR interpreter only
```

All calls remain `ConservativeBarrier`. P0c-MR claims neither termination nor
effect precision. `MAX_CALL_DEPTH` is a runtime resource boundary only.

## Authority split

| Concern | Authority | Non-authority |
| --- | --- | --- |
| source/catalog membership | `VerifiedCallableCatalogSourceUnitV1` | SCC product, MIR table |
| call-site target | `VerifiedResolvedFunctionV1` | raw source name in Lower |
| topology inventory | `VerifiedCallableGraphInventoryV1` | ABI, effect, evaluation order |
| acyclic proof | `VerifiedAcyclicCallableGraphV1` | SCC identity, call materialization |
| SCC proof | `VerifiedCallableSccPartitionV1` | target resolution, ABI, MIR |
| per-call ABI/effect | `VerifiedTrivialDirectCallV1` | graph/SCC rows |
| value materialization | function-owned Binding SSA | second reaching-value map |
| recursive admission | `VerifiedRecursiveCallableModulePlanV1` | runtime discovery |
| backend admission | one recursive module capability | per-call/function marker |
| publication | unpublished drafts plus atomic candidate publish | SCC order |

## P0c-MR-G0 — shared inventory extraction

State: closed on 2026-07-16.

Landed result:

```text
one non-Clone VerifiedCallableGraphInventoryV1
one inventory-specific error owner
one production inventory consumer: existing acyclic proof
inventory accepts self/cyclic topology
acyclic proof preserves prior self/cycle rejection
SCC production consumers = 0
production behavior delta = 0
```

Evidence:

```text
debug inventory fixtures 2/2
debug acyclic fixtures 3/3
debug typed-plan fixtures 3/3
release inventory fixtures 2/2
release acyclic fixtures 3/3
release typed-plan fixtures 3/3
resolved callable authority guard green
cargo check green
quick gate 66/66
all touched source/check files below 800 lines
```

```text
production behavior delta: 0
new authority: nodes + call sites + unique edges + inventory validation
production consumer: existing acyclic proof exactly once
SCC production consumers: 0
```

Create focused `callable_graph_inventory.rs`. Move only inventory rows,
resolved-callable-to-key projection, node-set checks, target membership, and
site uniqueness out of `acyclic_callable_graph.rs`. Keep self-edge rejection,
Kahn order, and cycle proof in the acyclic product.

G0 fixtures:

```text
pass:
  current P0c-F reorder parity
  repeated sites preserve multiplicity
  unique edges deduplicate topology only
  inventory accepts self/cyclic edges
  existing acyclic proof still rejects self/cycles

reject at inventory seal:
  catalog/function node-set mismatch
  missing caller root
  foreign target or target outside node set
  duplicate caller/source-site row
```

G0 exit gate:

```text
P0c-F observable behavior unchanged
one inventory extraction implementation
sealed inventory Clone/Arc/shared-mutable usage = 0
SCC production callers = 0
recursion/backend/grammar/runtime delta = 0
all touched source/check files < 800 lines
```

## P0c-MR-S0 — deterministic SCC partition

State: closed on 2026-07-16.

Landed result:

```text
one non-Clone VerifiedCallableSccPartitionV1
one iterative deterministic Kosaraju implementation
SCC ID = minimum CanonicalCallableKeyV1
canonical member/component ordering
exact recursion classification
sorted unique condensation edges
deterministic Kahn condensation order
production callers = 0
production behavior delta = 0
```

Evidence:

```text
debug SCC fixtures 4/4
release SCC fixtures 4/4
retained inventory fixtures 2/2
retained acyclic fixtures 3/3
resolved callable authority guard green
cargo check green
quick gate 66/66
pointer and format guards green
all touched source/check files below 800 lines
```

```text
SCC ID: minimum CanonicalCallableKeyV1 in component
member order: canonical-key order
component order: SCC-ID order
condensation order: Kahn order with SCC-ID tie break
classes: NonRecursive | SelfRecursive | MutualRecursive { contains_self_edge }
```

Fixtures cover acyclic graphs, singleton self edges, two/three-node cycles,
multiple SCCs, mixed DAG/SCC, isolated nodes, repeated sites, reorder parity,
and malformed private partitions. Identity must not depend on declaration,
DFS/Kosaraju discovery, owner slots, or physical symbols.

## P0c-MR-V0 — recursive module plan

State: closed on 2026-07-16.

Landed result:

```text
one VerifiedRecursiveCallableModulePlanV1
one consumed deterministic SCC partition
one finite trivial Binding-SSA plan per canonical key
function count >= 2
call-site count >= 1
recursive component count >= 1
exact inventory/function/component/plan cardinality
exact per-function inventory/profile call-row correspondence
production callers = 0
production behavior delta = 0
```

Evidence:

```text
debug recursive-plan fixtures 3/3
release recursive-plan fixtures 3/3
retained SCC fixtures 4/4
resolved callable authority guard green
cargo check green
quick gate 66/66
pointer and format guards green
all touched source/check files below 800 lines
```

It requires:

```text
function count >= 2
call-site count >= 1
recursive component count >= 1
partition nodes == module keys == typed-plan keys
per-function inventory sites == verified direct-call rows
all function plans are CanonicalTrivialBindingSsaPlanV1
```

It owns admission only, never targets, ABI, effects, MIR, publication, backend
execution, or runtime SCC discovery.

## P0c-MR-C0 — passive backend capability

State: closed on 2026-07-16.

Landed result:

```text
one module-level canonical_recursive_callable_module_v1 marker
ModuleMetadata storage = Option<marker>
exact schema validation
duplicate installation rejection
mir-interpreter acceptance
all other backends fail-fast with stable no-fallback tag
graph/SCC/runtime discovery = 0
production marker producers = 0
```

Evidence:

```text
debug capability/backend fixtures 4/4
release capability/backend fixtures 4/4
resolved callable authority guard green
cargo check green
quick gate 66/66
pointer and format guards green
all touched source/check files below 800 lines
```

It carries no SCC IDs, members, edges, or counts.

## P0c-MR-I1 — explicit VM-only activation

State: closed on 2026-07-16.

Landed result:

```text
one explicit compile_resolved_recursive_callable_module ingress
one recursive typed-plan consumer
one shared unpublished-draft collector
one atomic complete-function batch insertion
one module capability installation
acyclic/self/legacy route retries = 0
non-VM backend fallback = 0
ownership operations = 0
```

Evidence:

```text
debug activation/proof fixtures 7/7
release activation/proof fixtures 7/7
retained complete-frame transaction fixtures 6/6
even/odd and three-function SCC execution green
outer DAG caller into SCC green
rejection then compiler reuse green
MAX_CALL_DEPTH failure then interpreter reuse green
inner parameter-contract failure then interpreter reuse green
inner return-contract failure then interpreter reuse green
VM-only backend fail-fast green
resolved callable authority guard green
cargo check green
quick gate 66/66
pointer and format guards green
all touched source/check files below 800 lines
```

The ingress consumes the typed plan, reuses the unpublished draft collector,
atomically publishes the verified batch plus one marker into the isolated
candidate, finishes the
module, and commits the outer session.

```text
acyclic ingress on recursive graph: reject
recursive ingress with no recursive component: reject
route retry/fallback: 0
other backend effects before fail-fast: 0
```

Runtime fixtures include terminating even/odd, a terminating three-function
SCC, DAG caller into SCC, inner parameter/return contract
failure with frame restoration, MAX_CALL_DEPTH restoration, compiler reuse
after rejection, and declaration reorder parity.

The Rust reference interpreter keeps `MAX_CALL_DEPTH = 16`. This is a
host-stack-safe resource/fail-fast boundary verified in both debug and release,
not a language recursion limit, termination proof, or backend-wide ABI. The
previous value `1024` allowed Rust host-stack overflow before the VM could
publish its typed error; values `128` and `32` reproduced that ordering in the
focused debug fixture. The depth-error diagnostic is optional when Ring0 is
not initialized and never initializes global runtime state. Deeper reference-
interpreter recursion requires a separate iterative call-frame/trampoline
design row and is not part of P0c-MR-I1.

## P0c-MR-R0 — Program-only self-call authority

State: Candidate B accepted and taskized.

```text
P0c-MR-R0-S0
  -> P0c-MR-R0-P0
  -> P0c-MR-R0-CUT0
  -> P0c-MR-R0-G0
```

The sole final callable source authority is one owned function-only Program,
its immutable catalog, canonical-keyed resolved function map, shared graph
inventory, deterministic SCC partition, recursive typed plan, and unpublished-
draft atomic transaction. Top-level functions retain separate single-root
semantic owner forests.

Repository evidence shows seven non-definition
`resolve_function_with_root_callable` callers, all in tests, and zero non-test
production callers. Source compatibility with that public constructor is not
claimed. No adapter is added unless a separately versioned external contract
is discovered before CUT0.

### R0-S0 — disconnected singleton admission

Production behavior delta is zero. Refactor the recursive-plan verifier behind
one private temporary admission selector:

```text
ExistingTwoOrMore
OneOrMoreForR0
```

The production `verify()` remains `ExistingTwoOrMore`. Only disconnected R0
fixtures use `OneOrMoreForR0`. The new path must accept a one-function self
edge, repeated/nested self-call sites, and the existing `SelfRecursive` SCC
classification while rejecting zero calls, no recursive component, and
inventory/profile drift. No new SCC or singleton witness type is allowed.

### R0-P0 — normalized parity proof

Production behavior delta is zero. Test-only normalization compares the old
exact-one bare-function route and disconnected singleton Program route:

```text
header key/signature/symbol
binding/scope/region kind and cardinality
verified call target/ABI/argument/result/effect rows
normalized MIR control/value/call/return relation
parameter and return contracts
terminating runtime results
non-VM fail-fast and no fallback
CopyOwned/DestroyOwned/ReleaseStrong = 0
```

Invocation-local owner IDs, compilation brands, origins, source sites, and
source-file hints are excluded. The intentional difference is exactly one
recursive module marker on the Program route. P0 also migrates high-level
self-call fixtures for execution, post-If PHI arguments, local/assignment/
final-return call results, backend fail-fast, and compiler reuse.

### R0-CUT0 — atomic production cutover and retirement

CUT0 is one commit. No landed production state may contain both self-call
authorities.

```text
1. remove the recursive plan's minimum-function policy
2. admit every non-empty module with >=1 call site and >=1 recursive component
3. activate singleton Program through the existing explicit recursive ingress
4. require one direct-static function marker and one recursive module marker
5. migrate high-level fixtures to singleton Program
6. delete RootCallable source-unit/forest/index activation authority
7. replace exact-one admission/profile policy with explicit call-forbidden
   body-only policy plus finite one-or-more Program policy
8. prove old symbols/callers and every route retry are zero
```

CUT0 deletes:

```text
VerifiedResolvedSourceUnitV1::resolve_function_with_root_callable
ResolvedSourceUnitSemanticsV1::RootCallable and the enum wrapper
VerifiedResolvedSourceUnitV1::callable_index
VerifiedResolvedCallableForestV1 and its verification error/file
FunctionSemanticResolverSessionV1::resolve_forest_with_root_callable
old callable-forest resolver error variants
VerifiedCallableIndexV1::seal_one / sole_header
CallableIndexDraftV1::seal_one
CallableCatalogCardinalityErrorV1
one-entry-only seal_exact_i64_header helper
DirectCallAdmissionV1::ExistingExactOne
DirectCallPolicyV1::ExactlyOne
exact-one analyzer facade/implementation and cardinality-only fixtures
```

`CallableFunctionSyntaxViewV1` is explicitly retained: CAT0/MP0 catalog body
resolution uses it to keep one declaration's header/body pairing exact, so it
is not old one-entry authority. `CallableHeaderSyntaxViewV1`, `seal_many`, the
module carriers, finite analyzer, exact call rows, capabilities, SCC products,
and atomic transaction are also retained.

After CUT0 the only call-admission policies are:

```text
body-only compile_resolved: Forbidden
callable Program plans: FiniteOneOrMore
```

### R0-G0 — guards and closeout

Production behavior delta is zero. Update the existing resolved-callable
guards rather than adding a per-row shell script. Prove old type/constructor/
facade/analyzer symbols and callers are zero, explicit recursive Program
ingress remains exactly one, route retries remain zero, singleton product
cardinalities are all one, both marker counts are one, emitted call counts
match resolved/profile rows, ownership operations remain zero, and every
modified source/check file stays below 800 lines.

### R0 fail-fast and non-claims

```text
body-only compile_resolved + FunctionCall -> reject before Builder
recursive singleton with zero calls/no recursive component -> reject
recursive failure -> no bare/acyclic/legacy retry
unsupported backend -> reject before backend effects
old authority deletion -> same CUT0 commit as singleton production cutover
```

R0 does not claim source compatibility, general callables, termination, deep
or constant-stack recursion, tail calls, effect precision, MethodCall,
receiver, Lambda, Loop, early Return, ownership/Box/View/Shared ABI, imports,
plugins, FFI, or another backend.

## Required counters and guards

```text
raw Lower FunctionCall.name reads = 0
MIR module-table source resolution = 0
physical-symbol identity parsing = 0
runtime graph/SCC discovery = 0
legacy resolver/retry/fallback = 0
second BindingRef -> ValueId map = 0
graph/SCC product as ABI or effect authority = 0
incremental function publication = 0

inventory nodes = catalog functions = SCC memberships
                = typed plans = drafts = published functions
inventory call sites = resolved target facts
                     = verified call rows = emitted exact Calls
inventory unique edges = internal SCC edges + cross-component unique edges

recursive module capability count = 1 at I1
direct-static capability rows = calling-function count
CopyOwned = DestroyOwned = selected ReleaseStrong = 0
MethodCall/receiver activation = 0
unsupported backend fallback = 0
partial function/capability publication = 0
```

Extend the existing resolved-callable authority guard where possible. Add a
new guard only for SCC-specific structure not expressible by the existing one.

## Stop conditions

Stop if implementation requires:

1. target re-resolution from names, symbols, MIR, or runtime tables;
2. separate site/edge extraction for acyclic and SCC products;
3. cloning or sharing the sealed inventory;
4. mutable catalog updates during graph/SCC construction;
5. declaration/discovery/owner-slot SCC identity;
6. SCC-owned target, ABI, or effect tables;
7. graph/ABI joining in Lower;
8. effect fixed point, purity, or termination proof for activation;
9. condensation order as Lower/publication order;
10. callee-first or incremental publication;
11. runtime/MIR SCC discovery;
12. per-call/function/component recursive markers;
13. route retry or silent fallback;
14. ownership, MethodCall, receiver, Loop, early Return, imports, FFI,
    plugins, another backend, or serialized transport in the same row;
15. recursive host-stack traversal over an unbounded accepted graph;
16. a touched source/check file reaching 800 lines.

## R0-S0 closeout evidence

The private temporary `ExistingTwoOrMore | OneOrMoreForR0` selector is sealed.
Production `verify()` still selects `ExistingTwoOrMore`; only disconnected
tests can select `OneOrMoreForR0`. One-function self, repeated, and nested call
fixtures pass, zero-call and acyclic/no-recursive-component fixtures reject,
and the production singleton path still rejects `FunctionCardinality`.

```text
debug recursive plan: 5/5
release recursive plan: 5/5
retained SCC: 4/4
retained recursive activation: 7/7
resolved-callable authority guard: green
cargo check: green
quick gate: 66/66
production behavior delta: 0
```

## R0-P0 closeout evidence

One `#[cfg(test)]` parity module owns both normalized snapshots and the only
test-only singleton compiler connection. It excludes invocation-local IDs,
origins, and source sites, renumbers block/ValueId identities, and compares
semantic/profile/MIR/contract/runtime/backend/ownership relations. The old
module has no recursive marker; the Program module has exactly one. All other
observed rows are equal.

```text
debug parity: 4/4
release parity: 4/4
retained old self-call: 5/5
retained recursive activation: 7/7
retained recursive plan: 5/5
retained SCC: 4/4
resolved-callable authority guard: green
cargo check: green
quick gate: 66/66
production behavior delta: 0
```

## Immediate next action

Implement `P0c-MR-R0-CUT0` as one atomic commit: remove the recursive plan's
two-function floor, activate singleton Programs through the existing explicit
recursive ingress, migrate every high-level self-call fixture, delete the old
RootCallable/one-entry/exact-one authorities, and prove old symbol/caller and
route-retry counts are zero. Do not split production cutover from retirement,
add a compatibility adapter, or begin G0 cleanup as a separate semantic row.
