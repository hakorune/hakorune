---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-072A reclaim scheduler request ledger consume closeout guard.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-ssot.md
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-consume-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-559-MIMAP-072A-RECLAIM-SCHEDULER-LEDGER-CONSUME-CLOSEOUT-GUARD.md
---

# Hako Alloc Reclaim Scheduler Request Ledger Consume Closeout SSOT

## Decision

`MIMAP-072A` closes the scheduler request ledger consume route.

This row is a closeout guard only. It adds no `.hako` behavior and does not
execute scheduling, spawn workers, expose source-level concurrency features,
call page-source APIs, release OSVM memory, activate providers, or replace the
host allocator.

## Closed Row Set

| Row | Role |
| --- | --- |
| `MIMAP-068A` | scalar scheduler request ledger record route |
| `MIMAP-069A` | scheduler request ledger closeout guard |
| `MIMAP-071A` | scalar scheduler request ledger consume route |

## Guard Surface

```text
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_closeout_guard.sh
```

The guard verifies:

```text
MIMAP-071A card / SSOT / owner extension / proof app are landed
MIMAP-068A record route and MIMAP-071A consume route stay in the same ledger owner
MIMAP-071A guard is indexed and proof app is in proof_apps.toml
no scheduler app/owner matcher leaks into lang/c-abi/shims
provider / hook / replacement remain inactive
```

## Next Row

After this closeout, the current row should be:

```text
MIMAP-073A post-scheduler-consume row selection
```

That row decides whether to continue scalar allocator behavior, open a
compiler/language sidecar, or reconsider real scheduler substrate.

## Stop Lines

No part of this row may add:

```text
new .hako allocator behavior
real thread scheduling
worker spawning
source-level concurrency semantics
source-level worker_local
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```
