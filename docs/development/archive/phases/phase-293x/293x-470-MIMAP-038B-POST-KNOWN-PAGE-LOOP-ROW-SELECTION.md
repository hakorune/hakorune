# 293x-470 MIMAP-038B Post-Known-Page-Loop Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-038B` is a planning-only row. It selects exactly one next row after the
landed MIMAP-038A object-lifecycle known-page lookup cleanup.

It must not land code.

## Candidate Set

```text
candidate:
  continue cleanup by addressing remaining object-lifecycle page queue
  fixed-shape selection, if selected as BoxShape
candidate:
  pick a narrow allocator behavior row if the facade cleanup no longer blocks
  the next mimalloc completeness seam
candidate:
  pick a small named-constant cleanup, such as remote-free retry bound, if it is
  the lowest-risk review item
candidate:
  park allocator behavior and switch to a language/compiler sidecar only if the
  next allocator row exposes a compiler acceptance blocker
```

## Selection Criteria

The selected row must:

- build on MIMAP-032A through MIMAP-038A evidence
- name one owner, proof/guard, and stop lines before implementation
- keep allocator-provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive unless the selected row explicitly reopens a
  provider ladder
- keep BoxShape cleanup separate from allocator behavior

## Candidate Template

```text
row:
  MIMAP-039A <selected owner / behavior>
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

`MIMAP-038B` selects `MIMAP-039A`.

Rationale:

- MIMAP-038A removed the fixed three-page lookup from the facade-local
  known-page lookup helper.
- The object-lifecycle queue `selectPage()` fixed-shape cleanup was probed but
  exposed a compiler acceptance blocker around loop-returned page objects and
  existing same-module page method calls. That belongs in a separate
  compiler/language sidecar, not this allocator cleanup row.
- The smallest safe review item that does not require compiler widening is the
  remote-free retry bound hardcoded in `HakoAllocRemoteFreePolicy.pushRetry`.

Selected row:

```text
row:
  MIMAP-039A remote-free retry bound named constant cleanup
owner:
  lang/src/hako_alloc/memory/remote_free_policy_box.hako
proof app:
  apps/hako-alloc-remote-free-policy-proof/main.hako
guard:
  tools/checks/k2_wide_hako_alloc_remote_free_retry_bound_guard.sh
primary proof:
  existing hako_alloc remote-free policy EXE proof still sees 0 and 1 retries
stop lines:
  no retry policy behavior change
  no pointer atomic route change
  no page queue / facade behavior change
  no provider activation
  no host allocator replacement / hook / #[global_allocator]
  no backend .inc matcher shortcut
```

Closeout:

```text
current blocker moves to MIMAP-039A remote-free retry bound named constant cleanup.
```
