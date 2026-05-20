# Hako Alloc Host Replacement Optional Ladder Plan

Status: accepted
Decision: accepted
Scope: MIMAP-419A host replacement optional ladder plan.

## Purpose

MIMAP-419A defines the optional host replacement ladder after the real external
provider API call first-pattern closeout. This plan keeps replacement optional
and closed while naming the next rows that would be required before any process
allocator replacement is considered.

## Boundary

The current `hako_alloc` lane is still an allocator implementation and
comparison target. It is not the default process allocator.

Future replacement work must pass through explicit rows; no hidden env,
implicit discovery, backend matcher, hook installation, or global allocator
install may activate it.

## Planned Ladder

```text
MIMAP-420A host replacement explicit preflight inventory
MIMAP-421A host replacement blocked-state diagnostics
MIMAP-422A host replacement preflight closeout
MIMAP-423A hook-install preflight plan
MIMAP-424A backend matcher no-growth closeout
MIMAP-425A optional process allocator replacement proposal
```

## Still Closed

The following remain closed:

```text
hook installation
backend matcher additions
process allocator replacement
#[global_allocator]
worker/TLS or thread execution
process-global activation config
```

## Validation

```text
validation_profile = planning
exe = not-applicable
```
