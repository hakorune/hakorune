---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: Closeout for the M192-M213 hako_alloc purge, recommit, lifecycle, scheduler, and abandoned/reclaim inventory ladder.
Related:
  - docs/development/current/main/design/purge-lifecycle-ladder-map-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# Purge Lifecycle Ladder Closeout SSOT

## Decision

The M192-M213 purge/lifecycle ladder is closed as an observability and bounded
policy ladder.

It contains these behavior classes:

```text
read-only inventories
dry-run observers
bounded caller-provided decommit/recommit seams
state markers
duplicate guards
page-local reactivation after recommit
bounded scheduler small path
abandoned/reclaim vocabulary inventory
```

It does not open broad reclaim execution, thread scheduling, atomics expansion,
unreserve, OS release, provider activation, hooks, or process allocator
replacement.

## Closed ladder

| Range | Status | Notes |
| --- | --- | --- |
| M192-M194 | closed | purge policy inventory, dry-run observer, blocked execution entry |
| M195-M199 | closed | bounded decommit seam, page-source adapter, heap integration, marker, duplicate guard |
| M200-M205 | closed | decommitted reuse precondition, recommit seam, adapter, marker transition, page-local reactivation |
| M206 | closed | two-generation proof closeout |
| M207-C194b | closed | lifecycle vocabulary and verifier-owned invariants |
| M208-M210 | closed | reuse priority, stats observer, pure-first EXE hardening |
| M211-M212 | closed | lifecycle-report candidate inventory and bounded scheduler small path |
| M213 | closed | abandoned/reclaim vocabulary inventory |

## Inactive surfaces

These remain inactive after M213:

```text
thread scheduling
atomic claim / CAS based ownership transfer
remote-free drain during reclaim
reclaim execution
page ownership migration
unbounded purge loops
direct M197/M195/M196 calls from schedulers
direct page-source calls from schedulers
unreserve
OS release
provider activation
hooks
process allocator replacement
backend app/name matchers
```

Any future row that wants one of these must be opened explicitly and must not
reuse M192-M213 closeout as implicit permission.

## Stable seams

### Decommit path

```text
M192 classify raw local page facts
M193 observe heap page/backing
M195 bounded caller-provided decommit
M196 page-source decommit adapter
M197 heap integration
M198 marker
M199 duplicate guard
```

Schedulers must call M199, not M197/M195/M196 directly.

### Recommit path

```text
M200 reuse precondition
M202 bounded caller-provided recommit
M203 page-source recommit adapter
M204 marker transition
M205 page-local reactivate
```

Only M205 owns page-local reactivation in this ladder.

### Lifecycle and scheduling path

```text
M207 lifecycle report
M208 reuse priority
M209 stats snapshot
M211 purge candidate inventory
M212 bounded scheduler
M213 abandoned/reclaim inventory
```

M211 consumes reports only.
M212 may observe and may call M199 for one candidate.
M213 is vocabulary only.

## Next blocker selection

After this closeout, do not continue into reclaim execution automatically.

Recommended next blocker:

```text
D205 post-M213 next-lane selection
```

The next-lane selection should choose one of:

```text
language lane from D202 task breakdown
allocator provider ladder if explicitly reopened
reclaim execution proposal as a new guarded ladder
packed record/materialization work if current blocker reopens it
cleanup/guard manifest consolidation
```

Until D205 selects a lane, broad implementation work should stop here.

