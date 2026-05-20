# Hako Alloc Host Replacement Preflight Closeout

Status: accepted
Decision: accepted
Scope: MIMAP-422A host replacement preflight closeout.

## Purpose

MIMAP-422A closes the host replacement explicit preflight pack before any
hook-install preflight planning is selected. It binds together:

```text
MIMAP-420A host replacement explicit preflight inventory
MIMAP-421A host replacement blocked-state diagnostics
```

The closeout proves that explicit request / hook plan / rollback plan /
backend no-growth inputs can be inventoried and diagnosed while replacement
execution remains closed.

## Still Closed

```text
hook installation
backend matcher additions
process allocator replacement
#[global_allocator]
worker/TLS or thread execution
hidden env / implicit discovery / process-global activation config
```

## Validation

The closeout reuses the MIMAP-420A and MIMAP-421A L2 evidence:

```text
VM proof
MIR JSON emit
route preflight
```

No L3 process allocator replacement evidence exists in this row.
