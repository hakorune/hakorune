# 293x-470 MIMAP-038B Post-Known-Page-Loop Row Selection

Status: selected current
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
