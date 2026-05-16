# 293x-468 MIMAP-037B Post-Backing-Set-Helper Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-037B` is a planning-only row. It selects exactly one next row after the
landed MIMAP-037A backing-set helper cleanup.

It must not land code.

The next row should be chosen from the smallest safe action after the facade
huge unreserve closeout and backing-set cleanup:

```text
candidate:
  a narrow allocator behavior row only if it does not need more report/state
  helper cleanup first
candidate:
  another BoxShape cleanup if repeated facade huge report copy/reset code blocks
  the next behavior
candidate:
  a language minimal-surface lane switch if allocator completeness no longer
  has a narrower current blocker
candidate:
  provider/host allocator replacement remains parked unless explicitly reopened
```

## Selection Criteria

The selected row must:

- build on MIMAP-032A through MIMAP-037A evidence
- name one owner, proof/guard, and stop lines before implementation
- keep allocator-provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive unless the selected row explicitly reopens a
  provider ladder
- keep BoxShape cleanup separate from allocator behavior

## Candidate Template

The closeout for this card should fill in:

```text
row:
  MIMAP-038A <selected owner / behavior>
owner:
  <new or reused owner path>
proof app:
  <proof app path or none>
guard:
  <focused guard>
primary proof:
  <smallest scalar proof or closeout guard>
stop lines:
  no provider activation unless this is an explicit provider-ladder row
  no host allocator replacement / hook / #[global_allocator]
  no backend .inc matcher shortcut
```

If the next row needs a compiler/language sidecar, this card must name the
sidecar and keep allocator implementation parked until the sidecar is green.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `037B.1` | Review MIMAP-037A cleanup evidence. | Next row does not repeat helper extraction. | no code |
| `037B.2` | Pick exactly one next row. | Owner/proof/guard/stop lines are named. | no broad provider work |
| `037B.3` | Update current pointers. | Current state moves to the selected next row. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next row is selected with clear owner/proof/guard names
and provider/host allocator replacement still inactive unless explicitly
reopened.

## Selection Result

`MIMAP-037B` selects `MIMAP-038A`.

Rationale:

- MIMAP-037A removed the route-local backing-set parallel arrays from the
  unreserve fail-fast path.
- The next smallest cleanup from the design review is a concrete hardcoded
  shape in `HakoAllocObjectLifecycleFacade.objectLifecycleKnownPageIndexById`.
- The method currently recognizes only indices 0, 1, and 2 even though the
  facade queue can hold more pages.

Selected row:

```text
row:
  MIMAP-038A object-lifecycle known-page loop cleanup
owner:
  lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
proof app:
  apps/mimalloc-facade-known-page-loop-proof/main.hako
guard:
  tools/checks/k2_wide_mimalloc_facade_known_page_loop_guard.sh
primary proof:
  four known pages; fourth page is found and released through the facade
stop lines:
  no allocator behavior beyond lookup shape cleanup
  no object-lifecycle page queue selection rewrite
  no provider activation
  no host allocator replacement / hook / #[global_allocator]
  no backend .inc matcher shortcut
```

Closeout:

```text
current blocker moves to MIMAP-038A object-lifecycle known-page loop cleanup.
```
