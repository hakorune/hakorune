---
Status: Answered; decision taskized in P0c-F taskboard
Date: 2026-07-15
Decision: P0c-F absorbs P0c-N; P0c-F-DX0a selected next
Scope: select the callable-widening order after the closed P0c-B1 slice
Current baseline: 315747061e
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-P0C-NEXT-DESIGN-STOP-001
Parent card: mirbuilder-ssa-i1-compat-p0c-cat0-module-callable-catalog-consultation-2026-07-15.md
Implementation authority: none; this document is a question packet only
Decision taskboard: mirbuilder-p0c-f-acyclic-callable-module-task-2026-07-15.md
---

# Consultation: What Is the Correct Callable-Widening Order after P0c-B1?

## Requested decision

Please select and fully specify the next callable-widening sequence after the
closed exact sibling-call slice.

The three named candidate families are:

```text
P0c-N:
  multiple and/or nested exact direct calls
  no mutual recursion

P0c-F:
  broader acyclic function and call-edge cardinality
  no recursive SCC

P0c-MR:
  mutual recursion
  callable SCC validation and recursive effect closure
```

Do not answer only with “choose the smallest row.” The decision must optimize
for the shortest structurally sound path toward selfhosting while preserving
the current authority split, atomic publication, fail-fast behavior, and the
800-line source/check-file limit.

We need an explicit verdict on whether `P0c-N` and `P0c-F` are genuinely
separate durable semantic rows, or whether one should absorb the other.

## Closed production baseline

Commit `315747061e` closes exactly this production family:

```text
source module:
  exactly two static functions
  every parameter/result spelling exactly i64

call graph:
  exactly one direct-call edge
  edge is non-self
  self-only, zero-edge, and two-edge/mutual examples reject

source call:
  ASTNode::FunctionCall only
  exact name + arity lookup in the complete sealed catalog

semantic identity:
  CanonicalCallableKeyV1 for normalized module lookup
  ResolvedCallableRefV1 for invocation-local target membership
  CanonicalCallableSymbolV1 for physical MIR/backend materialization

authority:
  one Program/catalog source unit is co-sealed
  each function retains one single-root semantic owner forest
  every body resolves against the complete immutable catalog
  caller header and target header remain distinct sealed authorities
  direct-call ABI and target are co-sealed in VerifiedTrivialDirectCallV1

Lower:
  no raw FunctionCall name lookup
  no MIR module-table source resolution
  no physical-symbol parsing
  no legacy/global call resolver
  no fallback or retry

publication:
  all function plans finish before Builder effects
  every MirFunction remains unpublished until verified
  all drafts are batch-inserted atomically after exact correspondence checks

runtime/backend:
  Rust MIR interpreter only
  unsupported backends fail before backend effects

ownership:
  CopyOwned = 0
  DestroyOwned = 0
  selected-route ReleaseStrong = 0
```

Forward and backward declaration order produce the same semantic result, and
the first sibling fixture executes `caller(41) == 42`.

## Existing reusable products

The next slice should reuse rather than fork these products:

```text
VerifiedCallableCatalogSourceUnitV1
CatalogSealedOwnerContinuationV1
VerifiedResolvedCallableModuleV1
ResolvedCallableModuleLoweringInputV1
VerifiedResolvedCallableProgramV1
VerifiedCallableIndexV1
ResolvedCallableRefV1
VerifiedTrivialDirectCallV1
VerifiedResolvedModulePreflightV1
VerifiedMirFunctionDraftSetV1
CanonicalModuleLoweringSessionV1
```

The existing direct-call profile already stores a collection of call rows.
The P0c-B1 activation witness, not the generic row schema, owns the current
`two functions / one non-self edge` restriction.

## Main decision axis

### Candidate A — P0c-N first

Example intended shape:

```hako
static twice(x: i64): i64 {
    local a = step(x)
    return step(a)
}

static step(x: i64): i64 {
    return x + 1
}
```

Possible first claim:

```text
function count:
  exactly 2

direct call sites:
  exactly 2

call graph:
  acyclic

new proof:
  multiple exact call-row consumption in source order
  distinct result ValueIds
  nested or sequential argument/result composition
```

Question: does this isolate a real missing call-expression/coverage law, or is
it an artificial cardinality row that should be absorbed by P0c-F?

### Candidate B — P0c-F first

Example intended shape:

```hako
static entry(x: i64): i64 {
    return middle(x)
}

static middle(x: i64): i64 {
    return leaf(x) + 1
}

static leaf(x: i64): i64 {
    return x + 1
}
```

Possible first claim:

```text
function count:
  bounded but greater than 2

direct call sites/edges:
  bounded but greater than 1

call graph:
  acyclic

new proof:
  graph-wide exact edge inventory
  arbitrary declaration order
  all targets present in one complete catalog
  multiple call rows per function where the graph requires them
```

Question: should the next durable slice be “one verified acyclic exact-i64
module” rather than another exact cardinality increment? If bounded limits are
still required, which limits are semantic and which are merely fixture limits?

### Candidate C — P0c-MR first

Example intended shape:

```hako
static even(n: i64): i64 {
    if n == 0 {
        return 1
    }
    return odd(n - 1)
}

static odd(n: i64): i64 {
    if n == 0 {
        return 0
    }
    return even(n - 1)
}
```

This likely requires:

```text
call graph SCC construction
recursive callable-set validation
effect fixed point or an explicit conservative SCC effect contract
termination-independent runtime contract checks
mutually recursive frame execution
```

Question: is any of this required before an acyclic module can materially
advance selfhosting? If not, state the exact prerequisite that should keep
P0c-MR parked.

## Questions that must be answered

### 1. Select the task order

Choose one exact sequence, for example:

```text
P0c-N -> P0c-F -> P0c-MR
P0c-F (absorbs N) -> P0c-MR
P0c-F0 -> P0c-N -> P0c-F1 -> P0c-MR
```

Explain why that order advances actual selfhost capability rather than merely
making the easiest current test pass.

### 2. Decide whether N and F are separate semantic rows

State one of:

```text
N and F are separate:
  name the invariant unique to each

F absorbs N:
  name the one activation witness and bounded first grammar

N is only a fixture inside F:
  explain why no separate production authority is needed
```

Avoid keeping two rows that differ only by an arbitrary number.

### 3. Define the next accepted grammar exactly

Specify:

```text
function count
call-site count
edge count
self recursion allowed or rejected
mutual recursion allowed or rejected
nested call expressions allowed or rejected
calls in arguments allowed or rejected
calls in both If arms allowed or rejected
early Return allowed or rejected
Loop allowed or rejected
callable signature profile
backend profile
```

No unspecified “general direct calls” claim is acceptable.

### 4. Define the graph authority

Should the next activation witness consume:

```text
the existing per-function direct-call rows only
an additional whole-module verified call-edge inventory
an acyclic-order/topological witness
an SCC partition
```

If a new whole-module graph product is needed, define exactly what it owns and
what remains owned by the callable catalog, resolved function rows, Binding
SSA, and MIR draft transaction.

It must not become a second callable-target resolver.

### 5. Preserve the target/ABI authority split

Confirm whether each call continues to use one co-sealed
`VerifiedTrivialDirectCallV1` containing:

```text
resolved target header
ordered argument sites
exact result representation
conservative effect
```

Lower must not join a target table and a separate ABI table after the fact.

### 6. Decide the effect law

The current direct call is `ConservativeBarrier`.

State whether:

```text
acyclic widening keeps ConservativeBarrier
P0c-MR introduces the first effect fixed point
or a prior verified effect row is required
```

Do not claim purity from syntax, name, or lack of observed side effects.

### 7. Preserve transaction and declaration-order independence

Confirm the required order:

```text
complete catalog
all body resolution
whole-module preflight
all unpublished function drafts
all draft verification
exact catalog/draft correspondence
one atomic candidate-module insertion
outer session commit
```

State whether any candidate requires a stronger transaction product. No
incremental publication may be described as order independent.

### 8. Connect the choice to selfhost evidence

Specify the smallest read-only census that should be run before activation, if
one is necessary. Useful classifications may include:

```text
functions per source Program
direct FunctionCall sites per function
acyclic call-edge cardinality
nested calls
self recursion
mutual recursion / SCC size
unsupported MethodCall/receiver/Box-return boundaries
```

The census may rank candidates but must not become source or semantic
authority. If no census is needed, explain why the selected row is already the
unavoidable structural prerequisite.

### 9. Give the implementation sequence

Provide named, bounded steps. Prefer a small series such as:

```text
DX0:
  behavior-neutral graph/activation facade, production callers 0

S0:
  disconnected verified module-call product

V0:
  graph/cardinality/acyclic verifier and negative fixtures

I1:
  atomic production activation through the existing compiler ingress
```

For every step, state:

```text
production behavior delta
new authority, if any
caller count
fixtures/gates
claim and non-claim
```

Do not create one shell guard per observation. Extend the existing reusable
`resolved_callable_l0.py` authority guard unless a genuinely different proof
runner is required.

### 10. Give exact pass/reject fixtures and counters

At minimum include:

```text
forward/backward declaration order
multiple calls to one target
one caller reaching multiple targets
multi-hop acyclic chain
late function/draft failure with zero partial publication
unknown target
wrong arity
duplicate/missing call-row claim
self edge
mutual edge
unsupported backend fail-fast
```

State which are pass or reject for the selected first slice.

Required counters should cover:

```text
raw Lower FunctionCall.name reads = 0
MIR module-table source resolution = 0
legacy call resolver/retry = 0
physical-symbol identity parsing = 0
partial publication = 0
second BindingRef -> ValueId map = 0
CopyOwned/DestroyOwned/ReleaseStrong = 0
MethodCall/receiver activation = 0
unsupported backend fallback = 0
modified source/check files over 800 lines = 0
```

### 11. State implementation claims and stop conditions

Separate:

```text
Implementation may claim
Implementation must not claim
Stop conditions
```

Stop if the selected slice requires ownership widening, MethodCall/receiver,
Box/View/Shared result ABI, imports, plugin/FFI resolution, Loop activation,
legacy fallback, or a new callable identity authority.

## Non-authorities

The answer must preserve these as non-authorities:

```text
raw FunctionCall.name inside Lower
MirModule.functions as source resolver
builder declaration/static-method indexes
legacy global/static resolver
unique-name, suffix, or nearest-arity recovery
physical symbol strings
runtime function tables or result tags
BoxCallableRegistry / plugin/type catalogs
ValueId equality
declaration order
```

## Required response format

Please answer in this order:

```text
1. Final selection and task order
2. Whether P0c-N and P0c-F remain separate
3. Exact accepted first grammar
4. Whole-module call-graph product, or proof that none is needed
5. Authority and non-authority table
6. Effect and transaction laws
7. Selfhost census decision
8. Implementation steps
9. Pass/reject fixtures and counters
10. Implementation may claim / must not claim
11. Stop conditions
12. One-paragraph final decision lock
```

The final decision lock must select exactly one next code-facing row. It must
not defer the same N-versus-F question to implementation.
