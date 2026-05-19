---
Status: SSOT
Decision: accepted
Date: 2026-05-20
Scope: MIMAP-305A segment arena backing modeled allocation-ledger release/recycle continuation application bridge diagnostics.
---

# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Continuation Application Bridge Diagnostics

## Decision

MIMAP-305A adds an observer-only diagnostic surface for the MIMAP-304A
continuation application bridge.

The diagnostic owner reads a continuation application bridge inventory plus one
application report and publishes scalar summary facts. It does not record a
second application row and does not open real lifecycle generation, raw pointer
residence, real arena backing release/recycle, segment-map mutation, atomic
bitmap operation, OSVM/page-source calls, worker/TLS behavior, provider hooks,
or backend matchers.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_diagnostic_box.hako
```

## Input Contract

Accepted input:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleContinuationApplicationBridgeInventory
  inventory_count > 0

HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleContinuationApplicationBridgeReport
  accepted == 1
  continuation_application_present == 1
  modeled_continuation_application_present == 1
  application_token > 0
  inventory.hasApplicationToken(application_token) == 1
```

Accepted output:

```text
observed == 1
reason == 0
diagnostic_present == 1
application_accepted == 1
application_reason == 0
report_application_token == source report token
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| 0 | accepted continuation application bridge observed |
| 1 | missing application inventory |
| 2 | missing application report |
| 3 | rejected application report observed |
| 4 | missing or unknown application token |

Rejected application reports keep their source `report_reason`, so invalid
application-token, duplicate-token, and closed-substrate rows remain visible as
MIMAP-304A reason codes.

## Stop Lines

- No new continuation application row recording.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-305A
```

L3 EXE evidence is deferred to a future continuation-application closeout pack.
