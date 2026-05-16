# 293x-465 MIMAP-036A Post-Huge-Unreserve Closeout Guard

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-036A` is the closeout row selected by `MIMAP-035B`.
MIMAP-035B selected MIMAP-036A to reconcile docs and guards before any broader
allocator behavior is opened.

It does not add allocator behavior. It reconciles the current docs and guard
surface after the facade huge unreserve lane reached:

```text
MIMAP-034A:
  facade huge unreserve-after-decommit success

MIMAP-035A:
  duplicate/stale facade huge unreserve fail-fast diagnostics
```

## Scope

- Add the post-huge-unreserve closeout SSOT:
  `docs/development/current/main/design/mimalloc-post-huge-unreserve-closeout-ssot.md`.
- Add a focused guard:
  `tools/checks/k2_wide_mimalloc_post_huge_unreserve_closeout_guard.sh`.
- Update task pointers so future rows know unreserve is now live through
  MIMAP-035A, while provider/host replacement and broader lifecycle surfaces
  remain inactive.

## Stop Lines

- Do not add allocator behavior.
- Do not add recommit, purge scheduler, remote-free, TLS cache, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `036A.1` | Add closeout SSOT. | Completed/inactive surfaces are named without changing behavior. | no code behavior |
| `036A.2` | Add closeout guard. | Guard pins docs, owner files, proof guards, and inactive provider/replacement lines. | no backend matcher |
| `036A.3` | Close current pointers. | Current state moves to a planning row after closeout. | no implementation behavior |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_post_huge_unreserve_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when the post-huge-unreserve docs/guard inventory is green and
the current blocker moves to the next planning row without opening provider or
host allocator replacement.

## Landed Implementation

```text
closeout SSOT:
  docs/development/current/main/design/mimalloc-post-huge-unreserve-closeout-ssot.md
guard:
  tools/checks/k2_wide_mimalloc_post_huge_unreserve_closeout_guard.sh
```

The landed row reconciles the unreserve lane after MIMAP-034A/MIMAP-035A. It
names MIMAP-032A through MIMAP-035A as the completed huge unreserve surface and
keeps provider activation, hooks, host allocator replacement,
`#[global_allocator]`, recommit, purge scheduler widening, remote-free/TLS
behavior changes, and backend `.inc` matchers inactive.

Closeout:

```text
current blocker moves to MIMAP-036B post-huge-unreserve-closeout row selection.
```
