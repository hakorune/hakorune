# 293x-443 MIMAP-026B Post-Huge-Unregister Row Selection

Status: ready
Date: 2026-05-16

## Decision

`MIMAP-026B` is a planning-only row. It must select exactly one next allocator
behavior row after the facade huge-release path has reached:

```text
huge request allocation through MIMAP-023A
M181 success release through HakoAllocHugeReleaseSeam.releaseHugePtr(ptr)
metadata release + page-map unregister
```

The selection must keep OS page return, provider activation, host allocator
replacement, hooks, and `#[global_allocator]` inactive unless the chosen row
explicitly documents a future narrow ladder step.

## Scope

- Review the post-MIMAP-026A huge-release unregister state.
- Pick exactly one next allocator behavior row.
- Record the owner, proof app, guard, and stop lines for that row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  inactive.

## Candidate Questions

- Should the next row add narrow success-path OS page return / unreserve
  planning, or should it first add M181 facade fail-fast diagnostics for lookup
  miss / already-unregistered / stale huge pointers?
- Does the next row need a CorePlan / verifier contract promotion before any
  allocator behavior changes?
- Which proof app is the smallest durable observable contract after page-map
  unregister?

## Stop Lines

- Do not implement allocator behavior in this planning row.
- Do not add OSVM release/unreserve/decommit, small release/free, realloc,
  alignment, purge/reclaim, remote-free, TLS, atomic, provider hook, host
  allocator replacement, or backend `.inc` matcher shortcut.
- Do not widen MIMAP-026A while selecting the next row.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `026B.1` | Review MIMAP-026A landed evidence and nearby allocator backlog. | One next row is selected with owner/proof/guard names. | no implementation |
| `026B.2` | Update taskboard and granularity SSOT. | Current pointers move to the selected behavior row. | no provider activation |
| `026B.3` | Run pointer/quick gates. | Current docs are internally consistent. | no behavior widening |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
