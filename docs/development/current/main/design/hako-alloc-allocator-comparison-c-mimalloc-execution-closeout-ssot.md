# Hako Alloc Allocator Comparison C Mimalloc Execution Closeout SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Scope: MIMAP-450A allocator comparison C mimalloc execution closeout.

## Purpose

MIMAP-450A closes the C mimalloc execution inventory and diagnostics pack before
any actual C mimalloc comparison runner is executed.

The closeout proves that:

```text
MIMAP-448A C mimalloc execution inventory
  -> MIMAP-449A C mimalloc execution diagnostics
  -> C mimalloc execution package is observable
```

## Included Rows

```text
MIMAP-448A allocator comparison C mimalloc execution inventory
MIMAP-449A allocator comparison C mimalloc execution diagnostics
```

## Still Closed

```text
C mimalloc execution
hook installation
backend matcher additions
process allocator replacement
#[global_allocator]
worker/TLS or thread execution
hidden env / implicit discovery / process-global activation config
```

## Next Row

The next row may open a narrow explicit-runner pilot:

```text
MIMAP-451A Allocator Comparison C Mimalloc Explicit Runner Execution Pilot
```

The pilot must keep process allocator replacement, hooks, backend matchers, and
global allocator installation closed.

## Validation

```text
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_execution_closeout_guard.sh
```

The closeout re-runs the MIMAP-448A and MIMAP-449A L2 guards. It does not run C
mimalloc itself.
