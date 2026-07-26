---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-CALLABLE-MODULE0-R0-S0
Scope: select acyclic or recursive helper topology once from one verified inventory and seal the recursive normal helper plan without retrying another family
ceremony_tier: T1 bounded composition of existing graph inventory, SCC partition, function preflights, and Main/helper correspondence
proof_inventory_before: closed normal acyclic subplan plus existing deterministic SCC and recursive callable-module proofs
new_proofs: one one-shot normal helper-topology selection and one recursive normal helper variant
retired_or_merged_proofs: direct acyclic-only production selection remains disconnected and is absorbed by the topology selector
net_proof_delta: one durable recursive variant with one shared selector
sunset_budget: repaid when NORMAL-CALLABLE-MODULE0-TX0-S0 consumes both variants through one atomic terminal
sunset_row: NORMAL-CALLABLE-MODULE0-TX0-S0
retire_when: the transaction consumes VerifiedNormalHelperTopologyPlanV1 and no variant-specific production selector exists
Related:
  - docs/development/current/main/investigations/normal-callable-module0-a0-s0-execution-task-2026-07-26.md
  - src/mir/compiler/callable_graph_inventory.rs
  - src/mir/compiler/callable_scc_partition.rs
  - src/mir/compiler/recursive_callable_module_plan.rs
---

# NORMAL-CALLABLE-MODULE0-R0-S0

## Outcome

```rust
pub(crate) enum VerifiedNormalHelperTopologyPlanV1<'a> {
    Acyclic(VerifiedNormalAcyclicCallableModulePlanV1<'a>),
    Recursive(VerifiedNormalRecursiveCallableModulePlanV1<'a>),
}
```

Selection is:

```text
CompletedNormalMainHelperResolutionV1
  -> VerifiedCallableGraphInventoryV1 exactly once
  -> VerifiedCallableSccPartitionV1 exactly once
  -> recursive_component_count
       0     => Acyclic
       1..   => Recursive
```

It is not:

```text
try Acyclic
-> cycle error
-> retry Recursive
```

## Shared plan law

Both variants reuse:

```text
helper with zero direct calls
  -> CanonicalLoweringPreflightV1::verify_function

helper with one-or-more direct calls
  -> verify_function_with_finite_direct_calls_v1

Main/helper:
  one compilation brand
  exact target membership
  exact call-row count
  Main absent from helper catalog
```

The shared helper-plan correspondence is sealed before the variant is issued.

## Acyclic projection

When every SCC is non-recursive:

```text
verified SCC partition
  -> consume its existing inventory
  -> existing acyclic graph owner
```

Do not rescan source or rebuild a second graph inventory. A small consuming
`into_inventory()`/`from_nonrecursive_partition()` seam is allowed if it keeps
the partition and graph authorities singular.

## Recursive projection

When at least one recursive SCC exists, retain:

```text
one VerifiedCallableSccPartitionV1
one stable component identity per SCC
one condensation order
one helper plan per canonical key
one or more recursive components
```

Admit:

```text
self recursion
mutual recursion
recursive SCC plus call-free leaf components
recursive SCC plus acyclic upstream/downstream components
Main call into a recursive helper component
```

Reject:

```text
partition/cardinality drift
missing helper plan
helper body/result outside profile
Main/helper brand or target drift
```

## Failure retention

Topology planning borrows `CompletedNormalMainHelperResolutionV1`. Therefore
every inventory, partition, preflight, or correspondence rejection leaves the
complete owner with the caller. No `into_owner`, retry, rollback clone, Raw,
or Legacy terminal is added.

## Buildable task order

```text
R0-A:
  add consuming non-recursive partition -> inventory/acyclic seam

R0-B:
  extract shared normal helper function-plan/correspondence preparation
  remove duplicate A0-only planning decisions

R0-C:
  add one-shot VerifiedNormalHelperTopologyPlanV1 selector

R0-D:
  add recursive variant over existing SCC identities/order

R0-E:
  focused recursion/reorder/rejection/reuse fixtures
  extend the existing normal-source-plan lane guard only
```

## Fixture matrix

```text
one call-free helper                    -> Acyclic
finite helper DAG                       -> Acyclic
self-recursive helper                   -> Recursive
two-helper mutual recursion             -> Recursive
recursive SCC + independent leaf        -> Recursive
Main -> recursive helper                -> Recursive
declaration reorder                     -> same normalized partition/meaning
profile rejection                       -> owner retained
success -> rejection -> success         -> green
```

## Structural gate

```text
graph inventory producer                           = 1
SCC partition producer                             = 1
normal topology selector                           = 1
acyclic retry after failure                        = 0
recursive retry after failure                      = 0

normal Program owner count                         = 1
helper catalog/index                               = 1
resolver session                                   = 1
Main catalog membership                            = 0
AST clone/rewrite                                  = 0
fallback                                           = 0

Builder/MIR/module publication                     = 0
VM/runner/CLI caller                               = 0
existing VM0 route delta                           = 0
all modified/new source/check files                < 800 lines
```

## Immediate continuation

```text
NORMAL-CALLABLE-MODULE0-R0-S0
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
```

## Non-claims

```text
Builder/MIR draft creation
atomic module publication
physical Main thunk
VM execution
new production/default caller
imports / using
Main-box helper methods
instance methods / receiver
dynamic/object result carrier
cleanup activation
Legacy retirement
```

## Closeout

```text
Status:
  closed

Landed:
  26d714da0c feat: select normal helper topology once
  3670b7f14f test: close normal recursive module plan

Selection:
  one graph inventory
  -> one deterministic SCC partition
  -> recursive component count
  -> Acyclic or Recursive exactly once

Accepted:
  zero-edge helpers
  finite helper DAG
  self recursion
  mutual recursion
  recursive SCC plus independent leaf
  Main call into a recursive helper

Evidence:
  declaration reorder keeps the normalized partition
  profile rejection leaves the completed owner borrowable
  success -> rejection -> success is green
  Main is absent from the helper partition
  acyclic-error-to-recursive retry = 0
  second inventory/partition       = 0
  fallback                         = 0

Verification:
  normal_source_plan tests         = 53 green
  callable_scc_partition tests     = 4 green
  acyclic_callable_graph tests     = 3 green
  normal source-plan guard         = green
  vm-reference library check       = green
  pointer guard                    = green
  touched source/check files       < 800 lines

Next:
  NORMAL-CALLABLE-MODULE0-TX0-S0
```
