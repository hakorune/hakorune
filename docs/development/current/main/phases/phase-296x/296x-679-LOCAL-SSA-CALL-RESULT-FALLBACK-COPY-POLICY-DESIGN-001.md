---
Status: Active
Date: 2026-06-15
Task: LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-DESIGN-001
Scope: Design the narrow LocalSSA fallback Copy policy for page-hotpath helper
  call-result copy chains before any implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-678-PAGE-HOTPATH-HELPER-RESULT-EMISSION-OWNER-REFRESH-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-DESIGN-001

## Purpose

296x-678 refreshed the actual emission owner after the 296x-677 LocalSSA
terminal-consumer rewrite trial was rejected as a nonkeeper:

```text
dominant_emission_owner=LocalSSA::ensure_fallback_copy
selected_next_owner=local_ssa_call_result_fallback_copy_policy
candidate_result_copy_count=14
first_hop_call_result_copy_count=4
chain_internal_copy_count=10
terminal_compare_operand_count=4
terminal_compare_first_hop_root_count=4
```

This row designs the correct seam before touching `src/mir/builder/ssa/local.rs`.
The goal is to eliminate the selected page-hotpath helper result materialization
shape without turning LocalSSA into broad copy coalescing.

## Owner Boundary

```text
owner:
  src/mir/builder/ssa/local.rs
  LocalSSA::ensure fallback Copy emission

not owner:
  .hako source
  helper lowering
  allocator provider activation
  variable_map truth
  PHI / phi_lifecycle
  generic DCE / copy coalescing
```

## Design Questions

```text
1. Which fallback Copy cases are rematerialization requirements and which are
   redundant aliases?
2. Can CompareOperand consume an existing same-block call-result alias without
   emitting another fallback Copy?
3. Can Copy-def rematerialization stop at the nearest same-block alias instead
   of recursively emitting `copy copy` layers?
4. Which def kinds are allowed? Call only, Copy-of-Call only, or a bounded
   helper-result chain?
5. Which value-use kinds are allowed? CompareOperand only for first keeper, or
   Arg as well?
6. What guard proves this does not change variable_map / PHI / receiver
   materialization behavior?
```

## Required Output

```text
output_contract=hako-mimalloc-local-ssa-call-result-fallback-copy-policy-design-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-678
candidate_result_copy_count=14
first_hop_call_result_copy_count=4
chain_internal_copy_count=10
terminal_compare_operand_count=4
selected_policy_shape=<shape>
selected_policy_owner=LocalSSA::ensure_fallback_copy
selected_owner_confidence=<low|medium|high>
next_task=<task>
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not implement in this row
do not broaden LocalSSA copy coalescing
do not forward arbitrary Call results
do not assume helper purity from helper names alone
do not change variable_map binding semantics
do not change PHI lifecycle or phi input materialization
do not patch source .hako
do not claim a performance win
```

## Acceptance

```text
local_ssa_call_result_fallback_copy_policy_design_active=1
source_evidence=296x-678
design_probe_run=0
selected_policy_shape=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=pending
```
