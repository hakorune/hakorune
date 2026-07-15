---
Status: Design consultation; implementation authority zero
Date: 2026-07-16
Decision: Pending
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-P0C-MR-D0-SCC-DESIGN-CONSULTATION-001
Production baseline: 4bed234ceb
Previous card: mirbuilder-p0c-f-acyclic-callable-module-task-2026-07-15.md
---

# P0c-MR Callable SCC — Design Consultation Question

## Request

P0c-F is closed. Please select and specify the first durable mutual-recursion
row without implementing it.

The consultation must decide:

```text
shared topology substrate
SCC partition authority
recursive component identity/order
accepted first grammar
effect law
backend capability law
runtime frame proof
transaction reuse
task order and stop conditions
```

Do not broaden into MethodCall, receiver, Loop, early Return, ownership,
imports, plugins, FFI, or another backend.

## Closed baseline

The baseline commit is:

```text
4bed234ceb feat(mir): activate acyclic callable modules
```

P0c-F now supports one exact function-only Program containing two-or-more
static exact-`i64` functions and one-or-more direct calls, provided the module
call graph is acyclic.

Closed properties:

```text
complete immutable callable catalog
all top-level owners reserved before body resolution
all bodies resolved against the complete catalog
source call site -> resolved callable identity before Builder
one co-sealed target/argument/result/effect row per call
multiple and repeated call sites
nested and argument-position calls
multiple targets per caller
multi-hop DAGs
calls in both fallthrough If arms
one capability row per calling function
all MirFunction drafts unpublished until one atomic insertion
Rust MIR interpreter execution
CopyOwned / DestroyOwned / selected ReleaseStrong = 0
fallback/retry = 0
```

P0c-F intentionally rejects:

```text
self edge
two-or-more-node cycle
every recursive SCC
```

The existing one-function P0c-I1 route separately proves one exact self-call.
It is not a module-level SCC authority.

## Evidence from the current implementation

### Source and callable authority

The following boundaries are already sealed:

| Concern | Authority |
| --- | --- |
| Program/catalog membership | `VerifiedCallableCatalogSourceUnitV1` |
| source key lookup | `VerifiedCallableIndexV1` |
| semantic callable identity | `FunctionOwnerIdV1` through `ResolvedCallableRefV1` |
| function-relative call target | `VerifiedResolvedFunctionV1::direct_call_targets` |
| per-call target + ABI + effect | `VerifiedTrivialDirectCallV1` |
| value/PHI materialization | function-owned Binding SSA |
| unpublished function set | `VerifiedUnpublishedCallableDraftSetV1` |
| atomic publication | `MirModule::try_add_functions_atomic` |

Raw `FunctionCall.name`, physical symbols, the MIR module table, runtime tags,
and legacy resolver/retry paths are non-authorities.

### Current graph product

`VerifiedAcyclicCallableGraphV1` currently owns:

```text
canonical-key node inventory
function-relative call-site inventory
sorted unique caller/target edges
self-edge rejection
deterministic Kahn topological witness
cycle rejection with residual-node/source-site witnesses
```

It derives only from the already-resolved module. It does not resolve names or
own ABI, evaluation order, effects, MIR, publication, or SCC policy.

This type is deliberately an **acyclic proof**, not a reusable general graph
inventory. Adding an optional SCC field to it would make its name and existing
contract dishonest.

### Runtime frame evidence

The Rust MIR interpreter already:

```text
looks up an exact Callee::Global in the complete function table
opens one FunctionFrameTransactionV1 per invocation
saves and restores registers/current function/current block/call depth
rechecks final-callee parameter contracts on nested and recursive calls
rechecks final-callee return contracts on nested and recursive calls
enforces MAX_CALL_DEPTH for accidental nontermination
```

Existing focused tests prove recursive parameter and return contract checking.
This is useful substrate evidence, but it is not permission to activate source
mutual recursion without a sealed SCC product.

### Effect evidence

Every exact direct call remains:

```text
VerifiedDirectCallEffectV1::ConservativeBarrier
```

No syntax/body/name-based purity inference exists. Since the recursive edges
already carry the conservative top-like effect, an effect fixed point would
not add information in the first MR slice.

### Transaction evidence

All function drafts are already collected before the single atomic insertion.
Lower does not need the callee draft to be published in order to emit a call;
the target symbol and ABI are sealed pre-Builder. Therefore a cyclic source
graph does not by itself require a stronger publication transaction.

### Selfhost census evidence

The P0c-F census over `lang/src/**/*.hako` found:

```text
files = 1173
top-level static function declarations = 0
exact P0c-F candidate Programs = 0
recursive SCC candidates = 0
```

This means MR is also an ingress/architecture proof, not a current selfhost
coverage claim. Repeating the same census cannot select the SCC design.

## The core design question

How should a general callable graph be shared by the existing acyclic proof
and the new SCC proof without creating a second target resolver or duplicating
site/edge inventory logic?

## Candidate A — exact two-function mutual-recursion witness

```text
function count = 2
one SCC of size 2
at least one edge in each direction
self edge = rejected
```

Advantages:

```text
small activation fixture
minimal immediate implementation
```

Risks:

```text
cardinality is arbitrary rather than semantic
three-function SCC needs another activation authority
mixed DAG + SCC module needs another widening
duplicates graph inventory or still needs a general SCC product
```

Question: is there a durable invariant unique to this row, or is it only a
fixture inside a general finite-SCC module?

## Candidate B — general finite recursive exact-i64 module

Accept one complete exact-i64 module when:

```text
function count >= 2
total direct-call sites >= 1
all targets are in the complete catalog
the verified SCC partition contains at least one recursive component
recursive component = size > 1 OR singleton with a self edge
non-recursive singleton components are allowed
condensation graph is acyclic by construction
```

Advantages:

```text
no arbitrary SCC-size limit
two-function even/odd remains the first fixture
mixed DAG + recursive components use one authority
self-recursive helpers in a multi-function Program have a natural home
future SCC-aware analysis can reuse the partition
```

Risks:

```text
requires a clean shared graph-inventory substrate first
needs deterministic component identity/order
must avoid claiming termination or effect precision
```

## Candidate C — topology substrate first, then Candidate B

Refactor the current graph boundary before adding SCC semantics:

```text
P0c-MR-G0:
  VerifiedCallableGraphInventoryV1
  nodes + call_sites + unique_edges only
  derived once from VerifiedResolvedCallableModuleV1
  production behavior delta = 0

existing P0c-F proof:
  VerifiedAcyclicCallableGraphV1::verify(inventory)
  deterministic Kahn order
  rejects self/cycles exactly as today

P0c-MR-S0:
  VerifiedCallableSccPartitionV1::verify(inventory)
  deterministic SCC partition + condensation witness
  disconnected; production callers = 0

P0c-MR-V0:
  recursive module activation plan
  SCC witness + existing finite trivial function plans

P0c-MR-I1:
  existing compiler ingress + unpublished draft transaction
```

This is the local recommendation unless the consultation identifies a smaller
structure that avoids both duplicated inventory and a misleading acyclic type.

## Questions that must be answered

### 1. Select A, B, C, or a corrected alternative

State the exact task order. If A is selected, name the invariant that prevents
it from becoming a disposable cardinality witness. If B is selected directly,
explain how site/edge inventory is shared without duplicating the current S0
logic. If C is selected, state whether G0 is a BoxShape refactor series or one
behavior-neutral prerequisite commit.

### 2. Define the shared graph inventory

Should `VerifiedCallableGraphInventoryV1` own exactly:

```text
sorted canonical-key nodes
sorted function-relative call sites
sorted unique caller/target edges
node-set/site uniqueness validation
resolved callable -> canonical-key projection
```

Confirm that it owns none of:

```text
source name resolution
call ABI/effect
argument/evaluation order
Binding SSA
MIR symbols or drafts
acyclicity
SCC policy
backend capability
```

Should the current acyclic product consume this inventory by value, by borrow,
or retain a verified copy? State the ownership/lifetime shape so P0c-F does not
gain a second graph truth.

### 3. Define SCC identity and deterministic ordering

Specify the product. A possible shape is:

```rust
struct VerifiedCallableSccPartitionV1 {
    components: Box<[VerifiedCallableSccV1]>,
    component_by_callable: BTreeMap<CanonicalCallableKeyV1, CallableSccIdV1>,
    condensation_edges: Box<[VerifiedCallableSccEdgeV1]>,
    condensation_order: Box<[CallableSccIdV1]>,
}
```

Decide:

```text
Tarjan, Kosaraju, or algorithm-independent sealed contract
ordering of keys inside one component
stable CallableSccIdV1 derivation
ordering of independent components
self-edge representation
recursive-component predicate
source-site witnesses for malformed/cycle claims
```

Component identity must not depend on declaration order, DFS discovery order,
MIR symbols, or invocation-local owner slots.

### 4. Define the exact first grammar

Please fill every row:

```text
function count:
call-site count:
recursive SCC count:
SCC size:
self edges:
mutual recursion:
mixed DAG + SCC components:
nested calls:
calls in arguments:
calls in both fallthrough If arms:
early Return:
Loop:
MethodCall/receiver:
signature:
backend:
```

The existing grammar has only fallthrough statement If plus one final explicit
Return. Therefore an even/odd fixture must use assignment plus final Return,
not branch-local early Return.

### 5. Decide whether self recursion is unified

There are two relevant cases:

```text
one-function exact self-call:
  already supported by the older P0c-I1 route

multi-function Program containing a singleton self-recursive SCC:
  currently rejected by P0c-F
```

Should P0c-MR accept the second case? Should it eventually supersede the old
one-function self-call activation witness, or must that retirement be a
separate post-I1 cleanup row? Silent route retry is forbidden.

### 6. Keep or change the effect law

Recommended first law:

```text
every edge inside and outside an SCC = ConservativeBarrier
effect fixed point = not required
purity/termination inference = 0
```

Confirm or name the exact missing effect invariant. Do not infer purity from a
recursive component's syntax or lack of currently known side effects.

### 7. Define backend capability

The current capability is one direct-static-call marker per calling function
and only the Rust MIR interpreter accepts it.

Choose one:

```text
reuse it:
  recursion changes graph admission only; the runtime call operation is equal

add explicit recursive-module capability:
  necessary so a future backend may support direct calls but reject SCC calls
```

If a new capability is needed, decide whether it is module-level,
component-level, or function-level. It must not be duplicated per call site.

### 8. State the runtime proof obligation

The runtime already has frame transactions and a depth guard. Decide the first
required fixtures:

```text
two-function even/odd success
three-function recursive SCC success
mixed DAG caller -> recursive SCC success
parameter contract failure in an inner mutually-recursive frame
return contract failure in an inner mutually-recursive frame
MAX_CALL_DEPTH failure restores the outer frame exactly
rejected compilation followed by valid compilation on the same compiler
```

Clarify that `MAX_CALL_DEPTH` is a resource/fail-fast boundary, not a language
termination proof.

### 9. Confirm transaction reuse

Recommended order:

```text
1. complete immutable catalog
2. all body resolution
3. shared graph inventory
4. SCC partition and condensation proof
5. exact recursive-module preflight
6. all unpublished MirFunction drafts
7. every draft verification
8. exact catalog/graph/SCC/plan/draft correspondence
9. one atomic candidate-module insertion
10. outer session commit
```

State whether any stronger transaction is actually required. Incremental
publication must remain forbidden.

### 10. Give exact task names and claims

Recommended bounded sequence:

```text
P0c-MR-G0
  behavior-neutral shared graph inventory

P0c-MR-S0
  disconnected deterministic SCC partition

P0c-MR-V0
  disconnected recursive exact-i64 module plan

P0c-MR-I1
  atomic VM-only production activation

P0c-MR-R0 (optional later cleanup)
  retire/supersede duplicate one-function self-call activation authority
```

For each row state:

```text
production behavior delta
new authority
production caller count
pass/reject fixtures
guard/counter additions
implementation may claim
implementation must not claim
stop conditions
```

## Required non-authorities

The final decision must preserve all of these zeros:

```text
raw Lower FunctionCall.name reads = 0
MIR module-table source resolution = 0
physical-symbol identity parsing = 0
runtime graph/SCC discovery = 0
legacy resolver/retry/fallback = 0
second BindingRef -> ValueId map = 0
graph product as call ABI/effect authority = 0
incremental function publication = 0
CopyOwned / DestroyOwned / selected ReleaseStrong = 0
MethodCall/receiver activation = 0
unsupported backend fallback = 0
```

## Required implementation limits

```text
one blocker = one durable semantic slice
BoxShape graph extraction must not activate recursion
SCC activation must not add effect precision
all touched source/check files < 800 lines
unsupported backends fail before backend effects
no silent fallback or route retry
```

## Stop conditions

Stop the proposed implementation if it requires any of the following:

1. resolving a target from a source name, physical symbol, or MIR function
   table inside the graph/SCC verifier;
2. duplicating call-site/edge inventory between acyclic and SCC products;
3. mutating the callable catalog while resolving or partitioning bodies;
4. using declaration or DFS discovery order as SCC identity;
5. joining a graph target row with a separate ABI/effect table in Lower;
6. inferring purity or termination as an activation prerequisite;
7. publishing a callee draft before all recursive-component drafts verify;
8. discovering SCCs from emitted MIR or runtime tables;
9. adding a second BindingRef-to-ValueId authority;
10. adding per-call recursive capability rows;
11. enabling MethodCall, receiver, Loop, early Return, ownership, imports,
    plugins, FFI, or another backend in the same row;
12. treating `MAX_CALL_DEPTH` as a termination guarantee;
13. exceeding 800 lines in a touched source/check file;
14. silently retrying the acyclic, self-call, legacy, or another route after a
    recursive-module admission failure.

## Requested final answer format

Please answer with:

```text
selected architecture:
selected first grammar:
shared inventory owner:
SCC product and deterministic identity:
self-recursion unification law:
effect law:
backend capability law:
runtime proof obligations:
transaction law:
task order:
pass/reject fixtures:
counters/guards:
implementation may claim:
implementation must not claim:
stop conditions:
```

The most important invariant to preserve is:

> The callable catalog resolves identities once; a shared graph inventory
> projects those resolved identities once; acyclic and SCC proofs consume that
> inventory without becoming target, ABI, effect, or MIR authorities.
