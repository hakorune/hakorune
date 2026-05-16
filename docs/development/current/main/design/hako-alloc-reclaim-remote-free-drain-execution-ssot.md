---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-057A first modeled reclaim remote-free drain execution route.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-remote-free-drain-contract-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-544-MIMAP-057A-RECLAIM-REMOTE-FREE-DRAIN-FIRST-EXECUTION-ROUTE.md
  - lang/src/hako_alloc/memory/reclaim_remote_free_drain_execution_box.hako
  - apps/hako-alloc-reclaim-remote-free-drain-execution-proof/
---

# Hako Alloc Reclaim Remote-Free Drain Execution SSOT

## Decision

`MIMAP-057A` opens the first narrow modeled remote-free drain execution route.

The route may decrement one executor-local modeled pending count when the
MIMAP-056A contract reports pending remote-free work and the provided drain
budget can cover that pending work. It does not traverse remote-free pointer
lists, call the existing remote-free inbox, schedule workers, or continue into
full reclaim.

## Owner

```text
lang/src/hako_alloc/memory/reclaim_remote_free_drain_execution_box.hako
```

Responsibilities:

```text
compose HakoAllocReclaimRemoteFreeDrainContract
execute at most one modeled pending-count decrement
publish blocked reasons for no-work, invalid contract facts, and budget block
count attempts, modeled drains, no-work, contract blocks, and budget blocks
report inactive broader reclaim surfaces
```

Non-responsibilities:

```text
remote-free pointer traversal
real atomic operations
page-local releaseLocal
thread scheduling
page-source calls
OSVM unreserve / release
full reclaim / purge cascade
provider / hook / replacement
backend app/name matcher
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | one modeled remote-free pending entry was drained |
| `1` | no drain work is required |
| `2` | drain contract blocked due to invalid or inconsistent facts |
| `3` | drain budget does not cover pending remote-free work |

The report preserves the MIMAP-056A `contract_reason` so diagnostics do not
infer from the aggregate reason.

## Proof Surface

```text
apps/hako-alloc-reclaim-remote-free-drain-execution-proof/
tools/checks/k2_wide_hako_alloc_reclaim_remote_free_drain_execution_guard.sh
```

Required inactive facts:

```text
would_schedule_thread = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
would_activate_provider = 0
would_execute_full_reclaim = 0
would_change_production_page_owner = 0
would_call_page_release = 0
```

## Stop Lines

No part of `MIMAP-057A` may add:

```text
real remote-free queue drain
remote-free pointer traversal
thread scheduling
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
full reclaim
backend app/name matcher
```
