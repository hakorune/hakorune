---
Status: Accepted and taskized; P0c-F-DX0a next
Date: 2026-07-15
Decision: P0c-F absorbs P0c-N; P0c-MR remains parked
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-P0C-F-DX0A-FINITE-DIRECT-CALL-ANALYSIS-001
Production baseline: 315747061e
Consultation packet: mirbuilder-p0c-next-callable-widening-consultation-question-2026-07-15.md
Previous card: mirbuilder-ssa-i1-compat-p0c-cat0-module-callable-catalog-consultation-2026-07-15.md
---

# P0c-F Exact Acyclic Callable Module — Decision and Taskboard

## Decision lock

P0c-F absorbs P0c-N. Multiple, nested, and argument-position exact direct
calls are fixtures inside one durable acyclic-module row, not a separate
production authority.

```text
P0c-F-DX0a
  -> P0c-F-DX0b
  -> P0c-F-S0
  -> P0c-F-V0
  -> read-only census evidence
  -> P0c-F-I1

then, only after a new decision:
  P0c-MR-D0 -> P0c-MR-S0 -> P0c-MR-I1
```

The next code-facing row is `P0c-F-DX0a`.

P0c-F means:

> Consume one complete exact-i64 Program/catalog module, inventory every
> already-resolved direct-call site, seal a canonical-key DAG proof, consume
> every per-function direct-call profile row exactly once in source execution
> order, keep every function draft unpublished, and publish the verified
> module atomically.

This is not a cardinality bump from two functions to three. No arbitrary
function or call-site maximum becomes a semantic rule.

## Audit corrections

The consultation answer is accepted with four implementation corrections.

### DX0 must not widen the existing self-call ingress

The current single-function `compile_resolved` route accepts exactly one
self-call through the existing exact-one preflight/profile path. Replacing the
shared preflight rule globally with `one or more` would silently widen that
production route during a supposedly behavior-neutral step.

DX0 adds a distinct generalized request with production callers zero:

```text
existing exact-one policy:
  retained for the current one-function self-call ingress

new finite one-or-more policy:
  disconnected through DX0/S0/V0
  first production caller appears only in P0c-F-I1
```

Traversal helpers may be shared, but admission is explicit. No caller infers
the wider policy from call count alone.

### Nested call consumption is postorder

Arguments execute before the enclosing call. Analyzer and Lower therefore
record and consume child call rows before the parent.

```text
step(step(x)):
  claim inner step
  claim outer step
```

The topology product never owns this execution order.

### Kahn residuals are not an exact cycle path

The first graph row reports deterministic residual nodes and residual edges
when topological exhaustion fails. It must not call arbitrary residual sites
an exact cycle witness without a separate path proof.

### Census is evidence, not a fifth row

One read-only selfhost census is attached to V0/I1 evidence for fixture
selection. It never becomes source, catalog, resolver, or activation authority.

## Closed baseline

P0c-B1 already proves:

```text
one function-only Program/catalog source unit
exactly two static exact-i64 functions
exactly one non-self direct-call site
complete-catalog body resolution
caller and target header authority kept separate
co-sealed VerifiedTrivialDirectCallV1
function-owned Binding SSA
all drafts unpublished before one atomic insertion
declaration-order parity
Rust MIR interpreter execution
unsupported backend fail-fast
ownership operations and fallback zero
```

Reusable products are retained rather than forked:

```text
VerifiedCallableCatalogSourceUnitV1
VerifiedResolvedCallableModuleV1
ResolvedCallableModuleLoweringInputV1
VerifiedCallableIndexV1
ResolvedCallableRefV1
VerifiedTrivialDirectCallV1
VerifiedCallableModulePreflightV1
VerifiedUnpublishedCallableDraftSetV1
CanonicalModuleLoweringSessionV1
```

## Exact P0c-F grammar

### Module and headers

```text
root:
  one non-empty function-only ASTNode::Program

function count:
  at least 2
  no new arbitrary semantic upper bound

every function:
  static, non-main, non-override
  no uses/contracts/attrs
  one or more exact-i64 parameters
  exact-i64 result
```

Existing checked conversions and finite allocation limits still apply.

### Calls

```text
total direct-call sites:
  at least 1

per function:
  zero or more

allowed:
  repeated caller -> target sites
  one caller -> multiple targets
  multi-hop acyclic chains
  isolated zero-call functions
  sequential and nested calls
  calls in call arguments
  call results in binary expressions
  call results in local/assignment/BlockExpr/final Return
  calls in condition/then/else expressions under existing fallthrough If law

rejected:
  every self edge on the module route
  every multi-node cycle
  unknown target or wrong arity
  unused bare FunctionCall statement
```

Repeated sites retain multiplicity in the profile and collapse only for the
unique topology edge set. Indegree is computed from unique edges.

### Still outside the grammar

```text
early or branch-local Return
Loop
MethodCall or receiver/me
Lambda/capture
Outbox
non-i64 or Box/View/Shared ABI
imports, plugin, FFI, separate compilation
main entry activation
```

Backend support remains Rust MIR interpreter only. Every other backend fails
before backend effects with no fallback.

The existing one-function self-call ingress remains a separate exact-one
route. P0c-F does not redefine it.

## Authority split

| Concern | Authority | Non-authority |
| --- | --- | --- |
| Program/catalog membership | `VerifiedCallableCatalogSourceUnitV1` | filtered Program, declaration order |
| source name/arity lookup | `VerifiedCallableIndexV1` | graph, Lower, MIR tables |
| call-site target | `VerifiedResolvedFunctionV1::direct_call_targets` | raw name in Lower |
| module topology | `VerifiedAcyclicCallableGraphV1` | target/ABI resolver |
| per-call target/ABI/effect | `VerifiedTrivialDirectCallV1` | graph edge joined later |
| evaluation/exact consumption | profile coverage + consumption ledger | topological order |
| BindingRef values/PHIs | function-owned Binding SSA | second value map |
| function plan set | typed P0c-F activation plan | Builder inference |
| MIR publication | unpublished draft set + atomic batch | incremental publication |
| backend admission | function-level direct-call capability | generic Call scan |

Each call row continues to co-seal target header, ordered argument sites,
InlineI64 result, and ConservativeBarrier. Lower never joins graph and ABI
products after the fact.

## Graph product

P0c-F-S0 introduces one topology-only product:

```rust
pub(crate) struct VerifiedAcyclicCallableGraphV1 {
    nodes: Box<[CanonicalCallableKeyV1]>,
    call_sites: Box<[VerifiedCallableGraphSiteV1]>,
    unique_edges: Box<[VerifiedCallableGraphEdgeV1]>,
    topological_order: Box<[CanonicalCallableKeyV1]>,
}
```

Every site row contains caller canonical key, function-relative
`SourceExprSiteV1`, and target canonical key.

Seal only from `VerifiedResolvedCallableModuleV1`:

```text
1. compare catalog and functions_by_key node sets
2. inventory every already-resolved direct-call target
3. project callable ref to key through catalog reverse lookup
4. reject duplicate caller/site rows and self edges
5. derive sorted unique caller/target edges
6. run deterministic Kahn sort using canonical-key ready order
7. require topological node count == graph node count
8. seal immutable rows
```

It owns node correspondence, site inventory, unique edges, acyclicity, and a
deterministic topological witness. It does not own name lookup, ABI, argument
order, result representation, effect, evaluation order, symbols, MIR Callee,
BindingRef values, drafts, publication, or SCC partition.

## Effect, capability, and transaction laws

All calls remain `ConservativeBarrier`. No purity/body/effect propagation is
introduced.

Capability changes from per-call append to one marker per calling function:

```text
profile has zero calls:
  capability rows = 0

profile has one-or-more calls:
  install exactly one marker before body-expression lowering

call emitter:
  verify the installed marker
  never append another marker
```

The transaction order is:

```text
1. complete immutable catalog
2. all body resolution
3. acyclic graph seal
4. whole-module typed trivial preflight
5. all unpublished function drafts
6. every draft verification
7. catalog/function/graph/plan/draft correspondence
8. one atomic candidate-module insertion
9. candidate finish/verification
10. outer session commit
```

No stronger publication transaction is needed. V0 converts the plan set to a
canonical-keyed map of `CanonicalTrivialBindingSsaPlanV1` before opening the
candidate Builder, so `UnsupportedPlan` is not first discovered after effects.

## Task order

### P0c-F-DX0a — finite exact-call analysis, disconnected

```text
production behavior delta = 0
new authority = 0
generalized production callers = 0
```

Work:

1. retain the exact-one admission facade for the self-call route;
2. add explicit one-or-more exact-call analysis/preflight;
3. allow recursive call traversal in arguments only under that request;
4. use checked call-count increments;
5. preserve child-before-parent profile/Lower order;
6. keep generalized module/preflight production callers zero.

Fixtures:

```text
two sequential calls
nested call argument
two sites to one target
one caller to two targets
child-before-parent row order
duplicate/missing/wrong-order claim rejection
existing self-call exact-one route unchanged
existing P0c-B1 unchanged
```

The three current blockers are all covered without global widening:

```text
resolved-value analyzer exact-one/count gate
function capability preflight 0-or-1 count gate
call-argument verification using Closed instead of exact-call recursion
```

### P0c-F-DX0b — function-level capability installation

```text
production grammar delta = 0
new authority = 0
```

Work:

1. derive capability need from the sealed profile before body lowering;
2. install exactly one marker for a function containing one-or-more calls;
3. install zero markers for a zero-call function;
4. make each call emitter verify the installed marker without mutating
   metadata;
5. retain existing self-call and B1 runtime/backend behavior.

Fixtures prove one marker per calling function, zero per non-caller, rejection
when the marker is missing/drifted, and no second-call duplicate seam.

### P0c-F-S0 — disconnected acyclic graph

```text
production behavior delta = 0
production callers = 0
```

Fixtures cover declaration reorder, repeated sites, multiple targets,
multi-hop chains, isolated nodes, self/cycle rejection, foreign-target
invariants, and deterministic residual errors.

### P0c-F-V0 — disconnected typed activation plan

Seal:

```text
function count >= 2
total direct-call sites >= 1
graph node set == catalog/function/preflight key set
graph site count == resolved target count == verified call-row count
all plans are CanonicalTrivialBindingSsaPlanV1
self/cycle rows absent
```

V0 also records one read-only selfhost census for I1 fixture selection:
function/call/edge counts, nesting, DAG depth, SCC size, and the first
MethodCall/non-i64/Loop/early-Return/Lambda/ownership boundary. It is evidence
only and not a new semantic row.

### P0c-F-I1 — atomic production activation

Replace the B1 witness at the existing explicit compiler ingress with the
verified P0c-F plan. Reuse the unpublished draft transaction and atomic
insertion. P0c-B1 becomes a positive subset fixture, not a second module route.

```text
production caller = exactly one compiler ingress
backend = Rust MIR interpreter only
fallback/retry = 0
```

## I1 pass/reject matrix

Pass:

```text
forward/backward declaration order
two sequential calls to one target
nested call
one caller reaching multiple targets
three-function multi-hop chain
repeated sites / one unique edge
calls in both fallthrough If arms
isolated zero-call function
late lowering/draft failure with zero partial publication
rejected compile followed by valid compile on same compiler
MIR interpreter result parity
```

Reject:

```text
one-function module on F ingress
zero total call sites
unknown target or wrong arity before Builder
duplicate/missing/wrong-order call-row claim
self edge or multi-node cycle
early/branch Return
Loop
MethodCall/receiver
non-i64 ABI
unsupported backend before backend effects
```

## Counters and guard

```text
raw Lower FunctionCall.name reads = 0
MIR module-table source resolution = 0
legacy resolver/retry/fallback = 0
physical-symbol identity parsing = 0
graph source-name lookup = 0
partial publication = 0
second BindingRef -> ValueId map = 0
CopyOwned/DestroyOwned/selected ReleaseStrong = 0
MethodCall/receiver activation = 0
unsupported backend fallback = 0

graph nodes
  == catalog functions
  == resolved units
  == typed plans
  == unpublished drafts
  == published callables

graph call sites
  == resolved direct-call targets
  == verified direct-call rows

capability rows
  == functions containing one-or-more calls
```

Extend `resolved_callable_l0.py`; do not add one shell guard per sub-row. If a
touched Rust/check file approaches 800 lines, split a focused helper module
before adding policy. Never cross 800 lines or mix unrelated cleanup into the
semantic row.

## Claims and non-claims

After I1, implementation may claim exact-i64 static modules with two-or-more
functions, one-or-more direct-call sites, repeated/multiple-target/multi-hop
DAG edges, nested/argument calls, declaration-order independence, exact-once
call-row consumption, one capability marker per caller, atomic publication,
and Rust MIR interpreter execution.

It must not claim general callables, self/mutual recursion on the module route,
SCC/termination/purity/effect inference, early Return, Loop,
MethodCall/receiver, Lambda/capture, Box/View/Shared ownership, imports,
plugin/FFI, separate compilation, main, other backends, fallback, or recovery.

## Stop conditions

Stop if any step requires:

1. raw names, symbols, MIR tables, or runtime tables for target resolution;
2. graph rows as a second target/ABI/effect authority;
3. mutable catalog updates during body resolution;
4. declaration-order resolution semantics;
5. an arbitrary production function/call-site maximum;
6. per-call capability duplication;
7. incremental publication claimed as order independent;
8. a second BindingRef-to-ValueId map;
9. runtime/MIR-symbol cycle detection;
10. SCC vocabulary inside the acyclic graph row;
11. ownership, MethodCall, receiver, Loop, early Return, imports, or backend
    widening;
12. effect precision beyond ConservativeBarrier as a prerequisite;
13. widening the exact-one self-call ingress before I1;
14. any modified source/check file exceeding 800 lines.

## Post-F order

P0c-MR remains parked until P0c-F-I1 is green. Its first task is a new design
decision for SCC partition, recursive target-set closure, conservative
recursive-call effects, and mutually recursive frame execution. It inherits
no SCC authority from the acyclic graph product.
