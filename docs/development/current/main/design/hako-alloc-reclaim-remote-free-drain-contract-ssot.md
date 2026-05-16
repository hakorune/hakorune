---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-056A reclaim remote-free drain contract inventory.
Related:
  - docs/development/current/main/phases/phase-293x/293x-543-MIMAP-056A-RECLAIM-REMOTE-FREE-DRAIN-CONTRACT-INVENTORY.md
  - lang/src/hako_alloc/memory/reclaim_remote_free_drain_contract_box.hako
  - apps/hako-alloc-reclaim-remote-free-drain-contract-proof/
---

# Hako Alloc Reclaim Remote-Free Drain Contract SSOT

## Decision

`MIMAP-056A` names the remote-free drain boundary required before broader
reclaim execution may proceed.

The row is a no-execution contract. It does not drain remote-free queues.
Instead, it classifies scalar remote-free facts and reports whether the current
page can proceed without drain work.

## Owner

```text
lang/src/hako_alloc/memory/reclaim_remote_free_drain_contract_box.hako
```

Responsibilities:

```text
classify pending remote-free work
classify inconsistent pending/head facts
publish bounded-drain-readiness facts without executing a drain
count ready / pending / invalid / inconsistent rows
report inactive production surfaces
```

Non-responsibilities:

```text
remote-free drain execution
remote-free pointer traversal
thread scheduling
page-source calls
OSVM unreserve / release
production page-map mutation
provider / hook / replacement
backend app/name matcher
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | no remote-free drain is required |
| `1` | remote-free work is pending; drain is required before reclaim can proceed |
| `2` | remote-free pending count is invalid |
| `3` | remote-free head is present while pending count is zero |
| `4` | drain budget is invalid |

## Proof Surface

```text
apps/hako-alloc-reclaim-remote-free-drain-contract-proof/
tools/checks/k2_wide_hako_alloc_reclaim_remote_free_drain_contract_guard.sh
```

Required inactive facts:

```text
would_drain_remote_free = 0
would_schedule_thread = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
would_activate_provider = 0
would_execute_full_reclaim = 0
would_change_production_page_owner = 0
```

## Stop Lines

No part of `MIMAP-056A` may add:

```text
remote-free drain execution
thread scheduling
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```
