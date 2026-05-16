---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-058A reclaim post-drain owner-transfer integration route.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-remote-free-drain-execution-ssot.md
  - docs/development/current/main/design/hako-alloc-reclaim-owner-transfer-execution-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-545-MIMAP-058A-RECLAIM-POST-DRAIN-OWNER-TRANSFER-INTEGRATION-ROUTE.md
  - lang/src/hako_alloc/memory/reclaim_post_drain_owner_transfer_box.hako
  - apps/hako-alloc-reclaim-post-drain-owner-transfer-proof/
---

# Hako Alloc Reclaim Post-Drain Owner-Transfer SSOT

## Decision

`MIMAP-058A` composes the modeled one-entry drain route and modeled
owner-transfer route.

The row may:

```text
1. run one modeled remote-free drain step
2. proceed to modeled owner-transfer only if pending_after == 0
```

It must not execute full reclaim, schedule workers, call page-source or OSVM
seams, or activate providers.

## Owner

```text
lang/src/hako_alloc/memory/reclaim_post_drain_owner_transfer_box.hako
```

Responsibilities:

```text
compose HakoAllocReclaimRemoteFreeDrainExecution
compose HakoAllocReclaimOwnerTransferExecution
enforce drain-before-transfer ordering
block when one modeled drain still leaves pending work
publish aggregate and child reasons
```

Non-responsibilities:

```text
full reclaim / purge cascade
thread scheduling
page-source calls
OSVM unreserve / release
provider / hook / replacement
backend app/name matcher
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | modeled owner-transfer completed after drain/no-work |
| `1` | modeled drain route blocked |
| `2` | remote-free work remains after one modeled drain |
| `3` | modeled owner-transfer route blocked |

The report preserves `drain_reason` and `transfer_reason`.

## Proof Surface

```text
apps/hako-alloc-reclaim-post-drain-owner-transfer-proof/
tools/checks/k2_wide_hako_alloc_reclaim_post_drain_owner_transfer_guard.sh
```

Required inactive facts:

```text
would_execute_full_reclaim = 0
would_schedule_thread = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
would_activate_provider = 0
```

## Stop Lines

No part of `MIMAP-058A` may add:

```text
full reclaim
thread scheduling
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```
