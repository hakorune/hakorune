---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-064A scalar reclaim scheduler request marker contract.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-boundary-inventory-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-551-MIMAP-064A-RECLAIM-SCHEDULER-REQUEST-MARKER-CONTRACT.md
  - lang/src/hako_alloc/memory/reclaim_scheduler_request_marker_box.hako
  - apps/hako-alloc-reclaim-scheduler-request-marker-proof/
---

# Hako Alloc Reclaim Scheduler Request Marker SSOT

## Decision

`MIMAP-064A` adds a scalar scheduler request marker contract after reclaim
completion.

The route may compose `HakoAllocReclaimCompletionMarker` and decide whether a
completed scalar reclaim result would request modeled scheduler handoff. It
does not execute real scheduling, spawn workers, expose source-level
concurrency semantics, call page-source APIs, release OSVM memory, activate
providers, or replace the host allocator.

## Owner

```text
lang/src/hako_alloc/memory/reclaim_scheduler_request_marker_box.hako
```

Responsibilities:

```text
compose HakoAllocReclaimCompletionMarker
set request_marker only when completion succeeds and scheduler is enabled
report suppress reasons for completion-blocked and scheduler-disabled rows
count attempts, requests, and suppressions
report inactive production surfaces
```

Non-responsibilities:

```text
real thread scheduling
worker spawning
source-level concurrency features
page-source / OSVM release
provider / hook / replacement
backend app/name matcher
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | scheduler request marker was set |
| `1` | reclaim completion was blocked |
| `2` | scheduler request is disabled / local-only |

The report preserves the reclaim completion and integration facts that drove
the decision.

## Proof Surface

```text
apps/hako-alloc-reclaim-scheduler-request-marker-proof/
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh
```

Required inactive facts:

```text
would_schedule_thread = 0
would_spawn_worker = 0
would_touch_source_concurrency = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
would_activate_provider = 0
would_host_allocator_swap = 0
```

## Stop Lines

No part of `MIMAP-064A` may add:

```text
real thread scheduling
source-level concurrency semantics
source-level worker_local
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```
