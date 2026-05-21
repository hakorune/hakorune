# Hako Alloc Allocator Comparison C Mimalloc Execution Inventory SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-448A

## Decision: accepted

MIMAP-448A inventories the explicit inputs required before any C mimalloc
comparison execution row can run.

This row does not execute C mimalloc. It only records whether the runner,
representative workload, Hako representative metrics, output contract,
memory-usage contract, evidence storage, and run count are present.

## Input Contract

Accepted inventory requires:

```text
c_mimalloc_runner_present == 1
representative_workload_contract_present == 1
hako_representative_metrics_present == 1
output_contract_present == 1
memory_usage_contract_present == 1
evidence_storage_present == 1
run_count_present == 1
run_count >= 1
```

## Reject Reasons

| Reason | Meaning |
| --- | --- |
| 0 | accepted |
| 1 | C mimalloc runner missing |
| 2 | representative workload contract missing |
| 3 | Hako representative metrics missing |
| 4 | output contract missing |
| 5 | memory-usage contract missing |
| 6 | evidence storage missing |
| 7 | run count missing |
| 8 | invalid run count |

## Stop Lines

- No C mimalloc execution.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No global allocator installation.
- No hidden env or implicit discovery of C mimalloc behavior.
- No worker/thread execution.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI.
- No runtime sum materialization.

## Validation

Validation profile: `scalar-mir`.

MIMAP-448A runs L2 daily evidence:

- VM proof app output contract
- MIR JSON emit
- route preflight
- typed object / record declaration checks
- `.inc` no-growth check for app / owner names

L3/L4 evidence is deferred to a later C mimalloc execution closeout.
