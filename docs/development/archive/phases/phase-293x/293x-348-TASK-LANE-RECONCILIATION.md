# 293x-348 Task Lane Reconciliation

Status: landed.
Decision: accepted.

## Goal

Resolve task confusion before implementation continues.

## Decision

Active sidecar:

```text
CLEAN-WHILE-001 While deletion readiness inventory
```

Paused mainline:

```text
MIMAP-012 object-backed lifecycle queue LLVM route pilot
```

Parked diagnostic:

```text
VM-LIM-001 object-heavy page queue/facade route
```

## Why

The MIMAP lane, VM limitation follow-up, and compiler cleanup work were being
discussed in the same window. This card separates them so BoxShape cleanup does
not mix with MIMAP BoxCount work.

## Next

Start `CLEAN-WHILE-001` and do not resume MIMAP until the While cleanup decision
is complete or the user explicitly reselects MIMAP.
