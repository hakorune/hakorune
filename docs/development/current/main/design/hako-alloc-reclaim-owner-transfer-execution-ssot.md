---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-055A first guarded reclaim owner-transfer execution route.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-owner-transfer-contract-ssot.md
  - docs/development/current/main/design/hako-alloc-reclaim-atomic-claim-contract-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-542-MIMAP-055A-RECLAIM-OWNER-TRANSFER-FIRST-EXECUTION-ROUTE.md
  - lang/src/hako_alloc/memory/reclaim_owner_transfer_execution_box.hako
  - apps/hako-alloc-reclaim-owner-transfer-execution-proof/
---

# Hako Alloc Reclaim Owner-Transfer Execution SSOT

## Decision

`MIMAP-055A` opens the first guarded reclaim owner-transfer execution route.

The route may update only an executor-local modeled owner token after both
preconditions are true:

```text
owner-transfer contract:
  contract_ready == 1

atomic-claim contract:
  claim_succeeded == 1
```

This is not full reclaim. It does not drain remote-free queues, schedule
threads, call page-source APIs, unreserve or release OSVM pages, activate
providers, or mutate the production facade's page map.

## Owner

```text
lang/src/hako_alloc/memory/reclaim_owner_transfer_execution_box.hako
```

Responsibilities:

```text
compose HakoAllocReclaimOwnerTransferContract
compose HakoAllocReclaimAtomicClaimContract
execute one modeled owner-token transfer when both contracts succeed
publish blocked reasons from contract and claim subreports
count attempts, successes, contract blocks, and claim blocks
report inactive non-owner-transfer surfaces
```

Non-responsibilities:

```text
remote-free drain
thread scheduling
page-source calls
OSVM unreserve / release
production page-map mutation
provider / hook / replacement
secure entropy execution
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | owner transfer executed in the modeled executor |
| `1` | owner-transfer contract blocked the request |
| `2` | atomic-claim contract blocked the request |

The report preserves `contract_reason` and `claim_reason` so diagnostics do
not infer from the aggregate reason.

## Proof Surface

```text
apps/hako-alloc-reclaim-owner-transfer-execution-proof/
tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_execution_guard.sh
```

Required inactive facts:

```text
would_change_production_page_owner = 0
would_execute_full_reclaim = 0
would_drain_remote_free = 0
would_schedule_thread = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
would_activate_provider = 0
```

## Stop Lines

No part of `MIMAP-055A` may add:

```text
remote-free drain
thread scheduling
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```
