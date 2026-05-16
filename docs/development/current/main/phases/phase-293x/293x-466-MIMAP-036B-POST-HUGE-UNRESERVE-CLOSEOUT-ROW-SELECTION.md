# 293x-466 MIMAP-036B Post-Huge-Unreserve-Closeout Row Selection

Status: selected current
Date: 2026-05-16

## Decision

`MIMAP-036B` is a planning-only row. It selects exactly one next row after the
landed MIMAP-036A post-huge-unreserve closeout guard.

It must not land code.

The next row should be chosen from the smallest safe action after the huge
unreserve lane is closed through success, fail-fast diagnostics, and closeout
guard:

```text
candidate:
  a narrow allocator behavior row only if a missing mimalloc completeness seam
  is identified
candidate:
  a cleanup row if repeated facade huge state/report boilerplate blocks the
  next behavior
candidate:
  a language minimal-surface lane switch if allocator completeness no longer
  has a narrower current blocker
candidate:
  provider/host allocator replacement remains parked unless explicitly reopened
```

## Selection Criteria

The selected row must:

- build on the MIMAP-032A through MIMAP-036A unreserve closeout evidence
- name one owner, proof/guard, and stop lines before implementation
- keep allocator-provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive unless the selected row explicitly reopens a
  provider ladder
- keep BoxShape cleanup separate from allocator behavior

## Candidate Template

The closeout for this card should fill in:

```text
row:
  MIMAP-037A <selected owner / behavior>
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
| `036B.1` | Review MIMAP-036A closeout evidence. | Next row does not repeat closeout proof. | no code |
| `036B.2` | Pick exactly one next row. | Owner/proof/guard/stop lines are named. | no broad provider work |
| `036B.3` | Update current pointers. | Current state moves to the selected next row. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next row is selected with clear owner/proof/guard names
and provider/host allocator replacement still inactive unless explicitly
reopened.
