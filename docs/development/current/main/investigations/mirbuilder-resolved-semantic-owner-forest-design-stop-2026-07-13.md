---
Status: Design consultation stop
Date: 2026-07-13
Scope: Resolved Semantic Arena V1 nested owners and expression-level exits
Parent: mirbuilder-resolved-region-flow-v1-task-2026-07-13.md
---

# Resolved Semantic Owner Forest V1 — Design Consultation

## Decision already fixed

The accepted base architecture is not under reconsideration:

```text
canonical function AST
  -> FunctionSemanticResolverV1
  -> VerifiedResolvedFunctionV1
  -> Planner / RegionFlow
  -> Lower materialization
```

`VerifiedResolvedFunctionV1` is one immutable, owner-scoped semantic arena.
It owns `BindingId`, `ScopeId`, `RegionId`, resolved variable/assignment
indices, and exact control targets. `BindingOriginV1` and `RegionOriginV1`
are checked provenance, not identity authorities. Lower allocates `ValueId`
and blocks only; it does not resolve names or targets again.

The disconnected producer, seal/verifier, exhaustive 57-variant classifier,
leaf-expression traversal, assignment-target resolution, Nowait binding
identity, and TaskScope/FastMem lexical containers are green. Production
installation remains zero.

## Why implementation stops here

The remaining canonical variants cross a semantic owner or exit-domain
boundary rather than merely adding recursive traversal:

```text
Lambda:
  nested function owner, parameters/locals, captures, return target

QMarkPropagate:
  expression-site early return

MatchExpr / EnumMatchExpr:
  arm regions, arm lexical scopes, pattern binders, result merge

TryCatch / Throw:
  exception target, catch binder/scope, finally/cleanup ports

BlockExpr:
  expression result plus explicit lexical-scope contract
```

Putting a lambda's declarations into its parent function arena would violate
owner-scoped identity. Putting QMark/Throw into the existing statement-only
exit index would lose exact source identity. Letting the resolver decide
match result merges or finally execution would mix identity resolution with
RegionFlow semantics.

## Consultation question

Please choose the smallest clean extension of the accepted owner-scoped
semantic arena. The recommended candidate is below, but it is not yet an
implementation decision.

### 1. Semantic owner topology

Should nested functions use an owner forest?

```text
A — VerifiedSemanticOwnerForestV1 (recommended)

  compilation/function-body owner
    -> independently sealed VerifiedResolvedFunctionV1 products
    -> explicit parent/child owner edges
    -> explicit capture edges

B — independent product registry

  separately sealed function products keyed by owner
  no canonical parent/child forest

C — defer nested owners

  Lambda remains exact Unsupported through the production cutover
```

If A is selected, should a captured outer binding become a child-local
`Capture` binding whose record points to a parent `BindingRefV1`, rather than
reusing the parent's `BindingId` inside the child?

Recommended invariant:

```text
child BindingId equality:
  child lexical identity only

capture edge:
  child Capture BindingId -> parent BindingRefV1

cross-owner raw BindingId reuse:
  forbidden
```

Also decide who allocates owner IDs, how owner-forest sealing proves no
dangling/cyclic capture edge, and whether recursive/self capture requires a
separate closed constructor.

### 2. Exact exit-site identity

The current side table is statement-keyed. QMark and Throw may originate at
expression sites. Which closed identity should replace it?

```text
A — ResolvedExitSiteV1 (recommended)

  Statement(SourceStmtSiteV1)
  Expression(SourceExprSiteV1)

B — generalized SourceNodeSiteV1

  one node-site type with a verified statement/expression family tag

C — separate statement-exit and expression-exit maps
```

The exit vocabulary must distinguish semantic target identity from later
edge-state/cleanup behavior. Please classify the minimum accepted family:

```text
Continue(target_loop: RegionId)
Break(target_loop: RegionId)
Return(target_function: RegionId)
QMarkReturn(target_function: RegionId)
Throw(target_exception_region or function boundary)
```

Should `QMarkReturn` remain a distinct origin/kind for diagnostics while
sharing the function return target, or normalize immediately to `Return`?

### 3. Resolver versus RegionFlow ownership

Proposed boundary:

```text
FunctionSemanticResolverV1 owns:
  owner / binding / scope / region identity
  declaration and use resolution
  capture edges
  exact break/continue/return/throw target identity
  match-arm and catch lexical binder identity

ResolvedRegionFlowV1 owns:
  port propagation
  condition/evaluation flow
  match result merge
  QMark success/early-return branching
  throw/catch/finally flow
  edge binding-state contracts
  cleanup obligations

Lower owns:
  BindingId -> ValueId
  RegionId -> block materialization
```

Is this split correct? In particular:

1. Does the resolver create Match/arm and Try/Catch/Finally `RegionId`s while
   leaving result/exception flow entirely to RegionFlow?
2. Is `finally` a plain region plus verified cleanup-port contract, or does it
   require a different semantic owner/product?
3. Is a BlockExpr always a real lexical `ScopeId` plus expression region, or
   are there canonical AST variants where it is sequencing-only?
4. May the resolver reject unsupported control semantics before publishing,
   while RegionFlow reports unsupported execution vocabulary later?

## Canonical authority proposed

```text
per-function lexical/control identity:
  VerifiedResolvedFunctionV1

cross-function ownership and capture correspondence:
  VerifiedSemanticOwnerForestV1

source provenance:
  BindingOriginV1 / RegionOriginV1 / ResolvedExitSiteV1

control and state-flow execution contract:
  VerifiedRegionFlowV1

MIR identity:
  ValueId / BasicBlockId allocated only by Lower
```

## Non-authority

```text
variable/function names
raw owner-local integer IDs across functions
source paths as semantic equality
AST pointer identity
Lower loop stack depth
Recipe success alone
capture discovery in Lower
QMark desugaring into a synthetic statement
Match/Try result merge in the resolver
ProgramV0
```

## Smallest implementation slice after consultation

The next code-facing slice must prove the selected outer shape without
connecting Planner or Lower.

Recommended `OF0` if the owner forest is accepted:

```text
1. add owner-local FunctionOwnerIdV1 and sealed owner forest builder
2. resolve one non-capturing Lambda as an independently sealed child product
3. record one parent -> child owner edge
4. keep Lambda expression site -> child owner lookup as a verified index
5. reject captures explicitly in OF0
6. publish no AST clone and allocate no ValueId
7. keep production resolver installation = 0
```

Recommended `EX0` after the exit-site decision:

```text
1. add the chosen typed exit-site identity
2. move current statement exits without behavior change
3. resolve one QMark expression to the owning function return target
4. do not build QMark branch/merge MIR
5. keep Planner/RegionFlow/Lower connections = 0
```

These are separate commits. Match and Try/Catch remain later slices after the
owner and exit schemas are executable.

## Required gates

```text
owner forest:
  distinct owner-local BindingId(0) values never compare cross-owner
  child product sealed independently
  parent/child edge exact and acyclic
  missing child product rejected
  capture edge owner/kind checked
  nested owner AST clone ownership = 0

exit site:
  statement and expression sites cannot alias accidentally
  one exact exit per accepted source site
  target belongs to the same owner forest
  QMark target is owning function, not nearest loop
  Lower depth recount = 0

responsibility:
  resolver allocates no ValueId/block
  RegionFlow allocates no BindingId/ScopeId/RegionId
  Lower performs no name/capture/target rediscovery
```

Rust/Hako parity compares normalized origins and graph edges, never raw arena
numbers.

## Implementation may claim after the first slices

```text
OF0:
  nested function syntax can own an independently sealed semantic product
  parent/child owner identity is explicit
  no nested declaration is inserted into its parent function arena

EX0:
  statement and expression exits have exact typed source identity
  QMark target identity is resolved before Planner/Lower
```

## Implementation must not claim

```text
closure capture lowering complete
capture mode/ownership semantics complete
recursive closure support complete
Match result lowering complete
exception/finally semantics complete
full AST resolver coverage
production semantic authority cutover complete
legacy loop_var retirement complete
```

## Stop conditions

```text
1. Lambda declarations are inserted into the parent function arena.
2. A parent BindingId is reused as a child owner-local BindingId.
3. Capture identity is inferred again in Planner or Lower.
4. QMark/Throw are keyed by a fabricated statement path.
5. Expression exits are normalized before exact origin is recorded.
6. Resolver allocates ValueId or BasicBlockId.
7. RegionFlow allocates semantic owner/binding/scope/region IDs.
8. Match result merge or finally cleanup semantics become resolver facts.
9. Lower counts loop depth or searches AST to recover an exit target.
10. Unsupported nested/control syntax retries the legacy resolver.
11. Partial child products or a partial owner forest are published.
12. Raw owner-local IDs are compared for Rust/Hako parity.
```

## Requested final answer

Please return:

```text
Decision
Reasoning
owner topology and capture identity
exit-site schema and exit vocabulary
resolver / RegionFlow / Lower responsibility split
Match / Try / BlockExpr boundary
smallest implementation slices
required fixtures/gates
Rust/Hako parity
retirement path
implementation may/must-not claim
stop conditions
```

