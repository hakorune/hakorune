# 293x-445 MIMAP-027B Post-Huge-Unregister-Failfast Row Selection

Status: ready
Date: 2026-05-16

## Decision

`MIMAP-027B` is a planning-only row. It must select exactly one next allocator
behavior row after the facade huge-release path has both:

```text
MIMAP-026A M181 success unregister
MIMAP-027A double/stale post-unregister reject diagnostics
```

The selection must keep provider activation, host allocator replacement, hooks,
and `#[global_allocator]` inactive unless the chosen row explicitly documents a
future narrow ladder step.

## Scope

- Review the post-MIMAP-027A huge unregister success + reject state.
- Pick exactly one next allocator behavior row.
- Record the owner, proof app, guard, and stop lines for that row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  inactive.

## Candidate Questions

- Is the next durable row a narrow OS page return / unreserve planning step, or
  should the facade first expose a verifier/CorePlan no-fallback contract for
  released huge ownership?
- Does any later row need to promote M181 page-map unregister/fail-fast facts
  out of metadata-only observers before touching OSVM release?
- Which proof app is the smallest observable contract after M181 success and
  reject diagnostics are both green?

## Stop Lines

- Do not implement allocator behavior in this planning row.
- Do not add OSVM release/unreserve/decommit, small release/free, realloc,
  alignment, purge/reclaim, remote-free, TLS, atomic, provider hook, host
  allocator replacement, or backend `.inc` matcher shortcut.
- Do not widen MIMAP-027A while selecting the next row.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `027B.1` | Review MIMAP-027A landed evidence and nearby allocator backlog. | One next row is selected with owner/proof/guard names. | no implementation |
| `027B.2` | Update taskboard and granularity SSOT. | Current pointers move to the selected behavior row. | no provider activation |
| `027B.3` | Run pointer/quick gates. | Current docs are internally consistent. | no behavior widening |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
