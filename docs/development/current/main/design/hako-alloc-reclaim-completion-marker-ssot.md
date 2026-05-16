---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-060A scalar reclaim completion marker route.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-post-drain-owner-transfer-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-547-MIMAP-060A-RECLAIM-COMPLETION-MARKER-ROUTE.md
  - lang/src/hako_alloc/memory/reclaim_completion_marker_box.hako
  - apps/hako-alloc-reclaim-completion-marker-proof/
---

# Hako Alloc Reclaim Completion Marker SSOT

## Decision

`MIMAP-060A` marks scalar reclaim completion after the post-drain
owner-transfer integration route succeeds.

This row is still allocator-local modeling. It does not call page-source APIs,
release or unreserve OSVM memory, schedule workers, activate providers, or
replace the host allocator.

## Owner

```text
lang/src/hako_alloc/memory/reclaim_completion_marker_box.hako
```

Responsibilities:

```text
compose HakoAllocReclaimPostDrainOwnerTransfer
set executor-local completion_marker only on integration success
preserve integration reason and child observations
count attempts, completions, and blocked rows
report inactive production surfaces
```

Non-responsibilities:

```text
page-source / OSVM release
thread scheduling
provider / hook / replacement
real remote-free pointer traversal
page-map mutation
backend app/name matcher
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | scalar reclaim completion marker was set |
| `1` | post-drain owner-transfer integration blocked |

The report preserves the integration reason.

## Proof Surface

```text
apps/hako-alloc-reclaim-completion-marker-proof/
tools/checks/k2_wide_hako_alloc_reclaim_completion_marker_guard.sh
```

Required inactive facts:

```text
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
would_schedule_thread = 0
would_activate_provider = 0
would_host_allocator_swap = 0
```

## Stop Lines

No part of `MIMAP-060A` may add:

```text
page-source call
OSVM unreserve / release
thread scheduling
provider activation
hooks
host allocator replacement
backend app/name matcher
```
