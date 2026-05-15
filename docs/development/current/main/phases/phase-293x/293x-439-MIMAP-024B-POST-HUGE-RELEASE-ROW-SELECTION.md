# 293x-439 MIMAP-024B Post-Huge-Release Row Selection

Status: ready
Date: 2026-05-16

## Decision

`MIMAP-024B` is a planning-only row.

It selects the next allocator behavior row after the facade can allocate a
huge request through the MIMAP-023A route and retire that same live huge
pointer through the MIMAP-024A metadata release route:

```text
MIMAP-023A facade huge-page model allocation
MIMAP-024A facade huge-release metadata route
```

The row must decide whether the next durable slice should deepen release
diagnostics, add facade double-release / stale-pointer fail-fast, move toward a
page-map unregister seam, or select another allocator completeness boundary. It
must not implement allocator behavior itself.

## Scope

- Review the post-MIMAP-024A huge-handle lifetime path.
- Pick exactly one next allocator behavior row.
- Record the owner, proof app, guard, and stop lines for that row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  inactive.

## Stop Lines

- Do not implement allocator behavior in this row.
- Do not add page-map lookup/unregister, OSVM release/unreserve/decommit,
  double-release / stale-pointer fail-fast, small release/free, realloc,
  alignment, purge/reclaim, remote-free, TLS, atomic, provider hook, host
  allocator replacement, or backend `.inc` matcher shortcut.
- Do not widen MIMAP-024A beyond metadata release while selecting the next row.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
