---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-077A reclaim scheduler scalar lane closeout guard.
Related:
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-boundary-inventory-ssot.md
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-marker-ssot.md
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-ssot.md
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-consume-ssot.md
  - docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-564-MIMAP-077A-RECLAIM-SCHEDULER-SCALAR-LANE-CLOSEOUT-GUARD.md
---

# Hako Alloc Reclaim Scheduler Scalar Lane Closeout SSOT

## Decision

`MIMAP-077A` closes the scalar reclaim scheduler lane.

This row is a closeout guard only. It adds no `.hako` behavior and does not
execute scheduling, spawn workers, expose source concurrency features, call
page-source APIs, release OSVM memory, activate providers, or replace the host
allocator.

## Closed Row Set

| Row | Role |
| --- | --- |
| `MIMAP-063A` | scheduler boundary inventory |
| `MIMAP-064A` | scalar scheduler request marker |
| `MIMAP-065A` | scheduler marker closeout guard |
| `MIMAP-067A` | real scheduler substrate proposal-or-park |
| `MIMAP-068A` | scalar scheduler request ledger record route |
| `MIMAP-069A` | scheduler request ledger closeout guard |
| `MIMAP-071A` | scalar scheduler request ledger consume route |
| `MIMAP-072A` | scheduler ledger consume closeout guard |
| `MIMAP-074A` | scalar scheduler request ledger roundtrip route |
| `MIMAP-075A` | scheduler ledger roundtrip closeout guard |

## Guard Surface

```text
tools/checks/k2_wide_hako_alloc_reclaim_scheduler_scalar_lane_closeout_guard.sh
```

The guard verifies:

```text
scheduler marker / ledger / consume / roundtrip SSOT files remain accepted
focused scheduler guards remain indexed and executable
proof apps remain listed in proof_apps.toml
owners remain scalar allocator-local surfaces
no scheduler app/owner matcher leaks into lang/c-abi/shims
provider / hook / replacement remain inactive
```

## Next Row

After this closeout, the current row should be:

```text
MIMAP-078A post-scheduler-scalar-closeout row selection
```

That row decides whether to continue scalar allocator behavior, open a
compiler/language sidecar, or reconsider real scheduler substrate.

## Stop Lines

No part of this row may add:

```text
new .hako allocator behavior
real thread scheduling
worker spawning
source concurrency semantics
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```
