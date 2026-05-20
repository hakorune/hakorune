---
Status: SSOT
Decision: accepted
Date: 2026-05-20
Scope: MIMAP-308A segment arena backing modeled allocation-ledger release/recycle applied-state summary inventory.
---

# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Applied-State Summary

## Decision

MIMAP-308A adds a model-only applied-state summary after the MIMAP-306A
continuation application bridge closeout.

The summary owner consumes an accepted MIMAP-304A continuation application
bridge report and publishes compact scalar/model applied-state facts. It does
not record another continuation application row and does not open real
lifecycle generation, pointer residence, real arena backing release/recycle,
segment-map mutation, atomic bitmap operation, OSVM/page-source calls,
worker/TLS behavior, provider hooks, or backend matchers.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_box.hako
```

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleContinuationApplicationBridgeReport
  accepted == 1
  continuation_application_present == 1
  modeled_continuation_application_present == 1
  application_token > 0
  closed_substrate_blocker_count == 0
```

Accepted output:

```text
summarized == 1
reason == 0
applied_state_summary_present == 1
applied_state_ready == 1
application_token == source report application_token
continuation_token == source report continuation_token
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | applied-state summary recorded |
| 1 | missing continuation application report |
| 2 | rejected continuation application report |
| 3 | invalid application token |
| 4 | closed substrate requirement present |

## Stop Lines

- No real lifecycle generation token.
- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation, release, or recycle.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.

## Validation

Daily validation is L2:

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-308A
```

L3 EXE evidence is deferred to a future applied-state summary closeout pack.
