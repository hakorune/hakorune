# 293x-443 MIMAP-026B Post-Huge-Unregister Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-026B` is a planning-only row. It selects exactly one next allocator
behavior row after the facade huge-release path has reached:

```text
huge request allocation through MIMAP-023A
M181 success release through HakoAllocHugeReleaseSeam.releaseHugePtr(ptr)
metadata release + page-map unregister
```

```text
MIMAP-027A facade huge-unregister fail-fast diagnostics route
```

The selected next durable slice stays on the M181 seam and proves rejection
behavior after page-map unregister:

```text
first huge unregister -> MIMAP-026A success
second unregister of same pointer -> M181 lookup-miss reject
stale/unknown huge pointer -> M181 lookup-miss reject
scalar diagnostics only
```

OS page return, unreserve/decommit, provider activation, host allocator
replacement, hooks, and `#[global_allocator]` remain later.

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

## Selected Next Row

```text
row:
  MIMAP-027A facade huge-unregister fail-fast diagnostics route

owner:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unregister_failfast_box.hako

reused owners:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unregister_box.hako
  lang/src/hako_alloc/memory/huge_release_seam_box.hako
  lang/src/hako_alloc/memory/page_map_box.hako
  lang/src/hako_alloc/memory/huge_page_model_box.hako

proof app:
  apps/mimalloc-facade-huge-unregister-failfast-proof/main.hako

guard:
  tools/checks/k2_wide_mimalloc_facade_huge_unregister_failfast_exe_guard.sh
```

MIMAP-027A should compose the MIMAP-026A route, then use the same M181
`HakoAllocHugeReleaseSeam` to reject a second release of the now-unregistered
pointer and one stale/unknown pointer. The proof should show page-map live count
stays zero after rejects while lookup-miss and reject counters advance.

## Stop Lines

- Do not implement allocator behavior in this planning row.
- Do not add OSVM release/unreserve/decommit, small release/free, realloc,
  alignment, purge/reclaim, remote-free, TLS, atomic, provider hook, host
  allocator replacement, or backend `.inc` matcher shortcut.
- Do not widen MIMAP-026A while selecting the next row.

MIMAP-027A stop lines:

- Do not add OSVM release/unreserve/decommit, purge/reclaim, provider hooks,
  host allocator replacement, or `#[global_allocator]`.
- Do not add small release/free, realloc, alignment, remote-free, TLS, atomic,
  or backend `.inc` matcher shortcuts.
- Do not add direct page-map lookup/unregister or direct
  `HakoAllocHugePageModel.markReleased(ptr)` calls in the facade diagnostics
  owner; M181 remains the release/reject seam.

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

## Closeout

MIMAP-026B is closed as a docs-only row selection. The active blocker moves to
MIMAP-027A.
