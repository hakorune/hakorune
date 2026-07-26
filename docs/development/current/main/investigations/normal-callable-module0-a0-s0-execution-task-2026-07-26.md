---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-CALLABLE-MODULE0-A0-S0
Scope: combine one completed Main plan with one non-empty top-level helper module and seal a deterministic acyclic whole-normal-source plan before Builder effects
ceremony_tier: T1 bounded composition of existing Main, catalog, graph, and function-plan owners
proof_inventory_before: one Program-owned Main/helper catalog plan plus existing callable-module resolver and acyclic graph/function preflights
new_proofs: one normal Main/helper correspondence and one zero-edge-capable normal helper DAG plan
retired_or_merged_proofs: none
net_proof_delta: one durable outer normal-module plan
sunset_budget: repaid when NORMAL-CALLABLE-MODULE0-TX0-S0 consumes the outer proof into the sole atomic module transaction
sunset_row: NORMAL-CALLABLE-MODULE0-TX0-S0
retire_when: atomic normal module transaction consumes the plan and no disconnected A0 proof consumer remains
Related:
  - docs/development/current/main/investigations/normal-main-direct-call0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - src/mir/compiler/acyclic_callable_graph.rs
  - src/mir/compiler/acyclic_callable_module_plan.rs
---

# NORMAL-CALLABLE-MODULE0-A0-S0

## Outcome

Consume the closed Main/helper source plan into:

```text
one original Program owner
+ one completed Main F1/direct-call proof
+ one completely resolved helper module
+ one deterministic helper DAG
+ one exact Main-call-row/helper-catalog correspondence
+ one retained source-entry relation input
```

The row is Builder-free, MIR-free, backend-free, and runner-free.

## Ownership chain

```text
VerifiedNormalMainDirectCallPlanV1
  ↓ consuming split
PreparedNormalMainHelperResolutionV1
  - owned Main completion/control/value evidence
  - CallableCatalogSealOutcomeV1
  - exact Main source identity/site evidence
  ↓ existing sole helper resolution
VerifiedResolvedCallableModuleV1
  ↓ borrow-only whole-source preparation
VerifiedNormalAcyclicCallableModulePlanV1<'_>
  - Main proof view
  - helper DAG
  - helper function plans
  - Main/helper correspondence
  ↓ later consuming handoff
NORMAL-CALLABLE-MODULE0-TX0-S0
```

The original Program moves into `VerifiedResolvedCallableModuleV1`. Main's
forest/projection/completion/profile are durable products and remain paired
with the exact Main source site. Do not retain a second Program or clone the
Main function.

## Helper DAG profile

`VerifiedAcyclicCallableGraphV1` remains the sole topology owner and already
supports a non-empty graph with zero edges.

The historical `VerifiedAcyclicCallableModulePlanV1` keeps its existing
two-or-more/one-or-more-call admission unchanged. The normal row must not
widen that frozen proof in place.

Add one normal-specific borrowed preflight:

```text
helper with zero direct calls
  -> existing CanonicalLoweringPreflightV1::verify_function

helper with one-or-more direct calls
  -> existing verify_function_with_finite_direct_calls_v1

all helper plans
  -> exact graph/function/plan/call-row correspondence
```

This admits one call-free helper, multiple independent helpers, and finite
acyclic helper calls. Self edges, cycles, missing targets, arity drift,
unsupported helper profiles, and correspondence drift reject before Builder
effects. Recursive SCCs remain `NORMAL-CALLABLE-MODULE0-R0-S0`.

## Main/helper correspondence

Require:

```text
Main owner and every helper owner share one compilation brand
Main is absent from helper catalog keys
every Main direct-call row resolves to one exact helper header
Main call-row count equals its sealed profile count
helper catalog cardinality equals resolved helper cardinality
source identity and Main site remain unchanged
```

Do not infer correspondence from source names during lowering, physical
symbols, module inventory scans, VM results, or declaration order.

## Failure retention

```rust
pub(crate) struct RejectedNormalAcyclicCallableModulePlanV1 {
    owner: CompletedNormalMainHelperResolutionV1,
    stage: NormalAcyclicCallableModuleStageV1,
    error: NormalAcyclicCallableModuleErrorV1,
}
```

Only `stage()`, `error()`, and `discard(self)` are public terminals. Retry as
call-free, Raw, recursive, or Legacy is forbidden. Partial publication is
zero.

## Buildable task order

```text
A0-A:
  add the consuming Main-plan split
  retain durable Main evidence beside one catalog continuation

A0-B:
  consume the continuation through existing helper-module resolution
  retain complete owner on typed failure

A0-C:
  seal zero-edge-capable normal helper DAG/function plans
  reuse existing acyclic graph and both existing function preflights

A0-D:
  seal exact Main/helper brand, target, and cardinality correspondence

A0-E:
  focused success/rejection/reorder/reuse fixtures
  extend the existing normal-source-plan lane guard only
```

## Fixture matrix

Success:

```text
call-free Main + one call-free helper
Main -> one helper
Main -> helper + two independent helpers
Main -> root helper with a forward acyclic helper call
nested/multiple Main calls plus an acyclic helper graph
helper declaration reorder
```

Rejection:

```text
helper self-edge
helper cycle
missing helper target
helper arity mismatch
helper profile rejection
Main/helper compilation-brand drift fixture
call-row/catalog correspondence drift fixture
late helper resolution failure with owner retained
```

Reuse:

```text
success -> rejection -> success
```

This row claims semantic-owner reuse, not `MirCompiler` or VM-session reuse.

## Structural gate

```text
normal Program owner count                         = 1
helper catalog producer                            = 1
helper graph owner                                 = existing acyclic graph
Main catalog membership                            = 0
Main/helper compilation brand                      = 1

Main AST clone/rewrite                             = 0
second helper catalog/index                        = 0
second resolver session                            = 0
lowering-time name/symbol lookup                   = 0
retry/fallback                                     = 0

Builder/MIR/module publication                     = 0
VM/runner/CLI caller                               = 0
existing VM0 route delta                           = 0
all modified/new source/check files                < 800 lines
```

## Immediate continuation

```text
NORMAL-CALLABLE-MODULE0-A0-S0
  -> NORMAL-CALLABLE-MODULE0-R0-S0
  -> NORMAL-CALLABLE-MODULE0-TX0-S0
  -> NORMAL-FILE-CANONICAL-CORE0-PROFILE0-S0
  -> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a
  -> NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0
  -> NORMAL-FILE-CANONICAL-CORE0-CALLER0-I0
  -> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0b
  -> NORMAL-FILE-CANONICAL-CORE0-G0
  -> MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0
  -> NORMAL-ENTRY-PRODUCT-BACKEND-D0
  -> NORMAL-DEFAULT-CALLER-CENSUS0-P0
  -> NORMAL-ENTRY-PROMOTION-D3
  -> NORMAL-IMPORT-BUNDLE0
  -> MIRBUILDER-LEGACY-FENCE0
  -> MIRBUILDER-NORMAL-COMPLETE0
  -> MIRBUILDER-COMPLETE0-G0
```

## Non-claims

```text
recursive helper activation
Builder/MIR draft creation
atomic module publication
physical Main thunk
VM execution
new production/default caller
imports / using
Main-box helper methods
instance methods / receiver
dynamic/object result carrier
nested/multiple/all-path Return
cleanup activation
Legacy retirement
```

## Closeout

```text
Status:
  closed

Landed:
  9a42606151 feat: prepare normal acyclic module plan
  1e8a8dfde5 test: close normal acyclic module plan

Accepted normal acyclic surface:
  one call-free helper
  multiple independent helpers
  finite acyclic helper calls
  call-free or finite Main calls

Rejected:
  helper self edge
  helper cycle
  unresolved helper call

Structural result:
  original Program owner       = 1
  helper catalog               = 1
  helper resolver continuation = 1 consumed once
  acyclic graph owner          = existing owner
  Main catalog membership      = 0
  second resolver/catalog      = 0
  retry/fallback               = 0
  Builder/MIR/backend caller   = 0

Evidence:
  normal_source_plan tests     = 47 green
  resolved callable tests      = 9 green
  normal source-plan guard     = green
  vm-reference library check  = green
  pointer guard                = green
  touched source/check files   < 800 lines

Next:
  NORMAL-CALLABLE-MODULE0-R0-S0
```
